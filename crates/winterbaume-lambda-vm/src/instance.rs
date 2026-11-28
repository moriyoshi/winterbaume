//! One execution environment: a helper process running a microVM, the
//! Runtime API it talks to, and its console output.

use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::sync::{OwnedSemaphorePermit, oneshot, watch};
use winterbaume_lambda::execution::{frame_log, runtime_error};
use winterbaume_lambda::{InvocationOutcome, StubReason};

use crate::config::HelperCommand;
use crate::endpoint::EndpointServer;
use crate::proto::{
    AGENT_FAILED_EXIT, AWS_ENDPOINT_GUEST_PORT, AWS_ENDPOINT_VSOCK_PORT, Forward, HelperStatus,
    NO_BOOTSTRAP_EXIT, RUNTIME_API_GUEST_PORT, RUNTIME_API_VSOCK_PORT, STATUS_PREFIX, VmSpec,
};
use crate::runtime_api::{PendingInvocation, RuntimeApi, RuntimeOutcome, SubmitError};
use winterbaume_core::RequestDispatcher;

/// How long the helper may take to report its status.
const STATUS_TIMEOUT: Duration = Duration::from_secs(15);
/// How long to keep collecting console output after an invocation ends.
const LOG_GRACE: Duration = Duration::from_millis(50);
/// Console output kept per environment between invocations.
const MAX_BUFFERED_OUTPUT: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExitInfo {
    code: Option<i32>,
}

/// Console output of the guest.
#[derive(Default)]
struct Console {
    bytes: Mutex<Vec<u8>>,
}

impl Console {
    fn push(&self, chunk: &[u8]) {
        let mut bytes = self.bytes.lock().unwrap_or_else(|e| e.into_inner());
        bytes.extend(chunk.iter().copied().filter(|b| *b != b'\r'));
        if bytes.len() > 2 * MAX_BUFFERED_OUTPUT {
            let excess = bytes.len() - MAX_BUFFERED_OUTPUT;
            bytes.drain(..excess);
        }
    }

    fn take(&self) -> Vec<u8> {
        std::mem::take(&mut *self.bytes.lock().unwrap_or_else(|e| e.into_inner()))
    }
}

/// What one invocation is.
pub(crate) struct Call {
    pub request_id: String,
    pub invoked_function_arn: String,
    pub payload: Bytes,
    pub client_context: Option<String>,
    pub timeout: Duration,
    pub function_version: String,
    pub memory_size: u32,
}

pub(crate) struct Instance {
    api: RuntimeApi,
    /// Serves the guest's `AWS_ENDPOINT_URL`, when there is an endpoint.
    _endpoint: Option<EndpointServer>,
    /// Holds the host ends of the guest's vsock ports.
    _sockets: tempfile::TempDir,
    console: Arc<Console>,
    exit: watch::Receiver<Option<ExitInfo>>,
    /// Dropping it kills the helper.
    kill: Option<oneshot::Sender<()>>,
    pid: Option<u32>,
    init_timeout: Duration,
    pub(crate) last_used: Instant,
    pub(crate) generation: u64,
    /// Set when the environment must not be reused.
    pub(crate) poisoned: bool,
    _permits: Vec<OwnedSemaphorePermit>,
}

impl Drop for Instance {
    fn drop(&mut self) {
        // Dropping the sender wakes the supervisor, which kills the helper.
        self.kill.take();
    }
}

impl Instance {
    /// Start a helper for `spec` and wait until it says the VM is booting.
    pub(crate) async fn start(
        helper: &HelperCommand,
        mut spec: VmSpec,
        aws: Option<Arc<dyn RequestDispatcher>>,
        init_timeout: Duration,
        generation: u64,
        permits: Vec<OwnedSemaphorePermit>,
    ) -> Result<Self, StubReason> {
        let unavailable = |what: &str, e: std::io::Error| {
            tracing::warn!(error = %e, "{what}");
            StubReason::VmUnavailable
        };
        // Short paths: Unix socket paths are limited to about 100 bytes.
        let sockets = tempfile::Builder::new()
            .prefix("wb-lvm-")
            .tempdir_in(std::env::temp_dir())
            .map_err(|e| unavailable("cannot create the Lambda VM socket directory", e))?;
        let runtime_socket = sockets.path().join("rapi.sock");
        let api = RuntimeApi::bind(&runtime_socket)
            .await
            .map_err(|e| unavailable("cannot listen for the Lambda Runtime API", e))?;
        spec.forwards.push(Forward {
            guest_port: RUNTIME_API_GUEST_PORT,
            vsock_port: RUNTIME_API_VSOCK_PORT,
            host_socket: runtime_socket,
        });
        spec.env.insert(
            "AWS_LAMBDA_RUNTIME_API".to_string(),
            format!("127.0.0.1:{RUNTIME_API_GUEST_PORT}"),
        );
        let endpoint = match aws {
            Some(dispatcher) => {
                let aws_socket = sockets.path().join("aws.sock");
                let server = EndpointServer::bind(&aws_socket, dispatcher)
                    .map_err(|e| unavailable("cannot serve the AWS endpoint for Lambda", e))?;
                spec.forwards.push(Forward {
                    guest_port: AWS_ENDPOINT_GUEST_PORT,
                    vsock_port: AWS_ENDPOINT_VSOCK_PORT,
                    host_socket: aws_socket,
                });
                // A function's own setting wins.
                spec.env
                    .entry("AWS_ENDPOINT_URL".to_string())
                    .or_insert_with(|| format!("http://127.0.0.1:{AWS_ENDPOINT_GUEST_PORT}"));
                Some(server)
            }
            None => None,
        };
        let input = serde_json::to_vec(&spec).expect("a VM spec serialises");

        let mut child = tokio::process::Command::new(&helper.program)
            .args(&helper.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| {
                tracing::warn!(helper = %helper.program.display(), error = %e, "cannot start the Lambda VM helper");
                StubReason::VmHelperNotFound
            })?;
        let pid = child.id();
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(&input).await;
            let _ = stdin.shutdown().await;
        }

        let console = Arc::new(Console::default());
        let (status_tx, mut status_rx) = watch::channel::<Option<HelperStatus>>(None);
        if let Some(stdout) = child.stdout.take() {
            tokio::spawn(read_console(stdout, console.clone(), None));
        }
        if let Some(stderr) = child.stderr.take() {
            tokio::spawn(read_console(stderr, console.clone(), Some(status_tx)));
        }

        let (exit_tx, exit) = watch::channel(None);
        let (kill, killed) = oneshot::channel::<()>();
        tokio::spawn(supervise(child, killed, exit_tx));

        let mut exit_rx = exit.clone();
        let _ = tokio::time::timeout(STATUS_TIMEOUT, async {
            tokio::select! {
                _ = async { status_rx.wait_for(Option::is_some).await.is_ok() } => {}
                _ = async { exit_rx.wait_for(Option::is_some).await.is_ok() } => {
                    // The status line may still be on its way.
                    let _ = tokio::time::timeout(
                        Duration::from_millis(500),
                        async { status_rx.wait_for(Option::is_some).await.is_ok() },
                    )
                    .await;
                }
            }
        })
        .await;
        let status = status_rx.borrow().clone();
        let instance = Self {
            api,
            _endpoint: endpoint,
            _sockets: sockets,
            console,
            exit,
            kill: Some(kill),
            pid,
            init_timeout,
            last_used: Instant::now(),
            generation,
            poisoned: false,
            _permits: permits,
        };
        match status {
            Some(HelperStatus::Booting) => Ok(instance),
            Some(HelperStatus::Failed { reason, message }) => {
                tracing::warn!(%reason, %message, "the Lambda VM could not be started");
                Err(match reason.as_str() {
                    "libkrun-unavailable" => StubReason::LibkrunUnavailable,
                    _ => StubReason::VmUnavailable,
                })
            }
            None => {
                let output = String::from_utf8_lossy(&instance.console.take()).to_string();
                tracing::warn!(
                    helper = %helper.program.display(),
                    %output,
                    "the Lambda VM helper did not report a status; is it a VM helper?"
                );
                Err(StubReason::VmHelperNotFound)
            }
        }
    }

    /// The helper's process id.
    pub(crate) fn pid(&self) -> Option<u32> {
        self.pid
    }

    /// Whether the environment can take another invocation.
    pub(crate) fn is_usable(&self) -> bool {
        !self.poisoned && self.exit.borrow().is_none()
    }

    /// Run one invocation to completion.
    pub(crate) async fn invoke(&mut self, call: Call) -> InvocationOutcome {
        let started = Instant::now();
        let request_id = call.request_id.clone();
        let deadline_ms =
            chrono::Utc::now().timestamp_millis().max(0) as u64 + call.timeout.as_millis() as u64;
        let submitted = self.api.submit(PendingInvocation {
            request_id: request_id.clone(),
            payload: call.payload.clone(),
            deadline_ms,
            invoked_function_arn: call.invoked_function_arn.clone(),
            trace_id: trace_id(),
            client_context: decoded_client_context(call.client_context.as_deref()),
        });
        let (payload, error) = match submitted {
            Ok(rx) => self.wait(rx, &call).await,
            Err(SubmitError::InitFailed) => {
                self.poisoned = true;
                let (_, body) = self.api.init_error().unwrap_or_default();
                (body, true)
            }
            Err(SubmitError::Busy) => {
                self.poisoned = true;
                (
                    runtime_error(&request_id, "Runtime.ExitError", "The environment is busy"),
                    true,
                )
            }
        };
        if !self.poisoned {
            self.last_used = Instant::now();
        }
        tokio::time::sleep(LOG_GRACE).await;
        let output = self.console.take();
        let log_tail = frame_log(
            &request_id,
            &call.function_version,
            &output,
            started.elapsed(),
            call.memory_size,
        );
        tracing::debug!(
            %request_id,
            function_error = error,
            "Lambda invocation finished\n{}",
            String::from_utf8_lossy(&log_tail)
        );
        if error {
            InvocationOutcome::unhandled(payload, log_tail)
        } else {
            InvocationOutcome::success(payload, log_tail)
        }
    }

    /// The payload and whether it is an error document.
    async fn wait(
        &mut self,
        mut rx: oneshot::Receiver<RuntimeOutcome>,
        call: &Call,
    ) -> (Bytes, bool) {
        let request_id = &call.request_id;
        let mut exit = self.exit.clone();
        let mut handed_out = self.api.handed_out();

        // Until the runtime picks the invocation up it is initialising ( on
        // a new environment ) and the function's timeout has not started.
        let picked_up = tokio::time::timeout(self.init_timeout, async {
            tokio::select! {
                _ = handed_out.wait_for(|id| id.as_deref() == Some(request_id.as_str())) => None,
                outcome = &mut rx => Some(outcome.ok()),
                _ = exit.wait_for(Option::is_some) => Some(rx.try_recv().ok()),
            }
        })
        .await;
        let early = match picked_up {
            Ok(None) => None,
            Ok(Some(outcome)) => Some(outcome),
            Err(_) => {
                self.poisoned = true;
                self.api.abandon();
                return (
                    runtime_error(
                        request_id,
                        "Runtime.Unknown",
                        &format!(
                            "The runtime did not initialise within {:.2} seconds",
                            self.init_timeout.as_secs_f64()
                        ),
                    ),
                    true,
                );
            }
        };

        let outcome = match early {
            Some(outcome) => outcome,
            None => tokio::select! {
                outcome = &mut rx => outcome.ok(),
                // A result posted just before the exit still counts.
                _ = exit.wait_for(Option::is_some) => rx.try_recv().ok(),
                _ = tokio::time::sleep(call.timeout) => {
                    self.poisoned = true;
                    self.api.abandon();
                    self.kill.take();
                    let seconds = call.timeout.as_secs_f64();
                    self.console.push(
                        format!(
                            "{} {request_id} Task timed out after {seconds:.2} seconds\n",
                            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ"),
                        )
                        .as_bytes(),
                    );
                    return (
                        runtime_error(
                            request_id,
                            "Sandbox.Timedout",
                            &format!("Task timed out after {seconds:.2} seconds"),
                        ),
                        true,
                    );
                }
            },
        };

        match outcome {
            Some(RuntimeOutcome::Response(body)) => (body, false),
            Some(RuntimeOutcome::FunctionError { body, .. }) => (body, true),
            Some(RuntimeOutcome::InitError { body, .. }) => {
                self.poisoned = true;
                (body, true)
            }
            Some(RuntimeOutcome::ResponseTooLarge) => (
                Bytes::from(
                    serde_json::json!({
                        "errorMessage": format!(
                            "Response payload size exceeded maximum allowed payload size ({} bytes).",
                            crate::runtime_api::MAX_RESPONSE_BYTES
                        ),
                        "errorType": "Function.ResponseSizeTooLarge",
                    })
                    .to_string(),
                ),
                true,
            ),
            None => {
                self.poisoned = true;
                // Let the supervisor record the exit status.
                let _ = tokio::time::timeout(Duration::from_secs(1), exit.wait_for(Option::is_some)).await;
                let code = exit.borrow().and_then(|e| e.code);
                let never_polled = *self.api.polls().borrow() == 0;
                if never_polled && code == Some(AGENT_FAILED_EXIT) {
                    return (
                        runtime_error(
                            request_id,
                            "Runtime.ExitError",
                            "winterbaume: the guest agent did not start",
                        ),
                        true,
                    );
                }
                if never_polled && code == Some(NO_BOOTSTRAP_EXIT) {
                    return (
                        runtime_error(
                            request_id,
                            "Runtime.InvalidEntrypoint",
                            "Couldn't find valid bootstrap(s)",
                        ),
                        true,
                    );
                }
                let message = match code {
                    Some(0) => "Runtime exited without providing a reason".to_string(),
                    Some(code) => format!("Runtime exited with error: exit status {code}"),
                    None => "Runtime exited with error: signal: killed".to_string(),
                };
                (runtime_error(request_id, "Runtime.ExitError", &message), true)
            }
        }
    }
}

async fn supervise(
    mut child: tokio::process::Child,
    killed: oneshot::Receiver<()>,
    exit: watch::Sender<Option<ExitInfo>>,
) {
    let status = tokio::select! {
        status = child.wait() => status.ok(),
        _ = killed => {
            let _ = child.kill().await;
            child.wait().await.ok()
        }
    };
    let code = status.and_then(|s| s.code());
    exit.send_replace(Some(ExitInfo { code }));
}

/// Copy a helper stream into the console.  On stderr the helper's status
/// line comes first and is not console output.
async fn read_console(
    mut stream: impl AsyncRead + Unpin,
    console: Arc<Console>,
    status: Option<watch::Sender<Option<HelperStatus>>>,
) {
    let mut buffer = vec![0u8; 16 * 1024];
    let mut pending_status = status.map(|tx| (tx, Vec::<u8>::new()));
    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        let mut chunk = &buffer[..n];
        if let Some((tx, line)) = pending_status.as_mut() {
            match chunk.iter().position(|b| *b == b'\n') {
                Some(end) => {
                    line.extend_from_slice(&chunk[..=end]);
                    chunk = &chunk[end + 1..];
                    let text = String::from_utf8_lossy(line).to_string();
                    match HelperStatus::parse(&text) {
                        Some(parsed) => {
                            tx.send_replace(Some(parsed));
                        }
                        // Not a status line: it is output.
                        None => console.push(line),
                    }
                    pending_status = None;
                }
                None => {
                    line.extend_from_slice(chunk);
                    if !STATUS_PREFIX
                        .as_bytes()
                        .starts_with(&line[..line.len().min(STATUS_PREFIX.len())])
                    {
                        console.push(line);
                        pending_status = None;
                    }
                    continue;
                }
            }
        }
        console.push(chunk);
    }
}

fn trace_id() -> String {
    let id = uuid::Uuid::new_v4().simple().to_string();
    format!(
        "Root=1-{:08x}-{};Parent={};Sampled=0",
        chrono::Utc::now().timestamp(),
        &id[..24],
        &id[16..32]
    )
}

fn decoded_client_context(header: Option<&str>) -> Option<String> {
    winterbaume_lambda::execution::decode_client_context(header).map(|v| v.to_string())
}

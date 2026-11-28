//! The Lambda Runtime API, served on the host for one execution environment.
//!
//! The runtime interface client inside the guest polls
//! `GET /2018-06-01/runtime/invocation/next` for work and posts each result
//! to `/invocation/<request id>/response` or `/error`; a runtime that cannot
//! initialise posts `/init/error`.  `AWS_LAMBDA_RUNTIME_API` holds only
//! `host:port`, so every environment gets a listener of its own: a Unix
//! socket on the host, which the guest agent reaches over vsock from guest
//! `127.0.0.1:9001`.

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full, Limited};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::UnixListener;
use tokio::sync::{Notify, oneshot, watch};

/// The most a function may return synchronously ( 6 MB ).
pub const MAX_RESPONSE_BYTES: usize = 6 * 1024 * 1024;

const PREFIX: &str = "/2018-06-01/runtime";

/// An invocation waiting to be picked up by the runtime.
#[derive(Debug, Clone)]
pub struct PendingInvocation {
    pub request_id: String,
    pub payload: Bytes,
    /// Epoch milliseconds.
    pub deadline_ms: u64,
    pub invoked_function_arn: String,
    pub trace_id: String,
    /// Client context as JSON text ( already decoded from base64 ).
    pub client_context: Option<String>,
}

/// How an invocation ended, as far as the runtime reported it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeOutcome {
    /// `POST /invocation/<id>/response`.
    Response(Bytes),
    /// `POST /invocation/<id>/error`.
    FunctionError {
        error_type: Option<String>,
        body: Bytes,
    },
    /// `POST /init/error`: the runtime could not initialise.
    InitError {
        error_type: Option<String>,
        body: Bytes,
    },
    /// The response exceeded [`MAX_RESPONSE_BYTES`].
    ResponseTooLarge,
}

#[derive(Default)]
struct Inner {
    queued: Option<(PendingInvocation, oneshot::Sender<RuntimeOutcome>)>,
    in_flight: Option<(String, oneshot::Sender<RuntimeOutcome>)>,
    init_error: Option<(Option<String>, Bytes)>,
}

struct Shared {
    inner: Mutex<Inner>,
    queued: Notify,
    /// Incremented on every `GET /next`.  The first one means the runtime
    /// has initialised; each later one that the runtime is ready again.
    polls: watch::Sender<u64>,
    /// The id of the invocation most recently handed to the runtime.
    handed_out: watch::Sender<Option<String>>,
}

impl Shared {
    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// A Runtime API endpoint for one execution environment.  Dropping it stops
/// the listener.
pub struct RuntimeApi {
    path: PathBuf,
    shared: Arc<Shared>,
    server: tokio::task::JoinHandle<()>,
}

impl Drop for RuntimeApi {
    fn drop(&mut self) {
        self.server.abort();
    }
}

/// The submitted invocation was not accepted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubmitError {
    #[error("an invocation is already pending")]
    Busy,
    #[error("the runtime failed to initialise")]
    InitFailed,
}

impl RuntimeApi {
    /// Listen on the Unix socket `path`, which must not exist yet.
    pub async fn bind(path: &Path) -> std::io::Result<Self> {
        let listener = UnixListener::bind(path)?;
        let shared = Arc::new(Shared {
            inner: Mutex::new(Inner::default()),
            queued: Notify::new(),
            polls: watch::channel(0).0,
            handed_out: watch::channel(None).0,
        });
        let server = tokio::spawn(serve(listener, shared.clone()));
        Ok(Self {
            path: path.to_path_buf(),
            shared,
            server,
        })
    }

    /// The socket this endpoint listens on.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Queue an invocation for the runtime's next poll.  The receiver yields
    /// its outcome; it is dropped unanswered if the environment goes away.
    pub fn submit(
        &self,
        invocation: PendingInvocation,
    ) -> Result<oneshot::Receiver<RuntimeOutcome>, SubmitError> {
        let mut inner = self.shared.lock();
        if inner.init_error.is_some() {
            return Err(SubmitError::InitFailed);
        }
        if inner.queued.is_some() || inner.in_flight.is_some() {
            return Err(SubmitError::Busy);
        }
        let (tx, rx) = oneshot::channel();
        inner.queued = Some((invocation, tx));
        drop(inner);
        self.shared.queued.notify_one();
        Ok(rx)
    }

    /// Withdraw an invocation the runtime has not answered ( after a
    /// timeout ).
    pub fn abandon(&self) {
        let mut inner = self.shared.lock();
        inner.queued = None;
        inner.in_flight = None;
    }

    /// Counts `GET /next` polls.
    pub fn polls(&self) -> watch::Receiver<u64> {
        self.shared.polls.subscribe()
    }

    /// The request id most recently handed to the runtime.
    pub fn handed_out(&self) -> watch::Receiver<Option<String>> {
        self.shared.handed_out.subscribe()
    }

    /// What the runtime posted to `/init/error`, if it did.
    pub fn init_error(&self) -> Option<(Option<String>, Bytes)> {
        self.shared.lock().init_error.clone()
    }
}

async fn serve(listener: UnixListener, shared: Arc<Shared>) {
    loop {
        let Ok((stream, _)) = listener.accept().await else {
            continue;
        };
        let shared = shared.clone();
        tokio::spawn(async move {
            let io = hyper_util::rt::TokioIo::new(stream);
            let service = service_fn(move |request| {
                let shared = shared.clone();
                async move { Ok::<_, Infallible>(route(&shared, request).await) }
            });
            // `/next` long-polls; keep-alive lets the runtime reuse the
            // connection, as it does with the real service.
            let _ = http1::Builder::new()
                .keep_alive(true)
                .serve_connection(io, service)
                .await;
        });
    }
}

fn json_response(status: StatusCode, body: serde_json::Value) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .expect("static response parts are valid")
}

fn error_response(status: StatusCode, error_type: &str, message: &str) -> Response<Full<Bytes>> {
    json_response(
        status,
        serde_json::json!({ "errorMessage": message, "errorType": error_type }),
    )
}

fn accepted() -> Response<Full<Bytes>> {
    json_response(StatusCode::ACCEPTED, serde_json::json!({ "status": "OK" }))
}

async fn route(shared: &Shared, request: Request<Incoming>) -> Response<Full<Bytes>> {
    let path = request.uri().path().to_string();
    let Some(rest) = path.strip_prefix(PREFIX) else {
        return error_response(StatusCode::NOT_FOUND, "NotFound", "Not found");
    };
    let segments: Vec<&str> = rest.trim_start_matches('/').split('/').collect();
    match (request.method(), segments.as_slice()) {
        (&Method::GET, ["invocation", "next"]) => next(shared).await,
        (&Method::POST, ["invocation", id, "response"]) => {
            let id = id.to_string();
            match read_body(request).await {
                Ok(body) => complete(shared, &id, RuntimeOutcome::Response(body)),
                Err(BodyError::TooLarge) => {
                    // The invoker gets the error; the runtime is told too.
                    let _ = complete(shared, &id, RuntimeOutcome::ResponseTooLarge);
                    error_response(
                        StatusCode::PAYLOAD_TOO_LARGE,
                        "RequestEntityTooLarge",
                        &format!(
                            "Exceeded maximum allowed payload size ({MAX_RESPONSE_BYTES} bytes)."
                        ),
                    )
                }
                Err(BodyError::Broken) => {
                    error_response(StatusCode::BAD_REQUEST, "InvalidRequest", "Broken body")
                }
            }
        }
        (&Method::POST, ["invocation", id, "error"]) => {
            let id = id.to_string();
            let error_type = error_type_header(&request);
            let body = read_body(request).await.unwrap_or_default();
            complete(
                shared,
                &id,
                RuntimeOutcome::FunctionError { error_type, body },
            )
        }
        (&Method::POST, ["init", "error"]) => {
            let error_type = error_type_header(&request);
            let body = read_body(request).await.unwrap_or_default();
            let mut inner = shared.lock();
            inner.init_error = Some((error_type.clone(), body.clone()));
            let outcome = RuntimeOutcome::InitError { error_type, body };
            if let Some((_, tx)) = inner.queued.take() {
                let _ = tx.send(outcome.clone());
            }
            if let Some((_, tx)) = inner.in_flight.take() {
                let _ = tx.send(outcome);
            }
            accepted()
        }
        _ => error_response(StatusCode::NOT_FOUND, "NotFound", "Not found"),
    }
}

fn error_type_header(request: &Request<Incoming>) -> Option<String> {
    request
        .headers()
        .get("lambda-runtime-function-error-type")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
}

enum BodyError {
    TooLarge,
    Broken,
}

async fn read_body(request: Request<Incoming>) -> Result<Bytes, BodyError> {
    match Limited::new(request.into_body(), MAX_RESPONSE_BYTES)
        .collect()
        .await
    {
        Ok(collected) => Ok(collected.to_bytes()),
        Err(e)
            if e.downcast_ref::<http_body_util::LengthLimitError>()
                .is_some() =>
        {
            Err(BodyError::TooLarge)
        }
        Err(_) => Err(BodyError::Broken),
    }
}

fn complete(shared: &Shared, id: &str, outcome: RuntimeOutcome) -> Response<Full<Bytes>> {
    let mut inner = shared.lock();
    match inner.in_flight.take() {
        Some((in_flight, tx)) if in_flight == id => {
            let _ = tx.send(outcome);
            accepted()
        }
        other => {
            inner.in_flight = other;
            error_response(
                StatusCode::BAD_REQUEST,
                "InvalidRequestID",
                "Invalid request ID",
            )
        }
    }
}

async fn next(shared: &Shared) -> Response<Full<Bytes>> {
    shared.polls.send_modify(|n| *n += 1);
    loop {
        let notified = shared.queued.notified();
        {
            let mut inner = shared.lock();
            if let Some((invocation, tx)) = inner.queued.take() {
                inner.in_flight = Some((invocation.request_id.clone(), tx));
                drop(inner);
                shared
                    .handed_out
                    .send_replace(Some(invocation.request_id.clone()));
                return invocation_response(&invocation);
            }
        }
        notified.await;
    }
}

fn invocation_response(invocation: &PendingInvocation) -> Response<Full<Bytes>> {
    let mut builder = Response::builder()
        .status(StatusCode::OK)
        // The stock runtime interface clients need this; without it the
        // Node.js one crashes.
        .header(http::header::CONTENT_TYPE, "application/json")
        .header("Lambda-Runtime-Aws-Request-Id", &invocation.request_id)
        .header(
            "Lambda-Runtime-Deadline-Ms",
            invocation.deadline_ms.to_string(),
        )
        .header(
            "Lambda-Runtime-Invoked-Function-Arn",
            &invocation.invoked_function_arn,
        )
        .header("Lambda-Runtime-Trace-Id", &invocation.trace_id);
    if let Some(context) = invocation
        .client_context
        .as_deref()
        .filter(|c| http::HeaderValue::from_str(c).is_ok())
    {
        builder = builder.header("Lambda-Runtime-Client-Context", context);
    }
    builder
        .body(Full::new(invocation.payload.clone()))
        .unwrap_or_else(|_| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "InvalidInvocation",
                "Invocation headers are not valid",
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    /// A raw HTTP/1.1 exchange on a fresh connection.
    async fn http(
        address: &Path,
        method: &str,
        path: &str,
        headers: &[(&str, &str)],
        body: &[u8],
    ) -> (u16, String, Vec<u8>) {
        let mut stream = tokio::net::UnixStream::connect(address).await.unwrap();
        let mut head = format!(
            "{method} {path} HTTP/1.1\r\nHost: x\r\nConnection: close\r\nContent-Length: {}\r\n",
            body.len()
        );
        for (k, v) in headers {
            head.push_str(&format!("{k}: {v}\r\n"));
        }
        head.push_str("\r\n");
        stream.write_all(head.as_bytes()).await.unwrap();
        stream.write_all(body).await.unwrap();
        let mut raw = Vec::new();
        stream.read_to_end(&mut raw).await.unwrap();
        let split = raw.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
        let head = String::from_utf8_lossy(&raw[..split]).to_string();
        let status = head[9..12].parse().unwrap();
        (status, head, raw[split + 4..].to_vec())
    }

    fn invocation(id: &str) -> PendingInvocation {
        PendingInvocation {
            request_id: id.to_string(),
            payload: Bytes::from_static(br#"{"k":1}"#),
            deadline_ms: 1_700_000_000_000,
            invoked_function_arn: "arn:aws:lambda:us-east-1:123456789012:function:f".to_string(),
            trace_id: "Root=1-0-0".to_string(),
            client_context: Some(r#"{"custom":{}}"#.to_string()),
        }
    }

    #[tokio::test]
    async fn next_long_polls_and_hands_out_the_invocation_with_its_headers() {
        let dir = tempfile::tempdir().unwrap();
        let api = RuntimeApi::bind(&dir.path().join("rapi.sock"))
            .await
            .unwrap();
        let address = api.path().to_path_buf();
        let polled = address.clone();
        let poll = tokio::spawn(async move {
            http(
                &polled,
                "GET",
                "/2018-06-01/runtime/invocation/next",
                &[],
                b"",
            )
            .await
        });
        let mut polls = api.polls();
        polls.wait_for(|n| *n == 1).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(!poll.is_finished(), "next waits for work");

        let rx = api.submit(invocation("r-1")).unwrap();
        let (status, head, body) = poll.await.unwrap();
        assert_eq!(status, 200);
        let head = head.to_ascii_lowercase();
        assert!(head.contains("content-type: application/json"));
        assert!(head.contains("lambda-runtime-aws-request-id: r-1"));
        assert!(head.contains("lambda-runtime-deadline-ms: 1700000000000"));
        assert!(head.contains(
            "lambda-runtime-invoked-function-arn: arn:aws:lambda:us-east-1:123456789012:function:f"
        ));
        assert!(head.contains("lambda-runtime-trace-id: root=1-0-0"));
        assert!(head.contains(r#"lambda-runtime-client-context: {"custom":{}}"#));
        assert_eq!(body, br#"{"k":1}"#);
        assert_eq!(*api.handed_out().borrow(), Some("r-1".to_string()));

        // A second submission while one is in flight is refused.
        assert_eq!(
            api.submit(invocation("r-2")).unwrap_err(),
            SubmitError::Busy
        );

        let (status, _, body) = http(
            &address,
            "POST",
            "/2018-06-01/runtime/invocation/r-1/response",
            &[],
            b"\"done\"",
        )
        .await;
        assert_eq!(status, 202);
        assert_eq!(body, br#"{"status":"OK"}"#);
        assert_eq!(
            rx.await.unwrap(),
            RuntimeOutcome::Response(Bytes::from_static(b"\"done\""))
        );
    }

    #[tokio::test]
    async fn function_errors_carry_their_type_and_document() {
        let dir = tempfile::tempdir().unwrap();
        let api = RuntimeApi::bind(&dir.path().join("rapi.sock"))
            .await
            .unwrap();
        let rx = api.submit(invocation("r-e")).unwrap();
        http(
            api.path(),
            "GET",
            "/2018-06-01/runtime/invocation/next",
            &[],
            b"",
        )
        .await;
        let document = br#"{"errorType":"TypeError","errorMessage":"x is not a function"}"#;
        let (status, _, _) = http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/invocation/r-e/error",
            &[("Lambda-Runtime-Function-Error-Type", "Runtime.TypeError")],
            document,
        )
        .await;
        assert_eq!(status, 202);
        assert_eq!(
            rx.await.unwrap(),
            RuntimeOutcome::FunctionError {
                error_type: Some("Runtime.TypeError".to_string()),
                body: Bytes::from_static(document),
            }
        );
    }

    #[tokio::test]
    async fn unknown_and_stale_request_ids_are_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let api = RuntimeApi::bind(&dir.path().join("rapi.sock"))
            .await
            .unwrap();
        let (status, _, body) = http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/invocation/nope/response",
            &[],
            b"1",
        )
        .await;
        assert_eq!(status, 400);
        assert!(String::from_utf8_lossy(&body).contains("InvalidRequestID"));

        let rx = api.submit(invocation("r-s")).unwrap();
        http(
            api.path(),
            "GET",
            "/2018-06-01/runtime/invocation/next",
            &[],
            b"",
        )
        .await;
        let (status, _, _) = http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/invocation/other/response",
            &[],
            b"1",
        )
        .await;
        assert_eq!(status, 400);
        // The real one still completes.
        http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/invocation/r-s/response",
            &[],
            b"2",
        )
        .await;
        assert_eq!(
            rx.await.unwrap(),
            RuntimeOutcome::Response(Bytes::from_static(b"2"))
        );
        // Answering twice is refused.
        let (status, _, _) = http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/invocation/r-s/response",
            &[],
            b"3",
        )
        .await;
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn oversized_responses_are_refused_with_413() {
        let dir = tempfile::tempdir().unwrap();
        let api = RuntimeApi::bind(&dir.path().join("rapi.sock"))
            .await
            .unwrap();
        let rx = api.submit(invocation("r-big")).unwrap();
        http(
            api.path(),
            "GET",
            "/2018-06-01/runtime/invocation/next",
            &[],
            b"",
        )
        .await;
        let big = vec![b'x'; MAX_RESPONSE_BYTES + 1];
        let (status, _, _) = http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/invocation/r-big/response",
            &[],
            &big,
        )
        .await;
        assert_eq!(status, 413);
        assert_eq!(rx.await.unwrap(), RuntimeOutcome::ResponseTooLarge);
    }

    #[tokio::test]
    async fn init_errors_fail_the_pending_invocation_and_later_submissions() {
        let dir = tempfile::tempdir().unwrap();
        let api = RuntimeApi::bind(&dir.path().join("rapi.sock"))
            .await
            .unwrap();
        let rx = api.submit(invocation("r-i")).unwrap();
        let (status, _, _) = http(
            api.path(),
            "POST",
            "/2018-06-01/runtime/init/error",
            &[(
                "Lambda-Runtime-Function-Error-Type",
                "Runtime.ImportModuleError",
            )],
            br#"{"errorType":"Runtime.ImportModuleError"}"#,
        )
        .await;
        assert_eq!(status, 202);
        assert!(matches!(
            rx.await.unwrap(),
            RuntimeOutcome::InitError { error_type: Some(t), .. } if t == "Runtime.ImportModuleError"
        ));
        assert_eq!(
            api.submit(invocation("r-j")).unwrap_err(),
            SubmitError::InitFailed
        );
        assert!(api.init_error().is_some());
    }

    #[tokio::test]
    async fn unknown_paths_are_404() {
        let dir = tempfile::tempdir().unwrap();
        let api = RuntimeApi::bind(&dir.path().join("rapi.sock"))
            .await
            .unwrap();
        let (status, _, _) = http(api.path(), "GET", "/2018-06-01/runtime/x", &[], b"").await;
        assert_eq!(status, 404);
        let (status, _, _) = http(api.path(), "GET", "/other", &[], b"").await;
        assert_eq!(status, 404);
    }
}

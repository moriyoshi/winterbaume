//! Shared test harness: in-process HTTP server and terraform CLI helpers.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;
use tokio::process::Command;
use winterbaume_core::service::{MockRequest, MockResponse, MockService};

// ---------------------------------------------------------------------------
// In-process HTTP server (mirrors winterbaume-server logic)
// ---------------------------------------------------------------------------

struct ServiceRoute {
    pattern: Regex,
    service: Arc<dyn MockService>,
}

struct Router {
    routes: Vec<ServiceRoute>,
}

impl Router {
    fn new(services: Vec<Arc<dyn MockService>>) -> Self {
        let mut routes = Vec::new();
        for service in services {
            for pattern in service.url_patterns() {
                routes.push(ServiceRoute {
                    pattern: Regex::new(pattern)
                        .unwrap_or_else(|e| panic!("Invalid URL pattern '{pattern}': {e}")),
                    service: Arc::clone(&service),
                });
            }
        }
        Self { routes }
    }

    fn find_service(&self, uri: &str, headers: &http::HeaderMap) -> Option<Arc<dyn MockService>> {
        for route in &self.routes {
            if route.pattern.is_match(uri) {
                return Some(Arc::clone(&route.service));
            }
        }
        if let Some(name) = winterbaume_core::auth::extract_service_from_uri(uri) {
            if let Some(route) = self
                .routes
                .iter()
                .find(|r| r.service.service_name() == name)
            {
                return Some(Arc::clone(&route.service));
            }
        }
        if let Some(name) = winterbaume_core::auth::extract_service_from_headers(headers) {
            if let Some(route) = self
                .routes
                .iter()
                .find(|r| r.service.service_name() == name)
            {
                return Some(Arc::clone(&route.service));
            }
        }
        None
    }
}

async fn handle_request(
    router: Arc<Router>,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().to_string();
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost");
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");
    let full_uri = format!("https://{host}{path_and_query}");

    let mut headers = http::HeaderMap::new();
    for (name, value) in req.headers() {
        headers.insert(name.clone(), value.clone());
    }

    let body_bytes = req
        .into_body()
        .collect()
        .await
        .map(|c| c.to_bytes())
        .unwrap_or_default();

    let mock_request = MockRequest {
        method,
        uri: full_uri.clone(),
        headers,
        body: body_bytes,
    };

    let mock_response = match router.find_service(&full_uri, &mock_request.headers) {
        Some(service) => service.handle(mock_request).await,
        None => MockResponse::json(
            404,
            format!(
                r#"{{"__type":"UnknownService","message":"No service registered for: {full_uri}"}}"#,
            ),
        ),
    };

    let mut response = Response::builder().status(mock_response.status);
    for (name, value) in &mock_response.headers {
        response = response.header(name, value);
    }
    Ok(response
        .body(Full::new(mock_response.body))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(500)
                .body(Full::new(Bytes::from("Internal Server Error")))
                .unwrap()
        }))
}

/// Start an in-process HTTP server on a random port.
/// Returns the port number. The server runs until the runtime is dropped.
pub async fn start_server(services: Vec<Arc<dyn MockService>>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let router = Arc::new(Router::new(services));

    tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(conn) => conn,
                Err(_) => break,
            };
            let io = TokioIo::new(stream);
            let router = Arc::clone(&router);
            tokio::spawn(async move {
                let service = service_fn(move |req| {
                    let router = Arc::clone(&router);
                    async move { handle_request(router, req).await }
                });
                let _ = http1::Builder::new().serve_connection(io, service).await;
            });
        }
    });

    port
}

/// Create the set of services for terraform testing.
pub fn test_services() -> Vec<Arc<dyn MockService>> {
    vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::new(winterbaume_s3::S3Service::new()),
        Arc::new(winterbaume_iam::IamService::new()),
        Arc::new(winterbaume_sqs::SqsService::new()),
        Arc::new(winterbaume_dynamodb::DynamoDbService::new()),
        Arc::new(winterbaume_kms::KmsService::new()),
        Arc::new(winterbaume_sns::SnsService::new()),
        Arc::new(winterbaume_lambda::LambdaService::new()),
        Arc::new(winterbaume_secretsmanager::SecretsManagerService::new()),
        Arc::new(winterbaume_cloudwatchlogs::CloudWatchLogsService::new()),
        Arc::new(winterbaume_ssm::SsmService::new()),
        Arc::new(winterbaume_ecr::EcrService::new()),
        Arc::new(winterbaume_eventbridge::EventBridgeService::new()),
        Arc::new(winterbaume_firehose::FirehoseService::new()),
        Arc::new(winterbaume_cloudwatch::CloudWatchService::new()),
        Arc::new(winterbaume_sfn::SfnService::new()),
        Arc::new(winterbaume_kinesis::KinesisService::new()),
        Arc::new(winterbaume_cognitoidentityprovider::CognitoIdentityProviderService::new()),
        Arc::new(winterbaume_ecs::EcsService::new()),
        Arc::new(winterbaume_route53::Route53Service::new()),
        Arc::new(winterbaume_efs::EfsService::new()),
        Arc::new(winterbaume_acm::AcmService::new()),
        Arc::new(winterbaume_cloudfront::CloudFrontService::new()),
        Arc::new(winterbaume_wafv2::WafV2Service::new()),
        Arc::new(winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service::new()),
        Arc::new(winterbaume_eks::EksService::new()),
        Arc::new(winterbaume_organizations::OrganizationsService::new()),
        Arc::new(winterbaume_ec2::Ec2Service::new()),
        Arc::new(winterbaume_neptune::NeptuneService::new()),
        Arc::new(winterbaume_rds::RdsService::new()),
        Arc::new(winterbaume_redshift::RedshiftService::new()),
        Arc::new(winterbaume_appconfig::AppConfigService::new()),
        Arc::new(winterbaume_appfabric::AppFabricService::new()),
        Arc::new(winterbaume_appflow::AppFlowService::new()),
        Arc::new(winterbaume_applicationcostprofiler::ApplicationCostProfilerService::new()),
        Arc::new(winterbaume_codecommit::CodeCommitService::new()),
        Arc::new(winterbaume_transfer::TransferService::new()),
        Arc::new(winterbaume_workspaces::WorkSpacesService::new()),
        Arc::new(winterbaume_networkmanager::NetworkManagerService::new()),
        Arc::new(winterbaume_bedrockagent::BedrockAgentService::new()),
        Arc::new(winterbaume_batch::BatchService::new()),
        Arc::new(winterbaume_vpclattice::VpcLatticeService::new()),
        Arc::new(winterbaume_config::ConfigService::new()),
        Arc::new(winterbaume_apigatewayv2::ApiGatewayV2Service::new()),
        Arc::new(winterbaume_autoscaling::AutoScalingService::new()),
        Arc::new(winterbaume_scheduler::SchedulerService::new()),
        Arc::new(winterbaume_directconnect::DirectConnectService::new()),
        Arc::new(winterbaume_guardduty::GuardDutyService::new()),
        Arc::new(winterbaume_inspector2::Inspector2Service::new()),
        Arc::new(winterbaume_iot::IotService::new()),
        Arc::new(winterbaume_keyspaces::KeyspacesService::new()),
        Arc::new(winterbaume_medialive::MediaLiveService::new()),
        Arc::new(winterbaume_mediapackage::MediaPackageService::new()),
        Arc::new(winterbaume_mediastore::MediaStoreService::new()),
        Arc::new(winterbaume_opensearchserverless::OpenSearchServerlessService::new()),
        Arc::new(winterbaume_pinpoint::PinpointService::new()),
        Arc::new(winterbaume_quicksight::QuickSightService::new()),
    ]
}

// ---------------------------------------------------------------------------
// Terraform helpers
// ---------------------------------------------------------------------------

/// Path to the workspace root.
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("failed to resolve workspace root from CARGO_MANIFEST_DIR"))
        .to_path_buf()
}

/// Root directory for all terraform temporary files.
///
/// Defaults to the system temporary directory (`std::env::temp_dir()`).
/// Override by setting the `WINTERBAUME_TF_TMP_DIR` environment variable.
fn tf_tmp_root() -> PathBuf {
    std::env::var_os("WINTERBAUME_TF_TMP_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

/// Create a temporary directory for terraform configs.
pub fn create_tf_dir(test_name: &str) -> PathBuf {
    let dir = tf_tmp_root().join(format!("tf-test-{test_name}"));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).ok();
    }
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Write the AWS provider configuration pointing at the given server URL.
pub fn write_provider_tf(dir: &Path, server_url: &str) {
    // FIX(terraform-e2e): AWS SDK prepends the account-ID to the s3control endpoint hostname
    //   (e.g. "http://123456789012.127.0.0.1:PORT"), which cannot be resolved by DNS.
    //   "123456789012.localhost" resolves on macOS via mDNS, so we substitute "localhost"
    //   for "127.0.0.1" in the s3control endpoint.
    let s3control_url = server_url.replace("127.0.0.1", "localhost");
    let content = format!(
        r#"terraform {{
  required_providers {{
    aws = {{
      source  = "hashicorp/aws"
      version = "~> 6.0"
    }}
  }}
}}

provider "aws" {{
  region                      = "us-east-1"
  s3_use_path_style           = true
  skip_credentials_validation = true
  skip_metadata_api_check     = true

  endpoints {{
    s3             = "{server_url}"
    iam            = "{server_url}"
    sqs            = "{server_url}"
    sts            = "{server_url}"
    dynamodb       = "{server_url}"
    kms            = "{server_url}"
    sns            = "{server_url}"
    lambda         = "{server_url}"
    secretsmanager = "{server_url}"
    cloudwatchlogs = "{server_url}"
    ssm            = "{server_url}"
    ecr            = "{server_url}"
    eventbridge    = "{server_url}"
    cloudwatch     = "{server_url}"
    sfn            = "{server_url}"
    firehose       = "{server_url}"
    kinesis        = "{server_url}"
    cognitoidp     = "{server_url}"
    ecs            = "{server_url}"
    route53        = "{server_url}"
    efs            = "{server_url}"
    acm            = "{server_url}"
    cloudfront     = "{server_url}"
    wafv2          = "{server_url}"
    elbv2          = "{server_url}"
    eks            = "{server_url}"
    organizations  = "{server_url}"
    ec2            = "{server_url}"
    neptune        = "{server_url}"
    rds            = "{server_url}"
    redshift       = "{server_url}"
    appconfig      = "{server_url}"
    appfabric      = "{server_url}"
    appflow        = "{server_url}"
    codecommit     = "{server_url}"
    transfer       = "{server_url}"
    workspaces     = "{server_url}"
    networkmanager = "{server_url}"
    bedrockagent   = "{server_url}"
    batch          = "{server_url}"
    configservice  = "{server_url}"
    vpclattice     = "{server_url}"
    apigatewayv2   = "{server_url}"
    autoscaling    = "{server_url}"
    scheduler      = "{server_url}"
    directconnect  = "{server_url}"
    guardduty      = "{server_url}"
    inspector2     = "{server_url}"
    iot            = "{server_url}"
    keyspaces      = "{server_url}"
    medialive      = "{server_url}"
    mediapackage   = "{server_url}"
    mediastore     = "{server_url}"
    opensearchserverless = "{server_url}"
    pinpoint       = "{server_url}"
    quicksight     = "{server_url}"
    s3control      = "{s3control_url}"
  }}

  access_key = "test"
  secret_key = "test"
}}
"#
    );
    std::fs::write(dir.join("provider.tf"), content).unwrap();
}

/// Shared plugin cache directory to avoid re-downloading the AWS provider for each test.
fn plugin_cache_dir() -> PathBuf {
    let dir = tf_tmp_root().join("tf-plugin-cache");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Default timeout for terraform operations (init has no timeout since it may
/// need to download the ~884 MB provider binary on first run).
///
/// Batch waves can accumulate up to ~120 resources from multiple services.
/// With -parallelism=50, these apply in ~60–90 s on a dev machine; 360 s
/// gives a 4× safety margin so flaky-slow machines don't trigger false timeouts.
const DEFAULT_APPLY_TIMEOUT: Duration = Duration::from_secs(360);

/// Run a terraform command with a timeout. On timeout, the process is killed and
/// the TF_LOG=TRACE output is saved to `.agents-workspace/tmp/tf-logs/` for debugging.
/// On success, the trace file is deleted.
///
/// Returns (success, stdout, stderr). On timeout, success is false and stderr
/// contains a message pointing to the saved trace file.
pub async fn run_terraform(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let (ok, stdout, stderr, _trace) =
        run_terraform_raw(dir, args, DEFAULT_APPLY_TIMEOUT, false).await;
    (ok, stdout, stderr)
}

pub async fn run_terraform_with_timeout(
    dir: &Path,
    args: &[&str],
    timeout: Duration,
) -> (bool, String, String) {
    // Enable trace so we can produce a diagnostic summary on timeout.
    let (ok, stdout, stderr, trace_path) = run_terraform_raw(dir, args, timeout, true).await;
    if ok {
        if let Some(ref p) = trace_path {
            let _ = std::fs::remove_file(p);
        }
    }
    (ok, stdout, stderr)
}

/// Low-level terraform runner. Returns (success, stdout, stderr, trace_path).
///
/// When `enable_trace` is false, terraform runs without TF_LOG=TRACE, which is
/// much cheaper (no 50+ MB log files, no disk I/O overhead). Use this for normal
/// apply/plan calls where trace data is not needed.
///
/// When `enable_trace` is true, TF_LOG=TRACE is written to a file under
/// `.agents-workspace/tmp/tf-logs/`. The file is NOT deleted by this function — the caller
/// is responsible for cleanup (delete on success, keep on failure for debugging).
async fn run_terraform_raw(
    dir: &Path,
    args: &[&str],
    timeout: Duration,
    enable_trace: bool,
) -> (bool, String, String, Option<PathBuf>) {
    let subcommand = args.first().unwrap_or(&"unknown");

    let trace_file: Option<PathBuf> = if enable_trace {
        let dir_name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let path = tf_tmp_root()
            .join("tf-logs")
            .join(format!("{dir_name}-{subcommand}.trace.log"));
        let _ = std::fs::create_dir_all(path.parent().unwrap());
        Some(path)
    } else {
        None
    };

    // Build the stderr handle: file when tracing, piped when not.
    let stderr_handle: std::process::Stdio = match &trace_file {
        Some(path) => {
            let f = std::fs::File::create(path).expect("Failed to create trace file");
            f.into()
        }
        None => std::process::Stdio::piped(),
    };

    let mut cmd = Command::new("terraform");
    cmd.args(args)
        .current_dir(dir)
        .env("TF_IN_AUTOMATION", "1")
        .env("TF_PLUGIN_CACHE_DIR", plugin_cache_dir())
        .env("TF_PLUGIN_CACHE_MAY_BREAK_DEPENDENCY_LOCK_FILE", "1")
        .stdout(std::process::Stdio::piped())
        .stderr(stderr_handle);

    if enable_trace {
        cmd.env("TF_LOG", "TRACE");
    } else {
        cmd.env("TF_LOG", "");
    }

    // FIX(terraform-e2e): Run terraform in its own process group so that when
    //   we kill it (on timeout or panic), the provider plugin child processes
    //   (terraform-provider-aws, etc.) are killed along with the parent and do
    //   not linger as orphans.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        cmd.as_std_mut().process_group(0);
    }

    let mut child = cmd
        .spawn()
        .expect("Failed to run terraform — is it installed?");

    // On Unix, capture the process-group ID (= terraform's PID) so we can kill
    // the entire group on timeout, eliminating orphaned provider processes.
    #[cfg(unix)]
    let pgid = child.id();

    // Wait with timeout. We wait on the exit status (not wait_with_output) so we
    // can still kill the child on timeout, then read stdout from the pipe.
    let wait_result = tokio::time::timeout(timeout, child.wait()).await;

    match wait_result {
        Ok(Ok(status)) => {
            // Process completed — read stdout from the pipe.
            let mut stdout_bytes = Vec::new();
            if let Some(mut stdout_pipe) = child.stdout.take() {
                use tokio::io::AsyncReadExt;
                let _ = stdout_pipe.read_to_end(&mut stdout_bytes).await;
            }
            let success = status.success();
            let stdout = String::from_utf8_lossy(&stdout_bytes).to_string();
            let stderr = if success {
                String::new()
            } else if let Some(ref path) = trace_file {
                // Trace mode: filter out TRACE lines and return filtered content.
                std::fs::read_to_string(path)
                    .unwrap_or_default()
                    .lines()
                    .filter(|line| {
                        !line.contains("[TRACE]")
                            && !line.contains("Found resource type")
                            && !line.contains("Found data source type")
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                // No-trace mode: stderr was piped; read it from the child.
                let mut stderr_bytes = Vec::new();
                if let Some(mut stderr_pipe) = child.stderr.take() {
                    use tokio::io::AsyncReadExt;
                    let _ = stderr_pipe.read_to_end(&mut stderr_bytes).await;
                }
                String::from_utf8_lossy(&stderr_bytes).to_string()
            };
            (success, stdout, stderr, trace_file)
        }
        Ok(Err(e)) => (
            false,
            String::new(),
            format!("terraform process error: {e}"),
            trace_file,
        ),
        Err(_) => {
            // Timeout — kill the process group (includes all provider plugins).
            // Killing only the terraform process leaves the provider plugin children
            // (terraform-provider-aws etc.) as orphans; use killpg(2) to signal the
            // whole process group atomically.
            #[cfg(unix)]
            {
                if let Some(pid) = pgid {
                    // With process_group(0), terraform's pgid == its own PID.
                    // SAFETY: `pid` is the PID returned by child.id() for the terraform
                    //   process we just spawned; process_group(0) guarantees its pgid
                    //   equals its own PID. killpg is async-signal-safe.
                    unsafe { libc::killpg(pid as libc::pid_t, libc::SIGKILL) };
                }
            }
            let _ = child.kill().await;
            // FIX(terraform-e2e): SIGKILL does not allow terraform to release its
            //   state lock, leaving `.terraform.tfstate.lock.info` behind. Subsequent
            //   waves fail immediately with "Error acquiring the state lock", cascading
            //   failures across all remaining batch waves. Remove the stale lock file
            //   so subsequent waves can proceed.
            let lock_file = dir.join(".terraform.tfstate.lock.info");
            if lock_file.exists() {
                let _ = std::fs::remove_file(&lock_file);
            }
            let summary = if let Some(ref path) = trace_file {
                extract_timeout_summary(path)
            } else {
                "(no trace — re-run with enable_trace=true for diagnostics)\n".to_string()
            };
            let trace_display = trace_file
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            (
                false,
                String::new(),
                format!(
                    "TIMEOUT after {}s: {subcommand} did not complete.\n\
                     {}\
                     {summary}",
                    timeout.as_secs(),
                    if trace_display.is_empty() {
                        String::new()
                    } else {
                        format!("Trace saved to: {trace_display}\n")
                    },
                ),
                trace_file,
            )
        }
    }
}

/// Extract a short diagnostic summary from a TF_LOG=TRACE file after a timeout.
fn extract_timeout_summary(trace_file: &Path) -> String {
    let content = match std::fs::read_to_string(trace_file) {
        Ok(c) => c,
        Err(_) => return String::from("(could not read trace file)"),
    };

    let mut summary = String::new();

    // Count API operations called
    let mut op_counts = std::collections::HashMap::<String, usize>::new();
    for line in content.lines() {
        if let Some(pos) = line.find("x_amz_target=") {
            let rest = &line[pos + 13..];
            let op = rest.split_whitespace().next().unwrap_or(rest);
            *op_counts.entry(op.to_string()).or_default() += 1;
        }
    }
    if !op_counts.is_empty() {
        summary.push_str("API calls: ");
        let mut ops: Vec<_> = op_counts.iter().collect();
        ops.sort_by(|a, b| b.1.cmp(a.1));
        let parts: Vec<String> = ops.iter().map(|(op, n)| format!("{op} x{n}")).collect();
        summary.push_str(&parts.join(", "));
        summary.push('\n');
    }

    // Find missing operations
    let missing: Vec<&str> = content
        .lines()
        .filter(|l| l.contains("Could not find operation"))
        .collect();
    if !missing.is_empty() {
        summary.push_str("Missing operations:\n");
        for line in missing.iter().take(5) {
            // Extract just the error message
            if let Some(pos) = line.find("Could not find operation") {
                summary.push_str("  ");
                summary.push_str(&line[pos..pos + 80.min(line.len() - pos)]);
                summary.push('\n');
            }
        }
    }

    // Find failed requests
    let failures: Vec<&str> = content
        .lines()
        .filter(|l| l.contains("request failed with unretryable error"))
        .collect();
    if !failures.is_empty() {
        summary.push_str(&format!("Failed requests: {} (last: ", failures.len()));
        if let Some(last) = failures.last() {
            if let Some(pos) = last.find("api error") {
                let tail = &last[pos..];
                let end = tail.find(':').map(|p| p + 1).unwrap_or(tail.len().min(80));
                summary.push_str(&tail[..end]);
            }
        }
        summary.push_str(")\n");
    }

    if summary.is_empty() {
        summary.push_str("(no diagnostic info found in trace)\n");
    }

    summary
}

// ---------------------------------------------------------------------------
// Batched terraform apply
// ---------------------------------------------------------------------------

/// Settle window: how long to wait after the last registration before running
/// a batch. All tests that start simultaneously register within ~50ms; 200ms
/// gives ample margin.
const SETTLE_MS: u64 = 200;

/// Result returned to each test that called `batch_apply`.
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// Whether `terraform apply` succeeded.
    pub success: bool,
    /// Stdout from the apply (reflects only resources added in this batch).
    pub stdout: String,
    /// Stderr on failure (empty on success).
    pub stderr: String,
    /// Full contents of `terraform.tfstate` after the apply.
    pub state: String,
}

struct Registration {
    hcl: String,
    reply: tokio::sync::oneshot::Sender<BatchResult>,
}

/// Global batch scheduler — initialized on first call to `batch_apply`.
/// The background worker runs in its own `std::thread` with a dedicated
/// `current_thread` tokio runtime so it is independent of any test runtime.
pub struct BatchScheduler {
    tx: tokio::sync::mpsc::UnboundedSender<Registration>,
}

impl BatchScheduler {
    fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Registration>();
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build batch worker runtime")
                .block_on(batch_worker(rx));
        });
        Self { tx }
    }
}

static BATCH_SCHEDULER: std::sync::LazyLock<BatchScheduler> =
    std::sync::LazyLock::new(BatchScheduler::new);

/// Background worker: collects registrations, runs batched applies.
///
/// Each batch wave runs in a **fresh**, isolated terraform directory — no state
/// is accumulated across waves.  This avoids the "accumulated-state hang" where
/// terraform's refresh of hundreds of prior resources blocks the apply
/// indefinitely.  The cost is one `terraform init` per wave (≈1 s with the
/// shared plugin cache), which is acceptable.
async fn batch_worker(mut rx: tokio::sync::mpsc::UnboundedReceiver<Registration>) {
    // Start one shared mock server for all batch waves.
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");

    // Pre-initialise a template directory once so subsequent waves can reuse
    // the `.terraform` lock file and provider binary without re-downloading.
    let template_dir = create_tf_dir(&format!("batch-template-{}", std::process::id()));
    write_provider_tf(&template_dir, &url);
    std::fs::write(template_dir.join("main.tf"), "").unwrap();
    terraform_init(&template_dir).await;

    let mut wave: u32 = 0;

    loop {
        // Wait for the first registration of a new batch wave.
        let first = match rx.recv().await {
            Some(r) => r,
            None => break, // channel closed (test binary exiting)
        };

        let mut batch = vec![first];

        // Collect additional registrations during the settle window.
        while let Ok(Some(reg)) =
            tokio::time::timeout(std::time::Duration::from_millis(SETTLE_MS), rx.recv()).await
        {
            batch.push(reg);
        }

        wave += 1;

        // Build this wave's HCL from all registrations.
        let wave_hcl: String = batch
            .iter()
            .map(|r| r.hcl.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // Fresh directory per wave — no shared state with previous waves.
        let wave_dir = create_tf_dir(&format!("batch-wave-{}-{}", std::process::id(), wave));
        write_provider_tf(&wave_dir, &url);
        std::fs::write(wave_dir.join("main.tf"), &wave_hcl).unwrap();

        // Copy the lock file from the template so `terraform init` can reuse
        // the already-downloaded provider without a network round-trip.
        let lock_src = template_dir.join(".terraform.lock.hcl");
        let lock_dst = wave_dir.join(".terraform.lock.hcl");
        if lock_src.exists() {
            std::fs::copy(&lock_src, &lock_dst).ok();
        }

        // Init (fast — uses plugin cache) then apply.
        terraform_init(&wave_dir).await;
        let (ok, stdout, stderr) = terraform_apply(&wave_dir).await;
        let state = std::fs::read_to_string(wave_dir.join("terraform.tfstate")).unwrap_or_default();

        let result = BatchResult {
            success: ok,
            stdout,
            stderr,
            state,
        };

        // Broadcast to all tests in this wave.
        for reg in batch {
            let _ = reg.reply.send(result.clone());
        }
    }
}

/// Submit an HCL snippet to the shared batch queue and wait for the apply result.
///
/// # HCL naming requirement
///
/// Because all tests in a wave share a single `main.tf`, every resource must
/// have a **globally unique local name**. Convention: use the test-function slug
/// (everything after `test_`) as the local name, e.g.:
///
/// ```hcl
/// # test_kms_key_basic → local name "kms_key_basic"
/// resource "aws_kms_key" "kms_key_basic" { description = "kms-key-basic" }
/// ```
///
/// Tests that need two `terraform apply` passes (e.g. modify-in-place, plan
/// idempotency) should use the existing `terraform_apply` / `run_terraform`
/// helpers with their own isolated directory instead of this function.
#[allow(dead_code)]
pub async fn batch_apply(hcl: &str) -> BatchResult {
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    BATCH_SCHEDULER
        .tx
        .send(Registration {
            hcl: hcl.to_string(),
            reply: reply_tx,
        })
        .expect("batch worker channel closed");
    reply_rx.await.expect("batch worker dropped reply sender")
}

pub async fn terraform_init(dir: &Path) {
    // Init has no timeout — it may download the provider on first run.
    let output = Command::new("terraform")
        .args(["init", "-no-color"])
        .current_dir(dir)
        .env("TF_LOG", "")
        .env("TF_IN_AUTOMATION", "1")
        .env("TF_PLUGIN_CACHE_DIR", plugin_cache_dir())
        .env("TF_PLUGIN_CACHE_MAY_BREAK_DEPENDENCY_LOCK_FILE", "1")
        .output()
        .await
        .expect("Failed to run terraform — is it installed?");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.status.success(), "terraform init failed:\n{stderr}");
}

pub async fn terraform_apply(dir: &Path) -> (bool, String, String) {
    run_terraform(dir, &["apply", "-auto-approve", "-no-color"]).await
}

pub async fn terraform_plan(dir: &Path) -> (bool, String, String) {
    // -detailed-exitcode: 0 = no changes, 1 = error, 2 = changes pending
    run_terraform(dir, &["plan", "-no-color", "-detailed-exitcode"]).await
}

#[allow(dead_code)]
pub async fn terraform_destroy(dir: &Path) -> (bool, String, String) {
    run_terraform(dir, &["destroy", "-auto-approve", "-no-color"]).await
}

pub fn cleanup_tf_dir(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}

// ---------------------------------------------------------------------------
// Smoke-test utility
// ---------------------------------------------------------------------------

/// A single API call observed in the terraform trace.
#[derive(Debug, Clone)]
pub struct ApiCall {
    pub operation: String,
    pub status_code: u16,
    pub response_body: String,
}

/// An API call that failed with an unretryable error.
#[derive(Debug, Clone)]
pub struct FailedOp {
    pub operation: String,
    pub error_type: String,
    pub message: String,
}

/// Result of a smoke test run.
#[derive(Debug)]
pub struct SmokeTestResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
    /// Trace file path (kept on failure, deleted on success).
    pub trace_file: Option<PathBuf>,
    /// API calls parsed from the trace.
    pub api_calls: Vec<ApiCall>,
    /// Operations that returned unretryable errors.
    pub failed_operations: Vec<FailedOp>,
    /// Operations the provider tried but the handler didn't dispatch.
    pub missing_operations: Vec<String>,
}

impl std::fmt::Display for SmokeTestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.success { "PASS" } else { "FAIL" };
        write!(f, "{status} ({:.1}s)", self.duration.as_secs_f64())?;

        if !self.missing_operations.is_empty() {
            write!(f, " missing: {:?}", self.missing_operations)?;
        }
        if !self.failed_operations.is_empty() {
            let ops: Vec<&str> = self
                .failed_operations
                .iter()
                .map(|o| o.operation.as_str())
                .collect();
            write!(f, " errors: {ops:?}")?;
        }
        if let Some(ref trace) = self.trace_file {
            write!(f, " trace: {}", trace.display())?;
        }
        Ok(())
    }
}

/// Run a complete terraform init + apply cycle against an in-process server
/// and return a structured result with parsed trace data.
///
/// `main_tf` is the content of the `main.tf` file to apply.
/// `timeout` is the maximum time for the apply step.
#[allow(dead_code)]
pub async fn smoke_test_terraform(
    services: Vec<Arc<dyn MockService>>,
    main_tf: &str,
    timeout: Duration,
) -> SmokeTestResult {
    let start = std::time::Instant::now();

    let port = start_server(services).await;
    let url = format!("http://127.0.0.1:{port}");

    // Use a unique dir name based on a hash of the main_tf content.
    let hash = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        main_tf.hash(&mut h);
        h.finish()
    };
    let dir = create_tf_dir(&format!("smoke-{hash:016x}"));

    write_provider_tf(&dir, &url);
    std::fs::write(dir.join("main.tf"), main_tf).unwrap();

    // Init (no timeout — may download provider on first run)
    let init_output = Command::new("terraform")
        .args(["init", "-no-color"])
        .current_dir(&dir)
        .env("TF_LOG", "")
        .env("TF_IN_AUTOMATION", "1")
        .env("TF_PLUGIN_CACHE_DIR", plugin_cache_dir())
        .env("TF_PLUGIN_CACHE_MAY_BREAK_DEPENDENCY_LOCK_FILE", "1")
        .output()
        .await
        .expect("Failed to run terraform");

    if !init_output.status.success() {
        let stderr = String::from_utf8_lossy(&init_output.stderr).to_string();
        cleanup_tf_dir(&dir);
        return SmokeTestResult {
            success: false,
            stdout: String::new(),
            stderr: format!("terraform init failed:\n{stderr}"),
            duration: start.elapsed(),
            trace_file: None,
            api_calls: vec![],
            failed_operations: vec![],
            missing_operations: vec![],
        };
    }

    // Apply with timeout and trace capture. Use run_terraform_raw with trace
    // enabled so we can parse the trace before it's deleted.
    let (ok, stdout, stderr, trace_path) = run_terraform_raw(
        &dir,
        &["apply", "-auto-approve", "-no-color"],
        timeout,
        true,
    )
    .await;

    // Parse the trace (always available since run_terraform_raw doesn't delete it).
    let (api_calls, failed_operations, missing_operations) = match &trace_path {
        Some(p) if p.exists() => parse_trace(p),
        _ => (vec![], vec![], vec![]),
    };

    // On success, delete the trace. On failure, keep it for debugging.
    let trace_file = match trace_path {
        Some(p) if !ok && p.exists() => Some(p),
        Some(p) => {
            let _ = std::fs::remove_file(&p);
            None
        }
        None => None,
    };

    cleanup_tf_dir(&dir);

    SmokeTestResult {
        success: ok,
        stdout,
        stderr,
        duration: start.elapsed(),
        trace_file,
        api_calls,
        failed_operations,
        missing_operations,
    }
}

/// Parse a TF_LOG=TRACE file and extract structured API call data.
fn parse_trace(trace_file: &Path) -> (Vec<ApiCall>, Vec<FailedOp>, Vec<String>) {
    let content = match std::fs::read_to_string(trace_file) {
        Ok(c) => c,
        Err(_) => return (vec![], vec![], vec![]),
    };

    let mut api_calls = Vec::new();
    let mut failed_ops = Vec::new();
    let mut missing_ops = Vec::new();

    // State machine: after an "HTTP Response Received" line with an operation
    // and status code, the next line starting with "  | " is the response body.
    let mut pending_op: Option<(String, u16)> = None;

    for line in content.lines() {
        // Track pending response body
        if let Some((ref op, status)) = pending_op {
            if let Some(body) = line.strip_prefix("  | ") {
                api_calls.push(ApiCall {
                    operation: op.clone(),
                    status_code: status,
                    response_body: body.to_string(),
                });
                pending_op = None;
                continue;
            } else {
                // Response had no body line — record with empty body
                api_calls.push(ApiCall {
                    operation: op.clone(),
                    status_code: status,
                    response_body: String::new(),
                });
                pending_op = None;
            }
        }

        // "HTTP Response Received" with status code and operation
        if line.contains("HTTP Response Received") {
            let status = extract_field(line, "http.status_code=")
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(0);
            let op = extract_field(line, "rpc.method=").unwrap_or_default();
            if !op.is_empty() {
                pending_op = Some((op, status));
            }
            continue;
        }

        // Missing operation
        if line.contains("Could not find operation") {
            // Extract: "Could not find operation XxxYyy for ServiceName"
            if let Some(pos) = line.find("Could not find operation") {
                let rest = &line[pos + 25..];
                let op = rest.split_whitespace().next().unwrap_or(rest);
                let op_name = op.trim_end_matches(':');
                if !missing_ops.contains(&op_name.to_string()) {
                    missing_ops.push(op_name.to_string());
                }
            }
            continue;
        }

        // Failed request
        if line.contains("request failed with unretryable error") {
            let op = extract_field(line, "rpc.method=").unwrap_or_default();
            let (error_type, message) = if let Some(pos) = line.find("api error ") {
                let rest = &line[pos + 10..];
                if let Some(colon) = rest.find(':') {
                    (rest[..colon].to_string(), rest[colon + 2..].to_string())
                } else {
                    (rest.to_string(), String::new())
                }
            } else {
                ("Unknown".to_string(), String::new())
            };
            failed_ops.push(FailedOp {
                operation: op,
                error_type,
                message,
            });
        }
    }

    // Flush pending
    if let Some((op, status)) = pending_op {
        api_calls.push(ApiCall {
            operation: op,
            status_code: status,
            response_body: String::new(),
        });
    }

    (api_calls, failed_ops, missing_ops)
}

/// Extract a field value from a terraform trace log line.
/// e.g., extract_field(line, "http.status_code=") -> Some("200")
fn extract_field(line: &str, prefix: &str) -> Option<String> {
    let start = line.find(prefix)? + prefix.len();
    let rest = &line[start..];
    let end = rest
        .find(|c: char| c.is_whitespace() || c == ',')
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

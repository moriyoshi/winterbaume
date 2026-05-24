//! Integration tests that run real `terraform` commands against an in-process
//! winterbaume HTTP server.
//!
//! These tests are `#[ignore]` by default because they require:
//! - `terraform` CLI installed
//! - Network access (terraform init downloads the AWS provider on first run)
//!
//! Run with:
//!   cargo test -p winterbaume-terraform --test terraform_cli_test -- --ignored
//!
//! Run a single test:
//!   cargo test -p winterbaume-terraform --test terraform_cli_test test_terraform_apply_s3_bucket -- --ignored

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;
use tokio::process::Command;
use winterbaume_amplify::AmplifyService;
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
async fn start_server(services: Vec<Arc<dyn MockService>>) -> u16 {
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

/// Create a minimal set of services for terraform testing.
fn test_services() -> Vec<Arc<dyn MockService>> {
    vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::new(winterbaume_s3::S3Service::new()),
        Arc::new(winterbaume_iam::IamService::new()),
        Arc::new(winterbaume_sqs::SqsService::new()),
    ]
}

/// Create services including App Runner for terraform testing.
fn test_services_with_apprunner() -> Vec<Arc<dyn MockService>> {
    vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::new(winterbaume_s3::S3Service::new()),
        Arc::new(winterbaume_iam::IamService::new()),
        Arc::new(winterbaume_sqs::SqsService::new()),
        Arc::new(winterbaume_apprunner::AppRunnerService::new()),
    ]
}

/// Write the AWS provider configuration with App Runner endpoint.
fn write_provider_tf_with_apprunner(dir: &Path, server_url: &str) {
    let content = format!(
        r#"terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
    }}
  }}
}}

provider "aws" {{
  region                      = "us-east-1"
  s3_use_path_style           = true
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {{
    apprunner = "{server_url}"
    s3        = "{server_url}"
    iam       = "{server_url}"
    sqs       = "{server_url}"
    sts       = "{server_url}"
  }}

  access_key = "test"
  secret_key = "test"
}}
"#
    );
    std::fs::write(dir.join("provider.tf"), content).unwrap();
}

// ---------------------------------------------------------------------------
// Terraform helpers
// ---------------------------------------------------------------------------

/// Path to the workspace root.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Create a temporary directory for terraform configs under .agents-workspace/tmp.
fn create_tf_dir(test_name: &str) -> PathBuf {
    let dir = workspace_root()
        .join(".agents-workspace")
        .join("tmp")
        .join(format!("tf-test-{test_name}"));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).ok();
    }
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Write the AWS provider configuration pointing at the given server URL.
fn write_provider_tf(dir: &Path, server_url: &str) {
    let content = format!(
        r#"terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
    }}
  }}
}}

provider "aws" {{
  region                      = "us-east-1"
  s3_use_path_style           = true
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {{
    s3  = "{server_url}"
    iam = "{server_url}"
    sqs = "{server_url}"
    sts = "{server_url}"
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
    let dir = workspace_root()
        .join(".agents")
        .join("tmp")
        .join("tf-plugin-cache");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Run a terraform command and return (success, stdout, stderr).
async fn run_terraform(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let output = Command::new("terraform")
        .args(args)
        .current_dir(dir)
        .env("TF_LOG", "")
        .env("TF_IN_AUTOMATION", "1")
        .env("TF_PLUGIN_CACHE_DIR", plugin_cache_dir())
        .env("TF_PLUGIN_CACHE_MAY_BREAK_DEPENDENCY_LOCK_FILE", "1")
        .output()
        .await
        .expect("Failed to run terraform — is it installed?");

    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

async fn terraform_init(dir: &Path) {
    let (ok, _stdout, stderr) = run_terraform(dir, &["init", "-no-color"]).await;
    assert!(ok, "terraform init failed:\n{stderr}");
}

async fn terraform_apply(dir: &Path) -> (bool, String, String) {
    run_terraform(dir, &["apply", "-auto-approve", "-no-color"]).await
}

async fn terraform_plan(dir: &Path) -> (bool, String, String) {
    // -detailed-exitcode: 0 = no changes, 1 = error, 2 = changes pending
    run_terraform(dir, &["plan", "-no-color", "-detailed-exitcode"]).await
}

async fn terraform_destroy(dir: &Path) -> (bool, String, String) {
    run_terraform(dir, &["destroy", "-auto-approve", "-no-color"]).await
}

fn cleanup_tf_dir(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_s3_bucket() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-s3");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "test" {
  bucket = "my-terraform-test-bucket"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Apply complete! Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the bucket
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("my-terraform-test-bucket"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_sqs_queue() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-sqs");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_sqs_queue" "test" {
  name                       = "terraform-test-queue"
  visibility_timeout_seconds = 45
  delay_seconds              = 5
  max_message_size           = 131072
  message_retention_seconds  = 86400
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the queue
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("terraform-test-queue"));

    // Note: SQS destroy is not tested here because the terraform AWS provider
    // polls GetQueueAttributes for ~60s after DeleteQueue to confirm deletion,
    // which makes the destroy operation take 60+ seconds.

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_iam_role() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-iam-role");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_iam_role" "test" {
  name = "terraform-test-role"
  path = "/service-role/"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
    }]
  })

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("terraform-test-role"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_multi_service() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-multi");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "data" {
  bucket = "multi-test-data-bucket"
}

resource "aws_iam_role" "app" {
  name = "multi-test-app-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "ecs-tasks.amazonaws.com" }
    }]
  })
}

resource "aws_iam_policy" "s3_access" {
  name = "multi-test-s3-access"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = ["s3:GetObject", "s3:PutObject"]
      Effect   = "Allow"
      Resource = "${aws_s3_bucket.data.arn}/*"
    }]
  })
}

resource "aws_iam_role_policy_attachment" "attach" {
  role       = aws_iam_role.app.name
  policy_arn = aws_iam_policy.s3_access.arn
}

resource "aws_sqs_queue" "events" {
  name                       = "multi-test-events"
  visibility_timeout_seconds = 30
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 5 added"),
        "Expected 5 resources added:\n{stdout}"
    );

    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("multi-test-data-bucket"));
    assert!(state_content.contains("multi-test-app-role"));
    assert!(state_content.contains("multi-test-s3-access"));
    assert!(state_content.contains("multi-test-events"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_plan_idempotent_after_apply() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("plan-idempotent");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "test" {
  bucket = "idempotent-test-bucket"
}

resource "aws_sqs_queue" "test" {
  name = "idempotent-test-queue"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // With -detailed-exitcode: 0 = no changes, 2 = changes pending
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_then_modify() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_sqs_queue" "test" {
  name                       = "modify-test-queue"
  visibility_timeout_seconds = 30
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    // Modify the resource
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_sqs_queue" "test" {
  name                       = "modify-test-queue"
  visibility_timeout_seconds = 60
  delay_seconds              = 10
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// App Runner tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_apprunner_service() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-apprunner-service");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_service" "test" {
  service_name = "tf-test-apprunner-svc"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "8080"
      }
    }
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the service
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-test-apprunner-svc"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_apprunner_service() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-apprunner");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_service" "test" {
  service_name = "tf-destroy-apprunner-svc"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "8080"
      }
    }
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    // Apply
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 destroyed")
            || stdout.contains("Destroy complete! Resources: 1 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_apprunner_service_plan_idempotent() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apprunner-plan-idempotent");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_service" "test" {
  service_name = "tf-idempotent-apprunner-svc"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "8080"
      }
    }
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // With -detailed-exitcode: 0 = no changes, 2 = changes pending
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_multiple_apprunner_services() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-multi-apprunner");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_service" "web" {
  service_name = "tf-multi-web-svc"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "8080"
      }
    }
  }

  tags = {
    Role = "web"
  }
}

resource "aws_apprunner_service" "api" {
  service_name = "tf-multi-api-svc"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "3000"
      }
    }
  }

  tags = {
    Role = "api"
  }
}

resource "aws_apprunner_service" "worker" {
  service_name = "tf-multi-worker-svc"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "8080"
      }
    }
  }

  tags = {
    Role = "worker"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 3 added"),
        "Expected 3 resources added:\n{stdout}"
    );

    // Verify state file contains all services
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-multi-web-svc"));
    assert!(state_content.contains("tf-multi-api-svc"));
    assert!(state_content.contains("tf-multi-worker-svc"));

    // Destroy all
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 3 destroyed")
            || stdout.contains("Destroy complete! Resources: 3 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_apprunner_connection() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-apprunner-connection");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_connection" "test" {
  connection_name = "tf-test-github-connection"
  provider_type   = "GITHUB"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the connection
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-test-github-connection"));

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 destroyed")
            || stdout.contains("Destroy complete! Resources: 1 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_apprunner_auto_scaling() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-apprunner-autoscaling");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_auto_scaling_configuration_version" "test" {
  auto_scaling_configuration_name = "tf-test-scaling-config"
  min_size                        = 1
  max_size                        = 5
  max_concurrency                 = 50

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the auto scaling configuration
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-test-scaling-config"));

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 destroyed")
            || stdout.contains("Destroy complete! Resources: 1 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_apprunner_service_with_auto_scaling() {
    let port = start_server(test_services_with_apprunner()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-apprunner-svc-autoscaling");

    write_provider_tf_with_apprunner(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_apprunner_auto_scaling_configuration_version" "scaling" {
  auto_scaling_configuration_name = "tf-svc-scaling-config"
  min_size                        = 2
  max_size                        = 10
  max_concurrency                 = 100

  tags = {
    Component = "scaling"
  }
}

resource "aws_apprunner_service" "app" {
  service_name = "tf-svc-with-autoscaling"

  source_configuration {
    auto_deployments_enabled = false

    image_repository {
      image_identifier      = "public.ecr.aws/aws-containers/hello-app-runner:latest"
      image_repository_type = "ECR_PUBLIC"

      image_configuration {
        port = "8080"
      }
    }
  }

  auto_scaling_configuration_arn = aws_apprunner_auto_scaling_configuration_version.scaling.arn

  tags = {
    Component = "service"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 2 added"),
        "Expected 2 resources added:\n{stdout}"
    );

    // Verify state file contains both resources
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-svc-scaling-config"));
    assert!(state_content.contains("tf-svc-with-autoscaling"));

    // Destroy all
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 2 destroyed")
            || stdout.contains("Destroy complete! Resources: 2 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Amplify tests
// ---------------------------------------------------------------------------

/// Create services including Amplify for terraform testing.
fn amplify_services() -> Vec<Arc<dyn MockService>> {
    vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::new(AmplifyService::new()),
    ]
}

/// Write the AWS provider configuration with Amplify endpoint.
fn write_provider_tf_with_amplify(dir: &Path, server_url: &str) {
    let content = format!(
        r#"terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
    }}
  }}
}}

provider "aws" {{
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {{
    amplify = "{server_url}"
    sts     = "{server_url}"
  }}

  access_key = "test"
  secret_key = "test"
}}
"#
    );
    std::fs::write(dir.join("provider.tf"), content).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_amplify_app() {
    let port = start_server(amplify_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-amplify-app");

    write_provider_tf_with_amplify(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_amplify_app" "test" {
  name        = "my-amplify-test-app"
  description = "Test Amplify app for E2E"
  repository  = "https://github.com/example/repo"
  platform    = "WEB"

  build_spec = <<-EOT
    version: 1
    frontend:
      phases:
        build:
          commands:
            - npm run build
  EOT
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Apply complete! Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the app
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("my-amplify-test-app"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_amplify_branch() {
    let port = start_server(amplify_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-amplify-branch");

    write_provider_tf_with_amplify(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_amplify_app" "test" {
  name = "branch-test-app"
}

resource "aws_amplify_branch" "main" {
  app_id      = aws_amplify_app.test.id
  branch_name = "main"
  description = "Main branch"
  stage       = "PRODUCTION"
  framework   = "React"
}

resource "aws_amplify_branch" "develop" {
  app_id      = aws_amplify_app.test.id
  branch_name = "develop"
  description = "Development branch"
  stage       = "DEVELOPMENT"
  framework   = "React"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 3 added"),
        "Expected 3 resources added (1 app + 2 branches):\n{stdout}"
    );

    // Verify state file contains all resources
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("branch-test-app"));
    assert!(state_content.contains("main"));
    assert!(state_content.contains("develop"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_amplify_app_plan_idempotent() {
    let port = start_server(amplify_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("amplify-plan-idempotent");

    write_provider_tf_with_amplify(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_amplify_app" "test" {
  name        = "idempotent-amplify-app"
  description = "Idempotency check"
  platform    = "WEB"
}

resource "aws_amplify_branch" "main" {
  app_id      = aws_amplify_app.test.id
  branch_name = "main"
  stage       = "PRODUCTION"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // With -detailed-exitcode: 0 = no changes, 2 = changes pending
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_amplify_app() {
    let port = start_server(amplify_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-amplify-app");

    write_provider_tf_with_amplify(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_amplify_app" "test" {
  name        = "destroy-amplify-app"
  description = "App to be destroyed"
  platform    = "WEB"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    // Apply
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 destroyed")
            || stdout.contains("Destroy complete! Resources: 1 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_amplify_branch() {
    let port = start_server(amplify_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-amplify-branch");

    write_provider_tf_with_amplify(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_amplify_app" "test" {
  name = "destroy-branch-app"
}

resource "aws_amplify_branch" "main" {
  app_id      = aws_amplify_app.test.id
  branch_name = "main"
  stage       = "PRODUCTION"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    // Apply
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 2 added"),
        "Expected 2 resources added (1 app + 1 branch):\n{stdout}"
    );

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 2 destroyed")
            || stdout.contains("Destroy complete! Resources: 2 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Chatbot tests
// ---------------------------------------------------------------------------

/// Create services including Chatbot for terraform testing.
fn chatbot_services() -> Vec<Arc<dyn MockService>> {
    vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::new(winterbaume_chatbot::ChatbotService::new()),
    ]
}

/// Write the AWS provider configuration with Chatbot endpoint.
fn write_provider_tf_with_chatbot(dir: &Path, server_url: &str) {
    let content = format!(
        r#"terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
    }}
  }}
}}

provider "aws" {{
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {{
    chatbot = "{server_url}"
    sts     = "{server_url}"
  }}

  access_key = "test"
  secret_key = "test"
}}
"#
    );
    std::fs::write(dir.join("provider.tf"), content).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_chatbot_slack_channel_configuration() {
    let port = start_server(chatbot_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-chatbot-slack");

    write_provider_tf_with_chatbot(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_chatbot_slack_channel_configuration" "test" {
  configuration_name = "tf-test-slack-config"
  slack_team_id      = "T0123456789"
  slack_channel_id   = "C0123456789"
  iam_role_arn       = "arn:aws:iam::123456789012:role/chatbot-role"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Apply complete! Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the configuration
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-test-slack-config"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_chatbot_slack_channel_configuration() {
    let port = start_server(chatbot_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-chatbot-slack");

    write_provider_tf_with_chatbot(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_chatbot_slack_channel_configuration" "test" {
  configuration_name = "tf-destroy-slack-config"
  slack_team_id      = "T9876543210"
  slack_channel_id   = "C9876543210"
  iam_role_arn       = "arn:aws:iam::123456789012:role/chatbot-destroy-role"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    // Apply
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 destroyed")
            || stdout.contains("Destroy complete! Resources: 1 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_chatbot_teams_channel_configuration() {
    let port = start_server(chatbot_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-chatbot-teams");

    write_provider_tf_with_chatbot(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_chatbot_microsoft_teams_channel_configuration" "test" {
  configuration_name = "tf-test-teams-config"
  team_id            = "11111111-2222-3333-4444-555555555555"
  tenant_id          = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"
  channel_id         = "19:test-channel@thread.tacv2"
  iam_role_arn       = "arn:aws:iam::123456789012:role/chatbot-teams-role"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Apply complete! Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Verify state file contains the configuration
    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-test-teams-config"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_chatbot_teams_channel_configuration() {
    let port = start_server(chatbot_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-chatbot-teams");

    write_provider_tf_with_chatbot(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_chatbot_microsoft_teams_channel_configuration" "test" {
  configuration_name = "tf-destroy-teams-config"
  team_id            = "22222222-3333-4444-5555-666666666666"
  tenant_id          = "bbbbbbbb-cccc-dddd-eeee-ffffffffffff"
  channel_id         = "19:destroy-channel@thread.tacv2"
  iam_role_arn       = "arn:aws:iam::123456789012:role/chatbot-teams-destroy-role"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    // Apply
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Destroy
    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 1 destroyed")
            || stdout.contains("Destroy complete! Resources: 1 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Lambda permission tests
// ---------------------------------------------------------------------------

fn lambda_services() -> Vec<Arc<dyn MockService>> {
    vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::new(winterbaume_iam::IamService::new()),
        Arc::new(winterbaume_lambda::LambdaService::new()),
    ]
}

fn write_provider_tf_with_lambda(dir: &Path, server_url: &str) {
    let content = format!(
        r#"terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
    }}
  }}
}}

provider "aws" {{
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {{
    lambda = "{server_url}"
    iam    = "{server_url}"
    sts    = "{server_url}"
  }}

  access_key = "test"
  secret_key = "test"
}}
"#
    );
    std::fs::write(dir.join("provider.tf"), content).unwrap();
}

/// Write a minimal valid zip file (End-of-Central-Directory record only).
/// The Lambda mock only decodes the bytes to hash them, so an empty archive is fine.
fn write_empty_zip(dir: &Path, filename: &str) {
    let empty_zip: [u8; 22] = [
        0x50, 0x4b, 0x05, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    std::fs::write(dir.join(filename), empty_zip).unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_lambda_permission() {
    let port = start_server(lambda_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-lambda-permission");

    write_provider_tf_with_lambda(&dir, &url);
    write_empty_zip(&dir, "lambda.zip");
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lambda_function" "fn" {
  function_name = "tf-test-fn"
  role          = "arn:aws:iam::123456789012:role/lambda-role"
  runtime       = "python3.12"
  handler       = "index.handler"
  filename      = "lambda.zip"
}

resource "aws_lambda_permission" "allow_events" {
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Apply complete! Resources: 2 added"),
        "Unexpected apply output:\n{stdout}"
    );

    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(state_content.contains("tf-test-fn"));
    assert!(state_content.contains("AllowEvents"));

    cleanup_tf_dir(&dir);
}

/// Regression for the GetPolicy random-uuid bug fixed in 5e3ba012. GetPolicy used
/// to mint a fresh RevisionId on every read; the AWS provider's refresh would
/// then see a different value from the one in state and report drift. A second
/// `terraform plan` after apply must show no changes.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_plan_idempotent_lambda_permission_after_apply() {
    let port = start_server(lambda_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("plan-idempotent-lambda-permission");

    write_provider_tf_with_lambda(&dir, &url);
    write_empty_zip(&dir, "lambda.zip");
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lambda_function" "fn" {
  function_name = "tf-idempotent-fn"
  role          = "arn:aws:iam::123456789012:role/lambda-role"
  runtime       = "python3.12"
  handler       = "index.handler"
  filename      = "lambda.zip"
}

resource "aws_lambda_permission" "allow_events" {
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // -detailed-exitcode: 0 = no changes, 2 = changes pending
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

/// Apply + destroy exercises RemovePermission. The AWS provider reads the
/// policy first, then passes the resulting RevisionId as the `RevisionId` query
/// parameter of `DELETE /2015-03-31/functions/{name}/policy/{sid}`. Before
/// 5e3ba012 the handler ignored that query parameter, so a stale value would be
/// silently accepted; after the fix it must match the stored revision for the
/// call to succeed.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_and_destroy_lambda_permission() {
    let port = start_server(lambda_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-destroy-lambda-permission");

    write_provider_tf_with_lambda(&dir, &url);
    write_empty_zip(&dir, "lambda.zip");
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lambda_function" "fn" {
  function_name = "tf-destroy-fn"
  role          = "arn:aws:iam::123456789012:role/lambda-role"
  runtime       = "python3.12"
  handler       = "index.handler"
  filename      = "lambda.zip"
}

resource "aws_lambda_permission" "allow_events" {
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 2 added"),
        "Unexpected apply output:\n{stdout}"
    );

    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 2 destroyed")
            || stdout.contains("Destroy complete! Resources: 2 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

/// Add two distinct `aws_lambda_permission` statements on the same function,
/// then remove only one of them. Exercises AddPermission bumping the revision id
/// between statements and RemovePermission honouring the latest revision id
/// when the provider deletes the dropped statement; the surviving permission
/// must remain in state and the dropped one must not.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_lambda_permission_then_drop_one() {
    let port = start_server(lambda_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("apply-lambda-permission-drop-one");

    write_provider_tf_with_lambda(&dir, &url);
    write_empty_zip(&dir, "lambda.zip");
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lambda_function" "fn" {
  function_name = "tf-multi-fn"
  role          = "arn:aws:iam::123456789012:role/lambda-role"
  runtime       = "python3.12"
  handler       = "index.handler"
  filename      = "lambda.zip"
}

resource "aws_lambda_permission" "allow_events" {
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}

resource "aws_lambda_permission" "allow_sns" {
  statement_id  = "AllowSns"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fn.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = "arn:aws:sns:us-east-1:123456789012:my-topic"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 3 added"),
        "Unexpected apply output:\n{stdout}"
    );

    // Drop the SNS statement and re-apply: exactly one permission should be removed.
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lambda_function" "fn" {
  function_name = "tf-multi-fn"
  role          = "arn:aws:iam::123456789012:role/lambda-role"
  runtime       = "python3.12"
  handler       = "index.handler"
  filename      = "lambda.zip"
}

resource "aws_lambda_permission" "allow_events" {
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 0 changed, 1 destroyed"),
        "Unexpected second-apply output:\n{stdout}"
    );

    let state_content = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap();
    assert!(
        state_content.contains("AllowEvents"),
        "Surviving permission missing from state:\n{state_content}"
    );
    assert!(
        !state_content.contains("AllowSns"),
        "Dropped permission still present in state:\n{state_content}"
    );

    cleanup_tf_dir(&dir);
}

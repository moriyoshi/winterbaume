//! Cloud Control API E2E tests through the Terraform `awscc` provider.
//!
//! The `aws` provider talks to per-service APIs directly and never reaches
//! Cloud Control, so these tests use `awscc` instead. Each test starts an
//! isolated server where Cloud Control shares its owning services with the
//! native APIs, mirroring `winterbaume-server`, and then checks the result
//! through the native API as well as through Terraform state.

use std::sync::Arc;

use bytes::Bytes;
use serde_json::{Value, json};
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::service::{MockRequest, MockService};
use winterbaume_ssm::SsmService;

use crate::harness::*;

const PARAMETER_NAME: &str = "/carina-awscc-459/probe";

/// Call the native SSM API on the same `SsmService` instance that Cloud
/// Control is wired to, bypassing Terraform entirely.
async fn ssm_call(ssm: &SsmService, action: &str, body: Value) -> (u16, Value) {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        "x-amz-target",
        format!("AmazonSSM.{action}").parse().unwrap(),
    );
    headers.insert(
        "content-type",
        "application/x-amz-json-1.1".parse().unwrap(),
    );
    let response = ssm
        .handle(MockRequest {
            method: "POST".to_string(),
            uri: "https://ssm.us-east-1.amazonaws.com/".to_string(),
            headers,
            body: Bytes::from(body.to_string()),
        })
        .await;
    let body = serde_json::from_slice(&response.body).unwrap_or(Value::Null);
    (response.status, body)
}

async fn ssm_parameter(ssm: &SsmService) -> (u16, Value) {
    ssm_call(ssm, "GetParameter", json!({ "Name": PARAMETER_NAME })).await
}

async fn ssm_tags(ssm: &SsmService) -> Vec<(String, String)> {
    let (status, body) = ssm_call(
        ssm,
        "ListTagsForResource",
        json!({ "ResourceType": "Parameter", "ResourceId": PARAMETER_NAME }),
    )
    .await;
    assert_eq!(status, 200, "ListTagsForResource failed: {body}");
    let mut tags: Vec<(String, String)> = body["TagList"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            (
                t["Key"].as_str().unwrap().to_string(),
                t["Value"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    tags.sort();
    tags
}

fn state_attributes(dir: &std::path::Path) -> Value {
    let state: Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap())
            .unwrap();
    state["resources"][0]["instances"][0]["attributes"].clone()
}

fn parameter_tf(value: &str, tags: &str) -> String {
    format!(
        r#"
resource "awscc_ssm_parameter" "test" {{
  name        = "{PARAMETER_NAME}"
  type        = "String"
  value       = "{value}"
  description = "probe"
  tags        = {tags}
}}
"#
    )
}

// Issue #33: `AWS::SSM::Parameter` must be a real SSM parameter, not a
// Cloud Control-only record. Uses an isolated server because the test mutates
// the parameter out of band and expects Terraform to see the drift.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_awscc_ssm_parameter_backed_by_ssm() {
    let ssm = Arc::new(SsmService::new());
    let cloudcontrol = Arc::new(CloudControlService::with_ssm_service(Arc::clone(&ssm)));
    let port = start_server(vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::clone(&ssm) as Arc<dyn MockService>,
        cloudcontrol,
    ])
    .await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("awscc-ssm-parameter");
    write_awscc_provider_tf(&dir, &url);

    // Create through Cloud Control.
    std::fs::write(
        dir.join("main.tf"),
        parameter_tf("v1", r#"{ env = "dev", owner = "carina" }"#),
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let attributes = state_attributes(&dir);
    assert_eq!(
        attributes["arn"],
        "arn:aws:ssm:us-east-1:123456789012:parameter/carina-awscc-459/probe"
    );
    assert_eq!(attributes["data_type"], "text");
    assert_eq!(attributes["value"], "v1");

    // The parameter is a native SSM parameter.
    let (status, body) = ssm_parameter(&ssm).await;
    assert_eq!(status, 200, "GetParameter failed: {body}");
    assert_eq!(body["Parameter"]["Value"], "v1");
    assert_eq!(body["Parameter"]["DataType"], "text");
    assert_eq!(
        ssm_tags(&ssm).await,
        vec![
            ("env".to_string(), "dev".to_string()),
            ("owner".to_string(), "carina".to_string()),
        ]
    );

    // Write-only Description must not cause drift on refresh.
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    // An out-of-band SSM overwrite is visible to Cloud Control reads.
    let (status, body) = ssm_call(
        &ssm,
        "PutParameter",
        json!({ "Name": PARAMETER_NAME, "Value": "v2", "Overwrite": true }),
    )
    .await;
    assert_eq!(status, 200, "PutParameter failed: {body}");
    let (ok, stdout, _stderr) = terraform_plan(&dir).await;
    assert!(!ok, "expected drift after out-of-band update:\n{stdout}");
    assert!(
        stdout.contains("1 to change"),
        "Expected an in-place update to revert the out-of-band value:\n{stdout}"
    );

    // An in-place update through Cloud Control reaches SSM.
    std::fs::write(
        dir.join("main.tf"),
        parameter_tf("v3", r#"{ env = "dev", team = "platform" }"#),
    )
    .unwrap();
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );
    let (status, body) = ssm_parameter(&ssm).await;
    assert_eq!(status, 200, "GetParameter failed: {body}");
    assert_eq!(body["Parameter"]["Value"], "v3");
    assert_eq!(
        ssm_tags(&ssm).await,
        vec![
            ("env".to_string(), "dev".to_string()),
            ("team".to_string(), "platform".to_string()),
        ]
    );

    // Destroy through Cloud Control removes the SSM parameter.
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    let (status, body) = ssm_parameter(&ssm).await;
    assert_eq!(status, 400, "parameter still exists after destroy: {body}");
    assert_eq!(body["__type"], "ParameterNotFound");

    cleanup_tf_dir(&dir);
}

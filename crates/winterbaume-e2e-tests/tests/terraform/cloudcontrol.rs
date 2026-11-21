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
use winterbaume_cloudfront::CloudFrontService;
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

/// Call the native CloudFront API on the same `CloudFrontService` instance
/// that Cloud Control is wired to. Returns the status, the `ETag` header and
/// the raw XML body.
async fn cloudfront_call(
    cloudfront: &CloudFrontService,
    method: &str,
    path: &str,
    if_match: Option<&str>,
    body: String,
) -> (u16, String, String) {
    let mut headers = http::HeaderMap::new();
    if let Some(etag) = if_match {
        headers.insert("if-match", etag.parse().unwrap());
    }
    let response = cloudfront
        .handle(MockRequest {
            method: method.to_string(),
            uri: format!("https://cloudfront.amazonaws.com{path}"),
            headers,
            body: Bytes::from(body),
        })
        .await;
    let etag = response
        .headers
        .get("etag")
        .map(|v| v.to_str().unwrap().to_string())
        .unwrap_or_default();
    let body = String::from_utf8_lossy(&response.body).into_owned();
    (response.status, etag, body)
}

/// Start an isolated server where Cloud Control and the native CloudFront API
/// share one `CloudFrontService`, as `winterbaume-server` wires them.
async fn start_cloudfront_server() -> (Arc<CloudFrontService>, String) {
    let cloudfront = Arc::new(CloudFrontService::new());
    let cloudcontrol =
        Arc::new(CloudControlService::new().with_cloudfront_service(Arc::clone(&cloudfront)));
    let port = start_server(vec![
        Arc::new(winterbaume_sts::StsService::new()),
        Arc::clone(&cloudfront) as Arc<dyn MockService>,
        cloudcontrol,
    ])
    .await;
    (cloudfront, format!("http://127.0.0.1:{port}"))
}

const RHP_NAME: &str = "awscc-e2e-rhp";

/// All six `SecurityHeadersConfig` members are set. Cloud Control reports
/// unset members as empty objects and then rejects any patch applied on top
/// of them (see the partial-config test below), so only a complete
/// `SecurityHeadersConfig` can be updated in place.
fn response_headers_policy_tf(comment: &str, value: &str) -> String {
    format!(
        r#"
resource "awscc_cloudfront_response_headers_policy" "test" {{
  response_headers_policy_config = {{
    name    = "{RHP_NAME}"
    comment = "{comment}"
    custom_headers_config = {{
      items = [{{
        header   = "X-Probe"
        value    = "{value}"
        override = true
      }}]
    }}
    security_headers_config = {{
      content_security_policy = {{
        content_security_policy = "default-src 'self'"
        override                = true
      }}
      content_type_options = {{
        override = true
      }}
      frame_options = {{
        frame_option = "DENY"
        override     = true
      }}
      referrer_policy = {{
        referrer_policy = "same-origin"
        override        = true
      }}
      strict_transport_security = {{
        access_control_max_age_sec = 31536000
        include_subdomains         = true
        override                   = true
      }}
      xss_protection = {{
        protection = true
        mode_block = true
        override   = true
      }}
    }}
  }}
}}
"#
    )
}

/// Only two `SecurityHeadersConfig` members are set, as in the issue #37
/// reproduction.
fn partial_response_headers_policy_tf(comment: &str) -> String {
    format!(
        r#"
resource "awscc_cloudfront_response_headers_policy" "test" {{
  response_headers_policy_config = {{
    name    = "{RHP_NAME}"
    comment = "{comment}"
    security_headers_config = {{
      content_security_policy = {{
        content_security_policy = "default-src 'self'"
        override                = true
      }}
      content_type_options = {{
        override = true
      }}
    }}
  }}
}}
"#
    )
}

// Issue #37: `AWS::CloudFront::ResponseHeadersPolicy` must be a real
// CloudFront response headers policy whose nested configuration round-trips
// through Terraform without drift.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_awscc_cloudfront_response_headers_policy_backed_by_cloudfront() {
    let (cloudfront, url) = start_cloudfront_server().await;
    let dir = create_tf_dir("awscc-cloudfront-rhp");
    write_awscc_provider_tf(&dir, &url);

    // Create through Cloud Control.
    std::fs::write(
        dir.join("main.tf"),
        response_headers_policy_tf("probe", "v1"),
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let attributes = state_attributes(&dir);
    let id = attributes["response_headers_policy_id"]
        .as_str()
        .expect("response_headers_policy_id must be set")
        .to_string();
    assert_eq!(attributes["id"], id.as_str());
    assert!(
        attributes["last_modified_time"]
            .as_str()
            .is_some_and(|t| !t.is_empty()),
        "last_modified_time must be set: {attributes}"
    );
    let config = &attributes["response_headers_policy_config"];
    assert_eq!(config["name"], RHP_NAME);
    assert_eq!(config["custom_headers_config"]["items"][0]["value"], "v1");
    assert_eq!(
        config["security_headers_config"]["frame_options"]["frame_option"],
        "DENY"
    );

    // The policy is a native CloudFront response headers policy.
    let policy_path = format!("/2020-05-31/response-headers-policy/{id}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &policy_path, None, String::new()).await;
    assert_eq!(status, 200, "GetResponseHeadersPolicy failed: {body}");
    assert!(body.contains(&format!("<Name>{RHP_NAME}</Name>")), "{body}");
    assert!(body.contains("<Comment>probe</Comment>"), "{body}");
    assert!(body.contains("<Header>X-Probe</Header>"), "{body}");
    assert!(body.contains("<Value>v1</Value>"), "{body}");
    assert!(body.contains("<FrameOption>DENY</FrameOption>"), "{body}");
    assert!(
        body.contains("<ReferrerPolicy>same-origin</ReferrerPolicy>"),
        "{body}"
    );
    assert!(
        body.contains("<AccessControlMaxAgeSec>31536000</AccessControlMaxAgeSec>"),
        "{body}"
    );

    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    // An out-of-band CloudFront update is visible to Cloud Control reads.
    let config_path = format!("{policy_path}/config");
    let (status, etag, config_xml) =
        cloudfront_call(&cloudfront, "GET", &config_path, None, String::new()).await;
    assert_eq!(
        status, 200,
        "GetResponseHeadersPolicyConfig failed: {config_xml}"
    );
    let changed = config_xml.replace("<Comment>probe</Comment>", "<Comment>out-of-band</Comment>");
    assert_ne!(changed, config_xml, "comment not found in {config_xml}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "PUT", &policy_path, Some(&etag), changed).await;
    assert_eq!(status, 200, "UpdateResponseHeadersPolicy failed: {body}");
    let (ok, stdout, _stderr) = terraform_plan(&dir).await;
    assert!(!ok, "expected drift after out-of-band update:\n{stdout}");
    assert!(
        stdout.contains("1 to change"),
        "Expected an in-place update to revert the out-of-band comment:\n{stdout}"
    );

    // An in-place update through Cloud Control reaches CloudFront.
    std::fs::write(
        dir.join("main.tf"),
        response_headers_policy_tf("probe", "v2"),
    )
    .unwrap();
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &policy_path, None, String::new()).await;
    assert_eq!(status, 200, "GetResponseHeadersPolicy failed: {body}");
    assert!(body.contains("<Comment>probe</Comment>"), "{body}");
    assert!(body.contains("<Value>v2</Value>"), "{body}");

    // Destroy through Cloud Control removes the CloudFront policy.
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &policy_path, None, String::new()).await;
    assert_eq!(status, 404, "policy still exists after destroy: {body}");
    assert!(body.contains("NoSuchResponseHeadersPolicy"), "{body}");

    cleanup_tf_dir(&dir);
}

// Issue #37: real Cloud Control reports unset `SecurityHeadersConfig`
// members as empty objects, so an in-place update of a policy that sets only
// some of them fails model validation even though the patch touches only
// `Comment`. Terraform must surface that error and leave the policy alone.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_awscc_cloudfront_response_headers_policy_partial_security_headers_update_rejected() {
    let (cloudfront, url) = start_cloudfront_server().await;
    let dir = create_tf_dir("awscc-cloudfront-rhp-partial");
    write_awscc_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        partial_response_headers_policy_tf("probe"),
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let id = state_attributes(&dir)["response_headers_policy_id"]
        .as_str()
        .expect("response_headers_policy_id must be set")
        .to_string();

    // The empty objects in the read model do not show up as drift.
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    std::fs::write(
        dir.join("main.tf"),
        partial_response_headers_policy_tf("probe2"),
    )
    .unwrap();
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(
        !ok,
        "update over a partial SecurityHeadersConfig must fail:\n{stdout}"
    );
    // Terraform wraps the error text, so compare it with whitespace collapsed.
    let error = stderr.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        error.contains("ValidationException: Model validation failed"),
        "{stderr}"
    );
    assert!(
        error.contains("SecurityHeadersConfig/FrameOptions: required key [Override] not found"),
        "{stderr}"
    );

    // CloudFront still has the original comment.
    let policy_path = format!("/2020-05-31/response-headers-policy/{id}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &policy_path, None, String::new()).await;
    assert_eq!(status, 200, "GetResponseHeadersPolicy failed: {body}");
    assert!(body.contains("<Comment>probe</Comment>"), "{body}");

    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &policy_path, None, String::new()).await;
    assert_eq!(status, 404, "policy still exists after destroy: {body}");

    cleanup_tf_dir(&dir);
}

const FUNCTION_NAME: &str = "awscc-e2e-function";

fn function_tf(code: &str) -> String {
    format!(
        r#"
resource "awscc_cloudfront_function" "test" {{
  name          = "{FUNCTION_NAME}"
  function_code = "{code}"
  auto_publish  = true
  function_config = {{
    comment = "probe"
    runtime = "cloudfront-js-2.0"
  }}
}}
"#
    )
}

// Issue #38: `AWS::CloudFront::Function` must be a real CloudFront function
// identified by its ARN, and `AutoPublish` must publish every code change to
// the LIVE stage.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_awscc_cloudfront_function_backed_by_cloudfront() {
    let (cloudfront, url) = start_cloudfront_server().await;
    let dir = create_tf_dir("awscc-cloudfront-function");
    write_awscc_provider_tf(&dir, &url);
    let code_v1 = "function handler(event) { return event.request; }";
    let code_v2 = "function handler(event) { var r = event.request; return r; }";
    let arn = format!("arn:aws:cloudfront::123456789012:function/{FUNCTION_NAME}");
    let live_path = format!("/2020-05-31/function/{FUNCTION_NAME}?Stage=LIVE");
    let describe_live_path = format!("/2020-05-31/function/{FUNCTION_NAME}/describe?Stage=LIVE");

    // Create through Cloud Control.
    std::fs::write(dir.join("main.tf"), function_tf(code_v1)).unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let attributes = state_attributes(&dir);
    assert_eq!(attributes["id"], arn.as_str());
    assert_eq!(attributes["function_arn"], arn.as_str());
    assert_eq!(
        attributes["function_metadata"]["function_arn"],
        arn.as_str()
    );
    assert_eq!(attributes["stage"], "LIVE");
    assert_eq!(attributes["auto_publish"], true);
    assert_eq!(attributes["function_code"], code_v1);

    // AutoPublish put the code on the native LIVE stage.
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &live_path, None, String::new()).await;
    assert_eq!(status, 200, "GetFunction (LIVE) failed: {body}");
    assert_eq!(body, code_v1);

    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    // A code change through Cloud Control is published to LIVE.
    std::fs::write(dir.join("main.tf"), function_tf(code_v2)).unwrap();
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &live_path, None, String::new()).await;
    assert_eq!(status, 200, "GetFunction (LIVE) failed: {body}");
    assert_eq!(body, code_v2);

    // A function deleted natively is gone for Cloud Control too.
    let function_path = format!("/2020-05-31/function/{FUNCTION_NAME}");
    let (status, etag, body) =
        cloudfront_call(&cloudfront, "GET", &describe_live_path, None, String::new()).await;
    assert_eq!(status, 200, "DescribeFunction (LIVE) failed: {body}");
    let (status, _etag, body) = cloudfront_call(
        &cloudfront,
        "DELETE",
        &function_path,
        Some(&etag),
        String::new(),
    )
    .await;
    assert!(
        status == 204 || status == 200,
        "DeleteFunction failed ({status}): {body}"
    );
    let (ok, stdout, _stderr) = terraform_plan(&dir).await;
    assert!(!ok, "expected drift after out-of-band delete:\n{stdout}");
    assert!(
        stdout.contains("1 to add"),
        "Expected the natively deleted function to be recreated:\n{stdout}"
    );

    // Recreate, then destroy through Cloud Control removes the function.
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "re-apply after out-of-band delete failed:\n{stderr}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &live_path, None, String::new()).await;
    assert_eq!(
        status, 200,
        "GetFunction (LIVE) failed after recreate: {body}"
    );
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    let (status, _etag, body) =
        cloudfront_call(&cloudfront, "GET", &describe_live_path, None, String::new()).await;
    assert_eq!(status, 404, "function still exists after destroy: {body}");
    assert!(body.contains("NoSuchFunctionExists"), "{body}");

    cleanup_tf_dir(&dir);
}

//! `AWS::CloudFront::ResponseHeadersPolicy` (issue #37) and
//! `AWS::CloudFront::Function` (issue #38) are backed by live CloudFront state.

use std::sync::Arc;

use aws_sdk_cloudfront::primitives::Blob;
use aws_sdk_cloudfront::types::{FunctionConfig, FunctionRuntime, FunctionStage};
use serde_json::{Value, json};
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_cloudfront::CloudFrontService;
use winterbaume_core::{MockAws, default_account_id};

const RHP: &str = "AWS::CloudFront::ResponseHeadersPolicy";
const FUNCTION: &str = "AWS::CloudFront::Function";
const CODE: &str = "function handler(event) { return event.request; }";

async fn clients() -> (aws_sdk_cloudcontrol::Client, aws_sdk_cloudfront::Client) {
    let cloudfront = Arc::new(CloudFrontService::new());
    let cloudcontrol = CloudControlService::new().with_cloudfront_service(Arc::clone(&cloudfront));
    let mock = MockAws::builder()
        .with_service(cloudfront)
        .with_service(cloudcontrol)
        .build();
    let config = mock.sdk_config("us-east-1").await;
    (
        aws_sdk_cloudcontrol::Client::new(&config),
        aws_sdk_cloudfront::Client::new(&config),
    )
}

async fn create(cc: &aws_sdk_cloudcontrol::Client, type_name: &str, desired: Value) -> String {
    let created = cc
        .create_resource()
        .type_name(type_name)
        .desired_state(desired.to_string())
        .send()
        .await
        .unwrap();
    let event = created.progress_event().unwrap();
    assert_eq!(event.operation_status().unwrap().as_str(), "SUCCESS");
    event.identifier().unwrap().to_string()
}

async fn properties(cc: &aws_sdk_cloudcontrol::Client, type_name: &str, id: &str) -> Value {
    let result = cc
        .get_resource()
        .type_name(type_name)
        .identifier(id)
        .send()
        .await
        .unwrap();
    serde_json::from_str(result.resource_description().unwrap().properties().unwrap()).unwrap()
}

async fn update_error(
    cc: &aws_sdk_cloudcontrol::Client,
    type_name: &str,
    id: &str,
    patch: Value,
) -> (String, String) {
    let err = cc
        .update_resource()
        .type_name(type_name)
        .identifier(id)
        .patch_document(patch.to_string())
        .send()
        .await
        .expect_err("update must fail");
    let err = err.into_service_error();
    let meta = aws_sdk_cloudcontrol::error::ProvideErrorMetadata::meta(&err);
    (
        meta.code().unwrap_or_default().to_string(),
        meta.message().unwrap_or_default().to_string(),
    )
}

fn code_of<E: aws_sdk_cloudcontrol::error::ProvideErrorMetadata>(err: &E) -> String {
    err.code().unwrap_or_default().to_string()
}

fn rhp_desired(name: &str) -> Value {
    json!({"ResponseHeadersPolicyConfig": {
        "Name": name,
        "Comment": "probe",
        "SecurityHeadersConfig": {
            "ContentSecurityPolicy": {
                "ContentSecurityPolicy": "default-src 'self'", "Override": true
            },
            "ContentTypeOptions": {"Override": true}
        }
    }})
}

#[tokio::test]
async fn response_headers_policy_read_model_matches_aws() {
    let (cc, cf) = clients().await;
    let id = create(&cc, RHP, rhp_desired("carina-awscc-469-probe")).await;
    let model = properties(&cc, RHP, &id).await;

    // readOnlyProperties: Id is the primary identifier; LastModifiedTime is
    // an ISO 8601 timestamp with milliseconds.
    assert_eq!(model["Id"], id.as_str());
    let modified = model["LastModifiedTime"].as_str().unwrap();
    assert!(chrono::DateTime::parse_from_rfc3339(modified).is_ok());
    assert!(
        modified.ends_with('Z') && modified.len() == 24,
        "{modified}"
    );
    // Unset SecurityHeadersConfig members come back as empty objects.
    assert_eq!(
        model["ResponseHeadersPolicyConfig"],
        json!({
            "Name": "carina-awscc-469-probe",
            "Comment": "probe",
            "SecurityHeadersConfig": {
                "ContentSecurityPolicy": {
                    "ContentSecurityPolicy": "default-src 'self'", "Override": true
                },
                "ContentTypeOptions": {"Override": true},
                "FrameOptions": {},
                "ReferrerPolicy": {},
                "StrictTransportSecurity": {},
                "XSSProtection": {}
            }
        })
    );

    // The policy is a real CloudFront policy.
    let policy = cf
        .get_response_headers_policy()
        .id(&id)
        .send()
        .await
        .unwrap();
    let config = policy
        .response_headers_policy()
        .unwrap()
        .response_headers_policy_config()
        .unwrap();
    assert_eq!(config.name(), "carina-awscc-469-probe");
    assert_eq!(config.comment(), Some("probe"));
    let list = cc.list_resources().type_name(RHP).send().await.unwrap();
    assert_eq!(list.resource_descriptions().len(), 1);
    assert_eq!(
        list.resource_descriptions()[0].identifier(),
        Some(id.as_str())
    );
}

#[tokio::test]
async fn response_headers_policy_nested_patch_fails_model_validation() {
    let (cc, cf) = clients().await;
    let id = create(&cc, RHP, rhp_desired("nested-patch")).await;
    let (code, message) = update_error(
        &cc,
        RHP,
        &id,
        json!([{"op": "replace", "path": "/ResponseHeadersPolicyConfig/Comment", "value": "probe2"}]),
    )
    .await;
    assert_eq!(code, "ValidationException");
    assert!(
        message.starts_with("Model validation failed ("),
        "{message}"
    );
    for expected in [
        "#/ResponseHeadersPolicyConfig/SecurityHeadersConfig/FrameOptions: required key [Override] not found",
        "#/ResponseHeadersPolicyConfig/SecurityHeadersConfig/FrameOptions: required key [FrameOption] not found",
        "#/ResponseHeadersPolicyConfig/SecurityHeadersConfig/ReferrerPolicy: required key [ReferrerPolicy] not found",
    ] {
        assert!(message.contains(expected), "{message}");
    }
    // The failed update left CloudFront unchanged.
    let policy = cf
        .get_response_headers_policy()
        .id(&id)
        .send()
        .await
        .unwrap();
    let config = policy.response_headers_policy().unwrap();
    assert_eq!(
        config.response_headers_policy_config().unwrap().comment(),
        Some("probe")
    );

    // Replacing the whole config succeeds in place and keeps the Id.
    let mut replacement = rhp_desired("nested-patch")["ResponseHeadersPolicyConfig"].clone();
    replacement["Comment"] = json!("probe2");
    replacement["CustomHeadersConfig"] =
        json!({"Items": [{"Header": "X-Probe", "Value": "1", "Override": false}]});
    let updated = cc
        .update_resource()
        .type_name(RHP)
        .identifier(&id)
        .patch_document(
            json!([{"op": "replace", "path": "/ResponseHeadersPolicyConfig", "value": replacement}])
                .to_string(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(
        updated.progress_event().unwrap().identifier(),
        Some(id.as_str())
    );
    let model = properties(&cc, RHP, &id).await;
    assert_eq!(model["Id"], id.as_str());
    assert_eq!(model["ResponseHeadersPolicyConfig"]["Comment"], "probe2");
    assert_eq!(
        model["ResponseHeadersPolicyConfig"]["CustomHeadersConfig"],
        json!({"Items": [{"Header": "X-Probe", "Value": "1", "Override": false}]})
    );
    let policy = cf
        .get_response_headers_policy()
        .id(&id)
        .send()
        .await
        .unwrap();
    let config = policy
        .response_headers_policy()
        .unwrap()
        .response_headers_policy_config()
        .unwrap();
    assert_eq!(config.comment(), Some("probe2"));
    assert_eq!(config.custom_headers_config().unwrap().quantity(), 1);

    // Read-only properties cannot be patched.
    let (code, _) = update_error(
        &cc,
        RHP,
        &id,
        json!([{"op": "replace", "path": "/Id", "value": "other"}]),
    )
    .await;
    assert_eq!(code, "InvalidRequestException");

    cc.delete_resource()
        .type_name(RHP)
        .identifier(&id)
        .send()
        .await
        .unwrap();
    let err = cf
        .get_response_headers_policy()
        .id(&id)
        .send()
        .await
        .expect_err("policy must be deleted");
    assert!(
        err.into_service_error()
            .is_no_such_response_headers_policy()
    );
}

#[tokio::test]
async fn response_headers_policy_create_validation_and_duplicates() {
    let (cc, cf) = clients().await;
    let err = cc
        .create_resource()
        .type_name(RHP)
        .desired_state(
            json!({"ResponseHeadersPolicyConfig": {"Name": "bad", "SecurityHeadersConfig": {
                "FrameOptions": {"FrameOption": "ALLOW", "Override": true}
            }}})
            .to_string(),
        )
        .send()
        .await
        .expect_err("invalid FrameOption must be rejected");
    assert_eq!(code_of(&err), "ValidationException", "{err:?}");
    assert!(
        cf.list_response_headers_policies()
            .send()
            .await
            .unwrap()
            .response_headers_policy_list()
            .map(|l| l.quantity())
            .unwrap_or_default()
            == 0
    );

    create(&cc, RHP, rhp_desired("dup")).await;
    let err = cc
        .create_resource()
        .type_name(RHP)
        .desired_state(rhp_desired("dup").to_string())
        .send()
        .await
        .expect_err("duplicate name must be rejected");
    assert!(err.into_service_error().is_already_exists_exception());
}

#[tokio::test]
async fn response_headers_policy_created_natively_is_visible() {
    let (cc, cf) = clients().await;
    let created = cf
        .create_response_headers_policy()
        .response_headers_policy_config(
            aws_sdk_cloudfront::types::ResponseHeadersPolicyConfig::builder()
                .name("native")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let id = created.response_headers_policy().unwrap().id().to_string();
    let model = properties(&cc, RHP, &id).await;
    assert_eq!(
        model["ResponseHeadersPolicyConfig"],
        json!({"Name": "native"})
    );
}

fn function_desired(name: &str, auto_publish: bool) -> Value {
    json!({
        "Name": name,
        "AutoPublish": auto_publish,
        "FunctionCode": CODE,
        "FunctionConfig": {"Runtime": "cloudfront-js-2.0", "Comment": "probe"}
    })
}

async fn live_code(cf: &aws_sdk_cloudfront::Client, name: &str) -> String {
    let live = cf
        .get_function()
        .name(name)
        .stage(FunctionStage::Live)
        .send()
        .await
        .unwrap();
    String::from_utf8(live.function_code().unwrap().as_ref().to_vec()).unwrap()
}

#[tokio::test]
async fn function_read_model_matches_aws() {
    let (cc, cf) = clients().await;
    let name = "carina-awscc-474-probe";
    let arn = format!(
        "arn:aws:cloudfront::{}:function/{name}",
        default_account_id()
    );
    let id = create(&cc, FUNCTION, function_desired(name, true)).await;
    assert_eq!(id, arn);
    assert_eq!(
        properties(&cc, FUNCTION, &arn).await,
        json!({
            "FunctionConfig": {"Comment": "probe", "Runtime": "cloudfront-js-2.0"},
            "FunctionARN": arn,
            "FunctionMetadata": {"FunctionARN": arn},
            "FunctionCode": CODE,
            "Stage": "LIVE",
            "Tags": [],
            "Name": name
        })
    );
    // Name is not an identifier.
    let err = cc
        .get_resource()
        .type_name(FUNCTION)
        .identifier(name)
        .send()
        .await
        .expect_err("lookup by Name must fail");
    assert!(err.into_service_error().is_resource_not_found_exception());

    let described = cf
        .describe_function()
        .name(name)
        .stage(FunctionStage::Live)
        .send()
        .await
        .unwrap();
    let summary = described.function_summary().unwrap();
    assert_eq!(
        summary.function_metadata().unwrap().stage(),
        Some(&FunctionStage::Live)
    );
    assert_eq!(summary.function_config().unwrap().comment(), "probe");
    assert_eq!(live_code(&cf, name).await, CODE);
}

#[tokio::test]
async fn function_auto_publish_controls_live_stage() {
    let (cc, cf) = clients().await;
    let name = "staged";
    let arn = create(&cc, FUNCTION, function_desired(name, false)).await;
    assert_eq!(
        properties(&cc, FUNCTION, &arn).await["Stage"],
        "DEVELOPMENT"
    );
    let err = cf
        .describe_function()
        .name(name)
        .stage(FunctionStage::Live)
        .send()
        .await
        .expect_err("an unpublished function has no LIVE stage");
    assert!(err.into_service_error().is_no_such_function_exists());

    // FunctionCode plus AutoPublish publishes the new code to LIVE.
    let v2 = "function handler(event) { return event.response; }";
    cc.update_resource()
        .type_name(FUNCTION)
        .identifier(&arn)
        .patch_document(
            json!([
                {"op": "replace", "path": "/FunctionCode", "value": v2},
                {"op": "add", "path": "/AutoPublish", "value": true}
            ])
            .to_string(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(live_code(&cf, name).await, v2);
    let model = properties(&cc, FUNCTION, &arn).await;
    assert_eq!(model["Stage"], "LIVE");
    assert!(model.get("AutoPublish").is_none());

    // FunctionCode alone updates DEVELOPMENT; LIVE keeps the old code, while
    // GetResource reports the new code and Stage LIVE.
    let v3 = "function handler(event) { return event; }";
    cc.update_resource()
        .type_name(FUNCTION)
        .identifier(&arn)
        .patch_document(
            json!([{"op": "replace", "path": "/FunctionCode", "value": v3}]).to_string(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(live_code(&cf, name).await, v2);
    let development = cf.get_function().name(name).send().await.unwrap();
    assert_eq!(development.function_code().unwrap().as_ref(), v3.as_bytes());
    let model = properties(&cc, FUNCTION, &arn).await;
    assert_eq!(model["FunctionCode"], v3);
    assert_eq!(model["Stage"], "LIVE");

    // Tags are updatable and visible through CloudFront tagging.
    cc.update_resource()
        .type_name(FUNCTION)
        .identifier(&arn)
        .patch_document(
            json!([{"op": "add", "path": "/Tags", "value": [{"Key": "env", "Value": "dev"}]}])
                .to_string(),
        )
        .send()
        .await
        .unwrap();
    let tags = cf
        .list_tags_for_resource()
        .resource(&arn)
        .send()
        .await
        .unwrap();
    let items = tags.tags().unwrap().items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].key(), "env");

    // Name is create-only and Stage is read-only.
    for patch in [
        json!([{"op": "replace", "path": "/Name", "value": "renamed"}]),
        json!([{"op": "replace", "path": "/Stage", "value": "DEVELOPMENT"}]),
    ] {
        let (code, _) = update_error(&cc, FUNCTION, &arn, patch).await;
        assert_eq!(code, "InvalidRequestException");
    }

    cc.delete_resource()
        .type_name(FUNCTION)
        .identifier(&arn)
        .send()
        .await
        .unwrap();
    let err = cf
        .describe_function()
        .name(name)
        .send()
        .await
        .expect_err("function must be deleted");
    assert!(err.into_service_error().is_no_such_function_exists());
}

#[tokio::test]
async fn function_created_natively_is_visible_and_duplicates_rejected() {
    let (cc, cf) = clients().await;
    cf.create_function()
        .name("native")
        .function_config(
            FunctionConfig::builder()
                .comment("native")
                .runtime(FunctionRuntime::CloudfrontJs20)
                .build()
                .unwrap(),
        )
        .function_code(Blob::new(CODE.as_bytes()))
        .send()
        .await
        .unwrap();
    let arn = format!(
        "arn:aws:cloudfront::{}:function/native",
        default_account_id()
    );
    let model = properties(&cc, FUNCTION, &arn).await;
    assert_eq!(model["FunctionCode"], CODE);
    assert_eq!(model["FunctionConfig"]["Comment"], "native");
    assert_eq!(model["Stage"], "DEVELOPMENT");
    let list = cc
        .list_resources()
        .type_name(FUNCTION)
        .send()
        .await
        .unwrap();
    assert_eq!(list.resource_descriptions().len(), 1);

    let err = cc
        .create_resource()
        .type_name(FUNCTION)
        .desired_state(function_desired("native", false).to_string())
        .send()
        .await
        .expect_err("duplicate function must be rejected");
    assert!(err.into_service_error().is_already_exists_exception());

    let err = cc
        .create_resource()
        .type_name(FUNCTION)
        .desired_state(json!({"Name": "incomplete", "FunctionCode": CODE}).to_string())
        .send()
        .await
        .expect_err("missing FunctionConfig must be rejected");
    assert_eq!(code_of(&err), "ValidationException");
    let message = aws_sdk_cloudcontrol::error::ProvideErrorMetadata::message(&err).unwrap();
    assert_eq!(
        message,
        "Model validation failed (#: required key [FunctionConfig] not found)"
    );
}

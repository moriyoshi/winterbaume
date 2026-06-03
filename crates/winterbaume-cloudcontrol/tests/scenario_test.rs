//! End-to-end scenario tests for Cloud Control API.
//!
//! These exercise the per-CFN-resource-type read-model shaping contract that
//! a Terraform `awscc`-provider E2E run would otherwise catch. Each scenario
//! chains create / get / update / list / delete and asserts on the *content*
//! of the stored model — writeOnly stripped, readOnly generated, schema
//! defaults filled — rather than just on success/failure of each call.

use aws_sdk_cloudcontrol::config::BehaviorVersion;
use serde_json::Value;
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_cloudcontrol::Client {
    let mock = MockAws::builder()
        .with_service(CloudControlService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudcontrol::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_cloudcontrol::Client::new(&config)
}

fn parse_properties(
    resp: &aws_sdk_cloudcontrol::operation::get_resource::GetResourceOutput,
) -> Value {
    let json = resp
        .resource_description()
        .and_then(|d| d.properties())
        .expect("GetResource should return properties");
    serde_json::from_str(json).expect("properties must be valid JSON")
}

/// Scenario: AWS::KMS::Key full lifecycle with CFN-schema shaping.
///
/// Walks the same path a Terraform `awscc_kms_key` plan would: create with a
/// minimal DesiredState containing one writeOnly property, read back and
/// verify the schema-shaped output, mutate via JSON-Patch, re-verify, list,
/// and delete. Asserts at every step that the shaper's discipline holds:
/// `writeOnly` stays stripped, `readOnly` (`KeyId`, `Arn`) survives unchanged
/// across updates, and `default`s are stable.
#[tokio::test]
async fn scenario_kms_key_full_lifecycle_applies_cfn_schema() {
    let client = make_client().await;

    // Step 1: Create with writeOnly + pass-through properties, no defaults set.
    let create_resp = client
        .create_resource()
        .type_name("AWS::KMS::Key")
        .desired_state(
            r#"{
                "Description": "scenario probe",
                "KeyPolicy": {
                    "Version": "2012-10-17",
                    "Statement": [{
                        "Effect": "Allow",
                        "Principal": {"AWS": "arn:aws:iam::123456789012:root"},
                        "Action": "kms:*",
                        "Resource": "*"
                    }]
                },
                "PendingWindowInDays": 7,
                "BypassPolicyLockoutSafetyCheck": true
            }"#,
        )
        .send()
        .await
        .expect("create should succeed");

    let key_id = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("primary identifier (KeyId) should be returned")
        .to_string();
    assert!(!key_id.is_empty(), "KeyId must not be empty");

    // Step 2: GetResource — full shape assertions.
    let get1 = client
        .get_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .send()
        .await
        .expect("first get should succeed");
    let props1 = parse_properties(&get1);

    // writeOnly: both should be stripped.
    assert!(props1.get("PendingWindowInDays").is_none());
    assert!(props1.get("BypassPolicyLockoutSafetyCheck").is_none());

    // readOnly: generated.
    assert_eq!(props1["KeyId"].as_str(), Some(key_id.as_str()));
    let arn = props1["Arn"].as_str().expect("Arn must be present");
    assert!(arn.starts_with("arn:aws:kms:us-east-1:"), "got Arn={arn}");
    assert!(arn.ends_with(&format!(":key/{key_id}")), "got Arn={arn}");

    // Defaults: all seven filled.
    assert_eq!(props1["KeySpec"], "SYMMETRIC_DEFAULT");
    assert_eq!(props1["KeyUsage"], "ENCRYPT_DECRYPT");
    assert_eq!(props1["Origin"], "AWS_KMS");
    assert_eq!(props1["MultiRegion"], false);
    assert_eq!(props1["Enabled"], true);
    assert_eq!(props1["EnableKeyRotation"], false);
    assert!(props1["Tags"].as_array().unwrap().is_empty());

    // Pass-through.
    assert_eq!(props1["Description"], "scenario probe");
    assert!(props1["KeyPolicy"].is_object());

    // Step 3: ListResources of AWS::KMS::Key includes this key.
    let listed = client
        .list_resources()
        .type_name("AWS::KMS::Key")
        .send()
        .await
        .expect("list should succeed");
    let ids: Vec<&str> = listed
        .resource_descriptions()
        .iter()
        .filter_map(|d| d.identifier())
        .collect();
    assert!(
        ids.contains(&key_id.as_str()),
        "ListResources should surface KeyId {key_id}; got {ids:?}"
    );

    // Step 4: UpdateResource — flip a default, change a pass-through, and try
    // to smuggle back a writeOnly property. The shaper must drop the writeOnly
    // re-introduction on the way back to storage.
    client
        .update_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .patch_document(
            r#"[
                {"op": "replace", "path": "/Description", "value": "after update"},
                {"op": "replace", "path": "/EnableKeyRotation", "value": true},
                {"op": "add", "path": "/PendingWindowInDays", "value": 30}
            ]"#,
        )
        .send()
        .await
        .expect("update should succeed");

    // Step 5: GetResource — re-assert shape after update.
    let get2 = client
        .get_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .send()
        .await
        .expect("second get should succeed");
    let props2 = parse_properties(&get2);

    // writeOnly re-introduced via patch must be stripped again.
    assert!(
        props2.get("PendingWindowInDays").is_none(),
        "writeOnly property reintroduced by patch must be stripped on store"
    );

    // readOnly: KeyId + Arn must survive the update unchanged.
    assert_eq!(props2["KeyId"].as_str(), Some(key_id.as_str()));
    assert_eq!(props2["Arn"].as_str(), Some(arn));

    // The mutated pass-through and default are observable.
    assert_eq!(props2["Description"], "after update");
    assert_eq!(props2["EnableKeyRotation"], true);

    // Unmodified defaults still present.
    assert_eq!(props2["KeySpec"], "SYMMETRIC_DEFAULT");
    assert_eq!(props2["Origin"], "AWS_KMS");

    // Step 6: Delete, then confirm subsequent Get fails with the typed
    // ResourceNotFoundException variant.
    client
        .delete_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .send()
        .await
        .expect("delete should succeed");

    let err = client
        .get_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .send()
        .await
        .expect_err("get should fail after delete");
    assert!(err.into_service_error().is_resource_not_found_exception());
}

/// Scenario: an unshaped type round-trips verbatim.
///
/// Regression guard against accidentally routing every type through a shaper
/// once the registry grows. Types without a registered `CfnResourceShaper`
/// must keep the legacy "store DesiredState verbatim" behaviour so the
/// existing crate-level integration suite for S3 / Lambda / DynamoDB
/// continues to work.
#[tokio::test]
async fn scenario_unshaped_type_round_trips_verbatim() {
    let client = make_client().await;

    let desired = r#"{"BucketName":"scenario-bkt","Tags":[{"Key":"env","Value":"test"}],"VersioningConfiguration":{"Status":"Enabled"}}"#;

    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(desired)
        .send()
        .await
        .expect("create should succeed");

    let get = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("scenario-bkt")
        .send()
        .await
        .expect("get should succeed");
    let props = parse_properties(&get);

    // The stored model is exactly the DesiredState — no synthetic Arn,
    // no synthetic defaults — because no shaper is registered.
    assert_eq!(props["BucketName"], "scenario-bkt");
    assert!(props.get("Arn").is_none());
    assert_eq!(props["VersioningConfiguration"]["Status"], "Enabled");
    assert_eq!(props["Tags"][0]["Key"], "env");
}

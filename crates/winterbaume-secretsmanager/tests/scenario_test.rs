//! Smoke tests for winterbaume Secrets Manager service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_secretsmanager::config::BehaviorVersion;
use aws_sdk_secretsmanager::types::{RotationRulesType, Tag};
use winterbaume_core::MockAws;
use winterbaume_secretsmanager::SecretsManagerService;

async fn make_sm_client() -> aws_sdk_secretsmanager::Client {
    let mock = MockAws::builder()
        .with_service(SecretsManagerService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_secretsmanager::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_secretsmanager::Client::new(&config)
}

/// Scenario: secret rotation with manual stage promotion.
///
/// A service stores its DB password in Secrets Manager. The operator:
/// 1. Creates the secret at v1.
/// 2. Configures rotation against a Lambda + 30-day window.
/// 3. Triggers a rotation (which moves AWSCURRENT to a new version).
/// 4. Calls UpdateSecretVersionStage to roll AWSCURRENT back to v1
///    (a typical "rotation broke prod, revert" emergency flow).
/// 5. Verifies GetSecretValue resolves to the v1 plaintext.
#[tokio::test]
async fn test_secret_rotation_then_manual_revert_flow() {
    let client = make_sm_client().await;

    // 1. Create the secret.
    let create = client
        .create_secret()
        .name("db/master")
        .secret_string("password-v1")
        .tags(Tag::builder().key("App").value("orders").build())
        .send()
        .await
        .expect("create_secret");
    let v1_id = create
        .version_id()
        .expect("v1 version_id present")
        .to_string();
    let arn = create.arn().expect("arn present").to_string();
    assert_eq!(create.name(), Some("db/master"));

    // 2. Configure & trigger rotation.
    let rot = client
        .rotate_secret()
        .secret_id("db/master")
        .rotation_lambda_arn("arn:aws:lambda:us-east-1:123456789012:function:RotateDb")
        .rotation_rules(
            RotationRulesType::builder()
                .automatically_after_days(30)
                .build(),
        )
        .send()
        .await
        .expect("rotate_secret");
    let rotated_version = rot
        .version_id()
        .expect("rotation produced new version_id")
        .to_string();
    assert_ne!(
        rotated_version, v1_id,
        "rotation should mint a new version id"
    );
    assert_eq!(rot.arn(), Some(arn.as_str()));

    // Sanity: describe should reflect rotation_enabled.
    let desc = client
        .describe_secret()
        .secret_id("db/master")
        .send()
        .await
        .expect("describe_secret after rotate");
    assert_eq!(desc.rotation_enabled(), Some(true));
    assert!(desc.rotation_lambda_arn().is_some());

    // 3. Emergency revert: move AWSCURRENT back to v1, off the rotated version.
    client
        .update_secret_version_stage()
        .secret_id("db/master")
        .version_stage("AWSCURRENT")
        .move_to_version_id(&v1_id)
        .remove_from_version_id(&rotated_version)
        .send()
        .await
        .expect("update_secret_version_stage moves AWSCURRENT back");

    // 4. GetSecretValue must now return the original v1 plaintext + version id.
    let got = client
        .get_secret_value()
        .secret_id("db/master")
        .send()
        .await
        .expect("get_secret_value after revert");
    assert_eq!(got.secret_string(), Some("password-v1"));
    assert_eq!(got.version_id(), Some(v1_id.as_str()));
}

/// Scenario: PutSecretValue staging without rotation.
///
/// An operator publishes a new version manually via PutSecretValue, then
/// promotes it to AWSCURRENT, demoting the original. This mimics the
/// pattern used by deployment pipelines that pre-stage credentials before
/// flipping traffic over.
#[tokio::test]
async fn test_manual_put_and_stage_promotion_flow() {
    let client = make_sm_client().await;

    // Initial v1.
    let create = client
        .create_secret()
        .name("api/token")
        .secret_string("token-v1")
        .send()
        .await
        .expect("create_secret");
    let v1_id = create.version_id().unwrap().to_string();

    // Pre-stage v2 — PutSecretValue marks the new version AWSCURRENT by default.
    let put = client
        .put_secret_value()
        .secret_id("api/token")
        .secret_string("token-v2")
        .send()
        .await
        .expect("put_secret_value");
    let v2_id = put.version_id().unwrap().to_string();
    assert_ne!(v1_id, v2_id);

    // Verify v2 is current.
    let cur = client
        .get_secret_value()
        .secret_id("api/token")
        .send()
        .await
        .expect("get_secret_value reads current");
    assert_eq!(cur.secret_string(), Some("token-v2"));
    assert_eq!(cur.version_id(), Some(v2_id.as_str()));

    // Roll AWSCURRENT back to v1 (e.g. v2 broke prod).
    client
        .update_secret_version_stage()
        .secret_id("api/token")
        .version_stage("AWSCURRENT")
        .move_to_version_id(&v1_id)
        .remove_from_version_id(&v2_id)
        .send()
        .await
        .expect("update_secret_version_stage rolls back to v1");

    let cur2 = client
        .get_secret_value()
        .secret_id("api/token")
        .send()
        .await
        .expect("get_secret_value after rollback");
    assert_eq!(cur2.secret_string(), Some("token-v1"));
    assert_eq!(cur2.version_id(), Some(v1_id.as_str()));
}

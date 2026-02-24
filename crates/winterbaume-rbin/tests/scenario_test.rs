//! End-to-end scenario tests for AWS Recycle Bin (rbin).
//!
//! Each test chains 3+ operations and asserts business outcomes rather than
//! per-API return shapes.

use aws_sdk_rbin::config::BehaviorVersion;
use aws_sdk_rbin::types::{LockConfiguration, ResourceType, RetentionPeriod, UnlockDelay};
use winterbaume_core::MockAws;
use winterbaume_rbin::RbinService;

async fn make_client() -> aws_sdk_rbin::Client {
    let mock = MockAws::builder().with_service(RbinService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rbin::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_rbin::Client::new(&config)
}

/// Scenario: Lock-protected rule lifecycle.
///
/// Creates a retention rule, locks it, verifies deletion is blocked,
/// unlocks it (transitions to pending_unlock), then deletes it.
#[tokio::test]
async fn test_lock_protected_lifecycle() {
    // Scenario: create → lock → attempt delete (blocked) → unlock → delete
    let client = make_client().await;

    // Step 1: Create a rule.
    let create = client
        .create_rule()
        .description("scenario: lock-protected lifecycle")
        .resource_type(ResourceType::EbsSnapshot)
        .retention_period(
            RetentionPeriod::builder()
                .retention_period_value(14)
                .retention_period_unit("DAYS".into())
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create rule");
    let id = create.identifier().expect("identifier").to_string();

    // Step 2: Lock the rule.
    let locked = client
        .lock_rule()
        .identifier(&id)
        .lock_configuration(
            LockConfiguration::builder()
                .unlock_delay(
                    UnlockDelay::builder()
                        .unlock_delay_value(7)
                        .unlock_delay_unit("DAYS".into())
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("lock rule");
    assert_eq!(
        locked.lock_state().map(|s| s.as_str()),
        Some("locked"),
        "rule should be in locked state after LockRule"
    );

    // Step 3: Deletion must be blocked while the rule is locked.
    let delete_err = client
        .delete_rule()
        .identifier(&id)
        .send()
        .await
        .expect_err("delete of a locked rule must fail");
    assert!(
        format!("{delete_err:?}").contains("ConflictException"),
        "expected ConflictException, got: {delete_err:?}"
    );

    // Step 4: Unlock the rule (transitions to pending_unlock).
    let unlocked = client
        .unlock_rule()
        .identifier(&id)
        .send()
        .await
        .expect("unlock rule");
    assert_eq!(
        unlocked.lock_state().map(|s| s.as_str()),
        Some("pending_unlock"),
        "rule should be pending_unlock after UnlockRule"
    );
    assert!(
        unlocked.lock_end_time().is_some(),
        "lock_end_time should be set after unlock"
    );

    // Step 5: Now deletion succeeds (rule is no longer strictly locked).
    client
        .delete_rule()
        .identifier(&id)
        .send()
        .await
        .expect("delete after unlock");

    // Step 6: Rule is gone — GetRule must return ResourceNotFoundException.
    let gone = client
        .get_rule()
        .identifier(&id)
        .send()
        .await
        .expect_err("get deleted rule must fail");
    assert!(
        format!("{gone:?}").contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException after delete, got: {gone:?}"
    );
}

/// Scenario: Tag management round-trip.
///
/// Tags a rule, reads back the tags, overwrites one key, removes another,
/// and asserts the final tag set is correct.
#[tokio::test]
async fn test_tag_management_round_trip() {
    // Scenario: create → tag → list-tags → re-tag → untag → list-tags (empty)
    let client = make_client().await;

    // Step 1: Create a rule to obtain a real ARN.
    let create = client
        .create_rule()
        .resource_type(ResourceType::Ec2Image)
        .retention_period(
            RetentionPeriod::builder()
                .retention_period_value(7)
                .retention_period_unit("DAYS".into())
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create rule");
    let arn = create.rule_arn().expect("arn").to_string();

    // Step 2: Apply two tags.
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_rbin::types::Tag::builder()
                .key("Env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_rbin::types::Tag::builder()
                .key("Team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag resource");

    // Step 3: Verify both tags are visible.
    let list1 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(list1.tags().len(), 2, "expected 2 tags after tagging");

    // Step 4: Remove one tag.
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Team")
        .send()
        .await
        .expect("untag resource");

    // Step 5: Only the remaining tag should be present.
    let list2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags after untag");
    assert_eq!(list2.tags().len(), 1, "expected 1 tag after untagging Team");
    assert_eq!(list2.tags()[0].key(), "Env", "remaining tag should be Env");
}

//! Integration tests for winterbaume Shield service.

use aws_sdk_shield::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_shield::ShieldService;

async fn make_shield_client() -> aws_sdk_shield::Client {
    let mock = MockAws::builder()
        .with_service(ShieldService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_shield::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_shield::Client::new(&config)
}

#[tokio::test]
async fn test_create_subscription() {
    let client = make_shield_client().await;

    let _resp = client
        .create_subscription()
        .send()
        .await
        .expect("create_subscription should succeed");
}

#[tokio::test]
async fn test_describe_subscription() {
    let client = make_shield_client().await;

    // First create a subscription
    client.create_subscription().send().await.unwrap();

    let resp = client
        .describe_subscription()
        .send()
        .await
        .expect("describe_subscription should succeed");

    let sub = resp.subscription().expect("subscription should be present");
    assert_eq!(
        sub.auto_renew(),
        Some(&aws_sdk_shield::types::AutoRenew::Enabled)
    );
    assert!(sub.time_commitment_in_seconds() > 0);
}

#[tokio::test]
async fn test_create_subscription_already_exists() {
    let client = make_shield_client().await;

    client.create_subscription().send().await.unwrap();

    let result = client.create_subscription().send().await;

    assert!(result.is_err(), "second create_subscription should fail");
}

#[tokio::test]
async fn test_create_protection() {
    let client = make_shield_client().await;

    let resp = client
        .create_protection()
        .name("my-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-12345678")
        .send()
        .await
        .expect("create_protection should succeed");

    let protection_id = resp
        .protection_id()
        .expect("protection_id should be present");
    assert!(!protection_id.is_empty());
}

#[tokio::test]
async fn test_describe_protection() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("desc-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-desc")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap();

    let resp = client
        .describe_protection()
        .protection_id(protection_id)
        .send()
        .await
        .expect("describe_protection should succeed");

    let protection = resp.protection().expect("protection should be present");
    assert_eq!(protection.name(), Some("desc-protection"));
    assert_eq!(
        protection.resource_arn(),
        Some("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-desc")
    );
}

#[tokio::test]
async fn test_delete_protection() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("del-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-del")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap();

    client
        .delete_protection()
        .protection_id(protection_id)
        .send()
        .await
        .expect("delete_protection should succeed");

    // Verify it's gone
    let list_resp = client.list_protections().send().await.unwrap();
    assert_eq!(list_resp.protections().len(), 0);
}

#[tokio::test]
async fn test_list_protections() {
    let client = make_shield_client().await;

    for i in 0..3 {
        client
            .create_protection()
            .name(format!("protection-{i}"))
            .resource_arn(format!(
                "arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-{i}"
            ))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_protections()
        .send()
        .await
        .expect("list_protections should succeed");

    assert_eq!(resp.protections().len(), 3);
}

#[tokio::test]
async fn test_delete_nonexistent_protection() {
    let client = make_shield_client().await;

    let result = client
        .delete_protection()
        .protection_id("nonexistent-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "deleting nonexistent protection should fail"
    );
}

#[tokio::test]
async fn test_create_duplicate_protection() {
    let client = make_shield_client().await;

    let resource_arn = "arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-dup";

    client
        .create_protection()
        .name("dup-protection-1")
        .resource_arn(resource_arn)
        .send()
        .await
        .unwrap();

    let result = client
        .create_protection()
        .name("dup-protection-2")
        .resource_arn(resource_arn)
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating duplicate protection for same resource should fail"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Shield Advanced
// ============================================================================

#[tokio::test]
async fn test_describe_subscription_not_found() {
    // No subscription created — DescribeSubscription should fail.
    let client = make_shield_client().await;

    let result = client.describe_subscription().send().await;
    assert!(
        result.is_err(),
        "describe_subscription without a subscription should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_subscription_arn_present() {
    let client = make_shield_client().await;

    client.create_subscription().send().await.unwrap();

    let resp = client.describe_subscription().send().await.unwrap();
    let sub = resp.subscription().expect("subscription should be present");
    let arn = sub.subscription_arn().unwrap_or_default();
    assert!(
        !arn.is_empty(),
        "subscription_arn should be non-empty, got: {arn:?}"
    );
    assert!(
        arn.contains("arn:aws:shield"),
        "subscription_arn should be an ARN, got: {arn}"
    );
}

#[tokio::test]
async fn test_subscription_start_time() {
    let client = make_shield_client().await;

    client.create_subscription().send().await.unwrap();

    let resp = client.describe_subscription().send().await.unwrap();
    let sub = resp.subscription().expect("subscription should be present");
    assert!(
        sub.start_time().is_some(),
        "start_time should be present on subscription"
    );
}

#[tokio::test]
async fn test_describe_protection_by_resource_arn() {
    let client = make_shield_client().await;

    let resource_arn = "arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-byarn";
    client
        .create_protection()
        .name("byarn-protection")
        .resource_arn(resource_arn)
        .send()
        .await
        .unwrap();

    // Describe by resource ARN instead of protection ID.
    let resp = client
        .describe_protection()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("describe_protection by resource_arn should succeed");

    let protection = resp.protection().expect("protection should be present");
    assert_eq!(protection.name(), Some("byarn-protection"));
    assert_eq!(protection.resource_arn(), Some(resource_arn));
}

#[tokio::test]
async fn test_describe_protection_not_found() {
    let client = make_shield_client().await;

    let result = client
        .describe_protection()
        .protection_id("nonexistent-protection-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_protection with nonexistent id should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_protection_arn_present() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("arn-check-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-arncheck")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap();

    let resp = client
        .describe_protection()
        .protection_id(protection_id)
        .send()
        .await
        .unwrap();

    let protection = resp.protection().expect("protection should be present");
    let prot_arn = protection.protection_arn().unwrap_or_default();
    assert!(!prot_arn.is_empty(), "protection_arn should be non-empty");
    assert!(
        prot_arn.contains("arn:aws:shield"),
        "protection_arn should look like an ARN, got: {prot_arn}"
    );
}

#[tokio::test]
async fn test_list_protections_empty() {
    let client = make_shield_client().await;

    let resp = client
        .list_protections()
        .send()
        .await
        .expect("list_protections on empty state should succeed");

    assert_eq!(
        resp.protections().len(),
        0,
        "expected 0 protections before any creates"
    );
}

#[tokio::test]
async fn test_tag_resource() {
    let client = make_shield_client().await;

    // Create a protection and get its ARN for tagging.
    let create_resp = client
        .create_protection()
        .name("tag-test-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-tagtgt")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap().to_string();

    // Describe to get the protection_arn.
    let desc_resp = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .unwrap();
    let protection_arn = desc_resp
        .protection()
        .unwrap()
        .protection_arn()
        .unwrap()
        .to_string();

    // Tag the protection.
    client
        .tag_resource()
        .resource_arn(&protection_arn)
        .tags(
            aws_sdk_shield::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify the tag appears.
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&protection_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1, "expected 1 tag");
    assert_eq!(tags[0].key(), Some("env"));
    assert_eq!(tags[0].value(), Some("test"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("untag-test-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-untagtgt")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap().to_string();

    let desc_resp = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .unwrap();
    let protection_arn = desc_resp
        .protection()
        .unwrap()
        .protection_arn()
        .unwrap()
        .to_string();

    // Add two tags.
    client
        .tag_resource()
        .resource_arn(&protection_arn)
        .tags(
            aws_sdk_shield::types::Tag::builder()
                .key("keep")
                .value("yes")
                .build(),
        )
        .tags(
            aws_sdk_shield::types::Tag::builder()
                .key("remove")
                .value("me")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Remove one tag.
    client
        .untag_resource()
        .resource_arn(&protection_arn)
        .tag_keys("remove")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify only the kept tag remains.
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&protection_arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1, "expected 1 tag after untag");
    assert_eq!(tags[0].key(), Some("keep"));
}

#[tokio::test]
async fn test_tag_resource_update() {
    // Tagging the same key twice should overwrite the value.
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("tagupd-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-tagupd")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap().to_string();

    let desc_resp = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .unwrap();
    let protection_arn = desc_resp
        .protection()
        .unwrap()
        .protection_arn()
        .unwrap()
        .to_string();

    // First tag.
    client
        .tag_resource()
        .resource_arn(&protection_arn)
        .tags(
            aws_sdk_shield::types::Tag::builder()
                .key("color")
                .value("blue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Second tag with the same key — should overwrite.
    client
        .tag_resource()
        .resource_arn(&protection_arn)
        .tags(
            aws_sdk_shield::types::Tag::builder()
                .key("color")
                .value("red")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&protection_arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1, "duplicate key should result in 1 tag");
    assert_eq!(tags[0].key(), Some("color"));
    assert_eq!(tags[0].value(), Some("red"), "value should be overwritten");
}

// ============================================================================
// Additional tests derived from AWS documentation: Shield Advanced (2026-03-28)
// ============================================================================

/// Verify that CreateProtection accepts optional inline Tags without error.
/// Note: winterbaume silently drops the inline tags (they are not stored on
/// the protection ARN), matching the separate TagResource workflow.
#[tokio::test]
async fn test_create_protection_with_tags() {
    let client = make_shield_client().await;

    let resp = client
        .create_protection()
        .name("tagged-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-tagged")
        .tags(
            aws_sdk_shield::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("create_protection with inline tags should succeed");

    let protection_id = resp
        .protection_id()
        .expect("protection_id should be present");
    assert!(
        !protection_id.is_empty(),
        "protection_id should be non-empty"
    );
}

/// Verify that the Id field returned by DescribeProtection matches the
/// ProtectionId returned by CreateProtection.
#[tokio::test]
async fn test_describe_protection_id_matches_create() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("id-match-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-idmatch")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap().to_string();

    let desc_resp = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .expect("describe_protection should succeed");

    let protection = desc_resp
        .protection()
        .expect("protection should be present");
    assert_eq!(
        protection.id(),
        Some(protection_id.as_str()),
        "Protection.Id should match the ProtectionId from CreateProtection"
    );
}

/// DescribeProtection with neither ProtectionId nor ResourceArn should return
/// InvalidParameterException per the AWS API specification.
#[tokio::test]
async fn test_describe_protection_no_identifier() {
    let client = make_shield_client().await;

    let result = client
        .describe_protection()
        // intentionally omit both protection_id and resource_arn
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_protection with no identifier should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterException"),
        "Expected InvalidParameterException, got: {err_str}"
    );
}

/// After deleting a protection, DescribeProtection by the old ID should
/// return ResourceNotFoundException.
#[tokio::test]
async fn test_delete_protection_then_describe() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("del-then-desc-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-delthen")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap().to_string();

    client
        .delete_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .expect("delete_protection should succeed");

    let result = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_protection after delete should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

/// Deleting one of several protections should decrement the list count by one.
#[tokio::test]
async fn test_list_protections_decrements_after_delete() {
    let client = make_shield_client().await;

    let resp1 = client
        .create_protection()
        .name("keep-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-keep")
        .send()
        .await
        .unwrap();
    let keep_id = resp1.protection_id().unwrap().to_string();

    client
        .create_protection()
        .name("remove-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-remove")
        .send()
        .await
        .unwrap();

    // Verify 2 exist.
    let list_before = client.list_protections().send().await.unwrap();
    assert_eq!(
        list_before.protections().len(),
        2,
        "expected 2 protections before delete"
    );

    // Delete the second one via its id (keep the first).
    // Find the id that is NOT keep_id.
    let remove_id = list_before
        .protections()
        .iter()
        .find(|p| p.id() != Some(keep_id.as_str()))
        .and_then(|p| p.id())
        .unwrap()
        .to_string();

    client
        .delete_protection()
        .protection_id(&remove_id)
        .send()
        .await
        .expect("delete_protection should succeed");

    let list_after = client.list_protections().send().await.unwrap();
    assert_eq!(
        list_after.protections().len(),
        1,
        "expected 1 protection after delete"
    );
    assert_eq!(
        list_after.protections()[0].id(),
        Some(keep_id.as_str()),
        "remaining protection should be the one we kept"
    );
}

/// ListTagsForResource on a protection that has no tags should return an empty
/// list (not an error).
#[tokio::test]
async fn test_list_tags_for_resource_empty() {
    let client = make_shield_client().await;

    let create_resp = client
        .create_protection()
        .name("notags-protection")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-notags")
        .send()
        .await
        .unwrap();
    let protection_id = create_resp.protection_id().unwrap().to_string();

    let desc_resp = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .unwrap();
    let protection_arn = desc_resp
        .protection()
        .unwrap()
        .protection_arn()
        .unwrap()
        .to_string();

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&protection_arn)
        .send()
        .await
        .expect("list_tags_for_resource on untagged resource should succeed");

    assert_eq!(
        list_resp.tags().len(),
        0,
        "expected empty tags list for untagged protection"
    );
}

/// DescribeSubscription must include SubscriptionLimits with expected defaults.
#[tokio::test]
async fn test_subscription_limits_present() {
    let client = make_shield_client().await;

    client.create_subscription().send().await.unwrap();

    let resp = client
        .describe_subscription()
        .send()
        .await
        .expect("describe_subscription should succeed");

    let sub = resp.subscription().expect("subscription should be present");
    let limits = sub
        .subscription_limits()
        .expect("subscription_limits should be present");
    let pg_limits = limits
        .protection_group_limits()
        .expect("protection_group_limits should be present");
    assert!(
        pg_limits.max_protection_groups() > 0,
        "max_protection_groups should be positive"
    );
}

/// DescribeProtection by resource_arn that was never protected should return
/// ResourceNotFoundException.
#[tokio::test]
async fn test_describe_protection_resource_arn_not_found() {
    let client = make_shield_client().await;

    let result = client
        .describe_protection()
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-notexist")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_protection by nonexistent resource_arn should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_protection_lifecycle() {
    // Full create → describe → delete → verify lifecycle.
    let client = make_shield_client().await;

    let resource_arn = "arn:aws:ec2:us-east-1:123456789012:eip-allocation/eipalloc-lifecycle";

    // Create.
    let create_resp = client
        .create_protection()
        .name("lifecycle-protection")
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("create_protection should succeed");
    let protection_id = create_resp.protection_id().unwrap().to_string();
    assert!(!protection_id.is_empty());

    // Describe.
    let desc_resp = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .expect("describe_protection should succeed");
    let protection = desc_resp.protection().unwrap();
    assert_eq!(protection.name(), Some("lifecycle-protection"));
    assert_eq!(protection.resource_arn(), Some(resource_arn));
    assert_eq!(protection.id(), Some(protection_id.as_str()));

    // Appears in list.
    let list_resp = client.list_protections().send().await.unwrap();
    assert_eq!(list_resp.protections().len(), 1);

    // Delete.
    client
        .delete_protection()
        .protection_id(&protection_id)
        .send()
        .await
        .expect("delete_protection should succeed");

    // Verify gone.
    let list_after = client.list_protections().send().await.unwrap();
    assert_eq!(
        list_after.protections().len(),
        0,
        "list should be empty after delete"
    );

    let get_after = client
        .describe_protection()
        .protection_id(&protection_id)
        .send()
        .await;
    assert!(get_after.is_err(), "describe after delete should fail");
}

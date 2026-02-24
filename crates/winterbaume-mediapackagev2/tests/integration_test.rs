use aws_sdk_mediapackagev2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediapackagev2::MediaPackageV2Service;

async fn make_client() -> aws_sdk_mediapackagev2::Client {
    let mock = MockAws::builder()
        .with_service(MediaPackageV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediapackagev2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_mediapackagev2::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_channel_group() {
    let client = make_client().await;

    let resp = client
        .create_channel_group()
        .channel_group_name("test-group")
        .description("A test channel group")
        .send()
        .await
        .expect("create_channel_group should succeed");

    assert_eq!(resp.channel_group_name(), "test-group");
    assert!(resp.arn().contains("channelGroup/test-group"));
    assert!(resp.egress_domain().contains("mediapackagev2"));
    assert_eq!(
        resp.description().unwrap_or_default(),
        "A test channel group"
    );

    let get_resp = client
        .get_channel_group()
        .channel_group_name("test-group")
        .send()
        .await
        .expect("get_channel_group should succeed");

    assert_eq!(get_resp.channel_group_name(), "test-group");
    assert_eq!(
        get_resp.description().unwrap_or_default(),
        "A test channel group"
    );
    assert!(get_resp.egress_domain().contains("mediapackagev2"));
}

#[tokio::test]
async fn test_create_channel_group_conflict() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("dup-group")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_channel_group()
        .channel_group_name("dup-group")
        .send()
        .await;

    assert!(
        result.is_err(),
        "duplicate create should fail with ConflictException"
    );
}

#[tokio::test]
async fn test_delete_channel_group() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("delete-me")
        .send()
        .await
        .unwrap();

    client
        .delete_channel_group()
        .channel_group_name("delete-me")
        .send()
        .await
        .expect("delete_channel_group should succeed");

    let result = client
        .get_channel_group()
        .channel_group_name("delete-me")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_channel_groups() {
    let client = make_client().await;

    for i in 0..3 {
        client
            .create_channel_group()
            .channel_group_name(format!("group-{}", i))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_channel_groups()
        .send()
        .await
        .expect("list_channel_groups should succeed");

    assert_eq!(resp.items().len(), 3);
}

#[tokio::test]
async fn test_get_nonexistent_channel_group() {
    let client = make_client().await;

    let result = client
        .get_channel_group()
        .channel_group_name("nonexistent-group")
        .send()
        .await;
    assert!(
        result.is_err(),
        "get nonexistent should fail with ResourceNotFoundException"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_channel_group() {
    let client = make_client().await;

    let result = client
        .delete_channel_group()
        .channel_group_name("nonexistent-group")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent should fail");
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("ephemeral")
        .send()
        .await
        .unwrap();

    let list = client.list_channel_groups().send().await.unwrap();
    assert_eq!(list.items().len(), 1);

    client
        .delete_channel_group()
        .channel_group_name("ephemeral")
        .send()
        .await
        .unwrap();

    let list = client.list_channel_groups().send().await.unwrap();
    assert_eq!(list.items().len(), 0);
}

#[tokio::test]
async fn test_create_and_get_channel() {
    let client = make_client().await;

    // First create a channel group
    client
        .create_channel_group()
        .channel_group_name("test-group")
        .send()
        .await
        .expect("create_channel_group should succeed");

    // Create a channel within the group
    let resp = client
        .create_channel()
        .channel_group_name("test-group")
        .channel_name("test-channel")
        .description("A test channel")
        .send()
        .await
        .expect("create_channel should succeed");

    assert_eq!(resp.channel_name(), "test-channel");
    assert_eq!(resp.channel_group_name(), "test-group");
    assert!(resp.arn().contains("channel/test-channel"));

    // Get the channel
    let get_resp = client
        .get_channel()
        .channel_group_name("test-group")
        .channel_name("test-channel")
        .send()
        .await
        .expect("get_channel should succeed");

    assert_eq!(get_resp.channel_name(), "test-channel");
    assert_eq!(get_resp.description().unwrap_or_default(), "A test channel");
}

#[tokio::test]
async fn test_delete_channel() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("del-group")
        .send()
        .await
        .unwrap();

    client
        .create_channel()
        .channel_group_name("del-group")
        .channel_name("del-channel")
        .send()
        .await
        .unwrap();

    // Delete the channel
    client
        .delete_channel()
        .channel_group_name("del-group")
        .channel_name("del-channel")
        .send()
        .await
        .expect("delete_channel should succeed");

    // Verify it's gone
    let result = client
        .get_channel()
        .channel_group_name("del-group")
        .channel_name("del-channel")
        .send()
        .await;

    assert!(result.is_err(), "get_channel should fail after deletion");
}

#[tokio::test]
async fn test_get_nonexistent_channel_fails() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("empty-group")
        .send()
        .await
        .unwrap();

    let result = client
        .get_channel()
        .channel_group_name("empty-group")
        .channel_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "get_channel should fail for nonexistent");
}

#[tokio::test]
async fn test_create_duplicate_channel_fails() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("dup-group")
        .send()
        .await
        .unwrap();

    client
        .create_channel()
        .channel_group_name("dup-group")
        .channel_name("dup-channel")
        .send()
        .await
        .unwrap();

    let result = client
        .create_channel()
        .channel_group_name("dup-group")
        .channel_name("dup-channel")
        .send()
        .await;

    assert!(result.is_err(), "creating duplicate channel should fail");
}

// ============================================================================
// Tests derived from AWS documentation: MediaPackage v2
// ============================================================================

#[tokio::test]
async fn test_channel_group_arn_and_timestamps() {
    let client = make_client().await;

    let resp = client
        .create_channel_group()
        .channel_group_name("arn-ts-group")
        .send()
        .await
        .expect("create_channel_group should succeed");

    // ARN must contain the resource type and name
    let arn = resp.arn();
    assert!(
        arn.contains("channelGroup/arn-ts-group"),
        "ARN should contain channelGroup/arn-ts-group, got: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:mediapackagev2:"),
        "ARN should start with arn:aws:mediapackagev2:, got: {arn}"
    );

    // Timestamps must be present (created_at returns &DateTime, not Option)
    let _ = resp.created_at();
    let _ = resp.modified_at();
}

#[tokio::test]
async fn test_channel_group_description_optional() {
    let client = make_client().await;

    // Create without a description
    client
        .create_channel_group()
        .channel_group_name("no-desc-group")
        .send()
        .await
        .expect("create_channel_group without description should succeed");

    let get_resp = client
        .get_channel_group()
        .channel_group_name("no-desc-group")
        .send()
        .await
        .expect("get_channel_group should succeed");

    // Description should be absent or empty when not provided
    let desc = get_resp.description().unwrap_or_default();
    assert!(
        desc.is_empty(),
        "description should be empty when not set, got: {desc}"
    );
}

#[tokio::test]
async fn test_channel_group_etag_present() {
    let client = make_client().await;

    let resp = client
        .create_channel_group()
        .channel_group_name("etag-group")
        .send()
        .await
        .expect("create_channel_group should succeed");

    let etag = resp.e_tag().unwrap_or_default();
    assert!(
        !etag.is_empty(),
        "e_tag should be non-empty after create, got: {etag:?}"
    );
}

#[tokio::test]
async fn test_list_channel_groups_item_fields() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("list-fields-alpha")
        .send()
        .await
        .unwrap();

    client
        .create_channel_group()
        .channel_group_name("list-fields-beta")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_channel_groups()
        .send()
        .await
        .expect("list_channel_groups should succeed");

    let items = resp.items();
    assert_eq!(items.len(), 2, "expected 2 channel groups in list");

    let names: Vec<&str> = items.iter().map(|i| i.channel_group_name()).collect();
    assert!(
        names.contains(&"list-fields-alpha"),
        "list-fields-alpha should appear in list, got: {names:?}"
    );
    assert!(
        names.contains(&"list-fields-beta"),
        "list-fields-beta should appear in list, got: {names:?}"
    );

    for item in items {
        let item_arn = item.arn();
        assert!(
            !item_arn.is_empty(),
            "each list item should have a non-empty ARN"
        );
        assert!(
            item_arn.contains("channelGroup/"),
            "list item ARN should contain channelGroup/, got: {item_arn}"
        );
    }
}

#[tokio::test]
async fn test_delete_channel_nonexistent_fails() {
    let client = make_client().await;

    // Create the channel group but no channel inside it
    client
        .create_channel_group()
        .channel_group_name("present-group")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_channel()
        .channel_group_name("present-group")
        .channel_name("ghost-channel")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_channel should fail for nonexistent channel"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_channel_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .create_channel()
        .channel_group_name("no-such-group")
        .channel_name("any-channel")
        .send()
        .await;

    assert!(
        result.is_err(),
        "create_channel should fail when channel group does not exist"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_channel_lifecycle() {
    let client = make_client().await;

    // Setup: create channel group
    client
        .create_channel_group()
        .channel_group_name("lifecycle-group")
        .send()
        .await
        .expect("create_channel_group should succeed");

    // Create channel
    let create_resp = client
        .create_channel()
        .channel_group_name("lifecycle-group")
        .channel_name("lifecycle-channel")
        .description("lifecycle test channel")
        .send()
        .await
        .expect("create_channel should succeed");

    assert_eq!(create_resp.channel_name(), "lifecycle-channel");
    assert_eq!(create_resp.channel_group_name(), "lifecycle-group");

    // Get channel — should succeed
    let get_resp = client
        .get_channel()
        .channel_group_name("lifecycle-group")
        .channel_name("lifecycle-channel")
        .send()
        .await
        .expect("get_channel should succeed after create");

    assert_eq!(get_resp.channel_name(), "lifecycle-channel");
    assert_eq!(
        get_resp.description().unwrap_or_default(),
        "lifecycle test channel"
    );

    // Delete channel
    client
        .delete_channel()
        .channel_group_name("lifecycle-group")
        .channel_name("lifecycle-channel")
        .send()
        .await
        .expect("delete_channel should succeed");

    // Get channel — should now fail
    let result = client
        .get_channel()
        .channel_group_name("lifecycle-group")
        .channel_name("lifecycle-channel")
        .send()
        .await;

    assert!(result.is_err(), "get_channel should fail after deletion");
}

#[tokio::test]
async fn test_channel_arn_format() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("arn-cg")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_channel()
        .channel_group_name("arn-cg")
        .channel_name("arn-ch")
        .send()
        .await
        .expect("create_channel should succeed");

    let arn = resp.arn();
    assert!(
        arn.contains("channelGroup/arn-cg"),
        "channel ARN should contain channelGroup/arn-cg, got: {arn}"
    );
    assert!(
        arn.contains("channel/arn-ch"),
        "channel ARN should contain channel/arn-ch, got: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:mediapackagev2:"),
        "channel ARN should start with arn:aws:mediapackagev2:, got: {arn}"
    );
}

#[tokio::test]
async fn test_channel_timestamps() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("ts-cg")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_channel()
        .channel_group_name("ts-cg")
        .channel_name("ts-ch")
        .send()
        .await
        .expect("create_channel should succeed");

    // created_at and modified_at return &DateTime directly (not Option)
    let created_secs = resp.created_at().secs();
    let modified_secs = resp.modified_at().secs();
    assert!(
        created_secs > 0,
        "channel created_at should be a positive Unix timestamp, got: {created_secs}"
    );
    assert!(
        modified_secs > 0,
        "channel modified_at should be a positive Unix timestamp, got: {modified_secs}"
    );
}

// ============================================================================
// Tests derived from AWS documentation: MediaPackage v2 (additional coverage)
// ============================================================================

#[tokio::test]
async fn test_create_channel_group_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_channel_group()
        .channel_group_name("tagged-group")
        .tags("env", "test")
        .tags("project", "winterbaume")
        .send()
        .await
        .expect("create_channel_group with tags should succeed");

    let tags = resp.tags();
    assert_eq!(
        tags.and_then(|t| t.get("env")).map(|s| s.as_str()),
        Some("test"),
        "tag 'env' should be 'test', got: {tags:?}"
    );
    assert_eq!(
        tags.and_then(|t| t.get("project")).map(|s| s.as_str()),
        Some("winterbaume"),
        "tag 'project' should be 'winterbaume', got: {tags:?}"
    );
}

#[tokio::test]
async fn test_create_channel_with_tags() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("tag-cg")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_channel()
        .channel_group_name("tag-cg")
        .channel_name("tag-ch")
        .tags("team", "media")
        .send()
        .await
        .expect("create_channel with tags should succeed");

    let tags = resp.tags();
    assert_eq!(
        tags.and_then(|t| t.get("team")).map(|s| s.as_str()),
        Some("media"),
        "tag 'team' should be 'media', got: {tags:?}"
    );
}

#[tokio::test]
async fn test_channel_etag_present() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("etag-cg")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_channel()
        .channel_group_name("etag-cg")
        .channel_name("etag-ch")
        .send()
        .await
        .expect("create_channel should succeed");

    let etag = resp.e_tag().unwrap_or_default();
    assert!(
        !etag.is_empty(),
        "channel e_tag should be non-empty after create, got: {etag:?}"
    );
}

#[tokio::test]
async fn test_list_channel_groups_empty() {
    let client = make_client().await;

    // Do not create any groups — should return empty Items list
    let resp = client
        .list_channel_groups()
        .send()
        .await
        .expect("list_channel_groups on empty state should succeed");

    assert_eq!(
        resp.items().len(),
        0,
        "expected 0 channel groups in empty list, got: {}",
        resp.items().len()
    );
}

#[tokio::test]
async fn test_get_channel_in_nonexistent_group() {
    let client = make_client().await;

    // Do not create any channel group
    let result = client
        .get_channel()
        .channel_group_name("no-such-group")
        .channel_name("any-channel")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_channel should fail when channel group doesn't exist"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_channel_in_nonexistent_group() {
    let client = make_client().await;

    // Do not create any channel group
    let result = client
        .delete_channel()
        .channel_group_name("no-such-group")
        .channel_name("any-channel")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_channel should fail when channel group doesn't exist"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_channel_group_conflict_error_type() {
    let client = make_client().await;

    client
        .create_channel_group()
        .channel_group_name("conflict-group")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_channel_group()
        .channel_group_name("conflict-group")
        .send()
        .await;

    assert!(result.is_err(), "second create should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ConflictException") || err_str.contains("Conflict"),
        "expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_channel_group_not_found_error_type() {
    let client = make_client().await;

    let result = client
        .get_channel_group()
        .channel_group_name("ghost-group")
        .send()
        .await;

    assert!(result.is_err(), "get on nonexistent group should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

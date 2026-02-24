use aws_sdk_pipes::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pipes::PipesService;

async fn make_client() -> aws_sdk_pipes::Client {
    let mock = MockAws::builder().with_service(PipesService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pipes::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_pipes::Client::new(&config)
}

async fn create_test_pipe(client: &aws_sdk_pipes::Client, name: &str) -> String {
    let resp = client
        .create_pipe()
        .name(name)
        .source("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .target("arn:aws:lambda:us-east-1:123456789012:function:my-fn")
        .send()
        .await
        .expect("create_pipe should succeed");
    resp.arn().unwrap_or_default().to_string()
}

#[tokio::test]
async fn test_create_and_describe_pipe() {
    let client = make_client().await;

    let resp = client
        .create_pipe()
        .name("my-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .target("arn:aws:lambda:us-east-1:123456789012:function:my-fn")
        .send()
        .await
        .expect("create_pipe should succeed");

    assert_eq!(resp.name(), Some("my-pipe"));
    let arn = resp.arn().expect("should have arn");
    assert!(arn.contains("pipe/my-pipe"));

    let desc = client
        .describe_pipe()
        .name("my-pipe")
        .send()
        .await
        .expect("describe_pipe should succeed");

    assert_eq!(desc.name(), Some("my-pipe"));
    assert_eq!(
        desc.source(),
        Some("arn:aws:sqs:us-east-1:123456789012:my-queue")
    );
    assert_eq!(
        desc.target(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:my-fn")
    );
}

#[tokio::test]
async fn test_list_pipes() {
    let client = make_client().await;

    for name in ["pipe-a", "pipe-b"] {
        client
            .create_pipe()
            .name(name)
            .source("arn:aws:sqs:us-east-1:123456789012:q")
            .target("arn:aws:lambda:us-east-1:123456789012:function:f")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed");

    assert_eq!(resp.pipes().len(), 2);
}

#[tokio::test]
async fn test_delete_pipe() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("delete-me")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    client
        .delete_pipe()
        .name("delete-me")
        .send()
        .await
        .expect("delete_pipe should succeed");

    let result = client.describe_pipe().name("delete-me").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_pipe() {
    let client = make_client().await;

    let result = client.describe_pipe().name("nonexistent-pipe").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_pipes_empty() {
    let client = make_client().await;

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed on empty state");

    assert_eq!(resp.pipes().len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: Amazon EventBridge Pipes
// ============================================================================

#[tokio::test]
async fn test_create_pipe_response_fields() {
    let client = make_client().await;

    let resp = client
        .create_pipe()
        .name("resp-fields-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .expect("create_pipe should succeed");

    // Per docs: new pipes default to RUNNING desired state
    assert_eq!(resp.desired_state().map(|s| s.as_str()), Some("RUNNING"));
    assert_eq!(resp.current_state().map(|s| s.as_str()), Some("RUNNING"));
    assert!(
        resp.creation_time().is_some(),
        "creation_time should be present"
    );
}

#[tokio::test]
async fn test_create_pipe_duplicate() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("dup-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    let err = client
        .create_pipe()
        .name("dup-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q2")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f2")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_pipe_not_found() {
    let client = make_client().await;

    let err = client
        .describe_pipe()
        .name("no-such-pipe")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_pipe_not_found() {
    let client = make_client().await;

    let err = client
        .delete_pipe()
        .name("no-such-pipe")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_pipe_response() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("delete-resp-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_pipe()
        .name("delete-resp-pipe")
        .send()
        .await
        .expect("delete_pipe should succeed");

    assert_eq!(resp.name(), Some("delete-resp-pipe"));
    assert!(resp.arn().is_some(), "delete response should include ARN");
    // Per AWS docs: delete response sets CurrentState to DELETING
    assert_eq!(
        resp.current_state().map(|s| s.as_str()),
        Some("DELETING"),
        "CurrentState should be DELETING after delete"
    );
}

#[tokio::test]
async fn test_pipe_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_pipe()
        .name("arn-format-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    let arn = resp.arn().expect("should have ARN");
    // Expected: arn:aws:pipes:us-east-1:{account}:pipe/{name}
    assert!(
        arn.starts_with("arn:aws:pipes:us-east-1:"),
        "ARN should start with arn:aws:pipes:us-east-1:, got: {arn}"
    );
    assert!(
        arn.ends_with(":pipe/arn-format-pipe"),
        "ARN should end with :pipe/arn-format-pipe, got: {arn}"
    );
}

#[tokio::test]
async fn test_list_pipes_contains_fields() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("list-fields-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:source-queue")
        .target("arn:aws:lambda:us-east-1:123456789012:function:target-fn")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed");

    assert_eq!(resp.pipes().len(), 1);

    let pipe = &resp.pipes()[0];
    assert_eq!(pipe.name(), Some("list-fields-pipe"));
    assert!(
        pipe.arn()
            .unwrap_or_default()
            .contains("pipe/list-fields-pipe")
    );
    assert_eq!(
        pipe.source(),
        Some("arn:aws:sqs:us-east-1:123456789012:source-queue")
    );
    assert_eq!(
        pipe.target(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:target-fn")
    );
    assert_eq!(
        pipe.desired_state().map(|s| s.as_str()).unwrap_or_default(),
        "RUNNING"
    );
    assert_eq!(
        pipe.current_state().map(|s| s.as_str()).unwrap_or_default(),
        "RUNNING"
    );
}

#[tokio::test]
async fn test_pipe_creation_time() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("ts-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_pipe()
        .name("ts-pipe")
        .send()
        .await
        .expect("describe_pipe should succeed");

    assert!(
        desc.creation_time().is_some(),
        "CreationTime should be present in describe response"
    );
    // Ensure it's a non-zero timestamp (seconds since epoch > 0)
    let secs = desc.creation_time().unwrap().secs();
    assert!(secs > 0, "CreationTime should be a positive timestamp");
}

#[tokio::test]
async fn test_pipe_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_pipe()
        .name("lifecycle-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.arn().unwrap().to_string();
    assert!(!arn.is_empty());

    // Describe
    let desc = client
        .describe_pipe()
        .name("lifecycle-pipe")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.name(), Some("lifecycle-pipe"));
    assert_eq!(desc.arn(), Some(arn.as_str()));

    // Delete
    client
        .delete_pipe()
        .name("lifecycle-pipe")
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let err = client
        .describe_pipe()
        .name("lifecycle-pipe")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_tag_resource() {
    let client = make_client().await;

    let arn = create_test_pipe(&client, "tag-pipe").await;
    assert!(!arn.is_empty());

    // TagResource should succeed
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let arn = create_test_pipe(&client, "untag-pipe").await;

    // Tag first
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // Untag one key
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");
}

#[tokio::test]
async fn test_tag_resource_not_found() {
    let client = make_client().await;

    let err = client
        .tag_resource()
        .resource_arn("arn:aws:pipes:us-east-1:123456789012:pipe/nonexistent")
        .tags("env", "test")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException for tag on nonexistent pipe, got: {err_str}"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon EventBridge Pipes (extended)
// Source: https://docs.aws.amazon.com/eventbridge/latest/pipes-reference/API_Operations.html
// ============================================================================

#[tokio::test]
async fn test_create_pipe_missing_source() {
    let client = make_client().await;

    // Source is required per AWS docs; omitting it should produce a ValidationException
    let err = client
        .create_pipe()
        .name("no-source-pipe")
        // .source(...) intentionally omitted
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException for missing Source, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_pipe_missing_target() {
    let client = make_client().await;

    // Target is required per AWS docs; omitting it should produce a ValidationException
    let err = client
        .create_pipe()
        .name("no-target-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        // .target(...) intentionally omitted
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException for missing Target, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_pipe_fields_complete() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("fields-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:source-q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:target-fn")
        .send()
        .await
        .expect("create_pipe should succeed");

    let desc = client
        .describe_pipe()
        .name("fields-pipe")
        .send()
        .await
        .expect("describe_pipe should succeed");

    // Verify all required fields are populated
    assert_eq!(desc.name(), Some("fields-pipe"));
    assert!(desc.arn().is_some(), "Arn should be present");
    assert!(
        desc.arn().unwrap_or_default().contains("pipe/fields-pipe"),
        "Arn should contain pipe name"
    );
    assert_eq!(
        desc.source(),
        Some("arn:aws:sqs:us-east-1:123456789012:source-q")
    );
    assert_eq!(
        desc.target(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:target-fn")
    );
    assert!(
        desc.desired_state().is_some(),
        "DesiredState should be present"
    );
    assert!(
        desc.current_state().is_some(),
        "CurrentState should be present"
    );
    assert!(
        desc.creation_time().is_some(),
        "CreationTime should be present"
    );
}

#[tokio::test]
async fn test_list_pipes_multiple() {
    let client = make_client().await;

    for name in ["multi-pipe-a", "multi-pipe-b", "multi-pipe-c"] {
        client
            .create_pipe()
            .name(name)
            .source("arn:aws:sqs:us-east-1:123456789012:q")
            .target("arn:aws:lambda:us-east-1:123456789012:function:f")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed");

    assert_eq!(resp.pipes().len(), 3);
}

#[tokio::test]
async fn test_untag_resource_not_found() {
    let client = make_client().await;

    let err = client
        .untag_resource()
        .resource_arn("arn:aws:pipes:us-east-1:123456789012:pipe/nonexistent")
        .tag_keys("env")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException for untag on nonexistent pipe, got: {err_str}"
    );
}

#[tokio::test]
async fn test_tag_then_untag_removes_key() {
    let client = make_client().await;

    let arn = create_test_pipe(&client, "untag-keys-pipe").await;

    // Tag the pipe with two keys
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // Untag only the 'env' key
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify via list_tags_for_resource that 'env' is gone and 'team' remains
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let empty = std::collections::HashMap::new();
    let tags = list_resp.tags().unwrap_or(&empty);
    assert!(
        !tags.contains_key("env"),
        "Tag 'env' should have been removed, but it's still present"
    );
    assert_eq!(
        tags.get("team").map(|s| s.as_str()),
        Some("platform"),
        "Tag 'team' should still be present"
    );
}

#[tokio::test]
async fn test_list_pipes_after_delete() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("gone-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    client
        .delete_pipe()
        .name("gone-pipe")
        .send()
        .await
        .expect("delete_pipe should succeed");

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed");

    assert_eq!(
        resp.pipes().len(),
        0,
        "Deleted pipe should not appear in list"
    );
}

#[tokio::test]
async fn test_delete_pipe_twice() {
    let client = make_client().await;

    client
        .create_pipe()
        .name("double-delete-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .unwrap();

    // First delete should succeed
    client
        .delete_pipe()
        .name("double-delete-pipe")
        .send()
        .await
        .expect("first delete should succeed");

    // Second delete should return NotFoundException
    let err = client
        .delete_pipe()
        .name("double-delete-pipe")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException on second delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_pipe_arn_contains_account_and_region() {
    let client = make_client().await;

    let resp = client
        .create_pipe()
        .name("arn-check-pipe")
        .source("arn:aws:sqs:us-east-1:123456789012:q")
        .target("arn:aws:lambda:us-east-1:123456789012:function:f")
        .send()
        .await
        .expect("create_pipe should succeed");

    let arn = resp.arn().expect("ARN should be present");
    // Expected format: arn:aws:pipes:{region}:{account}:pipe/{name}
    assert!(
        arn.starts_with("arn:aws:pipes:"),
        "ARN should start with arn:aws:pipes:, got: {arn}"
    );
    assert!(
        arn.contains("us-east-1"),
        "ARN should contain region, got: {arn}"
    );
    assert!(
        arn.contains("pipe/arn-check-pipe"),
        "ARN should contain pipe name, got: {arn}"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource_basic() {
    let client = make_client().await;

    let arn = create_test_pipe(&client, "list-tags-pipe").await;

    // Tag the pipe
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("project", "winterbaume")
        .tags("stage", "dev")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let empty = std::collections::HashMap::new();
    let tags = resp.tags().unwrap_or(&empty);
    assert_eq!(
        tags.get("project").map(|s| s.as_str()),
        Some("winterbaume"),
        "Tag 'project' should be 'winterbaume'"
    );
    assert_eq!(
        tags.get("stage").map(|s| s.as_str()),
        Some("dev"),
        "Tag 'stage' should be 'dev'"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource_empty() {
    let client = make_client().await;

    let arn = create_test_pipe(&client, "no-tags-pipe").await;

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed for pipe with no tags");

    // Tags map should be empty (or absent) for an untagged pipe
    let tag_count = resp.tags().map(|t| t.len()).unwrap_or(0);
    assert_eq!(tag_count, 0, "Newly created pipe should have no tags");
}

#[tokio::test]
async fn test_list_tags_for_resource_not_found() {
    let client = make_client().await;

    let err = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:pipes:us-east-1:123456789012:pipe/nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException for list_tags on nonexistent pipe, got: {err_str}"
    );
}

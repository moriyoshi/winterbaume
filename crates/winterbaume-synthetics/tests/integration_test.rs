use aws_sdk_synthetics::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_synthetics::SyntheticsService;

async fn make_client() -> aws_sdk_synthetics::Client {
    let mock = MockAws::builder()
        .with_service(SyntheticsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_synthetics::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_synthetics::Client::new(&config)
}

fn make_code(handler: &str) -> aws_sdk_synthetics::types::CanaryCodeInput {
    aws_sdk_synthetics::types::CanaryCodeInput::builder()
        .handler(handler)
        .s3_bucket("my-bucket")
        .s3_key("my-canary-code.zip")
        .build()
}

fn make_schedule(expression: &str) -> aws_sdk_synthetics::types::CanaryScheduleInput {
    aws_sdk_synthetics::types::CanaryScheduleInput::builder()
        .expression(expression)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_and_get_canary() {
    let client = make_client().await;

    let resp = client
        .create_canary()
        .name("test-canary")
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .expect("create_canary should succeed");

    let canary = resp.canary().expect("should have canary in response");
    assert_eq!(canary.name(), Some("test-canary"));
    assert!(canary.id().is_some());

    // GetCanary
    let get_resp = client
        .get_canary()
        .name("test-canary")
        .send()
        .await
        .expect("get_canary should succeed");

    let canary = get_resp.canary().expect("should have canary");
    assert_eq!(canary.name(), Some("test-canary"));
    assert_eq!(canary.runtime_version(), Some("syn-nodejs-puppeteer-6.2"));
}

#[tokio::test]
async fn test_delete_canary() {
    let client = make_client().await;

    client
        .create_canary()
        .name("del-canary")
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .unwrap();

    client
        .delete_canary()
        .name("del-canary")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_canary().name("del-canary").send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_describe_canaries() {
    let client = make_client().await;

    client
        .create_canary()
        .name("canary-1")
        .code(make_code("handler1.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .unwrap();

    client
        .create_canary()
        .name("canary-2")
        .code(make_code("handler2.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(10 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_canaries()
        .send()
        .await
        .expect("describe_canaries should succeed");

    assert_eq!(resp.canaries().len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_canary_fails() {
    let client = make_client().await;

    client
        .create_canary()
        .name("dup-canary")
        .code(make_code("handler.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .unwrap();

    let result = client
        .create_canary()
        .name("dup-canary")
        .code(make_code("handler.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await;
    assert!(result.is_err(), "duplicate canary should fail");
}

#[tokio::test]
async fn test_get_nonexistent_canary_fails() {
    let client = make_client().await;

    let result = client.get_canary().name("nonexistent").send().await;
    assert!(result.is_err(), "get nonexistent canary should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_canary_fails() {
    let client = make_client().await;

    let result = client.delete_canary().name("nonexistent").send().await;
    assert!(result.is_err(), "delete nonexistent canary should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Tests derived from AWS documentation: CloudWatch Synthetics
// ============================================================================

/// Helper to create a canary and return the response canary object.
async fn create_test_canary(
    client: &aws_sdk_synthetics::Client,
    name: &str,
) -> aws_sdk_synthetics::types::Canary {
    client
        .create_canary()
        .name(name)
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .expect("create_canary should succeed")
        .canary()
        .expect("response should include canary")
        .clone()
}

/// Construct the expected ARN for a canary given its name.
/// Account ID is 123456789012 (DEFAULT_ACCOUNT_ID) and region is us-east-1.
fn canary_arn(name: &str) -> String {
    format!("arn:aws:synthetics:us-east-1:123456789012:canary:{name}")
}

#[tokio::test]
async fn test_canary_arn_format() {
    let client = make_client().await;
    create_test_canary(&client, "arn-check-canary").await;

    // Verify the ARN stored in state (accessible via list_tags_for_resource).
    // We construct the expected ARN and use list_tags_for_resource to confirm it works.
    let expected_arn = canary_arn("arn-check-canary");
    // ARN format: arn:aws:synthetics:{region}:{account-id}:canary:{name}
    assert!(
        expected_arn.starts_with("arn:aws:synthetics:"),
        "ARN should start with arn:aws:synthetics:"
    );
    assert!(
        expected_arn.contains(":canary:arn-check-canary"),
        "ARN should contain :canary:arn-check-canary"
    );

    // Confirm that list_tags_for_resource works with this ARN (validates ARN is correct)
    client
        .list_tags_for_resource()
        .resource_arn(&expected_arn)
        .send()
        .await
        .expect("list_tags_for_resource with computed ARN should succeed");
}

#[tokio::test]
async fn test_canary_initial_status() {
    let client = make_client().await;
    let canary = create_test_canary(&client, "status-check-canary").await;

    let status = canary.status().expect("canary should have status");
    assert_eq!(
        status.state().map(|s| s.as_str()),
        Some("CREATING"),
        "Initial canary state should be CREATING"
    );
}

#[tokio::test]
async fn test_canary_default_retention() {
    let client = make_client().await;

    // Create without specifying retention periods — should default to 31 days each
    let canary = create_test_canary(&client, "retention-default-canary").await;

    assert_eq!(
        canary.success_retention_period_in_days(),
        Some(31),
        "Default success retention should be 31 days"
    );
    assert_eq!(
        canary.failure_retention_period_in_days(),
        Some(31),
        "Default failure retention should be 31 days"
    );
}

#[tokio::test]
async fn test_canary_custom_retention() {
    let client = make_client().await;

    let resp = client
        .create_canary()
        .name("retention-custom-canary")
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .success_retention_period_in_days(14)
        .failure_retention_period_in_days(7)
        .send()
        .await
        .expect("create_canary should succeed");

    let canary = resp.canary().expect("should have canary");
    assert_eq!(
        canary.success_retention_period_in_days(),
        Some(14),
        "Custom success retention should be 14 days"
    );
    assert_eq!(
        canary.failure_retention_period_in_days(),
        Some(7),
        "Custom failure retention should be 7 days"
    );

    // Verify same values are returned via GetCanary
    let get_resp = client
        .get_canary()
        .name("retention-custom-canary")
        .send()
        .await
        .expect("get_canary should succeed");
    let canary = get_resp.canary().expect("should have canary");
    assert_eq!(canary.success_retention_period_in_days(), Some(14));
    assert_eq!(canary.failure_retention_period_in_days(), Some(7));
}

#[tokio::test]
async fn test_canary_schedule_fields() {
    let client = make_client().await;

    let resp = client
        .create_canary()
        .name("schedule-check-canary")
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(10 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .expect("create_canary should succeed");

    let canary = resp.canary().expect("should have canary");
    let schedule = canary.schedule().expect("should have schedule");
    assert_eq!(
        schedule.expression(),
        Some("rate(10 minutes)"),
        "Schedule expression should match input"
    );
}

#[tokio::test]
async fn test_canary_code_handler() {
    let client = make_client().await;

    let resp = client
        .create_canary()
        .name("handler-check-canary")
        .code(make_code("myScript.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .expect("create_canary should succeed");

    let canary = resp.canary().expect("should have canary");
    let code = canary.code().expect("should have code");
    assert_eq!(
        code.handler(),
        Some("myScript.handler"),
        "Code handler should match input"
    );
}

#[tokio::test]
async fn test_canary_execution_role_arn() {
    let client = make_client().await;
    let canary = create_test_canary(&client, "role-check-canary").await;

    assert_eq!(
        canary.execution_role_arn(),
        Some("arn:aws:iam::123456789012:role/canary-role"),
        "ExecutionRoleArn should match input"
    );
}

#[tokio::test]
async fn test_create_canary_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_canary()
        .name("tagged-canary")
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("create_canary with tags should succeed");

    let canary = resp.canary().expect("should have canary");
    let tags = canary.tags().expect("canary should have tags map");
    assert_eq!(
        tags.get("env").map(String::as_str),
        Some("test"),
        "Tag 'env' should be 'test'"
    );
    assert_eq!(
        tags.get("team").map(String::as_str),
        Some("platform"),
        "Tag 'team' should be 'platform'"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_client().await;

    client
        .create_canary()
        .name("list-tags-canary")
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .tags("project", "winterbaume")
        .send()
        .await
        .expect("create_canary should succeed");

    // Construct ARN from known components (account: 123456789012, region: us-east-1)
    let arn = canary_arn("list-tags-canary");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().expect("response should have tags map");
    assert_eq!(
        tags.get("project").map(String::as_str),
        Some("winterbaume"),
        "Tag 'project' should be 'winterbaume'"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource_no_tags() {
    let client = make_client().await;

    create_test_canary(&client, "no-tags-canary").await;
    let arn = canary_arn("no-tags-canary");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed for untagged canary");

    // When no tags, the response should return an empty (or absent) map (not an error)
    let is_empty = tags_resp.tags().map(|t| t.is_empty()).unwrap_or(true);
    assert!(is_empty, "Tags should be empty for an untagged canary");
}

#[tokio::test]
async fn test_describe_canaries_empty() {
    let client = make_client().await;

    let resp = client
        .describe_canaries()
        .send()
        .await
        .expect("describe_canaries should succeed on empty state");

    assert_eq!(
        resp.canaries().len(),
        0,
        "Should return empty list when no canaries exist"
    );
}

#[tokio::test]
async fn test_canary_id_is_present_and_nonempty() {
    let client = make_client().await;
    let canary = create_test_canary(&client, "id-check-canary").await;

    let id = canary.id().unwrap_or_default();
    assert!(!id.is_empty(), "Canary ID should be present and non-empty");
}

#[tokio::test]
async fn test_canary_id_is_stable() {
    // Verifies that GetCanary returns the same id on repeated calls
    let client = make_client().await;
    create_test_canary(&client, "stable-id-canary").await;

    let id1 = client
        .get_canary()
        .name("stable-id-canary")
        .send()
        .await
        .unwrap()
        .canary()
        .unwrap()
        .id()
        .unwrap_or_default()
        .to_string();

    let id2 = client
        .get_canary()
        .name("stable-id-canary")
        .send()
        .await
        .unwrap()
        .canary()
        .unwrap()
        .id()
        .unwrap_or_default()
        .to_string();

    assert_eq!(
        id1, id2,
        "Canary ID should be deterministic across GetCanary calls"
    );
}

#[tokio::test]
async fn test_full_canary_lifecycle() {
    let client = make_client().await;
    let name = "lifecycle-canary";

    // Create
    let create_resp = client
        .create_canary()
        .name(name)
        .code(make_code("pageLoadBlueprint.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .tags("lifecycle", "test")
        .send()
        .await
        .expect("create_canary should succeed");

    let canary = create_resp.canary().expect("should have canary");
    assert_eq!(canary.name(), Some(name));
    assert!(!canary.id().unwrap_or_default().is_empty());

    // Get
    let get_resp = client
        .get_canary()
        .name(name)
        .send()
        .await
        .expect("get_canary should succeed");
    let canary = get_resp.canary().expect("should have canary");
    assert_eq!(canary.name(), Some(name));
    assert_eq!(canary.runtime_version(), Some("syn-nodejs-puppeteer-6.2"));

    // Describe -- canary appears in list
    let desc_resp = client
        .describe_canaries()
        .send()
        .await
        .expect("describe_canaries should succeed");
    let names: Vec<Option<&str>> = desc_resp.canaries().iter().map(|c| c.name()).collect();
    assert!(
        names.contains(&Some(name)),
        "lifecycle-canary should appear in describe_canaries list"
    );

    // Delete
    client
        .delete_canary()
        .name(name)
        .send()
        .await
        .expect("delete_canary should succeed");

    // Verify gone
    let result = client.get_canary().name(name).send().await;
    assert!(
        result.is_err(),
        "get_canary after delete should return an error"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_canary_returns_resource_not_found() {
    let client = make_client().await;

    let err = client
        .delete_canary()
        .name("totally-missing-canary")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_duplicate_returns_conflict() {
    let client = make_client().await;

    client
        .create_canary()
        .name("conflict-canary")
        .code(make_code("handler.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .unwrap();

    let err = client
        .create_canary()
        .name("conflict-canary")
        .code(make_code("handler.handler"))
        .artifact_s3_location("s3://my-bucket/artifacts")
        .execution_role_arn("arn:aws:iam::123456789012:role/canary-role")
        .schedule(make_schedule("rate(5 minutes)"))
        .runtime_version("syn-nodejs-puppeteer-6.2")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException on duplicate, got: {err_str}"
    );
}

// ============================================================================
// Tests for new operations added in the second pass
// ============================================================================

#[tokio::test]
async fn test_start_and_stop_canary() {
    let client = make_client().await;
    create_test_canary(&client, "start-stop-canary").await;

    client
        .start_canary()
        .name("start-stop-canary")
        .send()
        .await
        .expect("start_canary should succeed");

    let get_resp = client
        .get_canary()
        .name("start-stop-canary")
        .send()
        .await
        .unwrap();
    let canary = get_resp.canary().unwrap();
    assert_eq!(
        canary.status().unwrap().state().map(|s| s.as_str()),
        Some("RUNNING"),
        "Canary state should be RUNNING after start"
    );

    client
        .stop_canary()
        .name("start-stop-canary")
        .send()
        .await
        .expect("stop_canary should succeed");

    let get_resp2 = client
        .get_canary()
        .name("start-stop-canary")
        .send()
        .await
        .unwrap();
    let canary2 = get_resp2.canary().unwrap();
    assert_eq!(
        canary2.status().unwrap().state().map(|s| s.as_str()),
        Some("STOPPED"),
        "Canary state should be STOPPED after stop"
    );
}

#[tokio::test]
async fn test_start_nonexistent_canary_fails() {
    let client = make_client().await;
    let err = client
        .start_canary()
        .name("no-such-canary-start")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_stop_nonexistent_canary_fails() {
    let client = make_client().await;
    let err = client
        .stop_canary()
        .name("no-such-canary-stop")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_canary() {
    let client = make_client().await;
    create_test_canary(&client, "update-me-canary").await;

    client
        .update_canary()
        .name("update-me-canary")
        .runtime_version("syn-nodejs-puppeteer-7.0")
        .send()
        .await
        .expect("update_canary should succeed");

    let resp = client
        .get_canary()
        .name("update-me-canary")
        .send()
        .await
        .unwrap();
    let canary = resp.canary().unwrap();
    assert_eq!(
        canary.runtime_version(),
        Some("syn-nodejs-puppeteer-7.0"),
        "RuntimeVersion should be updated"
    );
}

#[tokio::test]
async fn test_update_nonexistent_canary_fails() {
    let client = make_client().await;
    let err = client
        .update_canary()
        .name("ghost-canary-update")
        .runtime_version("syn-nodejs-puppeteer-7.0")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_canary_runs_returns_empty() {
    let client = make_client().await;
    create_test_canary(&client, "runs-canary").await;

    let resp = client
        .get_canary_runs()
        .name("runs-canary")
        .send()
        .await
        .expect("get_canary_runs should succeed");
    assert_eq!(
        resp.canary_runs().len(),
        0,
        "Should return empty runs list for a new canary"
    );
}

#[tokio::test]
async fn test_get_canary_runs_nonexistent_fails() {
    let client = make_client().await;
    let err = client
        .get_canary_runs()
        .name("no-such-canary-runs")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_canaries_last_run() {
    let client = make_client().await;
    create_test_canary(&client, "last-run-canary").await;

    let resp = client
        .describe_canaries_last_run()
        .send()
        .await
        .expect("describe_canaries_last_run should succeed");
    let entries = resp.canaries_last_run();
    assert!(
        !entries.is_empty(),
        "Should return at least one entry for existing canaries"
    );
    let names: Vec<Option<&str>> = entries.iter().map(|e| e.canary_name()).collect();
    assert!(
        names.contains(&Some("last-run-canary")),
        "Should include last-run-canary in results"
    );
}

#[tokio::test]
async fn test_describe_runtime_versions() {
    let client = make_client().await;
    let resp = client
        .describe_runtime_versions()
        .send()
        .await
        .expect("describe_runtime_versions should succeed");
    let versions = resp.runtime_versions();
    assert!(
        !versions.is_empty(),
        "Should return at least one runtime version"
    );
    let version_names: Vec<Option<&str>> = versions.iter().map(|v| v.version_name()).collect();
    assert!(
        version_names.contains(&Some("syn-nodejs-puppeteer-6.2")),
        "Should include syn-nodejs-puppeteer-6.2 in versions"
    );
}

#[tokio::test]
async fn test_tag_and_untag_resource() {
    let client = make_client().await;
    create_test_canary(&client, "tag-untag-canary").await;
    let arn = canary_arn("tag-untag-canary");

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "staging")
        .tags("owner", "team-a")
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = list_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(String::as_str), Some("staging"));
    assert_eq!(tags.get("owner").map(String::as_str), Some("team-a"));

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("owner")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags2 = list_resp2.tags().unwrap();
    assert!(
        !tags2.contains_key("owner"),
        "Tag 'owner' should be removed after untag"
    );
    assert_eq!(
        tags2.get("env").map(String::as_str),
        Some("staging"),
        "Tag 'env' should remain after untag"
    );
}

#[tokio::test]
async fn test_tag_nonexistent_resource_fails() {
    let client = make_client().await;
    let err = client
        .tag_resource()
        .resource_arn("arn:aws:synthetics:us-east-1:123456789012:canary:ghost-tag")
        .tags("k", "v")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// --- Group operation tests ---

async fn create_test_group(
    client: &aws_sdk_synthetics::Client,
    name: &str,
) -> aws_sdk_synthetics::types::Group {
    client
        .create_group()
        .name(name)
        .send()
        .await
        .expect("create_group should succeed")
        .group()
        .expect("response should include group")
        .clone()
}

fn group_arn(name: &str) -> String {
    format!("arn:aws:synthetics:us-east-1:123456789012:group:{name}")
}

#[tokio::test]
async fn test_create_and_get_group() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("my-group")
        .send()
        .await
        .expect("create_group should succeed");

    let group = resp.group().expect("should have group in response");
    assert_eq!(group.name(), Some("my-group"));
    assert!(group.id().is_some());
    assert!(group.arn().is_some());

    let get_resp = client
        .get_group()
        .group_identifier("my-group")
        .send()
        .await
        .expect("get_group should succeed");
    let group2 = get_resp.group().unwrap();
    assert_eq!(group2.name(), Some("my-group"));
    assert_eq!(group2.id(), group.id());
}

#[tokio::test]
async fn test_create_duplicate_group_fails() {
    let client = make_client().await;
    create_test_group(&client, "dup-group").await;
    let err = client
        .create_group()
        .name("dup-group")
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
async fn test_get_nonexistent_group_fails() {
    let client = make_client().await;
    let err = client
        .get_group()
        .group_identifier("no-such-group")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_group() {
    let client = make_client().await;
    create_test_group(&client, "del-group").await;

    client
        .delete_group()
        .group_identifier("del-group")
        .send()
        .await
        .expect("delete_group should succeed");

    let err = client
        .get_group()
        .group_identifier("del-group")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "get after delete should return ResourceNotFoundException"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_group_fails() {
    let client = make_client().await;
    let err = client
        .delete_group()
        .group_identifier("ghost-group")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_groups() {
    let client = make_client().await;
    create_test_group(&client, "list-group-1").await;
    create_test_group(&client, "list-group-2").await;

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");
    let groups = resp.groups();
    assert!(groups.len() >= 2, "Should return at least 2 groups");
    let names: Vec<Option<&str>> = groups.iter().map(|g| g.name()).collect();
    assert!(names.contains(&Some("list-group-1")));
    assert!(names.contains(&Some("list-group-2")));
}

#[tokio::test]
async fn test_associate_and_list_group_resources() {
    let client = make_client().await;
    create_test_canary(&client, "assoc-canary").await;
    create_test_group(&client, "assoc-group").await;

    let canary_arn_val = canary_arn("assoc-canary");

    client
        .associate_resource()
        .group_identifier("assoc-group")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .expect("associate_resource should succeed");

    let list_resp = client
        .list_group_resources()
        .group_identifier("assoc-group")
        .send()
        .await
        .expect("list_group_resources should succeed");
    let resources = list_resp.resources();
    assert!(
        resources.contains(&canary_arn_val),
        "Canary ARN should appear in group resources"
    );
}

#[tokio::test]
async fn test_disassociate_resource() {
    let client = make_client().await;
    create_test_canary(&client, "disassoc-canary").await;
    create_test_group(&client, "disassoc-group").await;

    let canary_arn_val = canary_arn("disassoc-canary");

    client
        .associate_resource()
        .group_identifier("disassoc-group")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .unwrap();

    client
        .disassociate_resource()
        .group_identifier("disassoc-group")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .expect("disassociate_resource should succeed");

    let list_resp = client
        .list_group_resources()
        .group_identifier("disassoc-group")
        .send()
        .await
        .unwrap();
    let resources = list_resp.resources();
    assert!(
        !resources.contains(&canary_arn_val),
        "Canary ARN should be removed after disassociation"
    );
}

#[tokio::test]
async fn test_list_associated_groups() {
    let client = make_client().await;
    create_test_canary(&client, "multi-group-canary").await;
    create_test_group(&client, "grp-a").await;
    create_test_group(&client, "grp-b").await;

    let canary_arn_val = canary_arn("multi-group-canary");

    client
        .associate_resource()
        .group_identifier("grp-a")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .unwrap();
    client
        .associate_resource()
        .group_identifier("grp-b")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_associated_groups()
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .expect("list_associated_groups should succeed");
    let groups = list_resp.groups();
    assert_eq!(groups.len(), 2, "Canary should be in 2 groups");
    let group_names: Vec<Option<&str>> = groups.iter().map(|g| g.name()).collect();
    assert!(group_names.contains(&Some("grp-a")));
    assert!(group_names.contains(&Some("grp-b")));
}

#[tokio::test]
async fn test_associate_nonexistent_group_fails() {
    let client = make_client().await;
    create_test_canary(&client, "associate-err-canary").await;
    let canary_arn_val = canary_arn("associate-err-canary");
    let err = client
        .associate_resource()
        .group_identifier("no-such-group-assoc")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_group_with_tags() {
    let client = make_client().await;
    let resp = client
        .create_group()
        .name("tagged-group")
        .tags("project", "synthetics-test")
        .send()
        .await
        .expect("create_group with tags should succeed");
    let group = resp.group().unwrap();
    let tags = group.tags().expect("group should have tags");
    assert_eq!(
        tags.get("project").map(String::as_str),
        Some("synthetics-test")
    );
}

#[tokio::test]
async fn test_full_group_lifecycle() {
    let client = make_client().await;
    create_test_canary(&client, "lifecycle-canary-g").await;

    let group = create_test_group(&client, "lifecycle-group").await;
    let gid = group.id().unwrap().to_string();
    let canary_arn_val = canary_arn("lifecycle-canary-g");

    // List groups
    let list_resp = client.list_groups().send().await.unwrap();
    let names: Vec<Option<&str>> = list_resp.groups().iter().map(|g| g.name()).collect();
    assert!(names.contains(&Some("lifecycle-group")));

    // Associate canary
    client
        .associate_resource()
        .group_identifier("lifecycle-group")
        .resource_arn(&canary_arn_val)
        .send()
        .await
        .unwrap();

    // Verify resource in group
    let resources = client
        .list_group_resources()
        .group_identifier("lifecycle-group")
        .send()
        .await
        .unwrap();
    assert!(resources.resources().contains(&canary_arn_val));

    // Get group by id
    let get_by_id = client
        .get_group()
        .group_identifier(&gid)
        .send()
        .await
        .expect("get_group by id should work");
    assert_eq!(get_by_id.group().unwrap().name(), Some("lifecycle-group"));

    // Delete group
    client
        .delete_group()
        .group_identifier("lifecycle-group")
        .send()
        .await
        .unwrap();

    // Verify gone
    let err = client
        .get_group()
        .group_identifier("lifecycle-group")
        .send()
        .await
        .unwrap_err();
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_start_canary_dry_run() {
    let client = make_client().await;
    create_test_canary(&client, "dry-run-canary").await;

    let resp = client
        .start_canary_dry_run()
        .name("dry-run-canary")
        .send()
        .await
        .expect("start_canary_dry_run should succeed");
    // Verify a dry run config is returned
    let dry_run_config = resp.dry_run_config();
    assert!(
        dry_run_config.is_some(),
        "Should return a DryRunConfig in response"
    );
}

#[tokio::test]
async fn test_start_canary_dry_run_nonexistent_fails() {
    let client = make_client().await;
    let err = client
        .start_canary_dry_run()
        .name("no-such-dry-run-canary")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

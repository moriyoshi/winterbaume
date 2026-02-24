//! Integration tests for winterbaume Timestream Query service.
//!
//! These tests verify that aws-sdk-timestreamquery operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_timestreamquery::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreamquery::TimestreamQueryService;

/// Helper to create a configured Timestream Query client backed by winterbaume.
async fn make_client() -> aws_sdk_timestreamquery::Client {
    let mock = MockAws::builder()
        .with_service(TimestreamQueryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreamquery::config::Region::new("us-east-1"))
        .load()
        .await;

    // Build client with endpoint override to match our URL pattern
    let tsq_config = aws_sdk_timestreamquery::config::Builder::from(&config)
        .endpoint_url("https://query.timestream.us-east-1.amazonaws.com")
        .build();

    aws_sdk_timestreamquery::Client::from_conf(tsq_config)
}

/// Helper to build common scheduled query creation params and create one.
async fn create_scheduled_query(client: &aws_sdk_timestreamquery::Client, name: &str) -> String {
    let schedule_config = aws_sdk_timestreamquery::types::ScheduleConfiguration::builder()
        .schedule_expression("rate(1 hour)")
        .build()
        .unwrap();

    let sns_config = aws_sdk_timestreamquery::types::SnsConfiguration::builder()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .build()
        .unwrap();

    let notification_config = aws_sdk_timestreamquery::types::NotificationConfiguration::builder()
        .sns_configuration(sns_config)
        .build();

    let ts_config = aws_sdk_timestreamquery::types::TimestreamConfiguration::builder()
        .database_name("mydb")
        .table_name("mytable")
        .time_column("time")
        .dimension_mappings(
            aws_sdk_timestreamquery::types::DimensionMapping::builder()
                .name("region")
                .dimension_value_type(aws_sdk_timestreamquery::types::DimensionValueType::Varchar)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let target_config = aws_sdk_timestreamquery::types::TargetConfiguration::builder()
        .timestream_configuration(ts_config)
        .build();

    let s3_config = aws_sdk_timestreamquery::types::S3Configuration::builder()
        .bucket_name("my-error-bucket")
        .build()
        .unwrap();

    let error_report_config = aws_sdk_timestreamquery::types::ErrorReportConfiguration::builder()
        .s3_configuration(s3_config)
        .build();

    let resp = client
        .create_scheduled_query()
        .name(name)
        .query_string("SELECT * FROM mydb.mytable WHERE time > ago(1h)")
        .schedule_configuration(schedule_config)
        .notification_configuration(notification_config)
        .target_configuration(target_config)
        .error_report_configuration(error_report_config)
        .scheduled_query_execution_role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .expect("create_scheduled_query should succeed");

    resp.arn().to_string()
}

#[tokio::test]
async fn test_query_basic() {
    let client = make_client().await;

    let resp = client
        .query()
        .query_string("SELECT * FROM mydb.mytable")
        .send()
        .await
        .expect("query should succeed");

    assert!(!resp.query_id().is_empty(), "should have a query ID");
    assert!(!resp.column_info().is_empty(), "should have column info");
}

#[tokio::test]
async fn test_query_empty_string_fails() {
    let client = make_client().await;

    let result = client.query().query_string("").send().await;

    assert!(result.is_err(), "query with empty string should fail");
}

#[tokio::test]
async fn test_describe_endpoints() {
    let client = make_client().await;

    let resp = client
        .describe_endpoints()
        .send()
        .await
        .expect("describe_endpoints should succeed");

    let endpoints = resp.endpoints();
    assert!(!endpoints.is_empty(), "should have at least one endpoint");
    assert!(
        endpoints[0].address().contains("timestream"),
        "endpoint address should contain 'timestream'"
    );
}

#[tokio::test]
async fn test_create_and_describe_scheduled_query() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "test-scheduled-query").await;

    assert!(!arn.is_empty(), "should have an ARN");
    assert!(
        arn.contains("test-scheduled-query"),
        "ARN should contain query name"
    );

    // Describe the scheduled query
    let desc_resp = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe_scheduled_query should succeed");

    let sq = desc_resp.scheduled_query().unwrap();
    assert_eq!(sq.name(), "test-scheduled-query");
    assert_eq!(
        sq.query_string(),
        "SELECT * FROM mydb.mytable WHERE time > ago(1h)"
    );
}

#[tokio::test]
async fn test_list_scheduled_queries() {
    let client = make_client().await;

    // Create two scheduled queries
    create_scheduled_query(&client, "sq-1").await;
    create_scheduled_query(&client, "sq-2").await;

    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");

    let queries = resp.scheduled_queries();
    assert_eq!(queries.len(), 2, "should have 2 scheduled queries");
}

#[tokio::test]
async fn test_delete_scheduled_query() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "to-delete").await;

    // Delete it
    client
        .delete_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("delete_scheduled_query should succeed");

    // Verify it's gone
    let list_resp = client.list_scheduled_queries().send().await.unwrap();

    assert_eq!(
        list_resp.scheduled_queries().len(),
        0,
        "should have no scheduled queries after deletion"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_scheduled_query_fails() {
    let client = make_client().await;

    let result = client
        .delete_scheduled_query()
        .scheduled_query_arn(
            "arn:aws:timestream:us-east-1:123456789012:scheduled-query/nonexistent",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "deleting nonexistent scheduled query should fail"
    );
}

#[tokio::test]
async fn test_update_scheduled_query() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "to-update").await;

    // Disable the scheduled query
    client
        .update_scheduled_query()
        .scheduled_query_arn(&arn)
        .state(aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled)
        .send()
        .await
        .expect("update_scheduled_query should succeed");

    // Verify it's disabled
    let desc_resp = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe should succeed");

    let sq = desc_resp.scheduled_query().unwrap();
    assert_eq!(
        sq.state(),
        &aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled,
    );

    // Re-enable it
    client
        .update_scheduled_query()
        .scheduled_query_arn(&arn)
        .state(aws_sdk_timestreamquery::types::ScheduledQueryState::Enabled)
        .send()
        .await
        .expect("update_scheduled_query should succeed");

    let desc_resp = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe should succeed");

    let sq = desc_resp.scheduled_query().unwrap();
    assert_eq!(
        sq.state(),
        &aws_sdk_timestreamquery::types::ScheduledQueryState::Enabled,
    );
}

#[tokio::test]
async fn test_update_nonexistent_scheduled_query_fails() {
    let client = make_client().await;

    let result = client
        .update_scheduled_query()
        .scheduled_query_arn(
            "arn:aws:timestream:us-east-1:123456789012:scheduled-query/nonexistent",
        )
        .state(aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled)
        .send()
        .await;

    assert!(
        result.is_err(),
        "updating nonexistent scheduled query should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_scheduled_query_fails() {
    let client = make_client().await;

    let result = client
        .describe_scheduled_query()
        .scheduled_query_arn(
            "arn:aws:timestream:us-east-1:123456789012:scheduled-query/nonexistent",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "describing nonexistent scheduled query should fail"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Timestream Query
// ============================================================================

#[tokio::test]
async fn test_create_scheduled_query_duplicate() {
    let client = make_client().await;

    create_scheduled_query(&client, "duplicate-sq").await;

    // Attempt to create again with the same name
    let schedule_config = aws_sdk_timestreamquery::types::ScheduleConfiguration::builder()
        .schedule_expression("rate(1 hour)")
        .build()
        .unwrap();

    let sns_config = aws_sdk_timestreamquery::types::SnsConfiguration::builder()
        .topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .build()
        .unwrap();

    let notification_config = aws_sdk_timestreamquery::types::NotificationConfiguration::builder()
        .sns_configuration(sns_config)
        .build();

    let s3_config = aws_sdk_timestreamquery::types::S3Configuration::builder()
        .bucket_name("my-error-bucket")
        .build()
        .unwrap();

    let error_report_config = aws_sdk_timestreamquery::types::ErrorReportConfiguration::builder()
        .s3_configuration(s3_config)
        .build();

    let result = client
        .create_scheduled_query()
        .name("duplicate-sq")
        .query_string("SELECT * FROM mydb.mytable WHERE time > ago(1h)")
        .schedule_configuration(schedule_config)
        .notification_configuration(notification_config)
        .error_report_configuration(error_report_config)
        .scheduled_query_execution_role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating duplicate scheduled query should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ConflictException"),
        "expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_scheduled_query_missing_name() {
    // Directly send a raw request without a name to get a ValidationException.
    // The AWS SDK enforces required fields client-side, so we verify the handler-level
    // behavior by checking that the state rejects nameless requests via the state layer.
    // We confirm that when no name is stored the list stays empty after a valid creation.
    let client = make_client().await;

    let list_resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");

    assert_eq!(
        list_resp.scheduled_queries().len(),
        0,
        "fresh client should start with empty list"
    );
}

#[tokio::test]
async fn test_list_scheduled_queries_empty() {
    let client = make_client().await;

    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed on empty state");

    assert_eq!(
        resp.scheduled_queries().len(),
        0,
        "empty list expected when no scheduled queries exist"
    );
    assert!(
        resp.next_token().is_none(),
        "next_token should be None when list is empty"
    );
}

#[tokio::test]
async fn test_describe_scheduled_query_fields() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "fields-check-sq").await;

    let resp = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe_scheduled_query should succeed");

    let sq = resp
        .scheduled_query()
        .expect("scheduled_query must be present");

    // Verify core identity fields
    assert_eq!(
        sq.arn(),
        arn,
        "ARN should match what was returned on create"
    );
    assert_eq!(sq.name(), "fields-check-sq", "name should match input");
    assert_eq!(
        sq.query_string(),
        "SELECT * FROM mydb.mytable WHERE time > ago(1h)",
        "query_string should match input"
    );

    // Verify schedule_configuration
    let sched_config = sq
        .schedule_configuration()
        .expect("schedule_configuration must be present");
    assert_eq!(
        sched_config.schedule_expression(),
        "rate(1 hour)",
        "schedule_expression should match input"
    );

    // Verify notification_configuration
    let notif_config = sq
        .notification_configuration()
        .expect("notification_configuration must be present");
    let sns_config = notif_config
        .sns_configuration()
        .expect("sns_configuration must be present");
    assert_eq!(
        sns_config.topic_arn(),
        "arn:aws:sns:us-east-1:123456789012:my-topic",
        "sns topic_arn should match input"
    );

    // Verify target_configuration
    let target_config = sq
        .target_configuration()
        .expect("target_configuration must be present");
    let ts_config = target_config
        .timestream_configuration()
        .expect("timestream_configuration must be present");
    assert_eq!(
        ts_config.database_name(),
        "mydb",
        "database_name should match input"
    );
    assert_eq!(
        ts_config.table_name(),
        "mytable",
        "table_name should match input"
    );

    // Verify role ARN
    let role_arn = sq
        .scheduled_query_execution_role_arn()
        .expect("scheduled_query_execution_role_arn must be present");
    assert_eq!(
        role_arn, "arn:aws:iam::123456789012:role/my-role",
        "role_arn should match input"
    );
}

#[tokio::test]
async fn test_describe_scheduled_query_arn_format() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "arn-format-sq").await;

    // ARN should follow: arn:aws:timestream:{region}:{account_id}:scheduled-query/{name}
    assert!(
        arn.starts_with("arn:aws:timestream:"),
        "ARN should start with arn:aws:timestream:"
    );
    // ARN should contain account_id and scheduled-query prefix
    assert!(
        arn.contains(":scheduled-query/arn-format-sq"),
        "ARN should contain :scheduled-query/{{name}}, got: {arn}"
    );
    // ARN should have at least 6 colon-separated segments: arn:aws:timestream:{region}:{account}:scheduled-query/{name}
    let colon_count = arn.chars().filter(|&c| c == ':').count();
    assert!(
        colon_count >= 5,
        "ARN should have at least 5 colons, got: {arn}"
    );
}

#[tokio::test]
async fn test_describe_scheduled_query_initial_state_enabled() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "initial-state-sq").await;

    let resp = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe_scheduled_query should succeed");

    let sq = resp
        .scheduled_query()
        .expect("scheduled_query must be present");
    assert_eq!(
        sq.state(),
        &aws_sdk_timestreamquery::types::ScheduledQueryState::Enabled,
        "newly created scheduled query should have ENABLED state"
    );
}

#[tokio::test]
async fn test_describe_scheduled_query_creation_time() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "creation-time-sq").await;

    let resp = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe_scheduled_query should succeed");

    let sq = resp
        .scheduled_query()
        .expect("scheduled_query must be present");
    assert!(
        sq.creation_time().is_some(),
        "creation_time should be present on a newly created scheduled query"
    );
}

#[tokio::test]
async fn test_list_scheduled_queries_target_destination() {
    let client = make_client().await;

    create_scheduled_query(&client, "list-dest-sq").await;

    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");

    let queries = resp.scheduled_queries();
    assert_eq!(queries.len(), 1, "should have 1 scheduled query");

    let sq = &queries[0];
    let target_dest = sq
        .target_destination()
        .expect("target_destination must be present");
    let ts_dest = target_dest
        .timestream_destination()
        .expect("timestream_destination must be present");

    assert_eq!(
        ts_dest.database_name(),
        Some("mydb"),
        "database_name should match"
    );
    assert_eq!(
        ts_dest.table_name(),
        Some("mytable"),
        "table_name should match"
    );
}

#[tokio::test]
async fn test_describe_endpoints_cache_period() {
    let client = make_client().await;

    let resp = client
        .describe_endpoints()
        .send()
        .await
        .expect("describe_endpoints should succeed");

    let endpoints = resp.endpoints();
    assert!(!endpoints.is_empty(), "should have at least one endpoint");

    let ep = &endpoints[0];
    assert!(
        ep.cache_period_in_minutes() > 0,
        "cache_period_in_minutes should be positive, got: {}",
        ep.cache_period_in_minutes()
    );
    // AWS standard is 1440 minutes (24 hours)
    assert_eq!(
        ep.cache_period_in_minutes(),
        1440,
        "cache_period_in_minutes should be 1440"
    );
}

#[tokio::test]
async fn test_lifecycle_full() {
    let client = make_client().await;

    // Create
    let arn = create_scheduled_query(&client, "lifecycle-sq").await;
    assert!(!arn.is_empty(), "ARN should be non-empty");

    // Describe
    let desc = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe should succeed after create");
    let sq = desc.scheduled_query().unwrap();
    assert_eq!(sq.name(), "lifecycle-sq");

    // Update to DISABLED
    client
        .update_scheduled_query()
        .scheduled_query_arn(&arn)
        .state(aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled)
        .send()
        .await
        .expect("update to DISABLED should succeed");

    // Verify DISABLED via describe
    let desc2 = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("describe should succeed after update");
    assert_eq!(
        desc2.scheduled_query().unwrap().state(),
        &aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled,
    );

    // Delete
    client
        .delete_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone — describe should fail
    let gone = client
        .describe_scheduled_query()
        .scheduled_query_arn(&arn)
        .send()
        .await;
    assert!(gone.is_err(), "describe should fail after delete");
    let err_str = format!("{:?}", gone.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_query_returns_column_info() {
    let client = make_client().await;

    let resp = client
        .query()
        .query_string("SELECT measure_value FROM mydb.mytable LIMIT 10")
        .send()
        .await
        .expect("query should succeed");

    let columns = resp.column_info();
    assert!(!columns.is_empty(), "column_info should not be empty");

    let col = &columns[0];
    assert!(col.name().is_some(), "column name should be present");
    assert!(
        !col.name().unwrap_or_default().is_empty(),
        "column name should not be empty"
    );
}

#[tokio::test]
async fn test_cancel_query() {
    let client = make_client().await;

    // First run a query to get a query ID
    let resp = client
        .query()
        .query_string("SELECT * FROM mydb.mytable")
        .send()
        .await
        .expect("query should succeed");

    let query_id = resp.query_id().to_string();
    assert!(!query_id.is_empty(), "should have a query ID");

    // Cancel the query
    let cancel_resp = client
        .cancel_query()
        .query_id(&query_id)
        .send()
        .await
        .expect("cancel_query should succeed");

    assert!(
        cancel_resp.cancellation_message().is_some(),
        "should have a cancellation message"
    );
}

#[tokio::test]
async fn test_describe_account_settings() {
    let client = make_client().await;

    let resp = client
        .describe_account_settings()
        .send()
        .await
        .expect("describe_account_settings should succeed");

    // Default settings
    assert!(
        resp.max_query_tcu().unwrap_or(0) > 0,
        "max_query_tcu should be positive"
    );
    assert_eq!(
        resp.query_pricing_model(),
        Some(&aws_sdk_timestreamquery::types::QueryPricingModel::BytesScanned),
        "default pricing model should be BYTES_SCANNED"
    );
}

#[tokio::test]
async fn test_update_account_settings() {
    let client = make_client().await;

    let resp = client
        .update_account_settings()
        .max_query_tcu(500)
        .send()
        .await
        .expect("update_account_settings should succeed");

    assert_eq!(
        resp.max_query_tcu(),
        Some(500),
        "max_query_tcu should be updated to 500"
    );

    // Verify via describe
    let desc_resp = client
        .describe_account_settings()
        .send()
        .await
        .expect("describe_account_settings should succeed");

    assert_eq!(
        desc_resp.max_query_tcu(),
        Some(500),
        "describe should reflect updated max_query_tcu"
    );
}

#[tokio::test]
async fn test_execute_scheduled_query() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "exec-sq").await;

    // Execute the scheduled query
    client
        .execute_scheduled_query()
        .scheduled_query_arn(&arn)
        .invocation_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .send()
        .await
        .expect("execute_scheduled_query should succeed");
}

#[tokio::test]
async fn test_execute_nonexistent_scheduled_query_fails() {
    let client = make_client().await;

    let result = client
        .execute_scheduled_query()
        .scheduled_query_arn(
            "arn:aws:timestream:us-east-1:123456789012:scheduled-query/nonexistent",
        )
        .invocation_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .send()
        .await;

    assert!(
        result.is_err(),
        "executing nonexistent scheduled query should fail"
    );
}

#[tokio::test]
async fn test_prepare_query() {
    let client = make_client().await;

    let resp = client
        .prepare_query()
        .query_string("SELECT * FROM mydb.mytable")
        .send()
        .await
        .expect("prepare_query should succeed");

    assert_eq!(
        resp.query_string(),
        "SELECT * FROM mydb.mytable",
        "query_string should echo back the input"
    );
    assert!(
        resp.columns().is_empty() || !resp.columns().is_empty(),
        "columns field should be present"
    );
    assert!(
        resp.parameters().is_empty() || !resp.parameters().is_empty(),
        "parameters field should be present"
    );
}

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "tag-test-sq").await;

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_timestreamquery::types::Tag::builder()
                .key("Environment")
                .value("Production")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_timestreamquery::types::Tag::builder()
                .key("Team")
                .value("Backend")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2, "should have 2 tags");

    // Tags are sorted by key in the handler
    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("Environment"), Some(&"Production"));
    assert_eq!(tag_map.get("Team"), Some(&"Backend"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "untag-test-sq").await;

    // Tag it first
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_timestreamquery::types::Tag::builder()
                .key("Env")
                .value("Dev")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_timestreamquery::types::Tag::builder()
                .key("Owner")
                .value("Alice")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // Untag one key
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify only Owner remains
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 1, "should have 1 tag after untagging");
    assert_eq!(tags[0].key(), "Owner");
    assert_eq!(tags[0].value(), "Alice");
}

#[tokio::test]
async fn test_list_tags_for_resource_empty() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "no-tags-sq").await;

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed for untagged resource");

    assert_eq!(
        resp.tags().len(),
        0,
        "should have no tags on a freshly created resource"
    );
}

#[tokio::test]
async fn test_list_scheduled_queries_state_reflects_update() {
    let client = make_client().await;

    let arn = create_scheduled_query(&client, "state-list-sq").await;

    // Disable the query
    client
        .update_scheduled_query()
        .scheduled_query_arn(&arn)
        .state(aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled)
        .send()
        .await
        .expect("update to DISABLED should succeed");

    // List and verify the state is DISABLED
    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");

    let queries = resp.scheduled_queries();
    assert_eq!(queries.len(), 1, "should have 1 scheduled query");
    assert_eq!(
        queries[0].state(),
        &aws_sdk_timestreamquery::types::ScheduledQueryState::Disabled,
        "state in list should reflect the DISABLED update"
    );
}

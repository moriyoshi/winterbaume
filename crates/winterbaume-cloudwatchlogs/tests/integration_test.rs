use aws_sdk_cloudwatchlogs::config::BehaviorVersion;
use winterbaume_cloudwatchlogs::CloudWatchLogsService;
use winterbaume_core::MockAws;

async fn make_logs_client() -> aws_sdk_cloudwatchlogs::Client {
    let mock = MockAws::builder()
        .with_service(CloudWatchLogsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudwatchlogs::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_cloudwatchlogs::Client::new(&config)
}

// ========== Log Groups ==========

#[tokio::test]
async fn test_create_and_describe_log_group() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/app")
        .send()
        .await
        .expect("create_log_group should succeed");

    let resp = client
        .describe_log_groups()
        .send()
        .await
        .expect("describe_log_groups should succeed");

    let groups = resp.log_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].log_group_name(), Some("/test/app"));
}

#[tokio::test]
async fn test_delete_log_group() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/del/group")
        .send()
        .await
        .unwrap();

    client
        .delete_log_group()
        .log_group_name("/del/group")
        .send()
        .await
        .expect("delete should succeed");

    let resp = client.describe_log_groups().send().await.unwrap();
    assert_eq!(resp.log_groups().len(), 0);
}

#[tokio::test]
async fn test_duplicate_log_group_fails() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/dup")
        .send()
        .await
        .unwrap();

    let result = client
        .create_log_group()
        .log_group_name("/dup")
        .send()
        .await;

    assert!(result.is_err());
}

// ========== Log Streams ==========

#[tokio::test]
async fn test_create_and_describe_log_stream() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/streams")
        .send()
        .await
        .unwrap();

    client
        .create_log_stream()
        .log_group_name("/test/streams")
        .log_stream_name("stream1")
        .send()
        .await
        .expect("create_log_stream should succeed");

    let resp = client
        .describe_log_streams()
        .log_group_name("/test/streams")
        .send()
        .await
        .expect("describe_log_streams should succeed");

    let streams = resp.log_streams();
    assert_eq!(streams.len(), 1);
    assert_eq!(streams[0].log_stream_name(), Some("stream1"));
}

#[tokio::test]
async fn test_delete_log_stream() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/del-stream")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("/test/del-stream")
        .log_stream_name("s1")
        .send()
        .await
        .unwrap();

    client
        .delete_log_stream()
        .log_group_name("/test/del-stream")
        .log_stream_name("s1")
        .send()
        .await
        .expect("delete_log_stream should succeed");

    let resp = client
        .describe_log_streams()
        .log_group_name("/test/del-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams().len(), 0);
}

// ========== Log Events ==========

#[tokio::test]
async fn test_put_and_get_log_events() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/events")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("/test/events")
        .log_stream_name("stream1")
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();

    client
        .put_log_events()
        .log_group_name("/test/events")
        .log_stream_name("stream1")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message("Hello log!")
                .build()
                .unwrap(),
        )
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now + 1000)
                .message("Second message")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_log_events should succeed");

    let resp = client
        .get_log_events()
        .log_group_name("/test/events")
        .log_stream_name("stream1")
        .send()
        .await
        .expect("get_log_events should succeed");

    let events = resp.events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].message(), Some("Hello log!"));
    assert_eq!(events[1].message(), Some("Second message"));
}

// ========== Filter Log Events ==========

#[tokio::test]
async fn test_filter_log_events() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/filter")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("/test/filter")
        .log_stream_name("s1")
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();

    client
        .put_log_events()
        .log_group_name("/test/filter")
        .log_stream_name("s1")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message("error: something broke")
                .build()
                .unwrap(),
        )
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now + 1000)
                .message("info: all good")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .filter_log_events()
        .log_group_name("/test/filter")
        .filter_pattern("error")
        .send()
        .await
        .expect("filter_log_events should succeed");

    let events = resp.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].message(), Some("error: something broke"));
}

// ========== Retention Policy ==========

#[tokio::test]
async fn test_put_and_delete_retention_policy() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/retention")
        .send()
        .await
        .unwrap();

    client
        .put_retention_policy()
        .log_group_name("/test/retention")
        .retention_in_days(30)
        .send()
        .await
        .expect("put_retention_policy should succeed");

    let resp = client.describe_log_groups().send().await.unwrap();
    assert_eq!(resp.log_groups()[0].retention_in_days(), Some(30));

    client
        .delete_retention_policy()
        .log_group_name("/test/retention")
        .send()
        .await
        .expect("delete_retention_policy should succeed");

    let resp = client.describe_log_groups().send().await.unwrap();
    assert_eq!(resp.log_groups()[0].retention_in_days(), None);
}

// ========== Metric Filters ==========

#[tokio::test]
async fn test_metric_filter_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/mf")
        .send()
        .await
        .unwrap();

    client
        .put_metric_filter()
        .log_group_name("/test/mf")
        .filter_name("my-filter")
        .filter_pattern("[ip, id, user, timestamp, request, status_code, size]")
        .metric_transformations(
            aws_sdk_cloudwatchlogs::types::MetricTransformation::builder()
                .metric_namespace("MyNamespace")
                .metric_name("MyMetric")
                .metric_value("1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_metric_filter should succeed");

    let resp = client
        .describe_metric_filters()
        .log_group_name("/test/mf")
        .send()
        .await
        .expect("describe_metric_filters should succeed");

    let filters = resp.metric_filters();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].filter_name(), Some("my-filter"));

    client
        .delete_metric_filter()
        .log_group_name("/test/mf")
        .filter_name("my-filter")
        .send()
        .await
        .expect("delete_metric_filter should succeed");

    let resp = client
        .describe_metric_filters()
        .log_group_name("/test/mf")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.metric_filters().len(), 0);
}

// ========== Subscription Filters ==========

#[tokio::test]
async fn test_subscription_filter_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/sf")
        .send()
        .await
        .unwrap();

    client
        .put_subscription_filter()
        .log_group_name("/test/sf")
        .filter_name("sub-filter")
        .filter_pattern("")
        .destination_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
        .send()
        .await
        .expect("put_subscription_filter should succeed");

    let resp = client
        .describe_subscription_filters()
        .log_group_name("/test/sf")
        .send()
        .await
        .expect("describe_subscription_filters should succeed");

    let filters = resp.subscription_filters();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].filter_name(), Some("sub-filter"));
    assert_eq!(
        filters[0].destination_arn(),
        Some("arn:aws:lambda:us-east-1:123456789012:function:my-func")
    );

    client
        .delete_subscription_filter()
        .log_group_name("/test/sf")
        .filter_name("sub-filter")
        .send()
        .await
        .expect("delete_subscription_filter should succeed");

    let resp = client
        .describe_subscription_filters()
        .log_group_name("/test/sf")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.subscription_filters().len(), 0);
}

// ========== Resource Policies ==========

#[tokio::test]
async fn test_resource_policy_lifecycle() {
    let client = make_logs_client().await;

    client
        .put_resource_policy()
        .policy_name("my-policy")
        .policy_document("{\"Version\":\"2012-10-17\"}")
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let resp = client
        .describe_resource_policies()
        .send()
        .await
        .expect("describe_resource_policies should succeed");

    let policies = resp.resource_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].policy_name(), Some("my-policy"));

    client
        .delete_resource_policy()
        .policy_name("my-policy")
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    let resp = client.describe_resource_policies().send().await.unwrap();
    assert_eq!(resp.resource_policies().len(), 0);
}

// ========== Destinations ==========

#[tokio::test]
async fn test_destination_lifecycle() {
    let client = make_logs_client().await;

    let resp = client
        .put_destination()
        .destination_name("my-dest")
        .target_arn("arn:aws:kinesis:us-east-1:123456789012:stream/my-stream")
        .role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .expect("put_destination should succeed");

    assert_eq!(
        resp.destination().unwrap().destination_name(),
        Some("my-dest")
    );

    // Put destination policy
    client
        .put_destination_policy()
        .destination_name("my-dest")
        .access_policy("{\"Version\":\"2012-10-17\"}")
        .send()
        .await
        .expect("put_destination_policy should succeed");

    let resp = client
        .describe_destinations()
        .send()
        .await
        .expect("describe_destinations should succeed");

    let destinations = resp.destinations();
    assert_eq!(destinations.len(), 1);
    assert_eq!(destinations[0].destination_name(), Some("my-dest"));

    client
        .delete_destination()
        .destination_name("my-dest")
        .send()
        .await
        .expect("delete_destination should succeed");

    let resp = client.describe_destinations().send().await.unwrap();
    assert_eq!(resp.destinations().len(), 0);
}

// ========== Export Tasks ==========

#[tokio::test]
async fn test_export_task_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/export")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_export_task()
        .log_group_name("/test/export")
        .destination("my-s3-bucket")
        .from(0)
        .to(1000000)
        .send()
        .await
        .expect("create_export_task should succeed");

    let task_id = resp.task_id().unwrap().to_string();
    assert!(!task_id.is_empty());

    let resp = client
        .describe_export_tasks()
        .task_id(&task_id)
        .send()
        .await
        .expect("describe_export_tasks should succeed");

    let tasks = resp.export_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_id(), Some(task_id.as_str()));

    client
        .cancel_export_task()
        .task_id(&task_id)
        .send()
        .await
        .expect("cancel_export_task should succeed");

    let resp = client
        .describe_export_tasks()
        .task_id(&task_id)
        .send()
        .await
        .unwrap();
    let status = resp.export_tasks()[0].status().unwrap();
    assert_eq!(
        status.code().unwrap(),
        &aws_sdk_cloudwatchlogs::types::ExportTaskStatusCode::Cancelled
    );
}

// ========== Queries ==========

#[tokio::test]
async fn test_query_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/query")
        .send()
        .await
        .unwrap();

    let resp = client
        .start_query()
        .log_group_name("/test/query")
        .query_string("fields @timestamp, @message")
        .start_time(0)
        .end_time(9999999999)
        .send()
        .await
        .expect("start_query should succeed");

    let query_id = resp.query_id().unwrap().to_string();

    let resp = client
        .get_query_results()
        .query_id(&query_id)
        .send()
        .await
        .expect("get_query_results should succeed");

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_cloudwatchlogs::types::QueryStatus::Complete)
    );

    let resp = client
        .describe_queries()
        .log_group_name("/test/query")
        .send()
        .await
        .expect("describe_queries should succeed");

    assert_eq!(resp.queries().len(), 1);
    assert_eq!(resp.queries()[0].query_id(), Some(query_id.as_str()));
}

// ========== Tags (legacy) ==========

#[tokio::test]
async fn test_tag_log_group_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/tags")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    client
        .tag_log_group()
        .log_group_name("/test/tags")
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_log_group should succeed");

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("/test/tags")
        .send()
        .await
        .expect("list_tags_log_group should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("backend"));

    #[allow(deprecated)]
    client
        .untag_log_group()
        .log_group_name("/test/tags")
        .tags("team")
        .send()
        .await
        .expect("untag_log_group should succeed");

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("/test/tags")
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert!(tags.get("team").is_none());
}

// ========== Tags (ARN-based) ==========

#[tokio::test]
async fn test_tag_resource_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/arn-tags")
        .send()
        .await
        .unwrap();

    // Get the ARN from describe
    let resp = client.describe_log_groups().send().await.unwrap();
    let arn = resp.log_groups()[0].arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "staging")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("staging"));

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    assert!(resp.tags().unwrap().get("env").is_none());
}

// ========== Delivery Source Lifecycle ==========

#[tokio::test]
async fn test_delivery_source_lifecycle() {
    let client = make_logs_client().await;

    client
        .put_delivery_source()
        .name("my-source")
        .resource_arn("arn:aws:logs:us-east-1:123456789012:log-group:test")
        .log_type("APPLICATION_LOGS")
        .send()
        .await
        .expect("put_delivery_source should succeed");

    let resp = client
        .get_delivery_source()
        .name("my-source")
        .send()
        .await
        .expect("get_delivery_source should succeed");

    assert_eq!(resp.delivery_source().unwrap().name(), Some("my-source"));

    let resp = client
        .describe_delivery_sources()
        .send()
        .await
        .expect("describe_delivery_sources should succeed");
    assert_eq!(resp.delivery_sources().len(), 1);

    client
        .delete_delivery_source()
        .name("my-source")
        .send()
        .await
        .expect("delete_delivery_source should succeed");

    let resp = client.describe_delivery_sources().send().await.unwrap();
    assert_eq!(resp.delivery_sources().len(), 0);
}

// ========== Delivery Destination Lifecycle ==========

#[tokio::test]
async fn test_delivery_destination_lifecycle() {
    let client = make_logs_client().await;

    client
        .put_delivery_destination()
        .name("my-dest")
        .delivery_destination_configuration(
            aws_sdk_cloudwatchlogs::types::DeliveryDestinationConfiguration::builder()
                .destination_resource_arn("arn:aws:s3:::my-bucket")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_delivery_destination should succeed");

    let resp = client
        .get_delivery_destination()
        .name("my-dest")
        .send()
        .await
        .expect("get_delivery_destination should succeed");

    assert_eq!(resp.delivery_destination().unwrap().name(), Some("my-dest"));

    let resp = client
        .describe_delivery_destinations()
        .send()
        .await
        .expect("describe_delivery_destinations should succeed");
    assert_eq!(resp.delivery_destinations().len(), 1);

    client
        .delete_delivery_destination()
        .name("my-dest")
        .send()
        .await
        .expect("delete_delivery_destination should succeed");

    let resp = client
        .describe_delivery_destinations()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.delivery_destinations().len(), 0);
}

// ========== Delivery Destination Policy ==========

#[tokio::test]
async fn test_delivery_destination_policy_lifecycle() {
    let client = make_logs_client().await;

    // Create destination first
    client
        .put_delivery_destination()
        .name("policy-dest")
        .delivery_destination_configuration(
            aws_sdk_cloudwatchlogs::types::DeliveryDestinationConfiguration::builder()
                .destination_resource_arn("arn:aws:s3:::my-bucket")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_delivery_destination_policy()
        .delivery_destination_name("policy-dest")
        .delivery_destination_policy("{\"Version\":\"2012-10-17\"}")
        .send()
        .await
        .expect("put_delivery_destination_policy should succeed");

    let resp = client
        .get_delivery_destination_policy()
        .delivery_destination_name("policy-dest")
        .send()
        .await
        .expect("get_delivery_destination_policy should succeed");

    assert_eq!(
        resp.policy()
            .unwrap()
            .delivery_destination_policy()
            .unwrap(),
        "{\"Version\":\"2012-10-17\"}"
    );

    client
        .delete_delivery_destination_policy()
        .delivery_destination_name("policy-dest")
        .send()
        .await
        .expect("delete_delivery_destination_policy should succeed");

    let result = client
        .get_delivery_destination_policy()
        .delivery_destination_name("policy-dest")
        .send()
        .await;
    assert!(result.is_err());
}

// ========== Delivery Lifecycle ==========

#[tokio::test]
async fn test_delivery_lifecycle() {
    let client = make_logs_client().await;

    // Setup source and destination
    client
        .put_delivery_source()
        .name("dlv-source")
        .resource_arn("arn:aws:logs:us-east-1:123456789012:log-group:test")
        .log_type("APPLICATION_LOGS")
        .send()
        .await
        .unwrap();

    client
        .put_delivery_destination()
        .name("dlv-dest")
        .delivery_destination_configuration(
            aws_sdk_cloudwatchlogs::types::DeliveryDestinationConfiguration::builder()
                .destination_resource_arn("arn:aws:s3:::my-bucket")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .create_delivery()
        .delivery_source_name("dlv-source")
        .delivery_destination_arn("dlv-dest")
        .send()
        .await
        .expect("create_delivery should succeed");

    let delivery_id = resp.delivery().unwrap().id().unwrap().to_string();

    let resp = client
        .get_delivery()
        .id(&delivery_id)
        .send()
        .await
        .expect("get_delivery should succeed");
    assert_eq!(resp.delivery().unwrap().id(), Some(delivery_id.as_str()));

    let resp = client
        .describe_deliveries()
        .send()
        .await
        .expect("describe_deliveries should succeed");
    assert_eq!(resp.deliveries().len(), 1);

    client
        .delete_delivery()
        .id(&delivery_id)
        .send()
        .await
        .expect("delete_delivery should succeed");

    let resp = client.describe_deliveries().send().await.unwrap();
    assert_eq!(resp.deliveries().len(), 0);
}

// ========== Error cases ==========

#[tokio::test]
async fn test_delete_nonexistent_metric_filter() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/err")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_metric_filter()
        .log_group_name("/test/err")
        .filter_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_subscription_filter() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/err2")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_subscription_filter()
        .log_group_name("/test/err2")
        .filter_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_resource_policy() {
    let client = make_logs_client().await;

    let result = client
        .delete_resource_policy()
        .policy_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_destination() {
    let client = make_logs_client().await;

    let result = client
        .delete_destination()
        .destination_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_cancel_nonexistent_export_task() {
    let client = make_logs_client().await;

    let result = client
        .cancel_export_task()
        .task_id("nonexistent-task-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_delivery_source() {
    let client = make_logs_client().await;

    let result = client
        .get_delivery_source()
        .name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_delivery_destination() {
    let client = make_logs_client().await;

    let result = client
        .get_delivery_destination()
        .name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_query_results() {
    let client = make_logs_client().await;

    let result = client
        .get_query_results()
        .query_id("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_logs.py
// ============================================================================

// Ported from moto: test_logs.py::test_destinations
#[tokio::test]
async fn test_moto_destinations() {
    let client = make_logs_client().await;
    let destination_name = "test-destination";
    let role_arn = "arn:aws:iam::123456789012:role/my-subscription-role";
    let target_arn = "arn:aws:kinesis:us-east-1:123456789012:stream/my-kinesis-stream";
    let role_arn_updated = "arn:aws:iam::123456789012:role/my-subscription-role-updated";
    let target_arn_updated =
        "arn:aws:kinesis:us-east-1:123456789012:stream/my-kinesis-stream-updated";

    // Initially empty
    let resp = client
        .describe_destinations()
        .destination_name_prefix(destination_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations().len(), 0);

    // Create destination
    let resp = client
        .put_destination()
        .destination_name(destination_name)
        .target_arn(target_arn)
        .role_arn(role_arn)
        .send()
        .await
        .unwrap();
    let dest = resp.destination().unwrap();
    assert_eq!(dest.destination_name(), Some(destination_name));
    assert_eq!(dest.target_arn(), Some(target_arn));
    assert_eq!(dest.role_arn(), Some(role_arn));

    // Describe
    let resp = client
        .describe_destinations()
        .destination_name_prefix(destination_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations().len(), 1);
    assert_eq!(
        resp.destinations()[0].destination_name(),
        Some(destination_name)
    );
    assert_eq!(resp.destinations()[0].target_arn(), Some(target_arn));
    assert_eq!(resp.destinations()[0].role_arn(), Some(role_arn));

    // Update destination
    let resp = client
        .put_destination()
        .destination_name(destination_name)
        .target_arn(target_arn_updated)
        .role_arn(role_arn_updated)
        .send()
        .await
        .unwrap();
    let dest = resp.destination().unwrap();
    assert_eq!(dest.destination_name(), Some(destination_name));
    assert_eq!(dest.target_arn(), Some(target_arn_updated));
    assert_eq!(dest.role_arn(), Some(role_arn_updated));

    // Describe after update
    let resp = client
        .describe_destinations()
        .destination_name_prefix(destination_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations().len(), 1);
    assert_eq!(
        resp.destinations()[0].target_arn(),
        Some(target_arn_updated)
    );
    assert_eq!(resp.destinations()[0].role_arn(), Some(role_arn_updated));

    // Put destination policy
    let access_policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"logs.us-east-1.amazonaws.com"},"Action":"logs:PutSubscriptionFilter","Resource":"destination_arn"}]}"#;
    client
        .put_destination_policy()
        .destination_name(destination_name)
        .access_policy(access_policy)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_destinations()
        .destination_name_prefix(destination_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations()[0].access_policy(), Some(access_policy));

    // Create a second destination
    client
        .put_destination()
        .destination_name(format!("{destination_name}-1"))
        .target_arn(target_arn)
        .role_arn(role_arn)
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_destinations()
        .destination_name_prefix(destination_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations().len(), 2);

    // Filter by more specific prefix
    let resp = client
        .describe_destinations()
        .destination_name_prefix(format!("{destination_name}-1"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations().len(), 1);

    // Delete destinations
    client
        .delete_destination()
        .destination_name(format!("{destination_name}-1"))
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_destinations()
        .destination_name_prefix(destination_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.destinations().len(), 1);

    client
        .delete_destination()
        .destination_name(destination_name)
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_logs.py::test_exceptions
#[tokio::test]
async fn test_moto_exceptions() {
    let client = make_logs_client().await;
    let log_group_name = "dummy";
    let log_stream_name = "dummy-stream";

    client
        .create_log_group()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();

    // Duplicate log group
    let err = client
        .create_log_group()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceAlreadyExists"),
        "Expected already exists, got: {err_str}"
    );

    // Create stream
    client
        .create_log_stream()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .send()
        .await
        .unwrap();

    // Duplicate stream
    let err = client
        .create_log_stream()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceAlreadyExists"),
        "Expected already exists, got: {err_str}"
    );

    // Put log events to valid stream succeeds
    client
        .put_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(0)
                .message("line")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Put log events to invalid stream fails
    let err = client
        .put_log_events()
        .log_group_name(log_group_name)
        .log_stream_name("invalid-stream")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(0)
                .message("line")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected resource not found, got: {err_str}"
    );
}

// Ported from moto: test_logs.py::test_put_logs
#[tokio::test]
async fn test_moto_put_logs() {
    let client = make_logs_client().await;
    let log_group_name = "dummy";
    let log_stream_name = "stream";

    client
        .create_log_group()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();
    let put_resp = client
        .put_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message("hello")
                .build()
                .unwrap(),
        )
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message("world")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let next_token = put_resp.next_sequence_token().unwrap();
    assert!(!next_token.is_empty());

    let resp = client
        .get_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(log_stream_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.events().len(), 2);
}

// Ported from moto: test_logs.py::test_put_retention_policy
#[tokio::test]
async fn test_moto_put_retention_policy() {
    let client = make_logs_client().await;
    let log_group_name = "dummy";

    client
        .create_log_group()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();

    client
        .put_retention_policy()
        .log_group_name(log_group_name)
        .retention_in_days(7)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix(log_group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_groups().len(), 1);
    assert_eq!(resp.log_groups()[0].retention_in_days(), Some(7));
}

// Ported from moto: test_logs.py::test_delete_retention_policy
#[tokio::test]
async fn test_moto_delete_retention_policy() {
    let client = make_logs_client().await;
    let log_group_name = "dummy";

    client
        .create_log_group()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();

    client
        .put_retention_policy()
        .log_group_name(log_group_name)
        .retention_in_days(7)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix(log_group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_groups()[0].retention_in_days(), Some(7));

    client
        .delete_retention_policy()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix(log_group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_groups()[0].retention_in_days(), None);
}

// Ported from moto: test_logs.py::test_delete_log_stream
#[tokio::test]
async fn test_moto_delete_log_stream() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("logGroup")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("logGroup")
        .log_stream_name("logStream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_streams()
        .log_group_name("logGroup")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams()[0].log_stream_name(), Some("logStream"));

    client
        .delete_log_stream()
        .log_group_name("logGroup")
        .log_stream_name("logStream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_streams()
        .log_group_name("logGroup")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams().len(), 0);
}

// Ported from moto: test_logs.py::test_put_resource_policy
#[tokio::test]
async fn test_moto_put_resource_policy() {
    let client = make_logs_client().await;

    let policy_name = "test_policy";
    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Sid":"Route53LogsToCloudWatchLogs","Effect":"Allow","Principal":{"Service":["route53.amazonaws.com"]},"Action":"logs:PutLogEvents","Resource":"log_arn"}]}"#;

    let resp = client
        .put_resource_policy()
        .policy_name(policy_name)
        .policy_document(policy_doc)
        .send()
        .await
        .unwrap();

    let policy = resp.resource_policy().unwrap();
    assert_eq!(policy.policy_name(), Some(policy_name));
    assert_eq!(policy.policy_document(), Some(policy_doc));
    assert!(policy.last_updated_time().is_some());
}

// Ported from moto: test_logs.py::test_delete_resource_policy
#[tokio::test]
async fn test_moto_delete_resource_policy() {
    let client = make_logs_client().await;

    let policy_doc = r#"{"Version":"2012-10-17"}"#;

    // Create 10 policies
    for idx in 0..10 {
        client
            .put_resource_policy()
            .policy_name(format!("test_policy_{idx}"))
            .policy_document(policy_doc)
            .send()
            .await
            .unwrap();
    }

    // Delete all
    for idx in 0..10 {
        client
            .delete_resource_policy()
            .policy_name(format!("test_policy_{idx}"))
            .send()
            .await
            .unwrap();
    }

    // Verify empty
    let resp = client.describe_resource_policies().send().await.unwrap();
    assert_eq!(resp.resource_policies().len(), 0);

    // Delete non-existent
    let err = client
        .delete_resource_policy()
        .policy_name("non-existent")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected not found, got: {err_str}"
    );
}

// Ported from moto: test_logs.py::test_describe_resource_policies
#[tokio::test]
async fn test_moto_describe_resource_policies() {
    let client = make_logs_client().await;

    let policy_doc = r#"{"Version":"2012-10-17"}"#;

    for idx in 0..10 {
        client
            .put_resource_policy()
            .policy_name(format!("test_policy_{idx}"))
            .policy_document(policy_doc)
            .send()
            .await
            .unwrap();
    }

    let resp = client.describe_resource_policies().send().await.unwrap();
    assert_eq!(resp.resource_policies().len(), 10);

    for policy in resp.resource_policies() {
        assert!(policy.policy_name().is_some());
        assert_eq!(policy.policy_document(), Some(policy_doc));
        assert!(policy.last_updated_time().is_some());
    }
}

// Ported from moto: test_logs.py::test_list_tags_log_group
#[tokio::test]
async fn test_moto_list_tags_log_group() {
    let client = make_logs_client().await;

    // Create without tags - should return empty tags
    client
        .create_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert!(tags.is_empty());

    // Delete and recreate with tags
    client
        .delete_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();

    client
        .create_log_group()
        .log_group_name("dummy")
        .tags("tag_key_1", "tag_value_1")
        .tags("tag_key_2", "tag_value_2")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(
        tags.get("tag_key_1").map(|s| s.as_str()),
        Some("tag_value_1")
    );
    assert_eq!(
        tags.get("tag_key_2").map(|s| s.as_str()),
        Some("tag_value_2")
    );
}

// Ported from moto: test_logs.py::test_tag_log_group
#[tokio::test]
async fn test_moto_tag_log_group() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    client
        .tag_log_group()
        .log_group_name("dummy")
        .tags("tag_key_1", "tag_value_1")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(
        tags.get("tag_key_1").map(|s| s.as_str()),
        Some("tag_value_1")
    );

    // Add another tag
    #[allow(deprecated)]
    client
        .tag_log_group()
        .log_group_name("dummy")
        .tags("tag_key_2", "tag_value_2")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(
        tags.get("tag_key_1").map(|s| s.as_str()),
        Some("tag_value_1")
    );
    assert_eq!(
        tags.get("tag_key_2").map(|s| s.as_str()),
        Some("tag_value_2")
    );

    // Update existing tag
    #[allow(deprecated)]
    client
        .tag_log_group()
        .log_group_name("dummy")
        .tags("tag_key_1", "tag_value_XX")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(
        tags.get("tag_key_1").map(|s| s.as_str()),
        Some("tag_value_XX")
    );
    assert_eq!(
        tags.get("tag_key_2").map(|s| s.as_str()),
        Some("tag_value_2")
    );
}

// Ported from moto: test_logs.py::test_untag_log_group
#[tokio::test]
async fn test_moto_untag_log_group() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    client
        .tag_log_group()
        .log_group_name("dummy")
        .tags("tag_key_1", "tag_value_1")
        .tags("tag_key_2", "tag_value_2")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 2);

    #[allow(deprecated)]
    client
        .untag_log_group()
        .log_group_name("dummy")
        .tags("tag_key_1")
        .send()
        .await
        .unwrap();

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(
        tags.get("tag_key_2").map(|s| s.as_str()),
        Some("tag_value_2")
    );
    assert!(tags.get("tag_key_1").is_none());
}

// Ported from moto: test_logs.py::test_describe_subscription_filters
#[tokio::test]
async fn test_moto_describe_subscription_filters_empty() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_subscription_filters()
        .log_group_name("/test")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.subscription_filters().len(), 0);
}

// Ported from moto: test_logs.py::test_describe_subscription_filters_errors
#[tokio::test]
async fn test_moto_describe_subscription_filters_errors() {
    let client = make_logs_client().await;

    let err = client
        .describe_subscription_filters()
        .log_group_name("not-existing-log-group")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_logs_tags.py::test_log_groups_tags
#[tokio::test]
async fn test_moto_log_groups_tags() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("test")
        .tags("key1", "val1")
        .send()
        .await
        .unwrap();

    let arn = client
        .describe_log_groups()
        .send()
        .await
        .unwrap()
        .log_groups()[0]
        .arn()
        .unwrap()
        .to_string();

    // Add tag via tag_resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = tags.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("val1"));
    assert_eq!(tags.get("key2").map(|s| s.as_str()), Some("val2"));

    // Remove tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key2")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = tags.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("val1"));
    assert!(tags.get("key2").is_none());
}

// Ported from moto: test_logs_tags.py::test_destination_tags
#[tokio::test]
async fn test_moto_destination_tags() {
    let client = make_logs_client().await;

    let destination_name = "test-destination";
    let role_arn = "arn:aws:iam::123456789012:role/my-subscription-role";
    let target_arn = "arn:aws:kinesis:us-east-1:123456789012:stream/my-kinesis-stream";

    let resp = client
        .put_destination()
        .destination_name(destination_name)
        .target_arn(target_arn)
        .role_arn(role_arn)
        .tags("key1", "val1")
        .send()
        .await
        .unwrap();

    let dest_arn = resp.destination().unwrap().arn().unwrap().to_string();

    // Add tag
    client
        .tag_resource()
        .resource_arn(&dest_arn)
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&dest_arn)
        .send()
        .await
        .unwrap();
    let tags = tags.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("val1"));
    assert_eq!(tags.get("key2").map(|s| s.as_str()), Some("val2"));

    // Remove tag
    client
        .untag_resource()
        .resource_arn(&dest_arn)
        .tag_keys("key2")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&dest_arn)
        .send()
        .await
        .unwrap();
    let tags = tags.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("val1"));
    assert!(tags.get("key2").is_none());
}

// Ported from moto: test_logs.py::test_describe_log_groups_paging
#[tokio::test]
async fn test_moto_describe_log_groups_paging() {
    let client = make_logs_client().await;

    let group_names = [
        "/aws/lambda/lowercase-dev",
        "/aws/lambda/FileMonitoring",
        "/aws/events/GetMetricData",
        "/aws/lambda/fileAvailable",
    ];

    for name in &group_names {
        client
            .create_log_group()
            .log_group_name(*name)
            .send()
            .await
            .unwrap();
    }

    let resp = client.describe_log_groups().send().await.unwrap();
    assert_eq!(resp.log_groups().len(), 4);
    assert!(resp.next_token().is_none());

    let resp = client.describe_log_groups().limit(2).send().await.unwrap();
    assert_eq!(resp.log_groups().len(), 2);
    assert!(resp.next_token().is_some());

    let resp = client
        .describe_log_groups()
        .next_token(resp.next_token().unwrap())
        .limit(1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_groups().len(), 1);
    assert!(resp.next_token().is_some());

    let resp = client
        .describe_log_groups()
        .next_token(resp.next_token().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_groups().len(), 1);
    assert!(resp.next_token().is_none());
}

// Ported from moto: test_logs.py::test_describe_log_streams_simple_paging
#[tokio::test]
async fn test_moto_describe_log_streams_simple_paging() {
    let client = make_logs_client().await;

    let group_name = "/aws/lambda/lowercase-dev";
    client
        .create_log_group()
        .log_group_name(group_name)
        .send()
        .await
        .unwrap();

    for i in 0..10 {
        client
            .create_log_stream()
            .log_group_name(group_name)
            .log_stream_name(format!("stream{i}"))
            .send()
            .await
            .unwrap();
    }

    // Get all 10
    let resp = client
        .describe_log_streams()
        .log_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams().len(), 10);
    assert!(resp.next_token().is_none());

    // Get first 4
    let resp = client
        .describe_log_streams()
        .log_group_name(group_name)
        .limit(4)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams().len(), 4);
    assert!(resp.next_token().is_some());

    // Get next 4
    let resp = client
        .describe_log_streams()
        .log_group_name(group_name)
        .limit(4)
        .next_token(resp.next_token().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams().len(), 4);
    assert!(resp.next_token().is_some());

    // Get remaining 2
    let resp = client
        .describe_log_streams()
        .log_group_name(group_name)
        .limit(4)
        .next_token(resp.next_token().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.log_streams().len(), 2);
    assert!(resp.next_token().is_none());
}

// Ported from moto: test_logs.py::test_arn_formats_log_group_and_stream
#[tokio::test]
async fn test_moto_arn_formats_log_group_and_stream() {
    let client = make_logs_client().await;
    let log_group_name = "test-arn-formats";

    client
        .create_log_group()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix(log_group_name)
        .send()
        .await
        .unwrap();
    let group = &resp.log_groups()[0];
    assert_eq!(group.log_group_name(), Some(log_group_name));

    // arn should have :* suffix
    let arn = group.arn().unwrap();
    assert!(arn.ends_with(":*"), "arn should end with :*, got: {arn}");
    assert!(arn.contains(log_group_name));

    // logGroupArn should NOT have :* suffix
    let log_group_arn = group.log_group_arn().unwrap();
    assert!(
        !log_group_arn.ends_with(":*"),
        "logGroupArn should not end with :*, got: {log_group_arn}"
    );
    assert!(log_group_arn.contains(log_group_name));

    // Create a stream and check its ARN
    client
        .create_log_stream()
        .log_group_name(log_group_name)
        .log_stream_name("stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_streams()
        .log_group_name(log_group_name)
        .send()
        .await
        .unwrap();
    let stream = &resp.log_streams()[0];
    assert_eq!(stream.log_stream_name(), Some("stream"));

    let stream_arn = stream.arn().unwrap();
    assert!(stream_arn.contains(log_group_name));
    assert!(stream_arn.contains(":log-stream:stream"));
}

// Ported from moto: test_export_tasks.py::test_describe_export_tasks_happy_path
#[tokio::test]
async fn test_moto_describe_export_tasks() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/export")
        .send()
        .await
        .unwrap();

    let from_time = 1611316574_i64;
    let to_time = 1642852574_i64;

    let resp = client
        .create_export_task()
        .log_group_name("/test/export")
        .destination("mybucket")
        .from(from_time)
        .to(to_time)
        .send()
        .await
        .unwrap();

    let task_id = resp.task_id().unwrap().to_string();

    let resp = client.describe_export_tasks().send().await.unwrap();
    assert_eq!(resp.export_tasks().len(), 1);

    let task = &resp.export_tasks()[0];
    assert_eq!(task.task_id(), Some(task_id.as_str()));
    assert_eq!(task.log_group_name(), Some("/test/export"));
    assert_eq!(task.destination(), Some("mybucket"));
    assert_eq!(task.from(), Some(from_time));
    assert_eq!(task.to(), Some(to_time));
    assert!(task.status().is_some());
}

// Ported from moto: test_logs_filter_log_events.py::TestLogFilterParameters::test_filter_logs_interleaved
#[tokio::test]
#[allow(deprecated)]
async fn test_moto_filter_logs_interleaved() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("dummy")
        .log_stream_name("stream")
        .send()
        .await
        .unwrap();

    let ts = chrono::Utc::now().timestamp_millis();

    client
        .put_log_events()
        .log_group_name("dummy")
        .log_stream_name("stream")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(ts)
                .message("hello")
                .build()
                .unwrap(),
        )
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(ts)
                .message("world")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .filter_log_events()
        .log_group_name("dummy")
        .log_stream_names("stream")
        .interleaved(true)
        .send()
        .await
        .unwrap();

    let events = resp.events();
    assert_eq!(events.len(), 2);
    for event in events {
        assert!(event.event_id().is_some());
        assert_eq!(event.timestamp(), Some(ts));
    }
    // Verify messages are present (order may vary for same timestamp)
    let messages: Vec<&str> = events.iter().map(|e| e.message().unwrap()).collect();
    assert!(messages.contains(&"hello"));
    assert!(messages.contains(&"world"));
}

// Ported from moto: test_logs_filter_log_events.py::TestLogsFilterPattern::test_simple_word_pattern
#[tokio::test]
async fn test_moto_filter_simple_word_pattern() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("dummy")
        .log_stream_name("stream")
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();
    let messages_data = [
        "hello",
        "world",
        "hello world",
        "goodbye world",
        "hello cruela",
        "goodbye cruel world",
    ];

    let mut builder = client
        .put_log_events()
        .log_group_name("dummy")
        .log_stream_name("stream");

    for msg in &messages_data {
        builder = builder.log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message(*msg)
                .build()
                .unwrap(),
        );
    }
    builder.send().await.unwrap();

    // Filter for "hello"
    let resp = client
        .filter_log_events()
        .log_group_name("dummy")
        .log_stream_names("stream")
        .filter_pattern("hello")
        .send()
        .await
        .unwrap();

    let messages: std::collections::HashSet<&str> =
        resp.events().iter().map(|e| e.message().unwrap()).collect();
    assert_eq!(messages.len(), 3);
    assert!(messages.contains("hello"));
    assert!(messages.contains("hello world"));
    assert!(messages.contains("hello cruela"));
}

// Ported from moto: test_logs_filter_log_events.py::TestLogsFilterPattern::test_multiple_words_pattern
#[tokio::test]
async fn test_moto_filter_multiple_words_pattern() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("dummy")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("dummy")
        .log_stream_name("stream")
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();
    let messages_data = [
        "hello",
        "world",
        "hello world",
        "goodbye world",
        "hello cruela",
        "goodbye cruel world",
    ];

    let mut builder = client
        .put_log_events()
        .log_group_name("dummy")
        .log_stream_name("stream");

    for msg in &messages_data {
        builder = builder.log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message(*msg)
                .build()
                .unwrap(),
        );
    }
    builder.send().await.unwrap();

    // Filter for "goodbye world" (both words must appear)
    let resp = client
        .filter_log_events()
        .log_group_name("dummy")
        .log_stream_names("stream")
        .filter_pattern("goodbye world")
        .send()
        .await
        .unwrap();

    let messages: std::collections::HashSet<&str> =
        resp.events().iter().map(|e| e.message().unwrap()).collect();
    assert!(messages.contains("goodbye world"));
    assert!(messages.contains("goodbye cruel world"));
    assert_eq!(messages.len(), 2);
}

// Ported from moto: test_logs_metric_filters.py::test_describe_metric_filters_happy_prefix
#[tokio::test]
async fn test_moto_describe_metric_filters_prefix() {
    let client = make_logs_client().await;

    // Create two log groups with metric filters
    for i in 1..=2 {
        let group_name = format!("logGroupName{i}");
        client
            .create_log_group()
            .log_group_name(&group_name)
            .send()
            .await
            .unwrap();

        client
            .put_metric_filter()
            .log_group_name(&group_name)
            .filter_name(format!("filterName{i}"))
            .filter_pattern(format!("filterPattern{i}"))
            .metric_transformations(
                aws_sdk_cloudwatchlogs::types::MetricTransformation::builder()
                    .metric_namespace(format!("metricNamespace{i}"))
                    .metric_name(format!("metricName{i}"))
                    .metric_value(format!("metricValue{i}"))
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    // Filter by prefix
    let resp = client
        .describe_metric_filters()
        .filter_name_prefix("filter")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.metric_filters().len(), 2);

    // Filter by log group name
    let resp = client
        .describe_metric_filters()
        .log_group_name("logGroupName2")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.metric_filters().len(), 1);
    assert_eq!(
        resp.metric_filters()[0].log_group_name(),
        Some("logGroupName2")
    );

    // Filter by metric name + namespace
    let resp = client
        .describe_metric_filters()
        .metric_name("metricName1")
        .metric_namespace("metricNamespace1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.metric_filters().len(), 1);
}

// Ported from moto: test_export_tasks.py::test_cancel_unknown_export_task
#[tokio::test]
async fn test_moto_cancel_unknown_export_task() {
    let client = make_logs_client().await;

    let err = client
        .cancel_export_task()
        .task_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_export_tasks.py::test_describe_export_tasks_raises_ResourceNotFoundException_task_id_not_found
#[tokio::test]
async fn test_moto_describe_export_tasks_not_found() {
    let client = make_logs_client().await;

    let err = client
        .describe_export_tasks()
        .task_id("nonexistent-task-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

// ========== Prefix Filtering ==========

#[tokio::test]
async fn test_describe_log_groups_with_prefix() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/app/frontend")
        .send()
        .await
        .unwrap();

    client
        .create_log_group()
        .log_group_name("/app/backend")
        .send()
        .await
        .unwrap();

    client
        .create_log_group()
        .log_group_name("/infra/monitoring")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix("/app/")
        .send()
        .await
        .expect("describe_log_groups with prefix should succeed");

    let groups = resp.log_groups();
    assert_eq!(groups.len(), 2);
    for g in groups {
        assert!(g.log_group_name().unwrap().starts_with("/app/"));
    }
}

#[tokio::test]
async fn test_describe_log_streams_with_prefix() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/stream-prefix")
        .send()
        .await
        .unwrap();

    for name in &["alpha-1", "alpha-2", "beta-1"] {
        client
            .create_log_stream()
            .log_group_name("/test/stream-prefix")
            .log_stream_name(*name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_log_streams()
        .log_group_name("/test/stream-prefix")
        .log_stream_name_prefix("alpha")
        .send()
        .await
        .expect("describe_log_streams with prefix should succeed");

    let streams = resp.log_streams();
    assert_eq!(streams.len(), 2);
    for s in streams {
        assert!(s.log_stream_name().unwrap().starts_with("alpha"));
    }
}

// ========== Pagination ==========

#[tokio::test]
async fn test_describe_log_groups_pagination() {
    let client = make_logs_client().await;

    for i in 0..5 {
        client
            .create_log_group()
            .log_group_name(format!("/paged/group-{i:02}"))
            .send()
            .await
            .unwrap();
    }

    let page1 = client
        .describe_log_groups()
        .limit(2)
        .send()
        .await
        .expect("first page should succeed");

    assert_eq!(page1.log_groups().len(), 2);
    let token = page1.next_token().expect("should have next_token");

    let page2 = client
        .describe_log_groups()
        .limit(2)
        .next_token(token)
        .send()
        .await
        .expect("second page should succeed");

    assert_eq!(page2.log_groups().len(), 2);
    let token2 = page2
        .next_token()
        .expect("should have next_token for page 2");

    let page3 = client
        .describe_log_groups()
        .limit(2)
        .next_token(token2)
        .send()
        .await
        .expect("third page should succeed");

    assert_eq!(page3.log_groups().len(), 1);
    assert!(page3.next_token().is_none());
}

#[tokio::test]
async fn test_describe_log_streams_pagination() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/stream-paging")
        .send()
        .await
        .unwrap();

    for i in 0..4 {
        client
            .create_log_stream()
            .log_group_name("/test/stream-paging")
            .log_stream_name(format!("stream-{i:02}"))
            .send()
            .await
            .unwrap();
    }

    let page1 = client
        .describe_log_streams()
        .log_group_name("/test/stream-paging")
        .limit(2)
        .send()
        .await
        .expect("first page of streams should succeed");

    assert_eq!(page1.log_streams().len(), 2);
    let token = page1.next_token().expect("should have next_token");

    let page2 = client
        .describe_log_streams()
        .log_group_name("/test/stream-paging")
        .limit(2)
        .next_token(token)
        .send()
        .await
        .expect("second page of streams should succeed");

    assert_eq!(page2.log_streams().len(), 2);
    assert!(page2.next_token().is_none());
}

// ========== Time-range filtering for GetLogEvents ==========

#[tokio::test]
async fn test_get_log_events_with_time_range() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/time-range")
        .send()
        .await
        .unwrap();
    client
        .create_log_stream()
        .log_group_name("/test/time-range")
        .log_stream_name("s1")
        .send()
        .await
        .unwrap();

    let base = 1_700_000_000_000_i64;

    client
        .put_log_events()
        .log_group_name("/test/time-range")
        .log_stream_name("s1")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(base)
                .message("early")
                .build()
                .unwrap(),
        )
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(base + 5_000)
                .message("middle")
                .build()
                .unwrap(),
        )
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(base + 10_000)
                .message("late")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_log_events()
        .log_group_name("/test/time-range")
        .log_stream_name("s1")
        .start_time(base + 1_000)
        .end_time(base + 9_000)
        .send()
        .await
        .expect("get_log_events with time range should succeed");

    let events = resp.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].message(), Some("middle"));
}

// ========== Filter across multiple streams ==========

#[tokio::test]
async fn test_filter_log_events_across_streams() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/multi-stream-filter")
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();

    for (stream, msg) in &[("s1", "warn: disk almost full"), ("s2", "info: ok")] {
        client
            .create_log_stream()
            .log_group_name("/test/multi-stream-filter")
            .log_stream_name(*stream)
            .send()
            .await
            .unwrap();

        client
            .put_log_events()
            .log_group_name("/test/multi-stream-filter")
            .log_stream_name(*stream)
            .log_events(
                aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                    .timestamp(now)
                    .message(*msg)
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .filter_log_events()
        .log_group_name("/test/multi-stream-filter")
        .filter_pattern("warn")
        .send()
        .await
        .expect("filter_log_events across streams should succeed");

    let events = resp.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].message(), Some("warn: disk almost full"));
    assert_eq!(events[0].log_stream_name(), Some("s1"));
}

// ========== Log group created with tags ==========

#[tokio::test]
async fn test_create_log_group_with_tags() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/tagged-group")
        .tags("environment", "test")
        .tags("project", "winterbaume")
        .send()
        .await
        .expect("create_log_group with tags should succeed");

    #[allow(deprecated)]
    let resp = client
        .list_tags_log_group()
        .log_group_name("/test/tagged-group")
        .send()
        .await
        .expect("list_tags_log_group should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("environment").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("winterbaume"));
}

// ========== Error: put events to nonexistent group ==========

#[tokio::test]
async fn test_put_log_events_nonexistent_group() {
    let client = make_logs_client().await;

    let now = chrono::Utc::now().timestamp_millis();

    let result = client
        .put_log_events()
        .log_group_name("/does/not/exist")
        .log_stream_name("s1")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message("hello")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

// ========== Error: put events to nonexistent stream ==========

#[tokio::test]
async fn test_put_log_events_nonexistent_stream() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/no-stream")
        .send()
        .await
        .unwrap();

    let now = chrono::Utc::now().timestamp_millis();

    let result = client
        .put_log_events()
        .log_group_name("/test/no-stream")
        .log_stream_name("nonexistent-stream")
        .log_events(
            aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .timestamp(now)
                .message("hello")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

// ========== Error: delete nonexistent log stream ==========

#[tokio::test]
async fn test_delete_nonexistent_log_stream() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/del-stream-err")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_log_stream()
        .log_group_name("/test/del-stream-err")
        .log_stream_name("phantom")
        .send()
        .await;

    assert!(result.is_err());
}

// ========== Error: delete nonexistent log group ==========

#[tokio::test]
async fn test_delete_nonexistent_log_group() {
    let client = make_logs_client().await;

    let result = client
        .delete_log_group()
        .log_group_name("/does/not/exist")
        .send()
        .await;

    assert!(result.is_err());
}

// ========== Metric filter describe by name prefix ==========

#[tokio::test]
async fn test_describe_metric_filters_by_name_prefix() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/mf-prefix")
        .send()
        .await
        .unwrap();

    for name in &["error-filter", "error-count", "latency-filter"] {
        client
            .put_metric_filter()
            .log_group_name("/test/mf-prefix")
            .filter_name(*name)
            .filter_pattern("")
            .metric_transformations(
                aws_sdk_cloudwatchlogs::types::MetricTransformation::builder()
                    .metric_namespace("NS")
                    .metric_name("M")
                    .metric_value("1")
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_metric_filters()
        .log_group_name("/test/mf-prefix")
        .filter_name_prefix("error")
        .send()
        .await
        .expect("describe_metric_filters with prefix should succeed");

    let filters = resp.metric_filters();
    assert_eq!(filters.len(), 2);
    for f in filters {
        assert!(f.filter_name().unwrap().starts_with("error"));
    }
}

// ========== Duplicate log stream fails ==========

#[tokio::test]
async fn test_duplicate_log_stream_fails() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/dup-stream")
        .send()
        .await
        .unwrap();

    client
        .create_log_stream()
        .log_group_name("/test/dup-stream")
        .log_stream_name("dup")
        .send()
        .await
        .unwrap();

    let result = client
        .create_log_stream()
        .log_group_name("/test/dup-stream")
        .log_stream_name("dup")
        .send()
        .await;

    assert!(result.is_err());
}

// ========== Log stream on nonexistent group fails ==========

#[tokio::test]
async fn test_create_log_stream_on_nonexistent_group() {
    let client = make_logs_client().await;

    let result = client
        .create_log_stream()
        .log_group_name("/nonexistent/group")
        .log_stream_name("s1")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): ARN normalization in tag_resource — terraform sends
// ARNs without the trailing `:*` suffix but winterbaume stores them with it.
#[tokio::test]
async fn test_tag_resource_with_arn_without_star_suffix() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/tag-no-star")
        .send()
        .await
        .unwrap();

    // Get the ARN from describe (it will have the `:*` suffix)
    let resp = client
        .describe_log_groups()
        .log_group_name_prefix("/test/tag-no-star")
        .send()
        .await
        .unwrap();
    let full_arn = resp.log_groups()[0].arn().unwrap().to_string();
    assert!(full_arn.ends_with(":*"), "stored ARN should end with :*");

    // Strip the trailing `:*` to simulate terraform behavior
    let arn_without_star = full_arn.strip_suffix(":*").unwrap();

    // Tag using the ARN without `:*`
    client
        .tag_resource()
        .resource_arn(arn_without_star)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource with ARN without :* should succeed");

    // List tags using the ARN without `:*`
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn_without_star)
        .send()
        .await
        .expect("list_tags_for_resource with ARN without :* should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

// Covers FIX(terraform-e2e): ARN normalization in untag_resource
#[tokio::test]
async fn test_untag_resource_with_arn_without_star_suffix() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/untag-no-star")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix("/test/untag-no-star")
        .send()
        .await
        .unwrap();
    let full_arn = resp.log_groups()[0].arn().unwrap().to_string();
    let arn_without_star = full_arn.strip_suffix(":*").unwrap();

    // Tag using the full ARN (with :*)
    client
        .tag_resource()
        .resource_arn(&full_arn)
        .tags("env", "staging")
        .tags("team", "backend")
        .send()
        .await
        .unwrap();

    // Untag using the ARN without `:*`
    client
        .untag_resource()
        .resource_arn(arn_without_star)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource with ARN without :* should succeed");

    // Verify the tag was removed
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn_without_star)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("staging"));
    assert!(
        tags.get("team").is_none(),
        "team tag should have been removed"
    );
}

// Covers FIX(terraform-e2e): ARN normalization in list_tags_for_resource —
// tags added with full ARN (with :*) should be retrievable with ARN without :*
#[tokio::test]
async fn test_list_tags_for_resource_with_arn_without_star_suffix() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/list-tags-no-star")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix("/test/list-tags-no-star")
        .send()
        .await
        .unwrap();
    let full_arn = resp.log_groups()[0].arn().unwrap().to_string();
    let arn_without_star = full_arn.strip_suffix(":*").unwrap();

    // Tag using the full ARN (with :*)
    client
        .tag_resource()
        .resource_arn(&full_arn)
        .tags("service", "logs")
        .send()
        .await
        .unwrap();

    // List tags using ARN without `:*`
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn_without_star)
        .send()
        .await
        .expect("list_tags_for_resource with ARN without :* should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("service").map(|s| s.as_str()), Some("logs"));

    // Also verify listing with the full ARN still works
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&full_arn)
        .send()
        .await
        .expect("list_tags_for_resource with full ARN should also succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("service").map(|s| s.as_str()), Some("logs"));
}

// Covers FIX(terraform-e2e): ARN normalization — tags added via both ARN forms
// should merge into the same tag set
#[tokio::test]
async fn test_tag_resource_mixed_arn_forms() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/mixed-arn-tags")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_log_groups()
        .log_group_name_prefix("/test/mixed-arn-tags")
        .send()
        .await
        .unwrap();
    let full_arn = resp.log_groups()[0].arn().unwrap().to_string();
    let arn_without_star = full_arn.strip_suffix(":*").unwrap();

    // Tag with the full ARN (with :*)
    client
        .tag_resource()
        .resource_arn(&full_arn)
        .tags("key1", "value1")
        .send()
        .await
        .unwrap();

    // Tag with the ARN without :*
    client
        .tag_resource()
        .resource_arn(arn_without_star)
        .tags("key2", "value2")
        .send()
        .await
        .unwrap();

    // Both tags should be present
    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn_without_star)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("value1"));
    assert_eq!(tags.get("key2").map(|s| s.as_str()), Some("value2"));
}

// Covers FIX(terraform-e2e): tag_resource with nonexistent ARN returns error
#[tokio::test]
async fn test_tag_resource_not_found_arn() {
    let client = make_logs_client().await;

    let result = client
        .tag_resource()
        .resource_arn("arn:aws:logs:us-east-1:123456789012:log-group:/nonexistent/group")
        .tags("key", "value")
        .send()
        .await;

    assert!(
        result.is_err(),
        "tagging a nonexistent resource should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ========== Account Policies ==========

#[tokio::test]
async fn test_account_policy_lifecycle() {
    let client = make_logs_client().await;

    let resp = client
        .put_account_policy()
        .policy_name("test-account-policy")
        .policy_document("{\"Name\":\"test\"}")
        .policy_type(aws_sdk_cloudwatchlogs::types::PolicyType::DataProtectionPolicy)
        .send()
        .await
        .expect("put_account_policy should succeed");

    let policy = resp.account_policy().unwrap();
    assert_eq!(policy.policy_name(), Some("test-account-policy"));
    assert_eq!(policy.policy_document(), Some("{\"Name\":\"test\"}"));

    let resp = client
        .describe_account_policies()
        .policy_type(aws_sdk_cloudwatchlogs::types::PolicyType::DataProtectionPolicy)
        .send()
        .await
        .expect("describe_account_policies should succeed");

    let policies = resp.account_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].policy_name(), Some("test-account-policy"));

    client
        .delete_account_policy()
        .policy_name("test-account-policy")
        .policy_type(aws_sdk_cloudwatchlogs::types::PolicyType::DataProtectionPolicy)
        .send()
        .await
        .expect("delete_account_policy should succeed");

    let resp = client
        .describe_account_policies()
        .policy_type(aws_sdk_cloudwatchlogs::types::PolicyType::DataProtectionPolicy)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.account_policies().len(), 0);
}

// ========== Data Protection Policies ==========

#[tokio::test]
async fn test_data_protection_policy_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/data-protection")
        .send()
        .await
        .unwrap();

    let resp = client
        .put_data_protection_policy()
        .log_group_identifier("/test/data-protection")
        .policy_document("{\"Name\":\"dp-policy\"}")
        .send()
        .await
        .expect("put_data_protection_policy should succeed");

    assert_eq!(resp.log_group_identifier(), Some("/test/data-protection"));
    assert_eq!(resp.policy_document(), Some("{\"Name\":\"dp-policy\"}"));

    let resp = client
        .get_data_protection_policy()
        .log_group_identifier("/test/data-protection")
        .send()
        .await
        .expect("get_data_protection_policy should succeed");

    assert_eq!(resp.log_group_identifier(), Some("/test/data-protection"));
    assert_eq!(resp.policy_document(), Some("{\"Name\":\"dp-policy\"}"));

    client
        .delete_data_protection_policy()
        .log_group_identifier("/test/data-protection")
        .send()
        .await
        .expect("delete_data_protection_policy should succeed");

    let resp = client
        .get_data_protection_policy()
        .log_group_identifier("/test/data-protection")
        .send()
        .await
        .unwrap();

    assert!(
        resp.policy_document().is_none(),
        "policy_document should be None after deletion"
    );
}

// ========== Index Policies ==========

#[tokio::test]
async fn test_index_policy_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/index-policy")
        .send()
        .await
        .unwrap();

    let resp = client
        .put_index_policy()
        .log_group_identifier("/test/index-policy")
        .policy_document("{\"Fields\":[\"requestId\"]}")
        .send()
        .await
        .expect("put_index_policy should succeed");

    let policy = resp.index_policy().unwrap();
    assert_eq!(policy.log_group_identifier(), Some("/test/index-policy"));

    let resp = client
        .describe_index_policies()
        .log_group_identifiers("/test/index-policy")
        .send()
        .await
        .expect("describe_index_policies should succeed");

    let policies = resp.index_policies();
    assert_eq!(policies.len(), 1);

    client
        .delete_index_policy()
        .log_group_identifier("/test/index-policy")
        .send()
        .await
        .expect("delete_index_policy should succeed");

    let resp = client
        .describe_index_policies()
        .log_group_identifiers("/test/index-policy")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.index_policies().len(), 0);
}

// ========== Query Definitions ==========

#[tokio::test]
async fn test_query_definition_lifecycle() {
    let client = make_logs_client().await;

    let resp = client
        .put_query_definition()
        .name("my-query")
        .query_string("fields @timestamp, @message | sort @timestamp desc")
        .send()
        .await
        .expect("put_query_definition should succeed");

    let qd_id = resp.query_definition_id().unwrap().to_string();
    assert!(!qd_id.is_empty());

    let resp = client
        .describe_query_definitions()
        .send()
        .await
        .expect("describe_query_definitions should succeed");

    let defs = resp.query_definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].name(), Some("my-query"));
    assert_eq!(defs[0].query_definition_id(), Some(qd_id.as_str()));

    let resp = client
        .delete_query_definition()
        .query_definition_id(&qd_id)
        .send()
        .await
        .expect("delete_query_definition should succeed");

    assert!(
        resp.success(),
        "delete_query_definition should return success=true"
    );

    let resp = client.describe_query_definitions().send().await.unwrap();
    assert_eq!(resp.query_definitions().len(), 0);
}

// ========== Log Anomaly Detectors ==========

#[tokio::test]
async fn test_log_anomaly_detector_lifecycle() {
    let client = make_logs_client().await;

    let resp = client
        .create_log_anomaly_detector()
        .log_group_arn_list("arn:aws:logs:us-east-1:123456789012:log-group:/test/anomaly:*")
        .detector_name("test-detector")
        .send()
        .await
        .expect("create_log_anomaly_detector should succeed");

    let arn = resp.anomaly_detector_arn().unwrap().to_string();
    assert!(arn.contains("anomaly-detector"));

    let resp = client
        .get_log_anomaly_detector()
        .anomaly_detector_arn(&arn)
        .send()
        .await
        .expect("get_log_anomaly_detector should succeed");

    assert_eq!(resp.detector_name(), Some("test-detector"));

    let resp = client
        .list_log_anomaly_detectors()
        .send()
        .await
        .expect("list_log_anomaly_detectors should succeed");

    assert_eq!(resp.anomaly_detectors().len(), 1);

    client
        .update_log_anomaly_detector()
        .anomaly_detector_arn(&arn)
        .enabled(false)
        .send()
        .await
        .expect("update_log_anomaly_detector should succeed");

    client
        .delete_log_anomaly_detector()
        .anomaly_detector_arn(&arn)
        .send()
        .await
        .expect("delete_log_anomaly_detector should succeed");

    let resp = client.list_log_anomaly_detectors().send().await.unwrap();
    assert_eq!(resp.anomaly_detectors().len(), 0);
}

// ========== Integrations ==========

#[tokio::test]
async fn test_integration_lifecycle() {
    let client = make_logs_client().await;

    let resp = client
        .put_integration()
        .integration_name("test-integration")
        .integration_type(aws_sdk_cloudwatchlogs::types::IntegrationType::Opensearch)
        .resource_config(
            aws_sdk_cloudwatchlogs::types::ResourceConfig::OpenSearchResourceConfig(
                aws_sdk_cloudwatchlogs::types::OpenSearchResourceConfig::builder()
                    .kms_key_arn("arn:aws:kms:us-east-1:123456789012:key/test-key")
                    .data_source_role_arn("arn:aws:iam::123456789012:role/test-role")
                    .dashboard_viewer_principals("arn:aws:iam::123456789012:root")
                    .retention_days(30)
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .expect("put_integration should succeed");

    assert_eq!(resp.integration_name(), Some("test-integration"));

    let resp = client
        .get_integration()
        .integration_name("test-integration")
        .send()
        .await
        .expect("get_integration should succeed");

    assert_eq!(resp.integration_name(), Some("test-integration"));

    let resp = client
        .list_integrations()
        .send()
        .await
        .expect("list_integrations should succeed");

    assert_eq!(resp.integration_summaries().len(), 1);

    client
        .delete_integration()
        .integration_name("test-integration")
        .force(true)
        .send()
        .await
        .expect("delete_integration should succeed");

    let resp = client.list_integrations().send().await.unwrap();
    assert_eq!(resp.integration_summaries().len(), 0);
}

// ========== Transformers ==========

#[tokio::test]
async fn test_transformer_lifecycle() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/transformer")
        .send()
        .await
        .unwrap();

    client
        .put_transformer()
        .log_group_identifier("/test/transformer")
        .transformer_config(
            aws_sdk_cloudwatchlogs::types::Processor::builder()
                .parse_json(
                    aws_sdk_cloudwatchlogs::types::ParseJson::builder()
                        .source("@message")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_transformer should succeed");

    let resp = client
        .get_transformer()
        .log_group_identifier("/test/transformer")
        .send()
        .await
        .expect("get_transformer should succeed");

    assert_eq!(resp.log_group_identifier(), Some("/test/transformer"));

    client
        .delete_transformer()
        .log_group_identifier("/test/transformer")
        .send()
        .await
        .expect("delete_transformer should succeed");

    let result = client
        .get_transformer()
        .log_group_identifier("/test/transformer")
        .send()
        .await;
    assert!(result.is_err(), "get_transformer after delete should fail");
}

// ========== List Anomalies (empty) ==========

#[tokio::test]
async fn test_list_anomalies_empty() {
    let client = make_logs_client().await;

    let resp = client
        .list_anomalies()
        .send()
        .await
        .expect("list_anomalies should succeed");

    assert_eq!(resp.anomalies().len(), 0);
}

// ========== KMS Key Association ==========

#[tokio::test]
async fn test_kms_key_association() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/kms")
        .send()
        .await
        .unwrap();

    client
        .associate_kms_key()
        .log_group_name("/test/kms")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/test-key-id")
        .send()
        .await
        .expect("associate_kms_key should succeed");

    // Disassociate should also succeed
    client
        .disassociate_kms_key()
        .log_group_name("/test/kms")
        .send()
        .await
        .expect("disassociate_kms_key should succeed");

    // Disassociate on nonexistent group should fail
    let result = client
        .disassociate_kms_key()
        .log_group_name("/nonexistent/group")
        .send()
        .await;
    assert!(
        result.is_err(),
        "disassociate on nonexistent group should fail"
    );
}

// ========== Stop Query ==========

#[tokio::test]
async fn test_stop_query() {
    let client = make_logs_client().await;

    client
        .create_log_group()
        .log_group_name("/test/stop-query")
        .send()
        .await
        .unwrap();

    let resp = client
        .start_query()
        .log_group_name("/test/stop-query")
        .query_string("fields @timestamp")
        .start_time(0)
        .end_time(9999999999)
        .send()
        .await
        .expect("start_query should succeed");

    let query_id = resp.query_id().unwrap().to_string();

    // In the mock, queries complete immediately, so stop returns success=false
    // (already completed). The important thing is the call succeeds without error.
    let _resp = client
        .stop_query()
        .query_id(&query_id)
        .send()
        .await
        .expect("stop_query should succeed");

    // Stopping a nonexistent query should fail
    let result = client
        .stop_query()
        .query_id("nonexistent-query-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "stop_query on nonexistent query should fail"
    );
}

// ========== Test Metric Filter ==========

#[tokio::test]
async fn test_test_metric_filter() {
    let client = make_logs_client().await;

    let resp = client
        .test_metric_filter()
        .filter_pattern("[ip, id, user, timestamp, request, status_code, size]")
        .log_event_messages("127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] \"GET /apache_pb.gif HTTP/1.0\" 200 2326")
        .send()
        .await
        .expect("test_metric_filter should succeed");

    // The mock returns an empty matches list but the call should succeed
    let _ = resp.matches();
}

// ========== Update Delivery Configuration ==========

#[tokio::test]
async fn test_update_delivery_configuration() {
    let client = make_logs_client().await;

    // First create the delivery pipeline
    client
        .put_delivery_source()
        .name("update-cfg-source")
        .log_type("APPLICATION_LOGS")
        .resource_arn("arn:aws:logs:us-east-1:123456789012:log-group:/test/update-cfg")
        .send()
        .await
        .unwrap();

    client
        .put_delivery_destination()
        .name("update-cfg-dest")
        .delivery_destination_configuration(
            aws_sdk_cloudwatchlogs::types::DeliveryDestinationConfiguration::builder()
                .destination_resource_arn("arn:aws:s3:::my-delivery-bucket")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let delivery = client
        .create_delivery()
        .delivery_source_name("update-cfg-source")
        .delivery_destination_arn("update-cfg-dest")
        .send()
        .await
        .unwrap();

    let delivery_id = delivery.delivery().unwrap().id().unwrap().to_string();

    // Now update the delivery configuration
    client
        .update_delivery_configuration()
        .id(&delivery_id)
        .field_delimiter(",")
        .send()
        .await
        .expect("update_delivery_configuration should succeed");
}

// ========== Query Definitions with name prefix filter ==========

#[tokio::test]
async fn test_describe_query_definitions_with_prefix() {
    let client = make_logs_client().await;

    client
        .put_query_definition()
        .name("prod-errors")
        .query_string("fields @timestamp | filter @message like /ERROR/")
        .send()
        .await
        .unwrap();

    client
        .put_query_definition()
        .name("prod-warnings")
        .query_string("fields @timestamp | filter @message like /WARN/")
        .send()
        .await
        .unwrap();

    client
        .put_query_definition()
        .name("dev-debug")
        .query_string("fields @timestamp | filter @message like /DEBUG/")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_query_definitions()
        .query_definition_name_prefix("prod-")
        .send()
        .await
        .expect("describe_query_definitions with prefix should succeed");

    assert_eq!(resp.query_definitions().len(), 2);
}

// ========== Scheduled Queries ==========

#[tokio::test]
async fn test_scheduled_query_lifecycle() {
    let client = make_logs_client().await;

    let resp = client
        .create_scheduled_query()
        .name("my-scheduled-query")
        .query_string("fields @timestamp, @message | sort @timestamp desc | limit 100")
        .schedule_expression("rate(1 hour)")
        .log_group_identifiers("arn:aws:logs:us-east-1:123456789012:log-group:/test/scheduled:*")
        .send()
        .await
        .expect("create_scheduled_query should succeed");

    let arn = resp.scheduled_query_arn().unwrap().to_string();
    assert!(arn.contains("scheduled-query"));

    let resp = client
        .get_scheduled_query()
        .identifier(&arn)
        .send()
        .await
        .expect("get_scheduled_query should succeed");

    assert_eq!(resp.name(), Some("my-scheduled-query"));

    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");

    assert_eq!(resp.scheduled_queries().len(), 1);

    client
        .delete_scheduled_query()
        .identifier(&arn)
        .send()
        .await
        .expect("delete_scheduled_query should succeed");

    let resp = client.list_scheduled_queries().send().await.unwrap();
    assert_eq!(resp.scheduled_queries().len(), 0);
}

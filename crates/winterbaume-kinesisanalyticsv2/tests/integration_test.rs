use aws_sdk_kinesisanalyticsv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service;

async fn make_client() -> aws_sdk_kinesisanalyticsv2::Client {
    let mock = MockAws::builder()
        .with_service(KinesisAnalyticsV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisanalyticsv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_kinesisanalyticsv2::Client::new(&config)
}

#[tokio::test]
async fn test_create_application() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .application_name("test-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .expect("create_application should succeed");

    let detail = resp
        .application_detail()
        .expect("should have application detail");
    assert_eq!(detail.application_name(), "test-app");
    assert!(detail.application_arn().contains("test-app"));
}

#[tokio::test]
async fn test_describe_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("desc-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_application()
        .application_name("desc-app")
        .send()
        .await
        .expect("describe_application should succeed");

    let detail = resp
        .application_detail()
        .expect("should have application detail");
    assert_eq!(detail.application_name(), "desc-app");
    assert_eq!(
        detail.application_status(),
        &aws_sdk_kinesisanalyticsv2::types::ApplicationStatus::Ready,
    );
}

#[tokio::test]
async fn test_delete_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("del-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .delete_application()
        .application_name("del-app")
        .create_timestamp(aws_smithy_types::DateTime::from_secs(0))
        .send()
        .await
        .expect("delete_application should succeed");

    let result = client
        .describe_application()
        .application_name("del-app")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_applications() {
    let client = make_client().await;

    for name in ["app-a", "app-b", "app-c"] {
        client
            .create_application()
            .application_name(name)
            .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
            .service_execution_role("arn:aws:iam::123456789012:role/test-role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    assert_eq!(resp.application_summaries().len(), 3);
}

#[tokio::test]
async fn test_create_duplicate_application_fails() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("dup-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let result = client
        .create_application()
        .application_name("dup-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_application_fails() {
    let client = make_client().await;

    let result = client
        .delete_application()
        .application_name("nonexistent-app")
        .create_timestamp(aws_smithy_types::DateTime::from_secs(0))
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_application_with_description() {
    // Port of moto test_create_application - verify description and service execution role are stored
    let client = make_client().await;

    let resp = client
        .create_application()
        .application_name("test_application")
        .application_description("test application description")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink120)
        .service_execution_role("arn:aws:iam::123456789012:role/application_role")
        .send()
        .await
        .expect("create_application should succeed");

    let app = resp.application_detail().unwrap();
    assert!(app.application_arn().contains("test_application"));
    assert_eq!(
        app.application_description().unwrap_or(""),
        "test application description"
    );
    assert_eq!(app.runtime_environment().as_str(), "FLINK-1_20");
    assert_eq!(
        app.service_execution_role().unwrap_or(""),
        "arn:aws:iam::123456789012:role/application_role"
    );
}

#[tokio::test]
async fn test_describe_application_arn_format() {
    // Port of moto test_describe_application - verify ARN format
    let client = make_client().await;

    client
        .create_application()
        .application_name("arn-test-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink120)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_application()
        .application_name("arn-test-app")
        .send()
        .await
        .expect("describe_application should succeed");

    let app = resp.application_detail().unwrap();
    assert_eq!(
        app.application_arn(),
        "arn:aws:kinesisanalytics:us-east-1:123456789012:application/arn-test-app"
    );
    assert_eq!(app.runtime_environment().as_str(), "FLINK-1_20");
    assert_eq!(
        app.service_execution_role().unwrap_or(""),
        "arn:aws:iam::123456789012:role/test-role"
    );
}

#[tokio::test]
async fn test_list_applications_summary_fields() {
    // Port of moto test_list_applications - verify summary fields
    let client = make_client().await;

    client
        .create_application()
        .application_name("list-test-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink120)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    let summaries = resp.application_summaries();
    assert_eq!(summaries.len(), 1);
    let summary = &summaries[0];
    assert_eq!(summary.application_name(), "list-test-app");
    assert_eq!(
        summary.application_arn(),
        "arn:aws:kinesisanalytics:us-east-1:123456789012:application/list-test-app"
    );
    assert_eq!(summary.application_version_id(), 1);
    assert_eq!(summary.runtime_environment().as_str(), "FLINK-1_20");
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

#[tokio::test]
async fn test_create_application_sql_runtime() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .application_name("sql-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Sql10)
        .service_execution_role("arn:aws:iam::123456789012:role/sql-role")
        .send()
        .await
        .expect("create_application with SQL runtime should succeed");

    let detail = resp
        .application_detail()
        .expect("should have application detail");
    assert_eq!(detail.application_name(), "sql-app");
    assert_eq!(detail.runtime_environment().as_str(), "SQL-1_0");
    assert_eq!(
        detail.service_execution_role().unwrap_or(""),
        "arn:aws:iam::123456789012:role/sql-role"
    );
}

#[tokio::test]
async fn test_create_application_version_id_starts_at_one() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .application_name("version-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .expect("create_application should succeed");

    let detail = resp
        .application_detail()
        .expect("should have application detail");
    assert_eq!(detail.application_version_id(), 1);
}

#[tokio::test]
async fn test_create_application_status_is_ready() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .application_name("status-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .expect("create_application should succeed");

    let detail = resp
        .application_detail()
        .expect("should have application detail");
    assert_eq!(
        detail.application_status(),
        &aws_sdk_kinesisanalyticsv2::types::ApplicationStatus::Ready,
    );
}

#[tokio::test]
async fn test_list_applications_empty() {
    let client = make_client().await;

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications on empty state should succeed");

    assert_eq!(resp.application_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_applications_returns_correct_arn() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("arn-list-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink120)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    let summaries = resp.application_summaries();
    assert_eq!(summaries.len(), 1);
    assert_eq!(
        summaries[0].application_arn(),
        "arn:aws:kinesisanalytics:us-east-1:123456789012:application/arn-list-app"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_application_fails() {
    let client = make_client().await;

    let result = client
        .describe_application()
        .application_name("does-not-exist")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_application_then_recreate_succeeds() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("recycle-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .delete_application()
        .application_name("recycle-app")
        .create_timestamp(aws_smithy_types::DateTime::from_secs(0))
        .send()
        .await
        .expect("delete_application should succeed");

    // Recreating after deletion should succeed (no ResourceInUseException)
    let resp = client
        .create_application()
        .application_name("recycle-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink120)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .expect("recreating deleted application should succeed");

    let detail = resp
        .application_detail()
        .expect("should have application detail");
    assert_eq!(detail.application_name(), "recycle-app");
    assert_eq!(detail.runtime_environment().as_str(), "FLINK-1_20");
}

#[tokio::test]
async fn test_list_applications_summary_runtime_and_status() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("summary-fields-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    let summaries = resp.application_summaries();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].application_name(), "summary-fields-app");
    assert_eq!(summaries[0].runtime_environment().as_str(), "FLINK-1_18");
    assert_eq!(
        summaries[0].application_status(),
        &aws_sdk_kinesisanalyticsv2::types::ApplicationStatus::Ready,
    );
    assert_eq!(summaries[0].application_version_id(), 1);
}

// ============================================================================
// Additional tests derived from AWS documentation: Managed Service for Apache Flink
// ============================================================================

#[tokio::test]
async fn test_describe_application_timestamps() {
    // Verify that create_timestamp and last_update_timestamp are populated and non-zero
    let client = make_client().await;

    client
        .create_application()
        .application_name("ts-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_application()
        .application_name("ts-app")
        .send()
        .await
        .expect("describe_application should succeed");

    let app = resp
        .application_detail()
        .expect("should have application_detail");
    assert!(
        app.create_timestamp().is_some(),
        "create_timestamp should be present"
    );
    assert!(
        app.last_update_timestamp().is_some(),
        "last_update_timestamp should be present"
    );
    // Both timestamps should be non-zero (after Unix epoch)
    let create_ts = app.create_timestamp().unwrap().secs();
    assert!(
        create_ts > 0,
        "create_timestamp should be > 0, got {create_ts}"
    );
    let update_ts = app.last_update_timestamp().unwrap().secs();
    assert!(
        update_ts > 0,
        "last_update_timestamp should be > 0, got {update_ts}"
    );
}

#[tokio::test]
async fn test_describe_application_configuration_present() {
    // Verify that ApplicationConfigurationDescription is present in the DescribeApplication response
    let client = make_client().await;

    client
        .create_application()
        .application_name("cfg-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_application()
        .application_name("cfg-app")
        .send()
        .await
        .expect("describe_application should succeed");

    let app = resp
        .application_detail()
        .expect("should have application_detail");
    assert!(
        app.application_configuration_description().is_some(),
        "application_configuration_description should be present"
    );
}

#[tokio::test]
async fn test_create_application_duplicate_error_type() {
    // Verify the error type for creating a duplicate application is ResourceInUseException
    let client = make_client().await;

    client
        .create_application()
        .application_name("err-type-dup-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let err = client
        .create_application()
        .application_name("err-type-dup-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceInUseException"),
        "Expected ResourceInUseException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_application_error_type() {
    // Verify the error type for deleting a nonexistent application is ResourceNotFoundException
    let client = make_client().await;

    let err = client
        .delete_application()
        .application_name("does-not-exist-del")
        .create_timestamp(aws_smithy_types::DateTime::from_secs(0))
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_application_error_type() {
    // Verify the error type for describing a nonexistent application is ResourceNotFoundException
    let client = make_client().await;

    let err = client
        .describe_application()
        .application_name("does-not-exist-desc")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// StartApplication / StopApplication
// ============================================================================

#[tokio::test]
async fn test_start_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("start-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .start_application()
        .application_name("start-app")
        .send()
        .await
        .expect("start_application should succeed");

    let resp = client
        .describe_application()
        .application_name("start-app")
        .send()
        .await
        .unwrap();
    let detail = resp.application_detail().unwrap();
    assert_eq!(
        detail.application_status(),
        &aws_sdk_kinesisanalyticsv2::types::ApplicationStatus::Running
    );
}

#[tokio::test]
async fn test_stop_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("stop-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .start_application()
        .application_name("stop-app")
        .send()
        .await
        .unwrap();

    client
        .stop_application()
        .application_name("stop-app")
        .send()
        .await
        .expect("stop_application should succeed");

    let resp = client
        .describe_application()
        .application_name("stop-app")
        .send()
        .await
        .unwrap();
    let detail = resp.application_detail().unwrap();
    assert_eq!(
        detail.application_status(),
        &aws_sdk_kinesisanalyticsv2::types::ApplicationStatus::Ready
    );
}

#[tokio::test]
async fn test_start_nonexistent_application_fails() {
    let client = make_client().await;

    let err = client
        .start_application()
        .application_name("no-such-app")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// UpdateApplication
// ============================================================================

#[tokio::test]
async fn test_update_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("update-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/old-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_application()
        .application_name("update-app")
        .send()
        .await
        .expect("update_application should succeed");

    let detail = resp.application_detail().unwrap();
    assert_eq!(detail.application_name(), "update-app");
    // version should be bumped after update
    assert_eq!(detail.application_version_id(), 2);
}

#[tokio::test]
async fn test_update_nonexistent_application_fails() {
    let client = make_client().await;

    let err = client
        .update_application()
        .application_name("no-such-app")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// TagResource / UntagResource / ListTagsForResource
// ============================================================================

#[tokio::test]
async fn test_tag_resource() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .application_name("tagged-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .application_detail()
        .unwrap()
        .application_arn()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_kinesisanalyticsv2::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value().unwrap_or(""), "test");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .application_name("untag-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .application_detail()
        .unwrap()
        .application_arn()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_kinesisanalyticsv2::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

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
        .expect("list_tags_for_resource should succeed");
    assert_eq!(resp.tags().len(), 0);
}

#[tokio::test]
async fn test_list_tags_nonexistent_resource_fails() {
    let client = make_client().await;

    let err = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:kinesisanalytics:us-east-1:123456789012:application/no-such-app")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// CreateApplicationSnapshot / DeleteApplicationSnapshot / DescribeApplicationSnapshot / ListApplicationSnapshots
// ============================================================================

#[tokio::test]
async fn test_create_application_snapshot() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("snap-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .create_application_snapshot()
        .application_name("snap-app")
        .snapshot_name("snap-1")
        .send()
        .await
        .expect("create_application_snapshot should succeed");
}

#[tokio::test]
async fn test_describe_application_snapshot() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("snap-desc-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .create_application_snapshot()
        .application_name("snap-desc-app")
        .snapshot_name("snap-desc-1")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_application_snapshot()
        .application_name("snap-desc-app")
        .snapshot_name("snap-desc-1")
        .send()
        .await
        .expect("describe_application_snapshot should succeed");

    let details = resp.snapshot_details().unwrap();
    assert_eq!(details.snapshot_name(), "snap-desc-1");
}

#[tokio::test]
async fn test_list_application_snapshots() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("snap-list-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    for name in ["snap-a", "snap-b", "snap-c"] {
        client
            .create_application_snapshot()
            .application_name("snap-list-app")
            .snapshot_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_application_snapshots()
        .application_name("snap-list-app")
        .send()
        .await
        .expect("list_application_snapshots should succeed");

    assert_eq!(resp.snapshot_summaries().len(), 3);
}

#[tokio::test]
async fn test_delete_application_snapshot() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("snap-del-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .create_application_snapshot()
        .application_name("snap-del-app")
        .snapshot_name("snap-del-1")
        .send()
        .await
        .unwrap();

    client
        .delete_application_snapshot()
        .application_name("snap-del-app")
        .snapshot_name("snap-del-1")
        .snapshot_creation_timestamp(aws_smithy_types::DateTime::from_secs(0))
        .send()
        .await
        .expect("delete_application_snapshot should succeed");

    let resp = client
        .list_application_snapshots()
        .application_name("snap-del-app")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.snapshot_summaries().len(), 0);
}

// ============================================================================
// ListApplicationVersions / DescribeApplicationVersion
// ============================================================================

#[tokio::test]
async fn test_list_application_versions() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("ver-list-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_application_versions()
        .application_name("ver-list-app")
        .send()
        .await
        .expect("list_application_versions should succeed");

    assert_eq!(resp.application_version_summaries().len(), 1);
    assert_eq!(
        resp.application_version_summaries()[0].application_version_id(),
        1
    );
}

#[tokio::test]
async fn test_describe_application_version() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("ver-desc-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_application_version()
        .application_name("ver-desc-app")
        .application_version_id(1)
        .send()
        .await
        .expect("describe_application_version should succeed");

    let detail = resp.application_version_detail().unwrap();
    assert_eq!(detail.application_name(), "ver-desc-app");
    assert_eq!(detail.application_version_id(), 1);
}

// ============================================================================
// RollbackApplication
// ============================================================================

#[tokio::test]
async fn test_rollback_application() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("rollback-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    // Update to bump version to 2
    client
        .update_application()
        .application_name("rollback-app")
        .send()
        .await
        .unwrap();

    let resp = client
        .rollback_application()
        .application_name("rollback-app")
        .current_application_version_id(2)
        .send()
        .await
        .expect("rollback_application should succeed");

    let detail = resp.application_detail().unwrap();
    assert_eq!(detail.application_version_id(), 1);
}

// ============================================================================
// CreateApplicationPresignedUrl
// ============================================================================

#[tokio::test]
async fn test_create_application_presigned_url() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("presigned-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_application_presigned_url()
        .application_name("presigned-app")
        .url_type(aws_sdk_kinesisanalyticsv2::types::UrlType::FlinkDashboardUrl)
        .send()
        .await
        .expect("create_application_presigned_url should succeed");

    assert!(
        resp.authorized_url().is_some(),
        "authorized_url should be present"
    );
}

// ============================================================================
// AddApplicationCloudWatchLoggingOption / DeleteApplicationCloudWatchLoggingOption
// ============================================================================

#[tokio::test]
async fn test_add_and_delete_cloud_watch_logging_option() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("cw-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let add_resp = client
        .add_application_cloud_watch_logging_option()
        .application_name("cw-app")
        .cloud_watch_logging_option(
            aws_sdk_kinesisanalyticsv2::types::CloudWatchLoggingOption::builder()
                .log_stream_arn(
                    "arn:aws:logs:us-east-1:123456789012:log-group:test:log-stream:test",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_application_cloud_watch_logging_option should succeed");

    assert_eq!(
        add_resp.application_version_id(),
        Some(2),
        "version should be bumped after adding CW logging"
    );

    client
        .delete_application_cloud_watch_logging_option()
        .application_name("cw-app")
        .cloud_watch_logging_option_id("cw-option-id")
        .current_application_version_id(2)
        .send()
        .await
        .expect("delete_application_cloud_watch_logging_option should succeed");
}

// ============================================================================
// AddApplicationVpcConfiguration / DeleteApplicationVpcConfiguration
// ============================================================================

#[tokio::test]
async fn test_add_and_delete_vpc_configuration() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("vpc-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let add_resp = client
        .add_application_vpc_configuration()
        .application_name("vpc-app")
        .vpc_configuration(
            aws_sdk_kinesisanalyticsv2::types::VpcConfiguration::builder()
                .subnet_ids("subnet-12345678")
                .security_group_ids("sg-12345678")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_application_vpc_configuration should succeed");

    assert_eq!(add_resp.application_version_id(), Some(2));

    client
        .delete_application_vpc_configuration()
        .application_name("vpc-app")
        .vpc_configuration_id("vpc-config-id")
        .current_application_version_id(2)
        .send()
        .await
        .expect("delete_application_vpc_configuration should succeed");
}

// ============================================================================
// ListApplicationOperations / DescribeApplicationOperation
// ============================================================================

#[tokio::test]
async fn test_list_application_operations() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("ops-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_application_operations()
        .application_name("ops-app")
        .send()
        .await
        .expect("list_application_operations should succeed");

    assert_eq!(resp.application_operation_info_list().len(), 0);
}

#[tokio::test]
async fn test_describe_application_operation() {
    let client = make_client().await;

    client
        .create_application()
        .application_name("op-desc-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    client
        .describe_application_operation()
        .application_name("op-desc-app")
        .operation_id("op-123")
        .send()
        .await
        .expect("describe_application_operation should succeed");
}

// ============================================================================
// UpdateApplicationMaintenanceConfiguration
// ============================================================================

#[tokio::test]
async fn test_update_application_maintenance_configuration() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .application_name("maint-app")
        .runtime_environment(aws_sdk_kinesisanalyticsv2::types::RuntimeEnvironment::Flink118)
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .application_detail()
        .unwrap()
        .application_arn()
        .to_string();

    let resp = client
        .update_application_maintenance_configuration()
        .application_name("maint-app")
        .application_maintenance_configuration_update(
            aws_sdk_kinesisanalyticsv2::types::ApplicationMaintenanceConfigurationUpdate::builder()
                .application_maintenance_window_start_time_update("06:00")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_application_maintenance_configuration should succeed");

    assert_eq!(resp.application_arn().unwrap_or(""), arn);
}

// ============================================================================
// DiscoverInputSchema
// ============================================================================

#[tokio::test]
async fn test_discover_input_schema() {
    let client = make_client().await;

    client
        .discover_input_schema()
        .resource_arn("arn:aws:kinesis:us-east-1:123456789012:stream/test-stream")
        .service_execution_role("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .expect("discover_input_schema should succeed");
}

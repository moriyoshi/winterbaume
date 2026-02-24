//! Integration tests for winterbaume Timestream Write service.
//!
//! These tests verify that aws-sdk-timestreamwrite operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_timestreamwrite::config::BehaviorVersion;
use aws_sdk_timestreamwrite::types::{
    Dimension, MagneticStoreWriteProperties, Record, RetentionProperties, Tag, TimeUnit,
};
use winterbaume_core::MockAws;
use winterbaume_timestreamwrite::TimestreamWriteService;

/// Helper to create a configured Timestream Write client backed by winterbaume.
async fn make_timestream_client() -> aws_sdk_timestreamwrite::Client {
    let mock = MockAws::builder()
        .with_service(TimestreamWriteService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreamwrite::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_timestreamwrite::Client::new(&config)
}

// --- Database tests ---

#[tokio::test]
async fn test_create_database() {
    let client = make_timestream_client().await;

    let resp = client
        .create_database()
        .database_name("test-db")
        .send()
        .await
        .expect("create_database should succeed");

    let db = resp.database().expect("should have database");
    assert_eq!(db.database_name(), Some("test-db"));
    assert!(db.arn().is_some(), "should have ARN");
    assert_eq!(db.table_count(), 0);
}

#[tokio::test]
async fn test_create_database_with_kms_key() {
    let client = make_timestream_client().await;

    let resp = client
        .create_database()
        .database_name("encrypted-db")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/my-key-id")
        .send()
        .await
        .expect("create_database with KMS key should succeed");

    let db = resp.database().expect("should have database");
    assert_eq!(db.database_name(), Some("encrypted-db"));
    assert_eq!(
        db.kms_key_id(),
        Some("arn:aws:kms:us-east-1:123456789012:key/my-key-id")
    );
}

#[tokio::test]
async fn test_describe_database() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("describe-db")
        .send()
        .await
        .expect("create_database should succeed");

    let resp = client
        .describe_database()
        .database_name("describe-db")
        .send()
        .await
        .expect("describe_database should succeed");

    let db = resp.database().expect("should have database");
    assert_eq!(db.database_name(), Some("describe-db"));
    assert!(db.arn().is_some(), "should have ARN");
}

#[tokio::test]
async fn test_describe_nonexistent_database() {
    let client = make_timestream_client().await;

    let result = client
        .describe_database()
        .database_name("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_database on nonexistent database should fail"
    );
}

#[tokio::test]
async fn test_delete_database() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("delete-db")
        .send()
        .await
        .expect("create_database should succeed");

    client
        .delete_database()
        .database_name("delete-db")
        .send()
        .await
        .expect("delete_database should succeed");

    // Describe should fail after deletion
    let result = client
        .describe_database()
        .database_name("delete-db")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_database after delete should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_database() {
    let client = make_timestream_client().await;

    let result = client
        .delete_database()
        .database_name("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_database on nonexistent database should fail"
    );
}

#[tokio::test]
async fn test_list_databases_empty() {
    let client = make_timestream_client().await;

    let resp = client
        .list_databases()
        .send()
        .await
        .expect("list_databases should succeed");

    assert!(
        resp.databases().is_empty(),
        "databases list should be empty"
    );
}

#[tokio::test]
async fn test_list_databases() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("db-alpha")
        .send()
        .await
        .unwrap();

    client
        .create_database()
        .database_name("db-beta")
        .send()
        .await
        .unwrap();

    client
        .create_database()
        .database_name("db-gamma")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_databases()
        .send()
        .await
        .expect("list_databases should succeed");

    let databases = resp.databases();
    assert_eq!(databases.len(), 3, "should have 3 databases");

    // Verify sorted order
    let names: Vec<&str> = databases
        .iter()
        .filter_map(|db| db.database_name())
        .collect();
    assert_eq!(names, vec!["db-alpha", "db-beta", "db-gamma"]);
}

#[tokio::test]
async fn test_create_duplicate_database() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("dup-db")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_database()
        .database_name("dup-db")
        .send()
        .await;

    assert!(result.is_err(), "creating duplicate database should fail");
}

// --- UpdateDatabase tests ---

#[tokio::test]
async fn test_update_database() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("update-db")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_database()
        .database_name("update-db")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/new-key")
        .send()
        .await
        .expect("update_database should succeed");

    let db = resp.database().expect("should have database");
    assert_eq!(db.database_name(), Some("update-db"));
    assert_eq!(
        db.kms_key_id(),
        Some("arn:aws:kms:us-east-1:123456789012:key/new-key")
    );
}

#[tokio::test]
async fn test_update_nonexistent_database() {
    let client = make_timestream_client().await;

    let result = client
        .update_database()
        .database_name("nonexistent")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/some-key")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_database on nonexistent should fail"
    );
}

// --- Table tests ---

#[tokio::test]
async fn test_create_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("table-db")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_table()
        .database_name("table-db")
        .table_name("test-table")
        .send()
        .await
        .expect("create_table should succeed");

    let table = resp.table().expect("should have table");
    assert_eq!(table.table_name(), Some("test-table"));
    assert_eq!(table.database_name(), Some("table-db"));
    assert!(table.arn().is_some());
    assert_eq!(
        table.table_status(),
        Some(&aws_sdk_timestreamwrite::types::TableStatus::Active)
    );
}

#[tokio::test]
async fn test_create_table_with_retention() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("ret-db")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_table()
        .database_name("ret-db")
        .table_name("ret-table")
        .retention_properties(
            RetentionProperties::builder()
                .memory_store_retention_period_in_hours(24)
                .magnetic_store_retention_period_in_days(365)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table with retention should succeed");

    let table = resp.table().expect("should have table");
    let rp = table.retention_properties().expect("should have retention");
    assert_eq!(rp.memory_store_retention_period_in_hours(), 24);
    assert_eq!(rp.magnetic_store_retention_period_in_days(), 365);
}

#[tokio::test]
async fn test_create_table_with_magnetic_store_writes() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("mag-db")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_table()
        .database_name("mag-db")
        .table_name("mag-table")
        .magnetic_store_write_properties(
            MagneticStoreWriteProperties::builder()
                .enable_magnetic_store_writes(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table with magnetic store writes should succeed");

    let table = resp.table().expect("should have table");
    let mswp = table
        .magnetic_store_write_properties()
        .expect("should have magnetic store write properties");
    assert!(mswp.enable_magnetic_store_writes());
}

#[tokio::test]
async fn test_create_duplicate_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("dup-table-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("dup-table-db")
        .table_name("dup-table")
        .send()
        .await
        .unwrap();

    let result = client
        .create_table()
        .database_name("dup-table-db")
        .table_name("dup-table")
        .send()
        .await;

    assert!(result.is_err(), "creating duplicate table should fail");
}

#[tokio::test]
async fn test_create_table_nonexistent_database() {
    let client = make_timestream_client().await;

    let result = client
        .create_table()
        .database_name("nonexistent-db")
        .table_name("some-table")
        .send()
        .await;

    assert!(
        result.is_err(),
        "create_table in nonexistent database should fail"
    );
}

#[tokio::test]
async fn test_describe_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("desc-table-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("desc-table-db")
        .table_name("desc-table")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_table()
        .database_name("desc-table-db")
        .table_name("desc-table")
        .send()
        .await
        .expect("describe_table should succeed");

    let table = resp.table().expect("should have table");
    assert_eq!(table.table_name(), Some("desc-table"));
    assert_eq!(table.database_name(), Some("desc-table-db"));
}

#[tokio::test]
async fn test_describe_nonexistent_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("desc-noexist-db")
        .send()
        .await
        .unwrap();

    let result = client
        .describe_table()
        .database_name("desc-noexist-db")
        .table_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "describe nonexistent table should fail");
}

#[tokio::test]
async fn test_delete_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("del-table-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("del-table-db")
        .table_name("del-table")
        .send()
        .await
        .unwrap();

    client
        .delete_table()
        .database_name("del-table-db")
        .table_name("del-table")
        .send()
        .await
        .expect("delete_table should succeed");

    let result = client
        .describe_table()
        .database_name("del-table-db")
        .table_name("del-table")
        .send()
        .await;

    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_delete_table_updates_count() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("count-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("count-db")
        .table_name("t1")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("count-db")
        .table_name("t2")
        .send()
        .await
        .unwrap();

    // Check table_count is 2
    let resp = client
        .describe_database()
        .database_name("count-db")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.database().unwrap().table_count(), 2);

    // Delete one table
    client
        .delete_table()
        .database_name("count-db")
        .table_name("t1")
        .send()
        .await
        .unwrap();

    // Check table_count is 1
    let resp = client
        .describe_database()
        .database_name("count-db")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.database().unwrap().table_count(), 1);
}

#[tokio::test]
async fn test_list_tables() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("list-table-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("list-table-db")
        .table_name("alpha")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("list-table-db")
        .table_name("beta")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tables()
        .database_name("list-table-db")
        .send()
        .await
        .expect("list_tables should succeed");

    let tables = resp.tables();
    assert_eq!(tables.len(), 2);

    let names: Vec<&str> = tables.iter().filter_map(|t| t.table_name()).collect();
    assert_eq!(names, vec!["alpha", "beta"]);
}

#[tokio::test]
async fn test_list_tables_empty() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("empty-table-db")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tables()
        .database_name("empty-table-db")
        .send()
        .await
        .expect("list_tables should succeed");

    assert!(resp.tables().is_empty());
}

#[tokio::test]
async fn test_update_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("upd-table-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("upd-table-db")
        .table_name("upd-table")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_table()
        .database_name("upd-table-db")
        .table_name("upd-table")
        .retention_properties(
            RetentionProperties::builder()
                .memory_store_retention_period_in_hours(48)
                .magnetic_store_retention_period_in_days(730)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_table should succeed");

    let table = resp.table().expect("should have table");
    let rp = table.retention_properties().expect("should have retention");
    assert_eq!(rp.memory_store_retention_period_in_hours(), 48);
    assert_eq!(rp.magnetic_store_retention_period_in_days(), 730);
}

// --- WriteRecords tests ---

#[tokio::test]
async fn test_write_records() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("write-db")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .database_name("write-db")
        .table_name("write-table")
        .send()
        .await
        .unwrap();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let resp = client
        .write_records()
        .database_name("write-db")
        .table_name("write-table")
        .records(
            Record::builder()
                .dimensions(
                    Dimension::builder()
                        .name("host")
                        .value("host1")
                        .build()
                        .unwrap(),
                )
                .measure_name("cpu")
                .measure_value("80.5")
                .measure_value_type(aws_sdk_timestreamwrite::types::MeasureValueType::Double)
                .time(now.clone())
                .time_unit(TimeUnit::Milliseconds)
                .build(),
        )
        .send()
        .await
        .expect("write_records should succeed");

    let ingested = resp
        .records_ingested()
        .expect("should have records_ingested");
    assert_eq!(ingested.total(), 1);
    assert_eq!(ingested.memory_store(), 1);
}

#[tokio::test]
async fn test_write_records_nonexistent_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("write-noexist-db")
        .send()
        .await
        .unwrap();

    let result = client
        .write_records()
        .database_name("write-noexist-db")
        .table_name("nonexistent")
        .records(
            Record::builder()
                .measure_name("test")
                .measure_value("1")
                .measure_value_type(aws_sdk_timestreamwrite::types::MeasureValueType::Bigint)
                .time("1000000")
                .time_unit(TimeUnit::Milliseconds)
                .build(),
        )
        .send()
        .await;

    assert!(result.is_err(), "write to nonexistent table should fail");
}

// --- DescribeEndpoints tests ---

#[tokio::test]
async fn test_describe_endpoints() {
    let client = make_timestream_client().await;

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
    assert!(
        endpoints[0].cache_period_in_minutes() > 0,
        "cache period should be positive"
    );
}

// --- Tag tests ---

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_timestream_client().await;

    let create_resp = client
        .create_database()
        .database_name("tag-db")
        .send()
        .await
        .unwrap();

    let arn = create_resp.database().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .tags(
            Tag::builder()
                .key("team")
                .value("platform")
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
    assert_eq!(tags.len(), 2, "should have 2 tags");

    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("env"), Some(&"prod"));
    assert_eq!(tag_map.get("team"), Some(&"platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_timestream_client().await;

    let create_resp = client
        .create_database()
        .database_name("untag-db")
        .send()
        .await
        .unwrap();

    let arn = create_resp.database().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .tags(Tag::builder().key("team").value("ops").build().unwrap())
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
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1, "should have 1 tag after untag");
    assert_eq!(tags[0].key(), "team");
}

#[tokio::test]
async fn test_tag_table() {
    let client = make_timestream_client().await;

    client
        .create_database()
        .database_name("tag-table-db")
        .send()
        .await
        .unwrap();

    let table_resp = client
        .create_table()
        .database_name("tag-table-db")
        .table_name("tag-table")
        .send()
        .await
        .unwrap();

    let arn = table_resp.table().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            Tag::builder()
                .key("project")
                .value("analytics")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource on table should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "project");
    assert_eq!(resp.tags()[0].value(), "analytics");
}

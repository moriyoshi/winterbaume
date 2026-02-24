use aws_sdk_s3tables::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3tables::S3TablesService;

async fn make_client() -> aws_sdk_s3tables::Client {
    let mock = MockAws::builder()
        .with_service(S3TablesService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3tables::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_s3tables::Client::new(&config)
}

// ---------------------------------------------------------------------------
// Table Bucket tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_table_bucket() {
    let client = make_client().await;

    let resp = client
        .create_table_bucket()
        .name("my-bucket")
        .send()
        .await
        .expect("create_table_bucket should succeed");

    let arn = resp.arn();
    assert!(arn.contains("my-bucket"), "ARN should contain bucket name");

    let get_resp = client
        .get_table_bucket()
        .table_bucket_arn(arn)
        .send()
        .await
        .expect("get_table_bucket should succeed");

    assert_eq!(get_resp.name(), "my-bucket");
    assert_eq!(get_resp.arn(), arn);
}

#[tokio::test]
async fn test_list_table_buckets() {
    let client = make_client().await;

    client
        .create_table_bucket()
        .name("bucket-a")
        .send()
        .await
        .unwrap();
    client
        .create_table_bucket()
        .name("bucket-b")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_table_buckets()
        .send()
        .await
        .expect("list_table_buckets should succeed");

    let names: Vec<&str> = list_resp.table_buckets().iter().map(|b| b.name()).collect();
    assert!(names.contains(&"bucket-a"));
    assert!(names.contains(&"bucket-b"));
}

#[tokio::test]
async fn test_delete_table_bucket() {
    let client = make_client().await;

    let resp = client
        .create_table_bucket()
        .name("del-bucket")
        .send()
        .await
        .unwrap();
    let arn = resp.arn().to_string();

    client
        .delete_table_bucket()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("delete_table_bucket should succeed");

    let result = client
        .get_table_bucket()
        .table_bucket_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_get_table_bucket_not_found() {
    let client = make_client().await;

    let result = client
        .get_table_bucket()
        .table_bucket_arn("arn:aws:s3tables:us-east-1:123456789012:bucket/nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Namespace tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_namespace() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("ns-test-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("mydb")
        .send()
        .await
        .expect("create_namespace should succeed");

    let get_resp = client
        .get_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("mydb")
        .send()
        .await
        .expect("get_namespace should succeed");

    assert_eq!(get_resp.namespace(), ["mydb"]);
}

#[tokio::test]
async fn test_list_namespaces() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("ns-list-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("db1")
        .send()
        .await
        .unwrap();
    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("db2")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_namespaces()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await
        .expect("list_namespaces should succeed");

    assert_eq!(list_resp.namespaces().len(), 2);
}

#[tokio::test]
async fn test_delete_namespace() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("ns-del-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("todelete")
        .send()
        .await
        .unwrap();

    client
        .delete_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("todelete")
        .send()
        .await
        .expect("delete_namespace should succeed");

    let result = client
        .get_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("todelete")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_get_namespace_not_found() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("ns-notfound-bucket")
        .send()
        .await
        .unwrap();

    let result = client
        .get_namespace()
        .table_bucket_arn(bucket_resp.arn())
        .namespace("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Table tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_table() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("tbl-test-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("mytable")
        .format(OpenTableFormat::Iceberg)
        .send()
        .await
        .expect("create_table should succeed");

    assert!(create_resp.table_arn().contains("mytable"));
    assert!(!create_resp.version_token().is_empty());

    let get_resp = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("mytable")
        .send()
        .await
        .expect("get_table should succeed");

    assert_eq!(get_resp.name(), "mytable");
    assert!(!get_resp.warehouse_location().is_empty());
}

#[tokio::test]
async fn test_list_tables() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("tbl-list-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("table1")
        .format(OpenTableFormat::Iceberg)
        .send()
        .await
        .unwrap();
    client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("table2")
        .format(OpenTableFormat::Iceberg)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_tables()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await
        .expect("list_tables should succeed");

    let names: Vec<&str> = list_resp.tables().iter().map(|t| t.name()).collect();
    assert!(names.contains(&"table1"));
    assert!(names.contains(&"table2"));
}

#[tokio::test]
async fn test_delete_table() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("tbl-del-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .send()
        .await
        .unwrap();

    client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("deltable")
        .format(OpenTableFormat::Iceberg)
        .send()
        .await
        .unwrap();

    client
        .delete_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("deltable")
        .send()
        .await
        .expect("delete_table should succeed");

    let result = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("deltable")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_get_table_not_found() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("tbl-notfound-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("ns")
        .send()
        .await
        .unwrap();

    let result = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("ns")
        .name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Tag tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_tag_table_bucket() {
    let client = make_client().await;

    let resp = client
        .create_table_bucket()
        .name("tag-bucket")
        .send()
        .await
        .unwrap();
    let arn = resp.arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let empty = std::collections::HashMap::new();
    let tags = tags_resp.tags().unwrap_or(&empty);
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

#[tokio::test]
async fn test_untag_table_bucket() {
    let client = make_client().await;

    let resp = client
        .create_table_bucket()
        .name("untag-bucket")
        .send()
        .await
        .unwrap();
    let arn = resp.arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("keep", "yes")
        .tags("remove", "no")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("remove")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let empty = std::collections::HashMap::new();
    let tags = tags_resp.tags().unwrap_or(&empty);
    assert!(tags.contains_key("keep"));
    assert!(!tags.contains_key("remove"));
}

// ---------------------------------------------------------------------------
// Lifecycle test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_full_lifecycle() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;

    // Create bucket
    let bucket_resp = client
        .create_table_bucket()
        .name("lifecycle-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    // Create namespace
    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("prod")
        .send()
        .await
        .unwrap();

    // Create table
    let create_resp = client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("prod")
        .name("events")
        .format(OpenTableFormat::Iceberg)
        .send()
        .await
        .unwrap();
    let table_arn = create_resp.table_arn().to_string();

    // Tag table
    client
        .tag_resource()
        .resource_arn(&table_arn)
        .tags("stage", "prod")
        .send()
        .await
        .unwrap();

    // Verify tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&table_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        tags_resp
            .tags()
            .and_then(|t| t.get("stage"))
            .map(|s| s.as_str()),
        Some("prod")
    );

    // Delete table
    client
        .delete_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("prod")
        .name("events")
        .send()
        .await
        .unwrap();

    // Delete namespace
    client
        .delete_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("prod")
        .send()
        .await
        .unwrap();

    // Delete bucket
    client
        .delete_table_bucket()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_table_bucket()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// State view tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_s3tables::S3TablesStateView;
    use winterbaume_s3tables::views::TableBucketView;

    let svc = S3TablesService::new();

    // Seed state via restore
    let mut initial = S3TablesStateView::default();
    initial.table_buckets.insert(
        "arn:aws:s3tables:us-east-1:123456789012:bucket/snap-bucket".to_string(),
        TableBucketView {
            name: "snap-bucket".to_string(),
            arn: "arn:aws:s3tables:us-east-1:123456789012:bucket/snap-bucket".to_string(),
            owner_account_id: "123456789012".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            tags: std::collections::HashMap::new(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", initial)
        .await
        .expect("restore should succeed");

    // Snapshot and restore into new service
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.table_buckets.len(), 1);

    let svc2 = S3TablesService::new();
    svc2.restore("123456789012", "us-east-1", snapshot)
        .await
        .expect("restore should succeed");

    let snapshot2 = svc2.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot2.table_buckets.len(), 1);
    assert_eq!(
        snapshot2.table_buckets.values().next().unwrap().name,
        "snap-bucket"
    );
}

#[tokio::test]
async fn test_merge_does_not_remove_existing() {
    use winterbaume_core::StatefulService;
    use winterbaume_s3tables::S3TablesStateView;

    let svc = S3TablesService::new();

    // Pre-seed one bucket via restore
    let initial_view = {
        use std::collections::HashMap;
        let mut view = S3TablesStateView::default();
        let mut buckets = HashMap::new();
        buckets.insert(
            "arn:aws:s3tables:us-east-1:123456789012:bucket/original".to_string(),
            winterbaume_s3tables::views::TableBucketView {
                name: "original".to_string(),
                arn: "arn:aws:s3tables:us-east-1:123456789012:bucket/original".to_string(),
                owner_account_id: "123456789012".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                tags: HashMap::new(),
                ..Default::default()
            },
        );
        view.table_buckets = buckets;
        view
    };
    svc.restore("123456789012", "us-east-1", initial_view)
        .await
        .unwrap();

    // Merge a second bucket
    let merge_view = {
        use std::collections::HashMap;
        let mut view = S3TablesStateView::default();
        let mut buckets = HashMap::new();
        buckets.insert(
            "arn:aws:s3tables:us-east-1:123456789012:bucket/merged".to_string(),
            winterbaume_s3tables::views::TableBucketView {
                name: "merged".to_string(),
                arn: "arn:aws:s3tables:us-east-1:123456789012:bucket/merged".to_string(),
                owner_account_id: "123456789012".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                tags: HashMap::new(),
                ..Default::default()
            },
        );
        view.table_buckets = buckets;
        view
    };
    svc.merge("123456789012", "us-east-1", merge_view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(
        snapshot.table_buckets.len(),
        2,
        "both buckets should be present"
    );
    assert!(
        snapshot
            .table_buckets
            .contains_key("arn:aws:s3tables:us-east-1:123456789012:bucket/original")
    );
    assert!(
        snapshot
            .table_buckets
            .contains_key("arn:aws:s3tables:us-east-1:123456789012:bucket/merged")
    );
}

// ---------------------------------------------------------------------------
// State change notification tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = S3TablesService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        winterbaume_s3tables::S3TablesStateView::default(),
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_s3tables::S3TablesStateView;

    let svc = S3TablesService::new();

    // Seed initial state (fires first notification — ignore it)
    svc.restore("123456789012", "us-east-1", S3TablesStateView::default())
        .await
        .unwrap();

    // Subscribe after the first restore
    let snapshots: Arc<Mutex<Vec<S3TablesStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Restore with a non-empty bucket
    let mut view = S3TablesStateView::default();
    view.table_buckets.insert(
        "arn:aws:s3tables:us-east-1:123456789012:bucket/expected".to_string(),
        winterbaume_s3tables::views::TableBucketView {
            name: "expected".to_string(),
            arn: "arn:aws:s3tables:us-east-1:123456789012:bucket/expected".to_string(),
            owner_account_id: "123456789012".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            tags: std::collections::HashMap::new(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0]
            .table_buckets
            .contains_key("arn:aws:s3tables:us-east-1:123456789012:bucket/expected"),
        "snapshot should contain the restored bucket"
    );
}

// ---------------------------------------------------------------------------
// Table Bucket extended operation tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_table_bucket_encryption() {
    use aws_sdk_s3tables::types::{EncryptionConfiguration, SseAlgorithm};
    let client = make_client().await;

    let create = client
        .create_table_bucket()
        .name("enc-bucket")
        .send()
        .await
        .expect("create_table_bucket");
    let arn = create.arn().to_string();

    // Put encryption
    let enc = EncryptionConfiguration::builder()
        .sse_algorithm(SseAlgorithm::Aes256)
        .build()
        .unwrap();
    client
        .put_table_bucket_encryption()
        .table_bucket_arn(&arn)
        .encryption_configuration(enc)
        .send()
        .await
        .expect("put_table_bucket_encryption");

    // Get encryption
    let get = client
        .get_table_bucket_encryption()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("get_table_bucket_encryption");
    assert_eq!(
        get.encryption_configuration()
            .unwrap()
            .sse_algorithm()
            .as_str(),
        "AES256"
    );

    // Delete encryption
    client
        .delete_table_bucket_encryption()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("delete_table_bucket_encryption");
    // Note: SDK's correct_errors always synthesizes a non-None EncryptionConfiguration,
    // so we just verify the delete call succeeds without error.
}

#[tokio::test]
async fn test_table_bucket_policy() {
    let client = make_client().await;

    let create = client
        .create_table_bucket()
        .name("policy-bucket")
        .send()
        .await
        .expect("create_table_bucket");
    let arn = create.arn().to_string();

    let policy_json = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_table_bucket_policy()
        .table_bucket_arn(&arn)
        .resource_policy(policy_json)
        .send()
        .await
        .expect("put_table_bucket_policy");

    let get = client
        .get_table_bucket_policy()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("get_table_bucket_policy");
    assert_eq!(get.resource_policy(), policy_json);

    client
        .delete_table_bucket_policy()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("delete_table_bucket_policy");

    let get2 = client
        .get_table_bucket_policy()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("get after delete");
    assert!(get2.resource_policy().is_empty());
}

#[tokio::test]
async fn test_table_bucket_storage_class() {
    use aws_sdk_s3tables::types::{StorageClass, StorageClassConfiguration};
    let client = make_client().await;

    let create = client
        .create_table_bucket()
        .name("sc-bucket")
        .send()
        .await
        .expect("create_table_bucket");
    let arn = create.arn().to_string();

    let sc_config = StorageClassConfiguration::builder()
        .storage_class(StorageClass::Standard)
        .build()
        .unwrap();
    client
        .put_table_bucket_storage_class()
        .table_bucket_arn(&arn)
        .storage_class_configuration(sc_config)
        .send()
        .await
        .expect("put_table_bucket_storage_class");

    let get = client
        .get_table_bucket_storage_class()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("get_table_bucket_storage_class");
    assert_eq!(
        get.storage_class_configuration()
            .unwrap()
            .storage_class()
            .as_str(),
        "STANDARD"
    );
}

#[tokio::test]
async fn test_table_bucket_maintenance_configuration() {
    use aws_sdk_s3tables::types::{
        TableBucketMaintenanceConfigurationValue, TableBucketMaintenanceType,
    };
    let client = make_client().await;

    let create = client
        .create_table_bucket()
        .name("maint-bucket")
        .send()
        .await
        .expect("create_table_bucket");
    let arn = create.arn().to_string();

    let value = TableBucketMaintenanceConfigurationValue::builder()
        .status(aws_sdk_s3tables::types::MaintenanceStatus::Enabled)
        .set_settings(None)
        .build();
    client
        .put_table_bucket_maintenance_configuration()
        .table_bucket_arn(&arn)
        .r#type(TableBucketMaintenanceType::IcebergUnreferencedFileRemoval)
        .value(value)
        .send()
        .await
        .expect("put_table_bucket_maintenance_configuration");

    let get = client
        .get_table_bucket_maintenance_configuration()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("get_table_bucket_maintenance_configuration");
    assert!(!get.configuration().is_empty());
}

#[tokio::test]
async fn test_table_bucket_replication() {
    use aws_sdk_s3tables::types::{
        ReplicationDestination, TableBucketReplicationConfiguration, TableBucketReplicationRule,
    };
    let client = make_client().await;

    let create = client
        .create_table_bucket()
        .name("repl-bucket")
        .send()
        .await
        .expect("create_table_bucket");
    let arn = create.arn().to_string();

    let dest = ReplicationDestination::builder()
        .destination_table_bucket_arn("arn:aws:s3tables:us-west-2:123456789012:bucket/dest")
        .build()
        .unwrap();
    let rule = TableBucketReplicationRule::builder()
        .destinations(dest)
        .build()
        .unwrap();
    let config = TableBucketReplicationConfiguration::builder()
        .role("arn:aws:iam::123456789012:role/ReplicationRole")
        .rules(rule)
        .build()
        .unwrap();
    client
        .put_table_bucket_replication()
        .table_bucket_arn(&arn)
        .configuration(config)
        .send()
        .await
        .expect("put_table_bucket_replication");

    let get = client
        .get_table_bucket_replication()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("get_table_bucket_replication");
    assert!(get.configuration().is_some());

    client
        .delete_table_bucket_replication()
        .table_bucket_arn(&arn)
        .send()
        .await
        .expect("delete_table_bucket_replication");
    // Note: SDK's correct_errors always synthesizes a non-None configuration,
    // so we just verify the delete call succeeds without error.
}

// ---------------------------------------------------------------------------
// Table extended operation tests
// ---------------------------------------------------------------------------

async fn setup_table(
    client: &aws_sdk_s3tables::Client,
    bucket_name: &str,
    ns: &str,
    table_name: &str,
) -> (String, String, String) {
    let create_bucket = client
        .create_table_bucket()
        .name(bucket_name)
        .send()
        .await
        .expect("create_table_bucket");
    let bucket_arn = create_bucket.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace(ns)
        .send()
        .await
        .expect("create_namespace");

    client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(ns)
        .name(table_name)
        .format(aws_sdk_s3tables::types::OpenTableFormat::Iceberg)
        .send()
        .await
        .expect("create_table");

    (bucket_arn, ns.to_string(), table_name.to_string())
}

#[tokio::test]
async fn test_table_policy() {
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "tpol-bucket", "myns", "my-table").await;

    let policy_json = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_table_policy()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .resource_policy(policy_json)
        .send()
        .await
        .expect("put_table_policy");

    let get = client
        .get_table_policy()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table_policy");
    assert_eq!(get.resource_policy(), policy_json);

    client
        .delete_table_policy()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("delete_table_policy");
}

#[tokio::test]
async fn test_table_maintenance_configuration() {
    use aws_sdk_s3tables::types::{TableMaintenanceConfigurationValue, TableMaintenanceType};
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "tmaint-bucket", "myns", "my-table").await;

    let value = TableMaintenanceConfigurationValue::builder()
        .status(aws_sdk_s3tables::types::MaintenanceStatus::Enabled)
        .set_settings(None)
        .build();
    client
        .put_table_maintenance_configuration()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .r#type(TableMaintenanceType::IcebergCompaction)
        .value(value)
        .send()
        .await
        .expect("put_table_maintenance_configuration");

    let get = client
        .get_table_maintenance_configuration()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table_maintenance_configuration");
    assert!(!get.configuration().is_empty());
}

#[tokio::test]
async fn test_table_maintenance_job_status() {
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "tjob-bucket", "myns", "my-table").await;

    let resp = client
        .get_table_maintenance_job_status()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table_maintenance_job_status");
    assert!(resp.table_arn().contains("my-table"));
}

#[tokio::test]
async fn test_table_metadata_location() {
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "tmeta-bucket", "myns", "my-table").await;

    // Get current state (warehouse_location and version_token)
    let get = client
        .get_table_metadata_location()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table_metadata_location");
    let version_token = get.version_token().to_string();
    assert!(get.warehouse_location().contains("my-table"));

    // Update metadata location
    let new_location = "s3://tmeta-bucket/myns/my-table/metadata/v1.metadata.json";
    let update = client
        .update_table_metadata_location()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .metadata_location(new_location)
        .version_token(&version_token)
        .send()
        .await
        .expect("update_table_metadata_location");
    assert_eq!(update.metadata_location(), new_location);
    assert_ne!(update.version_token(), version_token.as_str());
}

#[tokio::test]
async fn test_table_encryption() {
    use aws_sdk_s3tables::types::{EncryptionConfiguration, SseAlgorithm};
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "tenc-bucket", "myns", "my-table").await;

    // Set encryption on the bucket first
    let enc = EncryptionConfiguration::builder()
        .sse_algorithm(SseAlgorithm::Aes256)
        .build()
        .unwrap();
    client
        .put_table_bucket_encryption()
        .table_bucket_arn(&bucket_arn)
        .encryption_configuration(enc)
        .send()
        .await
        .expect("put_table_bucket_encryption");

    let get = client
        .get_table_encryption()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table_encryption");
    assert_eq!(
        get.encryption_configuration()
            .unwrap()
            .sse_algorithm()
            .as_str(),
        "AES256"
    );
}

#[tokio::test]
async fn test_table_storage_class() {
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "tsc-bucket", "myns", "my-table").await;

    let get_empty = client
        .get_table_storage_class()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table_storage_class empty");
    // SDK's correct_errors always synthesizes a StorageClassConfiguration, so we just
    // verify the call succeeds without error.
    let _ = (get_empty, bucket_arn, ns, name);
}

#[tokio::test]
async fn test_table_replication() {
    use aws_sdk_s3tables::types::{
        ReplicationDestination, TableReplicationConfiguration, TableReplicationRule,
    };
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "trepl-bucket", "myns", "my-table").await;

    // Get the table ARN
    let table = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table");
    let table_arn = table.table_arn().to_string();

    let dest = ReplicationDestination::builder()
        .destination_table_bucket_arn("arn:aws:s3tables:us-west-2:123456789012:bucket/dest")
        .build()
        .unwrap();
    let rule = TableReplicationRule::builder()
        .destinations(dest)
        .build()
        .unwrap();
    let config = TableReplicationConfiguration::builder()
        .role("arn:aws:iam::123456789012:role/ReplicationRole")
        .rules(rule)
        .build()
        .unwrap();
    client
        .put_table_replication()
        .table_arn(&table_arn)
        .configuration(config)
        .send()
        .await
        .expect("put_table_replication");

    let get = client
        .get_table_replication()
        .table_arn(&table_arn)
        .send()
        .await
        .expect("get_table_replication");
    // SDK's correct_errors always synthesizes a non-None configuration, so we
    // just verify get succeeds. Check the version_token is non-empty.
    let version_token = get.version_token().to_string();
    assert!(!version_token.is_empty());

    client
        .delete_table_replication()
        .table_arn(&table_arn)
        .version_token(&version_token)
        .send()
        .await
        .expect("delete_table_replication");
    // Note: SDK's correct_errors always synthesizes a non-None configuration after delete,
    // so we just verify the delete call succeeds without error.
}

#[tokio::test]
async fn test_table_replication_status() {
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "treplst-bucket", "myns", "my-table").await;

    let table = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table");
    let table_arn = table.table_arn().to_string();

    client
        .get_table_replication_status()
        .table_arn(&table_arn)
        .send()
        .await
        .expect("get_table_replication_status");
}

#[tokio::test]
async fn test_table_record_expiration() {
    use aws_sdk_s3tables::types::{
        TableRecordExpirationConfigurationValue, TableRecordExpirationSettings,
        TableRecordExpirationStatus,
    };
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "texp-bucket", "myns", "my-table").await;

    let table = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table");
    let table_arn = table.table_arn().to_string();

    let settings = TableRecordExpirationSettings::builder().days(30).build();
    let config = TableRecordExpirationConfigurationValue::builder()
        .status(TableRecordExpirationStatus::Enabled)
        .settings(settings)
        .build();
    client
        .put_table_record_expiration_configuration()
        .table_arn(&table_arn)
        .value(config)
        .send()
        .await
        .expect("put_table_record_expiration_configuration");

    let get = client
        .get_table_record_expiration_configuration()
        .table_arn(&table_arn)
        .send()
        .await
        .expect("get_table_record_expiration_configuration");
    assert!(get.configuration().is_some());

    let status = client
        .get_table_record_expiration_job_status()
        .table_arn(&table_arn)
        .send()
        .await
        .expect("get_table_record_expiration_job_status");
    let _ = status;
}

#[tokio::test]
async fn test_rename_table() {
    let client = make_client().await;
    let (bucket_arn, ns, name) = setup_table(&client, "rename-bucket", "myns", "old-name").await;

    // Get version token
    let table = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .send()
        .await
        .expect("get_table");
    let _version_token = table.version_token().to_string();

    client
        .rename_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name(&name)
        .new_name("new-name")
        .send()
        .await
        .expect("rename_table");

    // Old name should be gone
    let err = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name("old-name")
        .send()
        .await;
    assert!(err.is_err(), "old name should not exist");

    // New name should exist
    client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name("new-name")
        .send()
        .await
        .expect("new name should exist");
}

// ============================================================================
// Tests derived from AWS documentation: S3 Tables
// ============================================================================

// ---------------------------------------------------------------------------
// Conflict error tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_table_bucket_duplicate() {
    let client = make_client().await;

    client
        .create_table_bucket()
        .name("dup-bucket")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_table_bucket()
        .name("dup-bucket")
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
async fn test_create_namespace_duplicate() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("dup-ns-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("dupns")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("dupns")
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
async fn test_create_table_duplicate() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;
    let (bucket_arn, ns, _) = setup_table(&client, "dup-tbl-bucket", "myns", "duptable").await;

    let err = client
        .create_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name("duptable")
        .format(OpenTableFormat::Iceberg)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

// ---------------------------------------------------------------------------
// Not-found error tests for delete operations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_delete_table_bucket_not_found() {
    let client = make_client().await;

    let err = client
        .delete_table_bucket()
        .table_bucket_arn("arn:aws:s3tables:us-east-1:123456789012:bucket/nonexistent")
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
async fn test_delete_namespace_not_found() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("del-ns-notfound-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_namespace()
        .table_bucket_arn(bucket_resp.arn())
        .namespace("nonexistent")
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
async fn test_delete_table_not_found() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("del-tbl-notfound-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("ns")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("ns")
        .name("nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// ---------------------------------------------------------------------------
// Prefix filter tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_table_buckets_prefix() {
    let client = make_client().await;

    client
        .create_table_bucket()
        .name("prefix-alpha")
        .send()
        .await
        .unwrap();
    client
        .create_table_bucket()
        .name("prefix-beta")
        .send()
        .await
        .unwrap();
    client
        .create_table_bucket()
        .name("other-bucket")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_table_buckets()
        .prefix("prefix-")
        .send()
        .await
        .expect("list_table_buckets with prefix should succeed");

    let names: Vec<&str> = list_resp.table_buckets().iter().map(|b| b.name()).collect();
    assert!(
        names.contains(&"prefix-alpha"),
        "prefix-alpha should appear"
    );
    assert!(names.contains(&"prefix-beta"), "prefix-beta should appear");
    assert!(
        !names.contains(&"other-bucket"),
        "other-bucket should be filtered out"
    );
}

#[tokio::test]
async fn test_list_namespaces_prefix() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("ns-prefix-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("prod_db")
        .send()
        .await
        .unwrap();
    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("prod_events")
        .send()
        .await
        .unwrap();
    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("staging_db")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_namespaces()
        .table_bucket_arn(&bucket_arn)
        .prefix("prod_")
        .send()
        .await
        .expect("list_namespaces with prefix should succeed");

    let names: Vec<String> = list_resp
        .namespaces()
        .iter()
        .map(|n| n.namespace().join("."))
        .collect();
    assert!(
        names.iter().any(|n| n == "prod_db"),
        "prod_db should appear"
    );
    assert!(
        names.iter().any(|n| n == "prod_events"),
        "prod_events should appear"
    );
    assert!(
        !names.iter().any(|n| n == "staging_db"),
        "staging_db should be filtered out"
    );
}

#[tokio::test]
async fn test_list_tables_prefix() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("tbl-prefix-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    client
        .create_namespace()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .send()
        .await
        .unwrap();

    for name in ["events_raw", "events_processed", "users"] {
        client
            .create_table()
            .table_bucket_arn(&bucket_arn)
            .namespace("myns")
            .name(name)
            .format(OpenTableFormat::Iceberg)
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_tables()
        .table_bucket_arn(&bucket_arn)
        .prefix("events_")
        .send()
        .await
        .expect("list_tables with prefix should succeed");

    let names: Vec<&str> = list_resp.tables().iter().map(|t| t.name()).collect();
    assert!(names.contains(&"events_raw"), "events_raw should appear");
    assert!(
        names.contains(&"events_processed"),
        "events_processed should appear"
    );
    assert!(!names.contains(&"users"), "users should be filtered out");
}

#[tokio::test]
async fn test_list_tables_by_namespace() {
    use aws_sdk_s3tables::types::OpenTableFormat;

    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("tbl-ns-filter-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    for ns in ["ns_a", "ns_b"] {
        client
            .create_namespace()
            .table_bucket_arn(&bucket_arn)
            .namespace(ns)
            .send()
            .await
            .unwrap();
    }

    for (ns, name) in [
        ("ns_a", "table_x"),
        ("ns_a", "table_y"),
        ("ns_b", "table_z"),
    ] {
        client
            .create_table()
            .table_bucket_arn(&bucket_arn)
            .namespace(ns)
            .name(name)
            .format(OpenTableFormat::Iceberg)
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_tables()
        .table_bucket_arn(&bucket_arn)
        .namespace("ns_a")
        .send()
        .await
        .expect("list_tables with namespace should succeed");

    let names: Vec<&str> = list_resp.tables().iter().map(|t| t.name()).collect();
    assert!(names.contains(&"table_x"), "table_x should appear");
    assert!(names.contains(&"table_y"), "table_y should appear");
    assert!(
        !names.contains(&"table_z"),
        "table_z should be filtered out"
    );
}

// ---------------------------------------------------------------------------
// Metrics configuration test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_table_bucket_metrics_configuration() {
    let client = make_client().await;

    let bucket_resp = client
        .create_table_bucket()
        .name("metrics-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.arn().to_string();

    // Put metrics config
    client
        .put_table_bucket_metrics_configuration()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await
        .expect("put_table_bucket_metrics_configuration should succeed");

    // Get metrics config
    let get_resp = client
        .get_table_bucket_metrics_configuration()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await
        .expect("get_table_bucket_metrics_configuration should succeed");
    assert!(
        !get_resp.table_bucket_arn().is_empty(),
        "table_bucket_arn should be present"
    );

    // Delete metrics config
    client
        .delete_table_bucket_metrics_configuration()
        .table_bucket_arn(&bucket_arn)
        .send()
        .await
        .expect("delete_table_bucket_metrics_configuration should succeed");
}

// ---------------------------------------------------------------------------
// Rename conflict test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_rename_table_conflict() {
    let client = make_client().await;
    let (bucket_arn, ns, _) =
        setup_table(&client, "rename-conflict-bucket", "myns", "table_a").await;

    // Create a second table
    {
        use aws_sdk_s3tables::types::OpenTableFormat;
        client
            .create_table()
            .table_bucket_arn(&bucket_arn)
            .namespace(&ns)
            .name("table_b")
            .format(OpenTableFormat::Iceberg)
            .send()
            .await
            .unwrap();
    }

    // Try to rename table_a to table_b — should conflict
    let err = client
        .rename_table()
        .table_bucket_arn(&bucket_arn)
        .namespace(&ns)
        .name("table_a")
        .new_name("table_b")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

// ---------------------------------------------------------------------------
// Tagging on Table (not just TableBucket)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_tag_table() {
    let client = make_client().await;
    let (_, _, _) = setup_table(&client, "tag-tbl-bucket", "myns", "tagged_table").await;

    // Get the table ARN
    let bucket_arn = {
        let list = client
            .list_table_buckets()
            .prefix("tag-tbl-bucket")
            .send()
            .await
            .unwrap();
        list.table_buckets()[0].arn().to_string()
    };
    let create_resp = client
        .get_table()
        .table_bucket_arn(&bucket_arn)
        .namespace("myns")
        .name("tagged_table")
        .send()
        .await
        .unwrap();
    let table_arn = create_resp.table_arn().to_string();

    // Tag the table
    client
        .tag_resource()
        .resource_arn(&table_arn)
        .tags("env", "prod")
        .tags("owner", "team-a")
        .send()
        .await
        .expect("tag_resource on table should succeed");

    // List tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&table_arn)
        .send()
        .await
        .expect("list_tags_for_resource on table should succeed");

    let empty = std::collections::HashMap::new();
    let tags = tags_resp.tags().unwrap_or(&empty);
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("owner").map(|s| s.as_str()), Some("team-a"));

    // Untag
    client
        .untag_resource()
        .resource_arn(&table_arn)
        .tag_keys("owner")
        .send()
        .await
        .expect("untag_resource on table should succeed");

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&table_arn)
        .send()
        .await
        .unwrap();
    let tags2 = tags_resp2.tags().unwrap_or(&empty);
    assert!(tags2.contains_key("env"), "env tag should remain");
    assert!(!tags2.contains_key("owner"), "owner tag should be removed");
}

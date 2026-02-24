use aws_sdk_keyspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_keyspaces::{KeyspacesService, KeyspacesStateView};

async fn make_client() -> aws_sdk_keyspaces::Client {
    let mock = MockAws::builder()
        .with_service(KeyspacesService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_keyspaces::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_keyspaces::Client::new(&config)
}

// ---------- Keyspace CRUD ----------

#[tokio::test]
async fn test_create_and_get_keyspace() {
    let client = make_client().await;

    let create_resp = client
        .create_keyspace()
        .keyspace_name("my_ks")
        .send()
        .await
        .expect("create_keyspace should succeed");
    assert!(create_resp.resource_arn().contains("my_ks"));

    let get_resp = client
        .get_keyspace()
        .keyspace_name("my_ks")
        .send()
        .await
        .expect("get_keyspace should succeed");
    assert_eq!(get_resp.keyspace_name(), "my_ks");
    assert_eq!(get_resp.replication_strategy().as_str(), "SINGLE_REGION");
}

#[tokio::test]
async fn test_create_keyspace_duplicate() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("dup_ks")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_keyspace()
        .keyspace_name("dup_ks")
        .send()
        .await
        .expect_err("duplicate create should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("Conflict"),
        "expected ConflictException: {msg}"
    );
}

#[tokio::test]
async fn test_delete_keyspace() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("del_ks")
        .send()
        .await
        .unwrap();

    client
        .delete_keyspace()
        .keyspace_name("del_ks")
        .send()
        .await
        .expect("delete should succeed");

    let err = client
        .get_keyspace()
        .keyspace_name("del_ks")
        .send()
        .await
        .expect_err("get after delete should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("ResourceNotFound"),
        "expected ResourceNotFoundException: {msg}"
    );
}

#[tokio::test]
async fn test_list_keyspaces() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("ks_a")
        .send()
        .await
        .unwrap();
    client
        .create_keyspace()
        .keyspace_name("ks_b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_keyspaces()
        .send()
        .await
        .expect("list should succeed");
    let names: Vec<_> = resp
        .keyspaces()
        .iter()
        .map(|ks| ks.keyspace_name().to_string())
        .collect();
    assert!(names.contains(&"ks_a".to_string()));
    assert!(names.contains(&"ks_b".to_string()));
}

#[tokio::test]
async fn test_update_keyspace() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("upd_ks")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_keyspace()
        .keyspace_name("upd_ks")
        .replication_specification(
            aws_sdk_keyspaces::types::ReplicationSpecification::builder()
                .replication_strategy(aws_sdk_keyspaces::types::Rs::MultiRegion)
                .region_list("us-east-1")
                .region_list("us-west-2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update should succeed");
    assert!(resp.resource_arn().contains("upd_ks"));

    let get_resp = client
        .get_keyspace()
        .keyspace_name("upd_ks")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.replication_strategy().as_str(), "MULTI_REGION");
}

// ---------- Table CRUD ----------

#[tokio::test]
async fn test_create_and_get_table() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("tbl_ks")
        .send()
        .await
        .unwrap();

    let schema = aws_sdk_keyspaces::types::SchemaDefinition::builder()
        .all_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("id")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .partition_keys(
            aws_sdk_keyspaces::types::PartitionKey::builder()
                .name("id")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let resp = client
        .create_table()
        .keyspace_name("tbl_ks")
        .table_name("my_table")
        .schema_definition(schema)
        .send()
        .await
        .expect("create_table should succeed");
    assert!(resp.resource_arn().contains("my_table"));

    let get = client
        .get_table()
        .keyspace_name("tbl_ks")
        .table_name("my_table")
        .send()
        .await
        .expect("get_table should succeed");
    assert_eq!(get.keyspace_name(), "tbl_ks");
    assert_eq!(get.table_name(), "my_table");
    assert_eq!(
        get.status().expect("status should be set").as_str(),
        "ACTIVE"
    );
}

#[tokio::test]
async fn test_delete_table() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("del_tbl_ks")
        .send()
        .await
        .unwrap();

    let schema = aws_sdk_keyspaces::types::SchemaDefinition::builder()
        .all_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("pk")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .partition_keys(
            aws_sdk_keyspaces::types::PartitionKey::builder()
                .name("pk")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_table()
        .keyspace_name("del_tbl_ks")
        .table_name("del_tbl")
        .schema_definition(schema)
        .send()
        .await
        .unwrap();

    client
        .delete_table()
        .keyspace_name("del_tbl_ks")
        .table_name("del_tbl")
        .send()
        .await
        .expect("delete_table should succeed");

    let err = client
        .get_table()
        .keyspace_name("del_tbl_ks")
        .table_name("del_tbl")
        .send()
        .await
        .expect_err("get after delete should fail");
    assert!(format!("{err:?}").contains("ResourceNotFound"));
}

#[tokio::test]
async fn test_list_tables() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("list_ks")
        .send()
        .await
        .unwrap();

    for name in &["tbl_1", "tbl_2"] {
        let schema = aws_sdk_keyspaces::types::SchemaDefinition::builder()
            .all_columns(
                aws_sdk_keyspaces::types::ColumnDefinition::builder()
                    .name("pk")
                    .r#type("text")
                    .build()
                    .unwrap(),
            )
            .partition_keys(
                aws_sdk_keyspaces::types::PartitionKey::builder()
                    .name("pk")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        client
            .create_table()
            .keyspace_name("list_ks")
            .table_name(*name)
            .schema_definition(schema)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_tables()
        .keyspace_name("list_ks")
        .send()
        .await
        .expect("list_tables should succeed");
    let names: Vec<_> = resp
        .tables()
        .iter()
        .map(|t| t.table_name().to_string())
        .collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"tbl_1".to_string()));
    assert!(names.contains(&"tbl_2".to_string()));
}

// ---------- Tag operations ----------

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_client().await;

    let resp = client
        .create_keyspace()
        .keyspace_name("tag_ks")
        .send()
        .await
        .unwrap();
    let arn = resp.resource_arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_keyspaces::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");
    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), "test");

    // Untag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_keyspaces::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("untag should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(tags_resp.tags().is_empty());
}

// ---------- Restore table ----------

#[tokio::test]
async fn test_restore_table() {
    let client = make_client().await;

    client
        .create_keyspace()
        .keyspace_name("restore_ks")
        .send()
        .await
        .unwrap();

    let schema = aws_sdk_keyspaces::types::SchemaDefinition::builder()
        .all_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("pk")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .partition_keys(
            aws_sdk_keyspaces::types::PartitionKey::builder()
                .name("pk")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_table()
        .keyspace_name("restore_ks")
        .table_name("src_table")
        .schema_definition(schema)
        .send()
        .await
        .unwrap();

    let resp = client
        .restore_table()
        .source_keyspace_name("restore_ks")
        .source_table_name("src_table")
        .target_keyspace_name("restore_ks")
        .target_table_name("dst_table")
        .send()
        .await
        .expect("restore_table should succeed");
    assert!(resp.restored_table_arn().contains("dst_table"));

    // Verify the restored table exists
    let get = client
        .get_table()
        .keyspace_name("restore_ks")
        .table_name("dst_table")
        .send()
        .await
        .expect("get restored table should succeed");
    assert_eq!(get.table_name(), "dst_table");
}

// ---------- Error paths ----------

#[tokio::test]
async fn test_get_nonexistent_keyspace() {
    let client = make_client().await;

    let err = client
        .get_keyspace()
        .keyspace_name("no_such_ks")
        .send()
        .await
        .expect_err("should fail");
    assert!(format!("{err:?}").contains("ResourceNotFound"));
}

#[tokio::test]
async fn test_create_table_in_nonexistent_keyspace() {
    let client = make_client().await;

    let schema = aws_sdk_keyspaces::types::SchemaDefinition::builder()
        .all_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("pk")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .partition_keys(
            aws_sdk_keyspaces::types::PartitionKey::builder()
                .name("pk")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let err = client
        .create_table()
        .keyspace_name("no_such_ks")
        .table_name("tbl")
        .schema_definition(schema)
        .send()
        .await
        .expect_err("should fail");
    assert!(format!("{err:?}").contains("ResourceNotFound"));
}

// ---------- State views ----------

#[tokio::test]
async fn test_snapshot_restore() {
    use winterbaume_core::StatefulService;
    let svc = KeyspacesService::new();

    // Create some state via a view
    let mut initial_view = KeyspacesStateView::default();
    initial_view.keyspaces.insert(
        "snap_ks".to_string(),
        winterbaume_keyspaces::views::KeyspaceView {
            name: "snap_ks".to_string(),
            arn: "arn:aws:cassandra:us-east-1:123456789012:/keyspace/snap_ks/".to_string(),
            replication_strategy: "SINGLE_REGION".to_string(),
            replication_regions: vec![],
            tags: Default::default(),
            creation_timestamp: None,
            status: "ACTIVE".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", initial_view)
        .await
        .unwrap();

    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(view.keyspaces.contains_key("snap_ks"));

    // Restore to different scope
    svc.restore("123456789012", "eu-west-1", view.clone())
        .await
        .unwrap();
    let view2 = svc.snapshot("123456789012", "eu-west-1").await;
    assert!(view2.keyspaces.contains_key("snap_ks"));
}

#[tokio::test]
async fn test_merge_additive() {
    use winterbaume_core::StatefulService;
    let svc = KeyspacesService::new();

    // Create initial state via restore
    let mut initial = KeyspacesStateView::default();
    initial.keyspaces.insert(
        "existing_ks".to_string(),
        winterbaume_keyspaces::views::KeyspaceView {
            name: "existing_ks".to_string(),
            arn: "arn:aws:cassandra:us-east-1:123456789012:/keyspace/existing_ks/".to_string(),
            replication_strategy: "SINGLE_REGION".to_string(),
            replication_regions: vec![],
            tags: Default::default(),
            creation_timestamp: None,
            status: "ACTIVE".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", initial)
        .await
        .unwrap();

    // Merge new keyspace
    let mut view = KeyspacesStateView::default();
    view.keyspaces.insert(
        "merged_ks".to_string(),
        winterbaume_keyspaces::views::KeyspaceView {
            name: "merged_ks".to_string(),
            arn: "arn:aws:cassandra:us-east-1:123456789012:/keyspace/merged_ks/".to_string(),
            replication_strategy: "SINGLE_REGION".to_string(),
            replication_regions: vec![],
            tags: Default::default(),
            creation_timestamp: None,
            status: "ACTIVE".to_string(),
        },
    );

    svc.merge("123456789012", "us-east-1", view).await.unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    // Both keyspaces should exist
    assert!(snap.keyspaces.contains_key("existing_ks"));
    assert!(snap.keyspaces.contains_key("merged_ks"));
}

// ---------- State change notifications ----------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    let svc = KeyspacesService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
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
    let svc = KeyspacesService::new();

    // Pre-seed state
    let mut view = KeyspacesStateView::default();
    view.keyspaces.insert(
        "notified_ks".to_string(),
        winterbaume_keyspaces::views::KeyspaceView {
            name: "notified_ks".to_string(),
            arn: "arn:aws:cassandra:us-east-1:123456789012:/keyspace/notified_ks/".to_string(),
            replication_strategy: "SINGLE_REGION".to_string(),
            replication_regions: vec![],
            tags: Default::default(),
            creation_timestamp: None,
            status: "ACTIVE".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Re-register and capture snapshot
    let snapshots: Arc<Mutex<Vec<KeyspacesStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut view2 = KeyspacesStateView::default();
    view2.keyspaces.insert(
        "notified_ks_2".to_string(),
        winterbaume_keyspaces::views::KeyspaceView {
            name: "notified_ks_2".to_string(),
            arn: "arn:aws:cassandra:us-east-1:123456789012:/keyspace/notified_ks_2/".to_string(),
            replication_strategy: "SINGLE_REGION".to_string(),
            replication_regions: vec![],
            tags: Default::default(),
            creation_timestamp: None,
            status: "ACTIVE".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view2)
        .await
        .unwrap();
    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].keyspaces.contains_key("notified_ks_2"));
}

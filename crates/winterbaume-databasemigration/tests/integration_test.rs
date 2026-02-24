use aws_sdk_databasemigration::config::BehaviorVersion;
use aws_sdk_databasemigration::types::{MigrationTypeValue, ReplicationEndpointTypeValue, Tag};
use winterbaume_core::MockAws;
use winterbaume_databasemigration::DatabaseMigrationService as DmsService;

async fn make_client() -> aws_sdk_databasemigration::Client {
    let mock = MockAws::builder().with_service(DmsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_databasemigration::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_databasemigration::Client::new(&config)
}

fn tag(key: &str, value: &str) -> Tag {
    Tag::builder().key(key).value(value).build()
}

// ---- ReplicationInstance tests ----

#[tokio::test]
async fn test_create_and_describe_replication_instance() {
    let client = make_client().await;

    let create_resp = client
        .create_replication_instance()
        .replication_instance_identifier("my-rep-instance")
        .replication_instance_class("dms.r5.large")
        .allocated_storage(100)
        .send()
        .await
        .expect("create_replication_instance should succeed");

    let instance = create_resp
        .replication_instance()
        .expect("response should contain replication instance");
    assert_eq!(
        instance.replication_instance_identifier().unwrap(),
        "my-rep-instance"
    );
    assert_eq!(
        instance.replication_instance_class().unwrap(),
        "dms.r5.large"
    );
    assert_eq!(instance.allocated_storage(), 100);
    assert_eq!(instance.replication_instance_status().unwrap(), "available");
    let arn = instance
        .replication_instance_arn()
        .expect("should have ARN");
    assert!(arn.starts_with("arn:aws:dms:us-east-1:"));

    // DescribeReplicationInstances
    let desc_resp = client
        .describe_replication_instances()
        .send()
        .await
        .unwrap();
    let instances = desc_resp.replication_instances();
    assert_eq!(instances.len(), 1);
    assert_eq!(
        instances[0].replication_instance_identifier().unwrap(),
        "my-rep-instance"
    );
}

#[tokio::test]
async fn test_delete_replication_instance() {
    let client = make_client().await;

    // Create first
    let create_resp = client
        .create_replication_instance()
        .replication_instance_identifier("to-delete")
        .replication_instance_class("dms.t3.medium")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .replication_instance()
        .unwrap()
        .replication_instance_arn()
        .unwrap()
        .to_string();

    // Delete
    client
        .delete_replication_instance()
        .replication_instance_arn(&arn)
        .send()
        .await
        .expect("delete_replication_instance should succeed");

    // Describe should be empty
    let desc_resp = client
        .describe_replication_instances()
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.replication_instances().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_replication_instance() {
    let client = make_client().await;
    let err = client
        .delete_replication_instance()
        .replication_instance_arn("arn:aws:dms:us-east-1:123:rep:nonexistent")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFound") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_modify_replication_instance() {
    let client = make_client().await;

    let create_resp = client
        .create_replication_instance()
        .replication_instance_identifier("modify-me")
        .replication_instance_class("dms.t3.medium")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .replication_instance()
        .unwrap()
        .replication_instance_arn()
        .unwrap()
        .to_string();

    let modify_resp = client
        .modify_replication_instance()
        .replication_instance_arn(&arn)
        .replication_instance_class("dms.r5.large")
        .multi_az(true)
        .send()
        .await
        .unwrap();
    let instance = modify_resp.replication_instance().unwrap();
    assert_eq!(
        instance.replication_instance_class().unwrap(),
        "dms.r5.large"
    );
    assert!(instance.multi_az());
}

// ---- Endpoint tests ----

#[tokio::test]
async fn test_create_and_describe_endpoints() {
    let client = make_client().await;

    let create_resp = client
        .create_endpoint()
        .endpoint_identifier("my-source")
        .endpoint_type(ReplicationEndpointTypeValue::Source)
        .engine_name("mysql")
        .server_name("db.example.com")
        .port(3306)
        .database_name("mydb")
        .username("admin")
        .send()
        .await
        .expect("create_endpoint should succeed");

    let endpoint = create_resp.endpoint().expect("should contain endpoint");
    assert_eq!(endpoint.endpoint_identifier().unwrap(), "my-source");
    assert_eq!(
        endpoint.endpoint_type().unwrap(),
        &ReplicationEndpointTypeValue::Source
    );
    assert_eq!(endpoint.engine_name().unwrap(), "mysql");
    assert_eq!(endpoint.status().unwrap(), "active");
    let arn = endpoint.endpoint_arn().unwrap();
    assert!(arn.starts_with("arn:aws:dms:us-east-1:"));

    // DescribeEndpoints
    let desc_resp = client.describe_endpoints().send().await.unwrap();
    let endpoints = desc_resp.endpoints();
    assert_eq!(endpoints.len(), 1);
    assert_eq!(endpoints[0].endpoint_identifier().unwrap(), "my-source");
}

#[tokio::test]
async fn test_delete_endpoint() {
    let client = make_client().await;

    let create_resp = client
        .create_endpoint()
        .endpoint_identifier("ep-to-delete")
        .endpoint_type(ReplicationEndpointTypeValue::Target)
        .engine_name("postgres")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .endpoint()
        .unwrap()
        .endpoint_arn()
        .unwrap()
        .to_string();

    client
        .delete_endpoint()
        .endpoint_arn(&arn)
        .send()
        .await
        .expect("delete_endpoint should succeed");

    let desc_resp = client.describe_endpoints().send().await.unwrap();
    assert_eq!(desc_resp.endpoints().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_endpoint() {
    let client = make_client().await;
    let err = client
        .delete_endpoint()
        .endpoint_arn("arn:aws:dms:us-east-1:123:endpoint:nonexistent")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFound") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_modify_endpoint() {
    let client = make_client().await;

    let create_resp = client
        .create_endpoint()
        .endpoint_identifier("ep-to-modify")
        .endpoint_type(ReplicationEndpointTypeValue::Source)
        .engine_name("mysql")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .endpoint()
        .unwrap()
        .endpoint_arn()
        .unwrap()
        .to_string();

    let modify_resp = client
        .modify_endpoint()
        .endpoint_arn(&arn)
        .server_name("new-db.example.com")
        .port(5432)
        .send()
        .await
        .unwrap();
    let endpoint = modify_resp.endpoint().unwrap();
    assert_eq!(endpoint.server_name().unwrap(), "new-db.example.com");
    assert_eq!(endpoint.port(), Some(5432));
}

// ---- ReplicationTask tests ----

async fn setup_endpoints(client: &aws_sdk_databasemigration::Client) -> (String, String) {
    let src = client
        .create_endpoint()
        .endpoint_identifier("source-ep")
        .endpoint_type(ReplicationEndpointTypeValue::Source)
        .engine_name("mysql")
        .send()
        .await
        .unwrap();
    let tgt = client
        .create_endpoint()
        .endpoint_identifier("target-ep")
        .endpoint_type(ReplicationEndpointTypeValue::Target)
        .engine_name("postgres")
        .send()
        .await
        .unwrap();
    let src_arn = src.endpoint().unwrap().endpoint_arn().unwrap().to_string();
    let tgt_arn = tgt.endpoint().unwrap().endpoint_arn().unwrap().to_string();
    (src_arn, tgt_arn)
}

async fn setup_instance(client: &aws_sdk_databasemigration::Client) -> String {
    let resp = client
        .create_replication_instance()
        .replication_instance_identifier("rep-inst")
        .replication_instance_class("dms.t3.medium")
        .send()
        .await
        .unwrap();
    resp.replication_instance()
        .unwrap()
        .replication_instance_arn()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_and_describe_replication_task() {
    let client = make_client().await;
    let (src_arn, tgt_arn) = setup_endpoints(&client).await;
    let inst_arn = setup_instance(&client).await;

    let create_resp = client
        .create_replication_task()
        .replication_task_identifier("my-task")
        .source_endpoint_arn(&src_arn)
        .target_endpoint_arn(&tgt_arn)
        .replication_instance_arn(&inst_arn)
        .migration_type(MigrationTypeValue::FullLoad)
        .table_mappings(r#"{"rules":[]}"#)
        .send()
        .await
        .unwrap();

    let task = create_resp
        .replication_task()
        .expect("should contain replication task");
    assert_eq!(task.replication_task_identifier().unwrap(), "my-task");
    assert_eq!(
        task.migration_type().unwrap(),
        &MigrationTypeValue::FullLoad
    );
    assert_eq!(task.status().unwrap(), "ready");
    let task_arn = task.replication_task_arn().unwrap();
    assert!(task_arn.starts_with("arn:aws:dms:us-east-1:"));

    // DescribeReplicationTasks
    let desc_resp = client.describe_replication_tasks().send().await.unwrap();
    let tasks = desc_resp.replication_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].replication_task_identifier().unwrap(), "my-task");
}

#[tokio::test]
async fn test_start_stop_replication_task() {
    let client = make_client().await;
    let (src_arn, tgt_arn) = setup_endpoints(&client).await;
    let inst_arn = setup_instance(&client).await;

    let create_resp = client
        .create_replication_task()
        .replication_task_identifier("task-to-start")
        .source_endpoint_arn(&src_arn)
        .target_endpoint_arn(&tgt_arn)
        .replication_instance_arn(&inst_arn)
        .migration_type(MigrationTypeValue::Cdc)
        .table_mappings(r#"{"rules":[]}"#)
        .send()
        .await
        .unwrap();
    let task_arn = create_resp
        .replication_task()
        .unwrap()
        .replication_task_arn()
        .unwrap()
        .to_string();

    // Start
    let start_resp = client
        .start_replication_task()
        .replication_task_arn(&task_arn)
        .start_replication_task_type(
            aws_sdk_databasemigration::types::StartReplicationTaskTypeValue::StartReplication,
        )
        .send()
        .await
        .unwrap();
    assert_eq!(
        start_resp.replication_task().unwrap().status().unwrap(),
        "running"
    );

    // Stop
    let stop_resp = client
        .stop_replication_task()
        .replication_task_arn(&task_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        stop_resp.replication_task().unwrap().status().unwrap(),
        "stopped"
    );
}

#[tokio::test]
async fn test_delete_replication_task() {
    let client = make_client().await;
    let (src_arn, tgt_arn) = setup_endpoints(&client).await;
    let inst_arn = setup_instance(&client).await;

    let create_resp = client
        .create_replication_task()
        .replication_task_identifier("del-task")
        .source_endpoint_arn(&src_arn)
        .target_endpoint_arn(&tgt_arn)
        .replication_instance_arn(&inst_arn)
        .migration_type(MigrationTypeValue::FullLoad)
        .table_mappings(r#"{"rules":[]}"#)
        .send()
        .await
        .unwrap();
    let task_arn = create_resp
        .replication_task()
        .unwrap()
        .replication_task_arn()
        .unwrap()
        .to_string();

    client
        .delete_replication_task()
        .replication_task_arn(&task_arn)
        .send()
        .await
        .expect("delete_replication_task should succeed");

    let desc_resp = client.describe_replication_tasks().send().await.unwrap();
    assert_eq!(desc_resp.replication_tasks().len(), 0);
}

// ---- Connections tests ----

#[tokio::test]
async fn test_test_connection() {
    let client = make_client().await;
    let (src_arn, _) = setup_endpoints(&client).await;
    let inst_arn = setup_instance(&client).await;

    let test_resp = client
        .test_connection()
        .replication_instance_arn(&inst_arn)
        .endpoint_arn(&src_arn)
        .send()
        .await
        .unwrap();

    let conn = test_resp.connection().expect("should contain connection");
    assert_eq!(conn.status().unwrap(), "successful");
    assert_eq!(conn.replication_instance_arn().unwrap(), inst_arn);
    assert_eq!(conn.endpoint_arn().unwrap(), src_arn);
}

#[tokio::test]
async fn test_describe_connections() {
    let client = make_client().await;
    let (src_arn, _) = setup_endpoints(&client).await;
    let inst_arn = setup_instance(&client).await;

    // Test a connection first
    client
        .test_connection()
        .replication_instance_arn(&inst_arn)
        .endpoint_arn(&src_arn)
        .send()
        .await
        .unwrap();

    let desc_resp = client.describe_connections().send().await.unwrap();
    let connections = desc_resp.connections();
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].status().unwrap(), "successful");
}

// ---- Tags tests ----

#[tokio::test]
async fn test_add_list_remove_tags() {
    let client = make_client().await;

    let create_resp = client
        .create_replication_instance()
        .replication_instance_identifier("tagged-instance")
        .replication_instance_class("dms.t3.medium")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .replication_instance()
        .unwrap()
        .replication_instance_arn()
        .unwrap()
        .to_string();

    // Add tags
    client
        .add_tags_to_resource()
        .resource_arn(&arn)
        .tags(tag("Environment", "test"))
        .tags(tag("Project", "myproject"))
        .send()
        .await
        .expect("add_tags_to_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.tag_list().len(), 2);

    // Remove one tag
    client
        .remove_tags_from_resource()
        .resource_arn(&arn)
        .tag_keys("Environment")
        .send()
        .await
        .expect("remove_tags_from_resource should succeed");

    // List again
    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tag_list2 = list_resp2.tag_list();
    assert_eq!(tag_list2.len(), 1);
    assert_eq!(tag_list2[0].key().unwrap(), "Project");
}

// ---- Subnet group tests ----

#[tokio::test]
async fn test_create_and_describe_replication_subnet_groups() {
    let client = make_client().await;

    let create_resp = client
        .create_replication_subnet_group()
        .replication_subnet_group_identifier("my-subnet-grp")
        .replication_subnet_group_description("Test subnet group")
        .subnet_ids("subnet-aaa")
        .subnet_ids("subnet-bbb")
        .send()
        .await
        .unwrap();

    let group = create_resp
        .replication_subnet_group()
        .expect("should contain subnet group");
    assert_eq!(
        group.replication_subnet_group_identifier().unwrap(),
        "my-subnet-grp"
    );
    assert_eq!(group.subnet_group_status().unwrap(), "Complete");

    // Describe
    let desc_resp = client
        .describe_replication_subnet_groups()
        .send()
        .await
        .unwrap();
    let groups = desc_resp.replication_subnet_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(
        groups[0].replication_subnet_group_identifier().unwrap(),
        "my-subnet-grp"
    );
}

#[tokio::test]
async fn test_delete_replication_subnet_group() {
    let client = make_client().await;

    client
        .create_replication_subnet_group()
        .replication_subnet_group_identifier("grp-to-delete")
        .replication_subnet_group_description("desc")
        .subnet_ids("subnet-1")
        .send()
        .await
        .unwrap();

    client
        .delete_replication_subnet_group()
        .replication_subnet_group_identifier("grp-to-delete")
        .send()
        .await
        .expect("delete_replication_subnet_group should succeed");

    let desc_resp = client
        .describe_replication_subnet_groups()
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.replication_subnet_groups().len(), 0);
}

// ---- State views tests ----

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use bytes::Bytes;
    use serde_json::json;
    use winterbaume_core::{MockRequest, MockService, StatefulService};
    use winterbaume_databasemigration::DmsStateView;

    // Snapshot/restore exercises the lower-level StatefulService trait, which
    // is not surfaced by the SDK client. Drive the service handle directly via
    // its dispatch path ( the same path the SDK would take ) for the seed step.
    let service = DmsService::new();

    let mut headers = http::HeaderMap::new();
    headers.insert(
        "x-amz-target",
        "AmazonDMSv20160101.CreateReplicationInstance"
            .parse()
            .unwrap(),
    );
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.1".parse().unwrap(),
    );
    let body = json!({
        "ReplicationInstanceIdentifier": "view-instance",
        "ReplicationInstanceClass": "dms.t3.medium"
    });
    let req = MockRequest {
        method: "POST".to_string(),
        uri: "https://dms.us-east-1.amazonaws.com".to_string(),
        headers,
        body: Bytes::from(body.to_string()),
    };
    service.handle(req).await;

    // Snapshot
    let snapshot = service.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.replication_instances.contains_key("view-instance"));

    // New service, restore
    let service2 = DmsService::new();
    service2
        .restore("123456789012", "us-east-1", snapshot)
        .await
        .expect("restore should succeed");

    // Verify
    let snapshot2 = service2.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot2
            .replication_instances
            .contains_key("view-instance")
    );

    // Empty state view - restore empty
    service2
        .restore("123456789012", "us-east-1", DmsStateView::default())
        .await
        .expect("restore empty should succeed");
    let snapshot3 = service2.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot3.replication_instances.is_empty());
}

#[tokio::test]
async fn test_state_view_merge() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_databasemigration::DmsStateView;
    use winterbaume_databasemigration::views::ReplicationInstanceView;

    let service = DmsService::new();

    // Pre-seed with one instance
    let mut initial_instances = HashMap::new();
    initial_instances.insert(
        "inst-a".to_string(),
        ReplicationInstanceView {
            replication_instance_identifier: "inst-a".to_string(),
            replication_instance_class: "dms.t3.medium".to_string(),
            allocated_storage: 50,
            status: "available".to_string(),
            replication_instance_arn: "arn:aws:dms:us-east-1:123:rep:inst-a".to_string(),
            availability_zone: None,
            publicly_accessible: true,
            multi_az: false,
            engine_version: None,
            instance_create_time: 1_000_000.0,
            tags: HashMap::new(),
        },
    );
    service
        .restore(
            "123456789012",
            "us-east-1",
            DmsStateView {
                replication_instances: initial_instances,
                ..Default::default()
            },
        )
        .await
        .unwrap();

    // Merge in a second instance
    let mut merge_instances = HashMap::new();
    merge_instances.insert(
        "inst-b".to_string(),
        ReplicationInstanceView {
            replication_instance_identifier: "inst-b".to_string(),
            replication_instance_class: "dms.r5.large".to_string(),
            allocated_storage: 100,
            status: "available".to_string(),
            replication_instance_arn: "arn:aws:dms:us-east-1:123:rep:inst-b".to_string(),
            availability_zone: None,
            publicly_accessible: false,
            multi_az: true,
            engine_version: None,
            instance_create_time: 2_000_000.0,
            tags: HashMap::new(),
        },
    );
    service
        .merge(
            "123456789012",
            "us-east-1",
            DmsStateView {
                replication_instances: merge_instances,
                ..Default::default()
            },
        )
        .await
        .unwrap();

    // Both instances should be present
    let snapshot = service.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.replication_instances.contains_key("inst-a"));
    assert!(snapshot.replication_instances.contains_key("inst-b"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_databasemigration::{DatabaseMigrationService as DmsService, DmsStateView};

    let svc = DmsService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", DmsStateView::default())
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
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_databasemigration::views::ReplicationInstanceView;
    use winterbaume_databasemigration::{DatabaseMigrationService as DmsService, DmsStateView};

    let svc = DmsService::new();

    // Pre-seed
    let mut instances = HashMap::new();
    instances.insert(
        "notified-inst".to_string(),
        ReplicationInstanceView {
            replication_instance_identifier: "notified-inst".to_string(),
            replication_instance_class: "dms.t3.medium".to_string(),
            allocated_storage: 50,
            status: "available".to_string(),
            replication_instance_arn: "arn:aws:dms:us-east-1:123:rep:notified-inst".to_string(),
            availability_zone: None,
            publicly_accessible: true,
            multi_az: false,
            engine_version: None,
            instance_create_time: 1_000_000.0,
            tags: HashMap::new(),
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        DmsStateView {
            replication_instances: instances.clone(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    // Re-register and capture snapshot
    let snapshots: Arc<Mutex<Vec<DmsStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        DmsStateView {
            replication_instances: instances,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].replication_instances.contains_key("notified-inst"));
}

// ---- Missing fields error tests ----

#[tokio::test]
async fn test_create_replication_instance_missing_identifier() {
    let client = make_client().await;
    // Use the typed `set_*` setter to explicitly omit the identifier
    // ( None serialises as a missing field in the JSON body ).
    let err = client
        .create_replication_instance()
        .set_replication_instance_identifier(None)
        .replication_instance_class("dms.t3.medium")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException")
            || err_str.contains("Missing")
            || err_str.contains("Invalid"),
        "Expected validation error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_endpoint_missing_fields() {
    let client = make_client().await;
    // EngineName is required by the handler but the SDK builder allows
    // omitting it; the handler should return a validation fault.
    let err = client
        .create_endpoint()
        .endpoint_identifier("ep")
        .endpoint_type(ReplicationEndpointTypeValue::Source)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException") || err_str.contains("Missing"),
        "Expected validation error, got: {err_str}"
    );
}

// ---- Certificate tests ----

#[tokio::test]
async fn test_import_and_describe_certificates() {
    let client = make_client().await;

    let resp = client
        .import_certificate()
        .certificate_identifier("my-cert")
        .certificate_pem("-----BEGIN CERTIFICATE-----\nfake\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();

    let cert = resp.certificate().expect("should contain certificate");
    assert_eq!(cert.certificate_identifier().unwrap(), "my-cert");
    let arn = cert.certificate_arn().expect("should have ARN");
    assert!(arn.starts_with("arn:aws:dms:us-east-1:"));

    let desc_resp = client.describe_certificates().send().await.unwrap();
    let certs = desc_resp.certificates();
    assert_eq!(certs.len(), 1);
    assert_eq!(certs[0].certificate_identifier().unwrap(), "my-cert");
}

#[tokio::test]
async fn test_delete_certificate() {
    let client = make_client().await;

    let create_resp = client
        .import_certificate()
        .certificate_identifier("to-delete-cert")
        .send()
        .await
        .unwrap();
    let arn = create_resp
        .certificate()
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();

    client
        .delete_certificate()
        .certificate_arn(&arn)
        .send()
        .await
        .expect("delete_certificate should succeed");

    let desc_resp = client.describe_certificates().send().await.unwrap();
    assert_eq!(desc_resp.certificates().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_certificate() {
    let client = make_client().await;
    let err = client
        .delete_certificate()
        .certificate_arn("arn:aws:dms:us-east-1:123:cert:nonexistent")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFound") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

// ---- EventSubscription tests ----

#[tokio::test]
async fn test_create_and_describe_event_subscriptions() {
    let client = make_client().await;

    let resp = client
        .create_event_subscription()
        .subscription_name("my-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .source_type("replication-instance")
        .event_categories("failure")
        .enabled(true)
        .send()
        .await
        .unwrap();

    let sub = resp
        .event_subscription()
        .expect("should contain event subscription");
    assert_eq!(sub.cust_subscription_id().unwrap(), "my-sub");
    assert_eq!(
        sub.sns_topic_arn().unwrap(),
        "arn:aws:sns:us-east-1:123456789012:my-topic"
    );
    assert_eq!(sub.status().unwrap(), "active");

    let desc_resp = client.describe_event_subscriptions().send().await.unwrap();
    let subs = desc_resp.event_subscriptions_list();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].cust_subscription_id().unwrap(), "my-sub");
}

#[tokio::test]
async fn test_delete_event_subscription() {
    let client = make_client().await;

    client
        .create_event_subscription()
        .subscription_name("sub-to-delete")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .send()
        .await
        .unwrap();

    client
        .delete_event_subscription()
        .subscription_name("sub-to-delete")
        .send()
        .await
        .expect("delete_event_subscription should succeed");

    let desc_resp = client.describe_event_subscriptions().send().await.unwrap();
    assert_eq!(desc_resp.event_subscriptions_list().len(), 0);
}

#[tokio::test]
async fn test_modify_event_subscription() {
    let client = make_client().await;

    client
        .create_event_subscription()
        .subscription_name("mod-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:old-topic")
        .enabled(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_event_subscription()
        .subscription_name("mod-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:new-topic")
        .enabled(false)
        .send()
        .await
        .unwrap();

    let sub = resp.event_subscription().unwrap();
    assert_eq!(
        sub.sns_topic_arn().unwrap(),
        "arn:aws:sns:us-east-1:123456789012:new-topic"
    );
    assert!(!sub.enabled());
}

#[tokio::test]
async fn test_delete_nonexistent_event_subscription() {
    let client = make_client().await;
    let err = client
        .delete_event_subscription()
        .subscription_name("nonexistent")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFound") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

// ---- ModifyReplicationSubnetGroup test ----

#[tokio::test]
async fn test_modify_replication_subnet_group() {
    let client = make_client().await;

    client
        .create_replication_subnet_group()
        .replication_subnet_group_identifier("mod-grp")
        .replication_subnet_group_description("old desc")
        .subnet_ids("subnet-aaa")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_replication_subnet_group()
        .replication_subnet_group_identifier("mod-grp")
        .replication_subnet_group_description("new desc")
        .subnet_ids("subnet-bbb")
        .subnet_ids("subnet-ccc")
        .send()
        .await
        .unwrap();

    let grp = resp.replication_subnet_group().unwrap();
    assert_eq!(
        grp.replication_subnet_group_description().unwrap(),
        "new desc"
    );
}

// ---- Describe-only operations returning empty lists ----

#[tokio::test]
async fn test_describe_account_attributes() {
    let client = make_client().await;
    client
        .describe_account_attributes()
        .send()
        .await
        .expect("describe_account_attributes should succeed");
}

#[tokio::test]
async fn test_describe_event_categories() {
    let client = make_client().await;
    client
        .describe_event_categories()
        .send()
        .await
        .expect("describe_event_categories should succeed");
}

#[tokio::test]
async fn test_describe_orderable_replication_instances() {
    let client = make_client().await;
    let resp = client
        .describe_orderable_replication_instances()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.orderable_replication_instances().len(), 0);
}

#[tokio::test]
async fn test_describe_engine_versions() {
    let client = make_client().await;
    let resp = client.describe_engine_versions().send().await.unwrap();
    assert_eq!(resp.engine_versions().len(), 0);
}

#[tokio::test]
async fn test_describe_endpoint_types() {
    let client = make_client().await;
    let resp = client.describe_endpoint_types().send().await.unwrap();
    assert_eq!(resp.supported_endpoint_types().len(), 0);
}

#[tokio::test]
async fn test_describe_endpoint_settings() {
    let client = make_client().await;
    // EngineName is required by the SDK builder for this op.
    let resp = client
        .describe_endpoint_settings()
        .engine_name("mysql")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.endpoint_settings().len(), 0);
}

#[tokio::test]
async fn test_describe_pending_maintenance_actions() {
    let client = make_client().await;
    let resp = client
        .describe_pending_maintenance_actions()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.pending_maintenance_actions().len(), 0);
}

#[tokio::test]
async fn test_describe_events() {
    let client = make_client().await;
    let resp = client.describe_events().send().await.unwrap();
    assert_eq!(resp.events().len(), 0);
}

#[tokio::test]
async fn test_describe_replication_task_assessment_results() {
    let client = make_client().await;
    let resp = client
        .describe_replication_task_assessment_results()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.replication_task_assessment_results().len(), 0);
}

#[tokio::test]
async fn test_describe_replication_task_assessment_runs() {
    let client = make_client().await;
    let resp = client
        .describe_replication_task_assessment_runs()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.replication_task_assessment_runs().len(), 0);
}

#[tokio::test]
async fn test_describe_replication_task_individual_assessments() {
    let client = make_client().await;
    let resp = client
        .describe_replication_task_individual_assessments()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.replication_task_individual_assessments().len(), 0);
}

#[tokio::test]
async fn test_describe_applicable_individual_assessments() {
    let client = make_client().await;
    // ReplicationTaskArn is required by the SDK builder; pass a synthetic ARN.
    let resp = client
        .describe_applicable_individual_assessments()
        .replication_task_arn("arn:aws:dms:us-east-1:123:task:placeholder")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.individual_assessment_names().len(), 0);
}

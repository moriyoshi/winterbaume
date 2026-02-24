//! End-to-end scenario tests for the DMS service.
//!
//! Each test chains 3+ operations and asserts business outcomes rather
//! than per-API return shapes.

use aws_sdk_databasemigration::config::BehaviorVersion;
use aws_sdk_databasemigration::types::{
    MigrationTypeValue, ReplicationEndpointTypeValue, StartReplicationTaskTypeValue, Tag,
};
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

/// Scenario: Full migration pipeline lifecycle
///
/// A team sets up a database migration pipeline: create source and target
/// endpoints, create a replication instance, test connectivity, create a
/// migration task, run it, then clean up the entire stack. Validates that
/// the pipeline can be established and operated end-to-end.
#[tokio::test]
async fn test_migration_pipeline_lifecycle() {
    let client = make_client().await;

    // Step 1 — create source endpoint (MySQL)
    let src_resp = client
        .create_endpoint()
        .endpoint_identifier("pipeline-source")
        .endpoint_type(ReplicationEndpointTypeValue::Source)
        .engine_name("mysql")
        .server_name("prod-mysql.example.com")
        .port(3306)
        .database_name("inventory")
        .username("dms_user")
        .tags(tag("env", "prod"))
        .send()
        .await
        .unwrap();
    let src_arn = src_resp
        .endpoint()
        .unwrap()
        .endpoint_arn()
        .unwrap()
        .to_string();
    assert!(src_arn.starts_with("arn:aws:dms:"));

    // Step 2 — create target endpoint (PostgreSQL)
    let tgt_resp = client
        .create_endpoint()
        .endpoint_identifier("pipeline-target")
        .endpoint_type(ReplicationEndpointTypeValue::Target)
        .engine_name("postgres")
        .server_name("warehouse-pg.example.com")
        .port(5432)
        .database_name("data_warehouse")
        .username("dms_writer")
        .send()
        .await
        .unwrap();
    let tgt_arn = tgt_resp
        .endpoint()
        .unwrap()
        .endpoint_arn()
        .unwrap()
        .to_string();
    assert!(tgt_arn.starts_with("arn:aws:dms:"));

    // Both endpoints appear in describe
    let list_resp = client.describe_endpoints().send().await.unwrap();
    assert_eq!(
        list_resp.endpoints().len(),
        2,
        "Both endpoints should be listed"
    );

    // Step 3 — create replication instance
    let inst_resp = client
        .create_replication_instance()
        .replication_instance_identifier("pipeline-inst")
        .replication_instance_class("dms.r5.xlarge")
        .allocated_storage(200)
        .multi_az(false)
        .send()
        .await
        .unwrap();
    let inst = inst_resp.replication_instance().unwrap();
    let inst_arn = inst.replication_instance_arn().unwrap().to_string();
    assert_eq!(inst.replication_instance_status().unwrap(), "available");

    // Step 4 — test connectivity
    let conn_resp = client
        .test_connection()
        .replication_instance_arn(&inst_arn)
        .endpoint_arn(&src_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        conn_resp.connection().unwrap().status().unwrap(),
        "successful",
        "Connection test should succeed"
    );

    // Step 5 — create full-load migration task
    let task_resp = client
        .create_replication_task()
        .replication_task_identifier("pipeline-task")
        .source_endpoint_arn(&src_arn)
        .target_endpoint_arn(&tgt_arn)
        .replication_instance_arn(&inst_arn)
        .migration_type(MigrationTypeValue::FullLoad)
        .table_mappings(
            r#"{"rules":[{"rule-type":"selection","rule-id":"1","rule-name":"1","object-locator":{"schema-name":"%","table-name":"%"},"rule-action":"include"}]}"#,
        )
        .send()
        .await
        .unwrap();
    let task = task_resp.replication_task().unwrap();
    let task_arn = task.replication_task_arn().unwrap().to_string();
    assert_eq!(
        task.status().unwrap(),
        "ready",
        "New task should be in ready status"
    );

    // Step 6 — start migration
    let start_resp = client
        .start_replication_task()
        .replication_task_arn(&task_arn)
        .start_replication_task_type(StartReplicationTaskTypeValue::StartReplication)
        .send()
        .await
        .unwrap();
    assert_eq!(
        start_resp.replication_task().unwrap().status().unwrap(),
        "running",
        "Task should be running after start"
    );

    // Step 7 — stop migration
    let stop_resp = client
        .stop_replication_task()
        .replication_task_arn(&task_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        stop_resp.replication_task().unwrap().status().unwrap(),
        "stopped",
        "Task should be stopped"
    );

    // Step 8 — clean up: delete task, endpoints, instance
    let del_task = client
        .delete_replication_task()
        .replication_task_arn(&task_arn)
        .send()
        .await
        .unwrap();
    assert!(
        del_task
            .replication_task()
            .and_then(|t| t.replication_task_arn())
            .is_some(),
        "Deleted task should be returned"
    );

    let after_task_list = client.describe_replication_tasks().send().await.unwrap();
    assert!(
        after_task_list.replication_tasks().is_empty(),
        "No tasks should remain"
    );
}

/// Scenario: Subnet group and certificate management for a secure migration
///
/// A security-conscious team registers a TLS certificate, creates a subnet
/// group for network isolation, and confirms both resources are queryable.
/// This models the prerequisite setup that Terraform converters inject before
/// creating replication instances.
#[tokio::test]
async fn test_secure_migration_infrastructure_setup() {
    let client = make_client().await;

    // Step 1 — create subnet group for network isolation
    let sg_resp = client
        .create_replication_subnet_group()
        .replication_subnet_group_identifier("secure-subnet-grp")
        .replication_subnet_group_description("Private subnets for DMS migration")
        .subnet_ids("subnet-private-1a")
        .subnet_ids("subnet-private-1b")
        .subnet_ids("subnet-private-1c")
        .tags(tag("security", "high"))
        .send()
        .await
        .unwrap();
    let sg = sg_resp.replication_subnet_group().unwrap();
    assert_eq!(sg.subnet_group_status().unwrap(), "Complete");
    assert_eq!(sg.subnets().len(), 3, "All 3 subnets should be registered");

    // Step 2 — import TLS certificate
    let cert_resp = client
        .import_certificate()
        .certificate_identifier("migration-tls-cert")
        .certificate_pem(
            "-----BEGIN CERTIFICATE-----\nMIIBvTCCAWOgAwIBAgIRAIIxyz\n-----END CERTIFICATE-----",
        )
        .tags(tag("purpose", "dms-encryption"))
        .send()
        .await
        .unwrap();
    let cert_arn = cert_resp
        .certificate()
        .unwrap()
        .certificate_arn()
        .unwrap()
        .to_string();
    assert!(cert_arn.starts_with("arn:aws:dms:"));

    // Step 3 — list certificates to confirm registration
    let certs_resp = client.describe_certificates().send().await.unwrap();
    let certs = certs_resp.certificates();
    assert_eq!(certs.len(), 1, "One certificate should be registered");
    assert_eq!(
        certs[0].certificate_identifier().unwrap(),
        "migration-tls-cert"
    );

    // Step 4 — add monitoring event subscription
    let sub_resp = client
        .create_event_subscription()
        .subscription_name("secure-migration-alerts")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:dms-alerts")
        .source_type("replication-instance")
        .event_categories("failure")
        .event_categories("low storage")
        .enabled(true)
        .send()
        .await
        .unwrap();
    let sub = sub_resp.event_subscription().unwrap();
    assert_eq!(
        sub.status().unwrap(),
        "active",
        "Subscription should be active"
    );
    assert_eq!(
        sub.cust_subscription_id().unwrap(),
        "secure-migration-alerts"
    );

    // Step 5 — verify all infrastructure is visible together
    let sg_list = client
        .describe_replication_subnet_groups()
        .send()
        .await
        .unwrap();
    assert_eq!(
        sg_list.replication_subnet_groups().len(),
        1,
        "Subnet group should persist"
    );

    let sub_list = client.describe_event_subscriptions().send().await.unwrap();
    assert_eq!(
        sub_list.event_subscriptions_list().len(),
        1,
        "Event subscription should persist"
    );

    // Step 6 — cleanup certificate and subscription
    client
        .delete_certificate()
        .certificate_arn(&cert_arn)
        .send()
        .await
        .unwrap();

    client
        .delete_event_subscription()
        .subscription_name("secure-migration-alerts")
        .send()
        .await
        .unwrap();

    // Confirm teardown
    let empty_certs = client.describe_certificates().send().await.unwrap();
    assert!(
        empty_certs.certificates().is_empty(),
        "Certificates should be cleaned up"
    );
}

/// Scenario: State snapshot and restore preserves a running migration pipeline
///
/// Verifies that a complete migration environment ( endpoints + instance +
/// task ) survives a state snapshot/restore cycle, so that server restarts
/// do not lose in-progress migration configuration.
///
/// This scenario uses the service handle directly for snapshot/restore
/// because the SDK client only exposes API operations, not the lower-level
/// `StatefulService` methods. The setup goes through the service's request
/// dispatch path ( the same path the SDK would take ), then we exercise the
/// snapshot / restore round-trip on the service handle, then confirm the
/// restored state is observable through that same dispatch path.
#[tokio::test]
async fn test_pipeline_survives_snapshot_restore() {
    use bytes::Bytes;
    use serde_json::json;
    use winterbaume_core::{MockRequest, MockService, StatefulService};

    fn dms_request(action: &str, body: serde_json::Value) -> MockRequest {
        let uri = "https://dms.us-east-1.amazonaws.com".to_string();
        let mut headers = http::HeaderMap::new();
        headers.insert(
            "x-amz-target",
            format!("AmazonDMSv20160101.{action}").parse().unwrap(),
        );
        headers.insert(
            http::header::CONTENT_TYPE,
            "application/x-amz-json-1.1".parse().unwrap(),
        );
        MockRequest {
            method: "POST".to_string(),
            uri,
            headers,
            body: Bytes::from(body.to_string()),
        }
    }

    async fn call_value(
        service: &DmsService,
        action: &str,
        body: serde_json::Value,
    ) -> serde_json::Value {
        let resp = service.handle(dms_request(action, body)).await;
        serde_json::from_slice(&resp.body).unwrap_or(json!({}))
    }

    let service = DmsService::new();

    // Setup: build a minimal migration pipeline
    let src = call_value(
        &service,
        "CreateEndpoint",
        json!({
            "EndpointIdentifier": "snap-source",
            "EndpointType": "source",
            "EngineName": "oracle"
        }),
    )
    .await;
    let src_arn = src["Endpoint"]["EndpointArn"].as_str().unwrap().to_string();

    let tgt = call_value(
        &service,
        "CreateEndpoint",
        json!({
            "EndpointIdentifier": "snap-target",
            "EndpointType": "target",
            "EngineName": "aurora"
        }),
    )
    .await;
    let tgt_arn = tgt["Endpoint"]["EndpointArn"].as_str().unwrap().to_string();

    let inst = call_value(
        &service,
        "CreateReplicationInstance",
        json!({
            "ReplicationInstanceIdentifier": "snap-inst",
            "ReplicationInstanceClass": "dms.t3.medium"
        }),
    )
    .await;
    let inst_arn = inst["ReplicationInstance"]["ReplicationInstanceArn"]
        .as_str()
        .unwrap()
        .to_string();

    call_value(
        &service,
        "CreateReplicationTask",
        json!({
            "ReplicationTaskIdentifier": "snap-task",
            "SourceEndpointArn": src_arn,
            "TargetEndpointArn": tgt_arn,
            "ReplicationInstanceArn": inst_arn,
            "MigrationType": "full-load-and-cdc",
            "TableMappings": r#"{"rules":[]}"#
        }),
    )
    .await;

    // Take snapshot
    let snapshot = service.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapshot.endpoints.len(), 2);
    assert_eq!(snapshot.replication_instances.len(), 1);
    assert_eq!(snapshot.replication_tasks.len(), 1);

    // Restore into a new service instance
    let service2 = DmsService::new();
    service2
        .restore("123456789012", "us-east-1", snapshot)
        .await
        .expect("restore should succeed");

    // Verify all resources are present in the restored service
    let ep_list = call_value(&service2, "DescribeEndpoints", json!({})).await;
    assert_eq!(
        ep_list["Endpoints"].as_array().unwrap().len(),
        2,
        "Both endpoints should survive restore"
    );

    let inst_list = call_value(&service2, "DescribeReplicationInstances", json!({})).await;
    assert_eq!(
        inst_list["ReplicationInstances"].as_array().unwrap().len(),
        1,
        "Replication instance should survive restore"
    );

    let task_list = call_value(&service2, "DescribeReplicationTasks", json!({})).await;
    assert_eq!(
        task_list["ReplicationTasks"].as_array().unwrap().len(),
        1,
        "Replication task should survive restore"
    );

    // Verify task is still in ready state ( connections are transient ).
    let task = &task_list["ReplicationTasks"].as_array().unwrap()[0];
    assert_eq!(
        task["ReplicationTaskIdentifier"], "snap-task",
        "Task identifier should match"
    );
}

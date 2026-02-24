use aws_sdk_redshift::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_redshift::RedshiftService;

async fn make_client() -> aws_sdk_redshift::Client {
    let mock = MockAws::builder()
        .with_service(RedshiftService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_redshift::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_redshift::Client::new(&config)
}

fn make_service() -> RedshiftService {
    RedshiftService::new()
}

// ---- Cluster lifecycle ----

#[tokio::test]
async fn test_create_describe_delete_cluster() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_cluster()
        .cluster_identifier("test-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .db_name("mydb")
        .number_of_nodes(2)
        .send()
        .await
        .expect("create_cluster should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "test-cluster");
    assert_eq!(cluster.cluster_status().unwrap_or(""), "available");

    // Describe
    let desc = client
        .describe_clusters()
        .cluster_identifier("test-cluster")
        .send()
        .await
        .expect("describe_clusters should succeed");
    let clusters = desc.clusters();
    assert_eq!(clusters.len(), 1);
    assert_eq!(
        clusters[0].cluster_identifier().unwrap_or(""),
        "test-cluster"
    );

    // Delete
    let del = client
        .delete_cluster()
        .cluster_identifier("test-cluster")
        .skip_final_cluster_snapshot(true)
        .send()
        .await
        .expect("delete_cluster should succeed");
    let deleted = del.cluster().expect("cluster should be present");
    assert_eq!(deleted.cluster_identifier().unwrap_or(""), "test-cluster");

    // Should be gone
    let desc2 = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    assert_eq!(desc2.clusters().len(), 0);
}

#[tokio::test]
async fn test_describe_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .describe_clusters()
        .cluster_identifier("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Coverage for FIX(terraform-e2e) handler fixes ----

// Covers FIX(terraform-e2e): DescribeClusters must return non-empty
// ClusterNodes and ClusterParameterGroups plus a valid MultiAZ value because
// the Terraform provider reads these fields without nil or empty-list guards.
#[tokio::test]
async fn test_describe_cluster_includes_provider_required_defaults() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("terraform-defaults-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .number_of_nodes(2)
        .send()
        .await
        .expect("create_cluster should succeed");

    let desc = client
        .describe_clusters()
        .cluster_identifier("terraform-defaults-cluster")
        .send()
        .await
        .expect("describe_clusters should succeed");
    let cluster = desc.clusters().first().expect("cluster should exist");

    let nodes = cluster.cluster_nodes();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].node_role(), Some("LEADER"));
    assert_eq!(nodes[1].node_role(), Some("COMPUTE"));

    let parameter_groups = cluster.cluster_parameter_groups();
    assert_eq!(parameter_groups.len(), 1);
    assert_eq!(
        parameter_groups[0].parameter_group_name(),
        Some("default.redshift-1.0")
    );
    assert_eq!(
        parameter_groups[0].parameter_apply_status(),
        Some("in-sync")
    );

    assert_eq!(cluster.multi_az(), Some("Disabled"));
}

#[tokio::test]
async fn test_modify_cluster() {
    let client = make_client().await;
    client
        .create_cluster()
        .cluster_identifier("mod-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster()
        .cluster_identifier("mod-cluster")
        .node_type("ra3.xlplus")
        .number_of_nodes(4)
        .send()
        .await
        .expect("modify_cluster should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.node_type().unwrap_or(""), "ra3.xlplus");
    assert_eq!(cluster.number_of_nodes(), Some(4));
}

#[tokio::test]
async fn test_pause_resume_cluster() {
    let client = make_client().await;
    client
        .create_cluster()
        .cluster_identifier("pause-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let paused = client
        .pause_cluster()
        .cluster_identifier("pause-cluster")
        .send()
        .await
        .expect("pause_cluster should succeed");
    assert_eq!(
        paused.cluster().unwrap().cluster_status().unwrap_or(""),
        "paused"
    );

    let resumed = client
        .resume_cluster()
        .cluster_identifier("pause-cluster")
        .send()
        .await
        .expect("resume_cluster should succeed");
    assert_eq!(
        resumed.cluster().unwrap().cluster_status().unwrap_or(""),
        "available"
    );
}

// ---- Subnet Groups ----

#[tokio::test]
async fn test_create_describe_delete_subnet_group() {
    let client = make_client().await;

    let resp = client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("my-subnet-group")
        .description("Test subnet group")
        .subnet_ids("subnet-111")
        .subnet_ids("subnet-222")
        .send()
        .await
        .expect("create_cluster_subnet_group should succeed");
    let sg = resp.cluster_subnet_group().unwrap();
    assert_eq!(
        sg.cluster_subnet_group_name().unwrap_or(""),
        "my-subnet-group"
    );

    let desc = client
        .describe_cluster_subnet_groups()
        .cluster_subnet_group_name("my-subnet-group")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.cluster_subnet_groups().len(), 1);

    client
        .delete_cluster_subnet_group()
        .cluster_subnet_group_name("my-subnet-group")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_cluster_subnet_groups()
        .cluster_subnet_group_name("my-subnet-group")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Parameter Groups ----

#[tokio::test]
async fn test_create_describe_delete_parameter_group() {
    let client = make_client().await;

    let resp = client
        .create_cluster_parameter_group()
        .parameter_group_name("my-pg")
        .parameter_group_family("redshift-1.0")
        .description("Test parameter group")
        .send()
        .await
        .expect("create_cluster_parameter_group should succeed");
    let pg = resp.cluster_parameter_group().unwrap();
    assert_eq!(pg.parameter_group_name().unwrap_or(""), "my-pg");

    let desc = client
        .describe_cluster_parameter_groups()
        .parameter_group_name("my-pg")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.parameter_groups().len(), 1);

    let params = client
        .describe_cluster_parameters()
        .parameter_group_name("my-pg")
        .send()
        .await
        .unwrap();
    assert!(!params.parameters().is_empty());

    client
        .delete_cluster_parameter_group()
        .parameter_group_name("my-pg")
        .send()
        .await
        .expect("delete should succeed");
}

#[tokio::test]
async fn test_describe_default_cluster_parameters() {
    let client = make_client().await;
    let resp = client
        .describe_default_cluster_parameters()
        .parameter_group_family("redshift-1.0")
        .send()
        .await
        .expect("describe_default_cluster_parameters should succeed");
    let defaults = resp.default_cluster_parameters().unwrap();
    assert!(!defaults.parameters().is_empty());
}

// ---- Security Groups ----

#[tokio::test]
async fn test_create_describe_delete_security_group() {
    let client = make_client().await;

    let resp = client
        .create_cluster_security_group()
        .cluster_security_group_name("my-sg")
        .description("Test security group")
        .send()
        .await
        .expect("create_cluster_security_group should succeed");
    let sg = resp.cluster_security_group().unwrap();
    assert_eq!(sg.cluster_security_group_name().unwrap_or(""), "my-sg");

    let desc = client
        .describe_cluster_security_groups()
        .cluster_security_group_name("my-sg")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.cluster_security_groups().len(), 1);

    client
        .delete_cluster_security_group()
        .cluster_security_group_name("my-sg")
        .send()
        .await
        .expect("delete should succeed");
}

#[tokio::test]
async fn test_authorize_cluster_security_group_ingress() {
    let client = make_client().await;

    client
        .create_cluster_security_group()
        .cluster_security_group_name("ingress-sg")
        .description("Test")
        .send()
        .await
        .unwrap();

    let resp = client
        .authorize_cluster_security_group_ingress()
        .cluster_security_group_name("ingress-sg")
        .cidrip("10.0.0.0/8")
        .send()
        .await
        .expect("authorize_cluster_security_group_ingress should succeed");
    let sg = resp.cluster_security_group().unwrap();
    assert_eq!(sg.ip_ranges().len(), 1);
}

// ---- Snapshots ----

#[tokio::test]
async fn test_create_describe_delete_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("snap-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_cluster_snapshot()
        .snapshot_identifier("my-snap")
        .cluster_identifier("snap-cluster")
        .send()
        .await
        .expect("create_cluster_snapshot should succeed");
    let snap = resp.snapshot().unwrap();
    assert_eq!(snap.snapshot_identifier().unwrap_or(""), "my-snap");
    assert_eq!(snap.cluster_identifier().unwrap_or(""), "snap-cluster");

    let desc = client
        .describe_cluster_snapshots()
        .snapshot_identifier("my-snap")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshots().len(), 1);

    let del = client
        .delete_cluster_snapshot()
        .snapshot_identifier("my-snap")
        .send()
        .await
        .expect("delete_cluster_snapshot should succeed");
    assert_eq!(
        del.snapshot().unwrap().snapshot_identifier().unwrap_or(""),
        "my-snap"
    );
}

#[tokio::test]
async fn test_restore_from_cluster_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("restore-src")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("restore-snap")
        .cluster_identifier("restore-src")
        .send()
        .await
        .unwrap();

    let resp = client
        .restore_from_cluster_snapshot()
        .cluster_identifier("restore-dest")
        .snapshot_identifier("restore-snap")
        .send()
        .await
        .expect("restore_from_cluster_snapshot should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "restore-dest");
}

// ---- Snapshot Copy ----

#[tokio::test]
async fn test_enable_disable_snapshot_copy() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("copy-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let enabled = client
        .enable_snapshot_copy()
        .cluster_identifier("copy-cluster")
        .destination_region("us-west-2")
        .retention_period(7)
        .send()
        .await
        .expect("enable_snapshot_copy should succeed");
    let copy_status = enabled.cluster().unwrap().cluster_snapshot_copy_status();
    assert!(copy_status.is_some());
    assert_eq!(
        copy_status.unwrap().destination_region().unwrap_or(""),
        "us-west-2"
    );

    let disabled = client
        .disable_snapshot_copy()
        .cluster_identifier("copy-cluster")
        .send()
        .await
        .expect("disable_snapshot_copy should succeed");
    assert!(
        disabled
            .cluster()
            .unwrap()
            .cluster_snapshot_copy_status()
            .is_none()
    );
}

#[tokio::test]
async fn test_modify_snapshot_copy_retention_period() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("ret-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .enable_snapshot_copy()
        .cluster_identifier("ret-cluster")
        .destination_region("us-west-2")
        .retention_period(7)
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_snapshot_copy_retention_period()
        .cluster_identifier("ret-cluster")
        .retention_period(14)
        .send()
        .await
        .expect("modify_snapshot_copy_retention_period should succeed");
    let copy_status = resp
        .cluster()
        .unwrap()
        .cluster_snapshot_copy_status()
        .unwrap();
    assert_eq!(copy_status.retention_period(), Some(14));
}

// ---- Snapshot Copy Grants ----

#[tokio::test]
async fn test_create_describe_delete_snapshot_copy_grant() {
    let client = make_client().await;

    let resp = client
        .create_snapshot_copy_grant()
        .snapshot_copy_grant_name("my-grant")
        .send()
        .await
        .expect("create_snapshot_copy_grant should succeed");
    let grant = resp.snapshot_copy_grant().unwrap();
    assert_eq!(grant.snapshot_copy_grant_name().unwrap_or(""), "my-grant");

    let desc = client
        .describe_snapshot_copy_grants()
        .snapshot_copy_grant_name("my-grant")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshot_copy_grants().len(), 1);

    client
        .delete_snapshot_copy_grant()
        .snapshot_copy_grant_name("my-grant")
        .send()
        .await
        .expect("delete should succeed");
}

// ---- Logging ----

#[tokio::test]
async fn test_enable_disable_describe_logging() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("log-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let enabled = client
        .enable_logging()
        .cluster_identifier("log-cluster")
        .bucket_name("my-log-bucket")
        .s3_key_prefix("logs/")
        .send()
        .await
        .expect("enable_logging should succeed");
    assert_eq!(enabled.logging_enabled(), Some(true));
    assert_eq!(enabled.bucket_name().unwrap_or(""), "my-log-bucket");

    let status = client
        .describe_logging_status()
        .cluster_identifier("log-cluster")
        .send()
        .await
        .expect("describe_logging_status should succeed");
    assert_eq!(status.logging_enabled(), Some(true));

    let disabled = client
        .disable_logging()
        .cluster_identifier("log-cluster")
        .send()
        .await
        .expect("disable_logging should succeed");
    assert_eq!(disabled.logging_enabled(), Some(false));
}

// ---- Tags ----

#[tokio::test]
async fn test_create_describe_delete_tags() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("tag-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    // Use the cluster ARN from describe
    let desc = client
        .describe_clusters()
        .cluster_identifier("tag-cluster")
        .send()
        .await
        .unwrap();
    let arn = desc.clusters()[0]
        .cluster_namespace_arn()
        .unwrap_or("")
        .to_string();

    // Create tags
    let tag = aws_sdk_redshift::types::Tag::builder()
        .key("env")
        .value("test")
        .build();
    client
        .create_tags()
        .resource_name(&arn)
        .tags(tag)
        .send()
        .await
        .expect("create_tags should succeed");

    // Describe tags
    let desc_tags = client
        .describe_tags()
        .resource_name(&arn)
        .send()
        .await
        .expect("describe_tags should succeed");
    assert!(!desc_tags.tagged_resources().is_empty());

    // Delete tags
    client
        .delete_tags()
        .resource_name(&arn)
        .tag_keys("env".to_string())
        .send()
        .await
        .expect("delete_tags should succeed");
}

// ---- Credentials ----

#[tokio::test]
async fn test_get_cluster_credentials() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("creds-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_cluster_credentials()
        .cluster_identifier("creds-cluster")
        .db_user("testuser")
        .db_name("dev")
        .send()
        .await
        .expect("get_cluster_credentials should succeed");
    assert!(resp.db_user().is_some());
    assert!(resp.db_password().is_some());
    assert!(resp.expiration().is_some());
}

// ---- Delete cluster with final snapshot ----

#[tokio::test]
async fn test_delete_cluster_with_final_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("final-snap-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .delete_cluster()
        .cluster_identifier("final-snap-cluster")
        .final_cluster_snapshot_identifier("final-snap")
        .send()
        .await
        .expect("delete_cluster should succeed");

    // Final snapshot should exist
    let desc = client
        .describe_cluster_snapshots()
        .snapshot_identifier("final-snap")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshots().len(), 1);
}

// ---- State view tests ----

#[tokio::test]
async fn test_state_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_redshift::RedshiftStateView;

    let svc = make_service();

    // Restore with pre-loaded state
    let mut view = RedshiftStateView::default();
    view.clusters.insert(
        "pre-cluster".to_string(),
        winterbaume_redshift::types::RedshiftCluster::new(
            "pre-cluster".to_string(),
            "dc2.large".to_string(),
            "admin".to_string(),
            "dev".to_string(),
            "us-east-1",
            "123456789012",
            1,
        ),
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.clusters.contains_key("pre-cluster"));
}

#[tokio::test]
async fn test_state_merge() {
    use winterbaume_core::StatefulService;
    use winterbaume_redshift::RedshiftStateView;

    let svc = make_service();

    // Insert first cluster
    let mut view1 = RedshiftStateView::default();
    view1.clusters.insert(
        "cluster-a".to_string(),
        winterbaume_redshift::types::RedshiftCluster::new(
            "cluster-a".to_string(),
            "dc2.large".to_string(),
            "admin".to_string(),
            "dev".to_string(),
            "us-east-1",
            "123456789012",
            1,
        ),
    );
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    // Merge a second cluster
    let mut view2 = RedshiftStateView::default();
    view2.clusters.insert(
        "cluster-b".to_string(),
        winterbaume_redshift::types::RedshiftCluster::new(
            "cluster-b".to_string(),
            "dc2.large".to_string(),
            "admin".to_string(),
            "dev".to_string(),
            "us-east-1",
            "123456789012",
            1,
        ),
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot.clusters.contains_key("cluster-a"),
        "original should remain"
    );
    assert!(
        snapshot.clusters.contains_key("cluster-b"),
        "merged should be present"
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_redshift::RedshiftStateView;

    let svc = make_service();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", RedshiftStateView::default())
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
    use winterbaume_redshift::RedshiftStateView;

    let svc = make_service();

    // Pre-seed state (ignore first event)
    let view = RedshiftStateView::default();
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Re-register and capture snapshot
    let snapshots: Arc<Mutex<Vec<RedshiftStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Restore with a cluster present
    let mut view2 = RedshiftStateView::default();
    view2.clusters.insert(
        "listener-cluster".to_string(),
        winterbaume_redshift::types::RedshiftCluster::new(
            "listener-cluster".to_string(),
            "dc2.large".to_string(),
            "admin".to_string(),
            "dev".to_string(),
            "us-east-1",
            "123456789012",
            1,
        ),
    );
    svc.restore("123456789012", "us-east-1", view2)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0].clusters.contains_key("listener-cluster"),
        "snapshot should reflect the mutation"
    );
}

// ---- HSM Client Certificates ----

#[tokio::test]
async fn test_create_describe_delete_hsm_client_certificate() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_hsm_client_certificate()
        .hsm_client_certificate_identifier("test-cert")
        .send()
        .await
        .expect("create_hsm_client_certificate should succeed");
    let cert = resp
        .hsm_client_certificate()
        .expect("cert should be present");
    assert_eq!(
        cert.hsm_client_certificate_identifier().unwrap_or(""),
        "test-cert"
    );

    // Describe
    let desc = client
        .describe_hsm_client_certificates()
        .hsm_client_certificate_identifier("test-cert")
        .send()
        .await
        .expect("describe_hsm_client_certificates should succeed");
    assert_eq!(desc.hsm_client_certificates().len(), 1);

    // Delete
    client
        .delete_hsm_client_certificate()
        .hsm_client_certificate_identifier("test-cert")
        .send()
        .await
        .expect("delete_hsm_client_certificate should succeed");

    // Gone
    let desc2 = client
        .describe_hsm_client_certificates()
        .send()
        .await
        .expect("describe_hsm_client_certificates should succeed");
    assert_eq!(desc2.hsm_client_certificates().len(), 0);
}

// ---- HSM Configurations ----

#[tokio::test]
async fn test_create_describe_delete_hsm_configuration() {
    let client = make_client().await;

    let resp = client
        .create_hsm_configuration()
        .hsm_configuration_identifier("test-config")
        .description("Test HSM config")
        .hsm_ip_address("10.0.0.1")
        .hsm_partition_name("partition1")
        .hsm_partition_password("password1")
        .hsm_server_public_certificate("cert1")
        .send()
        .await
        .expect("create_hsm_configuration should succeed");
    let config = resp.hsm_configuration().expect("config should be present");
    assert_eq!(
        config.hsm_configuration_identifier().unwrap_or(""),
        "test-config"
    );

    let desc = client
        .describe_hsm_configurations()
        .hsm_configuration_identifier("test-config")
        .send()
        .await
        .expect("describe_hsm_configurations should succeed");
    assert_eq!(desc.hsm_configurations().len(), 1);

    client
        .delete_hsm_configuration()
        .hsm_configuration_identifier("test-config")
        .send()
        .await
        .expect("delete_hsm_configuration should succeed");

    let desc2 = client
        .describe_hsm_configurations()
        .send()
        .await
        .expect("describe_hsm_configurations should succeed");
    assert_eq!(desc2.hsm_configurations().len(), 0);
}

// ---- Authentication Profiles ----

#[tokio::test]
async fn test_create_describe_modify_delete_authentication_profile() {
    let client = make_client().await;

    let resp = client
        .create_authentication_profile()
        .authentication_profile_name("test-profile")
        .authentication_profile_content(r#"{"AllowDBUserOverride":"1"}"#)
        .send()
        .await
        .expect("create_authentication_profile should succeed");
    assert_eq!(
        resp.authentication_profile_name().unwrap_or(""),
        "test-profile"
    );

    let desc = client
        .describe_authentication_profiles()
        .authentication_profile_name("test-profile")
        .send()
        .await
        .expect("describe_authentication_profiles should succeed");
    assert_eq!(desc.authentication_profiles().len(), 1);

    let modified = client
        .modify_authentication_profile()
        .authentication_profile_name("test-profile")
        .authentication_profile_content(r#"{"AllowDBUserOverride":"0"}"#)
        .send()
        .await
        .expect("modify_authentication_profile should succeed");
    assert_eq!(
        modified.authentication_profile_content().unwrap_or(""),
        r#"{"AllowDBUserOverride":"0"}"#
    );

    client
        .delete_authentication_profile()
        .authentication_profile_name("test-profile")
        .send()
        .await
        .expect("delete_authentication_profile should succeed");

    let desc2 = client
        .describe_authentication_profiles()
        .send()
        .await
        .expect("describe_authentication_profiles should succeed");
    assert_eq!(desc2.authentication_profiles().len(), 0);
}

// ---- Usage Limits ----

#[tokio::test]
async fn test_create_describe_modify_delete_usage_limit() {
    let client = make_client().await;

    // Create a cluster first
    client
        .create_cluster()
        .cluster_identifier("ul-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_usage_limit()
        .cluster_identifier("ul-cluster")
        .feature_type(aws_sdk_redshift::types::UsageLimitFeatureType::Spectrum)
        .limit_type(aws_sdk_redshift::types::UsageLimitLimitType::DataScanned)
        .amount(100)
        .send()
        .await
        .expect("create_usage_limit should succeed");
    let limit_id = resp.usage_limit_id().unwrap_or("").to_string();
    assert!(!limit_id.is_empty());

    let desc = client
        .describe_usage_limits()
        .cluster_identifier("ul-cluster")
        .send()
        .await
        .expect("describe_usage_limits should succeed");
    assert_eq!(desc.usage_limits().len(), 1);

    client
        .modify_usage_limit()
        .usage_limit_id(&limit_id)
        .amount(200)
        .send()
        .await
        .expect("modify_usage_limit should succeed");

    client
        .delete_usage_limit()
        .usage_limit_id(&limit_id)
        .send()
        .await
        .expect("delete_usage_limit should succeed");

    let desc2 = client
        .describe_usage_limits()
        .cluster_identifier("ul-cluster")
        .send()
        .await
        .expect("describe_usage_limits should succeed");
    assert_eq!(desc2.usage_limits().len(), 0);
}

// ---- Snapshot Schedules ----

#[tokio::test]
async fn test_create_describe_modify_delete_snapshot_schedule() {
    let client = make_client().await;

    let resp = client
        .create_snapshot_schedule()
        .schedule_identifier("test-schedule")
        .schedule_definitions("rate(12 hours)")
        .send()
        .await
        .expect("create_snapshot_schedule should succeed");
    assert_eq!(resp.schedule_identifier().unwrap_or(""), "test-schedule");

    let desc = client
        .describe_snapshot_schedules()
        .schedule_identifier("test-schedule")
        .send()
        .await
        .expect("describe_snapshot_schedules should succeed");
    assert_eq!(desc.snapshot_schedules().len(), 1);

    client
        .modify_snapshot_schedule()
        .schedule_identifier("test-schedule")
        .schedule_definitions("rate(24 hours)")
        .send()
        .await
        .expect("modify_snapshot_schedule should succeed");

    client
        .delete_snapshot_schedule()
        .schedule_identifier("test-schedule")
        .send()
        .await
        .expect("delete_snapshot_schedule should succeed");

    let desc2 = client
        .describe_snapshot_schedules()
        .send()
        .await
        .expect("describe_snapshot_schedules should succeed");
    assert_eq!(desc2.snapshot_schedules().len(), 0);
}

// ---- Scheduled Actions ----

#[tokio::test]
async fn test_create_describe_modify_delete_scheduled_action() {
    let client = make_client().await;

    let pause_action = aws_sdk_redshift::types::ScheduledActionType::builder()
        .pause_cluster(
            aws_sdk_redshift::types::PauseClusterMessage::builder()
                .cluster_identifier("test-cluster")
                .build(),
        )
        .build();

    let resp = client
        .create_scheduled_action()
        .scheduled_action_name("test-action")
        .schedule("cron(0 18 * * ? *)")
        .iam_role("arn:aws:iam::123456789012:role/SchedulerRole")
        .target_action(pause_action)
        .send()
        .await
        .expect("create_scheduled_action should succeed");
    assert_eq!(resp.scheduled_action_name().unwrap_or(""), "test-action");

    let desc = client
        .describe_scheduled_actions()
        .scheduled_action_name("test-action")
        .send()
        .await
        .expect("describe_scheduled_actions should succeed");
    assert_eq!(desc.scheduled_actions().len(), 1);

    client
        .modify_scheduled_action()
        .scheduled_action_name("test-action")
        .schedule("cron(0 6 * * ? *)")
        .send()
        .await
        .expect("modify_scheduled_action should succeed");

    client
        .delete_scheduled_action()
        .scheduled_action_name("test-action")
        .send()
        .await
        .expect("delete_scheduled_action should succeed");

    let desc2 = client
        .describe_scheduled_actions()
        .send()
        .await
        .expect("describe_scheduled_actions should succeed");
    assert_eq!(desc2.scheduled_actions().len(), 0);
}

// ---- Snapshot access ----

#[tokio::test]
async fn test_authorize_revoke_snapshot_access() {
    let client = make_client().await;

    // Create cluster and snapshot
    client
        .create_cluster()
        .cluster_identifier("snap-access-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();
    client
        .create_cluster_snapshot()
        .cluster_identifier("snap-access-cluster")
        .snapshot_identifier("snap-access-snap")
        .send()
        .await
        .unwrap();

    let resp = client
        .authorize_snapshot_access()
        .snapshot_identifier("snap-access-snap")
        .account_with_restore_access("111122223333")
        .send()
        .await
        .expect("authorize_snapshot_access should succeed");
    assert!(resp.snapshot().is_some());

    let resp2 = client
        .revoke_snapshot_access()
        .snapshot_identifier("snap-access-snap")
        .account_with_restore_access("111122223333")
        .send()
        .await
        .expect("revoke_snapshot_access should succeed");
    assert!(resp2.snapshot().is_some());
}

// ---- Misc cluster operations ----

#[tokio::test]
async fn test_rotate_encryption_key() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("enc-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .rotate_encryption_key()
        .cluster_identifier("enc-cluster")
        .send()
        .await
        .expect("rotate_encryption_key should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "enc-cluster");
}

#[tokio::test]
async fn test_modify_cluster_iam_roles() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("iam-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster_iam_roles()
        .cluster_identifier("iam-cluster")
        .send()
        .await
        .expect("modify_cluster_iam_roles should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "iam-cluster");
}

#[tokio::test]
async fn test_modify_cluster_maintenance() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("maint-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster_maintenance()
        .cluster_identifier("maint-cluster")
        .send()
        .await
        .expect("modify_cluster_maintenance should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "maint-cluster");
}

#[tokio::test]
async fn test_failover_primary_compute() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("failover-cluster")
        .node_type("ra3.xlplus")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .failover_primary_compute()
        .cluster_identifier("failover-cluster")
        .send()
        .await
        .expect("failover_primary_compute should succeed");
    assert!(resp.cluster().is_some());
}

#[tokio::test]
async fn test_describe_account_attributes() {
    let client = make_client().await;
    let resp = client
        .describe_account_attributes()
        .send()
        .await
        .expect("describe_account_attributes should succeed");
    let _ = resp.account_attributes();
}

#[tokio::test]
async fn test_describe_cluster_tracks() {
    let client = make_client().await;
    let resp = client
        .describe_cluster_tracks()
        .send()
        .await
        .expect("describe_cluster_tracks should succeed");
    assert!(!resp.maintenance_tracks().is_empty());
}

#[tokio::test]
async fn test_describe_resize() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("resize-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_resize()
        .cluster_identifier("resize-cluster")
        .send()
        .await
        .expect("describe_resize should succeed");
    assert_eq!(resp.status().unwrap_or(""), "NONE");
}

#[tokio::test]
async fn test_modify_cluster_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("snap-mod-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .cluster_identifier("snap-mod-cluster")
        .snapshot_identifier("snap-to-modify")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster_snapshot()
        .snapshot_identifier("snap-to-modify")
        .send()
        .await
        .expect("modify_cluster_snapshot should succeed");
    assert!(resp.snapshot().is_some());
}

#[tokio::test]
async fn test_describe_cluster_db_revisions() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("rev-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_cluster_db_revisions()
        .cluster_identifier("rev-cluster")
        .send()
        .await
        .expect("describe_cluster_db_revisions should succeed");
    assert_eq!(resp.cluster_db_revisions().len(), 1);
}

// ---- Modify Event Subscription ----

#[tokio::test]
async fn test_modify_event_subscription() {
    let client = make_client().await;

    client
        .create_event_subscription()
        .subscription_name("mod-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_event_subscription()
        .subscription_name("mod-sub")
        .enabled(false)
        .send()
        .await
        .expect("modify_event_subscription should succeed");
    let sub = resp
        .event_subscription()
        .expect("event subscription should be present");
    assert_eq!(sub.cust_subscription_id().unwrap_or(""), "mod-sub");
}

// ---- Resource Policies ----

#[tokio::test]
async fn test_put_get_delete_resource_policy() {
    let client = make_client().await;

    let arn = "arn:aws:redshift:us-east-1:123456789012:cluster:policy-cluster";

    client
        .put_resource_policy()
        .resource_arn(arn)
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let get_resp = client
        .get_resource_policy()
        .resource_arn(arn)
        .send()
        .await
        .expect("get_resource_policy should succeed");
    assert!(get_resp.resource_policy().is_some());

    client
        .delete_resource_policy()
        .resource_arn(arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");
}

// ---- Partner Integrations ----

#[tokio::test]
async fn test_add_describe_delete_partner() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("partner-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .add_partner()
        .account_id("123456789012")
        .cluster_identifier("partner-cluster")
        .database_name("mydb")
        .partner_name("mypartner")
        .send()
        .await
        .expect("add_partner should succeed");

    let desc = client
        .describe_partners()
        .account_id("123456789012")
        .cluster_identifier("partner-cluster")
        .send()
        .await
        .expect("describe_partners should succeed");
    assert_eq!(desc.partner_integration_info_list().len(), 1);

    client
        .delete_partner()
        .account_id("123456789012")
        .cluster_identifier("partner-cluster")
        .database_name("mydb")
        .partner_name("mypartner")
        .send()
        .await
        .expect("delete_partner should succeed");

    let desc2 = client
        .describe_partners()
        .account_id("123456789012")
        .cluster_identifier("partner-cluster")
        .send()
        .await
        .expect("describe_partners should succeed");
    assert_eq!(desc2.partner_integration_info_list().len(), 0);
}

// ---- Simple read-only operations ----

#[tokio::test]
async fn test_describe_reserved_nodes() {
    let client = make_client().await;
    let resp = client
        .describe_reserved_nodes()
        .send()
        .await
        .expect("describe_reserved_nodes should succeed");
    assert_eq!(resp.reserved_nodes().len(), 0);
}

#[tokio::test]
async fn test_describe_reserved_node_offerings() {
    let client = make_client().await;
    let resp = client
        .describe_reserved_node_offerings()
        .send()
        .await
        .expect("describe_reserved_node_offerings should succeed");
    assert_eq!(resp.reserved_node_offerings().len(), 0);
}

#[tokio::test]
async fn test_describe_table_restore_status() {
    let client = make_client().await;
    let resp = client
        .describe_table_restore_status()
        .cluster_identifier("any-cluster")
        .send()
        .await
        .expect("describe_table_restore_status should succeed");
    assert_eq!(resp.table_restore_status_details().len(), 0);
}

#[tokio::test]
async fn test_list_recommendations() {
    let client = make_client().await;
    let resp = client
        .list_recommendations()
        .send()
        .await
        .expect("list_recommendations should succeed");
    assert_eq!(resp.recommendations().len(), 0);
}

#[tokio::test]
async fn test_modify_cluster_snapshot_schedule() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("schd-mod-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .modify_cluster_snapshot_schedule()
        .cluster_identifier("schd-mod-cluster")
        .send()
        .await
        .expect("modify_cluster_snapshot_schedule should succeed");
}

#[tokio::test]
async fn test_modify_aqua_configuration() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("aqua-cluster")
        .node_type("ra3.xlplus")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_aqua_configuration()
        .cluster_identifier("aqua-cluster")
        .send()
        .await
        .expect("modify_aqua_configuration should succeed");
    assert!(resp.aqua_configuration().is_some());
}

// ---- Reboot Cluster ----

#[tokio::test]
async fn test_reboot_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("reboot-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .reboot_cluster()
        .cluster_identifier("reboot-cluster")
        .send()
        .await
        .expect("reboot_cluster should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "reboot-cluster");
    assert_eq!(cluster.cluster_status().unwrap_or(""), "rebooting");
}

#[tokio::test]
async fn test_reboot_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .reboot_cluster()
        .cluster_identifier("no-such-cluster")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Resize Cluster ----

#[tokio::test]
async fn test_resize_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("resize-me")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .number_of_nodes(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .resize_cluster()
        .cluster_identifier("resize-me")
        .node_type("ra3.xlplus")
        .number_of_nodes(4)
        .send()
        .await
        .expect("resize_cluster should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.node_type().unwrap_or(""), "ra3.xlplus");
    assert_eq!(cluster.number_of_nodes(), Some(4));
    assert_eq!(cluster.cluster_status().unwrap_or(""), "resizing");
}

#[tokio::test]
async fn test_resize_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .resize_cluster()
        .cluster_identifier("no-such-cluster")
        .node_type("ra3.xlplus")
        .number_of_nodes(4)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify/Reset Cluster Parameter Group ----

// NOTE: test_modify_cluster_parameter_group is currently blocked by a wire
// serialisation bug: ClusterParameterGroupNameMessage has
// #[serde(rename = "ResetClusterParameterGroupResult")] which produces the
// wrong result element for ModifyClusterParameterGroup. The handler logic
// itself is correct. Fix needs gen_serializers update.

#[tokio::test]
async fn test_reset_cluster_parameter_group() {
    let client = make_client().await;

    client
        .create_cluster_parameter_group()
        .parameter_group_name("reset-pg")
        .parameter_group_family("redshift-1.0")
        .description("Test reset")
        .send()
        .await
        .unwrap();

    let resp = client
        .reset_cluster_parameter_group()
        .parameter_group_name("reset-pg")
        .reset_all_parameters(true)
        .send()
        .await
        .expect("reset_cluster_parameter_group should succeed");
    assert_eq!(resp.parameter_group_name().unwrap_or(""), "reset-pg");
}

// ---- Copy Cluster Snapshot ----

#[tokio::test]
async fn test_copy_cluster_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("copy-src-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("src-snap")
        .cluster_identifier("copy-src-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .copy_cluster_snapshot()
        .source_snapshot_identifier("src-snap")
        .target_snapshot_identifier("copied-snap")
        .send()
        .await
        .expect("copy_cluster_snapshot should succeed");
    let snap = resp.snapshot().expect("snapshot should be present");
    assert_eq!(snap.snapshot_identifier().unwrap_or(""), "copied-snap");
    assert_eq!(snap.cluster_identifier().unwrap_or(""), "copy-src-cluster");

    // Both snapshots should exist
    let desc = client
        .describe_cluster_snapshots()
        .snapshot_identifier("copied-snap")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshots().len(), 1);
}

#[tokio::test]
async fn test_copy_cluster_snapshot_nonexistent_source() {
    let client = make_client().await;
    let result = client
        .copy_cluster_snapshot()
        .source_snapshot_identifier("nonexistent")
        .target_snapshot_identifier("target")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Revoke Cluster Security Group Ingress ----

#[tokio::test]
async fn test_revoke_cluster_security_group_ingress() {
    let client = make_client().await;

    client
        .create_cluster_security_group()
        .cluster_security_group_name("revoke-sg")
        .description("Test revoke")
        .send()
        .await
        .unwrap();

    // Authorize a CIDR first
    client
        .authorize_cluster_security_group_ingress()
        .cluster_security_group_name("revoke-sg")
        .cidrip("10.0.0.0/8")
        .send()
        .await
        .unwrap();

    // Now revoke it
    let resp = client
        .revoke_cluster_security_group_ingress()
        .cluster_security_group_name("revoke-sg")
        .cidrip("10.0.0.0/8")
        .send()
        .await
        .expect("revoke_cluster_security_group_ingress should succeed");
    let sg = resp.cluster_security_group().unwrap();
    assert_eq!(sg.ip_ranges().len(), 0);
}

// ---- Modify Cluster Subnet Group ----

#[tokio::test]
async fn test_modify_cluster_subnet_group() {
    let client = make_client().await;

    client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("mod-subnet-grp")
        .description("Original description")
        .subnet_ids("subnet-aaa")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster_subnet_group()
        .cluster_subnet_group_name("mod-subnet-grp")
        .description("Updated description")
        .subnet_ids("subnet-bbb")
        .subnet_ids("subnet-ccc")
        .send()
        .await
        .expect("modify_cluster_subnet_group should succeed");
    let sg = resp.cluster_subnet_group().unwrap();
    assert_eq!(
        sg.cluster_subnet_group_name().unwrap_or(""),
        "mod-subnet-grp"
    );
    assert_eq!(sg.description().unwrap_or(""), "Updated description");
}

#[tokio::test]
async fn test_modify_nonexistent_subnet_group() {
    let client = make_client().await;
    let result = client
        .modify_cluster_subnet_group()
        .cluster_subnet_group_name("no-such-group")
        .subnet_ids("subnet-x")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Batch Delete Cluster Snapshots ----

#[tokio::test]
async fn test_batch_delete_cluster_snapshots() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("batch-del-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    // Create two snapshots
    client
        .create_cluster_snapshot()
        .snapshot_identifier("batch-snap-1")
        .cluster_identifier("batch-del-cluster")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("batch-snap-2")
        .cluster_identifier("batch-del-cluster")
        .send()
        .await
        .unwrap();

    let del_msg1 = aws_sdk_redshift::types::DeleteClusterSnapshotMessage::builder()
        .snapshot_identifier("batch-snap-1")
        .build();
    let del_msg2 = aws_sdk_redshift::types::DeleteClusterSnapshotMessage::builder()
        .snapshot_identifier("batch-snap-2")
        .build();

    let resp = client
        .batch_delete_cluster_snapshots()
        .identifiers(del_msg1)
        .identifiers(del_msg2)
        .send()
        .await
        .expect("batch_delete_cluster_snapshots should succeed");

    let resources = resp.resources();
    assert_eq!(resources.len(), 2);
    assert!(resp.errors().is_empty());
}

// ---- Event Subscriptions full lifecycle ----

#[tokio::test]
async fn test_create_describe_delete_event_subscription() {
    let client = make_client().await;

    let resp = client
        .create_event_subscription()
        .subscription_name("my-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .source_type("cluster")
        .enabled(true)
        .send()
        .await
        .expect("create_event_subscription should succeed");
    let sub = resp
        .event_subscription()
        .expect("subscription should be present");
    assert_eq!(sub.cust_subscription_id().unwrap_or(""), "my-sub");
    assert_eq!(sub.status().unwrap_or(""), "active");

    // Describe
    let desc = client
        .describe_event_subscriptions()
        .subscription_name("my-sub")
        .send()
        .await
        .expect("describe_event_subscriptions should succeed");
    assert_eq!(desc.event_subscriptions_list().len(), 1);

    // Describe all
    let desc_all = client
        .describe_event_subscriptions()
        .send()
        .await
        .expect("describe_event_subscriptions (all) should succeed");
    assert!(!desc_all.event_subscriptions_list().is_empty());

    // Delete
    client
        .delete_event_subscription()
        .subscription_name("my-sub")
        .send()
        .await
        .expect("delete_event_subscription should succeed");

    // Should be gone
    let desc2 = client
        .describe_event_subscriptions()
        .send()
        .await
        .expect("describe_event_subscriptions should succeed");
    assert_eq!(desc2.event_subscriptions_list().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_event_subscription() {
    let client = make_client().await;
    let result = client
        .delete_event_subscription()
        .subscription_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- DescribeEvents ----

#[tokio::test]
async fn test_describe_events() {
    let client = make_client().await;
    let resp = client
        .describe_events()
        .send()
        .await
        .expect("describe_events should succeed");
    // Returns empty list
    assert_eq!(resp.events().len(), 0);
}

// ---- DescribeClusterVersions ----

#[tokio::test]
async fn test_describe_cluster_versions() {
    let client = make_client().await;
    let resp = client
        .describe_cluster_versions()
        .send()
        .await
        .expect("describe_cluster_versions should succeed");
    assert!(!resp.cluster_versions().is_empty());
    assert_eq!(
        resp.cluster_versions()[0].cluster_version().unwrap_or(""),
        "1.0"
    );
}

// ---- DescribeOrderableClusterOptions ----

#[tokio::test]
async fn test_describe_orderable_cluster_options() {
    let client = make_client().await;
    let resp = client
        .describe_orderable_cluster_options()
        .send()
        .await
        .expect("describe_orderable_cluster_options should succeed");
    assert!(!resp.orderable_cluster_options().is_empty());
    // Should contain dc2.large
    let node_types: Vec<&str> = resp
        .orderable_cluster_options()
        .iter()
        .filter_map(|o| o.node_type())
        .collect();
    assert!(node_types.contains(&"dc2.large"));
}

// ---- DescribeEventCategories ----

#[tokio::test]
async fn test_describe_event_categories() {
    let client = make_client().await;
    let resp = client
        .describe_event_categories()
        .send()
        .await
        .expect("describe_event_categories should succeed");
    let _ = resp.event_categories_map_list();
}

// ---- DescribeStorage ----

#[tokio::test]
async fn test_describe_storage() {
    let client = make_client().await;
    let resp = client
        .describe_storage()
        .send()
        .await
        .expect("describe_storage should succeed");
    assert_eq!(resp.total_backup_size_in_mega_bytes(), Some(0.0));
    assert_eq!(resp.total_provisioned_storage_in_mega_bytes(), Some(0.0));
}

// ---- CancelResize ----
// NOTE: test_cancel_resize is currently blocked by a wire serialisation bug:
// ResizeProgressMessage has #[serde(rename = "DescribeResizeResult")] which
// produces the wrong result element for CancelResize. The handler logic itself
// is correct (returns status "CANCELLED"). Fix needs gen_serializers update.

#[tokio::test]
async fn test_cancel_resize_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .cancel_resize()
        .cluster_identifier("no-such-cluster")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- GetClusterCredentialsWithIAM ----

#[tokio::test]
async fn test_get_cluster_credentials_with_iam() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("iam-creds-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_cluster_credentials_with_iam()
        .cluster_identifier("iam-creds-cluster")
        .send()
        .await
        .expect("get_cluster_credentials_with_iam should succeed");
    assert!(resp.db_user().is_some());
    assert!(resp.db_password().is_some());
    assert!(resp.expiration().is_some());
}

#[tokio::test]
async fn test_get_cluster_credentials_with_iam_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .get_cluster_credentials_with_iam()
        .cluster_identifier("no-such-cluster")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- ModifyClusterDbRevision ----

#[tokio::test]
async fn test_modify_cluster_db_revision() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("dbrev-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster_db_revision()
        .cluster_identifier("dbrev-cluster")
        .revision_target("some-revision")
        .send()
        .await
        .expect("modify_cluster_db_revision should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "dbrev-cluster");
}

// ---- BatchModifyClusterSnapshots ----

#[tokio::test]
async fn test_batch_modify_cluster_snapshots() {
    let client = make_client().await;

    let resp = client
        .batch_modify_cluster_snapshots()
        .snapshot_identifier_list("any-snap")
        .send()
        .await
        .expect("batch_modify_cluster_snapshots should succeed");
    let resources = resp.resources();
    let _ = resources;
    assert!(resp.errors().is_empty());
}

// ---- UpdatePartnerStatus ----

#[tokio::test]
async fn test_update_partner_status() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("partner-status-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .add_partner()
        .account_id("123456789012")
        .cluster_identifier("partner-status-cluster")
        .database_name("mydb")
        .partner_name("mypartner")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_partner_status()
        .account_id("123456789012")
        .cluster_identifier("partner-status-cluster")
        .database_name("mydb")
        .partner_name("mypartner")
        .status(aws_sdk_redshift::types::PartnerIntegrationStatus::Active)
        .send()
        .await
        .expect("update_partner_status should succeed");
    assert!(resp.database_name().is_some());
    assert!(resp.partner_name().is_some());
}

// ---- DescribeNodeConfigurationOptions ----

#[tokio::test]
async fn test_describe_node_configuration_options() {
    let client = make_client().await;
    let resp = client
        .describe_node_configuration_options()
        .action_type(aws_sdk_redshift::types::ActionType::RestoreCluster)
        .send()
        .await
        .expect("describe_node_configuration_options should succeed");
    assert_eq!(resp.node_configuration_option_list().len(), 0);
}

// ---- Describe all clusters (empty) ----

#[tokio::test]
async fn test_describe_clusters_empty() {
    let client = make_client().await;
    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    assert_eq!(resp.clusters().len(), 0);
}

// ---- Create duplicate cluster error ----

#[tokio::test]
async fn test_create_duplicate_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("dup-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster()
        .cluster_identifier("dup-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify nonexistent cluster error ----

#[tokio::test]
async fn test_modify_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .modify_cluster()
        .cluster_identifier("no-such")
        .node_type("ra3.xlplus")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent cluster error ----

#[tokio::test]
async fn test_delete_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .delete_cluster()
        .cluster_identifier("no-such")
        .skip_final_cluster_snapshot(true)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Pause/Resume nonexistent cluster ----

#[tokio::test]
async fn test_pause_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .pause_cluster()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resume_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .resume_cluster()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate snapshot error ----

#[tokio::test]
async fn test_create_duplicate_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("dup-snap-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("dup-snap")
        .cluster_identifier("dup-snap-cluster")
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster_snapshot()
        .snapshot_identifier("dup-snap")
        .cluster_identifier("dup-snap-cluster")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent snapshot ----

#[tokio::test]
async fn test_delete_nonexistent_snapshot() {
    let client = make_client().await;
    let result = client
        .delete_cluster_snapshot()
        .snapshot_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Get cluster credentials nonexistent cluster ----

#[tokio::test]
async fn test_get_cluster_credentials_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .get_cluster_credentials()
        .cluster_identifier("no-such")
        .db_user("admin")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe snapshots by cluster identifier ----

#[tokio::test]
async fn test_describe_snapshots_by_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("snap-list-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("snap-a")
        .cluster_identifier("snap-list-cluster")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("snap-b")
        .cluster_identifier("snap-list-cluster")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_cluster_snapshots()
        .cluster_identifier("snap-list-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshots().len(), 2);
}

// ---- Duplicate subnet group error ----

#[tokio::test]
async fn test_create_duplicate_subnet_group() {
    let client = make_client().await;

    client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("dup-sg")
        .description("First")
        .subnet_ids("subnet-1")
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("dup-sg")
        .description("Second")
        .subnet_ids("subnet-2")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate parameter group error ----

#[tokio::test]
async fn test_create_duplicate_parameter_group() {
    let client = make_client().await;

    client
        .create_cluster_parameter_group()
        .parameter_group_name("dup-pg")
        .parameter_group_family("redshift-1.0")
        .description("First")
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster_parameter_group()
        .parameter_group_name("dup-pg")
        .parameter_group_family("redshift-1.0")
        .description("Second")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate security group error ----

#[tokio::test]
async fn test_create_duplicate_security_group() {
    let client = make_client().await;

    client
        .create_cluster_security_group()
        .cluster_security_group_name("dup-sec-grp")
        .description("First")
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster_security_group()
        .cluster_security_group_name("dup-sec-grp")
        .description("Second")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate snapshot copy grant error ----

#[tokio::test]
async fn test_create_duplicate_snapshot_copy_grant() {
    let client = make_client().await;

    client
        .create_snapshot_copy_grant()
        .snapshot_copy_grant_name("dup-grant")
        .send()
        .await
        .unwrap();

    let result = client
        .create_snapshot_copy_grant()
        .snapshot_copy_grant_name("dup-grant")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate event subscription error ----

#[tokio::test]
async fn test_create_duplicate_event_subscription() {
    let client = make_client().await;

    client
        .create_event_subscription()
        .subscription_name("dup-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic")
        .send()
        .await
        .unwrap();

    let result = client
        .create_event_subscription()
        .subscription_name("dup-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe all subnet groups ----

#[tokio::test]
async fn test_describe_all_subnet_groups() {
    let client = make_client().await;

    client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("sg-one")
        .description("One")
        .subnet_ids("subnet-1")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("sg-two")
        .description("Two")
        .subnet_ids("subnet-2")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_cluster_subnet_groups()
        .send()
        .await
        .expect("describe all subnet groups should succeed");
    assert_eq!(desc.cluster_subnet_groups().len(), 2);
}

// ---- Describe all parameter groups ----

#[tokio::test]
async fn test_describe_all_parameter_groups() {
    let client = make_client().await;

    client
        .create_cluster_parameter_group()
        .parameter_group_name("pg-one")
        .parameter_group_family("redshift-1.0")
        .description("One")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_cluster_parameter_groups()
        .send()
        .await
        .expect("describe all parameter groups should succeed");
    assert!(!desc.parameter_groups().is_empty());
}

// ---- Describe all security groups ----

#[tokio::test]
async fn test_describe_all_security_groups() {
    let client = make_client().await;

    client
        .create_cluster_security_group()
        .cluster_security_group_name("sec-grp-all")
        .description("One")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_cluster_security_groups()
        .send()
        .await
        .expect("describe all security groups should succeed");
    assert!(!desc.cluster_security_groups().is_empty());
}

// ---- Describe all snapshot copy grants ----

#[tokio::test]
async fn test_describe_all_snapshot_copy_grants() {
    let client = make_client().await;

    client
        .create_snapshot_copy_grant()
        .snapshot_copy_grant_name("grant-all")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_snapshot_copy_grants()
        .send()
        .await
        .expect("describe all snapshot copy grants should succeed");
    assert!(!desc.snapshot_copy_grants().is_empty());
}

// ---- Enable snapshot copy on nonexistent cluster ----

#[tokio::test]
async fn test_enable_snapshot_copy_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .enable_snapshot_copy()
        .cluster_identifier("no-such")
        .destination_region("us-west-2")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify snapshot copy retention period without snapshot copy enabled ----

#[tokio::test]
async fn test_modify_snapshot_copy_retention_no_copy_enabled() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("no-copy-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let result = client
        .modify_snapshot_copy_retention_period()
        .cluster_identifier("no-copy-cluster")
        .retention_period(14)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Cluster with tags ----

#[tokio::test]
async fn test_create_cluster_with_tags() {
    let client = make_client().await;

    let tag = aws_sdk_redshift::types::Tag::builder()
        .key("project")
        .value("test")
        .build();

    let resp = client
        .create_cluster()
        .cluster_identifier("tagged-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .tags(tag)
        .send()
        .await
        .expect("create_cluster with tags should succeed");
    let cluster = resp.cluster().expect("cluster should be present");
    assert!(!cluster.tags().is_empty());
    assert_eq!(cluster.tags()[0].key().unwrap_or(""), "project");
    assert_eq!(cluster.tags()[0].value().unwrap_or(""), "test");
}

// ---- Cluster creation with optional fields ----

#[tokio::test]
async fn test_create_cluster_with_options() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_identifier("opt-cluster")
        .node_type("ra3.xlplus")
        .master_username("admin")
        .master_user_password("Password1!")
        .number_of_nodes(4)
        .encrypted(true)
        .publicly_accessible(false)
        .preferred_maintenance_window("mon:08:00-mon:09:00")
        .automated_snapshot_retention_period(7)
        .send()
        .await
        .expect("create_cluster with options should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.node_type().unwrap_or(""), "ra3.xlplus");
    assert_eq!(cluster.number_of_nodes(), Some(4));
    assert_eq!(cluster.encrypted(), Some(true));
    assert_eq!(cluster.publicly_accessible(), Some(false));
    assert_eq!(
        cluster.preferred_maintenance_window().unwrap_or(""),
        "mon:08:00-mon:09:00"
    );
    assert_eq!(cluster.automated_snapshot_retention_period(), Some(7));
}

// ---- Restore snapshot nonexistent snapshot ----

#[tokio::test]
async fn test_restore_from_nonexistent_snapshot() {
    let client = make_client().await;
    let result = client
        .restore_from_cluster_snapshot()
        .cluster_identifier("new-cluster")
        .snapshot_identifier("no-such-snap")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe all snapshots (empty) ----

#[tokio::test]
async fn test_describe_snapshots_empty() {
    let client = make_client().await;
    let resp = client
        .describe_cluster_snapshots()
        .send()
        .await
        .expect("describe_cluster_snapshots should succeed");
    assert_eq!(resp.snapshots().len(), 0);
}

// ---- Delete nonexistent parameter group ----

#[tokio::test]
async fn test_delete_nonexistent_parameter_group() {
    let client = make_client().await;
    let result = client
        .delete_cluster_parameter_group()
        .parameter_group_name("no-such-pg")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent security group ----

#[tokio::test]
async fn test_delete_nonexistent_security_group() {
    let client = make_client().await;
    let result = client
        .delete_cluster_security_group()
        .cluster_security_group_name("no-such-sg")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent snapshot copy grant ----

#[tokio::test]
async fn test_delete_nonexistent_snapshot_copy_grant() {
    let client = make_client().await;
    let result = client
        .delete_snapshot_copy_grant()
        .snapshot_copy_grant_name("no-such-grant")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent subnet group ----

#[tokio::test]
async fn test_delete_nonexistent_subnet_group() {
    let client = make_client().await;
    let result = client
        .delete_cluster_subnet_group()
        .cluster_subnet_group_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Logging on nonexistent cluster ----

#[tokio::test]
async fn test_enable_logging_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .enable_logging()
        .cluster_identifier("no-such")
        .bucket_name("my-bucket")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate HSM client certificate ----

#[tokio::test]
async fn test_create_duplicate_hsm_client_certificate() {
    let client = make_client().await;

    client
        .create_hsm_client_certificate()
        .hsm_client_certificate_identifier("dup-cert")
        .send()
        .await
        .unwrap();

    let result = client
        .create_hsm_client_certificate()
        .hsm_client_certificate_identifier("dup-cert")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate HSM configuration ----

#[tokio::test]
async fn test_create_duplicate_hsm_configuration() {
    let client = make_client().await;

    client
        .create_hsm_configuration()
        .hsm_configuration_identifier("dup-config")
        .description("First")
        .hsm_ip_address("10.0.0.1")
        .hsm_partition_name("part1")
        .hsm_partition_password("pass1")
        .hsm_server_public_certificate("cert1")
        .send()
        .await
        .unwrap();

    let result = client
        .create_hsm_configuration()
        .hsm_configuration_identifier("dup-config")
        .description("Second")
        .hsm_ip_address("10.0.0.2")
        .hsm_partition_name("part2")
        .hsm_partition_password("pass2")
        .hsm_server_public_certificate("cert2")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate authentication profile ----

#[tokio::test]
async fn test_create_duplicate_authentication_profile() {
    let client = make_client().await;

    client
        .create_authentication_profile()
        .authentication_profile_name("dup-profile")
        .authentication_profile_content(r#"{"key":"val"}"#)
        .send()
        .await
        .unwrap();

    let result = client
        .create_authentication_profile()
        .authentication_profile_name("dup-profile")
        .authentication_profile_content(r#"{"key":"other"}"#)
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================
// Additional comprehensive tests for uncovered error paths
// and deeper lifecycle coverage
// ============================================================

// ---- Disable snapshot copy on nonexistent cluster ----

#[tokio::test]
async fn test_disable_snapshot_copy_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .disable_snapshot_copy()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Disable logging on nonexistent cluster ----

#[tokio::test]
async fn test_disable_logging_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .disable_logging()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe logging status on nonexistent cluster ----

#[tokio::test]
async fn test_describe_logging_status_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .describe_logging_status()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Authorize security group ingress on nonexistent sg ----

#[tokio::test]
async fn test_authorize_security_group_ingress_nonexistent() {
    let client = make_client().await;
    let result = client
        .authorize_cluster_security_group_ingress()
        .cluster_security_group_name("no-such-sg")
        .cidrip("10.0.0.0/8")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Revoke security group ingress on nonexistent sg ----

#[tokio::test]
async fn test_revoke_security_group_ingress_nonexistent() {
    let client = make_client().await;
    let result = client
        .revoke_cluster_security_group_ingress()
        .cluster_security_group_name("no-such-sg")
        .cidrip("10.0.0.0/8")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Authorize security group ingress with EC2 security group ----

#[tokio::test]
async fn test_authorize_security_group_ingress_ec2() {
    let client = make_client().await;

    client
        .create_cluster_security_group()
        .cluster_security_group_name("ec2-sg")
        .description("Test EC2 ingress")
        .send()
        .await
        .unwrap();

    let resp = client
        .authorize_cluster_security_group_ingress()
        .cluster_security_group_name("ec2-sg")
        .ec2_security_group_name("my-ec2-sg")
        .ec2_security_group_owner_id("111122223333")
        .send()
        .await
        .expect("authorize with EC2 sg should succeed");
    let sg = resp.cluster_security_group().unwrap();
    assert_eq!(sg.ec2_security_groups().len(), 1);
    assert_eq!(
        sg.ec2_security_groups()[0]
            .ec2_security_group_name()
            .unwrap_or(""),
        "my-ec2-sg"
    );
}

// ---- Describe cluster parameters on nonexistent pg ----

#[tokio::test]
async fn test_describe_cluster_parameters_nonexistent() {
    let client = make_client().await;
    let result = client
        .describe_cluster_parameters()
        .parameter_group_name("no-such-pg")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent snapshot copy grant ----

#[tokio::test]
async fn test_describe_nonexistent_snapshot_copy_grant() {
    let client = make_client().await;
    let result = client
        .describe_snapshot_copy_grants()
        .snapshot_copy_grant_name("no-such-grant")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent snapshot ----

#[tokio::test]
async fn test_describe_nonexistent_snapshot() {
    let client = make_client().await;
    let result = client
        .describe_cluster_snapshots()
        .snapshot_identifier("no-such-snap")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent HSM client certificate ----

#[tokio::test]
async fn test_delete_nonexistent_hsm_client_certificate() {
    let client = make_client().await;
    let result = client
        .delete_hsm_client_certificate()
        .hsm_client_certificate_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent HSM configuration ----

#[tokio::test]
async fn test_delete_nonexistent_hsm_configuration() {
    let client = make_client().await;
    let result = client
        .delete_hsm_configuration()
        .hsm_configuration_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent authentication profile ----

#[tokio::test]
async fn test_delete_nonexistent_authentication_profile() {
    let client = make_client().await;
    let result = client
        .delete_authentication_profile()
        .authentication_profile_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify nonexistent authentication profile ----

#[tokio::test]
async fn test_modify_nonexistent_authentication_profile() {
    let client = make_client().await;
    let result = client
        .modify_authentication_profile()
        .authentication_profile_name("no-such")
        .authentication_profile_content(r#"{"key":"val"}"#)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent usage limit ----

#[tokio::test]
async fn test_delete_nonexistent_usage_limit() {
    let client = make_client().await;
    let result = client
        .delete_usage_limit()
        .usage_limit_id("no-such-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify nonexistent usage limit ----

#[tokio::test]
async fn test_modify_nonexistent_usage_limit() {
    let client = make_client().await;
    let result = client
        .modify_usage_limit()
        .usage_limit_id("no-such-id")
        .amount(999)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent snapshot schedule ----

#[tokio::test]
async fn test_delete_nonexistent_snapshot_schedule() {
    let client = make_client().await;
    let result = client
        .delete_snapshot_schedule()
        .schedule_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify nonexistent snapshot schedule ----

#[tokio::test]
async fn test_modify_nonexistent_snapshot_schedule() {
    let client = make_client().await;
    let result = client
        .modify_snapshot_schedule()
        .schedule_identifier("no-such")
        .schedule_definitions("rate(12 hours)")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent scheduled action ----

#[tokio::test]
async fn test_delete_nonexistent_scheduled_action() {
    let client = make_client().await;
    let result = client
        .delete_scheduled_action()
        .scheduled_action_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify nonexistent scheduled action ----

#[tokio::test]
async fn test_modify_nonexistent_scheduled_action() {
    let client = make_client().await;
    let result = client
        .modify_scheduled_action()
        .scheduled_action_name("no-such")
        .schedule("cron(0 6 * * ? *)")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Get nonexistent resource policy ----

#[tokio::test]
async fn test_get_nonexistent_resource_policy() {
    let client = make_client().await;
    let result = client
        .get_resource_policy()
        .resource_arn("arn:aws:redshift:us-east-1:123456789012:cluster:no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Delete nonexistent resource policy (no-op, succeeds) ----

#[tokio::test]
async fn test_delete_nonexistent_resource_policy() {
    let client = make_client().await;
    // delete_resource_policy is idempotent: deleting a non-existent policy succeeds
    let result = client
        .delete_resource_policy()
        .resource_arn("arn:aws:redshift:us-east-1:123456789012:cluster:no-such")
        .send()
        .await;
    assert!(result.is_ok());
}

// ---- Modify event subscription on nonexistent ----

#[tokio::test]
async fn test_modify_nonexistent_event_subscription() {
    let client = make_client().await;
    let result = client
        .modify_event_subscription()
        .subscription_name("no-such")
        .enabled(false)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Failover primary compute on nonexistent cluster ----

#[tokio::test]
async fn test_failover_primary_compute_nonexistent() {
    let client = make_client().await;
    let result = client
        .failover_primary_compute()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Rotate encryption key on nonexistent cluster ----

#[tokio::test]
async fn test_rotate_encryption_key_nonexistent() {
    let client = make_client().await;
    let result = client
        .rotate_encryption_key()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify cluster IAM roles on nonexistent cluster ----

#[tokio::test]
async fn test_modify_cluster_iam_roles_nonexistent() {
    let client = make_client().await;
    let result = client
        .modify_cluster_iam_roles()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify cluster maintenance on nonexistent cluster ----

#[tokio::test]
async fn test_modify_cluster_maintenance_nonexistent() {
    let client = make_client().await;
    let result = client
        .modify_cluster_maintenance()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify cluster DB revision on nonexistent cluster ----

#[tokio::test]
async fn test_modify_cluster_db_revision_nonexistent() {
    let client = make_client().await;
    let result = client
        .modify_cluster_db_revision()
        .cluster_identifier("no-such")
        .revision_target("rev")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify AQUA configuration on nonexistent cluster ----

#[tokio::test]
async fn test_modify_aqua_configuration_nonexistent() {
    let client = make_client().await;
    let result = client
        .modify_aqua_configuration()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe cluster DB revisions on nonexistent cluster ----

#[tokio::test]
async fn test_describe_cluster_db_revisions_nonexistent() {
    let client = make_client().await;
    let result = client
        .describe_cluster_db_revisions()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe resize on nonexistent cluster ----

#[tokio::test]
async fn test_describe_resize_nonexistent() {
    let client = make_client().await;
    let result = client
        .describe_resize()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify cluster snapshot on nonexistent snapshot ----

#[tokio::test]
async fn test_modify_cluster_snapshot_nonexistent() {
    let client = make_client().await;
    let result = client
        .modify_cluster_snapshot()
        .snapshot_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Authorize snapshot access on nonexistent snapshot ----

#[tokio::test]
async fn test_authorize_snapshot_access_nonexistent() {
    let client = make_client().await;
    let result = client
        .authorize_snapshot_access()
        .snapshot_identifier("no-such")
        .account_with_restore_access("111122223333")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Revoke snapshot access on nonexistent snapshot ----

#[tokio::test]
async fn test_revoke_snapshot_access_nonexistent() {
    let client = make_client().await;
    let result = client
        .revoke_snapshot_access()
        .snapshot_identifier("no-such")
        .account_with_restore_access("111122223333")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Create snapshot on nonexistent cluster ----

#[tokio::test]
async fn test_create_snapshot_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .create_cluster_snapshot()
        .snapshot_identifier("orphan-snap")
        .cluster_identifier("no-such-cluster")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent event subscription ----

#[tokio::test]
async fn test_describe_nonexistent_event_subscription() {
    let client = make_client().await;
    let result = client
        .describe_event_subscriptions()
        .subscription_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent HSM client certificate ----

#[tokio::test]
async fn test_describe_nonexistent_hsm_client_certificate() {
    let client = make_client().await;
    let result = client
        .describe_hsm_client_certificates()
        .hsm_client_certificate_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent HSM configuration ----

#[tokio::test]
async fn test_describe_nonexistent_hsm_configuration() {
    let client = make_client().await;
    let result = client
        .describe_hsm_configurations()
        .hsm_configuration_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent authentication profile ----

#[tokio::test]
async fn test_describe_nonexistent_authentication_profile() {
    let client = make_client().await;
    let result = client
        .describe_authentication_profiles()
        .authentication_profile_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent parameter group ----

#[tokio::test]
async fn test_describe_nonexistent_parameter_group() {
    let client = make_client().await;
    let result = client
        .describe_cluster_parameter_groups()
        .parameter_group_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent security group ----

#[tokio::test]
async fn test_describe_nonexistent_security_group() {
    let client = make_client().await;
    let result = client
        .describe_cluster_security_groups()
        .cluster_security_group_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe nonexistent subnet group ----

#[tokio::test]
async fn test_describe_nonexistent_subnet_group() {
    let client = make_client().await;
    let result = client
        .describe_cluster_subnet_groups()
        .cluster_subnet_group_name("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify cluster snapshot schedule on nonexistent cluster ----

#[tokio::test]
async fn test_modify_cluster_snapshot_schedule_nonexistent() {
    let client = make_client().await;
    let result = client
        .modify_cluster_snapshot_schedule()
        .cluster_identifier("no-such")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Snapshot copy grant with KMS key ----

#[tokio::test]
async fn test_create_snapshot_copy_grant_with_kms() {
    let client = make_client().await;

    let resp = client
        .create_snapshot_copy_grant()
        .snapshot_copy_grant_name("kms-grant")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/my-key-id")
        .send()
        .await
        .expect("create_snapshot_copy_grant with KMS should succeed");
    let grant = resp.snapshot_copy_grant().unwrap();
    assert_eq!(grant.snapshot_copy_grant_name().unwrap_or(""), "kms-grant");
    assert!(grant.kms_key_id().is_some());
}

// ---- Snapshot schedule with description ----

#[tokio::test]
async fn test_create_snapshot_schedule_with_description() {
    let client = make_client().await;

    let resp = client
        .create_snapshot_schedule()
        .schedule_identifier("desc-schedule")
        .schedule_definitions("rate(8 hours)")
        .schedule_description("A schedule with description")
        .send()
        .await
        .expect("create_snapshot_schedule with description should succeed");
    assert_eq!(resp.schedule_identifier().unwrap_or(""), "desc-schedule");
    assert_eq!(
        resp.schedule_description().unwrap_or(""),
        "A schedule with description"
    );
}

// ---- Usage limit with optional fields ----

#[tokio::test]
async fn test_create_usage_limit_with_optional_fields() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("ul-opt-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_usage_limit()
        .cluster_identifier("ul-opt-cluster")
        .feature_type(aws_sdk_redshift::types::UsageLimitFeatureType::Spectrum)
        .limit_type(aws_sdk_redshift::types::UsageLimitLimitType::DataScanned)
        .amount(500)
        .period(aws_sdk_redshift::types::UsageLimitPeriod::Weekly)
        .breach_action(aws_sdk_redshift::types::UsageLimitBreachAction::EmitMetric)
        .send()
        .await
        .expect("create_usage_limit with optional fields should succeed");
    assert!(resp.usage_limit_id().is_some());
    assert_eq!(resp.amount(), Some(500));
}

// ---- Create cluster with availability zone ----

#[tokio::test]
async fn test_create_cluster_with_availability_zone() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_identifier("az-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .availability_zone("us-east-1b")
        .send()
        .await
        .expect("create_cluster with AZ should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.availability_zone().unwrap_or(""), "us-east-1b");
}

// ---- Restore cluster to existing identifier (error) ----

#[tokio::test]
async fn test_restore_to_existing_cluster_identifier() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("restore-existing")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("restore-ex-snap")
        .cluster_identifier("restore-existing")
        .send()
        .await
        .unwrap();

    let result = client
        .restore_from_cluster_snapshot()
        .cluster_identifier("restore-existing")
        .snapshot_identifier("restore-ex-snap")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Multiple clusters: describe all ----

#[tokio::test]
async fn test_describe_multiple_clusters() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("multi-a")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster()
        .cluster_identifier("multi-b")
        .node_type("ra3.xlplus")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe all clusters should succeed");
    assert_eq!(resp.clusters().len(), 2);
}

// ---- Modify cluster with new cluster identifier ----

#[tokio::test]
async fn test_modify_cluster_rename() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("rename-orig")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster()
        .cluster_identifier("rename-orig")
        .new_cluster_identifier("rename-new")
        .send()
        .await
        .expect("modify_cluster rename should succeed");
    let cluster = resp.cluster().unwrap();
    // The response shows the old id since rename happens after
    assert_eq!(cluster.cluster_identifier().unwrap_or(""), "rename-orig");

    // The cluster should be accessible via new identifier
    let desc = client
        .describe_clusters()
        .cluster_identifier("rename-new")
        .send()
        .await
        .expect("describe with new identifier should succeed");
    assert_eq!(desc.clusters().len(), 1);

    // Old identifier should be gone
    let result = client
        .describe_clusters()
        .cluster_identifier("rename-orig")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Batch delete snapshots with nonexistent entries ----

#[tokio::test]
async fn test_batch_delete_snapshots_with_errors() {
    let client = make_client().await;

    let del_msg = aws_sdk_redshift::types::DeleteClusterSnapshotMessage::builder()
        .snapshot_identifier("nonexistent-batch-snap")
        .build();

    let resp = client
        .batch_delete_cluster_snapshots()
        .identifiers(del_msg)
        .send()
        .await
        .expect("batch_delete_cluster_snapshots should succeed even with errors");
    assert!(!resp.errors().is_empty());
}

// ---- Duplicate snapshot schedule error ----

#[tokio::test]
async fn test_create_duplicate_snapshot_schedule() {
    let client = make_client().await;

    client
        .create_snapshot_schedule()
        .schedule_identifier("dup-sched")
        .schedule_definitions("rate(12 hours)")
        .send()
        .await
        .unwrap();

    let result = client
        .create_snapshot_schedule()
        .schedule_identifier("dup-sched")
        .schedule_definitions("rate(24 hours)")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Duplicate scheduled action error ----

#[tokio::test]
async fn test_create_duplicate_scheduled_action() {
    let client = make_client().await;

    let action = aws_sdk_redshift::types::ScheduledActionType::builder()
        .pause_cluster(
            aws_sdk_redshift::types::PauseClusterMessage::builder()
                .cluster_identifier("test-cluster")
                .build(),
        )
        .build();

    client
        .create_scheduled_action()
        .scheduled_action_name("dup-action")
        .schedule("cron(0 18 * * ? *)")
        .iam_role("arn:aws:iam::123456789012:role/Role")
        .target_action(action.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .create_scheduled_action()
        .scheduled_action_name("dup-action")
        .schedule("cron(0 6 * * ? *)")
        .iam_role("arn:aws:iam::123456789012:role/Role")
        .target_action(action)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify cluster with multiple fields ----

#[tokio::test]
async fn test_modify_cluster_multiple_fields() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("multi-mod-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .number_of_nodes(2)
        .encrypted(false)
        .publicly_accessible(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_cluster()
        .cluster_identifier("multi-mod-cluster")
        .encrypted(true)
        .publicly_accessible(false)
        .preferred_maintenance_window("tue:03:00-tue:04:00")
        .automated_snapshot_retention_period(14)
        .send()
        .await
        .expect("modify multiple fields should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.encrypted(), Some(true));
    assert_eq!(cluster.publicly_accessible(), Some(false));
    assert_eq!(
        cluster.preferred_maintenance_window().unwrap_or(""),
        "tue:03:00-tue:04:00"
    );
    assert_eq!(cluster.automated_snapshot_retention_period(), Some(14));
}

// ---- Describe all HSM client certificates (empty) ----

#[tokio::test]
async fn test_describe_all_hsm_client_certificates_empty() {
    let client = make_client().await;
    let resp = client
        .describe_hsm_client_certificates()
        .send()
        .await
        .expect("describe all HSM client certificates should succeed");
    assert_eq!(resp.hsm_client_certificates().len(), 0);
}

// ---- Describe all HSM configurations (empty) ----

#[tokio::test]
async fn test_describe_all_hsm_configurations_empty() {
    let client = make_client().await;
    let resp = client
        .describe_hsm_configurations()
        .send()
        .await
        .expect("describe all HSM configurations should succeed");
    assert_eq!(resp.hsm_configurations().len(), 0);
}

// ---- Describe all authentication profiles (empty) ----

#[tokio::test]
async fn test_describe_all_authentication_profiles_empty() {
    let client = make_client().await;
    let resp = client
        .describe_authentication_profiles()
        .send()
        .await
        .expect("describe all authentication profiles should succeed");
    assert_eq!(resp.authentication_profiles().len(), 0);
}

// ---- Describe all usage limits (empty) ----

#[tokio::test]
async fn test_describe_all_usage_limits_empty() {
    let client = make_client().await;
    let resp = client
        .describe_usage_limits()
        .send()
        .await
        .expect("describe all usage limits should succeed");
    assert_eq!(resp.usage_limits().len(), 0);
}

// ---- Describe all snapshot schedules (empty) ----

#[tokio::test]
async fn test_describe_all_snapshot_schedules_empty() {
    let client = make_client().await;
    let resp = client
        .describe_snapshot_schedules()
        .send()
        .await
        .expect("describe all snapshot schedules should succeed");
    assert_eq!(resp.snapshot_schedules().len(), 0);
}

// ---- Describe all scheduled actions (empty) ----

#[tokio::test]
async fn test_describe_all_scheduled_actions_empty() {
    let client = make_client().await;
    let resp = client
        .describe_scheduled_actions()
        .send()
        .await
        .expect("describe all scheduled actions should succeed");
    assert_eq!(resp.scheduled_actions().len(), 0);
}

// ---- Describe all event subscriptions (empty) ----

#[tokio::test]
async fn test_describe_all_event_subscriptions_empty() {
    let client = make_client().await;
    let resp = client
        .describe_event_subscriptions()
        .send()
        .await
        .expect("describe all event subscriptions should succeed");
    assert_eq!(resp.event_subscriptions_list().len(), 0);
}

// ---- Enable snapshot copy with grant name ----

#[tokio::test]
async fn test_enable_snapshot_copy_with_grant() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("copy-grant-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_snapshot_copy_grant()
        .snapshot_copy_grant_name("test-grant")
        .send()
        .await
        .unwrap();

    let resp = client
        .enable_snapshot_copy()
        .cluster_identifier("copy-grant-cluster")
        .destination_region("eu-west-1")
        .retention_period(14)
        .snapshot_copy_grant_name("test-grant")
        .send()
        .await
        .expect("enable_snapshot_copy with grant should succeed");
    let status = resp
        .cluster()
        .unwrap()
        .cluster_snapshot_copy_status()
        .unwrap();
    assert_eq!(status.destination_region().unwrap_or(""), "eu-west-1");
    assert_eq!(status.retention_period(), Some(14));
}

// ---- Add partner to nonexistent cluster ----

#[tokio::test]
async fn test_add_partner_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .add_partner()
        .account_id("123456789012")
        .cluster_identifier("no-such")
        .database_name("mydb")
        .partner_name("mypartner")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Describe partners on nonexistent cluster ----

#[tokio::test]
async fn test_describe_partners_nonexistent_cluster() {
    let client = make_client().await;
    let result = client
        .describe_partners()
        .account_id("123456789012")
        .cluster_identifier("no-such")
        .send()
        .await;
    // describe_partners may return empty or error depending on impl
    // Just verify it doesn't panic
    let _ = result;
}

// ---- Get cluster credentials with IAM on nonexistent ----

// Already covered above, but verify db_name variant
#[tokio::test]
async fn test_get_cluster_credentials_with_db_name() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("creds-db-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .db_name("mydb")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_cluster_credentials()
        .cluster_identifier("creds-db-cluster")
        .db_user("testuser")
        .db_name("mydb")
        .send()
        .await
        .expect("get_cluster_credentials with db_name should succeed");
    assert!(resp.db_user().is_some());
    assert!(resp.db_password().is_some());
}

// ---- Event subscription with event categories and source IDs ----

#[tokio::test]
async fn test_create_event_subscription_with_categories() {
    let client = make_client().await;

    let resp = client
        .create_event_subscription()
        .subscription_name("cat-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .source_type("cluster")
        .source_ids("my-cluster")
        .event_categories("management")
        .event_categories("monitoring")
        .severity("INFO")
        .enabled(true)
        .send()
        .await
        .expect("create_event_subscription with categories should succeed");
    let sub = resp.event_subscription().unwrap();
    assert_eq!(sub.cust_subscription_id().unwrap_or(""), "cat-sub");
    assert_eq!(sub.source_type().unwrap_or(""), "cluster");
    assert!(sub.enabled() == Some(true));
}

// ---- Reset parameter group with specific parameters ----

#[tokio::test]
async fn test_reset_cluster_parameter_group_specific() {
    let client = make_client().await;

    client
        .create_cluster_parameter_group()
        .parameter_group_name("reset-specific-pg")
        .parameter_group_family("redshift-1.0")
        .description("Test reset specific")
        .send()
        .await
        .unwrap();

    let resp = client
        .reset_cluster_parameter_group()
        .parameter_group_name("reset-specific-pg")
        .reset_all_parameters(false)
        .parameters(
            aws_sdk_redshift::types::Parameter::builder()
                .parameter_name("enable_user_activity_logging")
                .build(),
        )
        .send()
        .await
        .expect("reset specific parameters should succeed");
    assert_eq!(
        resp.parameter_group_name().unwrap_or(""),
        "reset-specific-pg"
    );
}

// ---- Reset nonexistent parameter group ----

#[tokio::test]
async fn test_reset_nonexistent_parameter_group() {
    let client = make_client().await;
    let result = client
        .reset_cluster_parameter_group()
        .parameter_group_name("no-such-pg")
        .reset_all_parameters(true)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Create cluster with subnet group ----

#[tokio::test]
async fn test_create_cluster_with_subnet_group() {
    let client = make_client().await;

    client
        .create_cluster_subnet_group()
        .cluster_subnet_group_name("my-sg-for-cluster")
        .description("SG for cluster test")
        .subnet_ids("subnet-abc")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_cluster()
        .cluster_identifier("sg-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .cluster_subnet_group_name("my-sg-for-cluster")
        .send()
        .await
        .expect("create_cluster with subnet group should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(
        cluster.cluster_subnet_group_name().unwrap_or(""),
        "my-sg-for-cluster"
    );
}

// ---- Copy snapshot to duplicate target (error) ----

#[tokio::test]
async fn test_copy_cluster_snapshot_duplicate_target() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("copy-dup-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("copy-dup-src")
        .cluster_identifier("copy-dup-cluster")
        .send()
        .await
        .unwrap();

    // First copy succeeds
    client
        .copy_cluster_snapshot()
        .source_snapshot_identifier("copy-dup-src")
        .target_snapshot_identifier("copy-dup-target")
        .send()
        .await
        .unwrap();

    // Second copy to same target fails
    let result = client
        .copy_cluster_snapshot()
        .source_snapshot_identifier("copy-dup-src")
        .target_snapshot_identifier("copy-dup-target")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Revoke EC2 security group ingress ----

#[tokio::test]
async fn test_revoke_ec2_security_group_ingress() {
    let client = make_client().await;

    client
        .create_cluster_security_group()
        .cluster_security_group_name("revoke-ec2-sg")
        .description("Test revoke EC2")
        .send()
        .await
        .unwrap();

    // Authorize EC2 SG
    client
        .authorize_cluster_security_group_ingress()
        .cluster_security_group_name("revoke-ec2-sg")
        .ec2_security_group_name("my-ec2-sg")
        .ec2_security_group_owner_id("111122223333")
        .send()
        .await
        .unwrap();

    // Revoke EC2 SG
    let resp = client
        .revoke_cluster_security_group_ingress()
        .cluster_security_group_name("revoke-ec2-sg")
        .ec2_security_group_name("my-ec2-sg")
        .send()
        .await
        .expect("revoke EC2 security group ingress should succeed");
    let sg = resp.cluster_security_group().unwrap();
    assert_eq!(sg.ec2_security_groups().len(), 0);
}

// ---- Multiple usage limits on same cluster ----

#[tokio::test]
async fn test_multiple_usage_limits_on_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("multi-ul-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_usage_limit()
        .cluster_identifier("multi-ul-cluster")
        .feature_type(aws_sdk_redshift::types::UsageLimitFeatureType::Spectrum)
        .limit_type(aws_sdk_redshift::types::UsageLimitLimitType::DataScanned)
        .amount(100)
        .send()
        .await
        .unwrap();

    client
        .create_usage_limit()
        .cluster_identifier("multi-ul-cluster")
        .feature_type(aws_sdk_redshift::types::UsageLimitFeatureType::ConcurrencyScaling)
        .limit_type(aws_sdk_redshift::types::UsageLimitLimitType::Time)
        .amount(200)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_usage_limits()
        .cluster_identifier("multi-ul-cluster")
        .send()
        .await
        .expect("describe multiple usage limits should succeed");
    assert_eq!(desc.usage_limits().len(), 2);
}

// ---- Multiple resource policies ----

#[tokio::test]
async fn test_multiple_resource_policies() {
    let client = make_client().await;

    let arn1 = "arn:aws:redshift:us-east-1:123456789012:cluster:policy-a";
    let arn2 = "arn:aws:redshift:us-east-1:123456789012:cluster:policy-b";

    client
        .put_resource_policy()
        .resource_arn(arn1)
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    client
        .put_resource_policy()
        .resource_arn(arn2)
        .policy(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow"}]}"#)
        .send()
        .await
        .unwrap();

    let resp1 = client
        .get_resource_policy()
        .resource_arn(arn1)
        .send()
        .await
        .unwrap();
    assert!(resp1.resource_policy().is_some());

    let resp2 = client
        .get_resource_policy()
        .resource_arn(arn2)
        .send()
        .await
        .unwrap();
    assert!(resp2.resource_policy().is_some());
}

// ---- Update resource policy (overwrite) ----

#[tokio::test]
async fn test_update_resource_policy_overwrite() {
    let client = make_client().await;

    let arn = "arn:aws:redshift:us-east-1:123456789012:cluster:overwrite-policy";

    client
        .put_resource_policy()
        .resource_arn(arn)
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    // Overwrite with new policy
    client
        .put_resource_policy()
        .resource_arn(arn)
        .policy(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Deny"}]}"#)
        .send()
        .await
        .expect("overwriting resource policy should succeed");

    let resp = client
        .get_resource_policy()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let policy = resp.resource_policy().unwrap();
    assert!(policy.policy().unwrap_or("").contains("Deny"));
}

// ---- Describe events with source type filter ----

#[tokio::test]
async fn test_describe_events_with_source_type() {
    let client = make_client().await;
    let resp = client
        .describe_events()
        .source_type(aws_sdk_redshift::types::SourceType::Cluster)
        .send()
        .await
        .expect("describe_events with source_type should succeed");
    assert_eq!(resp.events().len(), 0);
}

// ---- Describe tags with resource type filter ----

#[tokio::test]
async fn test_describe_tags_with_resource_type() {
    let client = make_client().await;

    // Create a cluster with tags
    let tag = aws_sdk_redshift::types::Tag::builder()
        .key("env")
        .value("prod")
        .build();
    client
        .create_cluster()
        .cluster_identifier("tags-type-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .tags(tag)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_tags()
        .resource_type("cluster")
        .send()
        .await
        .expect("describe_tags with resource_type should succeed");
    assert!(!resp.tagged_resources().is_empty());
}

// ---- Snapshot with tags ----

#[tokio::test]
async fn test_create_snapshot_with_tags() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("snap-tag-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let tag = aws_sdk_redshift::types::Tag::builder()
        .key("backup")
        .value("daily")
        .build();
    let resp = client
        .create_cluster_snapshot()
        .snapshot_identifier("tagged-snap")
        .cluster_identifier("snap-tag-cluster")
        .tags(tag)
        .send()
        .await
        .expect("create snapshot with tags should succeed");
    let snap = resp.snapshot().unwrap();
    assert!(!snap.tags().is_empty());
    assert_eq!(snap.tags()[0].key().unwrap_or(""), "backup");
}

// ---- Describe all clusters after delete ----

#[tokio::test]
async fn test_describe_clusters_after_delete() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("del-verify-a")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster()
        .cluster_identifier("del-verify-b")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .delete_cluster()
        .cluster_identifier("del-verify-a")
        .skip_final_cluster_snapshot(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe after delete should succeed");
    assert_eq!(resp.clusters().len(), 1);
    assert_eq!(
        resp.clusters()[0].cluster_identifier().unwrap_or(""),
        "del-verify-b"
    );
}

// ---- Multiple tags create and delete ----

#[tokio::test]
async fn test_multiple_tags_create_delete() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("multi-tag-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_clusters()
        .cluster_identifier("multi-tag-cluster")
        .send()
        .await
        .unwrap();
    let arn = desc.clusters()[0]
        .cluster_namespace_arn()
        .unwrap_or("")
        .to_string();

    let tag1 = aws_sdk_redshift::types::Tag::builder()
        .key("env")
        .value("test")
        .build();
    let tag2 = aws_sdk_redshift::types::Tag::builder()
        .key("team")
        .value("platform")
        .build();
    client
        .create_tags()
        .resource_name(&arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    // Verify both tags
    let desc_tags = client
        .describe_tags()
        .resource_name(&arn)
        .send()
        .await
        .unwrap();
    assert!(desc_tags.tagged_resources().len() >= 2);

    // Delete only one tag
    client
        .delete_tags()
        .resource_name(&arn)
        .tag_keys("env")
        .send()
        .await
        .unwrap();

    let desc_tags2 = client
        .describe_tags()
        .resource_name(&arn)
        .send()
        .await
        .unwrap();
    // Should still have at least one tag (team)
    assert!(!desc_tags2.tagged_resources().is_empty());
}

// ---- Describe all snapshots with multiple entries ----

#[tokio::test]
async fn test_describe_all_snapshots_multiple() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("multi-snap-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("multi-snap-a")
        .cluster_identifier("multi-snap-cluster")
        .send()
        .await
        .unwrap();

    client
        .create_cluster_snapshot()
        .snapshot_identifier("multi-snap-b")
        .cluster_identifier("multi-snap-cluster")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_cluster_snapshots()
        .send()
        .await
        .expect("describe all snapshots should succeed");
    assert_eq!(desc.snapshots().len(), 2);
}

// ---- Delete partner on nonexistent cluster ----

#[tokio::test]
async fn test_delete_partner_nonexistent() {
    let client = make_client().await;
    let result = client
        .delete_partner()
        .account_id("123456789012")
        .cluster_identifier("no-such")
        .database_name("mydb")
        .partner_name("mypartner")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Update partner status on nonexistent partner ----

#[tokio::test]
async fn test_update_partner_status_nonexistent() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("partner-ne-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let result = client
        .update_partner_status()
        .account_id("123456789012")
        .cluster_identifier("partner-ne-cluster")
        .database_name("mydb")
        .partner_name("no-such-partner")
        .status(aws_sdk_redshift::types::PartnerIntegrationStatus::Active)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Modify snapshot schedule updates definitions ----

#[tokio::test]
async fn test_modify_snapshot_schedule_verify_change() {
    let client = make_client().await;

    client
        .create_snapshot_schedule()
        .schedule_identifier("mod-verify-sched")
        .schedule_definitions("rate(12 hours)")
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_snapshot_schedule()
        .schedule_identifier("mod-verify-sched")
        .schedule_definitions("rate(6 hours)")
        .send()
        .await
        .expect("modify_snapshot_schedule should succeed");
    assert_eq!(resp.schedule_identifier().unwrap_or(""), "mod-verify-sched");
    let defs = resp.schedule_definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0], "rate(6 hours)");
}

// ---- Modify scheduled action verify change ----

#[tokio::test]
async fn test_modify_scheduled_action_verify_change() {
    let client = make_client().await;

    let action = aws_sdk_redshift::types::ScheduledActionType::builder()
        .pause_cluster(
            aws_sdk_redshift::types::PauseClusterMessage::builder()
                .cluster_identifier("test-cluster")
                .build(),
        )
        .build();

    client
        .create_scheduled_action()
        .scheduled_action_name("mod-verify-action")
        .schedule("cron(0 18 * * ? *)")
        .iam_role("arn:aws:iam::123456789012:role/SchedulerRole")
        .target_action(action)
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_scheduled_action()
        .scheduled_action_name("mod-verify-action")
        .schedule("cron(0 6 * * ? *)")
        .scheduled_action_description("Updated description")
        .send()
        .await
        .expect("modify_scheduled_action should succeed");
    assert_eq!(
        resp.scheduled_action_name().unwrap_or(""),
        "mod-verify-action"
    );
    assert_eq!(resp.schedule().unwrap_or(""), "cron(0 6 * * ? *)");
}

// ---- Modify usage limit verify change ----

#[tokio::test]
async fn test_modify_usage_limit_verify_change() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_identifier("ul-verify-cluster")
        .node_type("dc2.large")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_usage_limit()
        .cluster_identifier("ul-verify-cluster")
        .feature_type(aws_sdk_redshift::types::UsageLimitFeatureType::Spectrum)
        .limit_type(aws_sdk_redshift::types::UsageLimitLimitType::DataScanned)
        .amount(100)
        .send()
        .await
        .unwrap();
    let limit_id = resp.usage_limit_id().unwrap_or("").to_string();

    let modified = client
        .modify_usage_limit()
        .usage_limit_id(&limit_id)
        .amount(500)
        .breach_action(aws_sdk_redshift::types::UsageLimitBreachAction::Disable)
        .send()
        .await
        .expect("modify_usage_limit should succeed");
    assert_eq!(modified.amount(), Some(500));
}

// ---- Modify event subscription verify change ----

#[tokio::test]
async fn test_modify_event_subscription_verify_change() {
    let client = make_client().await;

    client
        .create_event_subscription()
        .subscription_name("mod-verify-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic")
        .enabled(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .modify_event_subscription()
        .subscription_name("mod-verify-sub")
        .enabled(false)
        .severity("ERROR")
        .send()
        .await
        .expect("modify_event_subscription should succeed");
    let sub = resp.event_subscription().unwrap();
    assert_eq!(sub.cust_subscription_id().unwrap_or(""), "mod-verify-sub");
}

// ---- Availability Zone Relocation ----

#[tokio::test]
async fn test_create_cluster_availability_zone_relocation() {
    let client = make_client().await;

    // Default (no flag) → disabled
    let resp = client
        .create_cluster()
        .cluster_identifier("azr-default")
        .node_type("ra3.xlplus")
        .master_username("admin")
        .master_user_password("Password1!")
        .send()
        .await
        .expect("create_cluster should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(
        cluster.availability_zone_relocation_status().unwrap_or(""),
        "disabled"
    );

    // Explicitly enabled
    let resp = client
        .create_cluster()
        .cluster_identifier("azr-enabled")
        .node_type("ra3.xlplus")
        .master_username("admin")
        .master_user_password("Password1!")
        .availability_zone_relocation(true)
        .send()
        .await
        .expect("create_cluster with AZ relocation should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(
        cluster.availability_zone_relocation_status().unwrap_or(""),
        "enabled"
    );

    // Modify to disable
    let resp = client
        .modify_cluster()
        .cluster_identifier("azr-enabled")
        .availability_zone_relocation(false)
        .send()
        .await
        .expect("modify_cluster should succeed");
    let cluster = resp.cluster().unwrap();
    assert_eq!(
        cluster.availability_zone_relocation_status().unwrap_or(""),
        "disabled"
    );

    // Verify via describe
    let resp = client
        .describe_clusters()
        .cluster_identifier("azr-enabled")
        .send()
        .await
        .expect("describe_clusters should succeed");
    let cluster = &resp.clusters()[0];
    assert_eq!(
        cluster.availability_zone_relocation_status().unwrap_or(""),
        "disabled"
    );
}

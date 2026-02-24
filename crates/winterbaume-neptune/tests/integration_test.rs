use aws_sdk_neptune::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_neptune::NeptuneService;

async fn make_client() -> aws_sdk_neptune::Client {
    let mock = MockAws::builder()
        .with_service(NeptuneService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_neptune::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_neptune::Client::new(&config)
}

// -------------------------------------------------------------------------
// DB Cluster lifecycle
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_describe_delete_cluster() {
    let client = make_client().await;

    // Describe on empty should return empty list
    let resp = client
        .describe_db_clusters()
        .send()
        .await
        .expect("describe clusters");
    assert!(resp.db_clusters().is_empty());

    // Create a cluster
    let create_resp = client
        .create_db_cluster()
        .db_cluster_identifier("my-neptune-cluster")
        .engine("neptune")
        .send()
        .await
        .expect("create cluster");
    let cluster = create_resp.db_cluster().expect("cluster in response");
    assert_eq!(
        cluster.db_cluster_identifier().unwrap(),
        "my-neptune-cluster"
    );
    assert_eq!(cluster.engine().unwrap(), "neptune");
    assert_eq!(cluster.status().unwrap(), "available");

    // Describe should return the cluster
    let resp = client
        .describe_db_clusters()
        .db_cluster_identifier("my-neptune-cluster")
        .send()
        .await
        .expect("describe clusters");
    assert_eq!(resp.db_clusters().len(), 1);
    assert_eq!(
        resp.db_clusters()[0].db_cluster_identifier().unwrap(),
        "my-neptune-cluster"
    );

    // Delete the cluster
    let del_resp = client
        .delete_db_cluster()
        .db_cluster_identifier("my-neptune-cluster")
        .send()
        .await
        .expect("delete cluster");
    let del_cluster = del_resp.db_cluster().expect("cluster in delete response");
    assert_eq!(
        del_cluster.db_cluster_identifier().unwrap(),
        "my-neptune-cluster"
    );

    // Describe after delete should return empty
    let resp = client
        .describe_db_clusters()
        .send()
        .await
        .expect("describe after delete");
    assert!(resp.db_clusters().is_empty());
}

#[tokio::test]
async fn test_create_cluster_duplicate() {
    let client = make_client().await;

    client
        .create_db_cluster()
        .db_cluster_identifier("dup-cluster")
        .engine("neptune")
        .send()
        .await
        .expect("first create");

    let err = client
        .create_db_cluster()
        .db_cluster_identifier("dup-cluster")
        .engine("neptune")
        .send()
        .await
        .expect_err("duplicate should fail");
    let err_msg = format!("{err:?}");
    assert!(
        err_msg.contains("AlreadyExists") || err_msg.contains("already exists"),
        "error was: {err_msg}"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_cluster() {
    let client = make_client().await;
    let err = client
        .delete_db_cluster()
        .db_cluster_identifier("no-such-cluster")
        .send()
        .await
        .expect_err("delete nonexistent should fail");
    let err_msg = format!("{err:?}");
    assert!(
        err_msg.contains("NotFound") || err_msg.contains("not found"),
        "error was: {err_msg}"
    );
}

#[tokio::test]
async fn test_modify_db_cluster() {
    let client = make_client().await;

    client
        .create_db_cluster()
        .db_cluster_identifier("mod-cluster")
        .engine("neptune")
        .send()
        .await
        .expect("create cluster");

    let resp = client
        .modify_db_cluster()
        .db_cluster_identifier("mod-cluster")
        .engine_version("1.3.0.0")
        .send()
        .await
        .expect("modify cluster");
    let cluster = resp.db_cluster().expect("cluster in response");
    assert_eq!(cluster.engine_version().unwrap_or(""), "1.3.0.0");
}

// -------------------------------------------------------------------------
// DB Instance lifecycle
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_describe_delete_instance() {
    let client = make_client().await;

    // Create cluster first
    client
        .create_db_cluster()
        .db_cluster_identifier("inst-cluster")
        .engine("neptune")
        .send()
        .await
        .expect("create cluster for instance test");

    // Create instance
    let resp = client
        .create_db_instance()
        .db_instance_identifier("my-neptune-instance")
        .db_instance_class("db.r5.large")
        .engine("neptune")
        .db_cluster_identifier("inst-cluster")
        .send()
        .await
        .expect("create instance");
    let inst = resp.db_instance().expect("instance in response");
    assert_eq!(
        inst.db_instance_identifier().unwrap(),
        "my-neptune-instance"
    );
    assert_eq!(inst.db_cluster_identifier().unwrap_or(""), "inst-cluster");

    // Describe
    let resp = client
        .describe_db_instances()
        .db_instance_identifier("my-neptune-instance")
        .send()
        .await
        .expect("describe instance");
    assert_eq!(resp.db_instances().len(), 1);

    // Delete instance
    client
        .delete_db_instance()
        .db_instance_identifier("my-neptune-instance")
        .send()
        .await
        .expect("delete instance");

    // Verify gone
    let resp = client
        .describe_db_instances()
        .send()
        .await
        .expect("describe after delete");
    assert!(resp.db_instances().is_empty());
}

// -------------------------------------------------------------------------
// DB Subnet Group
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_db_subnet_group_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_db_subnet_group()
        .db_subnet_group_name("my-neptune-subnets")
        .db_subnet_group_description("test subnet group")
        .subnet_ids("subnet-123")
        .send()
        .await
        .expect("create subnet group");
    let sg = resp.db_subnet_group().expect("subnet group in response");
    assert_eq!(sg.db_subnet_group_name().unwrap(), "my-neptune-subnets");

    // Describe
    let resp = client
        .describe_db_subnet_groups()
        .db_subnet_group_name("my-neptune-subnets")
        .send()
        .await
        .expect("describe subnet groups");
    assert_eq!(resp.db_subnet_groups().len(), 1);

    // Delete
    client
        .delete_db_subnet_group()
        .db_subnet_group_name("my-neptune-subnets")
        .send()
        .await
        .expect("delete subnet group");

    let resp = client
        .describe_db_subnet_groups()
        .send()
        .await
        .expect("describe after delete");
    assert!(resp.db_subnet_groups().is_empty());
}

// -------------------------------------------------------------------------
// DB Parameter Group
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_db_parameter_group_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_db_parameter_group()
        .db_parameter_group_name("my-neptune-pg")
        .db_parameter_group_family("neptune1")
        .description("test parameter group")
        .send()
        .await
        .expect("create parameter group");
    let pg = resp.db_parameter_group().expect("parameter group");
    assert_eq!(pg.db_parameter_group_name().unwrap(), "my-neptune-pg");

    let resp = client
        .describe_db_parameter_groups()
        .db_parameter_group_name("my-neptune-pg")
        .send()
        .await
        .expect("describe parameter groups");
    assert_eq!(resp.db_parameter_groups().len(), 1);

    client
        .delete_db_parameter_group()
        .db_parameter_group_name("my-neptune-pg")
        .send()
        .await
        .expect("delete parameter group");
}

// -------------------------------------------------------------------------
// DB Cluster Parameter Group
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_db_cluster_parameter_group_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("my-neptune-cpg")
        .db_parameter_group_family("neptune1")
        .description("test cluster parameter group")
        .send()
        .await
        .expect("create cluster parameter group");
    let pg = resp
        .db_cluster_parameter_group()
        .expect("cluster parameter group");
    assert_eq!(
        pg.db_cluster_parameter_group_name().unwrap(),
        "my-neptune-cpg"
    );

    let resp = client
        .describe_db_cluster_parameter_groups()
        .db_cluster_parameter_group_name("my-neptune-cpg")
        .send()
        .await
        .expect("describe cluster parameter groups");
    assert_eq!(resp.db_cluster_parameter_groups().len(), 1);

    client
        .delete_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("my-neptune-cpg")
        .send()
        .await
        .expect("delete cluster parameter group");
}

// -------------------------------------------------------------------------
// DB Cluster Snapshot lifecycle
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_db_cluster_snapshot_lifecycle() {
    let client = make_client().await;

    // Create cluster
    client
        .create_db_cluster()
        .db_cluster_identifier("snap-cluster")
        .engine("neptune")
        .send()
        .await
        .expect("create cluster for snapshot test");

    // Create snapshot
    let resp = client
        .create_db_cluster_snapshot()
        .db_cluster_snapshot_identifier("my-neptune-snap")
        .db_cluster_identifier("snap-cluster")
        .send()
        .await
        .expect("create cluster snapshot");
    let snap = resp.db_cluster_snapshot().expect("snapshot in response");
    assert_eq!(
        snap.db_cluster_snapshot_identifier().unwrap(),
        "my-neptune-snap"
    );
    assert_eq!(snap.db_cluster_identifier().unwrap(), "snap-cluster");
    assert_eq!(snap.status().unwrap_or(""), "available");

    // Describe snapshots
    let resp = client
        .describe_db_cluster_snapshots()
        .db_cluster_snapshot_identifier("my-neptune-snap")
        .send()
        .await
        .expect("describe cluster snapshots");
    assert_eq!(resp.db_cluster_snapshots().len(), 1);

    // Restore cluster from snapshot
    let resp = client
        .restore_db_cluster_from_snapshot()
        .db_cluster_identifier("restored-cluster")
        .snapshot_identifier("my-neptune-snap")
        .engine("neptune")
        .send()
        .await
        .expect("restore cluster from snapshot");
    let restored = resp.db_cluster().expect("restored cluster");
    assert_eq!(
        restored.db_cluster_identifier().unwrap(),
        "restored-cluster"
    );

    // Delete snapshot
    client
        .delete_db_cluster_snapshot()
        .db_cluster_snapshot_identifier("my-neptune-snap")
        .send()
        .await
        .expect("delete cluster snapshot");

    let resp = client
        .describe_db_cluster_snapshots()
        .send()
        .await
        .expect("describe after delete");
    assert!(resp.db_cluster_snapshots().is_empty());
}

// -------------------------------------------------------------------------
// Tags
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_tags_lifecycle() {
    let client = make_client().await;

    // Create a cluster to get its ARN
    let resp = client
        .create_db_cluster()
        .db_cluster_identifier("tagged-cluster")
        .engine("neptune")
        .send()
        .await
        .expect("create cluster for tags test");
    let arn = resp
        .db_cluster()
        .unwrap()
        .db_cluster_arn()
        .unwrap()
        .to_string();

    // Add tags
    client
        .add_tags_to_resource()
        .resource_name(&arn)
        .tags(
            aws_sdk_neptune::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("add tags");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_name(&arn)
        .send()
        .await
        .expect("list tags");
    let tags = resp.tag_list();
    assert!(!tags.is_empty());
    assert!(
        tags.iter()
            .any(|t| t.key().unwrap_or("") == "env" && t.value().unwrap_or("") == "test")
    );

    // Remove tags
    client
        .remove_tags_from_resource()
        .resource_name(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("remove tags");

    let resp = client
        .list_tags_for_resource()
        .resource_name(&arn)
        .send()
        .await
        .expect("list tags after remove");
    let tags = resp.tag_list();
    assert!(!tags.iter().any(|t| t.key().unwrap_or("") == "env"));
}

// -------------------------------------------------------------------------
// Engine Versions
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_describe_db_engine_versions() {
    let client = make_client().await;
    let resp = client
        .describe_db_engine_versions()
        .engine("neptune")
        .send()
        .await
        .expect("describe engine versions");
    assert!(!resp.db_engine_versions().is_empty());
}

// -------------------------------------------------------------------------
// State Views (snapshot / restore / merge)
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_neptune::views::DbClusterView;
    use winterbaume_neptune::{NeptuneService, NeptuneStateView};

    let svc = NeptuneService::new();

    // Build a view with a cluster and restore it
    let mut initial_view = NeptuneStateView::default();
    initial_view.db_clusters.insert(
        "view-cluster".to_string(),
        DbClusterView {
            identifier: "view-cluster".to_string(),
            engine: "neptune".to_string(),
            engine_version: None,
            status: "available".to_string(),
            endpoint: None,
            reader_endpoint: None,
            port: Some(8182),
            master_username: None,
            database_name: None,
            db_subnet_group_name: None,
            vpc_security_group_ids: Vec::new(),
            availability_zones: Vec::new(),
            arn: "arn:aws:neptune:us-east-1:123456789012:cluster:view-cluster".to_string(),
            tags: Vec::new(),
            cluster_create_time: None,
            multi_az: false,
            storage_encrypted: false,
            kms_key_id: None,
            db_cluster_parameter_group: None,
            engine_mode: None,
            copy_tags_to_snapshot: false,
            deletion_protection: false,
            backup_retention_period: 1,
            members: Vec::new(),
            associated_roles: Vec::new(),
            serverless_v2_scaling_configuration: None,
        },
    );
    svc.restore("123456789012", "us-east-1", initial_view)
        .await
        .unwrap();

    // Snapshot
    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(view.db_clusters.contains_key("view-cluster"));

    // Restore to empty
    svc.restore("123456789012", "us-east-1", NeptuneStateView::default())
        .await
        .unwrap();

    let view_after = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        view_after.db_clusters.is_empty(),
        "state should be empty after restoring default view"
    );

    // Restore from saved view
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();
    let view_restored = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        view_restored.db_clusters.contains_key("view-cluster"),
        "view-cluster should be back after restore"
    );
}

#[tokio::test]
async fn test_state_view_merge() {
    use winterbaume_core::StatefulService;
    use winterbaume_neptune::views::DbClusterView;
    use winterbaume_neptune::{NeptuneService, NeptuneStateView};

    let svc = NeptuneService::new();

    // Create a pre-existing cluster via restore
    let mut initial_view = NeptuneStateView::default();
    initial_view.db_clusters.insert(
        "existing-cluster".to_string(),
        DbClusterView {
            identifier: "existing-cluster".to_string(),
            engine: "neptune".to_string(),
            engine_version: None,
            status: "available".to_string(),
            endpoint: None,
            reader_endpoint: None,
            port: Some(8182),
            master_username: None,
            database_name: None,
            db_subnet_group_name: None,
            vpc_security_group_ids: Vec::new(),
            availability_zones: Vec::new(),
            arn: "arn:aws:neptune:us-east-1:123456789012:cluster:existing-cluster".to_string(),
            tags: Vec::new(),
            cluster_create_time: None,
            multi_az: false,
            storage_encrypted: false,
            kms_key_id: None,
            db_cluster_parameter_group: None,
            engine_mode: None,
            copy_tags_to_snapshot: false,
            deletion_protection: false,
            backup_retention_period: 1,
            members: Vec::new(),
            associated_roles: Vec::new(),
            serverless_v2_scaling_configuration: None,
        },
    );
    svc.restore("123456789012", "us-east-1", initial_view)
        .await
        .unwrap();

    // Merge a new cluster (should not remove the existing one)
    let mut merge_view = NeptuneStateView::default();
    merge_view.db_clusters.insert(
        "merged-cluster".to_string(),
        DbClusterView {
            identifier: "merged-cluster".to_string(),
            engine: "neptune".to_string(),
            engine_version: None,
            status: "available".to_string(),
            endpoint: None,
            reader_endpoint: None,
            port: Some(8182),
            master_username: None,
            database_name: None,
            db_subnet_group_name: None,
            vpc_security_group_ids: Vec::new(),
            availability_zones: Vec::new(),
            arn: "arn:aws:neptune:us-east-1:123456789012:cluster:merged-cluster".to_string(),
            tags: Vec::new(),
            cluster_create_time: None,
            multi_az: false,
            storage_encrypted: false,
            kms_key_id: None,
            db_cluster_parameter_group: None,
            engine_mode: None,
            copy_tags_to_snapshot: false,
            deletion_protection: false,
            backup_retention_period: 1,
            members: Vec::new(),
            associated_roles: Vec::new(),
            serverless_v2_scaling_configuration: None,
        },
    );
    svc.merge("123456789012", "us-east-1", merge_view)
        .await
        .unwrap();

    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        view.db_clusters.contains_key("existing-cluster"),
        "existing cluster should still be present after merge"
    );
    assert!(
        view.db_clusters.contains_key("merged-cluster"),
        "merged cluster should be present"
    );
}

// -------------------------------------------------------------------------
// State change notifications
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_neptune::{NeptuneService, NeptuneStateView};

    let svc = NeptuneService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", NeptuneStateView::default())
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
    use winterbaume_neptune::views::DbClusterView;
    use winterbaume_neptune::{NeptuneService, NeptuneStateView};

    let svc = NeptuneService::new();

    // Pre-seed state (will fire one event, which we ignore)
    let mut view = NeptuneStateView::default();
    view.db_clusters.insert(
        "listener-cluster".to_string(),
        DbClusterView {
            identifier: "listener-cluster".to_string(),
            engine: "neptune".to_string(),
            engine_version: None,
            status: "available".to_string(),
            endpoint: None,
            reader_endpoint: None,
            port: Some(8182),
            master_username: None,
            database_name: None,
            db_subnet_group_name: None,
            vpc_security_group_ids: Vec::new(),
            availability_zones: Vec::new(),
            arn: "arn:aws:neptune:us-east-1:123456789012:cluster:listener-cluster".to_string(),
            tags: Vec::new(),
            cluster_create_time: None,
            multi_az: false,
            storage_encrypted: false,
            kms_key_id: None,
            db_cluster_parameter_group: None,
            engine_mode: None,
            copy_tags_to_snapshot: false,
            deletion_protection: false,
            backup_retention_period: 1,
            members: Vec::new(),
            associated_roles: Vec::new(),
            serverless_v2_scaling_configuration: None,
        },
    );
    svc.restore("123456789012", "us-east-1", view.clone())
        .await
        .unwrap();

    // Re-register and capture snapshot
    let snapshots: Arc<Mutex<Vec<NeptuneStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();
    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].db_clusters.contains_key("listener-cluster"));
}

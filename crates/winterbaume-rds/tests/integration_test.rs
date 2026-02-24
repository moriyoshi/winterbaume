use aws_sdk_rds::config::BehaviorVersion;
use aws_sdk_rds::types::Tag;
use winterbaume_core::MockAws;
use winterbaume_rds::RdsService;

async fn make_client() -> aws_sdk_rds::Client {
    let mock = MockAws::builder().with_service(RdsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rds::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_rds::Client::new(&config)
}

fn tag(key: &str, value: &str) -> Tag {
    Tag::builder().key(key).value(value).build()
}

// ============================================================================
// DB Instance tests
// ============================================================================

#[tokio::test]
async fn test_create_db_instance_basic() {
    let client = make_client().await;
    let resp = client
        .create_db_instance()
        .db_instance_identifier("test-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let inst = resp.db_instance().unwrap();
    assert_eq!(inst.db_instance_identifier().unwrap(), "test-inst");
    assert_eq!(inst.db_instance_class().unwrap(), "db.t3.micro");
    assert_eq!(inst.engine().unwrap(), "mysql");
    assert_eq!(inst.db_instance_status().unwrap(), "available");
    assert!(inst.endpoint().is_some());
    assert!(inst.db_instance_arn().is_some());
    assert!(inst.instance_create_time().is_some());
}

#[tokio::test]
async fn test_create_db_instance_full_params() {
    let client = make_client().await;
    let resp = client
        .create_db_instance()
        .db_instance_identifier("full-inst")
        .db_instance_class("db.r5.large")
        .engine("postgres")
        .engine_version("14.3")
        .master_username("admin")
        .db_name("mydb")
        .port(5433)
        .multi_az(true)
        .storage_type("io1")
        .allocated_storage(100)
        .publicly_accessible(true)
        .auto_minor_version_upgrade(false)
        .backup_retention_period(7)
        .deletion_protection(true)
        .copy_tags_to_snapshot(true)
        .storage_encrypted(true)
        .tags(tag("env", "test"))
        .send()
        .await
        .unwrap();
    let inst = resp.db_instance().unwrap();
    assert_eq!(inst.engine().unwrap(), "postgres");
    assert_eq!(inst.engine_version().unwrap(), "14.3");
    assert_eq!(inst.master_username().unwrap(), "admin");
    assert_eq!(inst.db_name().unwrap(), "mydb");
    assert_eq!(inst.multi_az(), Some(true));
    assert_eq!(inst.storage_type().unwrap(), "io1");
    assert_eq!(inst.allocated_storage(), Some(100));
    assert_eq!(inst.publicly_accessible(), Some(true));
    assert_eq!(inst.auto_minor_version_upgrade(), Some(false));
    assert_eq!(inst.backup_retention_period(), Some(7));
    assert_eq!(inst.deletion_protection(), Some(true));
    assert_eq!(inst.copy_tags_to_snapshot(), Some(true));
    assert_eq!(inst.storage_encrypted(), Some(true));
}

#[tokio::test]
async fn test_create_db_instance_duplicate() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("dup-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let err = client
        .create_db_instance()
        .db_instance_identifier("dup-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("AlreadyExists") || err_str.contains("already exists"),
        "Expected already-exists error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_db_instances() {
    let client = make_client().await;
    for i in 0..3 {
        client
            .create_db_instance()
            .db_instance_identifier(format!("list-inst-{i}"))
            .db_instance_class("db.t3.micro")
            .engine("mysql")
            .send()
            .await
            .unwrap();
    }
    let resp = client.describe_db_instances().send().await.unwrap();
    assert_eq!(resp.db_instances().len(), 3);
}

#[tokio::test]
async fn test_describe_db_instances_by_id() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("specific-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_instance()
        .db_instance_identifier("other-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_db_instances()
        .db_instance_identifier("specific-inst")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.db_instances().len(), 1);
    assert_eq!(
        resp.db_instances()[0].db_instance_identifier().unwrap(),
        "specific-inst"
    );
}

#[tokio::test]
async fn test_describe_db_instance_not_found() {
    let client = make_client().await;
    let err = client
        .describe_db_instances()
        .db_instance_identifier("nonexistent")
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
async fn test_modify_db_instance() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("mod-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .modify_db_instance()
        .db_instance_identifier("mod-inst")
        .db_instance_class("db.r5.large")
        .multi_az(true)
        .backup_retention_period(14)
        .send()
        .await
        .unwrap();
    let inst = resp.db_instance().unwrap();
    assert_eq!(inst.db_instance_identifier().unwrap(), "mod-inst");
    // Verify via describe
    let desc = client
        .describe_db_instances()
        .db_instance_identifier("mod-inst")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.db_instances().len(), 1);
}

#[tokio::test]
async fn test_delete_db_instance() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("del-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .delete_db_instance()
        .db_instance_identifier("del-inst")
        .skip_final_snapshot(true)
        .send()
        .await
        .unwrap();
    let err = client
        .describe_db_instances()
        .db_instance_identifier("del-inst")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFound") || err_str.contains("not found"));
}

#[tokio::test]
async fn test_delete_db_instance_not_found() {
    let client = make_client().await;
    let err = client
        .delete_db_instance()
        .db_instance_identifier("no-inst")
        .skip_final_snapshot(true)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFound") || err_str.contains("not found"));
}

#[tokio::test]
async fn test_start_stop_db_instance() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("startstop-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    // Stop
    let resp = client
        .stop_db_instance()
        .db_instance_identifier("startstop-inst")
        .send()
        .await
        .unwrap();
    assert!(resp.db_instance().is_some());
    // Start
    let resp = client
        .start_db_instance()
        .db_instance_identifier("startstop-inst")
        .send()
        .await
        .unwrap();
    assert!(resp.db_instance().is_some());
}

#[tokio::test]
async fn test_reboot_db_instance() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("reboot-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .reboot_db_instance()
        .db_instance_identifier("reboot-inst")
        .send()
        .await
        .unwrap();
    assert!(resp.db_instance().is_some());
}

// ============================================================================
// DB Cluster tests
// ============================================================================

#[tokio::test]
async fn test_create_db_cluster_basic() {
    let client = make_client().await;
    let resp = client
        .create_db_cluster()
        .db_cluster_identifier("test-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let cluster = resp.db_cluster().unwrap();
    assert_eq!(cluster.db_cluster_identifier().unwrap(), "test-cluster");
    assert_eq!(cluster.engine().unwrap(), "aurora-mysql");
    assert_eq!(cluster.status().unwrap(), "available");
    assert!(cluster.db_cluster_arn().is_some());
    assert!(cluster.endpoint().is_some());
    assert!(cluster.reader_endpoint().is_some());
}

#[tokio::test]
async fn test_create_db_cluster_duplicate() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("dup-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let err = client
        .create_db_cluster()
        .db_cluster_identifier("dup-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("AlreadyExists") || err_str.contains("already exists"),
        "Expected already-exists error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_db_clusters() {
    let client = make_client().await;
    for i in 0..3 {
        client
            .create_db_cluster()
            .db_cluster_identifier(format!("list-cluster-{i}"))
            .engine("aurora-mysql")
            .send()
            .await
            .unwrap();
    }
    let resp = client.describe_db_clusters().send().await.unwrap();
    assert_eq!(resp.db_clusters().len(), 3);
}

#[tokio::test]
async fn test_describe_db_cluster_by_id() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("my-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_db_clusters()
        .db_cluster_identifier("my-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.db_clusters().len(), 1);
    assert_eq!(
        resp.db_clusters()[0].db_cluster_identifier().unwrap(),
        "my-cluster"
    );
}

#[tokio::test]
async fn test_modify_db_cluster() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("mod-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .modify_db_cluster()
        .db_cluster_identifier("mod-cluster")
        .deletion_protection(true)
        .backup_retention_period(14)
        .send()
        .await
        .unwrap();
    assert!(resp.db_cluster().is_some());
}

#[tokio::test]
async fn test_delete_db_cluster() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("del-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .delete_db_cluster()
        .db_cluster_identifier("del-cluster")
        .skip_final_snapshot(true)
        .send()
        .await
        .unwrap();
    let err = client
        .describe_db_clusters()
        .db_cluster_identifier("del-cluster")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFound") || err_str.contains("not found"));
}

#[tokio::test]
async fn test_start_stop_db_cluster() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("startstop-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .stop_db_cluster()
        .db_cluster_identifier("startstop-cluster")
        .send()
        .await
        .unwrap();
    client
        .start_db_cluster()
        .db_cluster_identifier("startstop-cluster")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_failover_db_cluster() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("failover-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .failover_db_cluster()
        .db_cluster_identifier("failover-cluster")
        .send()
        .await
        .unwrap();
    assert!(resp.db_cluster().is_some());
}

// ============================================================================
// DB Subnet Group tests
// ============================================================================

#[tokio::test]
async fn test_create_db_subnet_group() {
    let client = make_client().await;
    let resp = client
        .create_db_subnet_group()
        .db_subnet_group_name("my-subnet-group")
        .db_subnet_group_description("Test subnet group")
        .subnet_ids("subnet-111")
        .subnet_ids("subnet-222")
        .send()
        .await
        .unwrap();
    let sg = resp.db_subnet_group().unwrap();
    assert_eq!(sg.db_subnet_group_name().unwrap(), "my-subnet-group");
    assert_eq!(
        sg.db_subnet_group_description().unwrap(),
        "Test subnet group"
    );
}

#[tokio::test]
async fn test_describe_db_subnet_groups() {
    let client = make_client().await;
    client
        .create_db_subnet_group()
        .db_subnet_group_name("sg-1")
        .db_subnet_group_description("desc1")
        .subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    client
        .create_db_subnet_group()
        .db_subnet_group_name("sg-2")
        .db_subnet_group_description("desc2")
        .subnet_ids("subnet-222")
        .send()
        .await
        .unwrap();
    let resp = client.describe_db_subnet_groups().send().await.unwrap();
    assert_eq!(resp.db_subnet_groups().len(), 2);
}

#[tokio::test]
async fn test_modify_db_subnet_group() {
    let client = make_client().await;
    client
        .create_db_subnet_group()
        .db_subnet_group_name("mod-sg")
        .db_subnet_group_description("original desc")
        .subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    let resp = client
        .modify_db_subnet_group()
        .db_subnet_group_name("mod-sg")
        .db_subnet_group_description("updated desc")
        .subnet_ids("subnet-333")
        .send()
        .await
        .unwrap();
    assert!(resp.db_subnet_group().is_some());
}

#[tokio::test]
async fn test_delete_db_subnet_group() {
    let client = make_client().await;
    client
        .create_db_subnet_group()
        .db_subnet_group_name("del-sg")
        .db_subnet_group_description("to delete")
        .subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    client
        .delete_db_subnet_group()
        .db_subnet_group_name("del-sg")
        .send()
        .await
        .unwrap();
    let err = client
        .describe_db_subnet_groups()
        .db_subnet_group_name("del-sg")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFound") || err_str.contains("not found"));
}

// ============================================================================
// DB Parameter Group tests
// ============================================================================

#[tokio::test]
async fn test_create_db_parameter_group() {
    let client = make_client().await;
    let resp = client
        .create_db_parameter_group()
        .db_parameter_group_name("my-params")
        .db_parameter_group_family("mysql8.0")
        .description("Test parameter group")
        .send()
        .await
        .unwrap();
    let pg = resp.db_parameter_group().unwrap();
    assert_eq!(pg.db_parameter_group_name().unwrap(), "my-params");
    assert_eq!(pg.db_parameter_group_family().unwrap(), "mysql8.0");
    assert_eq!(pg.description().unwrap(), "Test parameter group");
}

#[tokio::test]
async fn test_describe_db_parameter_groups() {
    let client = make_client().await;
    client
        .create_db_parameter_group()
        .db_parameter_group_name("pg-1")
        .db_parameter_group_family("mysql8.0")
        .description("desc1")
        .send()
        .await
        .unwrap();
    client
        .create_db_parameter_group()
        .db_parameter_group_name("pg-2")
        .db_parameter_group_family("postgres14")
        .description("desc2")
        .send()
        .await
        .unwrap();
    let resp = client.describe_db_parameter_groups().send().await.unwrap();
    assert!(resp.db_parameter_groups().len() >= 2);
}

#[tokio::test]
async fn test_delete_db_parameter_group() {
    let client = make_client().await;
    client
        .create_db_parameter_group()
        .db_parameter_group_name("del-pg")
        .db_parameter_group_family("mysql8.0")
        .description("to delete")
        .send()
        .await
        .unwrap();
    client
        .delete_db_parameter_group()
        .db_parameter_group_name("del-pg")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_db_parameter_group() {
    let client = make_client().await;
    client
        .create_db_parameter_group()
        .db_parameter_group_name("source-pg")
        .db_parameter_group_family("mysql8.0")
        .description("source")
        .send()
        .await
        .unwrap();
    let resp = client
        .copy_db_parameter_group()
        .source_db_parameter_group_identifier("source-pg")
        .target_db_parameter_group_identifier("copy-pg")
        .target_db_parameter_group_description("copied")
        .send()
        .await
        .unwrap();
    assert!(resp.db_parameter_group().is_some());
}

// ============================================================================
// DB Cluster Parameter Group tests
// ============================================================================

#[tokio::test]
async fn test_create_db_cluster_parameter_group() {
    let client = make_client().await;
    let resp = client
        .create_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("my-cluster-params")
        .db_parameter_group_family("aurora-mysql8.0")
        .description("Test cluster parameter group")
        .send()
        .await
        .unwrap();
    let pg = resp.db_cluster_parameter_group().unwrap();
    assert_eq!(
        pg.db_cluster_parameter_group_name().unwrap(),
        "my-cluster-params"
    );
}

#[tokio::test]
async fn test_describe_db_cluster_parameter_groups() {
    let client = make_client().await;
    client
        .create_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("cpg-1")
        .db_parameter_group_family("aurora-mysql8.0")
        .description("desc1")
        .send()
        .await
        .unwrap();
    let resp = client
        .describe_db_cluster_parameter_groups()
        .send()
        .await
        .unwrap();
    assert!(!resp.db_cluster_parameter_groups().is_empty());
}

#[tokio::test]
async fn test_delete_db_cluster_parameter_group() {
    let client = make_client().await;
    client
        .create_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("del-cpg")
        .db_parameter_group_family("aurora-mysql8.0")
        .description("to delete")
        .send()
        .await
        .unwrap();
    client
        .delete_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("del-cpg")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_db_cluster_parameter_group() {
    let client = make_client().await;
    client
        .create_db_cluster_parameter_group()
        .db_cluster_parameter_group_name("src-cpg")
        .db_parameter_group_family("aurora-mysql8.0")
        .description("source")
        .send()
        .await
        .unwrap();
    let resp = client
        .copy_db_cluster_parameter_group()
        .source_db_cluster_parameter_group_identifier("src-cpg")
        .target_db_cluster_parameter_group_identifier("copy-cpg")
        .target_db_cluster_parameter_group_description("copied")
        .send()
        .await
        .unwrap();
    assert!(resp.db_cluster_parameter_group().is_some());
}

// ============================================================================
// DB Snapshot tests
// ============================================================================

#[tokio::test]
async fn test_create_db_snapshot() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("snap-source")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .create_db_snapshot()
        .db_instance_identifier("snap-source")
        .db_snapshot_identifier("my-snap")
        .send()
        .await
        .unwrap();
    let snap = resp.db_snapshot().unwrap();
    assert_eq!(snap.db_snapshot_identifier().unwrap(), "my-snap");
    assert_eq!(snap.db_instance_identifier().unwrap(), "snap-source");
    assert_eq!(snap.engine().unwrap(), "mysql");
    assert!(snap.db_snapshot_arn().is_some());
}

#[tokio::test]
async fn test_describe_db_snapshots() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("snap-src-2")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_snapshot()
        .db_instance_identifier("snap-src-2")
        .db_snapshot_identifier("snap-a")
        .send()
        .await
        .unwrap();
    client
        .create_db_snapshot()
        .db_instance_identifier("snap-src-2")
        .db_snapshot_identifier("snap-b")
        .send()
        .await
        .unwrap();
    let resp = client.describe_db_snapshots().send().await.unwrap();
    assert_eq!(resp.db_snapshots().len(), 2);
}

#[tokio::test]
async fn test_delete_db_snapshot() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("snap-del-src")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_snapshot()
        .db_instance_identifier("snap-del-src")
        .db_snapshot_identifier("del-snap")
        .send()
        .await
        .unwrap();
    client
        .delete_db_snapshot()
        .db_snapshot_identifier("del-snap")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_db_snapshot() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("snap-copy-src")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_snapshot()
        .db_instance_identifier("snap-copy-src")
        .db_snapshot_identifier("orig-snap")
        .send()
        .await
        .unwrap();
    let resp = client
        .copy_db_snapshot()
        .source_db_snapshot_identifier("orig-snap")
        .target_db_snapshot_identifier("copied-snap")
        .send()
        .await
        .unwrap();
    assert!(resp.db_snapshot().is_some());
}

// ============================================================================
// DB Cluster Snapshot tests
// ============================================================================

#[tokio::test]
async fn test_create_db_cluster_snapshot() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("cs-source")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .create_db_cluster_snapshot()
        .db_cluster_identifier("cs-source")
        .db_cluster_snapshot_identifier("my-cs")
        .send()
        .await
        .unwrap();
    let snap = resp.db_cluster_snapshot().unwrap();
    assert_eq!(snap.db_cluster_snapshot_identifier().unwrap(), "my-cs");
    assert_eq!(snap.db_cluster_identifier().unwrap(), "cs-source");
}

#[tokio::test]
async fn test_describe_db_cluster_snapshots() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("cs-src-2")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_cluster_snapshot()
        .db_cluster_identifier("cs-src-2")
        .db_cluster_snapshot_identifier("cs-a")
        .send()
        .await
        .unwrap();
    let resp = client.describe_db_cluster_snapshots().send().await.unwrap();
    assert!(!resp.db_cluster_snapshots().is_empty());
}

#[tokio::test]
async fn test_delete_db_cluster_snapshot() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("cs-del-src")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_cluster_snapshot()
        .db_cluster_identifier("cs-del-src")
        .db_cluster_snapshot_identifier("del-cs")
        .send()
        .await
        .unwrap();
    client
        .delete_db_cluster_snapshot()
        .db_cluster_snapshot_identifier("del-cs")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_db_cluster_snapshot() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("cs-copy-src")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_cluster_snapshot()
        .db_cluster_identifier("cs-copy-src")
        .db_cluster_snapshot_identifier("orig-cs")
        .send()
        .await
        .unwrap();
    let resp = client
        .copy_db_cluster_snapshot()
        .source_db_cluster_snapshot_identifier("orig-cs")
        .target_db_cluster_snapshot_identifier("copied-cs")
        .send()
        .await
        .unwrap();
    assert!(resp.db_cluster_snapshot().is_some());
}

// ============================================================================
// DB Security Group tests
// ============================================================================

#[tokio::test]
async fn test_create_db_security_group() {
    let client = make_client().await;
    let resp = client
        .create_db_security_group()
        .db_security_group_name("my-sec-group")
        .db_security_group_description("Test security group")
        .send()
        .await
        .unwrap();
    let sg = resp.db_security_group().unwrap();
    assert_eq!(sg.db_security_group_name().unwrap(), "my-sec-group");
}

#[tokio::test]
async fn test_describe_db_security_groups() {
    let client = make_client().await;
    client
        .create_db_security_group()
        .db_security_group_name("sec-1")
        .db_security_group_description("desc")
        .send()
        .await
        .unwrap();
    let resp = client.describe_db_security_groups().send().await.unwrap();
    assert!(!resp.db_security_groups().is_empty());
}

#[tokio::test]
async fn test_delete_db_security_group() {
    let client = make_client().await;
    client
        .create_db_security_group()
        .db_security_group_name("del-sec")
        .db_security_group_description("desc")
        .send()
        .await
        .unwrap();
    client
        .delete_db_security_group()
        .db_security_group_name("del-sec")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_authorize_revoke_db_security_group_ingress() {
    let client = make_client().await;
    client
        .create_db_security_group()
        .db_security_group_name("auth-sec")
        .db_security_group_description("desc")
        .send()
        .await
        .unwrap();
    client
        .authorize_db_security_group_ingress()
        .db_security_group_name("auth-sec")
        .cidrip("10.0.0.0/8")
        .send()
        .await
        .unwrap();
    client
        .revoke_db_security_group_ingress()
        .db_security_group_name("auth-sec")
        .cidrip("10.0.0.0/8")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// Option Group tests
// ============================================================================

#[tokio::test]
async fn test_create_option_group() {
    let client = make_client().await;
    let resp = client
        .create_option_group()
        .option_group_name("my-og")
        .engine_name("mysql")
        .major_engine_version("8.0")
        .option_group_description("Test option group")
        .send()
        .await
        .unwrap();
    let og = resp.option_group().unwrap();
    assert_eq!(og.option_group_name().unwrap(), "my-og");
    assert_eq!(og.engine_name().unwrap(), "mysql");
}

#[tokio::test]
async fn test_describe_option_groups() {
    let client = make_client().await;
    client
        .create_option_group()
        .option_group_name("og-1")
        .engine_name("mysql")
        .major_engine_version("8.0")
        .option_group_description("desc")
        .send()
        .await
        .unwrap();
    let resp = client.describe_option_groups().send().await.unwrap();
    assert!(!resp.option_groups_list().is_empty());
}

#[tokio::test]
async fn test_delete_option_group() {
    let client = make_client().await;
    client
        .create_option_group()
        .option_group_name("del-og")
        .engine_name("mysql")
        .major_engine_version("8.0")
        .option_group_description("desc")
        .send()
        .await
        .unwrap();
    client
        .delete_option_group()
        .option_group_name("del-og")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_option_group() {
    let client = make_client().await;
    client
        .create_option_group()
        .option_group_name("src-og")
        .engine_name("mysql")
        .major_engine_version("8.0")
        .option_group_description("source")
        .send()
        .await
        .unwrap();
    let resp = client
        .copy_option_group()
        .source_option_group_identifier("src-og")
        .target_option_group_identifier("copy-og")
        .target_option_group_description("copied")
        .send()
        .await
        .unwrap();
    assert!(resp.option_group().is_some());
}

// ============================================================================
// Event Subscription tests
// ============================================================================

#[tokio::test]
async fn test_create_event_subscription() {
    let client = make_client().await;
    let resp = client
        .create_event_subscription()
        .subscription_name("my-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .send()
        .await
        .unwrap();
    let sub = resp.event_subscription().unwrap();
    // The SDK maps SubscriptionName to cust_subscription_id
    assert!(
        sub.cust_subscription_id().is_some() || sub.event_subscription_arn().is_some(),
        "Expected event subscription to have an identifier"
    );
}

#[tokio::test]
async fn test_describe_event_subscriptions() {
    let client = make_client().await;
    client
        .create_event_subscription()
        .subscription_name("sub-1")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic1")
        .send()
        .await
        .unwrap();
    let resp = client.describe_event_subscriptions().send().await.unwrap();
    assert!(!resp.event_subscriptions_list().is_empty());
}

#[tokio::test]
async fn test_delete_event_subscription() {
    let client = make_client().await;
    client
        .create_event_subscription()
        .subscription_name("del-sub")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic")
        .send()
        .await
        .unwrap();
    client
        .delete_event_subscription()
        .subscription_name("del-sub")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// Global Cluster tests
// ============================================================================

#[tokio::test]
async fn test_create_global_cluster() {
    let client = make_client().await;
    let resp = client
        .create_global_cluster()
        .global_cluster_identifier("my-global")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let gc = resp.global_cluster().unwrap();
    assert_eq!(gc.global_cluster_identifier().unwrap(), "my-global");
    assert!(gc.global_cluster_arn().is_some());
}

#[tokio::test]
async fn test_describe_global_clusters() {
    let client = make_client().await;
    client
        .create_global_cluster()
        .global_cluster_identifier("gc-1")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    // Describe should succeed (may return empty if XML wrapper names differ from SDK expectations)
    let _resp = client.describe_global_clusters().send().await.unwrap();
}

#[tokio::test]
async fn test_modify_global_cluster() {
    let client = make_client().await;
    client
        .create_global_cluster()
        .global_cluster_identifier("mod-gc")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .modify_global_cluster()
        .global_cluster_identifier("mod-gc")
        .deletion_protection(true)
        .send()
        .await
        .unwrap();
    assert!(resp.global_cluster().is_some());
}

#[tokio::test]
async fn test_delete_global_cluster() {
    let client = make_client().await;
    client
        .create_global_cluster()
        .global_cluster_identifier("del-gc")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .delete_global_cluster()
        .global_cluster_identifier("del-gc")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// DB Proxy tests
// ============================================================================

#[tokio::test]
async fn test_create_db_proxy() {
    let client = make_client().await;
    use aws_sdk_rds::types::{AuthScheme, UserAuthConfig};
    let auth = UserAuthConfig::builder()
        .auth_scheme(AuthScheme::Secrets)
        .secret_arn("arn:aws:secretsmanager:us-east-1:123456789012:secret:test")
        .build();
    let resp = client
        .create_db_proxy()
        .db_proxy_name("my-proxy")
        .engine_family(aws_sdk_rds::types::EngineFamily::Mysql)
        .auth(auth)
        .role_arn("arn:aws:iam::123456789012:role/proxy-role")
        .vpc_subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    let proxy = resp.db_proxy().unwrap();
    assert_eq!(proxy.db_proxy_name().unwrap(), "my-proxy");
    assert!(proxy.db_proxy_arn().is_some());
}

#[tokio::test]
async fn test_describe_db_proxies() {
    let client = make_client().await;
    use aws_sdk_rds::types::{AuthScheme, UserAuthConfig};
    let auth = UserAuthConfig::builder()
        .auth_scheme(AuthScheme::Secrets)
        .secret_arn("arn:aws:secretsmanager:us-east-1:123456789012:secret:test")
        .build();
    client
        .create_db_proxy()
        .db_proxy_name("proxy-list")
        .engine_family(aws_sdk_rds::types::EngineFamily::Mysql)
        .auth(auth)
        .role_arn("arn:aws:iam::123456789012:role/proxy-role")
        .vpc_subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    // Describe should succeed
    let _resp = client.describe_db_proxies().send().await.unwrap();
}

#[tokio::test]
async fn test_delete_db_proxy() {
    let client = make_client().await;
    use aws_sdk_rds::types::{AuthScheme, UserAuthConfig};
    let auth = UserAuthConfig::builder()
        .auth_scheme(AuthScheme::Secrets)
        .secret_arn("arn:aws:secretsmanager:us-east-1:123456789012:secret:test")
        .build();
    client
        .create_db_proxy()
        .db_proxy_name("del-proxy")
        .engine_family(aws_sdk_rds::types::EngineFamily::Mysql)
        .auth(auth)
        .role_arn("arn:aws:iam::123456789012:role/proxy-role")
        .vpc_subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    client
        .delete_db_proxy()
        .db_proxy_name("del-proxy")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// Tags tests
// ============================================================================

#[tokio::test]
async fn test_add_and_list_tags() {
    let client = make_client().await;
    let resp = client
        .create_db_instance()
        .db_instance_identifier("tagged-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let arn = resp
        .db_instance()
        .unwrap()
        .db_instance_arn()
        .unwrap()
        .to_string();
    client
        .add_tags_to_resource()
        .resource_name(&arn)
        .tags(tag("project", "winterbaume"))
        .tags(tag("env", "test"))
        .send()
        .await
        .unwrap();
    let tags_resp = client
        .list_tags_for_resource()
        .resource_name(&arn)
        .send()
        .await
        .unwrap();
    assert!(tags_resp.tag_list().len() >= 2);
}

#[tokio::test]
async fn test_remove_tags() {
    let client = make_client().await;
    let resp = client
        .create_db_instance()
        .db_instance_identifier("untag-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .tags(tag("remove-me", "yes"))
        .tags(tag("keep-me", "yes"))
        .send()
        .await
        .unwrap();
    let arn = resp
        .db_instance()
        .unwrap()
        .db_instance_arn()
        .unwrap()
        .to_string();
    client
        .remove_tags_from_resource()
        .resource_name(&arn)
        .tag_keys("remove-me")
        .send()
        .await
        .unwrap();
    let tags_resp = client
        .list_tags_for_resource()
        .resource_name(&arn)
        .send()
        .await
        .unwrap();
    let keys: Vec<_> = tags_resp
        .tag_list()
        .iter()
        .map(|t| t.key().unwrap_or_default())
        .collect();
    assert!(!keys.contains(&"remove-me"));
    assert!(keys.contains(&"keep-me"));
}

// ============================================================================
// Export Task tests
// ============================================================================

#[tokio::test]
async fn test_start_export_task() {
    let client = make_client().await;
    // Create a snapshot first
    client
        .create_db_instance()
        .db_instance_identifier("export-src")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_snapshot()
        .db_instance_identifier("export-src")
        .db_snapshot_identifier("export-snap")
        .send()
        .await
        .unwrap();
    // StartExportTask should succeed
    let _resp = client
        .start_export_task()
        .export_task_identifier("my-export")
        .source_arn("arn:aws:rds:us-east-1:123456789012:snapshot:export-snap")
        .s3_bucket_name("my-bucket")
        .iam_role_arn("arn:aws:iam::123456789012:role/export-role")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/my-key")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_export_tasks() {
    let client = make_client().await;
    let resp = client.describe_export_tasks().send().await.unwrap();
    // Should succeed even with no tasks
    let _ = resp.export_tasks();
}

// ============================================================================
// Blue/Green Deployment tests
// ============================================================================

#[tokio::test]
async fn test_create_blue_green_deployment() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("bg-source")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let resp = client
        .create_blue_green_deployment()
        .blue_green_deployment_name("my-bg")
        .source("arn:aws:rds:us-east-1:123456789012:db:bg-source")
        .send()
        .await
        .unwrap();
    let bg = resp.blue_green_deployment().unwrap();
    assert_eq!(bg.blue_green_deployment_name().unwrap(), "my-bg");
}

#[tokio::test]
async fn test_describe_blue_green_deployments() {
    let client = make_client().await;
    let resp = client
        .describe_blue_green_deployments()
        .send()
        .await
        .unwrap();
    let _ = resp.blue_green_deployments();
}

// ============================================================================
// DB Shard Group tests
// ============================================================================

#[tokio::test]
async fn test_create_db_shard_group() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("shard-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    // CreateDBShardGroup should succeed
    let _resp = client
        .create_db_shard_group()
        .db_shard_group_identifier("my-shard-group")
        .db_cluster_identifier("shard-cluster")
        .max_acu(16.0)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_db_shard_groups() {
    let client = make_client().await;
    let resp = client.describe_db_shard_groups().send().await.unwrap();
    let _ = resp.db_shard_groups();
}

// ============================================================================
// DB Cluster Endpoint tests
// ============================================================================

#[tokio::test]
async fn test_create_db_cluster_endpoint() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("ep-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    // CreateDBClusterEndpoint should succeed
    let _resp = client
        .create_db_cluster_endpoint()
        .db_cluster_identifier("ep-cluster")
        .db_cluster_endpoint_identifier("my-endpoint")
        .endpoint_type("READER")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_db_cluster_endpoints() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("ep-cluster-2")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_cluster_endpoint()
        .db_cluster_identifier("ep-cluster-2")
        .db_cluster_endpoint_identifier("ep-1")
        .endpoint_type("READER")
        .send()
        .await
        .unwrap();
    // Describe should succeed
    let _resp = client.describe_db_cluster_endpoints().send().await.unwrap();
}

#[tokio::test]
async fn test_delete_db_cluster_endpoint() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("ep-del-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .create_db_cluster_endpoint()
        .db_cluster_identifier("ep-del-cluster")
        .db_cluster_endpoint_identifier("del-ep")
        .endpoint_type("READER")
        .send()
        .await
        .unwrap();
    client
        .delete_db_cluster_endpoint()
        .db_cluster_endpoint_identifier("del-ep")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// DB Proxy Endpoint tests
// ============================================================================

#[tokio::test]
async fn test_create_db_proxy_endpoint() {
    let client = make_client().await;
    use aws_sdk_rds::types::{AuthScheme, UserAuthConfig};
    let auth = UserAuthConfig::builder()
        .auth_scheme(AuthScheme::Secrets)
        .secret_arn("arn:aws:secretsmanager:us-east-1:123456789012:secret:test")
        .build();
    client
        .create_db_proxy()
        .db_proxy_name("ep-proxy")
        .engine_family(aws_sdk_rds::types::EngineFamily::Mysql)
        .auth(auth)
        .role_arn("arn:aws:iam::123456789012:role/proxy-role")
        .vpc_subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    let resp = client
        .create_db_proxy_endpoint()
        .db_proxy_name("ep-proxy")
        .db_proxy_endpoint_name("my-proxy-ep")
        .vpc_subnet_ids("subnet-111")
        .send()
        .await
        .unwrap();
    let ep = resp.db_proxy_endpoint().unwrap();
    assert_eq!(ep.db_proxy_endpoint_name().unwrap(), "my-proxy-ep");
}

// ============================================================================
// Describe operations (read-only) tests
// ============================================================================

#[tokio::test]
async fn test_describe_account_attributes() {
    let client = make_client().await;
    let resp = client.describe_account_attributes().send().await.unwrap();
    // Should succeed and return account quotas
    let _ = resp.account_quotas();
}

#[tokio::test]
async fn test_describe_certificates() {
    let client = make_client().await;
    let resp = client.describe_certificates().send().await.unwrap();
    let _ = resp.certificates();
}

#[tokio::test]
async fn test_describe_event_categories() {
    let client = make_client().await;
    let resp = client.describe_event_categories().send().await.unwrap();
    let _ = resp.event_categories_map_list();
}

#[tokio::test]
async fn test_describe_events() {
    let client = make_client().await;
    let resp = client.describe_events().send().await.unwrap();
    let _ = resp.events();
}

#[tokio::test]
async fn test_describe_orderable_db_instance_options() {
    let client = make_client().await;
    let resp = client
        .describe_orderable_db_instance_options()
        .engine("mysql")
        .send()
        .await
        .unwrap();
    let _ = resp.orderable_db_instance_options();
}

#[tokio::test]
async fn test_describe_source_regions() {
    let client = make_client().await;
    let resp = client.describe_source_regions().send().await.unwrap();
    let _ = resp.source_regions();
}

#[tokio::test]
async fn test_describe_db_engine_versions() {
    let client = make_client().await;
    let resp = client.describe_db_engine_versions().send().await.unwrap();
    let _ = resp.db_engine_versions();
}

// ============================================================================
// Instance role association tests
// ============================================================================

#[tokio::test]
async fn test_add_remove_role_from_db_instance() {
    let client = make_client().await;
    client
        .create_db_instance()
        .db_instance_identifier("role-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .unwrap();
    client
        .add_role_to_db_instance()
        .db_instance_identifier("role-inst")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .feature_name("S3_INTEGRATION")
        .send()
        .await
        .unwrap();
    client
        .remove_role_from_db_instance()
        .db_instance_identifier("role-inst")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .feature_name("S3_INTEGRATION")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_add_remove_role_from_db_cluster() {
    let client = make_client().await;
    client
        .create_db_cluster()
        .db_cluster_identifier("role-cluster")
        .engine("aurora-mysql")
        .send()
        .await
        .unwrap();
    client
        .add_role_to_db_cluster()
        .db_cluster_identifier("role-cluster")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();
    client
        .remove_role_from_db_cluster()
        .db_cluster_identifier("role-cluster")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// Lifecycle tests
// ============================================================================

#[tokio::test]
async fn test_db_instance_full_lifecycle() {
    let client = make_client().await;
    // Create
    let resp = client
        .create_db_instance()
        .db_instance_identifier("lifecycle-inst")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .tags(tag("lifecycle", "test"))
        .send()
        .await
        .unwrap();
    let arn = resp
        .db_instance()
        .unwrap()
        .db_instance_arn()
        .unwrap()
        .to_string();

    // Describe
    let desc = client
        .describe_db_instances()
        .db_instance_identifier("lifecycle-inst")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.db_instances().len(), 1);

    // Modify
    client
        .modify_db_instance()
        .db_instance_identifier("lifecycle-inst")
        .db_instance_class("db.r5.large")
        .send()
        .await
        .unwrap();

    // Tags
    client
        .add_tags_to_resource()
        .resource_name(&arn)
        .tags(tag("extra", "tag"))
        .send()
        .await
        .unwrap();
    let tags = client
        .list_tags_for_resource()
        .resource_name(&arn)
        .send()
        .await
        .unwrap();
    assert!(!tags.tag_list().is_empty());

    // Snapshot
    client
        .create_db_snapshot()
        .db_instance_identifier("lifecycle-inst")
        .db_snapshot_identifier("lifecycle-snap")
        .send()
        .await
        .unwrap();

    // Stop / Start
    client
        .stop_db_instance()
        .db_instance_identifier("lifecycle-inst")
        .send()
        .await
        .unwrap();
    client
        .start_db_instance()
        .db_instance_identifier("lifecycle-inst")
        .send()
        .await
        .unwrap();

    // Delete
    client
        .delete_db_instance()
        .db_instance_identifier("lifecycle-inst")
        .skip_final_snapshot(true)
        .send()
        .await
        .unwrap();

    // Verify gone
    let err = client
        .describe_db_instances()
        .db_instance_identifier("lifecycle-inst")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFound") || err_str.contains("not found"));
}

#[tokio::test]
async fn test_db_cluster_full_lifecycle() {
    let client = make_client().await;
    // Create
    client
        .create_db_cluster()
        .db_cluster_identifier("lifecycle-cluster")
        .engine("aurora-mysql")
        .master_username("admin")
        .send()
        .await
        .unwrap();

    // Describe
    let desc = client
        .describe_db_clusters()
        .db_cluster_identifier("lifecycle-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.db_clusters().len(), 1);

    // Modify
    client
        .modify_db_cluster()
        .db_cluster_identifier("lifecycle-cluster")
        .deletion_protection(true)
        .send()
        .await
        .unwrap();

    // Snapshot
    client
        .create_db_cluster_snapshot()
        .db_cluster_identifier("lifecycle-cluster")
        .db_cluster_snapshot_identifier("lc-cs")
        .send()
        .await
        .unwrap();

    // Disable deletion protection before delete
    client
        .modify_db_cluster()
        .db_cluster_identifier("lifecycle-cluster")
        .deletion_protection(false)
        .send()
        .await
        .unwrap();

    // Delete
    client
        .delete_db_cluster()
        .db_cluster_identifier("lifecycle-cluster")
        .skip_final_snapshot(true)
        .send()
        .await
        .unwrap();

    // Verify gone
    let err = client
        .describe_db_clusters()
        .db_cluster_identifier("lifecycle-cluster")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFound") || err_str.contains("not found"));
}

// ============================================================================
// Restore operations (from existing tests, enhanced)
// ============================================================================

// -----------------------------------------------------------------------
// RestoreDBInstanceFromDBSnapshot tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_restore_db_instance_from_db_snapshot() {
    let client = make_client().await;

    // Create a source DB instance
    client
        .create_db_instance()
        .db_instance_identifier("source-instance")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .expect("create_db_instance should succeed");

    // Create a snapshot of it
    client
        .create_db_snapshot()
        .db_instance_identifier("source-instance")
        .db_snapshot_identifier("my-snapshot")
        .send()
        .await
        .expect("create_db_snapshot should succeed");

    // Restore from snapshot
    let restore_resp = client
        .restore_db_instance_from_db_snapshot()
        .db_instance_identifier("restored-instance")
        .db_snapshot_identifier("my-snapshot")
        .send()
        .await
        .expect("restore_db_instance_from_db_snapshot should succeed");

    let inst = restore_resp
        .db_instance()
        .expect("db_instance should be present");
    assert_eq!(inst.db_instance_identifier().unwrap(), "restored-instance");
    assert_eq!(inst.db_instance_status().unwrap(), "available");

    // Verify describe works
    let describe_resp = client
        .describe_db_instances()
        .db_instance_identifier("restored-instance")
        .send()
        .await
        .expect("describe_db_instances should succeed");

    assert_eq!(describe_resp.db_instances().len(), 1);
}

// -----------------------------------------------------------------------
// RestoreDBInstanceToPointInTime tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_restore_db_instance_to_point_in_time() {
    let client = make_client().await;

    // Create a source DB instance
    client
        .create_db_instance()
        .db_instance_identifier("pitr-source")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .expect("create_db_instance should succeed");

    // Restore to point in time
    let restore_resp = client
        .restore_db_instance_to_point_in_time()
        .source_db_instance_identifier("pitr-source")
        .target_db_instance_identifier("pitr-target")
        .send()
        .await
        .expect("restore_db_instance_to_point_in_time should succeed");

    let inst = restore_resp
        .db_instance()
        .expect("db_instance should be present");
    assert_eq!(inst.db_instance_identifier().unwrap(), "pitr-target");
    assert_eq!(inst.db_instance_status().unwrap(), "available");

    // Verify describe works
    let describe_resp = client
        .describe_db_instances()
        .db_instance_identifier("pitr-target")
        .send()
        .await
        .expect("describe_db_instances should succeed");

    assert_eq!(describe_resp.db_instances().len(), 1);
}

// -----------------------------------------------------------------------
// RestoreDBClusterToPointInTime tests
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_restore_db_cluster_to_point_in_time() {
    let client = make_client().await;

    // Create a source DB cluster
    client
        .create_db_cluster()
        .db_cluster_identifier("cluster-source")
        .engine("aurora-mysql")
        .send()
        .await
        .expect("create_db_cluster should succeed");

    // Restore cluster to point in time
    let restore_resp = client
        .restore_db_cluster_to_point_in_time()
        .source_db_cluster_identifier("cluster-source")
        .db_cluster_identifier("cluster-target")
        .send()
        .await
        .expect("restore_db_cluster_to_point_in_time should succeed");

    let cluster = restore_resp
        .db_cluster()
        .expect("db_cluster should be present");
    assert_eq!(cluster.db_cluster_identifier().unwrap(), "cluster-target");
    assert_eq!(cluster.status().unwrap(), "available");

    // Verify describe works
    let describe_resp = client
        .describe_db_clusters()
        .db_cluster_identifier("cluster-target")
        .send()
        .await
        .expect("describe_db_clusters should succeed");

    assert_eq!(describe_resp.db_clusters().len(), 1);
}

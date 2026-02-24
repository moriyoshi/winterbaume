//! Integration tests for winterbaume ElastiCache service.

use aws_sdk_elasticache::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticache::ElastiCacheService;

async fn make_client() -> aws_sdk_elasticache::Client {
    let mock = MockAws::builder()
        .with_service(ElastiCacheService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticache::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_elasticache::Client::new(&config)
}

// ---------------------------------------------------------------------------
// CacheCluster tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_cache_cluster() {
    let client = make_client().await;

    let resp = client
        .create_cache_cluster()
        .cache_cluster_id("test-cluster")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create_cache_cluster should succeed");

    let cluster = resp.cache_cluster().expect("should have cluster");
    assert_eq!(cluster.cache_cluster_id(), Some("test-cluster"));
    assert_eq!(cluster.engine(), Some("redis"));

    // Now describe
    let resp = client
        .describe_cache_clusters()
        .cache_cluster_id("test-cluster")
        .send()
        .await
        .expect("describe_cache_clusters should succeed");

    let clusters = resp.cache_clusters();
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].cache_cluster_id(), Some("test-cluster"));
}

#[tokio::test]
async fn test_describe_cache_clusters_all() {
    let client = make_client().await;

    // Create two clusters
    client
        .create_cache_cluster()
        .cache_cluster_id("cluster-a")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create cluster-a should succeed");

    client
        .create_cache_cluster()
        .cache_cluster_id("cluster-b")
        .engine("memcached")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(2)
        .send()
        .await
        .expect("create cluster-b should succeed");

    let resp = client
        .describe_cache_clusters()
        .send()
        .await
        .expect("describe_cache_clusters should succeed");

    assert!(resp.cache_clusters().len() >= 2);
}

#[tokio::test]
async fn test_delete_cache_cluster() {
    let client = make_client().await;

    client
        .create_cache_cluster()
        .cache_cluster_id("del-cluster")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .delete_cache_cluster()
        .cache_cluster_id("del-cluster")
        .send()
        .await
        .expect("delete_cache_cluster should succeed");

    let cluster = resp
        .cache_cluster()
        .expect("should have cluster in delete response");
    assert_eq!(cluster.cache_cluster_id(), Some("del-cluster"));

    // Verify gone
    let err = client
        .describe_cache_clusters()
        .cache_cluster_id("del-cluster")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("CacheClusterNotFound") || err_str.contains("not found"));
}

#[tokio::test]
async fn test_describe_nonexistent_cache_cluster() {
    let client = make_client().await;

    let err = client
        .describe_cache_clusters()
        .cache_cluster_id("nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(err_str.contains("CacheClusterNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// ReplicationGroup tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_replication_group() {
    let client = make_client().await;

    let resp = client
        .create_replication_group()
        .replication_group_id("test-rg")
        .replication_group_description("Test replication group")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_clusters(2)
        .send()
        .await
        .expect("create_replication_group should succeed");

    let rg = resp.replication_group().expect("should have rg");
    assert_eq!(rg.replication_group_id(), Some("test-rg"));
    assert_eq!(rg.description(), Some("Test replication group"));

    // Describe it
    let resp = client
        .describe_replication_groups()
        .replication_group_id("test-rg")
        .send()
        .await
        .expect("describe_replication_groups should succeed");

    let groups = resp.replication_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].replication_group_id(), Some("test-rg"));
}

#[tokio::test]
async fn test_delete_replication_group() {
    let client = make_client().await;

    client
        .create_replication_group()
        .replication_group_id("del-rg")
        .replication_group_description("To be deleted")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .delete_replication_group()
        .replication_group_id("del-rg")
        .send()
        .await
        .expect("delete_replication_group should succeed");

    let rg = resp
        .replication_group()
        .expect("should have rg in response");
    assert_eq!(rg.replication_group_id(), Some("del-rg"));

    // Verify gone
    let err = client
        .describe_replication_groups()
        .replication_group_id("del-rg")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("ReplicationGroupNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// CacheSubnetGroup tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_cache_subnet_group() {
    let client = make_client().await;

    let resp = client
        .create_cache_subnet_group()
        .cache_subnet_group_name("test-subnet-group")
        .cache_subnet_group_description("Test subnet group")
        .subnet_ids("subnet-12345")
        .subnet_ids("subnet-67890")
        .send()
        .await
        .expect("create_cache_subnet_group should succeed");

    let sg = resp.cache_subnet_group().expect("should have subnet group");
    assert_eq!(sg.cache_subnet_group_name(), Some("test-subnet-group"));

    // Describe it
    let resp = client
        .describe_cache_subnet_groups()
        .cache_subnet_group_name("test-subnet-group")
        .send()
        .await
        .expect("describe_cache_subnet_groups should succeed");

    let groups = resp.cache_subnet_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(
        groups[0].cache_subnet_group_name(),
        Some("test-subnet-group")
    );
}

#[tokio::test]
async fn test_delete_cache_subnet_group() {
    let client = make_client().await;

    client
        .create_cache_subnet_group()
        .cache_subnet_group_name("del-subnet-group")
        .cache_subnet_group_description("To be deleted")
        .subnet_ids("subnet-abc")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_cache_subnet_group()
        .cache_subnet_group_name("del-subnet-group")
        .send()
        .await
        .expect("delete_cache_subnet_group should succeed");

    // Verify gone
    let err = client
        .describe_cache_subnet_groups()
        .cache_subnet_group_name("del-subnet-group")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("CacheSubnetGroupNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// CacheParameterGroup tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_cache_parameter_group() {
    let client = make_client().await;

    let resp = client
        .create_cache_parameter_group()
        .cache_parameter_group_name("test-param-group")
        .cache_parameter_group_family("redis7")
        .description("Test parameter group")
        .send()
        .await
        .expect("create_cache_parameter_group should succeed");

    let pg = resp
        .cache_parameter_group()
        .expect("should have parameter group");
    assert_eq!(pg.cache_parameter_group_name(), Some("test-param-group"));
    assert_eq!(pg.cache_parameter_group_family(), Some("redis7"));

    // Describe it
    let resp = client
        .describe_cache_parameter_groups()
        .cache_parameter_group_name("test-param-group")
        .send()
        .await
        .expect("describe_cache_parameter_groups should succeed");

    let groups = resp.cache_parameter_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(
        groups[0].cache_parameter_group_name(),
        Some("test-param-group")
    );
}

#[tokio::test]
async fn test_delete_cache_parameter_group() {
    let client = make_client().await;

    client
        .create_cache_parameter_group()
        .cache_parameter_group_name("del-param-group")
        .cache_parameter_group_family("redis7")
        .description("To be deleted")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_cache_parameter_group()
        .cache_parameter_group_name("del-param-group")
        .send()
        .await
        .expect("delete_cache_parameter_group should succeed");

    // Verify gone
    let err = client
        .describe_cache_parameter_groups()
        .cache_parameter_group_name("del-param-group")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("CacheParameterGroupNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// CacheSecurityGroup tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_cache_security_group() {
    let client = make_client().await;

    let resp = client
        .create_cache_security_group()
        .cache_security_group_name("test-sg")
        .description("Test security group")
        .send()
        .await
        .expect("create_cache_security_group should succeed");

    let sg = resp
        .cache_security_group()
        .expect("should have security group");
    assert_eq!(sg.cache_security_group_name(), Some("test-sg"));

    // Describe it
    let resp = client
        .describe_cache_security_groups()
        .cache_security_group_name("test-sg")
        .send()
        .await
        .expect("describe_cache_security_groups should succeed");

    let groups = resp.cache_security_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].cache_security_group_name(), Some("test-sg"));
}

#[tokio::test]
async fn test_delete_cache_security_group() {
    let client = make_client().await;

    client
        .create_cache_security_group()
        .cache_security_group_name("del-sg")
        .description("To be deleted")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_cache_security_group()
        .cache_security_group_name("del-sg")
        .send()
        .await
        .expect("delete_cache_security_group should succeed");

    // Verify gone
    let err = client
        .describe_cache_security_groups()
        .cache_security_group_name("del-sg")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("CacheSecurityGroupNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// Snapshot tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_snapshot() {
    let client = make_client().await;

    // Create a cluster to snapshot
    client
        .create_cache_cluster()
        .cache_cluster_id("snap-cluster")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create cluster should succeed");

    let resp = client
        .create_snapshot()
        .snapshot_name("test-snapshot")
        .cache_cluster_id("snap-cluster")
        .send()
        .await
        .expect("create_snapshot should succeed");

    let snap = resp.snapshot().expect("should have snapshot");
    assert_eq!(snap.snapshot_name(), Some("test-snapshot"));

    // Describe it
    let resp = client
        .describe_snapshots()
        .snapshot_name("test-snapshot")
        .send()
        .await
        .expect("describe_snapshots should succeed");

    let snaps = resp.snapshots();
    assert_eq!(snaps.len(), 1);
    assert_eq!(snaps[0].snapshot_name(), Some("test-snapshot"));
}

#[tokio::test]
async fn test_delete_snapshot() {
    let client = make_client().await;

    client
        .create_snapshot()
        .snapshot_name("del-snapshot")
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .delete_snapshot()
        .snapshot_name("del-snapshot")
        .send()
        .await
        .expect("delete_snapshot should succeed");

    let snap = resp.snapshot().expect("should have snapshot in response");
    assert_eq!(snap.snapshot_name(), Some("del-snapshot"));

    // Verify gone
    let err = client
        .describe_snapshots()
        .snapshot_name("del-snapshot")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("SnapshotNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// User tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_user() {
    let client = make_client().await;

    let resp = client
        .create_user()
        .user_id("test-user")
        .user_name("testuser")
        .engine("Redis")
        .access_string("on ~* +@all")
        .send()
        .await
        .expect("create_user should succeed");

    assert_eq!(resp.user_id(), Some("test-user"));
    assert_eq!(resp.user_name(), Some("testuser"));

    // Describe it
    let resp = client
        .describe_users()
        .user_id("test-user")
        .send()
        .await
        .expect("describe_users should succeed");

    let users = resp.users();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].user_id(), Some("test-user"));
}

#[tokio::test]
async fn test_delete_user() {
    let client = make_client().await;

    client
        .create_user()
        .user_id("del-user")
        .user_name("deluser")
        .engine("Redis")
        .access_string("off")
        .send()
        .await
        .expect("create user should succeed");

    let resp = client
        .delete_user()
        .user_id("del-user")
        .send()
        .await
        .expect("delete_user should succeed");

    assert_eq!(resp.user_id(), Some("del-user"));

    // Verify gone
    let err = client
        .describe_users()
        .user_id("del-user")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(err_str.contains("UserNotFound") || err_str.contains("not found"));
}

// ---------------------------------------------------------------------------
// Tag tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_add_list_remove_tags_on_cluster() {
    let client = make_client().await;

    // Create a cluster so we have an ARN
    let resp = client
        .create_cache_cluster()
        .cache_cluster_id("tagged-cluster")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .cache_cluster()
        .and_then(|c| c.arn())
        .expect("should have ARN");

    // Add tags
    client
        .add_tags_to_resource()
        .resource_name(arn)
        .tags(
            aws_sdk_elasticache::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .tags(
            aws_sdk_elasticache::types::Tag::builder()
                .key("team")
                .value("eng")
                .build(),
        )
        .send()
        .await
        .expect("add_tags_to_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_name(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tag_list = resp.tag_list();
    assert_eq!(tag_list.len(), 2);
    let keys: Vec<_> = tag_list.iter().filter_map(|t| t.key()).collect();
    assert!(keys.contains(&"env"));
    assert!(keys.contains(&"team"));

    // Remove one tag
    client
        .remove_tags_from_resource()
        .resource_name(arn)
        .tag_keys("env")
        .send()
        .await
        .expect("remove_tags_from_resource should succeed");

    // Verify only one tag left
    let resp = client
        .list_tags_for_resource()
        .resource_name(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed after remove");

    let tag_list = resp.tag_list();
    assert_eq!(tag_list.len(), 1);
    assert_eq!(tag_list[0].key(), Some("team"));
}

// ---------------------------------------------------------------------------
// State view tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_snapshot_restore_merge() {
    use winterbaume_core::StatefulService;
    use winterbaume_elasticache::views::{CacheClusterView, TagView};
    use winterbaume_elasticache::{ElastiCacheService, ElastiCacheStateView};

    let svc = ElastiCacheService::new();

    // Build a view with a cluster
    let mut view = ElastiCacheStateView::default();
    view.cache_clusters.insert(
        "restored-cluster".to_string(),
        CacheClusterView {
            cache_cluster_id: "restored-cluster".to_string(),
            status: "available".to_string(),
            engine: "redis".to_string(),
            engine_version: "7.0.12".to_string(),
            cache_node_type: "cache.t3.micro".to_string(),
            num_cache_nodes: 1,
            preferred_availability_zone: "us-east-1a".to_string(),
            cache_subnet_group_name: None,
            replication_group_id: None,
            arn: "arn:aws:elasticache:us-east-1:123456789012:cluster:restored-cluster".to_string(),
            tags: vec![TagView {
                key: "restored".to_string(),
                value: "true".to_string(),
            }],
            log_delivery_configuration: None,
        },
    );

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot and verify
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.cache_clusters.contains_key("restored-cluster"));

    // Merge another cluster
    let mut view2 = ElastiCacheStateView::default();
    view2.cache_clusters.insert(
        "merged-cluster".to_string(),
        CacheClusterView {
            cache_cluster_id: "merged-cluster".to_string(),
            status: "available".to_string(),
            engine: "memcached".to_string(),
            engine_version: "1.6.12".to_string(),
            cache_node_type: "cache.t3.small".to_string(),
            num_cache_nodes: 2,
            preferred_availability_zone: "us-east-1b".to_string(),
            cache_subnet_group_name: None,
            replication_group_id: None,
            arn: "arn:aws:elasticache:us-east-1:123456789012:cluster:merged-cluster".to_string(),
            tags: vec![],
            log_delivery_configuration: None,
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    // Both clusters should be present
    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.cache_clusters.contains_key("restored-cluster"));
    assert!(snapshot.cache_clusters.contains_key("merged-cluster"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = ElastiCacheService::new();
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
    use winterbaume_elasticache::ElastiCacheStateView;
    use winterbaume_elasticache::views::{CacheClusterView, TagView};

    let svc = ElastiCacheService::new();

    // Pre-seed state
    let view = ElastiCacheStateView::default();
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap(); // ignore first event

    // Re-register and capture snapshot
    let snapshots: Arc<Mutex<Vec<ElastiCacheStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut view2 = ElastiCacheStateView::default();
    view2.cache_clusters.insert(
        "listener-cluster".to_string(),
        CacheClusterView {
            cache_cluster_id: "listener-cluster".to_string(),
            status: "available".to_string(),
            engine: "redis".to_string(),
            engine_version: "7.0.12".to_string(),
            cache_node_type: "cache.t3.micro".to_string(),
            num_cache_nodes: 1,
            preferred_availability_zone: "us-east-1a".to_string(),
            cache_subnet_group_name: None,
            replication_group_id: None,
            arn: "arn:aws:elasticache:us-east-1:123456789012:cluster:listener-cluster".to_string(),
            tags: vec![TagView {
                key: "k".to_string(),
                value: "v".to_string(),
            }],
            log_delivery_configuration: None,
        },
    );

    svc.restore("123456789012", "us-east-1", view2)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].cache_clusters.contains_key("listener-cluster"));
}

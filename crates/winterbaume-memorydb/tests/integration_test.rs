//! Integration tests for winterbaume MemoryDB service.
//! Translated from moto's test_memorydb.py with EXACT value assertions.

use aws_sdk_memorydb::config::BehaviorVersion;
use aws_sdk_memorydb::types::Tag;
use winterbaume_core::MockAws;
use winterbaume_memorydb::MemoryDbService;

/// Helper to create a configured MemoryDB client backed by winterbaume.
async fn make_client() -> aws_sdk_memorydb::Client {
    let mock = MockAws::builder()
        .with_service(MemoryDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_memorydb::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_memorydb::Client::new(&config)
}

// ==================== Cluster tests ====================

/// Moto parity: test_create_cluster
#[tokio::test]
async fn test_create_cluster() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert!(cluster.name().is_some());
    assert!(cluster.status().is_some());
    assert!(cluster.number_of_shards().is_some());
}

/// Moto parity: test_create_duplicate_cluster_fails
#[tokio::test]
async fn test_create_duplicate_cluster_fails() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("foo-bar")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_cluster()
        .cluster_name("foo-bar")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await;

    assert!(result.is_err(), "creating duplicate cluster should fail");
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterAlreadyExistsFault"));
}

/// Moto parity: test_describe_clusters
#[tokio::test]
async fn test_describe_clusters() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .expect("create_cluster should succeed");
    }

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");

    let clusters = resp.clusters();
    assert_eq!(clusters.len(), 2);
    // Without ShowShardDetails, Shards should be None
    assert!(clusters[0].shards().is_empty());
}

/// Moto parity: test_describe_clusters_with_shard_details
#[tokio::test]
async fn test_describe_clusters_with_shard_details() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .expect("create_cluster should succeed");
    }

    let resp = client
        .describe_clusters()
        .cluster_name("test-memory-db-1")
        .show_shard_details(true)
        .send()
        .await
        .expect("describe_clusters should succeed");

    let clusters = resp.clusters();
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].name(), Some("test-memory-db-1"));
    assert!(!clusters[0].shards().is_empty());
}

/// Moto parity: test_describe_clusters_with_cluster_name
#[tokio::test]
async fn test_describe_clusters_with_cluster_name() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .expect("create_cluster should succeed");
    }

    let resp = client
        .describe_clusters()
        .cluster_name("test-memory-db-1")
        .send()
        .await
        .expect("describe_clusters should succeed");

    let clusters = resp.clusters();
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].name(), Some("test-memory-db-1"));
}

/// Moto parity: test_update_cluster_replica_count
#[tokio::test]
async fn test_update_cluster_replica_count() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    // Before update: default 1 replica + 1 primary = 2 nodes per shard
    let desc_before = client
        .describe_clusters()
        .show_shard_details(true)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        desc_before.clusters()[0].shards()[0].number_of_nodes(),
        Some(2)
    );

    client
        .update_cluster()
        .cluster_name("test-memory-db")
        .description("Good cluster")
        .maintenance_window("thu:23:00-thu:01:30")
        .replica_configuration(
            aws_sdk_memorydb::types::ReplicaConfigurationRequest::builder()
                .replica_count(2)
                .build(),
        )
        .send()
        .await
        .expect("update_cluster should succeed");

    let desc_after = client
        .describe_clusters()
        .show_shard_details(true)
        .send()
        .await
        .expect("describe should succeed");
    let cluster = &desc_after.clusters()[0];
    assert_eq!(cluster.description(), Some("Good cluster"));
    assert_eq!(cluster.maintenance_window(), Some("thu:23:00-thu:01:30"));
    // After update: 2 replicas + 1 primary = 3 nodes per shard
    assert_eq!(cluster.shards()[0].number_of_nodes(), Some(3));
}

/// Moto parity: test_update_cluster_shards
#[tokio::test]
async fn test_update_cluster_shards() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    let desc_before = client
        .describe_clusters()
        .show_shard_details(true)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_before.clusters()[0].number_of_shards(), Some(1));

    client
        .update_cluster()
        .cluster_name("test-memory-db")
        .shard_configuration(
            aws_sdk_memorydb::types::ShardConfigurationRequest::builder()
                .shard_count(2)
                .build(),
        )
        .send()
        .await
        .expect("update_cluster should succeed");

    let desc_after = client
        .describe_clusters()
        .show_shard_details(true)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_after.clusters()[0].number_of_shards(), Some(2));
}

/// Moto parity: test_update_invalid_cluster_fails
#[tokio::test]
async fn test_update_invalid_cluster_fails() {
    let client = make_client().await;

    let result = client
        .update_cluster()
        .cluster_name("foobar")
        .description("Good cluster")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

/// Moto parity: test_delete_cluster
#[tokio::test]
async fn test_delete_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    let desc_before = client
        .describe_clusters()
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_before.clusters().len(), 1);

    let del_resp = client
        .delete_cluster()
        .cluster_name("test-memory-db")
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(del_resp.cluster().unwrap().name(), Some("test-memory-db"));

    let desc_after = client
        .describe_clusters()
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_after.clusters().len(), 0);
}

/// Moto parity: test_delete_cluster_with_snapshot
#[tokio::test]
async fn test_delete_cluster_with_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    let desc_before = client
        .describe_snapshots()
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_before.snapshots().len(), 0);

    let del_resp = client
        .delete_cluster()
        .cluster_name("test-memory-db")
        .final_snapshot_name("test-memory-db-snapshot")
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(del_resp.cluster().unwrap().name(), Some("test-memory-db"));

    let desc_after = client
        .describe_snapshots()
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc_after.snapshots().len(), 1);
    assert_eq!(
        desc_after.snapshots()[0].name(),
        Some("test-memory-db-snapshot")
    );
}

/// Moto parity: test_delete_invalid_cluster_fails
#[tokio::test]
async fn test_delete_invalid_cluster_fails() {
    let client = make_client().await;

    let result = client.delete_cluster().cluster_name("foobar").send().await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

// ==================== Snapshot tests ====================

/// Moto parity: test_create_snapshot
#[tokio::test]
async fn test_create_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .description("Test memorydb cluster")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    let resp = client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot-1")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/51d81fab-b138-4bd2-8a09-07fd6d37224d")
        .tags(Tag::builder().key("foo").value("bar").build())
        .send()
        .await
        .expect("create_snapshot should succeed");

    let snapshot = resp.snapshot().expect("should have snapshot");
    assert!(snapshot.name().is_some());
    assert!(snapshot.status().is_some());
    assert!(snapshot.source().is_some());
    assert!(snapshot.kms_key_id().is_some());
    assert!(snapshot.arn().is_some());
    assert!(snapshot.cluster_configuration().is_some());
    assert!(snapshot.data_tiering().is_some());
}

/// Moto parity: test_create_snapshot_with_non_existing_cluster_fails
#[tokio::test]
async fn test_create_snapshot_with_non_existing_cluster_fails() {
    let client = make_client().await;

    let result = client
        .create_snapshot()
        .cluster_name("foobar")
        .snapshot_name("my-snapshot-1")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

/// Moto parity: test_create_duplicate_snapshot_fails
#[tokio::test]
async fn test_create_duplicate_snapshot_fails() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster should succeed");

    client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot-1")
        .send()
        .await
        .expect("first snapshot should succeed");

    let result = client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot-1")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotAlreadyExistsFault"));
}

/// Moto parity: test_describe_snapshots
#[tokio::test]
async fn test_describe_snapshots() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .expect("create_cluster should succeed");

        client
            .create_snapshot()
            .cluster_name(format!("test-memory-db-{i}"))
            .snapshot_name(format!("my-snapshot-{i}"))
            .send()
            .await
            .expect("create_snapshot should succeed");
    }

    let resp = client
        .describe_snapshots()
        .send()
        .await
        .expect("describe_snapshots should succeed");

    let snapshots = resp.snapshots();
    assert_eq!(snapshots.len(), 2);
}

/// Moto parity: test_describe_snapshots_with_cluster_name
#[tokio::test]
async fn test_describe_snapshots_with_cluster_name() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .unwrap();
        client
            .create_snapshot()
            .cluster_name(format!("test-memory-db-{i}"))
            .snapshot_name(format!("my-snapshot-{i}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_snapshots()
        .cluster_name("test-memory-db-2")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(resp.snapshots().len(), 1);
    assert_eq!(
        resp.snapshots()[0].cluster_configuration().unwrap().name(),
        Some("test-memory-db-2")
    );
    // Without ShowDetail, Shards should be empty
    assert!(
        resp.snapshots()[0]
            .cluster_configuration()
            .unwrap()
            .shards()
            .is_empty()
    );
}

/// Moto parity: test_describe_snapshots_with_shard_details
#[tokio::test]
async fn test_describe_snapshots_with_shard_details() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .unwrap();
        client
            .create_snapshot()
            .cluster_name(format!("test-memory-db-{i}"))
            .snapshot_name(format!("my-snapshot-{i}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_snapshots()
        .cluster_name("test-memory-db-2")
        .show_detail(true)
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(resp.snapshots().len(), 1);
    assert_eq!(
        resp.snapshots()[0].cluster_configuration().unwrap().name(),
        Some("test-memory-db-2")
    );
    // With ShowDetail, Shards should be present
    assert!(
        !resp.snapshots()[0]
            .cluster_configuration()
            .unwrap()
            .shards()
            .is_empty()
    );
}

/// Moto parity: test_describe_snapshots_with_snapshot_name
#[tokio::test]
async fn test_describe_snapshots_with_snapshot_name() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_cluster()
            .cluster_name(format!("test-memory-db-{i}"))
            .node_type("db.t4g.small")
            .acl_name("open-access")
            .send()
            .await
            .unwrap();
        client
            .create_snapshot()
            .cluster_name(format!("test-memory-db-{i}"))
            .snapshot_name(format!("my-snapshot-{i}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_snapshots()
        .snapshot_name("my-snapshot-1")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(resp.snapshots().len(), 1);
    assert_eq!(resp.snapshots()[0].name(), Some("my-snapshot-1"));
}

/// Moto parity: test_describe_snapshots_with_snapshot_and_cluster
#[tokio::test]
async fn test_describe_snapshots_with_snapshot_and_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .unwrap();

    for i in 1..=2 {
        client
            .create_snapshot()
            .cluster_name("test-memory-db")
            .snapshot_name(format!("my-snapshot-{i}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_snapshots()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot-1")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(resp.snapshots().len(), 1);
    assert_eq!(resp.snapshots()[0].name(), Some("my-snapshot-1"));
}

/// Moto parity: test_describe_snapshots_with_invalid_cluster
#[tokio::test]
async fn test_describe_snapshots_with_invalid_cluster() {
    let client = make_client().await;

    let resp = client
        .describe_snapshots()
        .cluster_name("foobar")
        .send()
        .await
        .expect("describe with nonexistent cluster should return empty");

    assert_eq!(resp.snapshots().len(), 0);
}

/// Moto parity: test_describe_snapshots_invalid_snapshot_fails
#[tokio::test]
async fn test_describe_snapshots_invalid_snapshot_fails() {
    let client = make_client().await;

    let result = client
        .describe_snapshots()
        .snapshot_name("foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotNotFoundFault"));
}

/// Moto parity: test_describe_snapshots_with_cluster_and_invalid_snapshot_fails
#[tokio::test]
async fn test_describe_snapshots_with_cluster_and_invalid_snapshot_fails() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .unwrap();
    client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot")
        .send()
        .await
        .unwrap();

    let result = client
        .describe_snapshots()
        .cluster_name("test-memory-db")
        .snapshot_name("foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotNotFoundFault"));
}

/// Moto parity: test_delete_snapshot
#[tokio::test]
async fn test_delete_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .unwrap();

    client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot-1")
        .send()
        .await
        .unwrap();

    let desc_before = client.describe_snapshots().send().await.unwrap();
    assert_eq!(desc_before.snapshots().len(), 1);

    let del_resp = client
        .delete_snapshot()
        .snapshot_name("my-snapshot-1")
        .send()
        .await
        .expect("delete should succeed");
    assert!(del_resp.snapshot().is_some());

    let desc_after = client.describe_snapshots().send().await.unwrap();
    assert_eq!(desc_after.snapshots().len(), 0);
}

/// Moto parity: test_delete_invalid_snapshot_fails
#[tokio::test]
async fn test_delete_invalid_snapshot_fails() {
    let client = make_client().await;

    let result = client
        .delete_snapshot()
        .snapshot_name("foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotNotFoundFault"));
}

// ==================== Subnet group tests ====================

/// Moto parity: test_create_subnet_group
#[tokio::test]
async fn test_create_subnet_group() {
    let client = make_client().await;

    let resp = client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .description("This is my subnet group")
        .subnet_ids("subnet-12345")
        .subnet_ids("subnet-67890")
        .tags(Tag::builder().key("foo").value("bar").build())
        .send()
        .await
        .expect("create_subnet_group should succeed");

    let sg = resp.subnet_group().expect("should have subnet group");
    assert!(sg.name().is_some());
    assert!(sg.description().is_some());
    assert!(sg.vpc_id().is_some());
    assert!(!sg.subnets().is_empty());
    assert!(sg.arn().is_some());
}

/// Moto parity: test_create_duplicate_subnet_group_fails
#[tokio::test]
async fn test_create_duplicate_subnet_group_fails() {
    let client = make_client().await;

    client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .subnet_ids("subnet-12345")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SubnetGroupAlreadyExistsFault"));
}

/// Moto parity: test_describe_subnet_groups_with_subnet_group_name
#[tokio::test]
async fn test_describe_subnet_groups_with_subnet_group_name() {
    let client = make_client().await;

    for i in 1..=2 {
        client
            .create_subnet_group()
            .subnet_group_name(format!("my_subnet_group-{i}"))
            .description("This is my subnet group")
            .subnet_ids("subnet-12345")
            .subnet_ids("subnet-67890")
            .send()
            .await
            .expect("create_subnet_group should succeed");
    }

    let resp = client
        .describe_subnet_groups()
        .subnet_group_name("my_subnet_group-1")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(resp.subnet_groups().len(), 1);
    assert_eq!(resp.subnet_groups()[0].name(), Some("my_subnet_group-1"));
}

/// Moto parity: test_describe_subnet_groups_invalid_subnetgroupname_fails
#[tokio::test]
async fn test_describe_subnet_groups_invalid_subnetgroupname_fails() {
    let client = make_client().await;

    let result = client
        .describe_subnet_groups()
        .subnet_group_name("foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SubnetGroupNotFoundFault"));
}

/// Moto parity: test_delete_subnet_group
#[tokio::test]
async fn test_delete_subnet_group() {
    let client = make_client().await;

    client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create should succeed");

    let del_resp = client
        .delete_subnet_group()
        .subnet_group_name("my_subnet_group")
        .send()
        .await
        .expect("delete should succeed");

    assert!(del_resp.subnet_group().is_some());

    let result = client
        .describe_subnet_groups()
        .subnet_group_name("my_subnet_group")
        .send()
        .await;

    assert!(result.is_err());
}

/// Moto parity: test_delete_subnet_group_default_fails
#[tokio::test]
async fn test_delete_subnet_group_default_fails() {
    let client = make_client().await;

    let result = client
        .delete_subnet_group()
        .subnet_group_name("default")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("InvalidParameterValueException"));
}

/// Moto parity: test_delete_subnet_group_in_use_fails
#[tokio::test]
async fn test_delete_subnet_group_in_use_fails() {
    let client = make_client().await;

    client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create subnet group should succeed");

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .subnet_group_name("my_subnet_group")
        .acl_name("open-access")
        .send()
        .await
        .expect("create cluster should succeed");

    let result = client
        .delete_subnet_group()
        .subnet_group_name("my_subnet_group")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SubnetGroupInUseFault"));
}

/// Moto parity: test_create_cluster_with_subnet_group
#[tokio::test]
async fn test_create_cluster_with_subnet_group() {
    let client = make_client().await;

    client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .subnet_ids("subnet-12345")
        .send()
        .await
        .expect("create subnet group should succeed");

    let resp = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .subnet_group_name("my_subnet_group")
        .acl_name("open-access")
        .send()
        .await
        .expect("create cluster should succeed");

    assert_eq!(
        resp.cluster().unwrap().subnet_group_name(),
        Some("my_subnet_group")
    );
}

// ==================== Tag tests ====================

/// Moto parity: test_list_tags
#[tokio::test]
async fn test_list_tags() {
    let client = make_client().await;

    let cluster = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .tags(Tag::builder().key("foo").value("bar").build())
        .send()
        .await
        .expect("create_cluster should succeed");

    let arn = cluster.cluster().unwrap().arn().unwrap().to_string();

    let resp = client
        .list_tags()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = resp.tag_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("foo"));
    assert_eq!(tags[0].value(), Some("bar"));
}

/// Moto parity: test_list_tags_snapshot
#[tokio::test]
async fn test_list_tags_snapshot() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .unwrap();

    let snapshot = client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot")
        .tags(Tag::builder().key("foo1").value("bar1").build())
        .send()
        .await
        .unwrap();

    let arn = snapshot.snapshot().unwrap().arn().unwrap().to_string();

    let resp = client
        .list_tags()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");

    assert_eq!(resp.tag_list().len(), 1);
}

/// Moto parity: test_list_tags_subnetgroups
#[tokio::test]
async fn test_list_tags_subnetgroups() {
    let client = make_client().await;

    let sg = client
        .create_subnet_group()
        .subnet_group_name("my_subnet_group")
        .subnet_ids("subnet-12345")
        .tags(Tag::builder().key("foo").value("bar").build())
        .send()
        .await
        .unwrap();

    let arn = sg.subnet_group().unwrap().arn().unwrap().to_string();

    let resp = client
        .list_tags()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");

    assert_eq!(resp.tag_list().len(), 1);
}

/// Moto parity: test_list_tags_invalid_cluster_fails
#[tokio::test]
async fn test_list_tags_invalid_cluster_fails() {
    let client = make_client().await;

    let result = client
        .list_tags()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:cluster/foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

/// Moto parity: test_list_tags_invalid_snapshot_fails
#[tokio::test]
async fn test_list_tags_invalid_snapshot_fails() {
    let client = make_client().await;

    let result = client
        .list_tags()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:snapshot/foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotNotFoundFault"));
}

/// Moto parity: test_list_tags_invalid_subnetgroup_fails
#[tokio::test]
async fn test_list_tags_invalid_subnetgroup_fails() {
    let client = make_client().await;

    let result = client
        .list_tags()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:subnetgroup/foobar")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SubnetGroupNotFoundFault"));
}

/// Moto parity: test_tag_resource
#[tokio::test]
async fn test_tag_resource() {
    let client = make_client().await;

    let cluster = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .tags(Tag::builder().key("key1").value("value1").build())
        .send()
        .await
        .unwrap();

    let arn = cluster.cluster().unwrap().arn().unwrap().to_string();

    let resp = client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("key2").value("value2").build())
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags = resp.tag_list();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[1].key(), Some("key2"));
    assert_eq!(tags[1].value(), Some("value2"));
}

/// Moto parity: test_tag_resource_invalid_cluster_fails
#[tokio::test]
async fn test_tag_resource_invalid_cluster_fails() {
    let client = make_client().await;

    let result = client
        .tag_resource()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:cluster/foobar")
        .tags(Tag::builder().key("key2").value("value2").build())
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

/// Moto parity: test_tag_resource_invalid_snapshot_fails
#[tokio::test]
async fn test_tag_resource_invalid_snapshot_fails() {
    let client = make_client().await;

    let result = client
        .tag_resource()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:snapshot/foobar")
        .tags(Tag::builder().key("key2").value("value2").build())
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotNotFoundFault"));
}

/// Moto parity: test_tag_resource_invalid_subnetgroup_fails
#[tokio::test]
async fn test_tag_resource_invalid_subnetgroup_fails() {
    let client = make_client().await;

    let result = client
        .tag_resource()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:subnetgroup/foobar")
        .tags(Tag::builder().key("key2").value("value2").build())
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SubnetGroupNotFoundFault"));
}

/// Moto parity: test_untag_resource
#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let cluster = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .tags(Tag::builder().key("key1").value("value1").build())
        .tags(Tag::builder().key("key2").value("value2").build())
        .send()
        .await
        .unwrap();

    let arn = cluster.cluster().unwrap().arn().unwrap().to_string();

    let resp = client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key1")
        .send()
        .await
        .expect("untag should succeed");

    let tags = resp.tag_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("key2"));
    assert_eq!(tags[0].value(), Some("value2"));
}

/// Moto parity: test_untag_resource_invalid_cluster_fails
#[tokio::test]
async fn test_untag_resource_invalid_cluster_fails() {
    let client = make_client().await;

    let result = client
        .untag_resource()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:cluster/foobar")
        .tag_keys("key1")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

/// Moto parity: test_untag_resource_invalid_snapshot_fails
#[tokio::test]
async fn test_untag_resource_invalid_snapshot_fails() {
    let client = make_client().await;

    let result = client
        .untag_resource()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:snapshot/foobar")
        .tag_keys("key1")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SnapshotNotFoundFault"));
}

/// Moto parity: test_untag_resource_invalid_subnetgroup_fails
#[tokio::test]
async fn test_untag_resource_invalid_subnetgroup_fails() {
    let client = make_client().await;

    let result = client
        .untag_resource()
        .resource_arn("arn:aws:memorydb:us-east-1:123456789012:subnetgroup/foobar")
        .tag_keys("key1")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("SubnetGroupNotFoundFault"));
}

/// Moto parity: test_untag_resource_invalid_keys_fails
#[tokio::test]
async fn test_untag_resource_invalid_keys_fails() {
    let client = make_client().await;

    let cluster = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .tags(Tag::builder().key("key1").value("value1").build())
        .tags(Tag::builder().key("key2").value("value2").build())
        .send()
        .await
        .unwrap();

    let arn = cluster.cluster().unwrap().arn().unwrap().to_string();

    let result = client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key3")
        .tag_keys("key4")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("TagNotFoundFault"));
}

// ============================================================================
// Tests derived from AWS documentation: MemoryDB
// ============================================================================

/// Create a cluster with all optional fields set; verify each is reflected in the describe response.
#[tokio::test]
async fn test_create_cluster_with_optional_fields() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("test-full-cluster")
        .node_type("db.r6g.large")
        .acl_name("open-access")
        .description("My full cluster")
        .engine_version("7.1")
        .num_shards(2)
        .num_replicas_per_shard(2)
        .tls_enabled(false)
        .auto_minor_version_upgrade(false)
        .snapshot_retention_limit(5)
        .snapshot_window("02:00-03:00")
        .maintenance_window("mon:01:00-mon:02:00")
        .send()
        .await
        .expect("create_cluster with optional fields should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(cluster.name(), Some("test-full-cluster"));
    assert_eq!(cluster.description(), Some("My full cluster"));
    assert_eq!(cluster.engine_version(), Some("7.1"));
    assert_eq!(cluster.number_of_shards(), Some(2));
    assert_eq!(cluster.tls_enabled(), Some(false));
    assert_eq!(cluster.auto_minor_version_upgrade(), Some(false));
    assert_eq!(cluster.snapshot_retention_limit(), Some(5));
    assert_eq!(cluster.snapshot_window(), Some("02:00-03:00"));
    assert_eq!(cluster.maintenance_window(), Some("mon:01:00-mon:02:00"));

    // Verify via DescribeClusters as well
    let desc = client
        .describe_clusters()
        .cluster_name("test-full-cluster")
        .show_shard_details(true)
        .send()
        .await
        .expect("describe should succeed");
    let c = &desc.clusters()[0];
    assert_eq!(c.number_of_shards(), Some(2));
    // 2 replicas + 1 primary = 3 nodes per shard
    assert_eq!(c.shards()[0].number_of_nodes(), Some(3));
}

/// DescribeClusters with a nonexistent cluster name should return ClusterNotFoundFault.
#[tokio::test]
async fn test_describe_clusters_not_found() {
    let client = make_client().await;

    let result = client
        .describe_clusters()
        .cluster_name("nonexistent-cluster")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describing nonexistent cluster should fail"
    );
    let err = result.unwrap_err();
    let meta = err.into_service_error().meta().clone();
    assert_eq!(meta.code(), Some("ClusterNotFoundFault"));
}

/// UpdateCluster: update NodeType and ACLName; verify the changes are reflected.
#[tokio::test]
async fn test_update_cluster_node_type_and_acl() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create should succeed");

    let update_resp = client
        .update_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.r6g.large")
        .acl_name("my-acl")
        .send()
        .await
        .expect("update_cluster should succeed");

    let cluster = update_resp.cluster().expect("should have cluster");
    assert_eq!(cluster.node_type(), Some("db.r6g.large"));
    assert_eq!(cluster.acl_name(), Some("my-acl"));

    // Verify via describe
    let desc = client
        .describe_clusters()
        .cluster_name("test-memory-db")
        .send()
        .await
        .expect("describe should succeed");
    let c = &desc.clusters()[0];
    assert_eq!(c.node_type(), Some("db.r6g.large"));
    assert_eq!(c.acl_name(), Some("my-acl"));
}

/// UpdateCluster: update snapshot_window and snapshot_retention_limit; verify changes.
#[tokio::test]
async fn test_update_cluster_snapshot_settings() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create should succeed");

    client
        .update_cluster()
        .cluster_name("test-memory-db")
        .snapshot_window("03:00-04:00")
        .snapshot_retention_limit(7)
        .send()
        .await
        .expect("update snapshot settings should succeed");

    let desc = client
        .describe_clusters()
        .cluster_name("test-memory-db")
        .send()
        .await
        .expect("describe should succeed");

    let cluster = &desc.clusters()[0];
    assert_eq!(cluster.snapshot_window(), Some("03:00-04:00"));
    assert_eq!(cluster.snapshot_retention_limit(), Some(7));
}

/// DescribeSubnetGroups without a filter should return all subnet groups.
#[tokio::test]
async fn test_describe_subnet_groups_all() {
    let client = make_client().await;

    for i in 1..=3 {
        client
            .create_subnet_group()
            .subnet_group_name(format!("sg-{i}"))
            .subnet_ids("subnet-12345")
            .send()
            .await
            .expect("create_subnet_group should succeed");
    }

    let resp = client
        .describe_subnet_groups()
        .send()
        .await
        .expect("describe_subnet_groups without filter should succeed");

    assert_eq!(resp.subnet_groups().len(), 3);
}

/// DescribeSnapshots filtered by Source="manual" should return only manual snapshots.
#[tokio::test]
async fn test_describe_snapshots_source_filter() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create cluster should succeed");

    for i in 1..=2 {
        client
            .create_snapshot()
            .cluster_name("test-memory-db")
            .snapshot_name(format!("snap-{i}"))
            .send()
            .await
            .expect("create_snapshot should succeed");
    }

    // All created snapshots have source="manual"
    let resp = client
        .describe_snapshots()
        .source("manual")
        .send()
        .await
        .expect("describe_snapshots with source filter should succeed");

    assert_eq!(resp.snapshots().len(), 2);
    for snap in resp.snapshots() {
        assert_eq!(snap.source(), Some("manual"));
    }

    // An unknown source should return an empty list
    let resp_empty = client
        .describe_snapshots()
        .source("automated")
        .send()
        .await
        .expect("describe_snapshots with unknown source should return empty");

    assert_eq!(resp_empty.snapshots().len(), 0);
}

/// DeleteSnapshot should return the deleted snapshot with status set to "deleting".
#[tokio::test]
async fn test_delete_snapshot_returns_deleting_status() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create cluster should succeed");

    client
        .create_snapshot()
        .cluster_name("test-memory-db")
        .snapshot_name("my-snapshot")
        .send()
        .await
        .expect("create_snapshot should succeed");

    let del_resp = client
        .delete_snapshot()
        .snapshot_name("my-snapshot")
        .send()
        .await
        .expect("delete_snapshot should succeed");

    let snapshot = del_resp.snapshot().expect("should have snapshot");
    assert_eq!(snapshot.name(), Some("my-snapshot"));
    assert_eq!(
        snapshot.status(),
        Some("deleting"),
        "deleted snapshot should have status 'deleting'"
    );
}

/// TagResource with an existing key should update the value, not create a duplicate.
#[tokio::test]
async fn test_tag_resource_update_existing_key() {
    let client = make_client().await;

    let cluster = client
        .create_cluster()
        .cluster_name("test-memory-db")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .tags(Tag::builder().key("env").value("staging").build())
        .send()
        .await
        .expect("create_cluster should succeed");

    let arn = cluster.cluster().unwrap().arn().unwrap().to_string();

    // Overwrite "env" with a new value
    let resp = client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("env").value("production").build())
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags = resp.tag_list();
    // There should still be exactly 1 tag — no duplicate key
    assert_eq!(
        tags.len(),
        1,
        "tag with existing key should be updated, not duplicated"
    );
    assert_eq!(tags[0].key(), Some("env"));
    assert_eq!(tags[0].value(), Some("production"));

    // Verify via list_tags
    let list_resp = client
        .list_tags()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags should succeed");
    let list_tags = list_resp.tag_list();
    assert_eq!(list_tags.len(), 1);
    assert_eq!(list_tags[0].value(), Some("production"));
}

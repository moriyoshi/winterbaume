//! Smoke tests for winterbaume MemoryDB service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_memorydb::config::BehaviorVersion;
use aws_sdk_memorydb::types::Tag;
use winterbaume_core::MockAws;
use winterbaume_memorydb::MemoryDbService;

async fn make_memorydb_client() -> aws_sdk_memorydb::Client {
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

/// Scenario: cluster lifecycle CRUD.
///
/// An operator provisions a Redis-compatible MemoryDB cluster, verifies it
/// shows up in DescribeClusters, then tears it down and confirms the cluster
/// is gone.
#[tokio::test]
async fn test_cluster_lifecycle_crud() {
    let client = make_memorydb_client().await;

    // Create a cluster.
    let create = client
        .create_cluster()
        .cluster_name("orders-cache")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .description("Order service cache")
        .send()
        .await
        .expect("create_cluster");
    let created = create.cluster().expect("cluster present in response");
    assert_eq!(created.name(), Some("orders-cache"));
    assert!(created.status().is_some());
    assert!(created.arn().is_some());

    // Describe — exactly one cluster, named as created.
    let desc = client
        .describe_clusters()
        .cluster_name("orders-cache")
        .send()
        .await
        .expect("describe_clusters");
    assert_eq!(desc.clusters().len(), 1);
    assert_eq!(desc.clusters()[0].name(), Some("orders-cache"));
    assert_eq!(
        desc.clusters()[0].description(),
        Some("Order service cache")
    );

    // Delete the cluster (no final snapshot).
    let del = client
        .delete_cluster()
        .cluster_name("orders-cache")
        .send()
        .await
        .expect("delete_cluster");
    assert_eq!(del.cluster().unwrap().name(), Some("orders-cache"));

    // Describe — cluster gone.
    let desc_after = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters after delete");
    assert!(
        desc_after
            .clusters()
            .iter()
            .all(|c| c.name() != Some("orders-cache")),
        "deleted cluster should not appear"
    );
}

/// Scenario: snapshot lifecycle around cluster deletion.
///
/// A platform team takes an explicit snapshot mid-flight, then deletes the
/// cluster while specifying `FinalSnapshotName`. Both snapshots must persist
/// after the cluster is gone, so a recovery flow could later restore from
/// either point in time.
#[tokio::test]
async fn test_cluster_snapshot_lifecycle_with_final_snapshot() {
    let client = make_memorydb_client().await;

    // Provision the cluster.
    client
        .create_cluster()
        .cluster_name("session-store")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .send()
        .await
        .expect("create_cluster");

    // Take an explicit, mid-life snapshot.
    let snap = client
        .create_snapshot()
        .cluster_name("session-store")
        .snapshot_name("session-store-pre-deploy")
        .tags(Tag::builder().key("Stage").value("pre-deploy").build())
        .send()
        .await
        .expect("create_snapshot");
    assert_eq!(
        snap.snapshot().unwrap().name(),
        Some("session-store-pre-deploy")
    );
    assert!(snap.snapshot().unwrap().arn().is_some());

    // Delete the cluster, requesting a final snapshot at teardown time.
    client
        .delete_cluster()
        .cluster_name("session-store")
        .final_snapshot_name("session-store-final")
        .send()
        .await
        .expect("delete_cluster with final_snapshot_name");

    // Both snapshots must persist after the cluster is gone.
    let snaps = client
        .describe_snapshots()
        .send()
        .await
        .expect("describe_snapshots");
    let names: Vec<&str> = snaps.snapshots().iter().filter_map(|s| s.name()).collect();
    assert!(
        names.contains(&"session-store-pre-deploy"),
        "explicit snapshot should survive cluster delete; got {names:?}"
    );
    assert!(
        names.contains(&"session-store-final"),
        "final snapshot must persist after cluster delete; got {names:?}"
    );
}

/// Scenario: cluster create chained with associated subnet group.
///
/// A network admin first creates a subnet group, then provisions a cluster
/// that references it. Cluster describe must reflect the chosen subnet group,
/// and removing the cluster must not orphan the subnet group (it can be
/// deleted independently afterwards).
#[tokio::test]
async fn test_cluster_with_subnet_group_chain() {
    let client = make_memorydb_client().await;

    // Create the subnet group.
    let sg = client
        .create_subnet_group()
        .subnet_group_name("orders-subnets")
        .description("Order service subnets")
        .subnet_ids("subnet-aaaa")
        .subnet_ids("subnet-bbbb")
        .send()
        .await
        .expect("create_subnet_group");
    assert_eq!(sg.subnet_group().unwrap().name(), Some("orders-subnets"));

    // Provision a cluster pinned to that subnet group.
    let create = client
        .create_cluster()
        .cluster_name("orders-cache")
        .node_type("db.t4g.small")
        .acl_name("open-access")
        .subnet_group_name("orders-subnets")
        .send()
        .await
        .expect("create_cluster referencing subnet group");
    assert_eq!(
        create.cluster().unwrap().subnet_group_name(),
        Some("orders-subnets")
    );

    // Tear down the cluster and verify the subnet group can still be
    // deleted on its own.
    client
        .delete_cluster()
        .cluster_name("orders-cache")
        .send()
        .await
        .expect("delete_cluster");

    let del_sg = client
        .delete_subnet_group()
        .subnet_group_name("orders-subnets")
        .send()
        .await
        .expect("delete_subnet_group");
    assert_eq!(
        del_sg.subnet_group().unwrap().name(),
        Some("orders-subnets")
    );
}

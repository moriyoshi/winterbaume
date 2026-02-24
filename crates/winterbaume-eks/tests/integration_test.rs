use aws_sdk_eks::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_eks::EksService;

async fn make_eks_client() -> aws_sdk_eks::Client {
    let mock = MockAws::builder().with_service(EksService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_eks::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_eks::Client::new(&config)
}

#[tokio::test]
async fn test_create_cluster() {
    let client = make_eks_client().await;

    let resp = client
        .create_cluster()
        .name("test-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(cluster.name(), Some("test-cluster"));
    assert!(cluster.arn().is_some());
    assert!(cluster.endpoint().is_some());
    assert_eq!(
        cluster.status(),
        Some(&aws_sdk_eks::types::ClusterStatus::Active)
    );
}

#[tokio::test]
async fn test_describe_cluster() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("desc-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_cluster()
        .name("desc-cluster")
        .send()
        .await
        .expect("describe_cluster should succeed");

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.name(), Some("desc-cluster"));
    assert_eq!(
        cluster.status(),
        Some(&aws_sdk_eks::types::ClusterStatus::Active)
    );
}

#[tokio::test]
async fn test_list_clusters() {
    let client = make_eks_client().await;

    for name in ["cluster-a", "cluster-b", "cluster-c"] {
        client
            .create_cluster()
            .name(name)
            .role_arn("arn:aws:iam::123456789012:role/eks-role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");

    assert_eq!(resp.clusters().len(), 3);
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_cluster()
        .name("del-cluster")
        .send()
        .await
        .expect("delete_cluster should succeed");

    let cluster = resp.cluster().unwrap();
    assert_eq!(
        cluster.status(),
        Some(&aws_sdk_eks::types::ClusterStatus::Deleting)
    );

    let result = client.describe_cluster().name("del-cluster").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_cluster_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("dup-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster()
        .name("dup-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_nodegroup() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_nodegroup()
        .cluster_name("ng-cluster")
        .nodegroup_name("test-nodegroup")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(3)
                .desired_size(2)
                .build(),
        )
        .send()
        .await
        .expect("create_nodegroup should succeed");

    let ng = resp.nodegroup().expect("should have nodegroup");
    assert_eq!(ng.nodegroup_name(), Some("test-nodegroup"));
    assert_eq!(
        ng.status(),
        Some(&aws_sdk_eks::types::NodegroupStatus::Active)
    );
}

#[tokio::test]
async fn test_describe_nodegroup() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-desc-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("ng-desc-cluster")
        .nodegroup_name("desc-nodegroup")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_nodegroup()
        .cluster_name("ng-desc-cluster")
        .nodegroup_name("desc-nodegroup")
        .send()
        .await
        .expect("describe_nodegroup should succeed");

    let ng = resp.nodegroup().unwrap();
    assert_eq!(ng.nodegroup_name(), Some("desc-nodegroup"));
    assert_eq!(ng.cluster_name(), Some("ng-desc-cluster"));
}

#[tokio::test]
async fn test_describe_nonexistent_cluster() {
    let client = make_eks_client().await;

    let result = client.describe_cluster().name("nonexistent").send().await;

    assert!(result.is_err());
}

// --- Fargate Profile tests ---

#[tokio::test]
async fn test_create_fargate_profile() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_fargate_profile()
        .cluster_name("fp-cluster")
        .fargate_profile_name("test-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .expect("create_fargate_profile should succeed");

    let fp = resp.fargate_profile().expect("should have fargate profile");
    assert_eq!(fp.fargate_profile_name(), Some("test-fp"));
    assert!(fp.fargate_profile_arn().is_some());
    assert_eq!(
        fp.status(),
        Some(&aws_sdk_eks::types::FargateProfileStatus::Active)
    );
}

#[tokio::test]
async fn test_describe_fargate_profile() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-desc-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_fargate_profile()
        .cluster_name("fp-desc-cluster")
        .fargate_profile_name("desc-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_fargate_profile()
        .cluster_name("fp-desc-cluster")
        .fargate_profile_name("desc-fp")
        .send()
        .await
        .expect("describe_fargate_profile should succeed");

    let fp = resp.fargate_profile().unwrap();
    assert_eq!(fp.fargate_profile_name(), Some("desc-fp"));
    assert_eq!(fp.cluster_name(), Some("fp-desc-cluster"));
}

#[tokio::test]
async fn test_delete_fargate_profile() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-del-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_fargate_profile()
        .cluster_name("fp-del-cluster")
        .fargate_profile_name("del-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_fargate_profile()
        .cluster_name("fp-del-cluster")
        .fargate_profile_name("del-fp")
        .send()
        .await
        .expect("delete_fargate_profile should succeed");

    let fp = resp.fargate_profile().unwrap();
    assert_eq!(
        fp.status(),
        Some(&aws_sdk_eks::types::FargateProfileStatus::Deleting)
    );

    // Verify it's gone
    let result = client
        .describe_fargate_profile()
        .cluster_name("fp-del-cluster")
        .fargate_profile_name("del-fp")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_fargate_profiles() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-list-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    for name in ["fp-a", "fp-b"] {
        client
            .create_fargate_profile()
            .cluster_name("fp-list-cluster")
            .fargate_profile_name(name)
            .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_fargate_profiles()
        .cluster_name("fp-list-cluster")
        .send()
        .await
        .expect("list_fargate_profiles should succeed");

    assert_eq!(resp.fargate_profile_names().len(), 2);
}

// --- Nodegroup additional tests ---

#[tokio::test]
async fn test_delete_nodegroup() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-del-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("ng-del-cluster")
        .nodegroup_name("del-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_nodegroup()
        .cluster_name("ng-del-cluster")
        .nodegroup_name("del-ng")
        .send()
        .await
        .expect("delete_nodegroup should succeed");

    let ng = resp.nodegroup().unwrap();
    assert_eq!(
        ng.status(),
        Some(&aws_sdk_eks::types::NodegroupStatus::Deleting)
    );

    // Verify it's gone
    let result = client
        .describe_nodegroup()
        .cluster_name("ng-del-cluster")
        .nodegroup_name("del-ng")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_nodegroups() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-list-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    for name in ["ng-a", "ng-b", "ng-c"] {
        client
            .create_nodegroup()
            .cluster_name("ng-list-cluster")
            .nodegroup_name(name)
            .node_role("arn:aws:iam::123456789012:role/node-role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_nodegroups()
        .cluster_name("ng-list-cluster")
        .send()
        .await
        .expect("list_nodegroups should succeed");

    assert_eq!(resp.nodegroups().len(), 3);
}

#[tokio::test]
async fn test_update_nodegroup_config() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-upd-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("ng-upd-cluster")
        .nodegroup_name("upd-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(3)
                .desired_size(2)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .update_nodegroup_config()
        .cluster_name("ng-upd-cluster")
        .nodegroup_name("upd-ng")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(2)
                .max_size(5)
                .desired_size(3)
                .build(),
        )
        .send()
        .await
        .expect("update_nodegroup_config should succeed");

    let update = resp.update().expect("should have update");
    assert!(update.id().is_some());

    // Verify the scaling config was updated
    let desc = client
        .describe_nodegroup()
        .cluster_name("ng-upd-cluster")
        .nodegroup_name("upd-ng")
        .send()
        .await
        .unwrap();
    let ng = desc.nodegroup().unwrap();
    let sc = ng.scaling_config().unwrap();
    assert_eq!(sc.min_size(), Some(2));
    assert_eq!(sc.max_size(), Some(5));
    assert_eq!(sc.desired_size(), Some(3));
}

// --- Tag tests ---

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("tag-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("untag-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster().unwrap().arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .tags("team", "platform")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert!(!tags.contains_key("env"));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));
}

// --- UpdateClusterConfig test ---

#[tokio::test]
async fn test_update_cluster_config() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("config-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_cluster_config()
        .name("config-cluster")
        .send()
        .await
        .expect("update_cluster_config should succeed");

    let update = resp.update().expect("should have update");
    assert!(update.id().is_some());
}

// ============================================================================
// Ported from moto: test_eks.py
// ============================================================================

// Ported from moto: test_eks.py::test_list_clusters_returns_empty_by_default
#[tokio::test]
async fn test_list_clusters_returns_empty_by_default() {
    let client = make_eks_client().await;

    let resp = client.list_clusters().send().await.unwrap();
    assert_eq!(resp.clusters().len(), 0);
}

// Ported from moto: test_eks.py::test_list_tags_returns_empty_by_default
#[tokio::test]
async fn test_list_tags_returns_empty_by_default() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("tags-empty-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster().unwrap().arn().unwrap().to_string();
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 0);
}

// Ported from moto: test_eks.py::test_list_tags_returns_all
#[tokio::test]
async fn test_list_tags_returns_all() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("tags-all-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster().unwrap().arn().unwrap().to_string();
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags.get("key1"), Some(&"val1".to_string()));
    assert_eq!(tags.get("key2"), Some(&"val2".to_string()));
}

// Ported from moto: test_eks.py::test_list_tags_returns_all_after_delete
#[tokio::test]
async fn test_list_tags_returns_all_after_delete() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("tags-del-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster().unwrap().arn().unwrap().to_string();
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get("key2"), Some(&"val2".to_string()));
}

// Ported from moto: test_eks.py::test_create_cluster_generates_valid_cluster_arn
#[tokio::test]
async fn test_create_cluster_generates_valid_cluster_arn() {
    let client = make_eks_client().await;

    let resp = client
        .create_cluster()
        .name("arn-test-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = resp.cluster().unwrap().arn().unwrap();
    assert!(arn.starts_with("arn:aws:eks:us-east-1:"));
    assert!(arn.contains(":cluster/arn-test-cluster"));
}

// Ported from moto: test_eks.py::test_create_cluster_generates_valid_cluster_endpoint
#[tokio::test]
async fn test_create_cluster_generates_valid_cluster_endpoint() {
    let client = make_eks_client().await;

    let resp = client
        .create_cluster()
        .name("endpoint-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let endpoint = resp.cluster().unwrap().endpoint().unwrap();
    assert!(endpoint.starts_with("https://"));
    assert!(endpoint.contains("us-east-1"));
}

// Ported from moto: test_eks.py::test_delete_cluster_removes_deleted_cluster
#[tokio::test]
async fn test_delete_cluster_removes_deleted_cluster() {
    let client = make_eks_client().await;

    for name in ["del-a", "del-b", "del-c"] {
        client
            .create_cluster()
            .name(name)
            .role_arn("arn:aws:iam::123456789012:role/eks-role")
            .send()
            .await
            .unwrap();
    }

    client.delete_cluster().name("del-b").send().await.unwrap();

    let resp = client.list_clusters().send().await.unwrap();
    let clusters = resp.clusters();
    assert_eq!(clusters.len(), 2);
    assert!(!clusters.contains(&"del-b".to_string()));
}

// Ported from moto: test_eks.py::test_delete_cluster_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_delete_nonexistent_cluster_fails() {
    let client = make_eks_client().await;

    let err = client
        .delete_cluster()
        .name("nonexistent-cluster")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_delete_cluster_throws_exception_when_nodegroups_exist
#[tokio::test]
async fn test_delete_cluster_fails_when_nodegroups_exist() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("busy-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("busy-cluster")
        .nodegroup_name("some-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_cluster()
        .name("busy-cluster")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException") || err_str.contains("InUse"),
        "Expected resource-in-use error, got: {err_str}"
    );

    // Cluster should still exist
    let resp = client.list_clusters().send().await.unwrap();
    assert_eq!(resp.clusters().len(), 1);
}

// Ported from moto: test_eks.py::test_list_nodegroups_returns_empty_by_default
#[tokio::test]
async fn test_list_nodegroups_returns_empty_by_default() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("empty-ng-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_nodegroups()
        .cluster_name("empty-ng-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.nodegroups().len(), 0);
}

// Ported from moto: test_eks.py::test_create_nodegroup_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_create_nodegroup_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .create_nodegroup()
        .cluster_name("nonexistent-cluster")
        .nodegroup_name("ng-1")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_create_nodegroup_throws_exception_when_nodegroup_already_exists
#[tokio::test]
async fn test_create_duplicate_nodegroup_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("dup-ng-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("dup-ng-cluster")
        .nodegroup_name("dup-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let err = client
        .create_nodegroup()
        .cluster_name("dup-ng-cluster")
        .nodegroup_name("dup-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException") || err_str.contains("InUse"),
        "Expected resource-in-use error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_create_nodegroup_generates_valid_nodegroup_arn
#[tokio::test]
async fn test_create_nodegroup_generates_valid_arn() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("arn-ng-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_nodegroup()
        .cluster_name("arn-ng-cluster")
        .nodegroup_name("arn-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let arn = resp.nodegroup().unwrap().nodegroup_arn().unwrap();
    assert!(arn.starts_with("arn:aws:eks:us-east-1:"));
    assert!(arn.contains(":nodegroup/arn-ng-cluster/arn-ng"));
}

// Ported from moto: test_eks.py::test_describe_nodegroup_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_describe_nodegroup_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .describe_nodegroup()
        .cluster_name("nonexistent-cluster")
        .nodegroup_name("ng-1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_describe_nodegroup_throws_exception_when_nodegroup_not_found
#[tokio::test]
async fn test_describe_nonexistent_nodegroup_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("desc-ng-missing-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let err = client
        .describe_nodegroup()
        .cluster_name("desc-ng-missing-cluster")
        .nodegroup_name("nonexistent-ng")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_delete_nodegroup_removes_deleted_nodegroup
#[tokio::test]
async fn test_delete_nodegroup_removes_from_list() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-ng-list-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    for name in ["ng-x", "ng-y", "ng-z"] {
        client
            .create_nodegroup()
            .cluster_name("del-ng-list-cluster")
            .nodegroup_name(name)
            .node_role("arn:aws:iam::123456789012:role/node-role")
            .send()
            .await
            .unwrap();
    }

    client
        .delete_nodegroup()
        .cluster_name("del-ng-list-cluster")
        .nodegroup_name("ng-y")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_nodegroups()
        .cluster_name("del-ng-list-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.nodegroups().len(), 2);
    assert!(!resp.nodegroups().contains(&"ng-y".to_string()));
}

// Ported from moto: test_eks.py::test_delete_nodegroup_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_delete_nodegroup_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .delete_nodegroup()
        .cluster_name("nonexistent-cluster")
        .nodegroup_name("ng-1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_delete_nodegroup_throws_exception_when_nodegroup_not_found
#[tokio::test]
async fn test_delete_nonexistent_nodegroup_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-ng-missing-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_nodegroup()
        .cluster_name("del-ng-missing-cluster")
        .nodegroup_name("nonexistent-ng")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_create_fargate_profile_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_create_fargate_profile_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .create_fargate_profile()
        .cluster_name("nonexistent-cluster")
        .fargate_profile_name("fp-1")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_create_fargate_profile_throws_exception_when_fargate_profile_already_exists
#[tokio::test]
async fn test_create_duplicate_fargate_profile_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("dup-fp-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_fargate_profile()
        .cluster_name("dup-fp-cluster")
        .fargate_profile_name("dup-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .create_fargate_profile()
        .cluster_name("dup-fp-cluster")
        .fargate_profile_name("dup-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException") || err_str.contains("InUse"),
        "Expected resource-in-use error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_create_fargate_profile_generates_valid_profile_arn
#[tokio::test]
async fn test_create_fargate_profile_generates_valid_arn() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("arn-fp-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_fargate_profile()
        .cluster_name("arn-fp-cluster")
        .fargate_profile_name("arn-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let arn = resp
        .fargate_profile()
        .unwrap()
        .fargate_profile_arn()
        .unwrap();
    assert!(arn.starts_with("arn:aws:eks:us-east-1:"));
    assert!(arn.contains(":fargateprofile/arn-fp-cluster/arn-fp/"));
}

// Ported from moto: test_eks.py::test_describe_fargate_profile_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_describe_fargate_profile_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .describe_fargate_profile()
        .cluster_name("nonexistent-cluster")
        .fargate_profile_name("fp-1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_describe_fargate_profile_throws_exception_when_profile_not_found
#[tokio::test]
async fn test_describe_nonexistent_fargate_profile_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("desc-fp-missing-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let err = client
        .describe_fargate_profile()
        .cluster_name("desc-fp-missing-cluster")
        .fargate_profile_name("nonexistent-fp")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_delete_fargate_profile_throws_exception_when_cluster_not_found
#[tokio::test]
async fn test_delete_fargate_profile_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .delete_fargate_profile()
        .cluster_name("nonexistent-cluster")
        .fargate_profile_name("fp-1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_delete_fargate_profile_throws_exception_when_fargate_profile_not_found
#[tokio::test]
async fn test_delete_nonexistent_fargate_profile_fails() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-fp-missing-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_fargate_profile()
        .cluster_name("del-fp-missing-cluster")
        .fargate_profile_name("nonexistent-fp")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_delete_fargate_profile_removes_deleted_fargate_profile
#[tokio::test]
async fn test_delete_fargate_profile_removes_from_list() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-fp-list-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    for name in ["fp-x", "fp-y", "fp-z"] {
        client
            .create_fargate_profile()
            .cluster_name("del-fp-list-cluster")
            .fargate_profile_name(name)
            .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
            .selectors(
                aws_sdk_eks::types::FargateProfileSelector::builder()
                    .namespace("default")
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    client
        .delete_fargate_profile()
        .cluster_name("del-fp-list-cluster")
        .fargate_profile_name("fp-y")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_fargate_profiles()
        .cluster_name("del-fp-list-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.fargate_profile_names().len(), 2);
    assert!(!resp.fargate_profile_names().contains(&"fp-y".to_string()));
}

// Ported from moto: test_eks.py::test_list_fargate_profile_returns_empty_by_default
#[tokio::test]
async fn test_list_fargate_profiles_returns_empty_by_default() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("empty-fp-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_fargate_profiles()
        .cluster_name("empty-fp-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.fargate_profile_names().len(), 0);
}

// Ported from moto: test_eks.py::test_update_cluster_config_not_found
#[tokio::test]
async fn test_update_cluster_config_not_found() {
    let client = make_eks_client().await;

    let err = client
        .update_cluster_config()
        .name("nonexistent-cluster")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_update_nodegroup_config_labels
#[tokio::test]
async fn test_update_nodegroup_config_labels() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("label-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("label-cluster")
        .nodegroup_name("label-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    // Add labels
    let resp = client
        .update_nodegroup_config()
        .cluster_name("label-cluster")
        .nodegroup_name("label-ng")
        .labels(
            aws_sdk_eks::types::UpdateLabelsPayload::builder()
                .add_or_update_labels("env", "test")
                .add_or_update_labels("team", "platform")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let update = resp.update().unwrap();
    assert_eq!(update.status().unwrap().as_str(), "Successful");

    // Verify labels
    let ng = client
        .describe_nodegroup()
        .cluster_name("label-cluster")
        .nodegroup_name("label-ng")
        .send()
        .await
        .unwrap();
    let labels = ng.nodegroup().unwrap().labels().unwrap();
    assert_eq!(labels.get("env"), Some(&"test".to_string()));
    assert_eq!(labels.get("team"), Some(&"platform".to_string()));

    // Remove a label
    client
        .update_nodegroup_config()
        .cluster_name("label-cluster")
        .nodegroup_name("label-ng")
        .labels(
            aws_sdk_eks::types::UpdateLabelsPayload::builder()
                .remove_labels("env")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ng = client
        .describe_nodegroup()
        .cluster_name("label-cluster")
        .nodegroup_name("label-ng")
        .send()
        .await
        .unwrap();
    let labels = ng.nodegroup().unwrap().labels().unwrap();
    assert!(!labels.contains_key("env"));
    assert_eq!(labels.get("team"), Some(&"platform".to_string()));
}

// Ported from moto: test_eks.py::test_update_nodegroup_config_taints
#[tokio::test]
async fn test_update_nodegroup_config_taints() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("taint-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("taint-cluster")
        .nodegroup_name("taint-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    // Add taints
    let resp = client
        .update_nodegroup_config()
        .cluster_name("taint-cluster")
        .nodegroup_name("taint-ng")
        .taints(
            aws_sdk_eks::types::UpdateTaintsPayload::builder()
                .add_or_update_taints(
                    aws_sdk_eks::types::Taint::builder()
                        .key("dedicated")
                        .value("gpu")
                        .effect(aws_sdk_eks::types::TaintEffect::NoSchedule)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.update().unwrap().status().unwrap().as_str(),
        "Successful"
    );

    let ng = client
        .describe_nodegroup()
        .cluster_name("taint-cluster")
        .nodegroup_name("taint-ng")
        .send()
        .await
        .unwrap();
    let taints = ng.nodegroup().unwrap().taints();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].key(), Some("dedicated"));
    assert_eq!(taints[0].value(), Some("gpu"));

    // Remove taints
    client
        .update_nodegroup_config()
        .cluster_name("taint-cluster")
        .nodegroup_name("taint-ng")
        .taints(
            aws_sdk_eks::types::UpdateTaintsPayload::builder()
                .remove_taints(
                    aws_sdk_eks::types::Taint::builder()
                        .key("dedicated")
                        .effect(aws_sdk_eks::types::TaintEffect::NoSchedule)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ng = client
        .describe_nodegroup()
        .cluster_name("taint-cluster")
        .nodegroup_name("taint-ng")
        .send()
        .await
        .unwrap();
    let taints = ng.nodegroup().unwrap().taints();
    assert_eq!(taints.len(), 0);
}

// Ported from moto: test_eks.py::test_update_nodegroup_config_scaling
#[tokio::test]
async fn test_update_nodegroup_config_scaling() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("scale-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("scale-cluster")
        .nodegroup_name("scale-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_nodegroup_config()
        .cluster_name("scale-cluster")
        .nodegroup_name("scale-ng")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(5)
                .desired_size(3)
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.update().unwrap().status().unwrap().as_str(),
        "Successful"
    );

    let ng = client
        .describe_nodegroup()
        .cluster_name("scale-cluster")
        .nodegroup_name("scale-ng")
        .send()
        .await
        .unwrap();
    let sc = ng.nodegroup().unwrap().scaling_config().unwrap();
    assert_eq!(sc.min_size(), Some(1));
    assert_eq!(sc.max_size(), Some(5));
    assert_eq!(sc.desired_size(), Some(3));
}

// Ported from moto: test_eks.py::test_update_nodegroup_config_cluster_not_found
#[tokio::test]
async fn test_update_nodegroup_config_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .update_nodegroup_config()
        .cluster_name("nonexistent-cluster")
        .nodegroup_name("ng-1")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(3)
                .desired_size(2)
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py::test_update_nodegroup_config_nodegroup_not_found
#[tokio::test]
async fn test_update_nodegroup_config_nodegroup_not_found() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("upd-ng-missing-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let err = client
        .update_nodegroup_config()
        .cluster_name("upd-ng-missing-cluster")
        .nodegroup_name("nonexistent-ng")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(3)
                .desired_size(2)
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_eks.py - cluster lifecycle with tags on create
#[tokio::test]
async fn test_create_cluster_with_tags() {
    let client = make_eks_client().await;

    let resp = client
        .create_cluster()
        .name("tagged-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .tags("env", "production")
        .tags("team", "platform")
        .send()
        .await
        .unwrap();

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.name(), Some("tagged-cluster"));
    let arn = cluster.arn().unwrap().to_string();

    // Tags may not round-trip through the cluster object due to SDK deserialization quirks,
    // so verify via list_tags_for_resource which is the canonical way to read tags
    let tag_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let listed_tags = tag_resp.tags().unwrap();
    assert_eq!(listed_tags.get("env"), Some(&"production".to_string()));
    assert_eq!(listed_tags.get("team"), Some(&"platform".to_string()));
}

// Ported from moto: test_eks.py - cluster lifecycle with version
#[tokio::test]
async fn test_create_cluster_with_version() {
    let client = make_eks_client().await;

    let resp = client
        .create_cluster()
        .name("versioned-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .version("1.29")
        .send()
        .await
        .unwrap();

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.version(), Some("1.29"));

    // Verify via describe
    let desc = client
        .describe_cluster()
        .name("versioned-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.cluster().unwrap().version(), Some("1.29"));
}

// Ported from moto: test_eks.py - cluster lifecycle with default version
#[tokio::test]
async fn test_create_cluster_default_version() {
    let client = make_eks_client().await;

    let resp = client
        .create_cluster()
        .name("default-ver-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let cluster = resp.cluster().unwrap();
    // Should have some version set (default)
    assert!(cluster.version().is_some());
}

// Ported from moto: test_eks.py - detailed fargate profile with selectors
#[tokio::test]
async fn test_fargate_profile_with_selectors_and_labels() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-sel-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_fargate_profile()
        .cluster_name("fp-sel-cluster")
        .fargate_profile_name("sel-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("kube-system")
                .labels("app", "coredns")
                .build(),
        )
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let fp = resp.fargate_profile().unwrap();
    assert_eq!(fp.fargate_profile_name(), Some("sel-fp"));
    assert_eq!(fp.cluster_name(), Some("fp-sel-cluster"));
    assert_eq!(
        fp.pod_execution_role_arn(),
        Some("arn:aws:iam::123456789012:role/pod-role")
    );

    let selectors = fp.selectors();
    assert_eq!(selectors.len(), 2);
    assert_eq!(selectors[0].namespace(), Some("kube-system"));
    let labels = selectors[0].labels().unwrap();
    assert_eq!(labels.get("app"), Some(&"coredns".to_string()));
    assert_eq!(selectors[1].namespace(), Some("default"));
}

// Ported from moto: test_eks.py - detailed cluster lifecycle
#[tokio::test]
async fn test_cluster_full_lifecycle() {
    let client = make_eks_client().await;

    // Create
    let create_resp = client
        .create_cluster()
        .name("lifecycle-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .version("1.28")
        .send()
        .await
        .unwrap();

    let cluster = create_resp.cluster().unwrap();
    assert_eq!(cluster.name(), Some("lifecycle-cluster"));
    assert_eq!(
        cluster.role_arn(),
        Some("arn:aws:iam::123456789012:role/eks-role")
    );
    assert_eq!(
        cluster.status(),
        Some(&aws_sdk_eks::types::ClusterStatus::Active)
    );
    assert_eq!(cluster.version(), Some("1.28"));
    assert!(cluster.arn().is_some());
    assert!(cluster.endpoint().is_some());
    assert!(cluster.created_at().is_some());

    // Describe
    let desc_resp = client
        .describe_cluster()
        .name("lifecycle-cluster")
        .send()
        .await
        .unwrap();
    let desc_cluster = desc_resp.cluster().unwrap();
    assert_eq!(desc_cluster.name(), Some("lifecycle-cluster"));
    assert_eq!(desc_cluster.arn(), cluster.arn());

    // List
    let list_resp = client.list_clusters().send().await.unwrap();
    assert!(
        list_resp
            .clusters()
            .contains(&"lifecycle-cluster".to_string())
    );

    // Delete
    let del_resp = client
        .delete_cluster()
        .name("lifecycle-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(
        del_resp.cluster().unwrap().status(),
        Some(&aws_sdk_eks::types::ClusterStatus::Deleting)
    );

    // Verify gone
    let list_after = client.list_clusters().send().await.unwrap();
    assert!(
        !list_after
            .clusters()
            .contains(&"lifecycle-cluster".to_string())
    );
}

// Ported from moto: test_eks.py - delete cluster fails when fargate profiles exist
#[tokio::test]
async fn test_delete_cluster_fails_when_fargate_profiles_exist() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-busy-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_fargate_profile()
        .cluster_name("fp-busy-cluster")
        .fargate_profile_name("blocking-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .delete_cluster()
        .name("fp-busy-cluster")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException") || err_str.contains("InUse"),
        "Expected resource-in-use error, got: {err_str}"
    );
}

// --- Additional edge case tests ---

// list_nodegroups on a nonexistent cluster returns error
#[tokio::test]
async fn test_list_nodegroups_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .list_nodegroups()
        .cluster_name("nonexistent-cluster")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// list_fargate_profiles on a nonexistent cluster returns error
#[tokio::test]
async fn test_list_fargate_profiles_cluster_not_found() {
    let client = make_eks_client().await;

    let err = client
        .list_fargate_profiles()
        .cluster_name("nonexistent-cluster")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("NotFound"),
        "Expected not-found error, got: {err_str}"
    );
}

// Tagging a nodegroup ARN (tag operations work on nodegroup ARNs, not just cluster ARNs)
#[tokio::test]
async fn test_tag_nodegroup_arn() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-tag-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let ng_resp = client
        .create_nodegroup()
        .cluster_name("ng-tag-cluster")
        .nodegroup_name("ng-tag")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let ng_arn = ng_resp
        .nodegroup()
        .unwrap()
        .nodegroup_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&ng_arn)
        .tags("purpose", "testing")
        .send()
        .await
        .expect("tag_resource on nodegroup ARN should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&ng_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("purpose"), Some(&"testing".to_string()));
}

// Tagging a fargate profile ARN works via the tags API
#[tokio::test]
async fn test_tag_fargate_profile_arn() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("fp-tag-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let fp_resp = client
        .create_fargate_profile()
        .cluster_name("fp-tag-cluster")
        .fargate_profile_name("fp-tag")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let fp_arn = fp_resp
        .fargate_profile()
        .unwrap()
        .fargate_profile_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&fp_arn)
        .tags("env", "staging")
        .send()
        .await
        .expect("tag_resource on fargate profile ARN should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&fp_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"staging".to_string()));
}

// Untagging a key that does not exist is idempotent (no error)
#[tokio::test]
async fn test_untag_nonexistent_key_is_idempotent() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("untag-idempotent-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster().unwrap().arn().unwrap().to_string();

    // Untagging a key that was never added should not fail
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("does-not-exist")
        .send()
        .await
        .expect("untag_resource with nonexistent key should succeed (idempotent)");

    // Tags should still be empty
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().unwrap().len(), 0);
}

// Nodegroup default scaling config is applied when not specified
#[tokio::test]
async fn test_create_nodegroup_default_scaling_config() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("default-scale-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("default-scale-cluster")
        .nodegroup_name("default-scale-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_nodegroup()
        .cluster_name("default-scale-cluster")
        .nodegroup_name("default-scale-ng")
        .send()
        .await
        .unwrap();

    let sc = resp.nodegroup().unwrap().scaling_config().unwrap();
    // Default min_size=1, max_size=2, desired_size=2 per state.rs
    assert_eq!(sc.min_size(), Some(1));
    assert_eq!(sc.max_size(), Some(2));
    assert_eq!(sc.desired_size(), Some(2));
}

// Nodegroup node_role is preserved in describe response
#[tokio::test]
async fn test_nodegroup_node_role_preserved() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("node-role-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("node-role-cluster")
        .nodegroup_name("node-role-ng")
        .node_role("arn:aws:iam::123456789012:role/my-special-node-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_nodegroup()
        .cluster_name("node-role-cluster")
        .nodegroup_name("node-role-ng")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.nodegroup().unwrap().node_role(),
        Some("arn:aws:iam::123456789012:role/my-special-node-role")
    );
}

// Nodegroup state is isolated per cluster (same nodegroup name in two clusters is allowed)
#[tokio::test]
async fn test_nodegroups_isolated_per_cluster() {
    let client = make_eks_client().await;

    for cluster in ["iso-cluster-1", "iso-cluster-2"] {
        client
            .create_cluster()
            .name(cluster)
            .role_arn("arn:aws:iam::123456789012:role/eks-role")
            .send()
            .await
            .unwrap();

        client
            .create_nodegroup()
            .cluster_name(cluster)
            .nodegroup_name("shared-ng-name")
            .node_role("arn:aws:iam::123456789012:role/node-role")
            .send()
            .await
            .unwrap();
    }

    // Delete nodegroup from cluster-1 should not affect cluster-2
    client
        .delete_nodegroup()
        .cluster_name("iso-cluster-1")
        .nodegroup_name("shared-ng-name")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_nodegroups()
        .cluster_name("iso-cluster-2")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.nodegroups().len(), 1);
    assert!(resp.nodegroups().contains(&"shared-ng-name".to_string()));
}

// Delete cluster succeeds after all nodegroups have been removed
#[tokio::test]
async fn test_delete_cluster_succeeds_after_nodegroup_removed() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("cleanup-ng-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("cleanup-ng-cluster")
        .nodegroup_name("blocking-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    // Deletion should be blocked
    let err = client
        .delete_cluster()
        .name("cleanup-ng-cluster")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException") || err_str.contains("InUse"),
        "Expected resource-in-use error, got: {err_str}"
    );

    // Now remove the nodegroup
    client
        .delete_nodegroup()
        .cluster_name("cleanup-ng-cluster")
        .nodegroup_name("blocking-ng")
        .send()
        .await
        .unwrap();

    // Cluster deletion should now succeed
    client
        .delete_cluster()
        .name("cleanup-ng-cluster")
        .send()
        .await
        .expect("delete_cluster should succeed after nodegroup is removed");

    let list_resp = client.list_clusters().send().await.unwrap();
    assert!(
        !list_resp
            .clusters()
            .contains(&"cleanup-ng-cluster".to_string())
    );
}

// Delete cluster succeeds after all fargate profiles have been removed
#[tokio::test]
async fn test_delete_cluster_succeeds_after_fargate_profile_removed() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("cleanup-fp-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_fargate_profile()
        .cluster_name("cleanup-fp-cluster")
        .fargate_profile_name("blocking-fp")
        .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
        .selectors(
            aws_sdk_eks::types::FargateProfileSelector::builder()
                .namespace("default")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Deletion should be blocked
    let err = client
        .delete_cluster()
        .name("cleanup-fp-cluster")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException") || err_str.contains("InUse"),
        "Expected resource-in-use error, got: {err_str}"
    );

    // Remove the fargate profile
    client
        .delete_fargate_profile()
        .cluster_name("cleanup-fp-cluster")
        .fargate_profile_name("blocking-fp")
        .send()
        .await
        .unwrap();

    // Cluster deletion should now succeed
    client
        .delete_cluster()
        .name("cleanup-fp-cluster")
        .send()
        .await
        .expect("delete_cluster should succeed after fargate profile is removed");

    let list_resp = client.list_clusters().send().await.unwrap();
    assert!(
        !list_resp
            .clusters()
            .contains(&"cleanup-fp-cluster".to_string())
    );
}

// Cluster role_arn is preserved and returned in describe
#[tokio::test]
async fn test_cluster_role_arn_preserved() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("role-arn-cluster")
        .role_arn("arn:aws:iam::123456789012:role/my-specific-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_cluster()
        .name("role-arn-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.cluster().unwrap().role_arn(),
        Some("arn:aws:iam::123456789012:role/my-specific-role")
    );
}

// Updating a taint updates its value in place when key and effect match
#[tokio::test]
async fn test_update_nodegroup_taint_value_in_place() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("taint-upd-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("taint-upd-cluster")
        .nodegroup_name("taint-upd-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    // Add initial taint
    client
        .update_nodegroup_config()
        .cluster_name("taint-upd-cluster")
        .nodegroup_name("taint-upd-ng")
        .taints(
            aws_sdk_eks::types::UpdateTaintsPayload::builder()
                .add_or_update_taints(
                    aws_sdk_eks::types::Taint::builder()
                        .key("workload")
                        .value("v1")
                        .effect(aws_sdk_eks::types::TaintEffect::NoSchedule)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Update same key+effect with a new value
    client
        .update_nodegroup_config()
        .cluster_name("taint-upd-cluster")
        .nodegroup_name("taint-upd-ng")
        .taints(
            aws_sdk_eks::types::UpdateTaintsPayload::builder()
                .add_or_update_taints(
                    aws_sdk_eks::types::Taint::builder()
                        .key("workload")
                        .value("v2")
                        .effect(aws_sdk_eks::types::TaintEffect::NoSchedule)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_nodegroup()
        .cluster_name("taint-upd-cluster")
        .nodegroup_name("taint-upd-ng")
        .send()
        .await
        .unwrap();

    let taints = resp.nodegroup().unwrap().taints();
    // Should still be exactly one taint (updated in place), with new value
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].key(), Some("workload"));
    assert_eq!(taints[0].value(), Some("v2"));
}

// Fargate profiles are isolated per cluster
#[tokio::test]
async fn test_fargate_profiles_isolated_per_cluster() {
    let client = make_eks_client().await;

    for cluster in ["fp-iso-cluster-1", "fp-iso-cluster-2"] {
        client
            .create_cluster()
            .name(cluster)
            .role_arn("arn:aws:iam::123456789012:role/eks-role")
            .send()
            .await
            .unwrap();

        client
            .create_fargate_profile()
            .cluster_name(cluster)
            .fargate_profile_name("shared-fp-name")
            .pod_execution_role_arn("arn:aws:iam::123456789012:role/pod-role")
            .selectors(
                aws_sdk_eks::types::FargateProfileSelector::builder()
                    .namespace("default")
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    // Delete fargate profile from cluster-1 should not affect cluster-2
    client
        .delete_fargate_profile()
        .cluster_name("fp-iso-cluster-1")
        .fargate_profile_name("shared-fp-name")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_fargate_profiles()
        .cluster_name("fp-iso-cluster-2")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.fargate_profile_names().len(), 1);
    assert!(
        resp.fargate_profile_names()
            .contains(&"shared-fp-name".to_string())
    );
}

// Update nodegroup with both labels and scaling config simultaneously
#[tokio::test]
async fn test_update_nodegroup_config_labels_and_scaling_simultaneously() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("combined-upd-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("combined-upd-cluster")
        .nodegroup_name("combined-upd-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(3)
                .desired_size(1)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .update_nodegroup_config()
        .cluster_name("combined-upd-cluster")
        .nodegroup_name("combined-upd-ng")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(2)
                .max_size(6)
                .desired_size(4)
                .build(),
        )
        .labels(
            aws_sdk_eks::types::UpdateLabelsPayload::builder()
                .add_or_update_labels("tier", "backend")
                .build(),
        )
        .send()
        .await
        .expect("update with both scaling and labels should succeed");

    let resp = client
        .describe_nodegroup()
        .cluster_name("combined-upd-cluster")
        .nodegroup_name("combined-upd-ng")
        .send()
        .await
        .unwrap();

    let ng = resp.nodegroup().unwrap();
    let sc = ng.scaling_config().unwrap();
    assert_eq!(sc.min_size(), Some(2));
    assert_eq!(sc.max_size(), Some(6));
    assert_eq!(sc.desired_size(), Some(4));
    let labels = ng.labels().unwrap();
    assert_eq!(labels.get("tier"), Some(&"backend".to_string()));
}

// ========== Addon tests ==========

#[tokio::test]
async fn test_create_addon() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("addon-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_addon()
        .cluster_name("addon-cluster")
        .addon_name("vpc-cni")
        .send()
        .await
        .expect("create_addon should succeed");

    let addon = resp.addon().expect("should have addon");
    assert_eq!(addon.addon_name(), Some("vpc-cni"));
    assert_eq!(addon.cluster_name(), Some("addon-cluster"));
    assert!(addon.addon_arn().is_some());
    assert_eq!(
        addon.status(),
        Some(&aws_sdk_eks::types::AddonStatus::Active)
    );
}

#[tokio::test]
async fn test_describe_addon() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("desc-addon-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_addon()
        .cluster_name("desc-addon-cluster")
        .addon_name("coredns")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_addon()
        .cluster_name("desc-addon-cluster")
        .addon_name("coredns")
        .send()
        .await
        .expect("describe_addon should succeed");

    let addon = resp.addon().expect("should have addon");
    assert_eq!(addon.addon_name(), Some("coredns"));
    assert_eq!(addon.cluster_name(), Some("desc-addon-cluster"));
}

#[tokio::test]
async fn test_delete_addon() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-addon-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_addon()
        .cluster_name("del-addon-cluster")
        .addon_name("kube-proxy")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_addon()
        .cluster_name("del-addon-cluster")
        .addon_name("kube-proxy")
        .send()
        .await
        .expect("delete_addon should succeed");

    let addon = resp.addon().expect("should have addon");
    assert_eq!(addon.addon_name(), Some("kube-proxy"));
    assert_eq!(
        addon.status(),
        Some(&aws_sdk_eks::types::AddonStatus::Deleting)
    );

    // Verify addon is gone
    let list = client
        .list_addons()
        .cluster_name("del-addon-cluster")
        .send()
        .await
        .unwrap();
    assert!(list.addons().is_empty());
}

#[tokio::test]
async fn test_update_addon() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("upd-addon-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_addon()
        .cluster_name("upd-addon-cluster")
        .addon_name("vpc-cni")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_addon()
        .cluster_name("upd-addon-cluster")
        .addon_name("vpc-cni")
        .service_account_role_arn("arn:aws:iam::123456789012:role/new-role")
        .send()
        .await
        .expect("update_addon should succeed");

    let update = resp.update().expect("should have update");
    assert_eq!(
        update.status(),
        Some(&aws_sdk_eks::types::UpdateStatus::Successful)
    );
}

#[tokio::test]
async fn test_list_addons() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("list-addon-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    // Empty initially
    let resp = client
        .list_addons()
        .cluster_name("list-addon-cluster")
        .send()
        .await
        .unwrap();
    assert!(resp.addons().is_empty());

    client
        .create_addon()
        .cluster_name("list-addon-cluster")
        .addon_name("vpc-cni")
        .send()
        .await
        .unwrap();

    client
        .create_addon()
        .cluster_name("list-addon-cluster")
        .addon_name("coredns")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_addons()
        .cluster_name("list-addon-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.addons().len(), 2);
}

// ========== Access entry tests ==========

#[tokio::test]
async fn test_create_access_entry() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("access-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_access_entry()
        .cluster_name("access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .expect("create_access_entry should succeed");

    let entry = resp.access_entry().expect("should have access entry");
    assert_eq!(
        entry.principal_arn(),
        Some("arn:aws:iam::123456789012:role/my-role")
    );
    assert_eq!(entry.cluster_name(), Some("access-cluster"));
    assert!(entry.access_entry_arn().is_some());
}

#[tokio::test]
async fn test_describe_access_entry() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("desc-access-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("desc-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_access_entry()
        .cluster_name("desc-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .expect("describe_access_entry should succeed");

    let entry = resp.access_entry().expect("should have access entry");
    assert_eq!(
        entry.principal_arn(),
        Some("arn:aws:iam::123456789012:role/my-role")
    );
}

#[tokio::test]
async fn test_delete_access_entry() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-access-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("del-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    client
        .delete_access_entry()
        .cluster_name("del-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .expect("delete_access_entry should succeed");

    // Verify it's gone
    let list = client
        .list_access_entries()
        .cluster_name("del-access-cluster")
        .send()
        .await
        .unwrap();
    assert!(list.access_entries().is_empty());
}

#[tokio::test]
async fn test_list_access_entries() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("list-access-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_access_entries()
        .cluster_name("list-access-cluster")
        .send()
        .await
        .unwrap();
    assert!(resp.access_entries().is_empty());

    client
        .create_access_entry()
        .cluster_name("list-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/role-a")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("list-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/role-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_access_entries()
        .cluster_name("list-access-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.access_entries().len(), 2);
}

#[tokio::test]
async fn test_update_access_entry() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("upd-access-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("upd-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .kubernetes_groups("system:masters")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_access_entry()
        .cluster_name("upd-access-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .username("new-user")
        .send()
        .await
        .expect("update_access_entry should succeed");

    let entry = resp.access_entry().expect("should have access entry");
    assert_eq!(entry.username(), Some("new-user"));
}

#[tokio::test]
async fn test_associate_access_policy() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("assoc-policy-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("assoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .associate_access_policy()
        .cluster_name("assoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .policy_arn("arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy")
        .access_scope(
            aws_sdk_eks::types::AccessScope::builder()
                .r#type(aws_sdk_eks::types::AccessScopeType::Cluster)
                .build(),
        )
        .send()
        .await
        .expect("associate_access_policy should succeed");

    assert_eq!(resp.cluster_name(), Some("assoc-policy-cluster"));
    let policy = resp.associated_access_policy().expect("should have policy");
    assert_eq!(
        policy.policy_arn(),
        Some("arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy")
    );
}

#[tokio::test]
async fn test_disassociate_access_policy() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("disassoc-policy-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("disassoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    client
        .associate_access_policy()
        .cluster_name("disassoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .policy_arn("arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy")
        .access_scope(
            aws_sdk_eks::types::AccessScope::builder()
                .r#type(aws_sdk_eks::types::AccessScopeType::Cluster)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .disassociate_access_policy()
        .cluster_name("disassoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .policy_arn("arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy")
        .send()
        .await
        .expect("disassociate_access_policy should succeed");

    // Verify it's gone
    let list = client
        .list_associated_access_policies()
        .cluster_name("disassoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();
    assert!(list.associated_access_policies().is_empty());
}

#[tokio::test]
async fn test_list_associated_access_policies() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("list-assoc-policy-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_access_entry()
        .cluster_name("list-assoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    // Initially empty
    let resp = client
        .list_associated_access_policies()
        .cluster_name("list-assoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();
    assert!(resp.associated_access_policies().is_empty());

    client
        .associate_access_policy()
        .cluster_name("list-assoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .policy_arn("arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy")
        .access_scope(
            aws_sdk_eks::types::AccessScope::builder()
                .r#type(aws_sdk_eks::types::AccessScopeType::Cluster)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_associated_access_policies()
        .cluster_name("list-assoc-policy-cluster")
        .principal_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.associated_access_policies().len(), 1);
}

// ========== Pod identity association tests ==========

#[tokio::test]
async fn test_create_pod_identity_association() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("pod-id-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_pod_identity_association()
        .cluster_name("pod-id-cluster")
        .namespace("default")
        .service_account("my-sa")
        .role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .expect("create_pod_identity_association should succeed");

    let assoc = resp.association().expect("should have association");
    assert_eq!(assoc.cluster_name(), Some("pod-id-cluster"));
    assert_eq!(assoc.namespace(), Some("default"));
    assert_eq!(assoc.service_account(), Some("my-sa"));
    assert!(assoc.association_id().is_some());
    assert!(assoc.association_arn().is_some());
}

#[tokio::test]
async fn test_describe_pod_identity_association() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("desc-pod-id-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_pod_identity_association()
        .cluster_name("desc-pod-id-cluster")
        .namespace("kube-system")
        .service_account("coredns")
        .role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .unwrap();

    let assoc_id = create_resp
        .association()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_pod_identity_association()
        .cluster_name("desc-pod-id-cluster")
        .association_id(&assoc_id)
        .send()
        .await
        .expect("describe_pod_identity_association should succeed");

    let assoc = resp.association().expect("should have association");
    assert_eq!(assoc.namespace(), Some("kube-system"));
    assert_eq!(assoc.service_account(), Some("coredns"));
}

#[tokio::test]
async fn test_delete_pod_identity_association() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("del-pod-id-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_pod_identity_association()
        .cluster_name("del-pod-id-cluster")
        .namespace("default")
        .service_account("my-sa")
        .role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .unwrap();

    let assoc_id = create_resp
        .association()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .delete_pod_identity_association()
        .cluster_name("del-pod-id-cluster")
        .association_id(&assoc_id)
        .send()
        .await
        .expect("delete_pod_identity_association should succeed");

    let assoc = resp.association().expect("should have association");
    assert_eq!(assoc.cluster_name(), Some("del-pod-id-cluster"));

    // Verify it's gone
    let list = client
        .list_pod_identity_associations()
        .cluster_name("del-pod-id-cluster")
        .send()
        .await
        .unwrap();
    assert!(list.associations().is_empty());
}

#[tokio::test]
async fn test_update_pod_identity_association() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("upd-pod-id-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_pod_identity_association()
        .cluster_name("upd-pod-id-cluster")
        .namespace("default")
        .service_account("my-sa")
        .role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .unwrap();

    let assoc_id = create_resp
        .association()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .update_pod_identity_association()
        .cluster_name("upd-pod-id-cluster")
        .association_id(&assoc_id)
        .role_arn("arn:aws:iam::123456789012:role/new-pod-role")
        .send()
        .await
        .expect("update_pod_identity_association should succeed");

    let assoc = resp.association().expect("should have association");
    assert_eq!(
        assoc.role_arn(),
        Some("arn:aws:iam::123456789012:role/new-pod-role")
    );
}

#[tokio::test]
async fn test_list_pod_identity_associations() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("list-pod-id-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pod_identity_associations()
        .cluster_name("list-pod-id-cluster")
        .send()
        .await
        .unwrap();
    assert!(resp.associations().is_empty());

    client
        .create_pod_identity_association()
        .cluster_name("list-pod-id-cluster")
        .namespace("default")
        .service_account("sa-1")
        .role_arn("arn:aws:iam::123456789012:role/pod-role")
        .send()
        .await
        .unwrap();

    client
        .create_pod_identity_association()
        .cluster_name("list-pod-id-cluster")
        .namespace("kube-system")
        .service_account("sa-2")
        .role_arn("arn:aws:iam::123456789012:role/pod-role-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pod_identity_associations()
        .cluster_name("list-pod-id-cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.associations().len(), 2);
}

// ========== EKS Anywhere subscription tests ==========

#[tokio::test]
async fn test_create_eks_anywhere_subscription() {
    let client = make_eks_client().await;

    let resp = client
        .create_eks_anywhere_subscription()
        .name("test-sub")
        .send()
        .await
        .expect("create_eks_anywhere_subscription should succeed");

    let sub = resp.subscription().expect("should have subscription");
    assert!(sub.id().is_some());
    assert!(sub.arn().is_some());
    assert_eq!(sub.status(), Some("ACTIVE"));
}

#[tokio::test]
async fn test_describe_eks_anywhere_subscription() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_eks_anywhere_subscription()
        .name("desc-sub")
        .send()
        .await
        .unwrap();

    let sub_id = create_resp
        .subscription()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_eks_anywhere_subscription()
        .id(&sub_id)
        .send()
        .await
        .expect("describe_eks_anywhere_subscription should succeed");

    let sub = resp.subscription().expect("should have subscription");
    assert!(sub.id().is_some());
    assert!(sub.arn().is_some());
}

#[tokio::test]
async fn test_delete_eks_anywhere_subscription() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_eks_anywhere_subscription()
        .name("del-sub")
        .send()
        .await
        .unwrap();

    let sub_id = create_resp
        .subscription()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .delete_eks_anywhere_subscription()
        .id(&sub_id)
        .send()
        .await
        .expect("delete_eks_anywhere_subscription should succeed");

    let sub = resp.subscription().expect("should have subscription");
    assert_eq!(sub.status(), Some("DELETING"));

    // Verify it's gone
    let list = client
        .list_eks_anywhere_subscriptions()
        .send()
        .await
        .unwrap();
    assert!(list.subscriptions().is_empty());
}

#[tokio::test]
async fn test_update_eks_anywhere_subscription() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_eks_anywhere_subscription()
        .name("upd-sub")
        .send()
        .await
        .unwrap();

    let sub_id = create_resp
        .subscription()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .update_eks_anywhere_subscription()
        .id(&sub_id)
        .auto_renew(true)
        .send()
        .await
        .expect("update_eks_anywhere_subscription should succeed");

    let sub = resp.subscription().expect("should have subscription");
    assert!(sub.auto_renew());
}

#[tokio::test]
async fn test_list_eks_anywhere_subscriptions() {
    let client = make_eks_client().await;

    let resp = client
        .list_eks_anywhere_subscriptions()
        .send()
        .await
        .unwrap();
    assert!(resp.subscriptions().is_empty());

    client
        .create_eks_anywhere_subscription()
        .name("sub-1")
        .send()
        .await
        .unwrap();

    client
        .create_eks_anywhere_subscription()
        .name("sub-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_eks_anywhere_subscriptions()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.subscriptions().len(), 2);
}

// ========== Miscellaneous operation tests ==========

#[tokio::test]
async fn test_update_cluster_version() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ver-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_cluster_version()
        .name("ver-cluster")
        .version("1.30")
        .send()
        .await
        .expect("update_cluster_version should succeed");

    let update = resp.update().expect("should have update");
    assert_eq!(
        update.status(),
        Some(&aws_sdk_eks::types::UpdateStatus::InProgress)
    );
}

#[tokio::test]
async fn test_update_nodegroup_version() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("ng-ver-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    client
        .create_nodegroup()
        .cluster_name("ng-ver-cluster")
        .nodegroup_name("ng-ver")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_nodegroup_version()
        .cluster_name("ng-ver-cluster")
        .nodegroup_name("ng-ver")
        .send()
        .await
        .expect("update_nodegroup_version should succeed");

    let update = resp.update().expect("should have update");
    assert_eq!(
        update.status(),
        Some(&aws_sdk_eks::types::UpdateStatus::InProgress)
    );
}

#[tokio::test]
async fn test_describe_addon_versions() {
    let client = make_eks_client().await;

    let resp = client
        .describe_addon_versions()
        .send()
        .await
        .expect("describe_addon_versions should succeed");

    // Handler returns static addon list
    let addons = resp.addons();
    assert_eq!(addons.len(), 3);
    let addon_names: Vec<_> = addons.iter().filter_map(|a| a.addon_name()).collect();
    assert!(addon_names.contains(&"vpc-cni"));
    assert!(addon_names.contains(&"coredns"));
    assert!(addon_names.contains(&"kube-proxy"));
}

#[tokio::test]
async fn test_list_updates() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("updates-cluster")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_updates()
        .name("updates-cluster")
        .send()
        .await
        .expect("list_updates should succeed");

    assert!(resp.update_ids().is_empty());
}

#[tokio::test]
async fn test_describe_cluster_versions() {
    let client = make_eks_client().await;

    let resp = client
        .describe_cluster_versions()
        .send()
        .await
        .expect("describe_cluster_versions should succeed");

    let versions = resp.cluster_versions();
    assert!(!versions.is_empty());
    // Should contain well-known versions
    let version_strings: Vec<_> = versions
        .iter()
        .filter_map(|v| v.cluster_version().map(String::from))
        .collect();
    assert!(version_strings.contains(&"1.28".to_string()));
    assert!(version_strings.contains(&"1.31".to_string()));
}

#[tokio::test]
async fn test_list_access_policies() {
    let client = make_eks_client().await;

    let resp = client
        .list_access_policies()
        .send()
        .await
        .expect("list_access_policies should succeed");

    let policies = resp.access_policies();
    assert!(!policies.is_empty());
    // Should contain known policies
    let names: Vec<_> = policies
        .iter()
        .filter_map(|p| p.name().map(String::from))
        .collect();
    assert!(names.contains(&"AmazonEKSAdminPolicy".to_string()));
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================
//
// The fix in `extract_path_and_query` ensures URI path extraction works when
// requests arrive from localhost (e.g. "https://127.0.0.1:{port}/clusters/...")
// instead of from "*.amazonaws.com".  In integration tests the SDK client
// always connects to localhost via MockAws, so every test above already
// exercises the patched code path implicitly.  The tests below make this
// coverage explicit by exercising endpoints at different path depths -- from
// shallow (/clusters) to deep (/clusters/{name}/node-groups/{ng}) -- to
// confirm that the URI-stripping logic correctly preserves the full path
// component for localhost URIs.

/// Shallow path: GET /clusters (1 segment) via localhost URI.
#[tokio::test]
async fn test_fix_terraform_e2e_list_clusters_via_localhost() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("e2e-shallow")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters via localhost should succeed");

    assert!(resp.clusters().contains(&"e2e-shallow".to_string()));
}

/// Medium path: GET /clusters/{name} (2 segments) via localhost URI.
#[tokio::test]
async fn test_fix_terraform_e2e_describe_cluster_via_localhost() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("e2e-medium")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_cluster()
        .name("e2e-medium")
        .send()
        .await
        .expect("describe_cluster via localhost should succeed");

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.name(), Some("e2e-medium"));
}

/// Deep path: POST /clusters/{name}/node-groups (3 segments) and
/// GET /clusters/{name}/node-groups/{ng} (4 segments) via localhost URI.
#[tokio::test]
async fn test_fix_terraform_e2e_nodegroup_deep_path_via_localhost() {
    let client = make_eks_client().await;

    client
        .create_cluster()
        .name("e2e-deep")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    // 3-segment path: POST /clusters/e2e-deep/node-groups
    client
        .create_nodegroup()
        .cluster_name("e2e-deep")
        .nodegroup_name("e2e-ng")
        .node_role("arn:aws:iam::123456789012:role/node-role")
        .scaling_config(
            aws_sdk_eks::types::NodegroupScalingConfig::builder()
                .min_size(1)
                .max_size(2)
                .desired_size(1)
                .build(),
        )
        .send()
        .await
        .expect("create_nodegroup via localhost should succeed");

    // 4-segment path: GET /clusters/e2e-deep/node-groups/e2e-ng
    let resp = client
        .describe_nodegroup()
        .cluster_name("e2e-deep")
        .nodegroup_name("e2e-ng")
        .send()
        .await
        .expect("describe_nodegroup via localhost should succeed");

    let ng = resp.nodegroup().unwrap();
    assert_eq!(ng.nodegroup_name(), Some("e2e-ng"));
    assert_eq!(ng.cluster_name(), Some("e2e-deep"));
}

/// Tag path: POST /tags/{resourceArn+} (2+ segments with encoded ARN) via
/// localhost URI.  This exercises both percent-decoding and the multi-segment
/// /tags/ prefix routing through the localhost URI extraction.
#[tokio::test]
async fn test_fix_terraform_e2e_tag_resource_via_localhost() {
    let client = make_eks_client().await;

    let create_resp = client
        .create_cluster()
        .name("e2e-tags")
        .role_arn("arn:aws:iam::123456789012:role/eks-role")
        .send()
        .await
        .unwrap();

    let cluster_arn = create_resp.cluster().unwrap().arn().unwrap().to_string();

    // POST /tags/{arn} -- the ARN contains colons and slashes that get
    // percent-encoded, exercising the full URI-parse + decode path.
    client
        .tag_resource()
        .resource_arn(&cluster_arn)
        .tags("e2e-key", "e2e-value")
        .send()
        .await
        .expect("tag_resource via localhost should succeed");

    // GET /tags/{arn}
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&cluster_arn)
        .send()
        .await
        .expect("list_tags_for_resource via localhost should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("e2e-key").map(String::as_str), Some("e2e-value"));
}

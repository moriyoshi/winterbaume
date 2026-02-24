//! End-to-end scenario tests for winterbaume-mediastore.
//!
//! Each test exercises a real use-case workflow chaining 3+ operations
//! and asserts business outcomes rather than per-API return shapes.

use aws_sdk_mediastore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediastore::MediaStoreService;

async fn make_client() -> aws_sdk_mediastore::Client {
    let mock = MockAws::builder()
        .with_service(MediaStoreService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediastore::config::Region::new("eu-west-1"))
        .load()
        .await;

    aws_sdk_mediastore::Client::new(&config)
}

/// Scenario: Container policy lifecycle
///
/// A user creates a container, attaches a container-access policy,
/// verifies the policy is readable, updates it, then deletes the container.
/// This exercises CreateContainer → PutContainerPolicy → GetContainerPolicy
/// → PutContainerPolicy (overwrite) → DeleteContainer.
#[tokio::test]
async fn test_container_policy_lifecycle() {
    let client = make_client().await;
    let name = "policy-lifecycle-container";
    let policy_v1 = r#"{"Version":"2012-10-17","Statement":[]}"#;
    let policy_v2 = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow"}]}"#;

    // Step 1: Create container
    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .expect("create_container should succeed");

    // Step 2: Attach initial policy
    client
        .put_container_policy()
        .container_name(name)
        .policy(policy_v1)
        .send()
        .await
        .expect("put_container_policy should succeed");

    // Step 3: Read back and verify
    let resp = client
        .get_container_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_container_policy should succeed");
    assert_eq!(
        resp.policy(),
        policy_v1,
        "policy should match initial value"
    );

    // Step 4: Overwrite with updated policy
    client
        .put_container_policy()
        .container_name(name)
        .policy(policy_v2)
        .send()
        .await
        .expect("overwrite put_container_policy should succeed");

    let resp2 = client
        .get_container_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_container_policy after overwrite should succeed");
    assert_eq!(
        resp2.policy(),
        policy_v2,
        "policy should reflect updated value"
    );

    // Step 5: Delete container — policy access should fail afterwards
    client
        .delete_container()
        .container_name(name)
        .send()
        .await
        .expect("delete_container should succeed");

    let err = client
        .get_container_policy()
        .container_name(name)
        .send()
        .await;
    assert!(
        err.is_err(),
        "get_container_policy on deleted container should fail"
    );
}

/// Scenario: Metric policy and lifecycle policy on the same container
///
/// A user provisions a container, configures both a lifecycle policy and a
/// metric policy with rules, then verifies both are independently readable
/// and describes the container to confirm ACTIVE status.
/// Operations: CreateContainer → PutLifecyclePolicy → PutMetricPolicy
/// → GetLifecyclePolicy → GetMetricPolicy → DescribeContainer.
#[tokio::test]
async fn test_container_with_multiple_policies() {
    let client = make_client().await;
    let name = "multi-policy-container";

    // Step 1: Create container
    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .expect("create_container should succeed");

    // Step 2: Attach lifecycle policy
    let lifecycle_policy = r#"{"rules":[{"definition":{"path":[{"prefix":"/"}],"days_since_create":[{"unit":"DAYS","value":30}]},"action":"EXPIRE"}]}"#;
    client
        .put_lifecycle_policy()
        .container_name(name)
        .lifecycle_policy(lifecycle_policy)
        .send()
        .await
        .expect("put_lifecycle_policy should succeed");

    // Step 3: Attach metric policy with rules
    let rule = aws_sdk_mediastore::types::MetricPolicyRule::builder()
        .object_group(".*\\.mp4")
        .object_group_name("mp4-videos")
        .build()
        .unwrap();
    client
        .put_metric_policy()
        .container_name(name)
        .metric_policy(
            aws_sdk_mediastore::types::MetricPolicy::builder()
                .container_level_metrics(aws_sdk_mediastore::types::ContainerLevelMetrics::Enabled)
                .metric_policy_rules(rule)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_metric_policy should succeed");

    // Step 4: Verify lifecycle policy readable
    let lp = client
        .get_lifecycle_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_lifecycle_policy should succeed");
    assert_eq!(
        lp.lifecycle_policy(),
        lifecycle_policy,
        "lifecycle policy should round-trip"
    );

    // Step 5: Verify metric policy and rules readable
    let mp_resp = client
        .get_metric_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_metric_policy should succeed");
    let mp = mp_resp.metric_policy().expect("should have metric policy");
    assert_eq!(
        mp.container_level_metrics(),
        &aws_sdk_mediastore::types::ContainerLevelMetrics::Enabled
    );
    let rules = mp.metric_policy_rules();
    assert_eq!(rules.len(), 1, "should have exactly one rule");
    assert_eq!(rules[0].object_group(), ".*\\.mp4");

    // Step 6: Describe container — should be ACTIVE after describe
    let desc = client
        .describe_container()
        .container_name(name)
        .send()
        .await
        .expect("describe_container should succeed");
    assert_eq!(
        desc.container().and_then(|c| c.status()),
        Some(&aws_sdk_mediastore::types::ContainerStatus::Active),
        "container should be ACTIVE after describe"
    );
}

/// Scenario: Tag management and resource cleanup
///
/// A user creates two containers with tags, lists both, verifies tags on
/// the first, then deletes the first and confirms only one remains.
/// Operations: CreateContainer × 2 → ListContainers → ListTagsForResource
/// → DeleteContainer → ListContainers.
#[tokio::test]
async fn test_tagged_containers_and_cleanup() {
    let client = make_client().await;
    let name_a = "tagged-container-a";
    let name_b = "tagged-container-b";

    // Step 1: Create two containers with distinct tags
    client
        .create_container()
        .container_name(name_a)
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create container A should succeed");

    client
        .create_container()
        .container_name(name_b)
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create container B should succeed");

    // Step 2: List — both should appear
    let containers = client
        .list_containers()
        .send()
        .await
        .expect("list_containers should succeed")
        .containers()
        .to_vec();
    assert_eq!(containers.len(), 2, "should have two containers");

    // Step 3: Verify tags on container A
    let tags_resp = client
        .list_tags_for_resource()
        .resource(name_a)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), Some("prod"));

    // Step 4: Delete container A
    client
        .delete_container()
        .container_name(name_a)
        .send()
        .await
        .expect("delete container A should succeed");

    // Step 5: Only container B remains
    let remaining = client
        .list_containers()
        .send()
        .await
        .expect("list_containers after delete should succeed")
        .containers()
        .to_vec();
    assert_eq!(
        remaining.len(),
        1,
        "should have one container after deletion"
    );
    assert_eq!(
        remaining[0].name(),
        Some(name_b),
        "remaining container should be B"
    );
}

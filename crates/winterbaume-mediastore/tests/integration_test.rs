use aws_sdk_mediastore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediastore::MediaStoreService;

async fn make_mediastore_client() -> aws_sdk_mediastore::Client {
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

#[tokio::test]
async fn test_create_container_succeeds() {
    let client = make_mediastore_client().await;

    let resp = client
        .create_container()
        .container_name("Awesome container!")
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_container should succeed");

    let container = resp.container().expect("should have container");
    assert_eq!(container.name(), Some("Awesome container!"));
    assert_eq!(
        container.arn(),
        Some("arn:aws:mediastore:us-east-1:123456789012:container/Awesome container!")
    );
    assert_eq!(
        container.status(),
        Some(&aws_sdk_mediastore::types::ContainerStatus::Creating)
    );
}

#[tokio::test]
async fn test_describe_container_succeeds() {
    let client = make_mediastore_client().await;
    let name = "Awesome container!";

    client
        .create_container()
        .container_name(name)
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_container()
        .container_name(name)
        .send()
        .await
        .expect("describe_container should succeed");

    let container = resp.container().expect("should have container");
    assert_eq!(
        container.arn(),
        Some(format!("arn:aws:mediastore:us-east-1:123456789012:container/{name}").as_str())
    );
    assert_eq!(container.name(), Some(name));
    assert_eq!(
        container.status(),
        Some(&aws_sdk_mediastore::types::ContainerStatus::Active)
    );
}

#[tokio::test]
async fn test_list_containers_succeeds() {
    let client = make_mediastore_client().await;
    let name = "Awesome container!";

    client
        .create_container()
        .container_name(name)
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let containers = client
        .list_containers()
        .send()
        .await
        .unwrap()
        .containers()
        .to_vec();
    assert_eq!(containers.len(), 1);

    client
        .create_container()
        .container_name(format!("{name}2"))
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let containers = client
        .list_containers()
        .send()
        .await
        .unwrap()
        .containers()
        .to_vec();
    assert_eq!(containers.len(), 2);
}

#[tokio::test]
async fn test_describe_container_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .describe_container()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_lifecycle_policy_succeeds() {
    let client = make_mediastore_client().await;
    let name = "container-name";

    client
        .create_container()
        .container_name(name)
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_lifecycle_policy()
        .container_name(name)
        .lifecycle_policy("lifecycle-policy")
        .send()
        .await
        .expect("put_lifecycle_policy should succeed");

    let resp = client
        .get_lifecycle_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_lifecycle_policy should succeed");

    assert_eq!(resp.lifecycle_policy(), "lifecycle-policy");
}

#[tokio::test]
async fn test_put_lifecycle_policy_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .put_lifecycle_policy()
        .container_name("name")
        .lifecycle_policy("policy")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_lifecycle_policy_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .get_lifecycle_policy()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_lifecycle_policy_raises_error_if_no_policy() {
    let client = make_mediastore_client().await;

    client
        .create_container()
        .container_name("container-name")
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .get_lifecycle_policy()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_container_policy_succeeds() {
    let client = make_mediastore_client().await;
    let name = "container-name";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .unwrap();

    client
        .put_container_policy()
        .container_name(name)
        .policy("container-policy")
        .send()
        .await
        .expect("put_container_policy should succeed");

    let resp = client
        .get_container_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_container_policy should succeed");

    assert_eq!(resp.policy(), "container-policy");
}

#[tokio::test]
async fn test_put_container_policy_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .put_container_policy()
        .container_name("name")
        .policy("policy")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_container_policy_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .get_container_policy()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_container_policy_raises_error_if_no_policy() {
    let client = make_mediastore_client().await;

    client
        .create_container()
        .container_name("container-name")
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .get_container_policy()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_metric_policy_succeeds() {
    let client = make_mediastore_client().await;
    let name = "container-name";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .unwrap();

    client
        .put_metric_policy()
        .container_name(name)
        .metric_policy(
            aws_sdk_mediastore::types::MetricPolicy::builder()
                .container_level_metrics(aws_sdk_mediastore::types::ContainerLevelMetrics::Enabled)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_metric_policy should succeed");

    let resp = client
        .get_metric_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_metric_policy should succeed");

    let metric_policy = resp.metric_policy().expect("should have metric policy");
    assert_eq!(
        metric_policy.container_level_metrics(),
        &aws_sdk_mediastore::types::ContainerLevelMetrics::Enabled
    );
}

#[tokio::test]
async fn test_put_metric_policy_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .put_metric_policy()
        .container_name("container-name")
        .metric_policy(
            aws_sdk_mediastore::types::MetricPolicy::builder()
                .container_level_metrics(aws_sdk_mediastore::types::ContainerLevelMetrics::Enabled)
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_metric_policy_raises_error_if_container_does_not_exist() {
    let client = make_mediastore_client().await;

    let result = client
        .get_metric_policy()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_metric_policy_raises_error_if_no_policy() {
    let client = make_mediastore_client().await;

    client
        .create_container()
        .container_name("container-name")
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .get_metric_policy()
        .container_name("container-name")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_container() {
    let client = make_mediastore_client().await;
    let container_name = "Awesome container!";

    client
        .create_container()
        .container_name(container_name)
        .send()
        .await
        .unwrap();

    client
        .delete_container()
        .container_name(container_name)
        .send()
        .await
        .expect("delete_container should succeed");

    let containers = client
        .list_containers()
        .send()
        .await
        .unwrap()
        .containers()
        .to_vec();
    assert!(containers.iter().all(|c| c.name() != Some(container_name)));
}

#[tokio::test]
async fn test_delete_container_raise_error_if_container_not_found() {
    let client = make_mediastore_client().await;

    client
        .create_container()
        .container_name("Awesome container!")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_container()
        .container_name("notAvailable")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_mediastore_client().await;
    let name = "Awesome container!";

    client
        .create_container()
        .container_name(name)
        .tags(
            aws_sdk_mediastore::types::Tag::builder()
                .key("customer")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource(name)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "customer");
}

#[tokio::test]
async fn test_list_tags_for_resource_return_error_for_unknown_resource() {
    let client = make_mediastore_client().await;

    let result = client
        .list_tags_for_resource()
        .resource("not_existing")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: MediaStore
// ============================================================================

#[tokio::test]
async fn test_create_container_duplicate_returns_error() {
    let client = make_mediastore_client().await;
    let name = "my-container-dup";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_container()
        .container_name(name)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ContainerInUseException"),
        "Expected ContainerInUseException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_container_sets_endpoint() {
    let client = make_mediastore_client().await;
    let name = "my-endpoint-container";

    let resp = client
        .create_container()
        .container_name(name)
        .send()
        .await
        .expect("create_container should succeed");

    let container = resp.container().expect("should have container");
    assert!(
        container.endpoint().is_some(),
        "endpoint should be set after create"
    );
    assert!(
        !container.endpoint().unwrap_or_default().is_empty(),
        "endpoint should be non-empty"
    );
}

#[tokio::test]
async fn test_create_container_sets_creation_time() {
    let client = make_mediastore_client().await;

    let resp = client
        .create_container()
        .container_name("my-time-container")
        .send()
        .await
        .expect("create_container should succeed");

    let container = resp.container().expect("should have container");
    assert!(
        container.creation_time().is_some(),
        "creation_time should be set"
    );
}

#[tokio::test]
async fn test_list_containers_empty() {
    let client = make_mediastore_client().await;

    let containers = client
        .list_containers()
        .send()
        .await
        .expect("list_containers should succeed")
        .containers()
        .to_vec();

    assert_eq!(
        containers.len(),
        0,
        "expected empty list when no containers exist"
    );
}

#[tokio::test]
async fn test_full_lifecycle_create_describe_delete() {
    let client = make_mediastore_client().await;
    let name = "lifecycle-container";

    // Create
    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .expect("create should succeed");

    // Describe succeeds
    let desc = client
        .describe_container()
        .container_name(name)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.container().and_then(|c| c.name()), Some(name));

    // Delete
    client
        .delete_container()
        .container_name(name)
        .send()
        .await
        .expect("delete should succeed");

    // Describe after delete should fail
    let result = client
        .describe_container()
        .container_name(name)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should return error");
}

#[tokio::test]
async fn test_list_tags_for_resource_no_tags() {
    let client = make_mediastore_client().await;
    let name = "notags-container";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource(name)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(resp.tags().len(), 0, "expected no tags");
}

#[tokio::test]
async fn test_lifecycle_policy_overwrite() {
    let client = make_mediastore_client().await;
    let name = "lifecycle-overwrite-container";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .unwrap();

    client
        .put_lifecycle_policy()
        .container_name(name)
        .lifecycle_policy("policy-v1")
        .send()
        .await
        .expect("first put_lifecycle_policy should succeed");

    client
        .put_lifecycle_policy()
        .container_name(name)
        .lifecycle_policy("policy-v2")
        .send()
        .await
        .expect("second put_lifecycle_policy should succeed");

    let resp = client
        .get_lifecycle_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_lifecycle_policy should succeed");

    assert_eq!(resp.lifecycle_policy(), "policy-v2");
}

#[tokio::test]
async fn test_container_policy_overwrite() {
    let client = make_mediastore_client().await;
    let name = "policy-overwrite-container";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .unwrap();

    client
        .put_container_policy()
        .container_name(name)
        .policy("policy-v1")
        .send()
        .await
        .expect("first put_container_policy should succeed");

    client
        .put_container_policy()
        .container_name(name)
        .policy("policy-v2")
        .send()
        .await
        .expect("second put_container_policy should succeed");

    let resp = client
        .get_container_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_container_policy should succeed");

    assert_eq!(resp.policy(), "policy-v2");
}

#[tokio::test]
async fn test_metric_policy_with_rules() {
    let client = make_mediastore_client().await;
    let name = "metric-rules-container";

    client
        .create_container()
        .container_name(name)
        .send()
        .await
        .unwrap();

    let rule = aws_sdk_mediastore::types::MetricPolicyRule::builder()
        .object_group(".*\\.m3u8")
        .object_group_name("hls-manifests")
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
        .expect("put_metric_policy with rules should succeed");

    let resp = client
        .get_metric_policy()
        .container_name(name)
        .send()
        .await
        .expect("get_metric_policy should succeed");

    let mp = resp.metric_policy().expect("should have metric policy");
    assert_eq!(
        mp.container_level_metrics(),
        &aws_sdk_mediastore::types::ContainerLevelMetrics::Enabled
    );
    let rules = mp.metric_policy_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].object_group(), ".*\\.m3u8");
    assert_eq!(rules[0].object_group_name(), "hls-manifests");
}

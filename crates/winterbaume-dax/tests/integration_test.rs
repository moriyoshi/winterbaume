use aws_sdk_dax::config::BehaviorVersion;
use aws_sdk_dax::error::ProvideErrorMetadata;
use winterbaume_core::MockAws;
use winterbaume_dax::DaxService;

async fn make_client() -> aws_sdk_dax::Client {
    let mock = MockAws::builder().with_service(DaxService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dax::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_dax::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("my-cluster")
        .node_type("dax.r5.large")
        .replication_factor(3)
        .iam_role_arn("arn:aws:iam::123456789012:role/DAXRole")
        .description("Test cluster")
        .send()
        .await
        .expect("create_cluster should succeed");

    let resp = client
        .describe_clusters()
        .cluster_names("my-cluster")
        .send()
        .await
        .expect("describe_clusters should succeed");

    let clusters = resp.clusters();
    assert_eq!(clusters.len(), 1);
    let cluster = &clusters[0];
    assert_eq!(cluster.cluster_name(), Some("my-cluster"));
    assert_eq!(cluster.description(), Some("Test cluster"));
    assert_eq!(cluster.node_type(), Some("dax.r5.large"));
    assert_eq!(cluster.status(), Some("available"));
}

#[tokio::test]
async fn test_describe_all_clusters() {
    let client = make_client().await;

    for name in ["cluster-a", "cluster-b"] {
        client
            .create_cluster()
            .cluster_name(name)
            .node_type("dax.r5.large")
            .replication_factor(1)
            .iam_role_arn("arn:aws:iam::123456789012:role/DAXRole")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");

    assert_eq!(resp.clusters().len(), 2);
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("delete-me")
        .node_type("dax.r5.large")
        .replication_factor(1)
        .iam_role_arn("arn:aws:iam::123456789012:role/DAXRole")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_cluster()
        .cluster_name("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    let cluster = resp.cluster().expect("should have cluster in response");
    assert_eq!(cluster.cluster_name(), Some("delete-me"));

    let result = client
        .describe_clusters()
        .cluster_names("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Ported from moto test_dax.py ---

const IAM_ROLE_ARN: &str =
    "arn:aws:iam::123456789012:role/aws-service-role/dax.amazonaws.com/AWSServiceRoleForDAX";

#[tokio::test]
async fn test_create_cluster_minimal_fields() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(cluster.cluster_name(), Some("daxcluster"));
    let expected_arn = "arn:aws:dax:us-east-1:123456789012:cache/daxcluster";
    assert_eq!(cluster.cluster_arn(), Some(expected_arn));
    assert_eq!(cluster.total_nodes(), Some(3));
    assert_eq!(cluster.node_type(), Some("dax.t3.small"));
    assert_eq!(cluster.iam_role_arn(), Some(IAM_ROLE_ARN));
    // SSE should be disabled by default
    let sse = cluster
        .sse_description()
        .expect("should have SSEDescription");
    assert_eq!(sse.status().map(|s| s.as_str()), Some("DISABLED"));
    // Endpoint should be present
    let endpoint = cluster
        .cluster_discovery_endpoint()
        .expect("should have endpoint");
    assert_eq!(endpoint.port(), 8111);
    // ParameterGroup
    let pg = cluster
        .parameter_group()
        .expect("should have parameter group");
    assert_eq!(pg.parameter_group_name(), Some("default.dax1.0"));
    // SecurityGroups - should have at least one
    assert!(!cluster.security_groups().is_empty());
    // SubnetGroup
    assert_eq!(cluster.subnet_group(), Some("default"));
}

#[tokio::test]
async fn test_create_cluster_with_description() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .description("my cluster")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(cluster.description(), Some("my cluster"));

    // Verify via describe
    let describe_resp = client
        .describe_clusters()
        .cluster_names("daxcluster")
        .send()
        .await
        .expect("describe_clusters should succeed");
    assert_eq!(
        describe_resp.clusters()[0].description(),
        Some("my cluster")
    );
}

#[tokio::test]
async fn test_create_cluster_with_sse_enabled() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .sse_specification(
            aws_sdk_dax::types::SseSpecification::builder()
                .enabled(true)
                .build()
                .unwrap(),
        )
        .cluster_endpoint_encryption_type(aws_sdk_dax::types::ClusterEndpointEncryptionType::Tls)
        .send()
        .await
        .expect("create_cluster with SSE should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    let sse = cluster
        .sse_description()
        .expect("should have SSEDescription");
    assert_eq!(sse.status().map(|s| s.as_str()), Some("ENABLED"));
    assert_eq!(
        cluster
            .cluster_endpoint_encryption_type()
            .map(|t| t.as_str()),
        Some("TLS")
    );
}

#[tokio::test]
async fn test_create_cluster_invalid_arn_missing_prefix() {
    let client = make_client().await;
    let result = client
        .create_cluster()
        .cluster_name("valid-name")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn("n/a")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert_eq!(err_msg, "ARNs must start with 'arn:': n/a");
}

#[tokio::test]
async fn test_create_cluster_invalid_name() {
    let client = make_client().await;
    // Names starting with digit are invalid
    let result = client
        .create_cluster()
        .cluster_name("1invalid")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn("arn:aws:iam::123456789012:role/apigatewayrole")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not a valid identifier"));
}

#[tokio::test]
async fn test_create_cluster_invalid_name_trailing_hyphen() {
    let client = make_client().await;
    let result = client
        .create_cluster()
        .cluster_name("invalid-")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn("arn:aws:iam::123456789012:role/apigatewayrole")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not a valid identifier"));
}

#[tokio::test]
async fn test_create_cluster_invalid_name_double_hyphen() {
    let client = make_client().await;
    let result = client
        .create_cluster()
        .cluster_name("in--valid")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn("arn:aws:iam::123456789012:role/apigatewayrole")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not a valid identifier"));
}

#[tokio::test]
async fn test_describe_clusters_invalid_name_fails() {
    let client = make_client().await;
    let result = client
        .describe_clusters()
        .cluster_names("1invalid")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not a valid identifier"));
}

#[tokio::test]
async fn test_delete_cluster_unknown() {
    let client = make_client().await;
    let result = client.delete_cluster().cluster_name("unknown").send().await;
    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert_eq!(err_msg, "Cluster not found.");
}

#[tokio::test]
async fn test_describe_cluster_unknown() {
    let client = make_client().await;
    let result = client
        .describe_clusters()
        .cluster_names("unknown")
        .send()
        .await;
    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert_eq!(err_msg, "Cluster unknown not found.");
}

#[tokio::test]
async fn test_list_tags_unknown() {
    let client = make_client().await;
    let result = client.list_tags().resource_name("unknown").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_tags() {
    let client = make_client().await;

    let create_resp = client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .tags(
            aws_sdk_dax::types::Tag::builder()
                .key("tag1")
                .value("value1")
                .build(),
        )
        .tags(
            aws_sdk_dax::types::Tag::builder()
                .key("tag2")
                .value("value2")
                .build(),
        )
        .tags(
            aws_sdk_dax::types::Tag::builder()
                .key("tag3")
                .value("value3")
                .build(),
        )
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = create_resp.cluster().expect("should have cluster");
    let cluster_arn = cluster.cluster_arn().unwrap();

    // List by name and by ARN
    for resource_name in ["daxcluster", cluster_arn] {
        let resp = client
            .list_tags()
            .resource_name(resource_name)
            .send()
            .await
            .expect("list_tags should succeed");

        let tags = resp.tags();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0].key(), Some("tag1"));
        assert_eq!(tags[0].value(), Some("value1"));
        assert_eq!(tags[1].key(), Some("tag2"));
        assert_eq!(tags[1].value(), Some("value2"));
        assert_eq!(tags[2].key(), Some("tag3"));
        assert_eq!(tags[2].value(), Some("value3"));
    }
}

#[tokio::test]
async fn test_increase_replication_factor_unknown() {
    let client = make_client().await;
    let result = client
        .increase_replication_factor()
        .cluster_name("unknown")
        .new_replication_factor(2)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_increase_replication_factor() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(2)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    let resp = client
        .increase_replication_factor()
        .cluster_name("daxcluster")
        .new_replication_factor(5)
        .send()
        .await
        .expect("increase_replication_factor should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(cluster.total_nodes(), Some(5));

    // Verify via describe
    let desc = client
        .describe_clusters()
        .cluster_names("daxcluster")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.clusters()[0].total_nodes(), Some(5));
}

#[tokio::test]
async fn test_decrease_replication_factor_unknown() {
    let client = make_client().await;
    let result = client
        .decrease_replication_factor()
        .cluster_name("unknown")
        .new_replication_factor(2)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_decrease_replication_factor() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(5)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    let resp = client
        .decrease_replication_factor()
        .cluster_name("daxcluster")
        .new_replication_factor(3)
        .send()
        .await
        .expect("decrease_replication_factor should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(cluster.total_nodes(), Some(3));

    // Verify via describe
    let desc = client
        .describe_clusters()
        .cluster_names("daxcluster")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.clusters()[0].total_nodes(), Some(3));
}

#[tokio::test]
async fn test_create_cluster_duplicate_name() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("first create_cluster should succeed");

    let result = client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("already exists"));
}

#[tokio::test]
async fn test_create_cluster_invalid_arn_missing_partition() {
    let client = make_client().await;
    // "arn:" prefix present but no partition segment
    let result = client
        .create_cluster()
        .cluster_name("valid-name")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn("arn:")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("partition") || err_msg.contains("colon") || err_msg.contains("ARN"));
}

#[tokio::test]
async fn test_create_cluster_invalid_arn_too_few_segments() {
    let client = make_client().await;
    // Has "arn:aws" but only 2 segments — missing vendor/region/account
    let result = client
        .create_cluster()
        .cluster_name("valid-name")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn("arn:aws")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("colon") || err_msg.contains("ARN") || err_msg.contains("vendor"));
}

#[tokio::test]
async fn test_decrease_replication_factor_equal_is_error() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    // Decreasing to the same factor should fail
    let result = client
        .decrease_replication_factor()
        .cluster_name("daxcluster")
        .new_replication_factor(3)
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("less than current"));
}

#[tokio::test]
async fn test_decrease_replication_factor_to_zero_is_error() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    let result = client
        .decrease_replication_factor()
        .cluster_name("daxcluster")
        .new_replication_factor(0)
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("at least 1") || err_msg.contains("less than"));
}

#[tokio::test]
async fn test_increase_replication_factor_equal_is_error() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    // Increasing to the same factor should fail
    let result = client
        .increase_replication_factor()
        .cluster_name("daxcluster")
        .new_replication_factor(3)
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("greater than current"));
}

#[tokio::test]
async fn test_list_tags_no_tags() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("daxcluster")
        .node_type("dax.t3.small")
        .replication_factor(3)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    let resp = client
        .list_tags()
        .resource_name("daxcluster")
        .send()
        .await
        .expect("list_tags should succeed on cluster with no tags");

    assert!(resp.tags().is_empty());
}

#[tokio::test]
async fn test_describe_clusters_second_name_not_found() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("cluster-one")
        .node_type("dax.t3.small")
        .replication_factor(1)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    // One exists, second does not — should fail
    let result = client
        .describe_clusters()
        .cluster_names("cluster-one")
        .cluster_names("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("nonexistent"));
}

#[tokio::test]
async fn test_tag_resource_returns_not_implemented() {
    let client = make_client().await;

    // TagResource is a stub — it should return an error indicating not implemented
    let result = client
        .tag_resource()
        .resource_name("daxcluster")
        .tags(
            aws_sdk_dax::types::Tag::builder()
                .key("k")
                .value("v")
                .build(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not yet implemented") || err_msg.contains("NotImplemented"));
}

#[tokio::test]
async fn test_untag_resource_returns_not_implemented() {
    let client = make_client().await;

    let result = client
        .untag_resource()
        .resource_name("daxcluster")
        .tag_keys("somekey")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not yet implemented") || err_msg.contains("NotImplemented"));
}

#[tokio::test]
async fn test_update_cluster_returns_not_implemented() {
    let client = make_client().await;

    let result = client
        .update_cluster()
        .cluster_name("daxcluster")
        .description("new description")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not yet implemented") || err_msg.contains("NotImplemented"));
}

#[tokio::test]
async fn test_cluster_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("my-test-cluster")
        .node_type("dax.t3.small")
        .replication_factor(1)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    let arn = cluster.cluster_arn().expect("should have ARN");
    assert!(
        arn.starts_with("arn:aws:dax:"),
        "ARN should start with arn:aws:dax: but got {arn}"
    );
    assert!(
        arn.ends_with(":cache/my-test-cluster"),
        "ARN should end with :cache/<name> but got {arn}"
    );
}

#[tokio::test]
async fn test_delete_cluster_twice_fails_second_time() {
    let client = make_client().await;

    client
        .create_cluster()
        .cluster_name("delete-twice")
        .node_type("dax.t3.small")
        .replication_factor(1)
        .iam_role_arn(IAM_ROLE_ARN)
        .send()
        .await
        .expect("create_cluster should succeed");

    client
        .delete_cluster()
        .cluster_name("delete-twice")
        .send()
        .await
        .expect("first delete should succeed");

    let result = client
        .delete_cluster()
        .cluster_name("delete-twice")
        .send()
        .await;

    assert!(result.is_err());
    let err_msg = result
        .unwrap_err()
        .into_service_error()
        .message()
        .unwrap_or("")
        .to_string();
    assert!(err_msg.contains("not found") || err_msg.contains("Cluster"));
}

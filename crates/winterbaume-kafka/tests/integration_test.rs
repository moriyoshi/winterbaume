use std::collections::HashMap;

use aws_sdk_kafka::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kafka::KafkaService;

async fn make_kafka_client() -> aws_sdk_kafka::Client {
    let mock = MockAws::builder().with_service(KafkaService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kafka::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_kafka::Client::new(&config)
}

fn make_provisioned_request() -> aws_sdk_kafka::types::ProvisionedRequest {
    aws_sdk_kafka::types::ProvisionedRequest::builder()
        .kafka_version("3.5.1")
        .number_of_broker_nodes(3)
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-12345")
                .build(),
        )
        .build()
}

#[tokio::test]
async fn test_create_and_describe_cluster_v2() {
    let client = make_kafka_client().await;

    let create_resp = client
        .create_cluster_v2()
        .cluster_name("test-cluster")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .expect("create_cluster_v2 should succeed");

    let cluster_arn = create_resp.cluster_arn().expect("should have cluster ARN");
    assert!(!cluster_arn.is_empty());
    assert_eq!(create_resp.cluster_name().unwrap(), "test-cluster");
    assert_eq!(create_resp.state().unwrap().as_str(), "CREATING");

    let describe_resp = client
        .describe_cluster_v2()
        .cluster_arn(cluster_arn)
        .send()
        .await
        .expect("describe_cluster_v2 should succeed");

    let cluster_info = describe_resp
        .cluster_info()
        .expect("should have cluster info");
    assert_eq!(cluster_info.cluster_name().unwrap(), "test-cluster");
    assert_eq!(cluster_info.state().unwrap().as_str(), "CREATING");
    assert_eq!(cluster_info.cluster_type().unwrap().as_str(), "PROVISIONED");
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = make_kafka_client().await;

    let create_resp = client
        .create_cluster_v2()
        .cluster_name("delete-me")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    let cluster_arn = create_resp.cluster_arn().unwrap();

    let delete_resp = client
        .delete_cluster()
        .cluster_arn(cluster_arn)
        .send()
        .await
        .expect("delete_cluster should succeed");

    assert_eq!(delete_resp.state().unwrap().as_str(), "DELETING");

    let describe_result = client
        .describe_cluster_v2()
        .cluster_arn(cluster_arn)
        .send()
        .await;
    assert!(
        describe_result.is_err(),
        "describe after delete should fail"
    );
}

#[tokio::test]
async fn test_list_clusters_v2() {
    let client = make_kafka_client().await;

    client
        .create_cluster_v2()
        .cluster_name("cluster-a")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    client
        .create_cluster_v2()
        .cluster_name("cluster-b")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_clusters_v2()
        .send()
        .await
        .expect("list_clusters_v2 should succeed");

    let clusters = list_resp.cluster_info_list();
    assert_eq!(clusters.len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_cluster_fails() {
    let client = make_kafka_client().await;

    client
        .create_cluster_v2()
        .cluster_name("dup-cluster")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    let result = client
        .create_cluster_v2()
        .cluster_name("dup-cluster")
        .provisioned(make_provisioned_request())
        .send()
        .await;
    assert!(result.is_err(), "duplicate cluster should fail");
}

#[tokio::test]
async fn test_describe_nonexistent_cluster_fails() {
    let client = make_kafka_client().await;

    let result = client
        .describe_cluster_v2()
        .cluster_arn("arn:aws:kafka:us-east-1:123456789012:cluster/nonexistent/fake-id")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent cluster should fail");
}

// ===== Moto parity tests =====

/// Parity with moto test_create_cluster_v2: serverless + provisioned, with tags.
#[tokio::test]
async fn test_create_cluster_v2_serverless_and_provisioned() {
    let client = make_kafka_client().await;

    // Create serverless cluster
    let s_resp = client
        .create_cluster_v2()
        .cluster_name("TestServerlessCluster")
        .serverless(
            aws_sdk_kafka::types::ServerlessRequest::builder()
                .vpc_configs(
                    aws_sdk_kafka::types::VpcConfig::builder()
                        .subnet_ids("subnet-0123456789abcdef0")
                        .security_group_ids("sg-0123456789abcdef0")
                        .build(),
                )
                .build(),
        )
        .tags("TestKey", "TestValue")
        .tags("TestKey2", "TestValue2")
        .send()
        .await
        .expect("create serverless cluster should succeed");

    assert!(s_resp.cluster_arn().unwrap().starts_with("arn:aws:kafka"));
    assert_eq!(s_resp.cluster_name().unwrap(), "TestServerlessCluster");
    assert_eq!(s_resp.state().unwrap().as_str(), "CREATING");

    // Create provisioned cluster
    let p_resp = client
        .create_cluster_v2()
        .cluster_name("TestProvisionedCluster")
        .provisioned(
            aws_sdk_kafka::types::ProvisionedRequest::builder()
                .broker_node_group_info(
                    aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                        .instance_type("kafka.m5.large")
                        .client_subnets("subnet-0123456789abcdef0")
                        .security_groups("sg-0123456789abcdef0")
                        .build(),
                )
                .kafka_version("2.8.1")
                .number_of_broker_nodes(3)
                .build(),
        )
        .tags("TestKey", "TestValue")
        .tags("TestKey2", "TestValue2")
        .send()
        .await
        .expect("create provisioned cluster should succeed");

    assert!(p_resp.cluster_arn().unwrap().starts_with("arn:aws:kafka"));
    assert_eq!(p_resp.cluster_name().unwrap(), "TestProvisionedCluster");
    assert_eq!(p_resp.state().unwrap().as_str(), "CREATING");

    // List clusters
    let list = client.list_clusters_v2().send().await.unwrap();
    assert_eq!(list.cluster_info_list().len(), 2);

    // Describe serverless
    let s_desc = client
        .describe_cluster_v2()
        .cluster_arn(s_resp.cluster_arn().unwrap())
        .send()
        .await
        .unwrap();
    let s_info = s_desc.cluster_info().unwrap();
    assert_eq!(s_info.cluster_name().unwrap(), "TestServerlessCluster");
    assert_eq!(s_info.state().unwrap().as_str(), "CREATING");
    assert_eq!(s_info.cluster_type().unwrap().as_str(), "SERVERLESS");

    let s_serverless = s_info.serverless().unwrap();
    let vpc_configs = s_serverless.vpc_configs();
    assert_eq!(vpc_configs.len(), 1);
    assert_eq!(vpc_configs[0].subnet_ids(), &["subnet-0123456789abcdef0"]);
    assert_eq!(
        vpc_configs[0].security_group_ids(),
        &["sg-0123456789abcdef0"]
    );

    let s_tags = s_info.tags().unwrap();
    assert_eq!(s_tags.get("TestKey").unwrap(), "TestValue");
    assert_eq!(s_tags.get("TestKey2").unwrap(), "TestValue2");

    // Describe provisioned
    let p_desc = client
        .describe_cluster_v2()
        .cluster_arn(p_resp.cluster_arn().unwrap())
        .send()
        .await
        .unwrap();
    let p_info = p_desc.cluster_info().unwrap();
    assert_eq!(p_info.cluster_name().unwrap(), "TestProvisionedCluster");
    assert_eq!(p_info.state().unwrap().as_str(), "CREATING");
    assert_eq!(p_info.cluster_type().unwrap().as_str(), "PROVISIONED");

    let p_prov = p_info.provisioned().unwrap();
    assert_eq!(
        p_prov.broker_node_group_info().unwrap().instance_type(),
        Some("kafka.m5.large")
    );
}

/// Parity with moto test_create_cluster (V1 API).
#[tokio::test]
async fn test_create_cluster_v1() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("TestCluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-0123456789abcdef0")
                .security_groups("sg-0123456789abcdef0")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .tags("TestKey", "TestValue")
        .tags("TestKey2", "TestValue2")
        .send()
        .await
        .expect("create_cluster v1 should succeed");

    assert!(resp.cluster_arn().unwrap().starts_with("arn:aws:kafka"));
    assert_eq!(resp.cluster_name().unwrap(), "TestCluster");
    assert_eq!(resp.state().unwrap().as_str(), "CREATING");

    // List clusters (v1)
    let list = client.list_clusters().send().await.unwrap();
    assert_eq!(list.cluster_info_list().len(), 1);
    assert_eq!(
        list.cluster_info_list()[0].cluster_name().unwrap(),
        "TestCluster"
    );

    // Describe cluster (v1)
    let desc = client
        .describe_cluster()
        .cluster_arn(resp.cluster_arn().unwrap())
        .send()
        .await
        .unwrap();
    let info = desc.cluster_info().unwrap();
    assert_eq!(info.cluster_name().unwrap(), "TestCluster");
    assert_eq!(info.state().unwrap().as_str(), "CREATING");
    assert_eq!(
        info.current_broker_software_info()
            .unwrap()
            .kafka_version()
            .unwrap(),
        "2.8.1"
    );
    assert_eq!(info.number_of_broker_nodes(), Some(3));
    assert_eq!(
        info.broker_node_group_info().unwrap().instance_type(),
        Some("kafka.m5.large")
    );
    assert_eq!(
        info.broker_node_group_info().unwrap().client_subnets(),
        &["subnet-0123456789abcdef0"]
    );
    assert_eq!(
        info.broker_node_group_info().unwrap().security_groups(),
        &["sg-0123456789abcdef0"]
    );
    let tags = info.tags().unwrap();
    assert_eq!(tags.get("TestKey").unwrap(), "TestValue");
    assert_eq!(tags.get("TestKey2").unwrap(), "TestValue2");
}

/// Parity with moto test_delete_cluster (V1 API).
#[tokio::test]
async fn test_delete_cluster_v1() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("TestCluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-0123456789abcdef0")
                .security_groups("sg-0123456789abcdef0")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .tags("TestKey", "TestValue")
        .send()
        .await
        .unwrap();

    client
        .delete_cluster()
        .cluster_arn(resp.cluster_arn().unwrap())
        .send()
        .await
        .expect("delete_cluster should succeed");

    let list = client.list_clusters().send().await.unwrap();
    assert_eq!(list.cluster_info_list().len(), 0);
}

/// Parity with moto test_list_tags_for_resource.
#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("TestCluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-0123456789abcdef0")
                .security_groups("sg-0123456789abcdef0")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .tags("TestKey", "TestValue")
        .tags("TestKey2", "TestValue2")
        .send()
        .await
        .unwrap();

    let arn = resp.cluster_arn().unwrap();

    // Add a new tag
    client
        .tag_resource()
        .resource_arn(arn)
        .tags("TestKey3", "TestValue3")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags - should have all 3
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    let expected: HashMap<String, String> = [
        ("TestKey".to_string(), "TestValue".to_string()),
        ("TestKey2".to_string(), "TestValue2".to_string()),
        ("TestKey3".to_string(), "TestValue3".to_string()),
    ]
    .into_iter()
    .collect();
    assert_eq!(*tags, expected);

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("TestKey3")
        .send()
        .await
        .expect("untag_resource should succeed");

    // List tags - should have 2
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    let expected: HashMap<String, String> = [
        ("TestKey".to_string(), "TestValue".to_string()),
        ("TestKey2".to_string(), "TestValue2".to_string()),
    ]
    .into_iter()
    .collect();
    assert_eq!(*tags, expected);
}

/// Parity with moto test_list_clusters_v2 field assertions.
#[tokio::test]
async fn test_list_clusters_v2_fields() {
    let client = make_kafka_client().await;

    // Create serverless cluster
    client
        .create_cluster_v2()
        .cluster_name("TestServerlessCluster")
        .serverless(
            aws_sdk_kafka::types::ServerlessRequest::builder()
                .vpc_configs(
                    aws_sdk_kafka::types::VpcConfig::builder()
                        .subnet_ids("subnet-0123456789abcdef0")
                        .security_group_ids("sg-0123456789abcdef0")
                        .build(),
                )
                .build(),
        )
        .tags("TestKey", "TestValue")
        .send()
        .await
        .unwrap();

    // Create provisioned cluster
    client
        .create_cluster_v2()
        .cluster_name("TestProvisionedCluster")
        .provisioned(
            aws_sdk_kafka::types::ProvisionedRequest::builder()
                .broker_node_group_info(
                    aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                        .instance_type("kafka.m5.large")
                        .client_subnets("subnet-0123456789abcdef0")
                        .security_groups("sg-0123456789abcdef0")
                        .build(),
                )
                .kafka_version("2.8.1")
                .number_of_broker_nodes(3)
                .build(),
        )
        .tags("TestKey", "TestValue")
        .send()
        .await
        .unwrap();

    let list = client.list_clusters_v2().send().await.unwrap();
    assert_eq!(list.cluster_info_list().len(), 2);

    for cluster in list.cluster_info_list() {
        // All clusters should have these fields
        assert!(cluster.cluster_arn().is_some());
        assert!(cluster.cluster_name().is_some());
        assert!(cluster.cluster_type().is_some());
        assert!(cluster.state().is_some());
        assert!(cluster.tags().is_some());

        let ct = cluster.cluster_type().unwrap().as_str();
        match ct {
            "SERVERLESS" => {
                assert!(cluster.serverless().is_some());
            }
            "PROVISIONED" => {
                assert!(cluster.provisioned().is_some());
            }
            _ => panic!("unexpected cluster type: {ct}"),
        }
    }
}

#[tokio::test]
async fn test_list_clusters_v2_empty() {
    let client = make_kafka_client().await;

    let list_resp = client
        .list_clusters_v2()
        .send()
        .await
        .expect("list_clusters_v2 should succeed on empty state");

    assert_eq!(list_resp.cluster_info_list().len(), 0);
}

#[tokio::test]
async fn test_list_clusters_v1_empty() {
    let client = make_kafka_client().await;

    let list_resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed on empty state");

    assert_eq!(list_resp.cluster_info_list().len(), 0);
}

#[tokio::test]
async fn test_describe_nonexistent_cluster_v1_fails() {
    let client = make_kafka_client().await;

    let result = client
        .describe_cluster()
        .cluster_arn("arn:aws:kafka:us-east-1:123456789012:cluster/nonexistent/fake-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe nonexistent cluster (v1) should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_cluster_fails() {
    let client = make_kafka_client().await;

    let result = client
        .delete_cluster()
        .cluster_arn("arn:aws:kafka:us-east-1:123456789012:cluster/nonexistent/fake-id")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent cluster should fail");
}

#[tokio::test]
async fn test_tag_resource_then_describe_cluster_v1_has_tags() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("TaggedCluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-aabbccdd")
                .security_groups("sg-aabbccdd")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .send()
        .await
        .unwrap();

    let arn = resp.cluster_arn().unwrap();

    client
        .tag_resource()
        .resource_arn(arn)
        .tags("Env", "production")
        .tags("Team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let desc = client
        .describe_cluster()
        .cluster_arn(arn)
        .send()
        .await
        .unwrap();

    let tags = desc.cluster_info().unwrap().tags().unwrap();
    assert_eq!(tags.get("Env").map(String::as_str), Some("production"));
    assert_eq!(tags.get("Team").map(String::as_str), Some("platform"));
}

#[tokio::test]
async fn test_untag_resource_removes_only_specified_keys() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("UntagCluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-aabbccdd")
                .security_groups("sg-aabbccdd")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .tags("KeyA", "ValA")
        .tags("KeyB", "ValB")
        .tags("KeyC", "ValC")
        .send()
        .await
        .unwrap();

    let arn = resp.cluster_arn().unwrap();

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("KeyA")
        .tag_keys("KeyC")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert!(!tags.contains_key("KeyA"), "KeyA should have been removed");
    assert!(!tags.contains_key("KeyC"), "KeyC should have been removed");
    assert_eq!(
        tags.get("KeyB").map(String::as_str),
        Some("ValB"),
        "KeyB should remain"
    );
    assert_eq!(tags.len(), 1);
}

#[tokio::test]
async fn test_list_tags_for_resource_no_tags() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster()
        .cluster_name("NoTagCluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-aabbccdd")
                .security_groups("sg-aabbccdd")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .send()
        .await
        .unwrap();

    let arn = resp.cluster_arn().unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed even with no tags");

    let tags = tags_resp.tags();
    let is_empty = tags.map(|t| t.is_empty()).unwrap_or(true);
    assert!(
        is_empty,
        "cluster created without tags should have empty tag map"
    );
}

#[tokio::test]
async fn test_cluster_arn_contains_region_and_name() {
    let client = make_kafka_client().await;

    let resp = client
        .create_cluster_v2()
        .cluster_name("arn-format-cluster")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    let arn = resp.cluster_arn().expect("should have ARN");
    assert!(
        arn.starts_with("arn:aws:kafka:us-east-1:"),
        "ARN should embed the configured region"
    );
    assert!(
        arn.contains(":cluster/arn-format-cluster/"),
        "ARN should embed the cluster name"
    );
}

#[tokio::test]
async fn test_delete_cluster_removes_from_list_clusters_v2() {
    let client = make_kafka_client().await;

    let create_resp = client
        .create_cluster_v2()
        .cluster_name("transient-cluster")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    let arn = create_resp.cluster_arn().unwrap();

    let list_before = client.list_clusters_v2().send().await.unwrap();
    assert_eq!(list_before.cluster_info_list().len(), 1);

    client
        .delete_cluster()
        .cluster_arn(arn)
        .send()
        .await
        .unwrap();

    let list_after = client.list_clusters_v2().send().await.unwrap();
    assert_eq!(
        list_after.cluster_info_list().len(),
        0,
        "deleted cluster should not appear in list_clusters_v2"
    );
}

#[tokio::test]
async fn test_create_multiple_clusters_and_describe_each() {
    let client = make_kafka_client().await;

    let names = ["alpha", "beta", "gamma"];
    let mut arns = Vec::new();

    for name in &names {
        let resp = client
            .create_cluster_v2()
            .cluster_name(*name)
            .provisioned(make_provisioned_request())
            .send()
            .await
            .unwrap();
        arns.push(resp.cluster_arn().unwrap().to_string());
    }

    let list = client.list_clusters_v2().send().await.unwrap();
    assert_eq!(list.cluster_info_list().len(), 3);

    for (name, arn) in names.iter().zip(arns.iter()) {
        let desc = client
            .describe_cluster_v2()
            .cluster_arn(arn)
            .send()
            .await
            .unwrap();
        let info = desc.cluster_info().unwrap();
        assert_eq!(info.cluster_name().unwrap(), *name);
        assert_eq!(info.cluster_type().unwrap().as_str(), "PROVISIONED");
    }
}

#[tokio::test]
async fn test_v1_and_v2_clusters_share_list() {
    let client = make_kafka_client().await;

    client
        .create_cluster()
        .cluster_name("v1-cluster")
        .broker_node_group_info(
            aws_sdk_kafka::types::BrokerNodeGroupInfo::builder()
                .instance_type("kafka.m5.large")
                .client_subnets("subnet-0123456789abcdef0")
                .security_groups("sg-0123456789abcdef0")
                .build(),
        )
        .kafka_version("2.8.1")
        .number_of_broker_nodes(3)
        .send()
        .await
        .unwrap();

    client
        .create_cluster_v2()
        .cluster_name("v2-cluster")
        .provisioned(make_provisioned_request())
        .send()
        .await
        .unwrap();

    let list_v1 = client.list_clusters().send().await.unwrap();
    let list_v2 = client.list_clusters_v2().send().await.unwrap();

    // Both v1 and v2 list endpoints share the same backing store
    assert_eq!(list_v1.cluster_info_list().len(), 2);
    assert_eq!(list_v2.cluster_info_list().len(), 2);
}

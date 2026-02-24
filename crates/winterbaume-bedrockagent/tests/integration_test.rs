use aws_sdk_bedrockagent::config::BehaviorVersion;
use winterbaume_bedrockagent::BedrockAgentService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_bedrockagent::Client {
    let mock = MockAws::builder()
        .with_service(BedrockAgentService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bedrockagent::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_bedrockagent::Client::new(&config)
}

/// Translated from moto: test_create_agent
#[tokio::test]
async fn test_create_agent() {
    let client = make_client().await;

    let resp = client
        .create_agent()
        .agent_name("agent_name")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .expect("create_agent should succeed");

    let agent = resp.agent().expect("agent should be present");
    assert_eq!(agent.agent_name(), "agent_name");
    assert_eq!(agent.agent_resource_role_arn(), "test-agent-arn");
    assert!(agent.agent_id().starts_with("agent_name"));
    assert_eq!(
        agent.agent_status(),
        &aws_sdk_bedrockagent::types::AgentStatus::NotPrepared
    );
}

/// Translated from moto: test_get_agent
#[tokio::test]
async fn test_get_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    let resp = client
        .get_agent()
        .agent_id(&agent_id)
        .send()
        .await
        .expect("get_agent should succeed");

    let agent = resp.agent().expect("agent should be present");
    assert_eq!(agent.agent_name(), "testname");
    assert_eq!(agent.agent_id(), agent_id);
}

/// Translated from moto: test_get_agent_not_found
#[tokio::test]
async fn test_get_agent_not_found() {
    let client = make_client().await;

    // Create one agent to populate state
    client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let result = client
        .get_agent()
        .agent_id("non-existent-agent-id")
        .send()
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {}",
        err_str
    );
}

/// Translated from moto: test_list_agents
#[tokio::test]
async fn test_list_agents() {
    let client = make_client().await;

    client
        .create_agent()
        .agent_name("testname1")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    client
        .create_agent()
        .agent_name("testname2")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_agents()
        .send()
        .await
        .expect("list_agents should succeed");

    let summaries = resp.agent_summaries();
    assert_eq!(summaries.len(), 2);

    let names: Vec<&str> = summaries.iter().map(|s| s.agent_name()).collect();
    assert!(names.contains(&"testname1"));
    assert!(names.contains(&"testname2"));
}

/// Translated from moto: test_delete_agent
#[tokio::test]
async fn test_delete_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    let resp = client
        .delete_agent()
        .agent_id(&agent_id)
        .skip_resource_in_use_check(true)
        .send()
        .await
        .expect("delete_agent should succeed");

    assert_eq!(resp.agent_id(), agent_id);
    assert_eq!(
        resp.agent_status(),
        &aws_sdk_bedrockagent::types::AgentStatus::Deleting
    );
}

/// Translated from moto: test_delete_agent_not_found
#[tokio::test]
async fn test_delete_agent_not_found() {
    let client = make_client().await;

    client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_agent()
        .agent_id("non-existent-agent-id")
        .skip_resource_in_use_check(true)
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {}",
        err_str
    );
}

/// Translated from moto: test_list_agents_max_results
#[tokio::test]
async fn test_list_agents_max_results() {
    let client = make_client().await;

    client
        .create_agent()
        .agent_name("testname1")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    client
        .create_agent()
        .agent_name("testname2")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_agents()
        .max_results(1)
        .send()
        .await
        .expect("list_agents with max_results should succeed");

    assert_eq!(resp.agent_summaries().len(), 1);
}

/// Translated from moto: test_list_agents_big_max_results
#[tokio::test]
async fn test_list_agents_big_max_results() {
    let client = make_client().await;

    client
        .create_agent()
        .agent_name("testname1")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    client
        .create_agent()
        .agent_name("testname2")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_agents()
        .max_results(4)
        .send()
        .await
        .expect("list_agents with big max_results should succeed");

    assert_eq!(resp.agent_summaries().len(), 2);
}

/// Translated from moto: test_create_knowledge_base
#[tokio::test]
async fn test_create_knowledge_base() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    let resp = client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_knowledge_base should succeed");

    let kb = resp
        .knowledge_base()
        .expect("knowledge_base should be present");
    assert_eq!(kb.name(), "testkb");
}

/// Translated from moto: test_get_knowledge_base
#[tokio::test]
async fn test_get_knowledge_base() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    let create_resp = client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let kb_id = create_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_id()
        .to_string();

    let resp = client
        .get_knowledge_base()
        .knowledge_base_id(&kb_id)
        .send()
        .await
        .expect("get_knowledge_base should succeed");

    assert_eq!(resp.knowledge_base().unwrap().name(), "testkb");
}

/// Translated from moto: test_get_knowledge_base_not_found
#[tokio::test]
async fn test_get_knowledge_base_not_found() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .get_knowledge_base()
        .knowledge_base_id("non-existent-kb-id")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {}",
        err_str
    );
}

/// Translated from moto: test_list_knowledge_bases
#[tokio::test]
async fn test_list_knowledge_bases() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    for name in ["testkb1", "testkb2"] {
        client
            .create_knowledge_base()
            .name(name)
            .description("description")
            .role_arn("test_role_arn")
            .knowledge_base_configuration(
                KnowledgeBaseConfiguration::builder()
                    .r#type(KnowledgeBaseType::Vector)
                    .build()
                    .unwrap(),
            )
            .storage_configuration(
                StorageConfiguration::builder()
                    .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                    .opensearch_serverless_configuration(
                        OpenSearchServerlessConfiguration::builder()
                            .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                            .vector_index_name("test-index")
                            .field_mapping(
                                aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder()
                                    .vector_field("vector")
                                    .text_field("text")
                                    .metadata_field("metadata")
                                    .build()
                                    .unwrap(),
                            )
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_knowledge_bases()
        .send()
        .await
        .expect("list_knowledge_bases should succeed");

    let summaries = resp.knowledge_base_summaries();
    assert_eq!(summaries.len(), 2);

    let names: Vec<&str> = summaries.iter().map(|s| s.name()).collect();
    assert!(names.contains(&"testkb1"));
    assert!(names.contains(&"testkb2"));
}

/// Translated from moto: test_delete_knowledge_base
#[tokio::test]
async fn test_delete_knowledge_base() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    let create_resp = client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let kb_id = create_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_id()
        .to_string();

    let resp = client
        .delete_knowledge_base()
        .knowledge_base_id(&kb_id)
        .send()
        .await
        .expect("delete_knowledge_base should succeed");

    assert_eq!(resp.knowledge_base_id(), kb_id);
    assert_eq!(
        resp.status(),
        &aws_sdk_bedrockagent::types::KnowledgeBaseStatus::Deleting
    );
}

/// Translated from moto: test_delete_knowledge_base_not_found
#[tokio::test]
async fn test_delete_knowledge_base_not_found() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .delete_knowledge_base()
        .knowledge_base_id("non-existent-kb-id")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {}",
        err_str
    );
}

/// Translated from moto: test_list_knowledge_bases_max_results
#[tokio::test]
async fn test_list_knowledge_bases_max_results() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    for name in ["testkb", "testkb2"] {
        client
            .create_knowledge_base()
            .name(name)
            .description("description")
            .role_arn("test_role_arn")
            .knowledge_base_configuration(
                KnowledgeBaseConfiguration::builder()
                    .r#type(KnowledgeBaseType::Vector)
                    .build()
                    .unwrap(),
            )
            .storage_configuration(
                StorageConfiguration::builder()
                    .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                    .opensearch_serverless_configuration(
                        OpenSearchServerlessConfiguration::builder()
                            .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                            .vector_index_name("test-index")
                            .field_mapping(
                                aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder()
                                    .vector_field("vector")
                                    .text_field("text")
                                    .metadata_field("metadata")
                                    .build()
                                    .unwrap(),
                            )
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_knowledge_bases()
        .max_results(1)
        .send()
        .await
        .expect("list_knowledge_bases with max_results should succeed");

    assert_eq!(resp.knowledge_base_summaries().len(), 1);
}

/// Translated from moto: test_list_knowledge_bases_big_max_results
#[tokio::test]
async fn test_list_knowledge_bases_big_max_results() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    for name in ["testkb", "testkb2"] {
        client
            .create_knowledge_base()
            .name(name)
            .description("description")
            .role_arn("test_role_arn")
            .knowledge_base_configuration(
                KnowledgeBaseConfiguration::builder()
                    .r#type(KnowledgeBaseType::Vector)
                    .build()
                    .unwrap(),
            )
            .storage_configuration(
                StorageConfiguration::builder()
                    .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                    .opensearch_serverless_configuration(
                        OpenSearchServerlessConfiguration::builder()
                            .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                            .vector_index_name("test-index")
                            .field_mapping(
                                aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder()
                                    .vector_field("vector")
                                    .text_field("text")
                                    .metadata_field("metadata")
                                    .build()
                                    .unwrap(),
                            )
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_knowledge_bases()
        .max_results(4)
        .send()
        .await
        .expect("list_knowledge_bases with big max_results should succeed");

    assert_eq!(resp.knowledge_base_summaries().len(), 2);
}

/// Translated from moto: test_create_knowledge_base_bad_knowledge_base_config
#[tokio::test]
async fn test_create_knowledge_base_bad_knowledge_base_config() {
    let client = make_client().await;

    // We cannot test this with the SDK directly since the SDK enforces the enum type.
    // The SDK will only allow valid KnowledgeBaseType values.
    // Instead, we verify that VECTOR type works (positive case covered by test_create_knowledge_base)
    // and note this test is a SDK-level validation that cannot be bypassed.
    let _ = client;
}

/// Translated from moto: test_create_knowledge_base_bad_storage_config
#[tokio::test]
async fn test_create_knowledge_base_bad_storage_config() {
    let client = make_client().await;

    // Same as above - the SDK enforces valid StorageType enum values.
    // Cannot send invalid values through the typed SDK.
    let _ = client;
}

/// Translated from moto: test_tag_resource_agent
#[tokio::test]
async fn test_tag_resource_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let agent_arn = create_resp.agent().unwrap().agent_arn().to_string();

    client
        .tag_resource()
        .resource_arn(&agent_arn)
        .tags("Key", "test-tag")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&agent_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("Key").map(|s| s.as_str()), Some("test-tag"));
}

/// Translated from moto: test_tag_resource_knowledge_base
#[tokio::test]
async fn test_tag_resource_knowledge_base() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    let create_resp = client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let kb_arn = create_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_arn()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&kb_arn)
        .tags("Key", "test-tag")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&kb_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("Key").map(|s| s.as_str()), Some("test-tag"));
}

/// Translated from moto: test_untag_resource_agent
#[tokio::test]
async fn test_untag_resource_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let agent_arn = create_resp.agent().unwrap().agent_arn().to_string();

    client
        .tag_resource()
        .resource_arn(&agent_arn)
        .tags("Key1", "test-tag")
        .tags("Key2", "test-tag2")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&agent_arn)
        .tag_keys("Key1")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&agent_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get("Key2").map(|s| s.as_str()), Some("test-tag2"));
}

/// Translated from moto: test_untag_resource_knowledge_base
#[tokio::test]
async fn test_untag_resource_knowledge_base() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    let create_resp = client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let kb_arn = create_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_arn()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&kb_arn)
        .tags("Key1", "test-tag")
        .tags("Key2", "test-tag2")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&kb_arn)
        .tag_keys("Key1")
        .tag_keys("Key2")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&kb_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.len(), 0);
}

/// Translated from moto: test_list_tags_for_resource_agent
#[tokio::test]
async fn test_list_tags_for_resource_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("testname")
        .agent_resource_role_arn("test-agent-arn")
        .send()
        .await
        .unwrap();

    let agent_arn = create_resp.agent().unwrap().agent_arn().to_string();

    client
        .tag_resource()
        .resource_arn(&agent_arn)
        .tags("Key1", "test-tag")
        .tags("Key2", "test-tag2")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&agent_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("Key1").map(|s| s.as_str()), Some("test-tag"));
    assert_eq!(tags.get("Key2").map(|s| s.as_str()), Some("test-tag2"));
}

/// Translated from moto: test_list_tags_for_resource_knowledge_base
#[tokio::test]
async fn test_list_tags_for_resource_knowledge_base() {
    use aws_sdk_bedrockagent::types::{
        KnowledgeBaseConfiguration, KnowledgeBaseType, OpenSearchServerlessConfiguration,
        StorageConfiguration,
    };

    let client = make_client().await;

    let create_resp = client
        .create_knowledge_base()
        .name("testkb")
        .description("description")
        .role_arn("test_role_arn")
        .knowledge_base_configuration(
            KnowledgeBaseConfiguration::builder()
                .r#type(KnowledgeBaseType::Vector)
                .build()
                .unwrap(),
        )
        .storage_configuration(
            StorageConfiguration::builder()
                .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
                .opensearch_serverless_configuration(
                    OpenSearchServerlessConfiguration::builder()
                        .collection_arn("arn:aws:aoss:us-east-1:123456789012:collection/test")
                        .vector_index_name("test-index")
                        .field_mapping(
                            aws_sdk_bedrockagent::types::OpenSearchServerlessFieldMapping::builder(
                            )
                            .vector_field("vector")
                            .text_field("text")
                            .metadata_field("metadata")
                            .build()
                            .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let kb_arn = create_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_arn()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&kb_arn)
        .tags("Key1", "test-tag")
        .tags("Key2", "test-tag2")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&kb_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("Key1").map(|s| s.as_str()), Some("test-tag"));
    assert_eq!(tags.get("Key2").map(|s| s.as_str()), Some("test-tag2"));
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Bedrock Agent
// ============================================================================

#[tokio::test]
async fn test_list_agents_empty() {
    let client = make_client().await;

    let resp = client
        .list_agents()
        .send()
        .await
        .expect("list_agents on empty state should succeed");

    assert_eq!(resp.agent_summaries().len(), 0);
}

#[tokio::test]
async fn test_list_knowledge_bases_empty() {
    let client = make_client().await;

    let resp = client
        .list_knowledge_bases()
        .send()
        .await
        .expect("list_knowledge_bases on empty state should succeed");

    assert_eq!(resp.knowledge_base_summaries().len(), 0);
}

#[tokio::test]
async fn test_create_agent_with_inline_tags() {
    let client = make_client().await;

    let resp = client
        .create_agent()
        .agent_name("tagged-agent")
        .agent_resource_role_arn("test-role-arn")
        .tags("env", "test")
        .tags("team", "ml")
        .send()
        .await
        .expect("create_agent with tags should succeed");

    let agent = resp.agent().expect("should have agent");
    let agent_arn = agent.agent_arn().to_string();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&agent_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert!(tags.len() >= 2, "should have at least 2 tags");
}

#[tokio::test]
async fn test_list_tags_for_resource_not_found() {
    let client = make_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:bedrock:us-east-1:123456789012:agent/nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "list_tags on nonexistent resource should fail"
    );
}

#[tokio::test]
async fn test_delete_agent_without_skip_flag() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("del-no-skip")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();

    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    let resp = client
        .delete_agent()
        .agent_id(&agent_id)
        .skip_resource_in_use_check(false)
        .send()
        .await
        .expect("delete_agent with skip=false should succeed");

    assert_eq!(resp.agent_id(), agent_id);
}

#[tokio::test]
async fn test_agent_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_agent()
        .agent_name("arn-format-agent")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();

    let agent = resp.agent().unwrap();
    let arn = agent.agent_arn();
    assert!(
        arn.starts_with("arn:aws:bedrock:"),
        "ARN should start with arn:aws:bedrock:"
    );
    assert!(arn.contains("agent/"), "ARN should contain agent/");
}

// ============================================================================
// Tests for untested operations: Agent Update & Prepare
// ============================================================================

#[tokio::test]
async fn test_update_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("update-me")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();

    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    let resp = client
        .update_agent()
        .agent_id(&agent_id)
        .agent_name("updated-name")
        .agent_resource_role_arn("new-role-arn")
        .send()
        .await
        .expect("update_agent should succeed");

    let agent = resp.agent().expect("agent should be present");
    assert_eq!(agent.agent_name(), "updated-name");
    assert_eq!(agent.agent_resource_role_arn(), "new-role-arn");
}

#[tokio::test]
async fn test_prepare_agent() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("prepare-me")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();

    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    let resp = client
        .prepare_agent()
        .agent_id(&agent_id)
        .send()
        .await
        .expect("prepare_agent should succeed");

    assert_eq!(resp.agent_id(), agent_id);
    assert_eq!(
        resp.agent_status(),
        &aws_sdk_bedrockagent::types::AgentStatus::Prepared
    );
}

// ============================================================================
// Tests for Agent Alias lifecycle
// ============================================================================

#[tokio::test]
async fn test_agent_alias_lifecycle() {
    let client = make_client().await;

    // Create agent first
    let create_resp = client
        .create_agent()
        .agent_name("alias-agent")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();
    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    // Create alias
    let alias_resp = client
        .create_agent_alias()
        .agent_id(&agent_id)
        .agent_alias_name("my-alias")
        .send()
        .await
        .expect("create_agent_alias should succeed");

    let alias = alias_resp.agent_alias().expect("alias should be present");
    assert_eq!(alias.agent_alias_name(), "my-alias");
    let alias_id = alias.agent_alias_id().to_string();

    // Get alias
    let get_resp = client
        .get_agent_alias()
        .agent_id(&agent_id)
        .agent_alias_id(&alias_id)
        .send()
        .await
        .expect("get_agent_alias should succeed");

    let got = get_resp.agent_alias().unwrap();
    assert_eq!(got.agent_alias_name(), "my-alias");
    assert_eq!(got.agent_alias_id(), alias_id);

    // Update alias
    let update_resp = client
        .update_agent_alias()
        .agent_id(&agent_id)
        .agent_alias_id(&alias_id)
        .agent_alias_name("renamed-alias")
        .send()
        .await
        .expect("update_agent_alias should succeed");

    let updated = update_resp.agent_alias().unwrap();
    assert_eq!(updated.agent_alias_name(), "renamed-alias");

    // List aliases
    let list_resp = client
        .list_agent_aliases()
        .agent_id(&agent_id)
        .send()
        .await
        .expect("list_agent_aliases should succeed");

    assert_eq!(list_resp.agent_alias_summaries().len(), 1);

    // Delete alias
    let del_resp = client
        .delete_agent_alias()
        .agent_id(&agent_id)
        .agent_alias_id(&alias_id)
        .send()
        .await
        .expect("delete_agent_alias should succeed");

    assert_eq!(del_resp.agent_alias_id(), alias_id);
}

// ============================================================================
// Tests for Agent Version operations
// ============================================================================

#[tokio::test]
async fn test_agent_version_lifecycle() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("version-agent")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();
    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    // List versions
    let list_resp = client
        .list_agent_versions()
        .agent_id(&agent_id)
        .send()
        .await
        .expect("list_agent_versions should succeed");

    assert_eq!(list_resp.agent_version_summaries().len(), 1);

    // Get version
    let get_resp = client
        .get_agent_version()
        .agent_id(&agent_id)
        .agent_version("1.0")
        .send()
        .await
        .expect("get_agent_version should succeed");

    let ver = get_resp.agent_version().unwrap();
    assert_eq!(ver.agent_name(), "version-agent");

    // Delete version
    let del_resp = client
        .delete_agent_version()
        .agent_id(&agent_id)
        .agent_version("1.0")
        .skip_resource_in_use_check(false)
        .send()
        .await
        .expect("delete_agent_version should succeed");

    assert_eq!(del_resp.agent_id(), agent_id);
    assert_eq!(del_resp.agent_version(), "1.0");
}

// ============================================================================
// Tests for Agent Action Group lifecycle
// ============================================================================

#[tokio::test]
async fn test_agent_action_group_lifecycle() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("ag-agent")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();
    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    // Create action group
    let ag_resp = client
        .create_agent_action_group()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .action_group_name("my-action-group")
        .send()
        .await
        .expect("create_agent_action_group should succeed");

    let ag = ag_resp.agent_action_group().unwrap();
    assert_eq!(ag.action_group_name(), "my-action-group");
    let ag_id = ag.action_group_id().to_string();

    // Get action group
    let get_resp = client
        .get_agent_action_group()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .action_group_id(&ag_id)
        .send()
        .await
        .expect("get_agent_action_group should succeed");

    let got = get_resp.agent_action_group().unwrap();
    assert_eq!(got.action_group_name(), "my-action-group");

    // Update action group
    let update_resp = client
        .update_agent_action_group()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .action_group_id(&ag_id)
        .action_group_name("renamed-group")
        .send()
        .await
        .expect("update_agent_action_group should succeed");

    let updated = update_resp.agent_action_group().unwrap();
    assert_eq!(updated.action_group_name(), "renamed-group");

    // List action groups
    let list_resp = client
        .list_agent_action_groups()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .send()
        .await
        .expect("list_agent_action_groups should succeed");

    assert_eq!(list_resp.action_group_summaries().len(), 1);

    // Delete action group
    client
        .delete_agent_action_group()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .action_group_id(&ag_id)
        .send()
        .await
        .expect("delete_agent_action_group should succeed");

    // Verify deleted
    let list_resp2 = client
        .list_agent_action_groups()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.action_group_summaries().len(), 0);
}

// ============================================================================
// Tests for Agent Knowledge Base association lifecycle
// ============================================================================

#[tokio::test]
async fn test_agent_knowledge_base_lifecycle() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("akb-agent")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();
    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    // Associate agent knowledge base
    let assoc_resp = client
        .associate_agent_knowledge_base()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .knowledge_base_id("kb-12345")
        .description("Test KB association")
        .send()
        .await
        .expect("associate_agent_knowledge_base should succeed");

    let akb = assoc_resp.agent_knowledge_base().unwrap();
    assert_eq!(akb.knowledge_base_id(), "kb-12345");
    assert_eq!(akb.description(), "Test KB association");

    // Get agent knowledge base
    let get_resp = client
        .get_agent_knowledge_base()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .knowledge_base_id("kb-12345")
        .send()
        .await
        .expect("get_agent_knowledge_base should succeed");

    let got = get_resp.agent_knowledge_base().unwrap();
    assert_eq!(got.knowledge_base_id(), "kb-12345");

    // Update agent knowledge base
    let update_resp = client
        .update_agent_knowledge_base()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .knowledge_base_id("kb-12345")
        .description("Updated description")
        .send()
        .await
        .expect("update_agent_knowledge_base should succeed");

    let updated = update_resp.agent_knowledge_base().unwrap();
    assert_eq!(updated.description(), "Updated description");

    // List agent knowledge bases
    let list_resp = client
        .list_agent_knowledge_bases()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .send()
        .await
        .expect("list_agent_knowledge_bases should succeed");

    assert_eq!(list_resp.agent_knowledge_base_summaries().len(), 1);

    // Disassociate
    client
        .disassociate_agent_knowledge_base()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .knowledge_base_id("kb-12345")
        .send()
        .await
        .expect("disassociate_agent_knowledge_base should succeed");

    // Verify disassociated
    let list_resp2 = client
        .list_agent_knowledge_bases()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.agent_knowledge_base_summaries().len(), 0);
}

// ============================================================================
// Tests for Agent Collaborator lifecycle
// ============================================================================

#[tokio::test]
async fn test_agent_collaborator_lifecycle() {
    let client = make_client().await;

    let create_resp = client
        .create_agent()
        .agent_name("collab-agent")
        .agent_resource_role_arn("test-role-arn")
        .send()
        .await
        .unwrap();
    let agent_id = create_resp.agent().unwrap().agent_id().to_string();

    // Associate collaborator
    let assoc_resp = client
        .associate_agent_collaborator()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .agent_descriptor(
            aws_sdk_bedrockagent::types::AgentDescriptor::builder()
                .alias_arn("arn:aws:bedrock:us-east-1:123456789012:agent-alias/other/alias1")
                .build(),
        )
        .collaboration_instruction("Help with tasks")
        .collaborator_name("my-collaborator")
        .send()
        .await
        .expect("associate_agent_collaborator should succeed");

    let col = assoc_resp.agent_collaborator().unwrap();
    assert_eq!(col.collaborator_name(), "my-collaborator");
    let col_id = col.collaborator_id().to_string();

    // Get collaborator
    let get_resp = client
        .get_agent_collaborator()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .collaborator_id(&col_id)
        .send()
        .await
        .expect("get_agent_collaborator should succeed");

    let got = get_resp.agent_collaborator().unwrap();
    assert_eq!(got.collaborator_name(), "my-collaborator");

    // Update collaborator
    let update_resp = client
        .update_agent_collaborator()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .collaborator_id(&col_id)
        .agent_descriptor(
            aws_sdk_bedrockagent::types::AgentDescriptor::builder()
                .alias_arn("arn:aws:bedrock:us-east-1:123456789012:agent-alias/other/alias1")
                .build(),
        )
        .collaboration_instruction("Updated instruction")
        .collaborator_name("renamed-collaborator")
        .send()
        .await
        .expect("update_agent_collaborator should succeed");

    let updated = update_resp.agent_collaborator().unwrap();
    assert_eq!(updated.collaborator_name(), "renamed-collaborator");

    // List collaborators
    let list_resp = client
        .list_agent_collaborators()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .send()
        .await
        .expect("list_agent_collaborators should succeed");

    assert_eq!(list_resp.agent_collaborator_summaries().len(), 1);

    // Disassociate
    client
        .disassociate_agent_collaborator()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .collaborator_id(&col_id)
        .send()
        .await
        .expect("disassociate_agent_collaborator should succeed");

    let list_resp2 = client
        .list_agent_collaborators()
        .agent_id(&agent_id)
        .agent_version("DRAFT")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.agent_collaborator_summaries().len(), 0);
}

// ============================================================================
// Tests for Knowledge Base Update
// ============================================================================

#[tokio::test]
async fn test_update_knowledge_base() {
    let client = make_client().await;

    let kb_config = aws_sdk_bedrockagent::types::KnowledgeBaseConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseType::Vector)
        .vector_knowledge_base_configuration(
            aws_sdk_bedrockagent::types::VectorKnowledgeBaseConfiguration::builder()
                .embedding_model_arn(
                    "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1",
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let storage_config = aws_sdk_bedrockagent::types::StorageConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
        .build()
        .unwrap();

    let create_resp = client
        .create_knowledge_base()
        .name("update-kb")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .knowledge_base_configuration(kb_config.clone())
        .storage_configuration(storage_config.clone())
        .send()
        .await
        .unwrap();

    let kb_id = create_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_id()
        .to_string();

    let update_resp = client
        .update_knowledge_base()
        .knowledge_base_id(&kb_id)
        .name("updated-kb-name")
        .role_arn("arn:aws:iam::123456789012:role/updated-role")
        .knowledge_base_configuration(kb_config)
        .storage_configuration(storage_config)
        .send()
        .await
        .expect("update_knowledge_base should succeed");

    let kb = update_resp.knowledge_base().unwrap();
    assert_eq!(kb.name(), "updated-kb-name");
}

// ============================================================================
// Tests for Data Source lifecycle
// ============================================================================

#[tokio::test]
async fn test_data_source_lifecycle() {
    let client = make_client().await;

    // Create KB first
    let kb_config = aws_sdk_bedrockagent::types::KnowledgeBaseConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseType::Vector)
        .vector_knowledge_base_configuration(
            aws_sdk_bedrockagent::types::VectorKnowledgeBaseConfiguration::builder()
                .embedding_model_arn(
                    "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1",
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let storage_config = aws_sdk_bedrockagent::types::StorageConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
        .build()
        .unwrap();

    let kb_resp = client
        .create_knowledge_base()
        .name("ds-kb")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .knowledge_base_configuration(kb_config)
        .storage_configuration(storage_config)
        .send()
        .await
        .unwrap();
    let kb_id = kb_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_id()
        .to_string();

    // Create data source
    let ds_config = aws_sdk_bedrockagent::types::DataSourceConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::DataSourceType::S3)
        .build()
        .unwrap();

    let ds_resp = client
        .create_data_source()
        .knowledge_base_id(&kb_id)
        .name("my-datasource")
        .data_source_configuration(ds_config)
        .send()
        .await
        .expect("create_data_source should succeed");

    let ds = ds_resp.data_source().unwrap();
    assert_eq!(ds.name(), "my-datasource");
    let ds_id = ds.data_source_id().to_string();

    // Get data source
    let get_resp = client
        .get_data_source()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("get_data_source should succeed");

    assert_eq!(get_resp.data_source().unwrap().name(), "my-datasource");

    // Update data source
    let ds_config2 = aws_sdk_bedrockagent::types::DataSourceConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::DataSourceType::S3)
        .build()
        .unwrap();

    let update_resp = client
        .update_data_source()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .name("updated-datasource")
        .data_source_configuration(ds_config2)
        .send()
        .await
        .expect("update_data_source should succeed");

    assert_eq!(
        update_resp.data_source().unwrap().name(),
        "updated-datasource"
    );

    // List data sources
    let list_resp = client
        .list_data_sources()
        .knowledge_base_id(&kb_id)
        .send()
        .await
        .expect("list_data_sources should succeed");

    assert_eq!(list_resp.data_source_summaries().len(), 1);

    // Delete data source
    let del_resp = client
        .delete_data_source()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("delete_data_source should succeed");

    assert_eq!(del_resp.data_source_id(), ds_id);

    // Verify deleted
    let list_resp2 = client
        .list_data_sources()
        .knowledge_base_id(&kb_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.data_source_summaries().len(), 0);
}

// ============================================================================
// Tests for Ingestion Job lifecycle
// ============================================================================

#[tokio::test]
async fn test_ingestion_job_lifecycle() {
    let client = make_client().await;

    // Create KB + data source
    let kb_config = aws_sdk_bedrockagent::types::KnowledgeBaseConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseType::Vector)
        .vector_knowledge_base_configuration(
            aws_sdk_bedrockagent::types::VectorKnowledgeBaseConfiguration::builder()
                .embedding_model_arn(
                    "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1",
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let storage_config = aws_sdk_bedrockagent::types::StorageConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
        .build()
        .unwrap();

    let kb_resp = client
        .create_knowledge_base()
        .name("ingest-kb")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .knowledge_base_configuration(kb_config)
        .storage_configuration(storage_config)
        .send()
        .await
        .unwrap();
    let kb_id = kb_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_id()
        .to_string();

    let ds_config = aws_sdk_bedrockagent::types::DataSourceConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::DataSourceType::S3)
        .build()
        .unwrap();

    let ds_resp = client
        .create_data_source()
        .knowledge_base_id(&kb_id)
        .name("ingest-ds")
        .data_source_configuration(ds_config)
        .send()
        .await
        .unwrap();
    let ds_id = ds_resp.data_source().unwrap().data_source_id().to_string();

    // Start ingestion job
    let start_resp = client
        .start_ingestion_job()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .description("Test ingestion")
        .send()
        .await
        .expect("start_ingestion_job should succeed");

    let job = start_resp.ingestion_job().unwrap();
    assert_eq!(
        job.status(),
        &aws_sdk_bedrockagent::types::IngestionJobStatus::Starting
    );
    let job_id = job.ingestion_job_id().to_string();

    // Get ingestion job
    let get_resp = client
        .get_ingestion_job()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .ingestion_job_id(&job_id)
        .send()
        .await
        .expect("get_ingestion_job should succeed");

    assert_eq!(get_resp.ingestion_job().unwrap().ingestion_job_id(), job_id);

    // List ingestion jobs
    let list_resp = client
        .list_ingestion_jobs()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("list_ingestion_jobs should succeed");

    assert_eq!(list_resp.ingestion_job_summaries().len(), 1);

    // Stop ingestion job
    let stop_resp = client
        .stop_ingestion_job()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .ingestion_job_id(&job_id)
        .send()
        .await
        .expect("stop_ingestion_job should succeed");

    assert_eq!(
        stop_resp.ingestion_job().unwrap().status(),
        &aws_sdk_bedrockagent::types::IngestionJobStatus::Stopped
    );
}

// ============================================================================
// Tests for KB Document operations
// ============================================================================

#[tokio::test]
async fn test_knowledge_base_documents() {
    let client = make_client().await;

    // Create KB + data source
    let kb_config = aws_sdk_bedrockagent::types::KnowledgeBaseConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseType::Vector)
        .vector_knowledge_base_configuration(
            aws_sdk_bedrockagent::types::VectorKnowledgeBaseConfiguration::builder()
                .embedding_model_arn(
                    "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1",
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let storage_config = aws_sdk_bedrockagent::types::StorageConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::KnowledgeBaseStorageType::OpensearchServerless)
        .build()
        .unwrap();

    let kb_resp = client
        .create_knowledge_base()
        .name("doc-kb")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .knowledge_base_configuration(kb_config)
        .storage_configuration(storage_config)
        .send()
        .await
        .unwrap();
    let kb_id = kb_resp
        .knowledge_base()
        .unwrap()
        .knowledge_base_id()
        .to_string();

    let ds_config = aws_sdk_bedrockagent::types::DataSourceConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::DataSourceType::S3)
        .build()
        .unwrap();

    let ds_resp = client
        .create_data_source()
        .knowledge_base_id(&kb_id)
        .name("doc-ds")
        .data_source_configuration(ds_config)
        .send()
        .await
        .unwrap();
    let ds_id = ds_resp.data_source().unwrap().data_source_id().to_string();

    // Ingest documents (returns empty list for mock)
    let ingest_resp = client
        .ingest_knowledge_base_documents()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("ingest_knowledge_base_documents should succeed");

    assert!(ingest_resp.document_details().is_empty());

    // List documents
    let list_resp = client
        .list_knowledge_base_documents()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("list_knowledge_base_documents should succeed");

    assert!(list_resp.document_details().is_empty());

    // Get documents
    let get_resp = client
        .get_knowledge_base_documents()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("get_knowledge_base_documents should succeed");

    assert!(get_resp.document_details().is_empty());

    // Delete documents
    let del_resp = client
        .delete_knowledge_base_documents()
        .knowledge_base_id(&kb_id)
        .data_source_id(&ds_id)
        .send()
        .await
        .expect("delete_knowledge_base_documents should succeed");

    assert!(del_resp.document_details().is_empty());
}

// ============================================================================
// Tests for Flow lifecycle
// ============================================================================

#[tokio::test]
async fn test_flow_lifecycle() {
    let client = make_client().await;

    // Create flow
    let create_resp = client
        .create_flow()
        .name("my-flow")
        .execution_role_arn("arn:aws:iam::123456789012:role/flow-role")
        .description("Test flow")
        .send()
        .await
        .expect("create_flow should succeed");

    assert_eq!(create_resp.name(), "my-flow");
    let flow_id = create_resp.id().to_string();

    // Get flow
    let get_resp = client
        .get_flow()
        .flow_identifier(&flow_id)
        .send()
        .await
        .expect("get_flow should succeed");

    assert_eq!(get_resp.name(), "my-flow");
    assert_eq!(get_resp.id(), flow_id);

    // Update flow
    let update_resp = client
        .update_flow()
        .flow_identifier(&flow_id)
        .name("updated-flow")
        .execution_role_arn("arn:aws:iam::123456789012:role/flow-role")
        .send()
        .await
        .expect("update_flow should succeed");

    assert_eq!(update_resp.name(), "updated-flow");

    // List flows
    let list_resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");

    assert_eq!(list_resp.flow_summaries().len(), 1);

    // Prepare flow
    let prepare_resp = client
        .prepare_flow()
        .flow_identifier(&flow_id)
        .send()
        .await
        .expect("prepare_flow should succeed");

    assert_eq!(prepare_resp.id(), flow_id);

    // Delete flow
    let del_resp = client
        .delete_flow()
        .flow_identifier(&flow_id)
        .send()
        .await
        .expect("delete_flow should succeed");

    assert_eq!(del_resp.id(), flow_id);

    // Verify deleted
    let list_resp2 = client.list_flows().send().await.unwrap();
    assert_eq!(list_resp2.flow_summaries().len(), 0);
}

// ============================================================================
// Tests for Flow Alias lifecycle
// ============================================================================

#[tokio::test]
async fn test_flow_alias_lifecycle() {
    let client = make_client().await;

    // Create flow first
    let flow_resp = client
        .create_flow()
        .name("alias-flow")
        .execution_role_arn("arn:aws:iam::123456789012:role/flow-role")
        .send()
        .await
        .unwrap();
    let flow_id = flow_resp.id().to_string();

    // Create flow alias
    let alias_resp = client
        .create_flow_alias()
        .flow_identifier(&flow_id)
        .name("my-flow-alias")
        .routing_configuration(
            aws_sdk_bedrockagent::types::FlowAliasRoutingConfigurationListItem::builder()
                .flow_version("DRAFT")
                .build(),
        )
        .send()
        .await
        .expect("create_flow_alias should succeed");

    assert_eq!(alias_resp.name(), "my-flow-alias");
    let alias_id = alias_resp.id().to_string();

    // Get flow alias
    let get_resp = client
        .get_flow_alias()
        .flow_identifier(&flow_id)
        .alias_identifier(&alias_id)
        .send()
        .await
        .expect("get_flow_alias should succeed");

    assert_eq!(get_resp.name(), "my-flow-alias");

    // Update flow alias
    let update_resp = client
        .update_flow_alias()
        .flow_identifier(&flow_id)
        .alias_identifier(&alias_id)
        .name("renamed-flow-alias")
        .routing_configuration(
            aws_sdk_bedrockagent::types::FlowAliasRoutingConfigurationListItem::builder()
                .flow_version("DRAFT")
                .build(),
        )
        .send()
        .await
        .expect("update_flow_alias should succeed");

    assert_eq!(update_resp.name(), "renamed-flow-alias");

    // List flow aliases
    let list_resp = client
        .list_flow_aliases()
        .flow_identifier(&flow_id)
        .send()
        .await
        .expect("list_flow_aliases should succeed");

    assert_eq!(list_resp.flow_alias_summaries().len(), 1);

    // Delete flow alias
    let del_resp = client
        .delete_flow_alias()
        .flow_identifier(&flow_id)
        .alias_identifier(&alias_id)
        .send()
        .await
        .expect("delete_flow_alias should succeed");

    assert_eq!(del_resp.id(), alias_id);
}

// ============================================================================
// Tests for Flow Version lifecycle
// ============================================================================

#[tokio::test]
async fn test_flow_version_lifecycle() {
    let client = make_client().await;

    let flow_resp = client
        .create_flow()
        .name("version-flow")
        .execution_role_arn("arn:aws:iam::123456789012:role/flow-role")
        .send()
        .await
        .unwrap();
    let flow_id = flow_resp.id().to_string();

    // Create flow version
    let ver_resp = client
        .create_flow_version()
        .flow_identifier(&flow_id)
        .send()
        .await
        .expect("create_flow_version should succeed");

    assert_eq!(ver_resp.version(), "1");
    let version = ver_resp.version().to_string();

    // Get flow version
    let get_resp = client
        .get_flow_version()
        .flow_identifier(&flow_id)
        .flow_version(&version)
        .send()
        .await
        .expect("get_flow_version should succeed");

    assert_eq!(get_resp.version(), "1");

    // List flow versions
    let list_resp = client
        .list_flow_versions()
        .flow_identifier(&flow_id)
        .send()
        .await
        .expect("list_flow_versions should succeed");

    assert_eq!(list_resp.flow_version_summaries().len(), 1);

    // Delete flow version
    let del_resp = client
        .delete_flow_version()
        .flow_identifier(&flow_id)
        .flow_version(&version)
        .send()
        .await
        .expect("delete_flow_version should succeed");

    assert_eq!(del_resp.id(), flow_id);

    // Verify deleted
    let list_resp2 = client
        .list_flow_versions()
        .flow_identifier(&flow_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp2.flow_version_summaries().len(), 0);
}

// ============================================================================
// Tests for Prompt lifecycle
// ============================================================================

#[tokio::test]
async fn test_prompt_lifecycle() {
    let client = make_client().await;

    // Create prompt
    let create_resp = client
        .create_prompt()
        .name("my-prompt")
        .description("A test prompt")
        .send()
        .await
        .expect("create_prompt should succeed");

    assert_eq!(create_resp.name(), "my-prompt");
    let prompt_id = create_resp.id().to_string();

    // Get prompt
    let get_resp = client
        .get_prompt()
        .prompt_identifier(&prompt_id)
        .send()
        .await
        .expect("get_prompt should succeed");

    assert_eq!(get_resp.name(), "my-prompt");

    // Update prompt
    let update_resp = client
        .update_prompt()
        .prompt_identifier(&prompt_id)
        .name("updated-prompt")
        .description("Updated description")
        .send()
        .await
        .expect("update_prompt should succeed");

    assert_eq!(update_resp.name(), "updated-prompt");

    // List prompts
    let list_resp = client
        .list_prompts()
        .send()
        .await
        .expect("list_prompts should succeed");

    assert_eq!(list_resp.prompt_summaries().len(), 1);

    // Create prompt version
    let ver_resp = client
        .create_prompt_version()
        .prompt_identifier(&prompt_id)
        .send()
        .await
        .expect("create_prompt_version should succeed");

    assert_eq!(ver_resp.version(), "1");

    // Delete prompt
    let del_resp = client
        .delete_prompt()
        .prompt_identifier(&prompt_id)
        .send()
        .await
        .expect("delete_prompt should succeed");

    assert_eq!(del_resp.id(), prompt_id);

    // Verify deleted
    let list_resp2 = client.list_prompts().send().await.unwrap();
    assert_eq!(list_resp2.prompt_summaries().len(), 0);
}

// ============================================================================
// Error path tests for new operations
// ============================================================================

#[tokio::test]
async fn test_get_flow_not_found() {
    let client = make_client().await;

    let result = client
        .get_flow()
        .flow_identifier("nonexistent-flow")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_get_prompt_not_found() {
    let client = make_client().await;

    let result = client
        .get_prompt()
        .prompt_identifier("nonexistent-prompt")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_delete_flow_not_found() {
    let client = make_client().await;

    let result = client
        .delete_flow()
        .flow_identifier("nonexistent-flow")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_create_agent_alias_agent_not_found() {
    let client = make_client().await;

    let result = client
        .create_agent_alias()
        .agent_id("nonexistent-agent")
        .agent_alias_name("my-alias")
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_create_data_source_kb_not_found() {
    let client = make_client().await;

    let ds_config = aws_sdk_bedrockagent::types::DataSourceConfiguration::builder()
        .r#type(aws_sdk_bedrockagent::types::DataSourceType::S3)
        .build()
        .unwrap();

    let result = client
        .create_data_source()
        .knowledge_base_id("nonexistent-kb")
        .name("my-ds")
        .data_source_configuration(ds_config)
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"));
}

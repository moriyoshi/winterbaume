use aws_sdk_mq::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mq::MqService;

async fn make_mq_client() -> aws_sdk_mq::Client {
    let mock = MockAws::builder().with_service(MqService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mq::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_mq::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_broker() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("test-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .expect("create_broker should succeed");

    let broker_id = create_resp.broker_id().expect("should have broker_id");
    let broker_arn = create_resp.broker_arn().expect("should have broker_arn");
    assert!(!broker_id.is_empty());
    assert!(broker_arn.contains("test-broker"));

    let describe_resp = client
        .describe_broker()
        .broker_id(broker_id)
        .send()
        .await
        .expect("describe_broker should succeed");

    assert_eq!(describe_resp.broker_name().unwrap(), "test-broker");
    assert_eq!(describe_resp.broker_id().unwrap(), broker_id);
}

#[tokio::test]
async fn test_delete_broker() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("delete-me")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    let delete_resp = client
        .delete_broker()
        .broker_id(broker_id)
        .send()
        .await
        .expect("delete_broker should succeed");

    assert_eq!(delete_resp.broker_id().unwrap(), broker_id);

    // Describe after delete should fail
    let result = client.describe_broker().broker_id(broker_id).send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_brokers() {
    let client = make_mq_client().await;

    client
        .create_broker()
        .broker_name("broker-a")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    client
        .create_broker()
        .broker_name("broker-b")
        .engine_type(aws_sdk_mq::types::EngineType::Rabbitmq)
        .engine_version("3.11.20")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_brokers()
        .send()
        .await
        .expect("list_brokers should succeed");

    assert_eq!(list_resp.broker_summaries().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_broker_fails() {
    let client = make_mq_client().await;

    let result = client
        .describe_broker()
        .broker_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent broker should fail");
}

// --- Configuration tests ---

#[tokio::test]
async fn test_create_and_describe_configuration() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_configuration()
        .name("test-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .expect("create_configuration should succeed");

    let config_id = create_resp.id().expect("should have id");
    assert!(!config_id.is_empty());
    assert!(create_resp.arn().unwrap().contains("configuration"));
    assert_eq!(create_resp.name().unwrap(), "test-config");

    let describe_resp = client
        .describe_configuration()
        .configuration_id(config_id)
        .send()
        .await
        .expect("describe_configuration should succeed");

    assert_eq!(describe_resp.name().unwrap(), "test-config");
    assert_eq!(describe_resp.id().unwrap(), config_id);
    assert_eq!(
        describe_resp.engine_type().unwrap(),
        &aws_sdk_mq::types::EngineType::Activemq
    );
}

#[tokio::test]
async fn test_list_configurations() {
    let client = make_mq_client().await;

    client
        .create_configuration()
        .name("config-a")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .unwrap();

    client
        .create_configuration()
        .name("config-b")
        .engine_type(aws_sdk_mq::types::EngineType::Rabbitmq)
        .engine_version("3.11.20")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_configurations()
        .send()
        .await
        .expect("list_configurations should succeed");

    assert_eq!(list_resp.configurations().len(), 2);
}

#[tokio::test]
async fn test_update_configuration_and_revisions() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_configuration()
        .name("update-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .unwrap();

    let config_id = create_resp.id().unwrap();

    // Update the configuration
    let update_resp = client
        .update_configuration()
        .configuration_id(config_id)
        .data("PD94bWwgdmVyc2lvbj0iMS4wIj8+")
        .description("Updated config")
        .send()
        .await
        .expect("update_configuration should succeed");

    assert_eq!(update_resp.id().unwrap(), config_id);
    let latest = update_resp.latest_revision().unwrap();
    assert_eq!(latest.revision(), Some(2));
    assert_eq!(latest.description().unwrap(), "Updated config");

    // List revisions
    let revisions_resp = client
        .list_configuration_revisions()
        .configuration_id(config_id)
        .send()
        .await
        .expect("list_configuration_revisions should succeed");

    assert_eq!(revisions_resp.revisions().len(), 2);

    // Describe specific revision
    let rev_resp = client
        .describe_configuration_revision()
        .configuration_id(config_id)
        .configuration_revision("2")
        .send()
        .await
        .expect("describe_configuration_revision should succeed");

    assert_eq!(rev_resp.configuration_id().unwrap(), config_id);
    assert_eq!(rev_resp.description().unwrap(), "Updated config");
}

#[tokio::test]
async fn test_describe_nonexistent_configuration_fails() {
    let client = make_mq_client().await;

    let result = client
        .describe_configuration()
        .configuration_id("nonexistent-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe nonexistent configuration should fail"
    );
}

// --- Tag tests ---

#[tokio::test]
async fn test_create_list_delete_tags() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("tagged-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_arn = create_resp.broker_arn().unwrap();

    // Create tags
    client
        .create_tags()
        .resource_arn(broker_arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("create_tags should succeed");

    // List tags
    let tags_resp = client
        .list_tags()
        .resource_arn(broker_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert_eq!(tags.get("team").unwrap(), "backend");

    // Delete one tag
    client
        .delete_tags()
        .resource_arn(broker_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("delete_tags should succeed");

    // Verify tag was removed
    let tags_resp = client
        .list_tags()
        .resource_arn(broker_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert!(!tags.contains_key("team"));
}

// --- User tests ---

#[tokio::test]
async fn test_create_describe_delete_user() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    // Create user
    client
        .create_user()
        .broker_id(broker_id)
        .username("testuser")
        .console_access(true)
        .groups("admin")
        .send()
        .await
        .expect("create_user should succeed");

    // Describe user
    let user_resp = client
        .describe_user()
        .broker_id(broker_id)
        .username("testuser")
        .send()
        .await
        .expect("describe_user should succeed");

    assert_eq!(user_resp.username().unwrap(), "testuser");
    assert!(user_resp.console_access().unwrap());
    assert!(user_resp.groups().iter().any(|g| g == "admin"));

    // Delete user
    client
        .delete_user()
        .broker_id(broker_id)
        .username("testuser")
        .send()
        .await
        .expect("delete_user should succeed");

    // Describe after delete should fail
    let result = client
        .describe_user()
        .broker_id(broker_id)
        .username("testuser")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_users() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("list-users-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    client
        .create_user()
        .broker_id(broker_id)
        .username("user1")
        .send()
        .await
        .unwrap();

    client
        .create_user()
        .broker_id(broker_id)
        .username("user2")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_users()
        .broker_id(broker_id)
        .send()
        .await
        .expect("list_users should succeed");

    assert_eq!(list_resp.users().len(), 2);
    assert_eq!(list_resp.broker_id().unwrap(), broker_id);
}

#[tokio::test]
async fn test_create_duplicate_user_fails() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("dup-user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    client
        .create_user()
        .broker_id(broker_id)
        .username("dupuser")
        .send()
        .await
        .unwrap();

    let result = client
        .create_user()
        .broker_id(broker_id)
        .username("dupuser")
        .send()
        .await;
    assert!(result.is_err(), "creating duplicate user should fail");
}

// --- Reboot test ---

#[tokio::test]
async fn test_reboot_broker() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("reboot-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    client
        .reboot_broker()
        .broker_id(broker_id)
        .send()
        .await
        .expect("reboot_broker should succeed");
}

#[tokio::test]
async fn test_reboot_nonexistent_broker_fails() {
    let client = make_mq_client().await;

    let result = client
        .reboot_broker()
        .broker_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err(), "reboot nonexistent broker should fail");
}

// --- DescribeBrokerEngineTypes test ---

#[tokio::test]
async fn test_describe_broker_engine_types() {
    let client = make_mq_client().await;

    let resp = client
        .describe_broker_engine_types()
        .send()
        .await
        .expect("describe_broker_engine_types should succeed");

    let engine_types = resp.broker_engine_types();
    assert!(!engine_types.is_empty());

    let activemq = engine_types
        .iter()
        .find(|et| et.engine_type() == Some(&aws_sdk_mq::types::EngineType::Activemq))
        .expect("should have ACTIVEMQ engine type");
    assert!(!activemq.engine_versions().is_empty());
}

// ============================================================================
// Ported from moto: test_mq.py, test_mq_configuration.py,
//                    test_mq_tags.py, test_mq_users.py
// ============================================================================

// Ported from moto: test_mq.py::test_create_broker_minimal
#[tokio::test]
async fn test_create_broker_arn_format() {
    let client = make_mq_client().await;

    let resp = client
        .create_broker()
        .broker_name("arn-check-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    assert!(!resp.broker_id().unwrap().is_empty());
    assert!(resp.broker_arn().unwrap().starts_with("arn:aws"));
}

// Ported from moto: test_mq.py::test_create_with_tags
#[tokio::test]
async fn test_create_broker_with_tags() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("tagged-create-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Rabbitmq)
        .engine_version("3.11.20")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("key1").unwrap(), "val1");
    assert_eq!(tags.get("key2").unwrap(), "val2");
}

// Ported from moto: test_mq.py::test_describe_broker (check state fields)
#[tokio::test]
async fn test_describe_broker_state_fields() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("state-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(true)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.broker_id().unwrap(), broker_id);
    assert!(resp.broker_arn().unwrap().starts_with("arn:aws"));
    assert_eq!(
        resp.broker_state().unwrap(),
        &aws_sdk_mq::types::BrokerState::Running
    );
    assert!(resp.created().is_some());
    assert_eq!(resp.broker_name().unwrap(), "state-broker");
    assert_eq!(
        resp.engine_type().unwrap(),
        &aws_sdk_mq::types::EngineType::Activemq
    );
    assert_eq!(resp.engine_version().unwrap(), "5.17.6");
    assert_eq!(resp.host_instance_type().unwrap(), "mq.m5.large");
    assert_eq!(resp.publicly_accessible(), Some(true));
    assert_eq!(resp.auto_minor_version_upgrade(), Some(false));
}

// Ported from moto: test_mq.py::test_describe_broker_unknown (error message)
#[tokio::test]
async fn test_describe_broker_unknown_error_message() {
    let client = make_mq_client().await;

    let err = client
        .describe_broker()
        .broker_id("unknown")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_mq.py::test_list_brokers_empty
#[tokio::test]
async fn test_list_brokers_empty() {
    let client = make_mq_client().await;

    let resp = client.list_brokers().send().await.unwrap();
    assert_eq!(resp.broker_summaries().len(), 0);
}

// Ported from moto: test_mq.py::test_list_brokers (verify summary fields)
#[tokio::test]
async fn test_list_brokers_summary_fields() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("summary-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(true)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let resp = client.list_brokers().send().await.unwrap();
    assert_eq!(resp.broker_summaries().len(), 1);

    let summary = &resp.broker_summaries()[0];
    assert!(summary.broker_arn().is_some());
    assert_eq!(summary.broker_id().unwrap(), broker_id);
    assert_eq!(summary.broker_name().unwrap(), "summary-broker");
    assert_eq!(
        summary.broker_state().unwrap(),
        &aws_sdk_mq::types::BrokerState::Running
    );
    assert!(summary.created().is_some());
    assert_eq!(summary.host_instance_type().unwrap(), "mq.m5.large");
}

// Ported from moto: test_mq.py::test_delete_broker (verify list empty after delete)
#[tokio::test]
async fn test_delete_broker_and_verify_empty() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("del-verify-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let resp = client
        .delete_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.broker_id().unwrap(), broker_id);

    let list_resp = client.list_brokers().send().await.unwrap();
    assert_eq!(list_resp.broker_summaries().len(), 0);
}

// Ported from moto: test_mq_configuration.py::test_create_configuration_minimal
#[tokio::test]
async fn test_create_configuration_id_and_arn() {
    let client = make_mq_client().await;

    let resp = client
        .create_configuration()
        .name("id-check-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .unwrap();

    let config_id = resp.id().unwrap();
    assert!(config_id.starts_with("c-"));
    assert!(resp.arn().unwrap().contains(":configuration:"));
    assert!(resp.arn().unwrap().contains(config_id));
    assert_eq!(resp.name().unwrap(), "id-check-config");
    assert!(resp.latest_revision().is_some());
    let revision = resp.latest_revision().unwrap();
    assert_eq!(revision.revision(), Some(1));
}

// Ported from moto: test_mq_configuration.py::test_describe_configuration_unknown
#[tokio::test]
async fn test_describe_configuration_unknown_error_message() {
    let client = make_mq_client().await;

    let err = client
        .describe_configuration()
        .configuration_id("c-unknown")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_mq_configuration.py::test_list_configurations_empty
#[tokio::test]
async fn test_list_configurations_empty() {
    let client = make_mq_client().await;

    let resp = client.list_configurations().send().await.unwrap();
    assert_eq!(resp.configurations().len(), 0);
}

// Ported from moto: test_mq_configuration.py::test_list_configurations (check fields)
#[tokio::test]
async fn test_list_configurations_fields() {
    let client = make_mq_client().await;

    let config_id = client
        .create_configuration()
        .name("list-fields-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client.list_configurations().send().await.unwrap();
    assert_eq!(resp.configurations().len(), 1);

    let config = &resp.configurations()[0];
    assert!(config.arn().unwrap().starts_with("arn:aws"));
    assert!(config.created().is_some());
    assert_eq!(config.id().unwrap(), config_id);
    assert_eq!(config.name().unwrap(), "list-fields-config");
    assert_eq!(
        config.engine_type().unwrap(),
        &aws_sdk_mq::types::EngineType::Activemq
    );
    assert_eq!(config.engine_version().unwrap(), "5.17.6");
}

// Ported from moto: test_mq_configuration.py::test_describe_configuration_revision
#[tokio::test]
async fn test_describe_configuration_revision_details() {
    let client = make_mq_client().await;

    let config_id = client
        .create_configuration()
        .name("rev-detail-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_configuration_revision()
        .configuration_id(&config_id)
        .configuration_revision("1")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.configuration_id().unwrap(), config_id);
    assert!(resp.created().is_some());
}

// Ported from moto: test_mq_configuration.py::test_update_configuration
#[tokio::test]
async fn test_update_configuration_fields() {
    let client = make_mq_client().await;

    let config_id = client
        .create_configuration()
        .name("upd-fields-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let resp = client
        .update_configuration()
        .configuration_id(&config_id)
        .data("base64encodedxmlconfig")
        .description("updated config")
        .send()
        .await
        .unwrap();

    assert!(resp.arn().unwrap().starts_with("arn:aws:mq"));
    assert!(resp.created().is_some());
    assert!(resp.id().is_some());
    assert_eq!(resp.name().unwrap(), "upd-fields-config");
    let revision = resp.latest_revision().unwrap();
    assert_eq!(revision.description().unwrap(), "updated config");
    assert_eq!(revision.revision(), Some(2));
}

// Ported from moto: test_mq_tags.py::test_create_tags
#[tokio::test]
async fn test_create_tags_and_verify() {
    let client = make_mq_client().await;

    let resp = client
        .create_broker()
        .broker_name("tag-verify-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Rabbitmq)
        .engine_version("3.11.20")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_arn = resp.broker_arn().unwrap();
    let broker_id = resp.broker_id().unwrap();

    client
        .create_tags()
        .resource_arn(broker_arn)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    // Verify via describe_broker
    let desc_resp = client
        .describe_broker()
        .broker_id(broker_id)
        .send()
        .await
        .unwrap();
    let tags = desc_resp.tags().unwrap();
    assert_eq!(tags.get("key1").unwrap(), "val1");
    assert_eq!(tags.get("key2").unwrap(), "val2");

    // Verify via list_tags
    let tags_resp = client
        .list_tags()
        .resource_arn(broker_arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("key1").unwrap(), "val1");
    assert_eq!(tags.get("key2").unwrap(), "val2");
}

// Ported from moto: test_mq_tags.py::test_delete_tags
#[tokio::test]
async fn test_delete_tags_partial() {
    let client = make_mq_client().await;

    let resp = client
        .create_broker()
        .broker_name("del-tag-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Rabbitmq)
        .engine_version("3.11.20")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_arn = resp.broker_arn().unwrap();
    let broker_id = resp.broker_id().unwrap();

    client
        .create_tags()
        .resource_arn(broker_arn)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    client
        .delete_tags()
        .resource_arn(broker_arn)
        .tag_keys("key1")
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_broker()
        .broker_id(broker_id)
        .send()
        .await
        .unwrap();
    let tags = desc_resp.tags().unwrap();
    assert_eq!(tags.get("key2").unwrap(), "val2");
    assert!(!tags.contains_key("key1"));
}

// Ported from moto: test_mq_tags.py::test_create_configuration_with_tags
#[tokio::test]
async fn test_create_configuration_with_tags() {
    let client = make_mq_client().await;

    let resp = client
        .create_configuration()
        .name("tagged-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    // Describe should return tags
    let desc = client
        .describe_configuration()
        .configuration_id(resp.id().unwrap())
        .send()
        .await
        .unwrap();

    let tags = desc.tags().unwrap();
    assert_eq!(tags.get("key1").unwrap(), "val1");
    assert_eq!(tags.get("key2").unwrap(), "val2");
}

// Ported from moto: test_mq_users.py::test_create_user (verify in describe_broker)
#[tokio::test]
async fn test_create_user_shows_in_broker() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("user-broker-check")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    client
        .create_user()
        .broker_id(&broker_id)
        .username("admin")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();

    let users = resp.users();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].username().unwrap(), "admin");
}

// Ported from moto: test_mq_users.py::test_describe_user (ConsoleAccess, Groups)
#[tokio::test]
async fn test_describe_user_with_groups() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("desc-user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    client
        .create_user()
        .broker_id(&broker_id)
        .username("admin")
        .console_access(true)
        .groups("group1")
        .groups("group2")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_user()
        .broker_id(&broker_id)
        .username("admin")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.broker_id().unwrap(), broker_id);
    assert!(resp.console_access().unwrap());
    assert_eq!(resp.groups().len(), 2);
    assert!(resp.groups().contains(&"group1".to_string()));
    assert!(resp.groups().contains(&"group2".to_string()));
    assert_eq!(resp.username().unwrap(), "admin");
}

// Ported from moto: test_mq_users.py::test_describe_user_unknown
#[tokio::test]
async fn test_describe_user_unknown_error() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("unknown-user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let err = client
        .describe_user()
        .broker_id(&broker_id)
        .username("unknown")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_mq_users.py::test_list_users_empty
#[tokio::test]
async fn test_list_users_empty() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("empty-users-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let resp = client
        .list_users()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.broker_id().unwrap(), broker_id);
    assert_eq!(resp.users().len(), 0);
}

// Ported from moto: test_mq_users.py::test_delete_user (create two users, delete one, verify remaining)
#[tokio::test]
async fn test_delete_user_and_verify_remaining() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("del-user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    client
        .create_user()
        .broker_id(&broker_id)
        .username("admin")
        .send()
        .await
        .unwrap();

    client
        .create_user()
        .broker_id(&broker_id)
        .username("user1")
        .send()
        .await
        .unwrap();

    // Delete admin
    client
        .delete_user()
        .broker_id(&broker_id)
        .username("admin")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_users()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.broker_id().unwrap(), broker_id);
    assert_eq!(resp.users().len(), 1);
    assert_eq!(resp.users()[0].username().unwrap(), "user1");
}

// --- UpdateBroker / UpdateUser tests ---

#[tokio::test]
async fn test_update_broker() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("update-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.15.16")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    let update_resp = client
        .update_broker()
        .broker_id(broker_id)
        .engine_version("5.17.6")
        .auto_minor_version_upgrade(true)
        .send()
        .await
        .expect("update_broker should succeed");

    assert_eq!(update_resp.broker_id().unwrap(), broker_id);
    assert_eq!(update_resp.engine_version().unwrap(), "5.17.6");
    assert_eq!(update_resp.auto_minor_version_upgrade(), Some(true));
}

#[tokio::test]
async fn test_update_broker_not_found() {
    let client = make_mq_client().await;

    let result = client
        .update_broker()
        .broker_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err(), "update nonexistent broker should fail");
}

#[tokio::test]
async fn test_update_user() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_broker()
        .broker_name("update-user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap();

    let broker_id = create_resp.broker_id().unwrap();

    client
        .create_user()
        .broker_id(broker_id)
        .username("updateme")
        .console_access(false)
        .send()
        .await
        .unwrap();

    client
        .update_user()
        .broker_id(broker_id)
        .username("updateme")
        .console_access(true)
        .groups("admins")
        .send()
        .await
        .expect("update_user should succeed");

    let describe_resp = client
        .describe_user()
        .broker_id(broker_id)
        .username("updateme")
        .send()
        .await
        .unwrap();

    assert_eq!(describe_resp.console_access(), Some(true));
    assert_eq!(describe_resp.groups(), &["admins"]);
}

// ============================================================================
// Tests derived from AWS documentation: Amazon MQ
// ============================================================================

// 1. Duplicate broker name → ConflictException
#[tokio::test]
async fn test_create_broker_duplicate() {
    let client = make_mq_client().await;

    client
        .create_broker()
        .broker_name("dup-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .expect("first create_broker should succeed");

    let err = client
        .create_broker()
        .broker_name("dup-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException") || err_str.contains("Conflict"),
        "Expected ConflictException for duplicate broker name, got: {err_str}"
    );
}

// 2. Delete nonexistent broker → NotFoundException
#[tokio::test]
async fn test_delete_broker_not_found() {
    let client = make_mq_client().await;

    let err = client
        .delete_broker()
        .broker_id("nonexistent-broker-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException for deleting nonexistent broker, got: {err_str}"
    );
}

// 3. Delete nonexistent user → NotFoundException
#[tokio::test]
async fn test_delete_nonexistent_user() {
    let client = make_mq_client().await;

    let broker_id = client
        .create_broker()
        .broker_name("del-nonexist-user-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .unwrap()
        .broker_id()
        .unwrap()
        .to_string();

    let err = client
        .delete_user()
        .broker_id(&broker_id)
        .username("ghost-user")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException for deleting nonexistent user, got: {err_str}"
    );
}

// 4. CreateUser for nonexistent broker → NotFoundException
#[tokio::test]
async fn test_create_user_for_nonexistent_broker() {
    let client = make_mq_client().await;

    let err = client
        .create_user()
        .broker_id("no-such-broker")
        .username("someuser")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException when creating user for nonexistent broker, got: {err_str}"
    );
}

// 5. ListUsers for nonexistent broker → NotFoundException
#[tokio::test]
async fn test_list_users_nonexistent_broker() {
    let client = make_mq_client().await;

    let err = client
        .list_users()
        .broker_id("no-such-broker")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException when listing users for nonexistent broker, got: {err_str}"
    );
}

// 6. UpdateConfiguration not found → NotFoundException
#[tokio::test]
async fn test_update_configuration_not_found() {
    let client = make_mq_client().await;

    let err = client
        .update_configuration()
        .configuration_id("c-nonexistent")
        .data("somedata")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException when updating nonexistent configuration, got: {err_str}"
    );
}

// 7. ListConfigurationRevisions not found → NotFoundException
#[tokio::test]
async fn test_list_configuration_revisions_not_found() {
    let client = make_mq_client().await;

    let err = client
        .list_configuration_revisions()
        .configuration_id("c-nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException when listing revisions for nonexistent configuration, got: {err_str}"
    );
}

// 8. Tags lifecycle on configuration resource ARN
#[tokio::test]
async fn test_tags_on_configuration_resource() {
    let client = make_mq_client().await;

    let create_resp = client
        .create_configuration()
        .name("tags-lifecycle-config")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.17.6")
        .send()
        .await
        .expect("create_configuration should succeed");

    let config_arn = create_resp.arn().expect("should have ARN");
    let config_id = create_resp.id().expect("should have id").to_string();

    // Add tags via CreateTags using the config ARN
    client
        .create_tags()
        .resource_arn(config_arn)
        .tags("env", "staging")
        .tags("owner", "platform")
        .send()
        .await
        .expect("create_tags on configuration should succeed");

    // Verify tags via ListTags
    let tags_resp = client
        .list_tags()
        .resource_arn(config_arn)
        .send()
        .await
        .expect("list_tags on configuration should succeed");

    let tags = tags_resp.tags().expect("should have tags map");
    assert_eq!(tags.get("env").unwrap(), "staging");
    assert_eq!(tags.get("owner").unwrap(), "platform");

    // Verify tags appear in DescribeConfiguration
    let desc = client
        .describe_configuration()
        .configuration_id(&config_id)
        .send()
        .await
        .expect("describe_configuration should succeed");
    let desc_tags = desc.tags().expect("describe should return tags");
    assert_eq!(desc_tags.get("env").unwrap(), "staging");

    // Delete one tag
    client
        .delete_tags()
        .resource_arn(config_arn)
        .tag_keys("owner")
        .send()
        .await
        .expect("delete_tags on configuration should succeed");

    // Verify only one tag remains
    let tags_resp2 = client
        .list_tags()
        .resource_arn(config_arn)
        .send()
        .await
        .unwrap();
    let tags2 = tags_resp2.tags().unwrap();
    assert_eq!(tags2.get("env").unwrap(), "staging");
    assert!(!tags2.contains_key("owner"));
}

// 9. DescribeBrokerEngineTypes — verify RabbitMQ engine type present with versions
#[tokio::test]
async fn test_describe_broker_engine_types_rabbitmq() {
    let client = make_mq_client().await;

    let resp = client
        .describe_broker_engine_types()
        .send()
        .await
        .expect("describe_broker_engine_types should succeed");

    let engine_types = resp.broker_engine_types();

    let rabbitmq = engine_types
        .iter()
        .find(|et| et.engine_type() == Some(&aws_sdk_mq::types::EngineType::Rabbitmq))
        .expect("should have RABBITMQ engine type");

    assert!(
        !rabbitmq.engine_versions().is_empty(),
        "RABBITMQ should have at least one engine version"
    );

    // Also confirm ACTIVEMQ is present
    assert!(
        engine_types
            .iter()
            .any(|et| et.engine_type() == Some(&aws_sdk_mq::types::EngineType::Activemq)),
        "ACTIVEMQ engine type should be present"
    );

    assert_eq!(engine_types.len(), 2, "should have exactly 2 engine types");
}

// 10. Full broker lifecycle: create → describe → update → reboot → delete → verify gone
#[tokio::test]
async fn test_full_broker_lifecycle() {
    let client = make_mq_client().await;

    // Create
    let create_resp = client
        .create_broker()
        .broker_name("lifecycle-broker")
        .engine_type(aws_sdk_mq::types::EngineType::Activemq)
        .engine_version("5.15.16")
        .host_instance_type("mq.m5.large")
        .deployment_mode(aws_sdk_mq::types::DeploymentMode::SingleInstance)
        .publicly_accessible(false)
        .auto_minor_version_upgrade(false)
        .send()
        .await
        .expect("create_broker should succeed");

    let broker_id = create_resp.broker_id().unwrap().to_string();
    assert!(!broker_id.is_empty());

    // Describe
    let desc = client
        .describe_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .expect("describe_broker should succeed");
    assert_eq!(desc.broker_name().unwrap(), "lifecycle-broker");
    assert_eq!(desc.engine_version().unwrap(), "5.15.16");

    // Update (engine version upgrade)
    let upd = client
        .update_broker()
        .broker_id(&broker_id)
        .engine_version("5.17.6")
        .auto_minor_version_upgrade(true)
        .send()
        .await
        .expect("update_broker should succeed");
    assert_eq!(upd.engine_version().unwrap(), "5.17.6");
    assert_eq!(upd.auto_minor_version_upgrade(), Some(true));

    // Verify update persisted
    let desc2 = client
        .describe_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.engine_version().unwrap(), "5.17.6");
    assert_eq!(desc2.auto_minor_version_upgrade(), Some(true));

    // Reboot
    client
        .reboot_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .expect("reboot_broker should succeed");

    // Delete
    let del = client
        .delete_broker()
        .broker_id(&broker_id)
        .send()
        .await
        .expect("delete_broker should succeed");
    assert_eq!(del.broker_id().unwrap(), broker_id);

    // Verify gone
    let gone = client.describe_broker().broker_id(&broker_id).send().await;
    assert!(gone.is_err(), "describe after delete should return error");
    let err_str = format!("{:?}", gone.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("NotFound"),
        "Expected NotFoundException after delete, got: {err_str}"
    );
}

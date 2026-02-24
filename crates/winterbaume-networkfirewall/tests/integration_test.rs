//! Integration tests for winterbaume Network Firewall service.

use aws_sdk_networkfirewall::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkfirewall::NetworkFirewallService;

async fn make_client() -> aws_sdk_networkfirewall::Client {
    let mock = MockAws::builder()
        .with_service(NetworkFirewallService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkfirewall::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_networkfirewall::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_firewall() {
    let client = make_client().await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-12345678")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("my-firewall")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-12345678")
        .subnet_mappings(subnet_mapping)
        .send()
        .await
        .expect("create_firewall should succeed");

    let fw = create_resp.firewall().expect("firewall should be present");
    assert_eq!(fw.firewall_name().unwrap(), "my-firewall");
    assert!(fw.firewall_arn().unwrap().contains("my-firewall"));
    assert_eq!(fw.vpc_id(), "vpc-12345678");

    let status = create_resp
        .firewall_status()
        .expect("status should be present");
    assert_eq!(
        status.status(),
        &aws_sdk_networkfirewall::types::FirewallStatusValue::Ready
    );

    // Now describe
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(fw.firewall_arn().unwrap())
        .send()
        .await
        .expect("describe_firewall should succeed");

    let described_fw = describe_resp
        .firewall()
        .expect("firewall should be present");
    assert_eq!(described_fw.firewall_name().unwrap(), "my-firewall");
}

#[tokio::test]
async fn test_list_firewalls() {
    let client = make_client().await;

    for i in 0..3 {
        let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
            .subnet_id(format!("subnet-{i:08}"))
            .build()
            .unwrap();

        client
            .create_firewall()
            .firewall_name(format!("firewall-{i}"))
            .firewall_policy_arn(
                "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
            )
            .vpc_id(format!("vpc-{i:08}"))
            .subnet_mappings(subnet_mapping)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_firewalls()
        .send()
        .await
        .expect("list_firewalls should succeed");

    assert_eq!(resp.firewalls().len(), 3);
}

#[tokio::test]
async fn test_delete_firewall() {
    let client = make_client().await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-del12345")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("del-firewall")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-del12345")
        .subnet_mappings(subnet_mapping)
        .send()
        .await
        .unwrap();

    let fw_arn = create_resp
        .firewall()
        .unwrap()
        .firewall_arn()
        .unwrap()
        .to_string();

    let delete_resp = client
        .delete_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .expect("delete_firewall should succeed");

    let status = delete_resp
        .firewall_status()
        .expect("status should be present");
    assert_eq!(
        status.status(),
        &aws_sdk_networkfirewall::types::FirewallStatusValue::Deleting
    );

    // Verify it's gone
    let list_resp = client.list_firewalls().send().await.unwrap();
    assert_eq!(list_resp.firewalls().len(), 0);
}

#[tokio::test]
async fn test_describe_nonexistent() {
    let client = make_client().await;

    let result = client
        .describe_firewall()
        .firewall_arn("arn:aws:network-firewall:us-east-1:123456789012:firewall/nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe of nonexistent firewall should fail"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Network Firewall
// ============================================================================

// Additional tests added 2026-03-28

async fn create_firewall(
    client: &aws_sdk_networkfirewall::Client,
    name: &str,
    vpc_id: &str,
    subnet_id: &str,
) -> String {
    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id(subnet_id)
        .build()
        .unwrap();
    client
        .create_firewall()
        .firewall_name(name)
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/test-policy",
        )
        .vpc_id(vpc_id)
        .subnet_mappings(subnet_mapping)
        .send()
        .await
        .unwrap()
        .firewall()
        .unwrap()
        .firewall_arn()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_firewall_with_description_and_tags() {
    let client = make_client().await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-aabb1122")
        .build()
        .unwrap();

    let tag = aws_sdk_networkfirewall::types::Tag::builder()
        .key("Env")
        .value("test")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("tagged-firewall")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-aabb1122")
        .subnet_mappings(subnet_mapping)
        .description("A test firewall")
        .tags(tag)
        .send()
        .await
        .expect("create_firewall should succeed");

    let fw = create_resp.firewall().expect("firewall should be present");
    assert_eq!(fw.description(), Some("A test firewall"));
    assert_eq!(fw.tags().len(), 1);
    assert_eq!(fw.tags()[0].key(), "Env");
    assert_eq!(fw.tags()[0].value(), "test");

    // Verify via describe
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(fw.firewall_arn().unwrap())
        .send()
        .await
        .unwrap();
    let described = describe_resp.firewall().unwrap();
    assert_eq!(described.description(), Some("A test firewall"));
    assert_eq!(described.tags().len(), 1);
}

#[tokio::test]
async fn test_create_firewall_duplicate_name() {
    let client = make_client().await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-dup00001")
        .build()
        .unwrap();

    client
        .create_firewall()
        .firewall_name("dup-firewall")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-dup00001")
        .subnet_mappings(subnet_mapping)
        .send()
        .await
        .expect("first create should succeed");

    let subnet_mapping2 = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-dup00002")
        .build()
        .unwrap();

    let err = client
        .create_firewall()
        .firewall_name("dup-firewall")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-dup00002")
        .subnet_mappings(subnet_mapping2)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("InvalidRequestException"),
        "Expected InvalidRequestException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_firewall_by_name() {
    let client = make_client().await;

    create_firewall(&client, "name-lookup-fw", "vpc-name01", "subnet-name01").await;

    let describe_resp = client
        .describe_firewall()
        .firewall_name("name-lookup-fw")
        .send()
        .await
        .expect("describe by name should succeed");

    let fw = describe_resp
        .firewall()
        .expect("firewall should be present");
    assert_eq!(fw.firewall_name().unwrap(), "name-lookup-fw");
}

#[tokio::test]
async fn test_delete_firewall_by_name() {
    let client = make_client().await;

    create_firewall(&client, "name-del-fw", "vpc-namedel", "subnet-namedel").await;

    let delete_resp = client
        .delete_firewall()
        .firewall_name("name-del-fw")
        .send()
        .await
        .expect("delete by name should succeed");

    let status = delete_resp
        .firewall_status()
        .expect("status should be present");
    assert_eq!(
        status.status(),
        &aws_sdk_networkfirewall::types::FirewallStatusValue::Deleting
    );
}

#[tokio::test]
async fn test_delete_nonexistent_firewall() {
    let client = make_client().await;

    let err = client
        .delete_firewall()
        .firewall_arn("arn:aws:network-firewall:us-east-1:123456789012:firewall/does-not-exist")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_firewall_arn_format() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "arn-check-fw", "vpc-arncheck", "subnet-arncheck").await;

    assert!(
        fw_arn.starts_with("arn:aws:network-firewall:"),
        "ARN should start with arn:aws:network-firewall:, got: {fw_arn}"
    );
    assert!(
        fw_arn.contains("arn-check-fw"),
        "ARN should contain the firewall name, got: {fw_arn}"
    );
    // Format: arn:aws:network-firewall:{region}:{account}:firewall/{name}
    assert!(
        fw_arn.contains(":firewall/"),
        "ARN should contain ':firewall/', got: {fw_arn}"
    );
}

#[tokio::test]
async fn test_describe_logging_configuration_empty() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "log-empty-fw", "vpc-logempty", "subnet-logempty").await;

    let resp = client
        .describe_logging_configuration()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .expect("describe_logging_configuration should succeed");

    assert_eq!(resp.firewall_arn(), Some(fw_arn.as_str()));
    let logging = resp
        .logging_configuration()
        .expect("logging_configuration should be present");
    assert_eq!(
        logging.log_destination_configs().len(),
        0,
        "Expected empty log destination configs"
    );
}

#[tokio::test]
async fn test_update_logging_configuration() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "log-update-fw", "vpc-logupd", "subnet-logupd").await;

    let log_dest = {
        let mut m = std::collections::HashMap::new();
        m.insert("bucketName".to_string(), "my-log-bucket".to_string());
        m.insert("prefix".to_string(), "firewall-logs/".to_string());
        m
    };

    let log_dest_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Flow)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::S3)
        .set_log_destination(Some(log_dest))
        .build()
        .unwrap();

    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(log_dest_config)
        .build()
        .unwrap();

    let update_resp = client
        .update_logging_configuration()
        .firewall_arn(&fw_arn)
        .logging_configuration(logging_config)
        .send()
        .await
        .expect("update_logging_configuration should succeed");

    assert_eq!(update_resp.firewall_arn(), Some(fw_arn.as_str()));
    let logging = update_resp
        .logging_configuration()
        .expect("logging_configuration should be present");
    assert_eq!(logging.log_destination_configs().len(), 1);
    assert_eq!(
        logging.log_destination_configs()[0].log_type(),
        &aws_sdk_networkfirewall::types::LogType::Flow
    );
    assert_eq!(
        logging.log_destination_configs()[0].log_destination_type(),
        &aws_sdk_networkfirewall::types::LogDestinationType::S3
    );

    // Verify describe reflects the update
    let describe_resp = client
        .describe_logging_configuration()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    let described_logging = describe_resp.logging_configuration().unwrap();
    assert_eq!(described_logging.log_destination_configs().len(), 1);
    assert_eq!(
        described_logging.log_destination_configs()[0].log_type(),
        &aws_sdk_networkfirewall::types::LogType::Flow
    );
}

#[tokio::test]
async fn test_update_logging_configuration_by_name() {
    let client = make_client().await;

    create_firewall(&client, "log-byname-fw", "vpc-logname", "subnet-logname").await;

    let log_dest_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Alert)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::CloudwatchLogs)
        .log_destination("logGroup", "/aws/network-firewall/alerts")
        .build()
        .unwrap();

    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(log_dest_config)
        .build()
        .unwrap();

    let update_resp = client
        .update_logging_configuration()
        .firewall_name("log-byname-fw")
        .logging_configuration(logging_config)
        .send()
        .await
        .expect("update_logging_configuration by name should succeed");

    assert_eq!(update_resp.firewall_name(), Some("log-byname-fw"));
    assert!(
        update_resp.firewall_arn().is_some(),
        "firewall_arn should be present in response"
    );
}

#[tokio::test]
async fn test_describe_logging_configuration_not_found() {
    let client = make_client().await;

    let err = client
        .describe_logging_configuration()
        .firewall_arn("arn:aws:network-firewall:us-east-1:123456789012:firewall/nonexistent-log-fw")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_logging_configuration_not_found() {
    let client = make_client().await;

    let log_dest_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Flow)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::S3)
        .log_destination("bucketName", "my-bucket")
        .build()
        .unwrap();

    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(log_dest_config)
        .build()
        .unwrap();

    let err = client
        .update_logging_configuration()
        .firewall_arn("arn:aws:network-firewall:us-east-1:123456789012:firewall/ghost-firewall")
        .logging_configuration(logging_config)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_full_firewall_lifecycle() {
    let client = make_client().await;

    // Create
    let fw_arn =
        create_firewall(&client, "lifecycle-fw", "vpc-lifecycle", "subnet-lifecycle").await;

    // Describe
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        describe_resp.firewall().unwrap().firewall_name().unwrap(),
        "lifecycle-fw"
    );

    // Update logging configuration
    let log_dest_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Flow)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::S3)
        .log_destination("bucketName", "lifecycle-bucket")
        .build()
        .unwrap();
    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(log_dest_config)
        .build()
        .unwrap();
    client
        .update_logging_configuration()
        .firewall_arn(&fw_arn)
        .logging_configuration(logging_config)
        .send()
        .await
        .expect("update_logging_configuration should succeed");

    // Verify logging config persisted
    let log_resp = client
        .describe_logging_configuration()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        log_resp
            .logging_configuration()
            .unwrap()
            .log_destination_configs()
            .len(),
        1
    );

    // Delete
    let delete_resp = client
        .delete_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(
        delete_resp.firewall_status().unwrap().status(),
        &aws_sdk_networkfirewall::types::FirewallStatusValue::Deleting
    );

    // Verify gone
    let err = client
        .describe_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_firewall_returns_firewall_id() {
    let client = make_client().await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-id00001")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("id-check-fw")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-id00001")
        .subnet_mappings(subnet_mapping)
        .send()
        .await
        .expect("create_firewall should succeed");

    let fw = create_resp.firewall().expect("firewall should be present");
    let firewall_id = fw.firewall_id();
    assert!(
        !firewall_id.is_empty(),
        "FirewallId should be non-empty, got: {firewall_id:?}"
    );
}

#[tokio::test]
async fn test_create_firewall_with_delete_protection() {
    let client = make_client().await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-delprot1")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("delprot-fw")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-delprot1")
        .subnet_mappings(subnet_mapping)
        .delete_protection(true)
        .send()
        .await
        .expect("create_firewall should succeed");

    let fw = create_resp.firewall().expect("firewall should be present");
    assert!(fw.delete_protection(), "delete_protection should be true");

    // Also verify via describe
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(fw.firewall_arn().unwrap())
        .send()
        .await
        .unwrap();
    assert!(
        describe_resp.firewall().unwrap().delete_protection(),
        "describe should reflect delete_protection=true"
    );
}

#[tokio::test]
async fn test_create_firewall_multiple_subnets() {
    let client = make_client().await;

    let subnet1 = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-multi001")
        .build()
        .unwrap();
    let subnet2 = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-multi002")
        .build()
        .unwrap();
    let subnet3 = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-multi003")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("multi-subnet-fw")
        .firewall_policy_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
        )
        .vpc_id("vpc-multisub")
        .subnet_mappings(subnet1)
        .subnet_mappings(subnet2)
        .subnet_mappings(subnet3)
        .send()
        .await
        .expect("create_firewall with multiple subnets should succeed");

    let fw = create_resp.firewall().expect("firewall should be present");
    assert_eq!(
        fw.subnet_mappings().len(),
        3,
        "Should have 3 subnet mappings, got: {}",
        fw.subnet_mappings().len()
    );

    // Verify via describe as well
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(fw.firewall_arn().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.firewall().unwrap().subnet_mappings().len(),
        3,
        "describe should return 3 subnet mappings"
    );
}

#[tokio::test]
async fn test_list_firewalls_empty() {
    let client = make_client().await;

    let resp = client
        .list_firewalls()
        .send()
        .await
        .expect("list_firewalls should succeed on empty state");

    assert_eq!(
        resp.firewalls().len(),
        0,
        "Expected 0 firewalls on fresh state, got: {}",
        resp.firewalls().len()
    );
}

#[tokio::test]
async fn test_describe_firewall_returns_policy_arn() {
    let client = make_client().await;

    let policy_arn =
        "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/specific-policy";

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-pol00001")
        .build()
        .unwrap();

    let create_resp = client
        .create_firewall()
        .firewall_name("policy-arn-fw")
        .firewall_policy_arn(policy_arn)
        .vpc_id("vpc-pol00001")
        .subnet_mappings(subnet_mapping)
        .send()
        .await
        .expect("create_firewall should succeed");

    let fw_arn = create_resp
        .firewall()
        .unwrap()
        .firewall_arn()
        .unwrap()
        .to_string();

    let describe_resp = client
        .describe_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();

    let described_fw = describe_resp.firewall().unwrap();
    assert_eq!(
        described_fw.firewall_policy_arn(),
        policy_arn,
        "firewall_policy_arn should match what was provided at create"
    );
}

#[tokio::test]
async fn test_update_logging_config_clear() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "log-clear-fw", "vpc-logclear", "subnet-logclear").await;

    // First, set a logging destination
    let log_dest_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Flow)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::S3)
        .log_destination("bucketName", "my-clear-bucket")
        .build()
        .unwrap();
    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(log_dest_config)
        .build()
        .unwrap();
    client
        .update_logging_configuration()
        .firewall_arn(&fw_arn)
        .logging_configuration(logging_config)
        .send()
        .await
        .expect("initial update_logging_configuration should succeed");

    // Now clear the logging configuration by sending an empty LogDestinationConfigs.
    // The SDK requires at least specifying the field; pass an empty collection by
    // constructing directly rather than using the builder's required-field check.
    let empty_logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .set_log_destination_configs(Some(vec![]))
        .build()
        .unwrap();
    let update_resp = client
        .update_logging_configuration()
        .firewall_arn(&fw_arn)
        .logging_configuration(empty_logging_config)
        .send()
        .await
        .expect("clearing update_logging_configuration should succeed");

    let logging = update_resp
        .logging_configuration()
        .expect("logging_configuration should be present");
    assert_eq!(
        logging.log_destination_configs().len(),
        0,
        "Expected empty log destination configs after clearing"
    );

    // Verify describe reflects the cleared config
    let describe_resp = client
        .describe_logging_configuration()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp
            .logging_configuration()
            .unwrap()
            .log_destination_configs()
            .len(),
        0,
        "describe should return empty log configs after clearing"
    );
}

#[tokio::test]
async fn test_update_logging_config_kinesis() {
    let client = make_client().await;

    let fw_arn = create_firewall(
        &client,
        "log-kinesis-fw",
        "vpc-logkinesis",
        "subnet-logkinesis",
    )
    .await;

    let mut log_dest = std::collections::HashMap::new();
    log_dest.insert(
        "deliveryStream".to_string(),
        "my-delivery-stream".to_string(),
    );

    let log_dest_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Alert)
        .log_destination_type(
            aws_sdk_networkfirewall::types::LogDestinationType::KinesisDataFirehose,
        )
        .set_log_destination(Some(log_dest))
        .build()
        .unwrap();

    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(log_dest_config)
        .build()
        .unwrap();

    let update_resp = client
        .update_logging_configuration()
        .firewall_arn(&fw_arn)
        .logging_configuration(logging_config)
        .send()
        .await
        .expect("update_logging_configuration with Kinesis should succeed");

    let logging = update_resp
        .logging_configuration()
        .expect("logging_configuration should be present");
    assert_eq!(logging.log_destination_configs().len(), 1);
    assert_eq!(
        logging.log_destination_configs()[0].log_destination_type(),
        &aws_sdk_networkfirewall::types::LogDestinationType::KinesisDataFirehose,
        "log_destination_type should be KinesisDataFirehose"
    );
    assert_eq!(
        logging.log_destination_configs()[0].log_type(),
        &aws_sdk_networkfirewall::types::LogType::Alert,
        "log_type should be Alert"
    );
}

#[tokio::test]
async fn test_update_logging_config_multiple_destinations() {
    let client = make_client().await;

    let fw_arn = create_firewall(
        &client,
        "log-multi-dest-fw",
        "vpc-logmulti",
        "subnet-logmulti",
    )
    .await;

    let flow_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Flow)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::S3)
        .log_destination("bucketName", "flow-bucket")
        .build()
        .unwrap();

    let alert_config = aws_sdk_networkfirewall::types::LogDestinationConfig::builder()
        .log_type(aws_sdk_networkfirewall::types::LogType::Alert)
        .log_destination_type(aws_sdk_networkfirewall::types::LogDestinationType::CloudwatchLogs)
        .log_destination("logGroup", "/aws/network-firewall/alerts")
        .build()
        .unwrap();

    let logging_config = aws_sdk_networkfirewall::types::LoggingConfiguration::builder()
        .log_destination_configs(flow_config)
        .log_destination_configs(alert_config)
        .build()
        .unwrap();

    let update_resp = client
        .update_logging_configuration()
        .firewall_arn(&fw_arn)
        .logging_configuration(logging_config)
        .send()
        .await
        .expect("update_logging_configuration with multiple destinations should succeed");

    let logging = update_resp
        .logging_configuration()
        .expect("logging_configuration should be present");
    assert_eq!(
        logging.log_destination_configs().len(),
        2,
        "Expected 2 log destination configs, got: {}",
        logging.log_destination_configs().len()
    );

    // Verify describe also shows both
    let describe_resp = client
        .describe_logging_configuration()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp
            .logging_configuration()
            .unwrap()
            .log_destination_configs()
            .len(),
        2,
        "describe should return 2 log destinations"
    );
}

// ============================================================================
// Rule group tests
// ============================================================================

async fn make_client_for_rule_groups() -> (
    aws_sdk_networkfirewall::Client,
    winterbaume_networkfirewall::NetworkFirewallService,
) {
    use winterbaume_core::MockAws;
    let svc = winterbaume_networkfirewall::NetworkFirewallService::new();
    let mock = MockAws::builder()
        .with_service(winterbaume_networkfirewall::NetworkFirewallService::new())
        .build();
    let config = aws_config::defaults(aws_sdk_networkfirewall::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkfirewall::config::Region::new("us-east-1"))
        .load()
        .await;
    (aws_sdk_networkfirewall::Client::new(&config), svc)
}

#[tokio::test]
async fn test_create_and_describe_rule_group() {
    let client = make_client().await;

    let resp = client
        .create_rule_group()
        .rule_group_name("my-rule-group")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .capacity(100)
        .send()
        .await
        .expect("create_rule_group should succeed");

    let rg_resp = resp
        .rule_group_response()
        .expect("rule_group_response should be present");
    assert_eq!(rg_resp.rule_group_name(), "my-rule-group");
    assert!(rg_resp.rule_group_arn().contains("my-rule-group"));
    assert_eq!(rg_resp.capacity(), Some(100));

    // Describe by name
    let describe_resp = client
        .describe_rule_group()
        .rule_group_name("my-rule-group")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .send()
        .await
        .expect("describe_rule_group should succeed");

    let described = describe_resp
        .rule_group_response()
        .expect("rule_group_response should be present");
    assert_eq!(described.rule_group_name(), "my-rule-group");
}

#[tokio::test]
async fn test_list_rule_groups() {
    let client = make_client().await;

    for i in 0..3 {
        client
            .create_rule_group()
            .rule_group_name(format!("rg-{i}"))
            .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
            .capacity(50)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_rule_groups()
        .send()
        .await
        .expect("list_rule_groups should succeed");

    assert_eq!(resp.rule_groups().len(), 3, "expected 3 rule groups");
}

#[tokio::test]
async fn test_delete_rule_group() {
    let client = make_client().await;

    client
        .create_rule_group()
        .rule_group_name("del-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateless)
        .capacity(50)
        .send()
        .await
        .unwrap();

    client
        .delete_rule_group()
        .rule_group_name("del-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateless)
        .send()
        .await
        .expect("delete_rule_group should succeed");

    // Verify it's gone
    let err = client
        .describe_rule_group()
        .rule_group_name("del-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateless)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_rule_group() {
    let client = make_client().await;

    client
        .create_rule_group()
        .rule_group_name("upd-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .capacity(100)
        .send()
        .await
        .unwrap();

    // UpdateRuleGroup requires an UpdateToken — use a dummy one for the mock
    let upd_resp = client
        .update_rule_group()
        .rule_group_name("upd-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .description("updated description")
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_rule_group should succeed");

    let rg_resp = upd_resp
        .rule_group_response()
        .expect("rule_group_response should be present");
    assert_eq!(rg_resp.description().unwrap_or(""), "updated description");
}

#[tokio::test]
async fn test_describe_rule_group_not_found() {
    let client = make_client().await;

    let err = client
        .describe_rule_group()
        .rule_group_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:stateful-rulegroup/nonexistent",
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Firewall policy tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_firewall_policy() {
    let client = make_client().await;

    let policy = aws_sdk_networkfirewall::types::FirewallPolicy::builder()
        .stateless_default_actions("aws:pass")
        .stateless_fragment_default_actions("aws:drop")
        .build()
        .unwrap();

    let resp = client
        .create_firewall_policy()
        .firewall_policy_name("my-policy")
        .firewall_policy(policy)
        .send()
        .await
        .expect("create_firewall_policy should succeed");

    let fp_resp = resp
        .firewall_policy_response()
        .expect("firewall_policy_response should be present");
    assert_eq!(fp_resp.firewall_policy_name(), "my-policy");
    assert!(fp_resp.firewall_policy_arn().contains("my-policy"));

    // Describe by name
    let describe_resp = client
        .describe_firewall_policy()
        .firewall_policy_name("my-policy")
        .send()
        .await
        .expect("describe_firewall_policy should succeed");

    let described = describe_resp
        .firewall_policy_response()
        .expect("firewall_policy_response should be present");
    assert_eq!(described.firewall_policy_name(), "my-policy");
}

#[tokio::test]
async fn test_list_firewall_policies() {
    let client = make_client().await;

    for i in 0..3 {
        let policy = aws_sdk_networkfirewall::types::FirewallPolicy::builder()
            .stateless_default_actions("aws:pass")
            .stateless_fragment_default_actions("aws:drop")
            .build()
            .unwrap();
        client
            .create_firewall_policy()
            .firewall_policy_name(format!("policy-{i}"))
            .firewall_policy(policy)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_firewall_policies()
        .send()
        .await
        .expect("list_firewall_policies should succeed");

    assert_eq!(resp.firewall_policies().len(), 3);
}

#[tokio::test]
async fn test_delete_firewall_policy() {
    let client = make_client().await;

    let policy = aws_sdk_networkfirewall::types::FirewallPolicy::builder()
        .stateless_default_actions("aws:pass")
        .stateless_fragment_default_actions("aws:drop")
        .build()
        .unwrap();

    client
        .create_firewall_policy()
        .firewall_policy_name("del-policy")
        .firewall_policy(policy)
        .send()
        .await
        .unwrap();

    client
        .delete_firewall_policy()
        .firewall_policy_name("del-policy")
        .send()
        .await
        .expect("delete_firewall_policy should succeed");

    let err = client
        .describe_firewall_policy()
        .firewall_policy_name("del-policy")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_firewall_policy() {
    let client = make_client().await;

    let policy = aws_sdk_networkfirewall::types::FirewallPolicy::builder()
        .stateless_default_actions("aws:pass")
        .stateless_fragment_default_actions("aws:drop")
        .build()
        .unwrap();

    client
        .create_firewall_policy()
        .firewall_policy_name("upd-policy")
        .firewall_policy(policy)
        .send()
        .await
        .unwrap();

    let updated_policy = aws_sdk_networkfirewall::types::FirewallPolicy::builder()
        .stateless_default_actions("aws:drop")
        .stateless_fragment_default_actions("aws:drop")
        .build()
        .unwrap();

    let upd_resp = client
        .update_firewall_policy()
        .firewall_policy_name("upd-policy")
        .firewall_policy(updated_policy)
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_firewall_policy should succeed");

    let fp_resp = upd_resp
        .firewall_policy_response()
        .expect("firewall_policy_response should be present");
    assert_eq!(fp_resp.firewall_policy_name(), "upd-policy");
}

// ============================================================================
// Tags tests
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags_for_firewall() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "tag-test-fw", "vpc-tag001", "subnet-tag001").await;

    // Tag the firewall
    let tag = aws_sdk_networkfirewall::types::Tag::builder()
        .key("Environment")
        .value("prod")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn(&fw_arn)
        .tags(tag)
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&fw_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "Environment");
    assert_eq!(tags[0].value(), "prod");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "untag-test-fw", "vpc-untag", "subnet-untag").await;

    // Add tags first
    let tag1 = aws_sdk_networkfirewall::types::Tag::builder()
        .key("Env")
        .value("test")
        .build()
        .unwrap();
    let tag2 = aws_sdk_networkfirewall::types::Tag::builder()
        .key("Team")
        .value("platform")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn(&fw_arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    // Verify 2 tags
    let before = client
        .list_tags_for_resource()
        .resource_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(before.tags().len(), 2);

    // Untag one
    client
        .untag_resource()
        .resource_arn(&fw_arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify 1 tag remains
    let after = client
        .list_tags_for_resource()
        .resource_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(after.tags().len(), 1);
    assert_eq!(after.tags()[0].key(), "Team");
}

#[tokio::test]
async fn test_tag_rule_group() {
    let client = make_client().await;

    let create_resp = client
        .create_rule_group()
        .rule_group_name("tagged-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .capacity(100)
        .send()
        .await
        .unwrap();

    let rg_arn = create_resp
        .rule_group_response()
        .unwrap()
        .rule_group_arn()
        .to_string();

    let tag = aws_sdk_networkfirewall::types::Tag::builder()
        .key("Purpose")
        .value("security")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn(&rg_arn)
        .tags(tag)
        .send()
        .await
        .expect("tag_resource should succeed for rule group");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&rg_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "Purpose");
}

// ============================================================================
// Associate/Disassociate subnet tests
// ============================================================================

#[tokio::test]
async fn test_associate_and_disassociate_subnets() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "subnet-mod-fw", "vpc-submod", "subnet-orig001").await;

    // Add a subnet
    let new_subnet = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-new001")
        .build()
        .unwrap();

    let assoc_resp = client
        .associate_subnets()
        .firewall_arn(&fw_arn)
        .subnet_mappings(new_subnet)
        .update_token("dummy-token")
        .send()
        .await
        .expect("associate_subnets should succeed");

    assert_eq!(assoc_resp.subnet_mappings().len(), 2);

    // Disassociate the original subnet
    let disassoc_resp = client
        .disassociate_subnets()
        .firewall_arn(&fw_arn)
        .subnet_ids("subnet-orig001")
        .update_token("dummy-token")
        .send()
        .await
        .expect("disassociate_subnets should succeed");

    assert_eq!(disassoc_resp.subnet_mappings().len(), 1);
    assert_eq!(
        disassoc_resp.subnet_mappings()[0].subnet_id(),
        "subnet-new001"
    );
}

// ============================================================================
// Associate firewall policy test
// ============================================================================

#[tokio::test]
async fn test_associate_firewall_policy() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "assoc-pol-fw", "vpc-assocpol", "subnet-assocpol").await;

    let new_policy_arn =
        "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/new-policy";

    let resp = client
        .associate_firewall_policy()
        .firewall_arn(&fw_arn)
        .firewall_policy_arn(new_policy_arn)
        .update_token("dummy-token")
        .send()
        .await
        .expect("associate_firewall_policy should succeed");

    assert_eq!(resp.firewall_policy_arn(), Some(new_policy_arn));

    // Verify describe returns the new policy ARN
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.firewall().unwrap().firewall_policy_arn(),
        new_policy_arn
    );
}

// ============================================================================
// UpdateFirewallDescription and UpdateFirewallDeleteProtection tests
// ============================================================================

#[tokio::test]
async fn test_update_firewall_description() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "desc-update-fw", "vpc-descupd", "subnet-descupd").await;

    let resp = client
        .update_firewall_description()
        .firewall_arn(&fw_arn)
        .description("new description")
        .send()
        .await
        .expect("update_firewall_description should succeed");

    assert_eq!(resp.description(), Some("new description"));

    // Verify via describe
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.firewall().unwrap().description(),
        Some("new description")
    );
}

#[tokio::test]
async fn test_update_firewall_delete_protection() {
    let client = make_client().await;

    let fw_arn = create_firewall(
        &client,
        "delprot-upd-fw",
        "vpc-delprotupd",
        "subnet-delprotupd",
    )
    .await;

    let resp = client
        .update_firewall_delete_protection()
        .firewall_arn(&fw_arn)
        .delete_protection(true)
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_firewall_delete_protection should succeed");

    assert!(resp.delete_protection());

    // Verify via describe
    let describe_resp = client
        .describe_firewall()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert!(describe_resp.firewall().unwrap().delete_protection());
}

// ============================================================================
// Resource policy tests
// ============================================================================

#[tokio::test]
async fn test_put_describe_delete_resource_policy() {
    let client = make_client().await;

    let create_resp = client
        .create_rule_group()
        .rule_group_name("policy-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .capacity(100)
        .send()
        .await
        .unwrap();

    let rg_arn = create_resp
        .rule_group_response()
        .unwrap()
        .rule_group_arn()
        .to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"arn:aws:iam::999999999999:root"},"Action":["network-firewall:CreateFirewallPolicy"],"Resource":"*"}]}"#;

    // Put policy
    client
        .put_resource_policy()
        .resource_arn(&rg_arn)
        .policy(policy_doc)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    // Describe policy
    let describe_resp = client
        .describe_resource_policy()
        .resource_arn(&rg_arn)
        .send()
        .await
        .expect("describe_resource_policy should succeed");

    assert_eq!(describe_resp.policy(), Some(policy_doc));

    // Delete policy
    client
        .delete_resource_policy()
        .resource_arn(&rg_arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    // Verify it's gone
    let err = client
        .describe_resource_policy()
        .resource_arn(&rg_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// TLS Inspection Configuration tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_tls_inspection_configuration() {
    let client = make_client().await;

    let tls_config = aws_sdk_networkfirewall::types::TlsInspectionConfiguration::builder().build();

    let resp = client
        .create_tls_inspection_configuration()
        .tls_inspection_configuration_name("my-tls-config")
        .tls_inspection_configuration(tls_config)
        .send()
        .await
        .expect("create_tls_inspection_configuration should succeed");

    let tls_resp = resp
        .tls_inspection_configuration_response()
        .expect("tls_inspection_configuration_response should be present");
    assert_eq!(
        tls_resp.tls_inspection_configuration_name(),
        "my-tls-config"
    );
    assert!(
        tls_resp
            .tls_inspection_configuration_arn()
            .contains("my-tls-config"),
        "ARN should contain configuration name"
    );

    let arn = tls_resp.tls_inspection_configuration_arn().to_string();

    // Describe by ARN
    let describe_resp = client
        .describe_tls_inspection_configuration()
        .tls_inspection_configuration_arn(&arn)
        .send()
        .await
        .expect("describe_tls_inspection_configuration should succeed");

    let described = describe_resp
        .tls_inspection_configuration_response()
        .expect("tls_inspection_configuration_response should be present");
    assert_eq!(
        described.tls_inspection_configuration_name(),
        "my-tls-config"
    );
}

#[tokio::test]
async fn test_list_tls_inspection_configurations() {
    let client = make_client().await;

    for i in 0..3 {
        let tls_config =
            aws_sdk_networkfirewall::types::TlsInspectionConfiguration::builder().build();
        client
            .create_tls_inspection_configuration()
            .tls_inspection_configuration_name(format!("tls-config-{i}"))
            .tls_inspection_configuration(tls_config)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_tls_inspection_configurations()
        .send()
        .await
        .expect("list_tls_inspection_configurations should succeed");

    assert_eq!(
        resp.tls_inspection_configurations().len(),
        3,
        "expected 3 TLS inspection configurations"
    );
}

#[tokio::test]
async fn test_delete_tls_inspection_configuration() {
    let client = make_client().await;

    let tls_config = aws_sdk_networkfirewall::types::TlsInspectionConfiguration::builder().build();

    let create_resp = client
        .create_tls_inspection_configuration()
        .tls_inspection_configuration_name("del-tls-config")
        .tls_inspection_configuration(tls_config)
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .tls_inspection_configuration_response()
        .unwrap()
        .tls_inspection_configuration_arn()
        .to_string();

    client
        .delete_tls_inspection_configuration()
        .tls_inspection_configuration_arn(&arn)
        .send()
        .await
        .expect("delete_tls_inspection_configuration should succeed");

    // Verify it's gone
    let err = client
        .describe_tls_inspection_configuration()
        .tls_inspection_configuration_arn(&arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_tls_inspection_configuration() {
    let client = make_client().await;

    let tls_config = aws_sdk_networkfirewall::types::TlsInspectionConfiguration::builder().build();

    let create_resp = client
        .create_tls_inspection_configuration()
        .tls_inspection_configuration_name("upd-tls-config")
        .tls_inspection_configuration(tls_config)
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .tls_inspection_configuration_response()
        .unwrap()
        .tls_inspection_configuration_arn()
        .to_string();

    let updated_config =
        aws_sdk_networkfirewall::types::TlsInspectionConfiguration::builder().build();

    let upd_resp = client
        .update_tls_inspection_configuration()
        .tls_inspection_configuration_arn(&arn)
        .tls_inspection_configuration(updated_config)
        .description("updated tls config")
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_tls_inspection_configuration should succeed");

    let tls_resp = upd_resp
        .tls_inspection_configuration_response()
        .expect("tls_inspection_configuration_response should be present");
    assert_eq!(
        tls_resp.tls_inspection_configuration_name(),
        "upd-tls-config"
    );
    assert_eq!(tls_resp.description(), Some("updated tls config"));
}

#[tokio::test]
async fn test_describe_tls_inspection_configuration_not_found() {
    let client = make_client().await;

    let err = client
        .describe_tls_inspection_configuration()
        .tls_inspection_configuration_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:tls-configuration/nonexistent",
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// VPC Endpoint Association tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_vpc_endpoint_association() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "vpc-ep-fw", "vpc-ep001", "subnet-ep001").await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-assoc001")
        .build()
        .unwrap();

    let resp = client
        .create_vpc_endpoint_association()
        .firewall_arn(&fw_arn)
        .vpc_id("vpc-ep001")
        .subnet_mapping(subnet_mapping)
        .send()
        .await
        .expect("create_vpc_endpoint_association should succeed");

    let assoc = resp
        .vpc_endpoint_association()
        .expect("vpc_endpoint_association should be present");
    assert!(
        assoc
            .vpc_endpoint_association_arn()
            .contains("vpc-endpoint-association"),
        "ARN should contain 'vpc-endpoint-association'"
    );
    assert_eq!(assoc.firewall_arn(), fw_arn.as_str());

    let assoc_arn = assoc.vpc_endpoint_association_arn().to_string();

    // Describe by ARN
    let describe_resp = client
        .describe_vpc_endpoint_association()
        .vpc_endpoint_association_arn(&assoc_arn)
        .send()
        .await
        .expect("describe_vpc_endpoint_association should succeed");

    let described = describe_resp
        .vpc_endpoint_association()
        .expect("vpc_endpoint_association should be present");
    assert_eq!(described.vpc_endpoint_association_arn(), assoc_arn.as_str());
}

#[tokio::test]
async fn test_list_vpc_endpoint_associations() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "vpc-ep-list-fw", "vpc-eplist", "subnet-eplist").await;

    for i in 0..2 {
        let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
            .subnet_id(format!("subnet-list{i:03}"))
            .build()
            .unwrap();
        client
            .create_vpc_endpoint_association()
            .firewall_arn(&fw_arn)
            .vpc_id("vpc-eplist")
            .subnet_mapping(subnet_mapping)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_vpc_endpoint_associations()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .expect("list_vpc_endpoint_associations should succeed");

    assert_eq!(
        resp.vpc_endpoint_associations().len(),
        2,
        "expected 2 VPC endpoint associations"
    );
}

#[tokio::test]
async fn test_delete_vpc_endpoint_association() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "vpc-ep-del-fw", "vpc-epdel", "subnet-epdel").await;

    let subnet_mapping = aws_sdk_networkfirewall::types::SubnetMapping::builder()
        .subnet_id("subnet-del001")
        .build()
        .unwrap();

    let create_resp = client
        .create_vpc_endpoint_association()
        .firewall_arn(&fw_arn)
        .vpc_id("vpc-epdel")
        .subnet_mapping(subnet_mapping)
        .send()
        .await
        .unwrap();

    let assoc_arn = create_resp
        .vpc_endpoint_association()
        .unwrap()
        .vpc_endpoint_association_arn()
        .to_string();

    client
        .delete_vpc_endpoint_association()
        .vpc_endpoint_association_arn(&assoc_arn)
        .send()
        .await
        .expect("delete_vpc_endpoint_association should succeed");

    // Verify it's gone
    let list_resp = client
        .list_vpc_endpoint_associations()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        list_resp.vpc_endpoint_associations().len(),
        0,
        "expected 0 associations after delete"
    );
}

#[tokio::test]
async fn test_describe_vpc_endpoint_association_not_found() {
    let client = make_client().await;

    let err = client
        .describe_vpc_endpoint_association()
        .vpc_endpoint_association_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:vpc-endpoint-association/nonexistent",
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Protection flag update tests
// ============================================================================

#[tokio::test]
async fn test_update_subnet_change_protection() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "scp-fw", "vpc-scp", "subnet-scp").await;

    let resp = client
        .update_subnet_change_protection()
        .firewall_arn(&fw_arn)
        .subnet_change_protection(true)
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_subnet_change_protection should succeed");

    assert!(resp.subnet_change_protection());
    assert_eq!(resp.firewall_arn(), Some(fw_arn.as_str()));
}

#[tokio::test]
async fn test_update_firewall_policy_change_protection() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "fpcp-fw", "vpc-fpcp", "subnet-fpcp").await;

    let resp = client
        .update_firewall_policy_change_protection()
        .firewall_arn(&fw_arn)
        .firewall_policy_change_protection(true)
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_firewall_policy_change_protection should succeed");

    assert!(resp.firewall_policy_change_protection());
    assert_eq!(resp.firewall_arn(), Some(fw_arn.as_str()));
}

#[tokio::test]
async fn test_update_availability_zone_change_protection() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "azcp-fw", "vpc-azcp", "subnet-azcp").await;

    let resp = client
        .update_availability_zone_change_protection()
        .firewall_arn(&fw_arn)
        .availability_zone_change_protection(true)
        .update_token("dummy-token")
        .send()
        .await
        .expect("update_availability_zone_change_protection should succeed");

    assert!(resp.availability_zone_change_protection());
    assert_eq!(resp.firewall_arn(), Some(fw_arn.as_str()));
}

// ============================================================================
// DescribeFirewallMetadata, DescribeRuleGroupMetadata, DescribeRuleGroupSummary tests
// ============================================================================

#[tokio::test]
async fn test_describe_firewall_metadata() {
    let client = make_client().await;

    let fw_arn = create_firewall(&client, "meta-fw", "vpc-meta", "subnet-meta").await;

    let resp = client
        .describe_firewall_metadata()
        .firewall_arn(&fw_arn)
        .send()
        .await
        .expect("describe_firewall_metadata should succeed");

    assert_eq!(resp.firewall_arn(), Some(fw_arn.as_str()));
    assert_eq!(resp.status().map(|s| s.as_str()), Some("READY"));
}

#[tokio::test]
async fn test_describe_rule_group_metadata() {
    let client = make_client().await;

    let create_resp = client
        .create_rule_group()
        .rule_group_name("meta-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateful)
        .capacity(200)
        .description("test rule group")
        .send()
        .await
        .unwrap();

    let rg_arn = create_resp
        .rule_group_response()
        .unwrap()
        .rule_group_arn()
        .to_string();

    let resp = client
        .describe_rule_group_metadata()
        .rule_group_arn(&rg_arn)
        .send()
        .await
        .expect("describe_rule_group_metadata should succeed");

    assert_eq!(resp.rule_group_arn(), rg_arn.as_str());
    assert_eq!(resp.rule_group_name(), "meta-rg");
    assert_eq!(resp.capacity(), Some(200));
    assert_eq!(resp.description(), Some("test rule group"));
}

#[tokio::test]
async fn test_describe_rule_group_summary() {
    let client = make_client().await;

    client
        .create_rule_group()
        .rule_group_name("summary-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateless)
        .capacity(50)
        .description("summary test")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_rule_group_summary()
        .rule_group_name("summary-rg")
        .r#type(aws_sdk_networkfirewall::types::RuleGroupType::Stateless)
        .send()
        .await
        .expect("describe_rule_group_summary should succeed");

    assert_eq!(resp.rule_group_name(), "summary-rg");
    assert_eq!(resp.description(), Some("summary test"));
}

#[tokio::test]
async fn test_describe_firewall_metadata_not_found() {
    let client = make_client().await;

    let err = client
        .describe_firewall_metadata()
        .firewall_arn(
            "arn:aws:network-firewall:us-east-1:123456789012:firewall/nonexistent-meta-fw",
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

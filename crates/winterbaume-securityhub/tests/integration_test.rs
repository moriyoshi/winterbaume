use aws_sdk_securityhub::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_securityhub::SecurityHubService;

async fn make_client() -> aws_sdk_securityhub::Client {
    let mock = MockAws::builder()
        .with_service(SecurityHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_securityhub::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_securityhub::Client::new(&config)
}

async fn make_client_region(region: &str) -> aws_sdk_securityhub::Client {
    let mock = MockAws::builder()
        .with_service(SecurityHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_securityhub::config::Region::new(region.to_string()))
        .load()
        .await;

    aws_sdk_securityhub::Client::new(&config)
}

fn make_finding(
    id: &str,
    region: &str,
    description: &str,
    title: &str,
    resource_id: &str,
) -> aws_sdk_securityhub::types::AwsSecurityFinding {
    use aws_sdk_securityhub::types::{AwsSecurityFinding, Resource, Severity, SeverityLabel};

    AwsSecurityFinding::builder()
        .aws_account_id("123456789012")
        .created_at("2024-01-01T00:00:00.000Z")
        .updated_at("2024-01-01T00:00:00.000Z")
        .description(description)
        .generator_id("test-generator")
        .id(id)
        .product_arn(format!(
            "arn:aws:securityhub:{region}:123456789012:product/123456789012/default"
        ))
        .resources(
            Resource::builder()
                .id(resource_id)
                .r#type("AwsEc2Instance")
                .build(),
        )
        .schema_version("2018-10-08")
        .severity(Severity::builder().label(SeverityLabel::High).build())
        .title(title)
        .types("Software and Configuration Checks")
        .build()
}

/// Translated from moto: test_enable_security_hub
#[tokio::test]
async fn test_enable_security_hub() {
    let client = make_client().await;

    // Before enabling, describe_hub should fail
    let result = client.describe_hub().send().await;
    assert!(result.is_err(), "describe_hub should fail before enabling");

    // Enable security hub
    let resp = client.enable_security_hub().send().await;
    assert!(resp.is_ok(), "enable_security_hub should succeed");

    // Describe hub should now succeed
    let hub_info = client
        .describe_hub()
        .send()
        .await
        .expect("describe_hub should succeed after enabling");
    let hub_arn = hub_info.hub_arn().expect("HubArn should be present");
    assert_eq!(
        hub_arn,
        "arn:aws:securityhub:us-east-1:123456789012:hub/default"
    );
    assert!(
        hub_info.subscribed_at().is_some(),
        "SubscribedAt should be present"
    );

    // Enabling again should succeed (idempotent)
    let resp = client.enable_security_hub().send().await;
    assert!(resp.is_ok(), "enable_security_hub should be idempotent");
}

/// Translated from moto: test_disable_security_hub
#[tokio::test]
async fn test_disable_security_hub() {
    let client = make_client().await;

    // Disabling before enabling should fail
    let result = client.disable_security_hub().send().await;
    assert!(
        result.is_err(),
        "disable_security_hub should fail before enabling"
    );

    // Enable then disable
    client.enable_security_hub().send().await.unwrap();
    let resp = client.disable_security_hub().send().await;
    assert!(
        resp.is_ok(),
        "disable_security_hub should succeed after enabling"
    );

    // describe_hub should fail again
    let result = client.describe_hub().send().await;
    assert!(result.is_err(), "describe_hub should fail after disabling");
}

/// Translated from moto: test_enable_security_hub_with_parameters
#[tokio::test]
async fn test_enable_security_hub_with_parameters() {
    let client = make_client().await;

    let resp = client
        .enable_security_hub()
        .enable_default_standards(false)
        .send()
        .await;
    assert!(resp.is_ok());

    let hub_info = client
        .describe_hub()
        .send()
        .await
        .expect("describe_hub should work");
    assert!(hub_info.hub_arn().is_some());

    client.disable_security_hub().send().await.unwrap();

    let resp = client
        .enable_security_hub()
        .enable_default_standards(true)
        .send()
        .await;
    assert!(resp.is_ok());
}

/// Translated from moto: test_batch_import_findings
#[tokio::test]
async fn test_batch_import_findings() {
    let client = make_client_region("us-east-2").await;

    let finding = make_finding(
        "test-finding-001",
        "us-east-2",
        "Test finding description",
        "Test Finding",
        "test-resource",
    );

    let resp = client
        .batch_import_findings()
        .findings(finding)
        .send()
        .await
        .expect("batch_import_findings should succeed");

    assert_eq!(resp.success_count(), Some(1));
    assert_eq!(resp.failed_count(), Some(0));
    assert_eq!(resp.failed_findings().len(), 0);
}

/// Translated from moto: test_get_findings
#[tokio::test]
async fn test_get_findings() {
    let client = make_client().await;

    let finding = make_finding(
        "test-finding-001",
        "us-east-1",
        "Test finding description",
        "Test Finding",
        "test-resource",
    );

    let import_resp = client
        .batch_import_findings()
        .findings(finding)
        .send()
        .await
        .expect("batch_import_findings should succeed");
    assert_eq!(import_resp.success_count(), Some(1));

    let resp = client
        .get_findings()
        .send()
        .await
        .expect("get_findings should succeed");

    let findings = resp.findings();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].id(), Some("test-finding-001"));
    assert_eq!(findings[0].schema_version(), Some("2018-10-08"));
}

/// Translated from moto: test_get_findings_invalid_parameters
#[tokio::test]
async fn test_get_findings_invalid_parameters() {
    let client = make_client().await;

    let result = client.get_findings().max_results(101).send().await;
    assert!(
        result.is_err(),
        "get_findings with MaxResults=101 should fail"
    );
    let err = result.unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidInput")
            || err_str.contains("MaxResults must be a number between 1 and 100"),
        "Expected InvalidInputException, got: {}",
        err_str
    );
}

/// Translated from moto: test_batch_import_multiple_findings
#[tokio::test]
async fn test_batch_import_multiple_findings() {
    let client = make_client().await;

    let mut builder = client.batch_import_findings();
    for i in 1..=3 {
        let finding = make_finding(
            &format!("test-finding-{i:03}"),
            "us-east-1",
            &format!("Test finding description {i}"),
            &format!("Test Finding {i}"),
            &format!("test-resource-{i}"),
        );
        builder = builder.findings(finding);
    }

    let import_resp = builder.send().await.expect("batch_import should succeed");
    assert_eq!(import_resp.success_count(), Some(3));
    assert_eq!(import_resp.failed_count(), Some(0));
    assert_eq!(import_resp.failed_findings().len(), 0);

    let get_resp = client
        .get_findings()
        .send()
        .await
        .expect("get_findings should succeed");

    let findings = get_resp.findings();
    assert_eq!(findings.len(), 3);

    let mut imported_ids: Vec<&str> = findings.iter().filter_map(|f| f.id()).collect();
    imported_ids.sort();
    assert_eq!(
        imported_ids,
        vec!["test-finding-001", "test-finding-002", "test-finding-003"]
    );
}

/// Translated from moto: test_get_findings_max_results
#[tokio::test]
async fn test_get_findings_max_results() {
    let client = make_client().await;

    let mut builder = client.batch_import_findings();
    for i in 1..=3 {
        let finding = make_finding(
            &format!("test-finding-{i:03}"),
            "us-east-1",
            &format!("Test finding description {i}"),
            &format!("Test Finding {i}"),
            &format!("test-resource-{i}"),
        );
        builder = builder.findings(finding);
    }

    let import_resp = builder.send().await.unwrap();
    assert_eq!(import_resp.success_count(), Some(3));

    let get_resp = client
        .get_findings()
        .max_results(1)
        .send()
        .await
        .expect("get_findings with MaxResults=1 should succeed");

    assert_eq!(get_resp.findings().len(), 1);
    assert!(
        get_resp.next_token().is_some(),
        "NextToken should be present when more results exist"
    );
}

/// Translated from moto: test_create_members
#[tokio::test]
async fn test_create_members() {
    use aws_sdk_securityhub::types::AccountDetails;

    let client = make_client_region("us-east-2").await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .create_members()
        .account_details(
            AccountDetails::builder()
                .account_id("123456789012")
                .email("test1@example.com")
                .build(),
        )
        .account_details(
            AccountDetails::builder()
                .account_id("123456789013")
                .email("test2@example.com")
                .build(),
        )
        .send()
        .await
        .expect("create_members should succeed");

    assert_eq!(resp.unprocessed_accounts().len(), 0);

    // Creating duplicate should return unprocessed
    let resp2 = client
        .create_members()
        .account_details(
            AccountDetails::builder()
                .account_id("123456789012")
                .email("test1@example.com")
                .build(),
        )
        .send()
        .await
        .expect("create_members should succeed even with duplicates");

    assert_eq!(resp2.unprocessed_accounts().len(), 1);
    assert_eq!(
        resp2.unprocessed_accounts()[0].account_id(),
        Some("123456789012")
    );
}

/// Translated from moto: test_get_members
#[tokio::test]
async fn test_get_members() {
    use aws_sdk_securityhub::types::AccountDetails;

    let client = make_client_region("us-east-2").await;

    client.enable_security_hub().send().await.unwrap();

    client
        .create_members()
        .account_details(
            AccountDetails::builder()
                .account_id("123456789012")
                .email("test1@example.com")
                .build(),
        )
        .account_details(
            AccountDetails::builder()
                .account_id("123456789013")
                .email("test2@example.com")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_members()
        .account_ids("123456789012")
        .account_ids("123456789013")
        .send()
        .await
        .expect("get_members should succeed");

    assert_eq!(resp.members().len(), 2);
    assert_eq!(resp.unprocessed_accounts().len(), 0);

    let member_ids: std::collections::HashSet<&str> = resp
        .members()
        .iter()
        .filter_map(|m| m.account_id())
        .collect();
    assert!(member_ids.contains("123456789012"));
    assert!(member_ids.contains("123456789013"));

    for member in resp.members() {
        assert!(member.account_id().is_some());
        // Check member status is "ENABLED" via string
        let status_str = format!("{:?}", member.member_status());
        assert!(
            status_str.contains("Enabled") || status_str.contains("ENABLED"),
            "Expected Enabled status, got: {}",
            status_str
        );
    }

    // Get non-existent member
    let resp2 = client
        .get_members()
        .account_ids("999999999999")
        .send()
        .await
        .expect("get_members should succeed for non-existent accounts");

    assert_eq!(resp2.members().len(), 0);
    assert_eq!(resp2.unprocessed_accounts().len(), 1);
    assert_eq!(
        resp2.unprocessed_accounts()[0].account_id(),
        Some("999999999999")
    );
    let processing_result = resp2.unprocessed_accounts()[0]
        .processing_result()
        .unwrap_or("");
    assert!(
        processing_result.contains("not a member"),
        "Expected 'not a member' in processing result, got: {}",
        processing_result
    );

    // Mixed: some exist, some don't
    let resp3 = client
        .get_members()
        .account_ids("123456789012")
        .account_ids("999999999999")
        .account_ids("123456789013")
        .send()
        .await
        .unwrap();

    assert_eq!(resp3.members().len(), 2);
    assert_eq!(resp3.unprocessed_accounts().len(), 1);
}

/// Translated from moto: test_list_members
#[tokio::test]
async fn test_list_members() {
    use aws_sdk_securityhub::types::AccountDetails;

    let client = make_client_region("us-east-2").await;

    client.enable_security_hub().send().await.unwrap();

    // Empty list initially
    let resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");
    assert_eq!(resp.members().len(), 0);

    // Create 3 members
    client
        .create_members()
        .account_details(
            AccountDetails::builder()
                .account_id("123456789012")
                .email("test1@example.com")
                .build(),
        )
        .account_details(
            AccountDetails::builder()
                .account_id("123456789013")
                .email("test2@example.com")
                .build(),
        )
        .account_details(
            AccountDetails::builder()
                .account_id("123456789014")
                .email("test3@example.com")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client.list_members().send().await.unwrap();
    assert_eq!(resp.members().len(), 3);

    let member_ids: std::collections::HashSet<&str> = resp
        .members()
        .iter()
        .filter_map(|m| m.account_id())
        .collect();
    assert!(member_ids.contains("123456789012"));
    assert!(member_ids.contains("123456789013"));
    assert!(member_ids.contains("123456789014"));

    for member in resp.members() {
        let status_str = format!("{:?}", member.member_status());
        assert!(
            status_str.contains("Enabled") || status_str.contains("ENABLED"),
            "Expected Enabled status, got: {}",
            status_str
        );
    }

    // Pagination with MaxResults
    let resp = client.list_members().max_results(2).send().await.unwrap();
    assert_eq!(resp.members().len(), 2);
    assert!(resp.next_token().is_some(), "NextToken should be present");

    let next_token = resp.next_token().unwrap().to_string();
    let resp2 = client
        .list_members()
        .max_results(2)
        .next_token(&next_token)
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.members().len(), 1);
}

/// Test CreateActionTarget, DescribeActionTargets, DeleteActionTarget lifecycle
#[tokio::test]
async fn test_action_target_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create an action target
    let resp = client
        .create_action_target()
        .name("TestAction")
        .description("A test action target")
        .id("test-action-1")
        .send()
        .await
        .expect("create_action_target should succeed");

    let arn = resp
        .action_target_arn()
        .expect("ActionTargetArn should be present");
    assert!(
        arn.contains("action/custom/test-action-1"),
        "ARN should contain action ID, got: {}",
        arn
    );

    // Describe action targets (all)
    let resp = client
        .describe_action_targets()
        .send()
        .await
        .expect("describe_action_targets should succeed");

    assert_eq!(resp.action_targets().len(), 1);
    assert_eq!(resp.action_targets()[0].name(), Some("TestAction"));
    assert_eq!(
        resp.action_targets()[0].description(),
        Some("A test action target")
    );

    // Describe action targets by ARN
    let resp = client
        .describe_action_targets()
        .action_target_arns(arn)
        .send()
        .await
        .expect("describe_action_targets with ARN should succeed");

    assert_eq!(resp.action_targets().len(), 1);
    assert_eq!(resp.action_targets()[0].action_target_arn(), Some(arn));

    // Delete action target
    let del_resp = client
        .delete_action_target()
        .action_target_arn(arn)
        .send()
        .await
        .expect("delete_action_target should succeed");

    assert_eq!(del_resp.action_target_arn(), Some(arn));

    // Describe should return empty after deletion
    let resp = client
        .describe_action_targets()
        .send()
        .await
        .expect("describe_action_targets should succeed after deletion");

    assert_eq!(resp.action_targets().len(), 0);
}

/// Test duplicate action target creation
#[tokio::test]
async fn test_create_action_target_duplicate() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    client
        .create_action_target()
        .name("TestAction")
        .description("A test action target")
        .id("dup-test")
        .send()
        .await
        .expect("first create should succeed");

    // Attempt duplicate creation
    let result = client
        .create_action_target()
        .name("TestAction2")
        .description("Another")
        .id("dup-test")
        .send()
        .await;

    assert!(
        result.is_err(),
        "duplicate create_action_target should fail"
    );
}

/// Test delete nonexistent action target
#[tokio::test]
async fn test_delete_action_target_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .delete_action_target()
        .action_target_arn("arn:aws:securityhub:us-east-1:123456789012:action/custom/nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "deleting nonexistent action target should fail"
    );
}

/// Test CreateFindingAggregator
#[tokio::test]
async fn test_create_finding_aggregator() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .create_finding_aggregator()
        .region_linking_mode("ALL_REGIONS")
        .send()
        .await
        .expect("create_finding_aggregator should succeed");

    assert!(
        resp.finding_aggregator_arn().is_some(),
        "FindingAggregatorArn should be present"
    );
    assert_eq!(resp.region_linking_mode().unwrap(), "ALL_REGIONS");
    assert_eq!(resp.finding_aggregation_region().unwrap(), "us-east-1");
}

/// Test CreateFindingAggregator with specific regions
#[tokio::test]
async fn test_create_finding_aggregator_with_regions() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .create_finding_aggregator()
        .region_linking_mode("SPECIFIED_REGIONS")
        .regions("us-west-2")
        .regions("eu-west-1")
        .send()
        .await
        .expect("create_finding_aggregator should succeed");

    assert!(resp.finding_aggregator_arn().is_some());
    assert_eq!(resp.region_linking_mode().unwrap(), "SPECIFIED_REGIONS");
    let regions = resp.regions();
    assert_eq!(regions.len(), 2);
    assert!(regions.contains(&"us-west-2".to_string()));
    assert!(regions.contains(&"eu-west-1".to_string()));
}

/// Test GetEnabledStandards (returns empty when no standards enabled)
#[tokio::test]
async fn test_get_enabled_standards() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .get_enabled_standards()
        .send()
        .await
        .expect("get_enabled_standards should succeed");

    // By default no standards are enabled in mock
    assert_eq!(resp.standards_subscriptions().len(), 0);
}

/// Test ListTagsForResource, TagResource, UntagResource
#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;

    // Enable with tags
    client
        .enable_security_hub()
        .tags("env", "test")
        .tags("team", "security")
        .send()
        .await
        .unwrap();

    let hub_arn = "arn:aws:securityhub:us-east-1:123456789012:hub/default";

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(hub_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("security"));

    // Add more tags
    client
        .tag_resource()
        .resource_arn(hub_arn)
        .tags("project", "winterbaume")
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify new tag is present
    let resp = client
        .list_tags_for_resource()
        .resource_arn(hub_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("winterbaume"));
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));

    // Untag
    client
        .untag_resource()
        .resource_arn(hub_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tag removed
    let resp = client
        .list_tags_for_resource()
        .resource_arn(hub_arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert!(tags.get("env").is_none(), "env tag should be removed");
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("security"));
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("winterbaume"));
}

/// Test GetMasterAccount (returns empty when no admin configured)
#[tokio::test]
#[allow(deprecated)]
async fn test_get_master_account() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .get_master_account()
        .send()
        .await
        .expect("get_master_account should succeed");

    // No master configured by default
    assert!(
        resp.master().is_none(),
        "Master should be None when not configured"
    );
}

/// Test GetAdministratorAccount (returns empty when no admin configured)
#[tokio::test]
async fn test_get_administrator_account() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .get_administrator_account()
        .send()
        .await
        .expect("get_administrator_account should succeed");

    // No administrator configured by default
    assert!(
        resp.administrator().is_none(),
        "Administrator should be None when not configured"
    );
}

/// Test EnableOrganizationAdminAccount
#[tokio::test]
async fn test_enable_organization_admin_account() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .enable_organization_admin_account()
        .admin_account_id("123456789012")
        .send()
        .await
        .expect("enable_organization_admin_account should succeed");

    // Response may contain AdminAccountId
    let _ = resp;

    // Now GetAdministratorAccount should return the configured account
    let admin_resp = client
        .get_administrator_account()
        .send()
        .await
        .expect("get_administrator_account should succeed");

    let administrator = admin_resp.administrator();
    assert!(
        administrator.is_some(),
        "Administrator should be set after enable_organization_admin_account"
    );
    assert_eq!(administrator.unwrap().account_id(), Some("123456789012"));
}

/// Test DescribeOrganizationConfiguration
#[tokio::test]
async fn test_describe_organization_configuration() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe_organization_configuration should succeed");

    // Default state: auto_enable = false
    assert_eq!(resp.auto_enable(), Some(false));
}

/// Test UpdateOrganizationConfiguration
#[tokio::test]
async fn test_update_organization_configuration() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Update configuration
    client
        .update_organization_configuration()
        .auto_enable(true)
        .send()
        .await
        .expect("update_organization_configuration should succeed");

    // Verify the update
    let resp = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe_organization_configuration should succeed");

    assert_eq!(
        resp.auto_enable(),
        Some(true),
        "AutoEnable should be updated to true"
    );
}

// ============================================================================
// Delete members lifecycle
// ============================================================================

/// Test DeleteMembers
#[tokio::test]
async fn test_delete_members() {
    use aws_sdk_securityhub::types::AccountDetails;

    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create members
    client
        .create_members()
        .account_details(
            AccountDetails::builder()
                .account_id("111111111111")
                .email("a@example.com")
                .build(),
        )
        .account_details(
            AccountDetails::builder()
                .account_id("222222222222")
                .email("b@example.com")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Verify both exist
    let resp = client.list_members().send().await.unwrap();
    assert_eq!(resp.members().len(), 2);

    // Delete one member
    client
        .delete_members()
        .account_ids("111111111111")
        .send()
        .await
        .expect("delete_members should succeed");

    // Verify only one remains
    let resp = client.list_members().send().await.unwrap();
    assert_eq!(resp.members().len(), 1);
    assert_eq!(resp.members()[0].account_id(), Some("222222222222"));
}

// ============================================================================
// Automation rules (v1) lifecycle
// ============================================================================

/// Test CreateAutomationRule, ListAutomationRules, BatchGetAutomationRules
#[tokio::test]
async fn test_automation_rule_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create an automation rule
    let resp = client
        .create_automation_rule()
        .rule_name("TestRule")
        .rule_order(1)
        .rule_status(aws_sdk_securityhub::types::RuleStatus::Enabled)
        .description("A test automation rule")
        .is_terminal(false)
        .criteria(aws_sdk_securityhub::types::AutomationRulesFindingFilters::builder().build())
        .actions(
            aws_sdk_securityhub::types::AutomationRulesAction::builder()
                .r#type(aws_sdk_securityhub::types::AutomationRulesActionType::FindingFieldsUpdate)
                .build(),
        )
        .send()
        .await
        .expect("create_automation_rule should succeed");

    let rule_arn = resp.rule_arn().expect("RuleArn should be present");
    assert!(
        rule_arn.contains("automation-rule/"),
        "ARN should contain automation-rule/, got: {}",
        rule_arn
    );

    // List automation rules
    let list_resp = client
        .list_automation_rules()
        .send()
        .await
        .expect("list_automation_rules should succeed");

    let metadata = list_resp.automation_rules_metadata();
    assert_eq!(metadata.len(), 1);
    assert_eq!(metadata[0].rule_name(), Some("TestRule"));
    assert_eq!(metadata[0].rule_order(), Some(1));

    // Batch get automation rules
    let get_resp = client
        .batch_get_automation_rules()
        .automation_rules_arns(rule_arn)
        .send()
        .await
        .expect("batch_get_automation_rules should succeed");

    let rules = get_resp.rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].rule_arn(), Some(rule_arn));
    assert_eq!(rules[0].rule_name(), Some("TestRule"));
    assert_eq!(rules[0].description(), Some("A test automation rule"));
}

/// Test BatchUpdateAutomationRules
#[tokio::test]
async fn test_batch_update_automation_rules() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .create_automation_rule()
        .rule_name("UpdateMe")
        .rule_order(1)
        .rule_status(aws_sdk_securityhub::types::RuleStatus::Enabled)
        .criteria(aws_sdk_securityhub::types::AutomationRulesFindingFilters::builder().build())
        .actions(
            aws_sdk_securityhub::types::AutomationRulesAction::builder()
                .r#type(aws_sdk_securityhub::types::AutomationRulesActionType::FindingFieldsUpdate)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_arn = resp.rule_arn().unwrap().to_string();

    // Update the rule
    let update_resp = client
        .batch_update_automation_rules()
        .update_automation_rules_request_items(
            aws_sdk_securityhub::types::UpdateAutomationRulesRequestItem::builder()
                .rule_arn(&rule_arn)
                .rule_status(aws_sdk_securityhub::types::RuleStatus::Disabled)
                .rule_order(5)
                .build(),
        )
        .send()
        .await
        .expect("batch_update_automation_rules should succeed");

    assert_eq!(update_resp.processed_automation_rules().len(), 1);
    assert_eq!(update_resp.processed_automation_rules()[0], rule_arn);

    // Verify update via batch get
    let get_resp = client
        .batch_get_automation_rules()
        .automation_rules_arns(&rule_arn)
        .send()
        .await
        .unwrap();

    let rules = get_resp.rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].rule_status().map(|s| s.as_str()), Some("DISABLED"));
    assert_eq!(rules[0].rule_order(), Some(5));
}

/// Test BatchDeleteAutomationRules
#[tokio::test]
async fn test_batch_delete_automation_rules() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .create_automation_rule()
        .rule_name("DeleteMe")
        .rule_order(1)
        .rule_status(aws_sdk_securityhub::types::RuleStatus::Enabled)
        .criteria(aws_sdk_securityhub::types::AutomationRulesFindingFilters::builder().build())
        .actions(
            aws_sdk_securityhub::types::AutomationRulesAction::builder()
                .r#type(aws_sdk_securityhub::types::AutomationRulesActionType::FindingFieldsUpdate)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let rule_arn = resp.rule_arn().unwrap().to_string();

    // Delete the rule
    let del_resp = client
        .batch_delete_automation_rules()
        .automation_rules_arns(&rule_arn)
        .send()
        .await
        .expect("batch_delete_automation_rules should succeed");

    assert_eq!(del_resp.processed_automation_rules().len(), 1);

    // Verify deleted: batch get returns empty
    let get_resp = client
        .batch_get_automation_rules()
        .automation_rules_arns(&rule_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.rules().len(), 0);
}

/// Test BatchDeleteAutomationRules with nonexistent ARN
#[tokio::test]
async fn test_batch_delete_automation_rules_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let del_resp = client
        .batch_delete_automation_rules()
        .automation_rules_arns(
            "arn:aws:securityhub:us-east-1:123456789012:automation-rule/nonexistent",
        )
        .send()
        .await
        .expect("batch_delete_automation_rules should succeed with unprocessed");

    assert_eq!(del_resp.unprocessed_automation_rules().len(), 1);
}

// ============================================================================
// Standards enable/disable lifecycle
// ============================================================================

/// Test BatchEnableStandards, GetEnabledStandards, BatchDisableStandards
#[tokio::test]
async fn test_standards_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Enable a standard
    let enable_resp = client
        .batch_enable_standards()
        .standards_subscription_requests(
            aws_sdk_securityhub::types::StandardsSubscriptionRequest::builder()
                .standards_arn(
                    "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
                )
                .build(),
        )
        .send()
        .await
        .expect("batch_enable_standards should succeed");

    let subs = enable_resp.standards_subscriptions();
    assert_eq!(subs.len(), 1);
    let sub_arn = subs[0]
        .standards_subscription_arn()
        .expect("subscription ARN should be present");
    assert_eq!(
        subs[0].standards_status().map(|s| s.as_str()),
        Some("READY")
    );

    // Get enabled standards
    let get_resp = client
        .get_enabled_standards()
        .send()
        .await
        .expect("get_enabled_standards should succeed");

    assert_eq!(get_resp.standards_subscriptions().len(), 1);

    // Disable the standard
    client
        .batch_disable_standards()
        .standards_subscription_arns(sub_arn)
        .send()
        .await
        .expect("batch_disable_standards should succeed");

    // Verify disabled
    let get_resp = client
        .get_enabled_standards()
        .send()
        .await
        .expect("get_enabled_standards should succeed after disable");

    assert_eq!(get_resp.standards_subscriptions().len(), 0);
}

// ============================================================================
// Finding aggregator full lifecycle
// ============================================================================

/// Test CreateFindingAggregator, GetFindingAggregator, ListFindingAggregators,
/// UpdateFindingAggregator, DeleteFindingAggregator
#[tokio::test]
async fn test_finding_aggregator_full_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create
    let create_resp = client
        .create_finding_aggregator()
        .region_linking_mode("ALL_REGIONS")
        .send()
        .await
        .expect("create_finding_aggregator should succeed");

    let agg_arn = create_resp
        .finding_aggregator_arn()
        .expect("arn should be present")
        .to_string();

    // List
    let list_resp = client
        .list_finding_aggregators()
        .send()
        .await
        .expect("list_finding_aggregators should succeed");

    assert_eq!(list_resp.finding_aggregators().len(), 1);
    assert_eq!(
        list_resp.finding_aggregators()[0]
            .finding_aggregator_arn()
            .unwrap(),
        agg_arn
    );

    // Get
    let get_resp = client
        .get_finding_aggregator()
        .finding_aggregator_arn(&agg_arn)
        .send()
        .await
        .expect("get_finding_aggregator should succeed");

    assert_eq!(get_resp.finding_aggregator_arn().unwrap(), agg_arn);
    assert_eq!(get_resp.region_linking_mode().unwrap(), "ALL_REGIONS");

    // Update
    let update_resp = client
        .update_finding_aggregator()
        .finding_aggregator_arn(&agg_arn)
        .region_linking_mode("SPECIFIED_REGIONS")
        .regions("eu-west-1")
        .send()
        .await
        .expect("update_finding_aggregator should succeed");

    assert_eq!(
        update_resp.region_linking_mode().unwrap(),
        "SPECIFIED_REGIONS"
    );

    // Delete
    client
        .delete_finding_aggregator()
        .finding_aggregator_arn(&agg_arn)
        .send()
        .await
        .expect("delete_finding_aggregator should succeed");

    // List should be empty
    let list_resp = client
        .list_finding_aggregators()
        .send()
        .await
        .expect("list_finding_aggregators should succeed after delete");

    assert_eq!(list_resp.finding_aggregators().len(), 0);
}

// ============================================================================
// Insight lifecycle
// ============================================================================

/// Test CreateInsight, GetInsights, UpdateInsight, DeleteInsight, GetInsightResults
#[tokio::test]
async fn test_insight_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create insight
    let create_resp = client
        .create_insight()
        .name("TestInsight")
        .group_by_attribute("ResourceId")
        .filters(aws_sdk_securityhub::types::AwsSecurityFindingFilters::builder().build())
        .send()
        .await
        .expect("create_insight should succeed");

    let insight_arn = create_resp
        .insight_arn()
        .expect("InsightArn should be present")
        .to_string();
    assert!(
        insight_arn.contains("insight/"),
        "ARN should contain insight/"
    );

    // GetInsights (all)
    let get_resp = client
        .get_insights()
        .send()
        .await
        .expect("get_insights should succeed");

    let insights = get_resp.insights();
    assert_eq!(insights.len(), 1);
    assert_eq!(insights[0].name(), Some("TestInsight"));
    assert_eq!(insights[0].group_by_attribute(), Some("ResourceId"));

    // GetInsights by ARN
    let get_resp = client
        .get_insights()
        .insight_arns(&insight_arn)
        .send()
        .await
        .expect("get_insights with ARN should succeed");

    assert_eq!(get_resp.insights().len(), 1);

    // UpdateInsight
    client
        .update_insight()
        .insight_arn(&insight_arn)
        .name("UpdatedInsight")
        .group_by_attribute("AccountId")
        .send()
        .await
        .expect("update_insight should succeed");

    // Verify update
    let get_resp = client
        .get_insights()
        .insight_arns(&insight_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.insights()[0].name(), Some("UpdatedInsight"));
    assert_eq!(
        get_resp.insights()[0].group_by_attribute(),
        Some("AccountId")
    );

    // GetInsightResults
    let results_resp = client
        .get_insight_results()
        .insight_arn(&insight_arn)
        .send()
        .await
        .expect("get_insight_results should succeed");

    // The mock returns None for results, just verify no error
    let _ = results_resp;

    // DeleteInsight
    let del_resp = client
        .delete_insight()
        .insight_arn(&insight_arn)
        .send()
        .await
        .expect("delete_insight should succeed");

    assert_eq!(del_resp.insight_arn(), Some(insight_arn.as_str()));

    // Verify deleted
    let get_resp = client.get_insights().send().await.unwrap();
    assert_eq!(get_resp.insights().len(), 0);
}

/// Test creating multiple insights
#[tokio::test]
async fn test_create_multiple_insights() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    for i in 1..=3 {
        client
            .create_insight()
            .name(format!("Insight{i}"))
            .group_by_attribute("ResourceId")
            .filters(aws_sdk_securityhub::types::AwsSecurityFindingFilters::builder().build())
            .send()
            .await
            .unwrap();
    }

    let get_resp = client.get_insights().send().await.unwrap();
    assert_eq!(get_resp.insights().len(), 3);
}

// ============================================================================
// Configuration policy lifecycle
// ============================================================================

/// Test CreateConfigurationPolicy, GetConfigurationPolicy,
/// ListConfigurationPolicies, UpdateConfigurationPolicy, DeleteConfigurationPolicy
#[tokio::test]
async fn test_configuration_policy_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create
    let create_resp = client
        .create_configuration_policy()
        .name("TestPolicy")
        .description("A test configuration policy")
        .send()
        .await
        .expect("create_configuration_policy should succeed");

    let policy_id = create_resp.id().expect("Id should be present").to_string();
    let policy_arn = create_resp
        .arn()
        .expect("Arn should be present")
        .to_string();
    assert_eq!(create_resp.name(), Some("TestPolicy"));
    assert_eq!(
        create_resp.description(),
        Some("A test configuration policy")
    );
    assert!(create_resp.created_at().is_some());
    assert!(create_resp.updated_at().is_some());
    assert!(policy_arn.contains("configuration-policy/"));

    // Get by ID
    let get_resp = client
        .get_configuration_policy()
        .identifier(&policy_id)
        .send()
        .await
        .expect("get_configuration_policy should succeed");

    assert_eq!(get_resp.id(), Some(policy_id.as_str()));
    assert_eq!(get_resp.name(), Some("TestPolicy"));

    // List
    let list_resp = client
        .list_configuration_policies()
        .send()
        .await
        .expect("list_configuration_policies should succeed");

    let summaries = list_resp.configuration_policy_summaries();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].name(), Some("TestPolicy"));

    // Update
    let update_resp = client
        .update_configuration_policy()
        .identifier(&policy_id)
        .name("UpdatedPolicy")
        .description("Updated description")
        .send()
        .await
        .expect("update_configuration_policy should succeed");

    assert_eq!(update_resp.name(), Some("UpdatedPolicy"));
    assert_eq!(update_resp.description(), Some("Updated description"));

    // Verify update
    let get_resp = client
        .get_configuration_policy()
        .identifier(&policy_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.name(), Some("UpdatedPolicy"));

    // Delete
    client
        .delete_configuration_policy()
        .identifier(&policy_id)
        .send()
        .await
        .expect("delete_configuration_policy should succeed");

    // Verify deleted
    let list_resp = client.list_configuration_policies().send().await.unwrap();

    assert_eq!(list_resp.configuration_policy_summaries().len(), 0);
}

/// Test GetConfigurationPolicy for nonexistent policy
#[tokio::test]
async fn test_get_configuration_policy_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .get_configuration_policy()
        .identifier("nonexistent-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_configuration_policy should fail for nonexistent ID"
    );
}

/// Test DeleteConfigurationPolicy for nonexistent policy
#[tokio::test]
async fn test_delete_configuration_policy_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .delete_configuration_policy()
        .identifier("nonexistent-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_configuration_policy should fail for nonexistent ID"
    );
}

// ============================================================================
// Organization admin account lifecycle
// ============================================================================

/// Test EnableOrganizationAdminAccount, ListOrganizationAdminAccounts,
/// DisableOrganizationAdminAccount
#[tokio::test]
async fn test_organization_admin_account_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // List: initially empty
    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .expect("list_organization_admin_accounts should succeed");

    assert_eq!(list_resp.admin_accounts().len(), 0);

    // Enable admin
    client
        .enable_organization_admin_account()
        .admin_account_id("111111111111")
        .send()
        .await
        .expect("enable_organization_admin_account should succeed");

    // List: one admin
    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.admin_accounts().len(), 1);
    assert_eq!(
        list_resp.admin_accounts()[0].account_id(),
        Some("111111111111")
    );
    assert_eq!(
        list_resp.admin_accounts()[0].status().map(|s| s.as_str()),
        Some("ENABLED")
    );

    // Disable admin
    client
        .disable_organization_admin_account()
        .admin_account_id("111111111111")
        .send()
        .await
        .expect("disable_organization_admin_account should succeed");

    // List: empty again
    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.admin_accounts().len(), 0);

    // GetAdministratorAccount: should be None
    let get_resp = client.get_administrator_account().send().await.unwrap();

    assert!(
        get_resp.administrator().is_none(),
        "Administrator should be None after disabling"
    );
}

// ============================================================================
// Action target update
// ============================================================================

/// Test UpdateActionTarget
#[tokio::test]
async fn test_update_action_target() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let create_resp = client
        .create_action_target()
        .name("OriginalName")
        .description("Original description")
        .id("update-test")
        .send()
        .await
        .unwrap();

    let arn = create_resp.action_target_arn().unwrap().to_string();

    // Update
    client
        .update_action_target()
        .action_target_arn(&arn)
        .name("UpdatedName")
        .description("Updated description")
        .send()
        .await
        .expect("update_action_target should succeed");

    // Verify
    let desc_resp = client
        .describe_action_targets()
        .action_target_arns(&arn)
        .send()
        .await
        .unwrap();

    assert_eq!(desc_resp.action_targets()[0].name(), Some("UpdatedName"));
    assert_eq!(
        desc_resp.action_targets()[0].description(),
        Some("Updated description")
    );
}

/// Test UpdateActionTarget for nonexistent target
#[tokio::test]
async fn test_update_action_target_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .update_action_target()
        .action_target_arn("arn:aws:securityhub:us-east-1:123456789012:action/custom/nonexistent")
        .name("Name")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_action_target should fail for nonexistent target"
    );
}

// ============================================================================
// Stub operations (return empty/default responses)
// ============================================================================

/// Test DescribeProducts returns empty products list
#[tokio::test]
async fn test_describe_products() {
    let client = make_client().await;

    let resp = client
        .describe_products()
        .send()
        .await
        .expect("describe_products should succeed");

    assert_eq!(resp.products().len(), 0);
}

/// Test DescribeStandards returns empty
#[tokio::test]
async fn test_describe_standards() {
    let client = make_client().await;

    let resp = client
        .describe_standards()
        .send()
        .await
        .expect("describe_standards should succeed");

    assert_eq!(resp.standards().len(), 0);
}

/// Test GetInvitationsCount returns 0
#[tokio::test]
async fn test_get_invitations_count() {
    let client = make_client().await;

    let resp = client
        .get_invitations_count()
        .send()
        .await
        .expect("get_invitations_count should succeed");

    assert_eq!(resp.invitations_count(), Some(0));
}

/// Test ListInvitations returns empty
#[tokio::test]
async fn test_list_invitations() {
    let client = make_client().await;

    let resp = client
        .list_invitations()
        .send()
        .await
        .expect("list_invitations should succeed");

    assert_eq!(resp.invitations().len(), 0);
}

/// Test DeclineInvitations returns no unprocessed accounts
#[tokio::test]
async fn test_decline_invitations() {
    let client = make_client().await;

    let resp = client
        .decline_invitations()
        .account_ids("999999999999")
        .send()
        .await
        .expect("decline_invitations should succeed");

    assert_eq!(resp.unprocessed_accounts().len(), 0);
}

/// Test DeleteInvitations returns no unprocessed accounts
#[tokio::test]
async fn test_delete_invitations() {
    let client = make_client().await;

    let resp = client
        .delete_invitations()
        .account_ids("999999999999")
        .send()
        .await
        .expect("delete_invitations should succeed");

    assert_eq!(resp.unprocessed_accounts().len(), 0);
}

/// Test AcceptAdministratorInvitation (stub)
#[tokio::test]
async fn test_accept_administrator_invitation() {
    let client = make_client().await;

    let resp = client
        .accept_administrator_invitation()
        .administrator_id("111111111111")
        .invitation_id("test-invitation")
        .send()
        .await;

    assert!(
        resp.is_ok(),
        "accept_administrator_invitation should succeed"
    );
}

/// Test DisassociateFromAdministratorAccount (stub)
#[tokio::test]
async fn test_disassociate_from_administrator_account() {
    let client = make_client().await;

    let resp = client
        .disassociate_from_administrator_account()
        .send()
        .await;

    assert!(
        resp.is_ok(),
        "disassociate_from_administrator_account should succeed"
    );
}

/// Test DisassociateMembers (stub)
#[tokio::test]
async fn test_disassociate_members() {
    let client = make_client().await;

    let resp = client
        .disassociate_members()
        .account_ids("999999999999")
        .send()
        .await;

    assert!(resp.is_ok(), "disassociate_members should succeed");
}

/// Test InviteMembers (stub)
#[tokio::test]
async fn test_invite_members() {
    let client = make_client().await;

    let resp = client
        .invite_members()
        .account_ids("999999999999")
        .send()
        .await
        .expect("invite_members should succeed");

    assert_eq!(resp.unprocessed_accounts().len(), 0);
}

/// Test EnableImportFindingsForProduct (stub)
#[tokio::test]
async fn test_enable_import_findings_for_product() {
    let client = make_client().await;

    let resp = client
        .enable_import_findings_for_product()
        .product_arn("arn:aws:securityhub:us-east-1::product/aws/guardduty")
        .send()
        .await
        .expect("enable_import_findings_for_product should succeed");

    assert!(resp.product_subscription_arn().is_some());
}

/// Test DisableImportFindingsForProduct (stub)
#[tokio::test]
async fn test_disable_import_findings_for_product() {
    let client = make_client().await;

    let resp = client
        .disable_import_findings_for_product()
        .product_subscription_arn(
            "arn:aws:securityhub:us-east-1:123456789012:product-subscription/test",
        )
        .send()
        .await;

    assert!(
        resp.is_ok(),
        "disable_import_findings_for_product should succeed"
    );
}

/// Test ListEnabledProductsForImport (stub)
#[tokio::test]
async fn test_list_enabled_products_for_import() {
    let client = make_client().await;

    let resp = client
        .list_enabled_products_for_import()
        .send()
        .await
        .expect("list_enabled_products_for_import should succeed");

    assert_eq!(resp.product_subscriptions().len(), 0);
}

/// Test UpdateSecurityHubConfiguration (stub)
#[tokio::test]
async fn test_update_security_hub_configuration() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let resp = client
        .update_security_hub_configuration()
        .auto_enable_controls(true)
        .send()
        .await;

    assert!(
        resp.is_ok(),
        "update_security_hub_configuration should succeed"
    );
}

/// Test GetFindingHistory (stub)
#[tokio::test]
async fn test_get_finding_history() {
    let client = make_client().await;

    let resp = client
        .get_finding_history()
        .finding_identifier(
            aws_sdk_securityhub::types::AwsSecurityFindingIdentifier::builder()
                .id("finding-001")
                .product_arn(
                    "arn:aws:securityhub:us-east-1:123456789012:product/123456789012/default",
                )
                .build(),
        )
        .send()
        .await
        .expect("get_finding_history should succeed");

    assert_eq!(resp.records().len(), 0);
}

/// Test BatchUpdateFindings (stub)
#[tokio::test]
async fn test_batch_update_findings() {
    let client = make_client().await;

    // First import a finding so it exists in state
    let region = "us-east-1";
    let finding = make_finding(
        "batch-update-001",
        region,
        "Test finding for batch update",
        "Test Batch Update",
        "arn:aws:ec2:us-east-1:123456789012:instance/i-abc",
    );
    client
        .batch_import_findings()
        .findings(finding)
        .send()
        .await
        .expect("batch_import_findings should succeed");

    // Update that finding
    let resp = client
        .batch_update_findings()
        .finding_identifiers(
            aws_sdk_securityhub::types::AwsSecurityFindingIdentifier::builder()
                .id("batch-update-001")
                .product_arn(
                    "arn:aws:securityhub:us-east-1:123456789012:product/123456789012/default",
                )
                .build(),
        )
        .send()
        .await
        .expect("batch_update_findings should succeed");

    // The finding was found and processed
    assert_eq!(resp.processed_findings().len(), 1);
    assert_eq!(resp.unprocessed_findings().len(), 0);

    // A non-existent finding goes to unprocessed
    let resp2 = client
        .batch_update_findings()
        .finding_identifiers(
            aws_sdk_securityhub::types::AwsSecurityFindingIdentifier::builder()
                .id("nonexistent-finding")
                .product_arn(
                    "arn:aws:securityhub:us-east-1:123456789012:product/123456789012/default",
                )
                .build(),
        )
        .send()
        .await
        .expect("batch_update_findings should succeed for unknown finding");

    assert_eq!(resp2.processed_findings().len(), 0);
    assert_eq!(resp2.unprocessed_findings().len(), 1);
}

/// Test BatchGetSecurityControls (stub)
#[tokio::test]
async fn test_batch_get_security_controls() {
    let client = make_client().await;

    let resp = client
        .batch_get_security_controls()
        .security_control_ids("IAM.1")
        .send()
        .await
        .expect("batch_get_security_controls should succeed");

    assert_eq!(resp.security_controls().len(), 0);
}

/// Test ListSecurityControlDefinitions (stub)
#[tokio::test]
async fn test_list_security_control_definitions() {
    let client = make_client().await;

    let resp = client
        .list_security_control_definitions()
        .send()
        .await
        .expect("list_security_control_definitions should succeed");

    assert_eq!(resp.security_control_definitions().len(), 0);
}

/// Test UpdateSecurityControl (stub)
#[tokio::test]
async fn test_update_security_control() {
    let client = make_client().await;

    let resp = client
        .update_security_control()
        .security_control_id("IAM.1")
        .parameters(
            "MaxPasswordAge",
            aws_sdk_securityhub::types::ParameterConfiguration::builder()
                .value_type(aws_sdk_securityhub::types::ParameterValueType::Default)
                .build(),
        )
        .last_update_reason("test update")
        .send()
        .await;

    assert!(resp.is_ok(), "update_security_control should succeed");
}

/// Test BatchGetStandardsControlAssociations (stub)
#[tokio::test]
async fn test_batch_get_standards_control_associations() {
    let client = make_client().await;

    let resp = client
        .batch_get_standards_control_associations()
        .standards_control_association_ids(
            aws_sdk_securityhub::types::StandardsControlAssociationId::builder()
                .security_control_id("IAM.1")
                .standards_arn(
                    "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
                )
                .build(),
        )
        .send()
        .await
        .expect("batch_get_standards_control_associations should succeed");

    assert_eq!(resp.standards_control_association_details().len(), 0);
}

/// Test BatchUpdateStandardsControlAssociations (stub)
#[tokio::test]
async fn test_batch_update_standards_control_associations() {
    let client = make_client().await;

    let resp = client
        .batch_update_standards_control_associations()
        .standards_control_association_updates(
            aws_sdk_securityhub::types::StandardsControlAssociationUpdate::builder()
                .security_control_id("IAM.1")
                .standards_arn(
                    "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
                )
                .association_status(aws_sdk_securityhub::types::AssociationStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("batch_update_standards_control_associations should succeed");

    assert_eq!(resp.unprocessed_association_updates().len(), 0);
}

/// Test ListStandardsControlAssociations (stub)
#[tokio::test]
async fn test_list_standards_control_associations() {
    let client = make_client().await;

    let resp = client
        .list_standards_control_associations()
        .security_control_id("IAM.1")
        .send()
        .await
        .expect("list_standards_control_associations should succeed");

    assert_eq!(resp.standards_control_association_summaries().len(), 0);
}

/// Test DescribeStandardsControls (stub)
#[tokio::test]
async fn test_describe_standards_controls() {
    let client = make_client().await;

    let resp = client
        .describe_standards_controls()
        .standards_subscription_arn("arn:aws:securityhub:us-east-1:123456789012:subscription/test")
        .send()
        .await
        .expect("describe_standards_controls should succeed");

    assert_eq!(resp.controls().len(), 0);
}

/// Test UpdateStandardsControl (stub)
#[tokio::test]
async fn test_update_standards_control() {
    let client = make_client().await;

    let resp = client
        .update_standards_control()
        .standards_control_arn("arn:aws:securityhub:us-east-1:123456789012:control/test/1")
        .control_status(aws_sdk_securityhub::types::ControlStatus::Disabled)
        .disabled_reason("test reason")
        .send()
        .await;

    assert!(resp.is_ok(), "update_standards_control should succeed");
}

/// Test GetSecurityControlDefinition (stub)
#[tokio::test]
async fn test_get_security_control_definition() {
    let client = make_client().await;

    let resp = client
        .get_security_control_definition()
        .security_control_id("IAM.1")
        .send()
        .await
        .expect("get_security_control_definition should succeed");

    // Stub returns a default response
    let _ = resp.security_control_definition();
}

// ============================================================================
// Configuration policy association stubs
// ============================================================================

/// Test BatchGetConfigurationPolicyAssociations (stub)
#[tokio::test]
async fn test_batch_get_configuration_policy_associations() {
    let client = make_client().await;

    let resp = client
        .batch_get_configuration_policy_associations()
        .configuration_policy_association_identifiers(
            aws_sdk_securityhub::types::ConfigurationPolicyAssociation::builder().build(),
        )
        .send()
        .await
        .expect("batch_get_configuration_policy_associations should succeed");

    assert_eq!(resp.configuration_policy_associations().len(), 0);
}

/// Test GetConfigurationPolicyAssociation (stub)
#[tokio::test]
async fn test_get_configuration_policy_association() {
    let client = make_client().await;

    // First associate a policy, then get it
    let _ = client
        .start_configuration_policy_association()
        .configuration_policy_identifier("policy-abc")
        .target(aws_sdk_securityhub::types::Target::AccountId(
            "123456789012".to_string(),
        ))
        .send()
        .await
        .expect("start_configuration_policy_association should succeed");

    let resp = client
        .get_configuration_policy_association()
        .target(aws_sdk_securityhub::types::Target::AccountId(
            "123456789012".to_string(),
        ))
        .send()
        .await
        .expect("get_configuration_policy_association should succeed");

    assert_eq!(resp.configuration_policy_id(), Some("policy-abc"));

    // Non-existent association should return 404
    let err = client
        .get_configuration_policy_association()
        .target(aws_sdk_securityhub::types::Target::AccountId(
            "999999999999".to_string(),
        ))
        .send()
        .await;
    assert!(err.is_err());
}

/// Test ListConfigurationPolicyAssociations (stub)
#[tokio::test]
async fn test_list_configuration_policy_associations() {
    let client = make_client().await;

    let resp = client
        .list_configuration_policy_associations()
        .send()
        .await
        .expect("list_configuration_policy_associations should succeed");

    assert_eq!(resp.configuration_policy_association_summaries().len(), 0);
}

/// Test StartConfigurationPolicyAssociation (stub)
#[tokio::test]
async fn test_start_configuration_policy_association() {
    let client = make_client().await;

    let resp = client
        .start_configuration_policy_association()
        .configuration_policy_identifier("test-policy-id")
        .target(aws_sdk_securityhub::types::Target::AccountId(
            "123456789012".to_string(),
        ))
        .send()
        .await
        .expect("start_configuration_policy_association should succeed");

    // Returns the policy id and association details
    assert_eq!(resp.configuration_policy_id(), Some("test-policy-id"));
    assert_eq!(resp.target_id(), Some("123456789012"));
    assert_eq!(
        resp.association_status(),
        Some(&aws_sdk_securityhub::types::ConfigurationPolicyAssociationStatus::Success)
    );
}

/// Test StartConfigurationPolicyDisassociation (stub)
#[tokio::test]
async fn test_start_configuration_policy_disassociation() {
    let client = make_client().await;

    let resp = client
        .start_configuration_policy_disassociation()
        .configuration_policy_identifier("test-policy-id")
        .target(aws_sdk_securityhub::types::Target::AccountId(
            "123456789012".to_string(),
        ))
        .send()
        .await;

    assert!(
        resp.is_ok(),
        "start_configuration_policy_disassociation should succeed"
    );
}

// ============================================================================
// Finding pagination and token
// ============================================================================

/// Test GetFindings pagination with next_token
#[tokio::test]
async fn test_get_findings_pagination() {
    let client = make_client().await;

    // Import 5 findings
    let mut builder = client.batch_import_findings();
    for i in 1..=5 {
        let finding = make_finding(
            &format!("test-finding-{i:03}"),
            "us-east-1",
            &format!("Description {i}"),
            &format!("Title {i}"),
            &format!("resource-{i}"),
        );
        builder = builder.findings(finding);
    }
    builder.send().await.unwrap();

    // Page 1: 2 results
    let page1 = client.get_findings().max_results(2).send().await.unwrap();

    assert_eq!(page1.findings().len(), 2);
    let token1 = page1
        .next_token()
        .expect("should have next_token")
        .to_string();

    // Page 2: 2 more results
    let page2 = client
        .get_findings()
        .max_results(2)
        .next_token(&token1)
        .send()
        .await
        .unwrap();

    assert_eq!(page2.findings().len(), 2);
    let token2 = page2
        .next_token()
        .expect("should have next_token")
        .to_string();

    // Page 3: last result
    let page3 = client
        .get_findings()
        .max_results(2)
        .next_token(&token2)
        .send()
        .await
        .unwrap();

    assert_eq!(page3.findings().len(), 1);
    assert!(
        page3.next_token().is_none(),
        "no more pages after last result"
    );
}

/// Test finding update (re-import same ID)
#[tokio::test]
async fn test_finding_update_by_reimport() {
    let client = make_client().await;

    // Import initial finding
    let finding1 = make_finding(
        "test-update-001",
        "us-east-1",
        "Original description",
        "Original Title",
        "resource-1",
    );
    client
        .batch_import_findings()
        .findings(finding1)
        .send()
        .await
        .unwrap();

    // Re-import same finding with different title
    let finding2 = make_finding(
        "test-update-001",
        "us-east-1",
        "Updated description",
        "Updated Title",
        "resource-1",
    );
    let import_resp = client
        .batch_import_findings()
        .findings(finding2)
        .send()
        .await
        .unwrap();

    assert_eq!(import_resp.success_count(), Some(1));

    // Should only have one finding (not two)
    let findings = client.get_findings().send().await.unwrap();
    assert_eq!(findings.findings().len(), 1);
    assert_eq!(findings.findings()[0].id(), Some("test-update-001"));
    assert_eq!(findings.findings()[0].title(), Some("Updated Title"));
}

// ============================================================================
// UpdateFindings (stub)
// ============================================================================

/// Test UpdateFindings stub
#[tokio::test]
async fn test_update_findings() {
    let client = make_client().await;

    let resp = client
        .update_findings()
        .filters(aws_sdk_securityhub::types::AwsSecurityFindingFilters::builder().build())
        .send()
        .await;

    assert!(resp.is_ok(), "update_findings should succeed");
}

// ============================================================================
// DisassociateFromMasterAccount (legacy stub)
// ============================================================================

/// Test DisassociateFromMasterAccount (legacy stub)
#[tokio::test]
#[allow(deprecated)]
async fn test_disassociate_from_master_account() {
    let client = make_client().await;

    let resp = client.disassociate_from_master_account().send().await;

    assert!(
        resp.is_ok(),
        "disassociate_from_master_account should succeed"
    );
}

/// Test AcceptInvitation (legacy stub)
#[tokio::test]
#[allow(deprecated)]
async fn test_accept_invitation() {
    let client = make_client().await;

    let resp = client
        .accept_invitation()
        .master_id("111111111111")
        .invitation_id("test-invitation")
        .send()
        .await;

    assert!(resp.is_ok(), "accept_invitation should succeed");
}

// ============================================================================
// Multiple action targets
// ============================================================================

/// Test creating multiple action targets and listing them
#[tokio::test]
async fn test_multiple_action_targets() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    for i in 1..=3 {
        client
            .create_action_target()
            .name(format!("Action{i}"))
            .description(format!("Description {i}"))
            .id(format!("action-{i}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client.describe_action_targets().send().await.unwrap();

    assert_eq!(resp.action_targets().len(), 3);
}

// ============================================================================
// Connector V2 lifecycle
// ============================================================================

/// Test ConnectorV2: Create, Get, List, Update, Delete
#[tokio::test]
async fn test_connector_v2_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create
    let create_resp = client
        .create_connector_v2()
        .name("TestConnector")
        .description("A test connector")
        .send()
        .await
        .expect("create_connector_v2 should succeed");

    let connector_id = create_resp
        .connector_id()
        .expect("ConnectorId should be present")
        .to_string();
    assert!(create_resp.connector_arn().is_some());
    assert_eq!(
        create_resp.connector_status().map(|s| s.as_str()),
        Some("ACTIVE")
    );

    // List
    let list_resp = client
        .list_connectors_v2()
        .send()
        .await
        .expect("list_connectors_v2 should succeed");

    assert_eq!(list_resp.connectors().len(), 1);
    assert_eq!(
        list_resp.connectors()[0].connector_id(),
        Some(connector_id.as_str())
    );

    // Get
    let get_resp = client
        .get_connector_v2()
        .connector_id(&connector_id)
        .send()
        .await
        .expect("get_connector_v2 should succeed");

    assert_eq!(get_resp.connector_id(), Some(connector_id.as_str()));
    assert_eq!(get_resp.name(), Some("TestConnector"));
    assert_eq!(get_resp.description(), Some("A test connector"));

    // Update (connector_id is the only setter; the body-level fields are
    // serialised directly by the SDK)
    client
        .update_connector_v2()
        .connector_id(&connector_id)
        .send()
        .await
        .expect("update_connector_v2 should succeed");

    // Delete
    client
        .delete_connector_v2()
        .connector_id(&connector_id)
        .send()
        .await
        .expect("delete_connector_v2 should succeed");

    // Verify deleted
    let list_resp = client.list_connectors_v2().send().await.unwrap();
    assert_eq!(list_resp.connectors().len(), 0);
}

/// Test GetConnectorV2 not found
#[tokio::test]
async fn test_get_connector_v2_not_found() {
    let client = make_client().await;

    let result = client
        .get_connector_v2()
        .connector_id("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_connector_v2 should fail for nonexistent ID"
    );
}

/// Test DeleteConnectorV2 not found
#[tokio::test]
async fn test_delete_connector_v2_not_found() {
    let client = make_client().await;

    let result = client
        .delete_connector_v2()
        .connector_id("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_connector_v2 should fail for nonexistent ID"
    );
}

// ============================================================================
// Automation rules V2 lifecycle
// ============================================================================

/// Test AutomationRuleV2: Create, Get, List, Update, Delete
#[tokio::test]
async fn test_automation_rule_v2_lifecycle() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    // Create
    let create_resp = client
        .create_automation_rule_v2()
        .rule_name("TestRuleV2")
        .rule_status(aws_sdk_securityhub::types::RuleStatusV2::Enabled)
        .rule_order(10.0)
        .send()
        .await
        .expect("create_automation_rule_v2 should succeed");

    let rule_arn = create_resp
        .rule_arn()
        .expect("RuleArn should be present")
        .to_string();
    let rule_id = create_resp
        .rule_id()
        .expect("RuleId should be present")
        .to_string();
    assert!(rule_arn.contains("automation-rule-v2/"));

    // List
    let list_resp = client
        .list_automation_rules_v2()
        .send()
        .await
        .expect("list_automation_rules_v2 should succeed");

    assert_eq!(list_resp.rules().len(), 1);
    assert_eq!(list_resp.rules()[0].rule_name(), Some("TestRuleV2"));

    // Get
    let get_resp = client
        .get_automation_rule_v2()
        .identifier(&rule_id)
        .send()
        .await
        .expect("get_automation_rule_v2 should succeed");

    assert_eq!(get_resp.rule_arn(), Some(rule_arn.as_str()));
    assert_eq!(get_resp.rule_id(), Some(rule_id.as_str()));
    assert_eq!(get_resp.rule_name(), Some("TestRuleV2"));

    // Update
    client
        .update_automation_rule_v2()
        .identifier(&rule_id)
        .rule_name("UpdatedRuleV2")
        .rule_status(aws_sdk_securityhub::types::RuleStatusV2::Disabled)
        .send()
        .await
        .expect("update_automation_rule_v2 should succeed");

    // Verify update
    let get_resp = client
        .get_automation_rule_v2()
        .identifier(&rule_id)
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.rule_name(), Some("UpdatedRuleV2"));

    // Delete
    client
        .delete_automation_rule_v2()
        .identifier(&rule_id)
        .send()
        .await
        .expect("delete_automation_rule_v2 should succeed");

    // Verify deleted
    let list_resp = client.list_automation_rules_v2().send().await.unwrap();
    assert_eq!(list_resp.rules().len(), 0);
}

/// Test GetAutomationRuleV2 not found
#[tokio::test]
async fn test_get_automation_rule_v2_not_found() {
    let client = make_client().await;

    let result = client
        .get_automation_rule_v2()
        .identifier("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_automation_rule_v2 should fail for nonexistent ID"
    );
}

/// Test DeleteAutomationRuleV2 not found
#[tokio::test]
async fn test_delete_automation_rule_v2_not_found() {
    let client = make_client().await;

    let result = client
        .delete_automation_rule_v2()
        .identifier("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_automation_rule_v2 should fail for nonexistent ID"
    );
}

// ============================================================================
// UpdateInsight not found
// ============================================================================

/// Test UpdateInsight for nonexistent insight
#[tokio::test]
async fn test_update_insight_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .update_insight()
        .insight_arn("arn:aws:securityhub:us-east-1:123456789012:insight/nonexistent")
        .name("Updated")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_insight should fail for nonexistent ARN"
    );
}

// ============================================================================
// Findings with GetInsightResults for nonexistent
// ============================================================================

/// Test GetInsightResults for nonexistent insight
#[tokio::test]
async fn test_get_insight_results_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .get_insight_results()
        .insight_arn("arn:aws:securityhub:us-east-1:123456789012:insight/nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_insight_results should fail for nonexistent insight"
    );
}

// ============================================================================
// UpdateConnectorV2 not found
// ============================================================================

/// Test UpdateConnectorV2 not found
#[tokio::test]
async fn test_update_connector_v2_not_found() {
    let client = make_client().await;

    let result = client
        .update_connector_v2()
        .connector_id("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_connector_v2 should fail for nonexistent ID"
    );
}

// ============================================================================
// UpdateAutomationRuleV2 not found
// ============================================================================

/// Test UpdateAutomationRuleV2 not found
#[tokio::test]
async fn test_update_automation_rule_v2_not_found() {
    let client = make_client().await;

    let result = client
        .update_automation_rule_v2()
        .identifier("nonexistent")
        .rule_name("Updated")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_automation_rule_v2 should fail for nonexistent ID"
    );
}

// ============================================================================
// UpdateConfigurationPolicy not found
// ============================================================================

/// Test UpdateConfigurationPolicy not found
#[tokio::test]
async fn test_update_configuration_policy_not_found() {
    let client = make_client().await;

    client.enable_security_hub().send().await.unwrap();

    let result = client
        .update_configuration_policy()
        .identifier("nonexistent")
        .name("Updated")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_configuration_policy should fail for nonexistent ID"
    );
}

// ============================================================================
// RegisterConnectorV2 (stub)
// ============================================================================

/// Test RegisterConnectorV2 (stub)
#[tokio::test]
async fn test_register_connector_v2() {
    let client = make_client().await;

    let resp = client
        .register_connector_v2()
        .send()
        .await
        .expect("register_connector_v2 should succeed");

    // Stub returns a default response
    let _ = resp.connector_id();
}

// ============================================================================
// V2 stub operations
// ============================================================================

/// Test EnableSecurityHubV2 (stub)
#[tokio::test]
async fn test_enable_security_hub_v2() {
    let client = make_client().await;

    let resp = client
        .enable_security_hub_v2()
        .send()
        .await
        .expect("enable_security_hub_v2 should succeed");

    // Returns a real hub_v2_arn
    assert!(resp.hub_v2_arn().is_some());
    assert!(resp.hub_v2_arn().unwrap().contains("hub-v2/default"));
}

/// Test DescribeSecurityHubV2 (stub)
#[tokio::test]
async fn test_describe_security_hub_v2() {
    let client = make_client().await;

    let resp = client
        .describe_security_hub_v2()
        .send()
        .await
        .expect("describe_security_hub_v2 should succeed");

    assert!(resp.hub_v2_arn().is_none());
}

/// Test DisableSecurityHubV2 (stub)
#[tokio::test]
async fn test_disable_security_hub_v2() {
    let client = make_client().await;

    let resp = client.disable_security_hub_v2().send().await;

    assert!(resp.is_ok(), "disable_security_hub_v2 should succeed");
}

/// Test GetFindingsV2 (stub)
#[tokio::test]
async fn test_get_findings_v2() {
    let client = make_client().await;

    let resp = client
        .get_findings_v2()
        .send()
        .await
        .expect("get_findings_v2 should succeed");

    assert_eq!(resp.findings().len(), 0);
}

/// Test BatchUpdateFindingsV2 (stub)
#[tokio::test]
async fn test_batch_update_findings_v2() {
    let client = make_client().await;

    let resp = client
        .batch_update_findings_v2()
        .send()
        .await
        .expect("batch_update_findings_v2 should succeed");

    assert_eq!(resp.processed_findings().len(), 0);
    assert_eq!(resp.unprocessed_findings().len(), 0);
}

/// Test GetFindingStatisticsV2 (stub)
#[tokio::test]
async fn test_get_finding_statistics_v2() {
    let client = make_client().await;

    let resp = client
        .get_finding_statistics_v2()
        .send()
        .await
        .expect("get_finding_statistics_v2 should succeed");

    assert_eq!(resp.group_by_results().len(), 0);
}

/// Test GetFindingsTrendsV2 (stub)
#[tokio::test]
async fn test_get_findings_trends_v2() {
    let client = make_client().await;

    let resp = client
        .get_findings_trends_v2()
        .send()
        .await
        .expect("get_findings_trends_v2 should succeed");

    assert_eq!(resp.trends_metrics().len(), 0);
}

/// Test GetResourcesV2 (stub)
#[tokio::test]
async fn test_get_resources_v2() {
    let client = make_client().await;

    let resp = client
        .get_resources_v2()
        .send()
        .await
        .expect("get_resources_v2 should succeed");

    assert_eq!(resp.resources().len(), 0);
}

/// Test GetResourcesStatisticsV2 (stub)
#[tokio::test]
async fn test_get_resources_statistics_v2() {
    let client = make_client().await;

    let resp = client
        .get_resources_statistics_v2()
        .send()
        .await
        .expect("get_resources_statistics_v2 should succeed");

    assert_eq!(resp.group_by_results().len(), 0);
}

/// Test GetResourcesTrendsV2 (stub)
#[tokio::test]
async fn test_get_resources_trends_v2() {
    let client = make_client().await;

    let resp = client
        .get_resources_trends_v2()
        .send()
        .await
        .expect("get_resources_trends_v2 should succeed");

    assert_eq!(resp.trends_metrics().len(), 0);
}

/// Test DescribeProductsV2 (stub)
#[tokio::test]
async fn test_describe_products_v2() {
    let client = make_client().await;

    let resp = client
        .describe_products_v2()
        .send()
        .await
        .expect("describe_products_v2 should succeed");

    assert_eq!(resp.products_v2().len(), 0);
}

/// Test CreateAggregatorV2 (stub)
#[tokio::test]
async fn test_create_aggregator_v2() {
    let client = make_client().await;

    let resp = client
        .create_aggregator_v2()
        .region_linking_mode("ALL_REGIONS")
        .send()
        .await
        .expect("create_aggregator_v2 should succeed");

    // Returns a real aggregator_v2_arn
    assert!(resp.aggregator_v2_arn().is_some());
    assert_eq!(resp.region_linking_mode(), Some("ALL_REGIONS"));
}

/// Test ListAggregatorsV2 (stub)
#[tokio::test]
async fn test_list_aggregators_v2() {
    let client = make_client().await;

    let resp = client
        .list_aggregators_v2()
        .send()
        .await
        .expect("list_aggregators_v2 should succeed");

    assert_eq!(resp.aggregators_v2().len(), 0);
}

/// Test CreateTicketV2 (stub)
#[tokio::test]
async fn test_create_ticket_v2() {
    let client = make_client().await;

    let resp = client
        .create_ticket_v2()
        .connector_id("test-connector")
        .send()
        .await
        .expect("create_ticket_v2 should succeed");

    // Stub returns a default response
    let _ = resp.ticket_id();
}

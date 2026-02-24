use aws_sdk_securityhub::config::BehaviorVersion;
use aws_sdk_securityhub::types::{
    AccountDetails, AwsSecurityFinding, Resource, Severity, SeverityLabel,
    StandardsSubscriptionRequest, WorkflowStatus,
};
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

fn make_finding(id: &str, title: &str) -> AwsSecurityFinding {
    AwsSecurityFinding::builder()
        .aws_account_id("123456789012")
        .created_at("2024-01-01T00:00:00.000Z")
        .updated_at("2024-01-01T00:00:00.000Z")
        .description("Test finding")
        .generator_id("test-generator")
        .id(id)
        .product_arn("arn:aws:securityhub:us-east-1:123456789012:product/123456789012/default")
        .resources(
            Resource::builder()
                .id("arn:aws:ec2:us-east-1:123456789012:instance/i-1234567890abcdef0")
                .r#type("AwsEc2Instance")
                .build(),
        )
        .schema_version("2018-10-08")
        .severity(Severity::builder().label(SeverityLabel::High).build())
        .title(title)
        .types("Software and Configuration Checks")
        .build()
}

/// Scenario: Finding Triage Workflow
///
/// Enable hub → subscribe to standards → import findings → triage → tag hub →
/// disable standards. Chains 8 operations asserting business outcomes.
#[tokio::test]
async fn test_finding_triage_workflow() {
    let client = make_client().await;

    // 1. Enable Security Hub
    client
        .enable_security_hub()
        .enable_default_standards(false)
        .tags("env", "test")
        .send()
        .await
        .expect("enable_security_hub should succeed");

    let hub_info = client
        .describe_hub()
        .send()
        .await
        .expect("describe_hub should succeed");
    assert!(
        hub_info.hub_arn().is_some(),
        "HubArn must be present after enabling"
    );

    // 2. Subscribe to a standards pack
    let sub_resp = client
        .batch_enable_standards()
        .standards_subscription_requests(
            StandardsSubscriptionRequest::builder()
                .standards_arn(
                    "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
                )
                .build(),
        )
        .send()
        .await
        .expect("batch_enable_standards should succeed");
    let subs = sub_resp.standards_subscriptions();
    assert_eq!(subs.len(), 1, "One subscription should be created");
    let sub_arn = subs[0]
        .standards_subscription_arn()
        .expect("sub arn")
        .to_string();

    // 3. Import two findings
    let f1 = make_finding("finding-1", "Exposed S3 Bucket");
    let f2 = make_finding("finding-2", "Weak IAM Password Policy");
    let import_resp = client
        .batch_import_findings()
        .findings(f1)
        .findings(f2)
        .send()
        .await
        .expect("batch_import_findings should succeed");
    assert_eq!(
        import_resp.success_count(),
        Some(2),
        "Both findings must succeed"
    );
    assert_eq!(import_resp.failed_count(), Some(0), "No failures expected");

    // 4. Retrieve findings and verify count
    let get_resp = client
        .get_findings()
        .max_results(10)
        .send()
        .await
        .expect("get_findings should succeed");
    let findings = get_resp.findings();
    assert_eq!(
        findings.len(),
        2,
        "Both imported findings should be returned"
    );

    // 5. Update workflow status (triage: suppress finding-1)
    let update_resp = client
        .batch_update_findings()
        .finding_identifiers(
            aws_sdk_securityhub::types::AwsSecurityFindingIdentifier::builder()
                .id("finding-1")
                .product_arn(
                    "arn:aws:securityhub:us-east-1:123456789012:product/123456789012/default",
                )
                .build(),
        )
        .workflow(
            aws_sdk_securityhub::types::WorkflowUpdate::builder()
                .status(WorkflowStatus::Suppressed)
                .build(),
        )
        .send()
        .await
        .expect("batch_update_findings should succeed");
    assert_eq!(
        update_resp.processed_findings().len(),
        1,
        "finding-1 should be processed"
    );
    assert_eq!(
        update_resp.unprocessed_findings().len(),
        0,
        "No unprocessed findings"
    );

    // 6. Create a custom action target for escalation
    let action_resp = client
        .create_action_target()
        .name("EscalateCritical")
        .description("Escalate critical findings to security team")
        .id("EscalateCritical")
        .send()
        .await
        .expect("create_action_target should succeed");
    let action_arn = action_resp
        .action_target_arn()
        .expect("ActionTargetArn must be present");
    assert!(
        action_arn.contains("EscalateCritical"),
        "ARN should contain the custom action ID"
    );

    // 7. Tag the hub resource
    let hub_arn = hub_info
        .hub_arn()
        .expect("HubArn must be present")
        .to_string();
    client
        .tag_resource()
        .resource_arn(&hub_arn)
        .tags("severity", "high")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tag_resp = client
        .list_tags_for_resource()
        .resource_arn(&hub_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let empty_tags = std::collections::HashMap::new();
    let tags = tag_resp.tags().unwrap_or(&empty_tags);
    assert_eq!(
        tags.get("severity").map(|s| s.as_str()),
        Some("high"),
        "severity tag must be present"
    );
    assert_eq!(
        tags.get("env").map(|s| s.as_str()),
        Some("test"),
        "env tag from enable must be present"
    );

    // 8. Disable the standards subscription and verify removal
    client
        .batch_disable_standards()
        .standards_subscription_arns(sub_arn)
        .send()
        .await
        .expect("batch_disable_standards should succeed");

    let enabled = client
        .get_enabled_standards()
        .send()
        .await
        .expect("get_enabled_standards should succeed");
    assert_eq!(
        enabled.standards_subscriptions().len(),
        0,
        "No standards should remain enabled after disabling"
    );
}

/// Scenario: Members Management Workflow
///
/// Create members → list → get by ID → delete → verify removal.
/// Asserts multi-account member lifecycle business outcomes.
#[tokio::test]
async fn test_members_management_workflow() {
    let client = make_client().await;

    // Enable hub first
    client
        .enable_security_hub()
        .send()
        .await
        .expect("enable_security_hub");

    // 1. Create two members
    let create_resp = client
        .create_members()
        .account_details(
            AccountDetails::builder()
                .account_id("111111111111")
                .email("member1@example.com")
                .build(),
        )
        .account_details(
            AccountDetails::builder()
                .account_id("222222222222")
                .email("member2@example.com")
                .build(),
        )
        .send()
        .await
        .expect("create_members should succeed");
    assert_eq!(
        create_resp.unprocessed_accounts().len(),
        0,
        "No unprocessed accounts"
    );

    // 2. List members and assert both present
    let list_resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");
    let members = list_resp.members();
    assert_eq!(members.len(), 2, "Both members should be listed");

    // 3. Get specific member by account ID
    let get_resp = client
        .get_members()
        .account_ids("111111111111")
        .send()
        .await
        .expect("get_members should succeed");
    let got = get_resp.members();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0].account_id().unwrap_or_default(),
        "111111111111",
        "Correct member returned"
    );

    // 4. Delete one member
    client
        .delete_members()
        .account_ids("111111111111")
        .send()
        .await
        .expect("delete_members should succeed");

    // 5. Verify only one member remains
    let list2 = client
        .list_members()
        .send()
        .await
        .expect("list_members after delete");
    let remaining = list2.members();
    assert_eq!(
        remaining.len(),
        1,
        "One member should remain after deletion"
    );
    assert_eq!(
        remaining[0].account_id().unwrap_or_default(),
        "222222222222"
    );
}

/// Scenario: Automation Rule Lifecycle
///
/// Create automation rule → list → batch get → batch update → batch delete.
/// Validates rule management across 5 chained API operations.
#[tokio::test]
async fn test_automation_rule_lifecycle() {
    let client = make_client().await;

    client
        .enable_security_hub()
        .send()
        .await
        .expect("enable_security_hub");

    // 1. Create an automation rule
    let create_resp = client
        .create_automation_rule()
        .rule_name("SuppressTestFindings")
        .rule_order(1)
        .description("Suppresses test environment findings")
        .is_terminal(false)
        .send()
        .await
        .expect("create_automation_rule should succeed");
    let rule_arn = create_resp.rule_arn().expect("RuleArn must be present");
    assert!(
        rule_arn.contains("automation-rule"),
        "ARN should contain automation-rule"
    );

    // 2. List automation rules
    let list_resp = client
        .list_automation_rules()
        .send()
        .await
        .expect("list_automation_rules");
    let rules = list_resp.automation_rules_metadata();
    assert_eq!(rules.len(), 1, "One rule should be listed");
    assert_eq!(rules[0].rule_arn().unwrap_or_default(), rule_arn);

    // 3. Batch get the rule
    let get_resp = client
        .batch_get_automation_rules()
        .automation_rules_arns(rule_arn)
        .send()
        .await
        .expect("batch_get_automation_rules");
    let fetched = get_resp.rules();
    assert_eq!(fetched.len(), 1);
    assert_eq!(
        fetched[0].rule_name().unwrap_or_default(),
        "SuppressTestFindings"
    );
    let fetched_arn = fetched[0].rule_arn().expect("rule arn").to_string();

    // 4. Batch update the rule
    let update_resp = client
        .batch_update_automation_rules()
        .update_automation_rules_request_items(
            aws_sdk_securityhub::types::UpdateAutomationRulesRequestItem::builder()
                .rule_arn(&fetched_arn)
                .rule_name("SuppressTestFindings-v2")
                .rule_order(2)
                .build(),
        )
        .send()
        .await
        .expect("batch_update_automation_rules");
    assert_eq!(
        update_resp.processed_automation_rules().len(),
        1,
        "Rule should be updated"
    );

    // 5. Batch delete the rule
    let delete_resp = client
        .batch_delete_automation_rules()
        .automation_rules_arns(&fetched_arn)
        .send()
        .await
        .expect("batch_delete_automation_rules");
    assert_eq!(
        delete_resp.processed_automation_rules().len(),
        1,
        "Rule should be deleted"
    );

    // 6. List should now be empty
    let list2 = client
        .list_automation_rules()
        .send()
        .await
        .expect("list_automation_rules after delete");
    assert_eq!(
        list2.automation_rules_metadata().len(),
        0,
        "No rules should remain"
    );
}

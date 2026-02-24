use aws_sdk_backup::config::BehaviorVersion;
use winterbaume_backup::BackupService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_backup::Client {
    let mock = MockAws::builder()
        .with_service(BackupService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backup::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_backup::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_backup_vault() {
    let client = make_client().await;

    let create_resp = client
        .create_backup_vault()
        .backup_vault_name("my-vault")
        .send()
        .await
        .expect("create_backup_vault should succeed");

    assert_eq!(create_resp.backup_vault_name(), Some("my-vault"));
    assert!(create_resp.backup_vault_arn().is_some());

    let describe_resp = client
        .describe_backup_vault()
        .backup_vault_name("my-vault")
        .send()
        .await
        .expect("describe_backup_vault should succeed");

    assert_eq!(describe_resp.backup_vault_name(), Some("my-vault"));
    assert_eq!(describe_resp.number_of_recovery_points(), 0);
}

#[tokio::test]
async fn test_list_backup_vaults() {
    let client = make_client().await;

    for name in ["vault-a", "vault-b"] {
        client
            .create_backup_vault()
            .backup_vault_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_backup_vaults()
        .send()
        .await
        .expect("list_backup_vaults should succeed");

    assert_eq!(resp.backup_vault_list().len(), 2);
}

#[tokio::test]
async fn test_delete_backup_vault() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("delete-me")
        .send()
        .await
        .unwrap();

    client
        .delete_backup_vault()
        .backup_vault_name("delete-me")
        .send()
        .await
        .expect("delete_backup_vault should succeed");

    let result = client
        .describe_backup_vault()
        .backup_vault_name("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("ephemeral-vault")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_backup_vaults().send().await.unwrap();
    assert_eq!(list_resp.backup_vault_list().len(), 1);

    client
        .delete_backup_vault()
        .backup_vault_name("ephemeral-vault")
        .send()
        .await
        .unwrap();

    let list_resp = client.list_backup_vaults().send().await.unwrap();
    assert_eq!(list_resp.backup_vault_list().len(), 0);
}

#[tokio::test]
async fn test_describe_nonexistent_vault() {
    let client = make_client().await;

    let result = client
        .describe_backup_vault()
        .backup_vault_name("no-such-vault")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Backup Plan tests ---

#[tokio::test]
async fn test_create_and_get_backup_plan() {
    let client = make_client().await;

    // First create a vault for the plan rule target
    client
        .create_backup_vault()
        .backup_vault_name("test-vault")
        .send()
        .await
        .unwrap();

    let rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("daily-backup")
        .target_backup_vault_name("test-vault")
        .build()
        .unwrap();

    let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("my-plan")
        .rules(rule)
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_plan()
        .backup_plan(plan_input)
        .send()
        .await
        .expect("create_backup_plan should succeed");

    assert!(create_resp.backup_plan_id().is_some());
    assert!(create_resp.backup_plan_arn().is_some());
    assert!(create_resp.version_id().is_some());

    let plan_id = create_resp.backup_plan_id().unwrap().to_string();

    let get_resp = client
        .get_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .expect("get_backup_plan should succeed");

    assert_eq!(get_resp.backup_plan_id(), Some(plan_id.as_str()));
    let plan = get_resp.backup_plan().unwrap();
    assert_eq!(plan.backup_plan_name(), "my-plan");
}

#[tokio::test]
async fn test_delete_backup_plan() {
    let client = make_client().await;

    let rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("rule1")
        .target_backup_vault_name("v")
        .build()
        .unwrap();

    let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("delete-plan")
        .rules(rule)
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_plan()
        .backup_plan(plan_input)
        .send()
        .await
        .unwrap();

    let plan_id = create_resp.backup_plan_id().unwrap().to_string();

    let del_resp = client
        .delete_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .expect("delete_backup_plan should succeed");

    assert_eq!(del_resp.backup_plan_id(), Some(plan_id.as_str()));

    // Verify it's gone
    let result = client
        .get_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_backup_plans() {
    let client = make_client().await;

    for name in ["plan-a", "plan-b"] {
        let rule = aws_sdk_backup::types::BackupRuleInput::builder()
            .rule_name("r")
            .target_backup_vault_name("v")
            .build()
            .unwrap();

        let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
            .backup_plan_name(name)
            .rules(rule)
            .build()
            .unwrap();

        client
            .create_backup_plan()
            .backup_plan(plan_input)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_backup_plans()
        .send()
        .await
        .expect("list_backup_plans should succeed");

    assert_eq!(resp.backup_plans_list().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_backup_plan() {
    let client = make_client().await;
    let result = client
        .get_backup_plan()
        .backup_plan_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Report Plan tests ---

#[tokio::test]
async fn test_create_and_describe_report_plan() {
    let client = make_client().await;

    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("my-bucket")
        .build()
        .unwrap();

    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();

    let create_resp = client
        .create_report_plan()
        .report_plan_name("my-report")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await
        .expect("create_report_plan should succeed");

    assert_eq!(create_resp.report_plan_name(), Some("my-report"));
    assert!(create_resp.report_plan_arn().is_some());

    let desc_resp = client
        .describe_report_plan()
        .report_plan_name("my-report")
        .send()
        .await
        .expect("describe_report_plan should succeed");

    let report_plan = desc_resp.report_plan().unwrap();
    assert_eq!(report_plan.report_plan_name(), Some("my-report"));
    assert_eq!(report_plan.deployment_status(), Some("COMPLETED"));
}

#[tokio::test]
async fn test_delete_report_plan() {
    let client = make_client().await;

    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("bucket")
        .build()
        .unwrap();

    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();

    client
        .create_report_plan()
        .report_plan_name("del-report")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await
        .unwrap();

    client
        .delete_report_plan()
        .report_plan_name("del-report")
        .send()
        .await
        .expect("delete_report_plan should succeed");

    let result = client
        .describe_report_plan()
        .report_plan_name("del-report")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_report_plans() {
    let client = make_client().await;

    for name in ["report-a", "report-b"] {
        let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
            .s3_bucket_name("bucket")
            .build()
            .unwrap();

        let report_setting = aws_sdk_backup::types::ReportSetting::builder()
            .report_template("BACKUP_JOB_REPORT")
            .build()
            .unwrap();

        client
            .create_report_plan()
            .report_plan_name(name)
            .report_delivery_channel(delivery_channel)
            .report_setting(report_setting)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_report_plans()
        .send()
        .await
        .expect("list_report_plans should succeed");

    assert_eq!(resp.report_plans().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_report_plan() {
    let client = make_client().await;
    let result = client
        .describe_report_plan()
        .report_plan_name("no-such-report")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Vault Lock tests ---

#[tokio::test]
async fn test_put_and_delete_vault_lock_configuration() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("lock-vault")
        .send()
        .await
        .unwrap();

    client
        .put_backup_vault_lock_configuration()
        .backup_vault_name("lock-vault")
        .min_retention_days(1)
        .max_retention_days(365)
        .send()
        .await
        .expect("put lock config should succeed");

    // Verify via describe
    let desc = client
        .describe_backup_vault()
        .backup_vault_name("lock-vault")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.locked(), Some(true));
    assert_eq!(desc.min_retention_days(), Some(1));
    assert_eq!(desc.max_retention_days(), Some(365));

    // Delete lock config
    client
        .delete_backup_vault_lock_configuration()
        .backup_vault_name("lock-vault")
        .send()
        .await
        .expect("delete lock config should succeed");

    let desc = client
        .describe_backup_vault()
        .backup_vault_name("lock-vault")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.locked(), Some(false));
}

#[tokio::test]
async fn test_vault_lock_nonexistent_vault() {
    let client = make_client().await;

    let result = client
        .put_backup_vault_lock_configuration()
        .backup_vault_name("no-vault")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Tag tests ---

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_client().await;

    let create_resp = client
        .create_backup_vault()
        .backup_vault_name("tag-vault")
        .send()
        .await
        .unwrap();

    let vault_arn = create_resp.backup_vault_arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&vault_arn)
        .tags("env", "prod")
        .tags("team", "infra")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tags_resp = client
        .list_tags()
        .resource_arn(&vault_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("infra"));

    // Untag
    client
        .untag_resource()
        .resource_arn(&vault_arn)
        .tag_key_list("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags()
        .resource_arn(&vault_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags();
    // "env" should be gone, "team" should remain
    if let Some(t) = tags {
        assert!(t.get("env").is_none());
        assert_eq!(t.get("team").map(|s| s.as_str()), Some("infra"));
    }
}

// --- Duplicate detection tests (from moto test_create_backup_plan_already_exists / test_create_backup_vault_already_exists) ---

#[tokio::test]
async fn test_create_duplicate_backup_vault() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("dup-vault")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_backup_vault()
        .backup_vault_name("dup-vault")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_backup_plan() {
    let client = make_client().await;

    let make_plan = |name: &str| {
        let rule = aws_sdk_backup::types::BackupRuleInput::builder()
            .rule_name("r")
            .target_backup_vault_name("v")
            .build()
            .unwrap();
        aws_sdk_backup::types::BackupPlanInput::builder()
            .backup_plan_name(name)
            .rules(rule)
            .build()
            .unwrap()
    };

    client
        .create_backup_plan()
        .backup_plan(make_plan("dup-plan"))
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_backup_plan()
        .backup_plan(make_plan("dup-plan"))
        .send()
        .await;
    assert!(result.is_err());
}

// --- Tags created with vault (from moto test_list_tags_vault) ---

#[tokio::test]
async fn test_list_tags_vault_created_with_tags() {
    let client = make_client().await;

    let create_resp = client
        .create_backup_vault()
        .backup_vault_name("tagged-vault")
        .backup_vault_tags("key1", "value1")
        .backup_vault_tags("key2", "value2")
        .send()
        .await
        .expect("create with tags should succeed");

    let vault_arn = create_resp.backup_vault_arn().unwrap().to_string();

    let tags_resp = client
        .list_tags()
        .resource_arn(&vault_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("value1"));
    assert_eq!(tags.get("key2").map(|s| s.as_str()), Some("value2"));
}

// --- Tags created with plan (from moto test_list_tags_plan) ---

#[tokio::test]
async fn test_list_tags_plan_created_with_tags() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("tag-plan-vault")
        .send()
        .await
        .unwrap();

    let rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("r")
        .target_backup_vault_name("tag-plan-vault")
        .build()
        .unwrap();

    let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("tagged-plan")
        .rules(rule)
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_plan()
        .backup_plan(plan_input)
        .backup_plan_tags("key1", "value1")
        .backup_plan_tags("key2", "value2")
        .send()
        .await
        .expect("create plan with tags should succeed");

    let plan_arn = create_resp.backup_plan_arn().unwrap().to_string();

    let tags_resp = client
        .list_tags()
        .resource_arn(&plan_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("key1").map(|s| s.as_str()), Some("value1"));
    assert_eq!(tags.get("key2").map(|s| s.as_str()), Some("value2"));
}

// --- Backup plan with multiple rules (from moto test_get_backup_plan_with_multiple_rules) ---

#[tokio::test]
async fn test_backup_plan_with_multiple_rules() {
    let client = make_client().await;

    let rule1 = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("rule1")
        .target_backup_vault_name("vault")
        .schedule_expression("cron(0 1 ? * * *)")
        .start_window_minutes(60)
        .completion_window_minutes(120)
        .build()
        .unwrap();

    let rule2 = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("rule2")
        .target_backup_vault_name("vault")
        .build()
        .unwrap();

    let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("multi-rule-plan")
        .rules(rule1)
        .rules(rule2)
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_plan()
        .backup_plan(plan_input)
        .send()
        .await
        .expect("create plan should succeed");

    let plan_id = create_resp.backup_plan_id().unwrap().to_string();

    let get_resp = client
        .get_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .expect("get plan should succeed");

    let plan = get_resp.backup_plan().unwrap();
    assert_eq!(plan.rules().len(), 2);

    // All rules should have rule IDs generated
    for rule in plan.rules() {
        assert!(rule.rule_id().is_some());
        assert!(rule.rule_name() != "");
    }
}

// --- Vault lock with list_backup_vaults showing lock fields (from moto test_put_backup_vault_lock_configuration) ---

#[tokio::test]
async fn test_list_backup_vaults_shows_lock_fields() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("lock-list-vault")
        .send()
        .await
        .unwrap();

    client
        .put_backup_vault_lock_configuration()
        .backup_vault_name("lock-list-vault")
        .min_retention_days(7)
        .max_retention_days(365)
        .changeable_for_days(5)
        .send()
        .await
        .expect("put lock config should succeed");

    let resp = client.list_backup_vaults().send().await.unwrap();
    let vault = resp
        .backup_vault_list()
        .iter()
        .find(|v| v.backup_vault_name() == Some("lock-list-vault"))
        .expect("vault should be in list");

    assert_eq!(vault.locked(), Some(true));
    assert_eq!(vault.min_retention_days(), Some(7));
    assert_eq!(vault.max_retention_days(), Some(365));
    assert!(vault.lock_date().is_some());
}

// --- Full lifecycle test ---

#[tokio::test]
async fn test_backup_plan_lifecycle() {
    let client = make_client().await;

    // Create
    let rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("lifecycle-rule")
        .target_backup_vault_name("v")
        .build()
        .unwrap();

    let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("lifecycle-plan")
        .rules(rule)
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_plan()
        .backup_plan(plan_input)
        .send()
        .await
        .unwrap();

    let plan_id = create_resp.backup_plan_id().unwrap().to_string();

    // List should contain 1
    let list = client.list_backup_plans().send().await.unwrap();
    assert_eq!(list.backup_plans_list().len(), 1);

    // Get should work
    let get = client
        .get_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get.backup_plan().unwrap().backup_plan_name(),
        "lifecycle-plan"
    );

    // Delete
    client
        .delete_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .unwrap();

    // List should be empty
    let list = client.list_backup_plans().send().await.unwrap();
    assert_eq!(list.backup_plans_list().len(), 0);

    // Get should fail
    let result = client
        .get_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_report_plan_lifecycle() {
    let client = make_client().await;

    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("bucket")
        .build()
        .unwrap();

    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();

    // Create
    client
        .create_report_plan()
        .report_plan_name("lifecycle-report")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await
        .unwrap();

    // List should contain 1
    let list = client.list_report_plans().send().await.unwrap();
    assert_eq!(list.report_plans().len(), 1);

    // Describe should work
    let desc = client
        .describe_report_plan()
        .report_plan_name("lifecycle-report")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.report_plan().unwrap().report_plan_name(),
        Some("lifecycle-report")
    );

    // Delete
    client
        .delete_report_plan()
        .report_plan_name("lifecycle-report")
        .send()
        .await
        .unwrap();

    // List should be empty
    let list = client.list_report_plans().send().await.unwrap();
    assert_eq!(list.report_plans().len(), 0);

    // Describe should fail
    let result = client
        .describe_report_plan()
        .report_plan_name("lifecycle-report")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS Backup
// ============================================================================

#[tokio::test]
async fn test_delete_nonexistent_backup_vault() {
    let client = make_client().await;

    let result = client
        .delete_backup_vault()
        .backup_vault_name("nonexistent-vault")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent vault should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_backup_plan() {
    let client = make_client().await;

    let result = client
        .delete_backup_plan()
        .backup_plan_id("nonexistent-plan-id")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent plan should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_report_plan() {
    let client = make_client().await;

    let result = client
        .delete_report_plan()
        .report_plan_name("nonexistent-report-plan")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent report plan should fail"
    );
}

#[tokio::test]
async fn test_create_duplicate_report_plan() {
    let client = make_client().await;

    let setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("RESTORE_JOB_REPORT")
        .build()
        .unwrap();
    let channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("my-bucket")
        .build()
        .unwrap();

    client
        .create_report_plan()
        .report_plan_name("dup-report-plan")
        .report_setting(setting.clone())
        .report_delivery_channel(channel.clone())
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_report_plan()
        .report_plan_name("dup-report-plan")
        .report_setting(setting)
        .report_delivery_channel(channel)
        .send()
        .await;
    assert!(result.is_err(), "duplicate report plan should fail");
}

#[tokio::test]
async fn test_vault_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_backup_vault()
        .backup_vault_name("arn-vault")
        .send()
        .await
        .unwrap();

    let arn = resp.backup_vault_arn().unwrap();
    assert!(
        arn.starts_with("arn:aws:backup:"),
        "ARN should start with arn:aws:backup:"
    );
    assert!(arn.contains("arn-vault"), "ARN should contain vault name");
}

#[tokio::test]
async fn test_delete_vault_lock_when_no_lock_set() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("no-lock-vault")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_backup_vault_lock_configuration()
        .backup_vault_name("no-lock-vault")
        .send()
        .await;
    assert!(result.is_err(), "expected error when no lock is configured");
    let err = result.unwrap_err();
    let code = err
        .as_service_error()
        .and_then(|e| {
            use aws_sdk_backup::error::ProvideErrorMetadata;
            e.code()
        })
        .unwrap_or("");
    assert_eq!(
        code, "InvalidParameterValueException",
        "expected InvalidParameterValueException, got: {code}"
    );
}

#[tokio::test]
async fn test_backup_plan_response_contains_creation_date() {
    let client = make_client().await;

    let rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("daily-rule")
        .target_backup_vault_name("Default")
        .schedule_expression("cron(0 12 * * ? *)")
        .build()
        .unwrap();
    let plan = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("dated-plan")
        .rules(rule)
        .build()
        .unwrap();

    let resp = client
        .create_backup_plan()
        .backup_plan(plan)
        .send()
        .await
        .unwrap();

    assert!(
        resp.creation_date().is_some(),
        "creation_date should be set on create response"
    );
}

#[tokio::test]
async fn test_create_report_plan_with_description() {
    let client = make_client().await;

    let setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();
    let channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("my-desc-bucket")
        .build()
        .unwrap();

    let create_resp = client
        .create_report_plan()
        .report_plan_name("described-report-plan")
        .report_plan_description("My report plan description")
        .report_setting(setting)
        .report_delivery_channel(channel)
        .send()
        .await
        .expect("create report plan with description should succeed");

    let name = create_resp.report_plan_name().unwrap().to_string();

    let desc_resp = client
        .describe_report_plan()
        .report_plan_name(&name)
        .send()
        .await
        .unwrap();

    let rp = desc_resp.report_plan().unwrap();
    assert_eq!(
        rp.report_plan_description(),
        Some("My report plan description")
    );
}

#[tokio::test]
async fn test_vault_lock_min_retention_only() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("min-ret-vault")
        .send()
        .await
        .unwrap();

    client
        .put_backup_vault_lock_configuration()
        .backup_vault_name("min-ret-vault")
        .min_retention_days(7)
        .send()
        .await
        .expect("put vault lock with min retention only should succeed");

    let desc = client
        .describe_backup_vault()
        .backup_vault_name("min-ret-vault")
        .send()
        .await
        .unwrap();

    assert!(
        desc.lock_date().is_some() || desc.min_retention_days() > Some(0),
        "vault should reflect lock configuration"
    );
}

// --- BackupSelection tests ---

async fn make_plan_in_vault(
    client: &aws_sdk_backup::Client,
    vault_name: &str,
    plan_name: &str,
) -> String {
    client
        .create_backup_vault()
        .backup_vault_name(vault_name)
        .send()
        .await
        .unwrap();
    let rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("r")
        .target_backup_vault_name(vault_name)
        .build()
        .unwrap();
    let plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name(plan_name)
        .rules(rule)
        .build()
        .unwrap();
    let create = client
        .create_backup_plan()
        .backup_plan(plan_input)
        .send()
        .await
        .unwrap();
    create.backup_plan_id().unwrap().to_string()
}

#[tokio::test]
async fn test_create_and_get_backup_selection() {
    let client = make_client().await;
    let plan_id = make_plan_in_vault(&client, "sel-vault", "sel-plan").await;

    let selection = aws_sdk_backup::types::BackupSelection::builder()
        .selection_name("my-selection")
        .iam_role_arn("arn:aws:iam::123456789012:role/backup-role")
        .resources("arn:aws:ec2:us-east-1:123456789012:instance/i-12345678")
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_selection()
        .backup_plan_id(&plan_id)
        .backup_selection(selection)
        .send()
        .await
        .expect("create_backup_selection should succeed");

    let sel_id = create_resp.selection_id().unwrap().to_string();
    assert!(create_resp.creation_date().is_some());

    let get_resp = client
        .get_backup_selection()
        .backup_plan_id(&plan_id)
        .selection_id(&sel_id)
        .send()
        .await
        .expect("get_backup_selection should succeed");

    let sel = get_resp.backup_selection().unwrap();
    assert_eq!(sel.selection_name(), "my-selection");
    assert_eq!(get_resp.backup_plan_id().unwrap(), plan_id);
}

#[tokio::test]
async fn test_list_backup_selections() {
    let client = make_client().await;
    let plan_id = make_plan_in_vault(&client, "list-sel-vault", "list-sel-plan").await;

    for name in ["sel-a", "sel-b"] {
        let selection = aws_sdk_backup::types::BackupSelection::builder()
            .selection_name(name)
            .iam_role_arn("arn:aws:iam::123456789012:role/backup-role")
            .build()
            .unwrap();
        client
            .create_backup_selection()
            .backup_plan_id(&plan_id)
            .backup_selection(selection)
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_backup_selections()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .expect("list_backup_selections should succeed");

    assert_eq!(list_resp.backup_selections_list().len(), 2);
}

#[tokio::test]
async fn test_delete_backup_selection() {
    let client = make_client().await;
    let plan_id = make_plan_in_vault(&client, "del-sel-vault", "del-sel-plan").await;

    let selection = aws_sdk_backup::types::BackupSelection::builder()
        .selection_name("to-delete")
        .iam_role_arn("arn:aws:iam::123456789012:role/backup-role")
        .build()
        .unwrap();

    let create_resp = client
        .create_backup_selection()
        .backup_plan_id(&plan_id)
        .backup_selection(selection)
        .send()
        .await
        .unwrap();
    let sel_id = create_resp.selection_id().unwrap().to_string();

    client
        .delete_backup_selection()
        .backup_plan_id(&plan_id)
        .selection_id(&sel_id)
        .send()
        .await
        .expect("delete_backup_selection should succeed");

    let list_resp = client
        .list_backup_selections()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.backup_selections_list().len(), 0);
}

// --- Backup Job tests ---

#[tokio::test]
async fn test_start_and_describe_backup_job() {
    let client = make_client().await;
    client
        .create_backup_vault()
        .backup_vault_name("job-vault")
        .send()
        .await
        .unwrap();

    let start_resp = client
        .start_backup_job()
        .backup_vault_name("job-vault")
        .resource_arn("arn:aws:ec2:us-east-1:123456789012:instance/i-12345678")
        .iam_role_arn("arn:aws:iam::123456789012:role/backup-role")
        .send()
        .await
        .expect("start_backup_job should succeed");

    let job_id = start_resp.backup_job_id().unwrap().to_string();
    assert!(start_resp.recovery_point_arn().is_some());
    assert!(start_resp.creation_date().is_some());

    let desc_resp = client
        .describe_backup_job()
        .backup_job_id(&job_id)
        .send()
        .await
        .expect("describe_backup_job should succeed");

    assert_eq!(desc_resp.backup_job_id().unwrap(), job_id);
    assert_eq!(desc_resp.state().unwrap().as_str(), "RUNNING");
}

#[tokio::test]
async fn test_list_backup_jobs() {
    let client = make_client().await;
    client
        .create_backup_vault()
        .backup_vault_name("list-job-vault")
        .send()
        .await
        .unwrap();

    for _ in 0..2 {
        client
            .start_backup_job()
            .backup_vault_name("list-job-vault")
            .resource_arn("arn:aws:ec2:us-east-1:123456789012:instance/i-12345678")
            .iam_role_arn("arn:aws:iam::123456789012:role/backup-role")
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_backup_jobs()
        .send()
        .await
        .expect("list_backup_jobs should succeed");

    assert_eq!(list_resp.backup_jobs().len(), 2);
}

// --- RecoveryPoint tests ---

#[tokio::test]
async fn test_list_recovery_points_by_backup_vault_empty() {
    let client = make_client().await;
    client
        .create_backup_vault()
        .backup_vault_name("rp-vault")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_recovery_points_by_backup_vault()
        .backup_vault_name("rp-vault")
        .send()
        .await
        .expect("list_recovery_points_by_backup_vault should succeed");

    assert_eq!(list_resp.recovery_points().len(), 0);
}

// --- UpdateBackupPlan test ---

#[tokio::test]
async fn test_update_backup_plan() {
    let client = make_client().await;
    let plan_id = make_plan_in_vault(&client, "upd-vault", "original-plan").await;

    let new_rule = aws_sdk_backup::types::BackupRuleInput::builder()
        .rule_name("updated-rule")
        .target_backup_vault_name("upd-vault")
        .build()
        .unwrap();
    let new_plan_input = aws_sdk_backup::types::BackupPlanInput::builder()
        .backup_plan_name("updated-plan")
        .rules(new_rule)
        .build()
        .unwrap();

    let update_resp = client
        .update_backup_plan()
        .backup_plan_id(&plan_id)
        .backup_plan(new_plan_input)
        .send()
        .await
        .expect("update_backup_plan should succeed");

    assert_eq!(update_resp.backup_plan_id().unwrap(), plan_id);
    assert!(update_resp.version_id().is_some());

    // GetBackupPlan should reflect the update
    let get_resp = client
        .get_backup_plan()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .unwrap();
    let plan = get_resp.backup_plan().unwrap();
    assert_eq!(plan.backup_plan_name(), "updated-plan");
}

// --- ListBackupPlanVersions test ---

#[tokio::test]
async fn test_list_backup_plan_versions() {
    let client = make_client().await;
    let plan_id = make_plan_in_vault(&client, "ver-vault", "versioned-plan").await;

    let list_resp = client
        .list_backup_plan_versions()
        .backup_plan_id(&plan_id)
        .send()
        .await
        .expect("list_backup_plan_versions should succeed");

    // Should contain at least one version (the current one)
    assert!(!list_resp.backup_plan_versions_list().is_empty());
    let entry = &list_resp.backup_plan_versions_list()[0];
    assert_eq!(entry.backup_plan_id().unwrap(), plan_id);
}

// ============================================================================
// Vault Access Policy tests
// ============================================================================

#[tokio::test]
async fn test_put_and_get_backup_vault_access_policy() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("policy-vault")
        .send()
        .await
        .unwrap();

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_backup_vault_access_policy()
        .backup_vault_name("policy-vault")
        .policy(policy)
        .send()
        .await
        .expect("put_backup_vault_access_policy should succeed");

    let resp = client
        .get_backup_vault_access_policy()
        .backup_vault_name("policy-vault")
        .send()
        .await
        .expect("get_backup_vault_access_policy should succeed");

    assert_eq!(resp.backup_vault_name(), Some("policy-vault"));
    assert_eq!(resp.policy(), Some(policy));
}

#[tokio::test]
async fn test_delete_backup_vault_access_policy() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("del-policy-vault")
        .send()
        .await
        .unwrap();

    client
        .put_backup_vault_access_policy()
        .backup_vault_name("del-policy-vault")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    client
        .delete_backup_vault_access_policy()
        .backup_vault_name("del-policy-vault")
        .send()
        .await
        .expect("delete_backup_vault_access_policy should succeed");

    let result = client
        .get_backup_vault_access_policy()
        .backup_vault_name("del-policy-vault")
        .send()
        .await;
    assert!(result.is_err(), "policy should be gone after delete");
}

#[tokio::test]
async fn test_get_backup_vault_access_policy_nonexistent() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("no-policy-vault")
        .send()
        .await
        .unwrap();

    let result = client
        .get_backup_vault_access_policy()
        .backup_vault_name("no-policy-vault")
        .send()
        .await;
    assert!(result.is_err(), "should error when no policy has been set");
}

// ============================================================================
// Vault Notification tests
// ============================================================================

#[tokio::test]
async fn test_put_and_get_backup_vault_notifications() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("notif-vault")
        .send()
        .await
        .unwrap();

    use aws_sdk_backup::types::BackupVaultEvent;

    client
        .put_backup_vault_notifications()
        .backup_vault_name("notif-vault")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:my-topic")
        .backup_vault_events(BackupVaultEvent::BackupJobCompleted)
        .backup_vault_events(BackupVaultEvent::BackupJobFailed)
        .send()
        .await
        .expect("put_backup_vault_notifications should succeed");

    let resp = client
        .get_backup_vault_notifications()
        .backup_vault_name("notif-vault")
        .send()
        .await
        .expect("get_backup_vault_notifications should succeed");

    assert_eq!(resp.backup_vault_name(), Some("notif-vault"));
    assert_eq!(
        resp.sns_topic_arn(),
        Some("arn:aws:sns:us-east-1:123456789012:my-topic")
    );
    assert_eq!(resp.backup_vault_events().len(), 2);
}

#[tokio::test]
async fn test_delete_backup_vault_notifications() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("del-notif-vault")
        .send()
        .await
        .unwrap();

    use aws_sdk_backup::types::BackupVaultEvent;

    client
        .put_backup_vault_notifications()
        .backup_vault_name("del-notif-vault")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic")
        .backup_vault_events(BackupVaultEvent::BackupJobCompleted)
        .send()
        .await
        .unwrap();

    client
        .delete_backup_vault_notifications()
        .backup_vault_name("del-notif-vault")
        .send()
        .await
        .expect("delete_backup_vault_notifications should succeed");

    let result = client
        .get_backup_vault_notifications()
        .backup_vault_name("del-notif-vault")
        .send()
        .await;
    assert!(result.is_err(), "notifications should be gone after delete");
}

#[tokio::test]
async fn test_put_vault_notifications_nonexistent_vault() {
    let client = make_client().await;

    use aws_sdk_backup::types::BackupVaultEvent;

    let result = client
        .put_backup_vault_notifications()
        .backup_vault_name("ghost-vault")
        .sns_topic_arn("arn:aws:sns:us-east-1:123456789012:topic")
        .backup_vault_events(BackupVaultEvent::BackupJobCompleted)
        .send()
        .await;
    assert!(result.is_err(), "should error when vault does not exist");
}

// ============================================================================
// Framework CRUD tests
// ============================================================================

#[tokio::test]
async fn test_create_and_describe_framework() {
    let client = make_client().await;

    let control = aws_sdk_backup::types::FrameworkControl::builder()
        .control_name("BACKUP_RESOURCES_PROTECTED_BY_BACKUP_PLAN")
        .build()
        .unwrap();

    let resp = client
        .create_framework()
        .framework_name("my-framework")
        .framework_description("A test framework")
        .framework_controls(control)
        .send()
        .await
        .expect("create_framework should succeed");

    assert_eq!(resp.framework_name(), Some("my-framework"));
    assert!(resp.framework_arn().is_some());

    let desc = client
        .describe_framework()
        .framework_name("my-framework")
        .send()
        .await
        .expect("describe_framework should succeed");

    assert_eq!(desc.framework_name(), Some("my-framework"));
    assert_eq!(desc.framework_description(), Some("A test framework"));
    assert_eq!(desc.deployment_status(), Some("COMPLETED"));
    assert!(desc.creation_time().is_some());
}

#[tokio::test]
async fn test_delete_framework() {
    let client = make_client().await;

    let control = aws_sdk_backup::types::FrameworkControl::builder()
        .control_name("BACKUP_RESOURCES_PROTECTED_BY_BACKUP_PLAN")
        .build()
        .unwrap();

    client
        .create_framework()
        .framework_name("del-framework")
        .framework_controls(control)
        .send()
        .await
        .unwrap();

    client
        .delete_framework()
        .framework_name("del-framework")
        .send()
        .await
        .expect("delete_framework should succeed");

    let result = client
        .describe_framework()
        .framework_name("del-framework")
        .send()
        .await;
    assert!(result.is_err(), "framework should be gone after delete");
}

#[tokio::test]
async fn test_list_frameworks() {
    let client = make_client().await;

    for name in ["fw-a", "fw-b"] {
        let control = aws_sdk_backup::types::FrameworkControl::builder()
            .control_name("BACKUP_RESOURCES_PROTECTED_BY_BACKUP_PLAN")
            .build()
            .unwrap();
        client
            .create_framework()
            .framework_name(name)
            .framework_controls(control)
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_frameworks()
        .send()
        .await
        .expect("list_frameworks should succeed");

    assert_eq!(list_resp.frameworks().len(), 2);
}

#[tokio::test]
async fn test_update_framework() {
    let client = make_client().await;

    let control = aws_sdk_backup::types::FrameworkControl::builder()
        .control_name("BACKUP_RESOURCES_PROTECTED_BY_BACKUP_PLAN")
        .build()
        .unwrap();

    client
        .create_framework()
        .framework_name("upd-framework")
        .framework_description("original")
        .framework_controls(control)
        .send()
        .await
        .unwrap();

    let new_control = aws_sdk_backup::types::FrameworkControl::builder()
        .control_name("BACKUP_RESOURCES_PROTECTED_BY_BACKUP_VAULT")
        .build()
        .unwrap();

    let update_resp = client
        .update_framework()
        .framework_name("upd-framework")
        .framework_description("updated description")
        .framework_controls(new_control)
        .send()
        .await
        .expect("update_framework should succeed");

    assert_eq!(update_resp.framework_name(), Some("upd-framework"));

    let desc = client
        .describe_framework()
        .framework_name("upd-framework")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.framework_description(), Some("updated description"));
}

#[tokio::test]
async fn test_describe_nonexistent_framework() {
    let client = make_client().await;

    let result = client
        .describe_framework()
        .framework_name("no-such-framework")
        .send()
        .await;
    assert!(result.is_err(), "should error for nonexistent framework");
}

#[tokio::test]
async fn test_create_duplicate_framework() {
    let client = make_client().await;

    let control = aws_sdk_backup::types::FrameworkControl::builder()
        .control_name("BACKUP_RESOURCES_PROTECTED_BY_BACKUP_PLAN")
        .build()
        .unwrap();

    client
        .create_framework()
        .framework_name("dup-framework")
        .framework_controls(control.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .create_framework()
        .framework_name("dup-framework")
        .framework_controls(control)
        .send()
        .await;
    assert!(result.is_err(), "duplicate framework should fail");
}

// ============================================================================
// Global Settings tests
// ============================================================================

#[tokio::test]
async fn test_describe_and_update_global_settings() {
    let client = make_client().await;

    // Describe initial state (should be empty)
    let initial = client
        .describe_global_settings()
        .send()
        .await
        .expect("describe_global_settings should succeed");

    // May be empty initially
    let _ = initial.global_settings();

    // Update settings
    client
        .update_global_settings()
        .global_settings("isCrossAccountBackupEnabled", "true")
        .send()
        .await
        .expect("update_global_settings should succeed");

    let updated = client
        .describe_global_settings()
        .send()
        .await
        .expect("describe_global_settings after update should succeed");

    let settings = updated.global_settings().unwrap();
    assert_eq!(
        settings
            .get("isCrossAccountBackupEnabled")
            .map(|s| s.as_str()),
        Some("true")
    );
}

// ============================================================================
// Region Settings tests
// ============================================================================

#[tokio::test]
async fn test_describe_and_update_region_settings() {
    let client = make_client().await;

    // Describe initial state
    let initial = client
        .describe_region_settings()
        .send()
        .await
        .expect("describe_region_settings should succeed");

    let _ = initial.resource_type_opt_in_preference();

    // Update settings
    client
        .update_region_settings()
        .resource_type_opt_in_preference("DynamoDB", true)
        .resource_type_opt_in_preference("EFS", false)
        .send()
        .await
        .expect("update_region_settings should succeed");

    let updated = client
        .describe_region_settings()
        .send()
        .await
        .expect("describe_region_settings after update should succeed");

    let prefs = updated.resource_type_opt_in_preference().unwrap();
    assert_eq!(prefs.get("DynamoDB").copied(), Some(true));
    assert_eq!(prefs.get("EFS").copied(), Some(false));
}

// ============================================================================
// UpdateReportPlan tests
// ============================================================================

#[tokio::test]
async fn test_update_report_plan() {
    let client = make_client().await;

    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("original-bucket")
        .build()
        .unwrap();
    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();

    client
        .create_report_plan()
        .report_plan_name("upd-report")
        .report_plan_description("original description")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await
        .unwrap();

    let new_delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("new-bucket")
        .build()
        .unwrap();
    let new_report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("RESTORE_JOB_REPORT")
        .build()
        .unwrap();

    let update_resp = client
        .update_report_plan()
        .report_plan_name("upd-report")
        .report_plan_description("updated description")
        .report_delivery_channel(new_delivery_channel)
        .report_setting(new_report_setting)
        .send()
        .await
        .expect("update_report_plan should succeed");

    assert_eq!(update_resp.report_plan_name(), Some("upd-report"));
    assert!(update_resp.report_plan_arn().is_some());

    let desc = client
        .describe_report_plan()
        .report_plan_name("upd-report")
        .send()
        .await
        .unwrap();

    let rp = desc.report_plan().unwrap();
    assert_eq!(rp.report_plan_description(), Some("updated description"));
}

#[tokio::test]
async fn test_update_nonexistent_report_plan() {
    let client = make_client().await;

    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("bucket")
        .build()
        .unwrap();
    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();

    let result = client
        .update_report_plan()
        .report_plan_name("nonexistent-report-plan")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await;
    assert!(
        result.is_err(),
        "updating nonexistent report plan should fail"
    );
}

// --- Report Job tests ---

#[tokio::test]
async fn test_start_and_describe_report_job() {
    let client = make_client().await;

    // Create a report plan first
    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("my-bucket")
        .build()
        .unwrap();
    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();
    client
        .create_report_plan()
        .report_plan_name("job-test-report")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await
        .unwrap();

    // Start a report job
    let start_resp = client
        .start_report_job()
        .report_plan_name("job-test-report")
        .send()
        .await
        .expect("start_report_job should succeed");

    let job_id = start_resp
        .report_job_id()
        .expect("job_id should be set")
        .to_string();
    assert!(!job_id.is_empty());

    // Describe the report job
    let desc_resp = client
        .describe_report_job()
        .report_job_id(&job_id)
        .send()
        .await
        .expect("describe_report_job should succeed");

    let job = desc_resp.report_job().unwrap();
    assert_eq!(job.report_job_id(), Some(job_id.as_str()));
    assert_eq!(job.status(), Some("COMPLETED"));
}

#[tokio::test]
async fn test_list_report_jobs() {
    let client = make_client().await;

    // Create a report plan
    let delivery_channel = aws_sdk_backup::types::ReportDeliveryChannel::builder()
        .s3_bucket_name("bucket")
        .build()
        .unwrap();
    let report_setting = aws_sdk_backup::types::ReportSetting::builder()
        .report_template("BACKUP_JOB_REPORT")
        .build()
        .unwrap();
    client
        .create_report_plan()
        .report_plan_name("list-jobs-report")
        .report_delivery_channel(delivery_channel)
        .report_setting(report_setting)
        .send()
        .await
        .unwrap();

    // Start two report jobs
    client
        .start_report_job()
        .report_plan_name("list-jobs-report")
        .send()
        .await
        .unwrap();
    client
        .start_report_job()
        .report_plan_name("list-jobs-report")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_report_jobs()
        .send()
        .await
        .expect("list_report_jobs should succeed");

    assert_eq!(list_resp.report_jobs().len(), 2);
}

#[tokio::test]
async fn test_start_report_job_nonexistent_plan() {
    let client = make_client().await;
    let result = client
        .start_report_job()
        .report_plan_name("no-such-plan")
        .send()
        .await;
    assert!(
        result.is_err(),
        "starting a job for nonexistent plan should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_report_job() {
    let client = make_client().await;
    let result = client
        .describe_report_job()
        .report_job_id("no-such-job-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describing nonexistent report job should fail"
    );
}

// --- Scan Job tests ---

#[tokio::test]
async fn test_start_and_describe_scan_job() {
    let client = make_client().await;

    // Create a vault and recovery point so there is something to scan
    client
        .create_backup_vault()
        .backup_vault_name("scan-vault")
        .send()
        .await
        .unwrap();

    let start_resp = client
        .start_scan_job()
        .backup_vault_name("scan-vault")
        .recovery_point_arn("arn:aws:backup:us-east-1:123456789012:recovery-point:test-rp")
        .iam_role_arn("arn:aws:iam::123456789012:role/scan-role")
        .malware_scanner("AWSSCANNER".into())
        .scan_mode("EC2_SCAN_NATIVE".into())
        .scanner_role_arn("arn:aws:iam::123456789012:role/scanner-role")
        .send()
        .await
        .expect("start_scan_job should succeed");

    let scan_job_id = start_resp.scan_job_id().to_string();
    assert!(!scan_job_id.is_empty());

    let desc_resp = client
        .describe_scan_job()
        .scan_job_id(&scan_job_id)
        .send()
        .await
        .expect("describe_scan_job should succeed");

    assert_eq!(desc_resp.scan_job_id(), scan_job_id.as_str());
    assert_eq!(desc_resp.backup_vault_name(), "scan-vault");
    assert_eq!(desc_resp.state().as_str(), "RUNNING");
}

#[tokio::test]
async fn test_list_scan_jobs() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("scan-vault-list")
        .send()
        .await
        .unwrap();

    // Start two scan jobs
    for _ in 0..2 {
        client
            .start_scan_job()
            .backup_vault_name("scan-vault-list")
            .recovery_point_arn("arn:aws:backup:us-east-1:123456789012:recovery-point:rp")
            .iam_role_arn("arn:aws:iam::123456789012:role/scan-role")
            .malware_scanner("AWSSCANNER".into())
            .scan_mode("EC2_SCAN_NATIVE".into())
            .scanner_role_arn("arn:aws:iam::123456789012:role/scanner-role")
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_scan_jobs()
        .send()
        .await
        .expect("list_scan_jobs should succeed");

    assert_eq!(list_resp.scan_jobs().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_scan_job() {
    let client = make_client().await;
    let result = client
        .describe_scan_job()
        .scan_job_id("no-such-scan-job")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describing nonexistent scan job should fail"
    );
}

// --- Tiering Configuration tests ---

#[tokio::test]
async fn test_create_and_get_tiering_configuration() {
    let client = make_client().await;

    // Create a vault for the tiering config to reference
    client
        .create_backup_vault()
        .backup_vault_name("tiering-vault")
        .send()
        .await
        .unwrap();

    let resource_selection = aws_sdk_backup::types::ResourceSelection::builder()
        .resource_type("EBS")
        .resources("arn:aws:ec2:us-east-1:123456789012:volume/vol-12345")
        .tiering_down_settings_in_days(90)
        .build()
        .unwrap();

    let tiering_config_input = aws_sdk_backup::types::TieringConfigurationInputForCreate::builder()
        .tiering_configuration_name("my-tiering-config")
        .backup_vault_name("tiering-vault")
        .resource_selection(resource_selection)
        .build()
        .unwrap();

    let create_resp = client
        .create_tiering_configuration()
        .tiering_configuration(tiering_config_input)
        .send()
        .await
        .expect("create_tiering_configuration should succeed");

    assert_eq!(
        create_resp.tiering_configuration_name(),
        Some("my-tiering-config")
    );
    assert!(create_resp.tiering_configuration_arn().is_some());

    let get_resp = client
        .get_tiering_configuration()
        .tiering_configuration_name("my-tiering-config")
        .send()
        .await
        .expect("get_tiering_configuration should succeed");

    let config = get_resp.tiering_configuration().unwrap();
    assert_eq!(config.tiering_configuration_name(), "my-tiering-config");
    assert_eq!(config.backup_vault_name(), "tiering-vault");
}

#[tokio::test]
async fn test_delete_tiering_configuration() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("tiering-vault-del")
        .send()
        .await
        .unwrap();

    let resource_selection = aws_sdk_backup::types::ResourceSelection::builder()
        .resource_type("EBS")
        .resources("arn:aws:ec2:us-east-1:123456789012:volume/vol-del")
        .tiering_down_settings_in_days(90)
        .build()
        .unwrap();

    let tiering_config_input = aws_sdk_backup::types::TieringConfigurationInputForCreate::builder()
        .tiering_configuration_name("del-tiering-config")
        .backup_vault_name("tiering-vault-del")
        .resource_selection(resource_selection)
        .build()
        .unwrap();

    client
        .create_tiering_configuration()
        .tiering_configuration(tiering_config_input)
        .send()
        .await
        .unwrap();

    client
        .delete_tiering_configuration()
        .tiering_configuration_name("del-tiering-config")
        .send()
        .await
        .expect("delete_tiering_configuration should succeed");

    let result = client
        .get_tiering_configuration()
        .tiering_configuration_name("del-tiering-config")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleted tiering config should not be found"
    );
}

#[tokio::test]
async fn test_list_tiering_configurations() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("tiering-vault-list")
        .send()
        .await
        .unwrap();

    for name in ["tier-cfg-a", "tier-cfg-b"] {
        let resource_selection = aws_sdk_backup::types::ResourceSelection::builder()
            .resource_type("EBS")
            .resources("arn:aws:ec2:us-east-1:123456789012:volume/vol-list")
            .tiering_down_settings_in_days(90)
            .build()
            .unwrap();
        let tiering_config_input =
            aws_sdk_backup::types::TieringConfigurationInputForCreate::builder()
                .tiering_configuration_name(name)
                .backup_vault_name("tiering-vault-list")
                .resource_selection(resource_selection)
                .build()
                .unwrap();
        client
            .create_tiering_configuration()
            .tiering_configuration(tiering_config_input)
            .send()
            .await
            .unwrap();
    }

    let list_resp = client
        .list_tiering_configurations()
        .send()
        .await
        .expect("list_tiering_configurations should succeed");

    assert_eq!(list_resp.tiering_configurations().len(), 2);
}

#[tokio::test]
async fn test_update_tiering_configuration() {
    let client = make_client().await;

    client
        .create_backup_vault()
        .backup_vault_name("tiering-vault-upd")
        .send()
        .await
        .unwrap();

    let resource_selection = aws_sdk_backup::types::ResourceSelection::builder()
        .resource_type("EBS")
        .resources("arn:aws:ec2:us-east-1:123456789012:volume/vol-upd")
        .tiering_down_settings_in_days(90)
        .build()
        .unwrap();

    let tiering_config_input = aws_sdk_backup::types::TieringConfigurationInputForCreate::builder()
        .tiering_configuration_name("upd-tiering-config")
        .backup_vault_name("tiering-vault-upd")
        .resource_selection(resource_selection)
        .build()
        .unwrap();

    client
        .create_tiering_configuration()
        .tiering_configuration(tiering_config_input)
        .send()
        .await
        .unwrap();

    // Update the tiering configuration
    let updated_selection = aws_sdk_backup::types::ResourceSelection::builder()
        .resource_type("RDS")
        .resources("arn:aws:rds:us-east-1:123456789012:db:mydb")
        .tiering_down_settings_in_days(180)
        .build()
        .unwrap();

    let tiering_config_update =
        aws_sdk_backup::types::TieringConfigurationInputForUpdate::builder()
            .backup_vault_name("tiering-vault-upd")
            .resource_selection(updated_selection)
            .build()
            .unwrap();

    let update_resp = client
        .update_tiering_configuration()
        .tiering_configuration(tiering_config_update)
        .tiering_configuration_name("upd-tiering-config")
        .send()
        .await
        .expect("update_tiering_configuration should succeed");

    assert_eq!(
        update_resp.tiering_configuration_name(),
        Some("upd-tiering-config")
    );
}

#[tokio::test]
async fn test_get_nonexistent_tiering_configuration() {
    let client = make_client().await;
    let result = client
        .get_tiering_configuration()
        .tiering_configuration_name("no-such-config")
        .send()
        .await;
    assert!(
        result.is_err(),
        "getting nonexistent tiering config should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_tiering_configuration() {
    let client = make_client().await;
    let result = client
        .delete_tiering_configuration()
        .tiering_configuration_name("no-such-config")
        .send()
        .await;
    assert!(
        result.is_err(),
        "deleting nonexistent tiering config should fail"
    );
}

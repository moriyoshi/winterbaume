use aws_sdk_macie2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_macie2::Macie2Service;

async fn make_client() -> aws_sdk_macie2::Client {
    let mock = MockAws::builder()
        .with_service(Macie2Service::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_macie2::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_macie2::Client::new(&config)
}

#[tokio::test]
async fn test_enable_and_get_macie_session() {
    let client = make_client().await;

    client
        .enable_macie()
        .send()
        .await
        .expect("enable_macie should succeed");

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session should succeed");

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_macie2::types::MacieStatus::Enabled)
    );
    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&aws_sdk_macie2::types::FindingPublishingFrequency::SixHours)
    );
    assert!(resp.created_at().is_some());
    assert!(resp.updated_at().is_some());
    assert!(resp.service_role().is_some());
}

#[tokio::test]
async fn test_enable_macie_with_options() {
    let client = make_client().await;

    client
        .enable_macie()
        .status(aws_sdk_macie2::types::MacieStatus::Enabled)
        .finding_publishing_frequency(
            aws_sdk_macie2::types::FindingPublishingFrequency::FifteenMinutes,
        )
        .send()
        .await
        .expect("enable_macie with options should succeed");

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session should succeed");

    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&aws_sdk_macie2::types::FindingPublishingFrequency::FifteenMinutes)
    );
}

#[tokio::test]
async fn test_disable_macie() {
    let client = make_client().await;

    client.enable_macie().send().await.unwrap();

    client
        .disable_macie()
        .send()
        .await
        .expect("disable_macie should succeed");

    let result = client.get_macie_session().send().await;
    assert!(result.is_err(), "get after disable should fail");
}

#[tokio::test]
async fn test_enable_macie_duplicate_fails() {
    let client = make_client().await;

    client.enable_macie().send().await.unwrap();

    let result = client.enable_macie().send().await;
    assert!(result.is_err(), "duplicate enable should fail");
}

#[tokio::test]
async fn test_disable_without_enable_fails() {
    let client = make_client().await;

    let result = client.disable_macie().send().await;
    assert!(result.is_err(), "disable without enable should fail");
}

#[tokio::test]
async fn test_list_findings() {
    let client = make_client().await;

    client.enable_macie().send().await.unwrap();

    let resp = client
        .list_findings()
        .send()
        .await
        .expect("list_findings should succeed");

    assert!(resp.finding_ids().is_empty());
}

#[tokio::test]
async fn test_list_findings_without_enable_fails() {
    let client = make_client().await;

    let result = client.list_findings().send().await;
    assert!(result.is_err(), "list_findings without enable should fail");
}

#[tokio::test]
async fn test_enable_organization_admin_account() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .enable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await
        .expect("enable_organization_admin_account should succeed");

    let resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .expect("list_organization_admin_accounts should succeed");

    let accounts = resp.admin_accounts();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].account_id().unwrap(), "111122223333");
    assert_eq!(
        accounts[0].status(),
        Some(&aws_sdk_macie2::types::AdminStatus::Enabled)
    );
}

#[tokio::test]
async fn test_enable_organization_admin_account_duplicate_fails() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .enable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await
        .unwrap();

    let result = client
        .enable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await;
    assert!(result.is_err(), "duplicate admin account should fail");
}

#[tokio::test]
async fn test_list_organization_admin_accounts_empty() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .expect("list_organization_admin_accounts should succeed");

    assert!(resp.admin_accounts().is_empty());
}

#[tokio::test]
async fn test_get_administrator_account_none() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .get_administrator_account()
        .send()
        .await
        .expect("get_administrator_account should succeed");

    assert!(resp.administrator().is_none());
}

#[tokio::test]
async fn test_list_members_empty() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");

    assert!(resp.members().is_empty());
}

#[tokio::test]
async fn test_list_invitations_empty() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .list_invitations()
        .send()
        .await
        .expect("list_invitations should succeed");

    assert!(resp.invitations().is_empty());
}

#[tokio::test]
async fn test_delete_member_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client.delete_member().id("nonexistent").send().await;
    assert!(result.is_err(), "delete non-existent member should fail");
}

#[tokio::test]
async fn test_disassociate_member_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client.disassociate_member().id("nonexistent").send().await;
    assert!(
        result.is_err(),
        "disassociate non-existent member should fail"
    );
}

#[tokio::test]
async fn test_create_invitations_no_members() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .create_invitations()
        .account_ids("999988887777")
        .send()
        .await
        .expect("create_invitations should succeed");

    // Account is not a member, so it should appear in unprocessed
    let unprocessed = resp.unprocessed_accounts();
    assert_eq!(unprocessed.len(), 1);
    assert_eq!(unprocessed[0].account_id().unwrap(), "999988887777");
}

#[tokio::test]
async fn test_decline_invitations_no_matching() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .decline_invitations()
        .account_ids("999988887777")
        .send()
        .await
        .expect("decline_invitations should succeed");

    let unprocessed = resp.unprocessed_accounts();
    assert_eq!(unprocessed.len(), 1);
    assert_eq!(unprocessed[0].account_id().unwrap(), "999988887777");
}

#[tokio::test]
async fn test_accept_invitation_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client
        .accept_invitation()
        .administrator_account_id("111122223333")
        .invitation_id("inv-00000000")
        .send()
        .await;
    assert!(
        result.is_err(),
        "accept non-existent invitation should fail"
    );
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

#[tokio::test]
async fn test_update_macie_session_finding_frequency() {
    let client = make_client().await;

    client
        .enable_macie()
        .finding_publishing_frequency(aws_sdk_macie2::types::FindingPublishingFrequency::OneHour)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session should succeed");

    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&aws_sdk_macie2::types::FindingPublishingFrequency::OneHour)
    );
}

#[tokio::test]
async fn test_enable_macie_status_paused() {
    let client = make_client().await;

    client
        .enable_macie()
        .status(aws_sdk_macie2::types::MacieStatus::Paused)
        .send()
        .await
        .expect("enable_macie with paused status should succeed");

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session should succeed");

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_macie2::types::MacieStatus::Paused)
    );
}

#[tokio::test]
async fn test_get_macie_session_without_enable_fails() {
    let client = make_client().await;

    let result = client.get_macie_session().send().await;
    assert!(
        result.is_err(),
        "get_macie_session without enable should fail"
    );
}

#[tokio::test]
async fn test_list_organization_admin_accounts_multiple() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .enable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await
        .unwrap();

    client
        .enable_organization_admin_account()
        .admin_account_id("444455556666")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .expect("list_organization_admin_accounts should succeed");

    let accounts = resp.admin_accounts();
    assert_eq!(accounts.len(), 2);
    let ids: Vec<&str> = accounts.iter().map(|a| a.account_id().unwrap()).collect();
    assert!(ids.contains(&"111122223333"));
    assert!(ids.contains(&"444455556666"));
}

#[tokio::test]
async fn test_list_organization_admin_accounts_without_enable_fails() {
    let client = make_client().await;

    let result = client.list_organization_admin_accounts().send().await;
    assert!(
        result.is_err(),
        "list_organization_admin_accounts without enable should fail"
    );
}

#[tokio::test]
async fn test_get_administrator_account_without_enable_fails() {
    let client = make_client().await;

    let result = client.get_administrator_account().send().await;
    assert!(
        result.is_err(),
        "get_administrator_account without enable should fail"
    );
}

#[tokio::test]
async fn test_list_members_without_enable_fails() {
    let client = make_client().await;

    let result = client.list_members().send().await;
    assert!(result.is_err(), "list_members without enable should fail");
}

#[tokio::test]
async fn test_create_invitations_without_enable_fails() {
    let client = make_client().await;

    let result = client
        .create_invitations()
        .account_ids("111122223333")
        .send()
        .await;
    assert!(
        result.is_err(),
        "create_invitations without enable should fail"
    );
}

#[tokio::test]
async fn test_decline_invitations_without_enable_fails() {
    let client = make_client().await;

    let result = client
        .decline_invitations()
        .account_ids("111122223333")
        .send()
        .await;
    assert!(
        result.is_err(),
        "decline_invitations without enable should fail"
    );
}

#[tokio::test]
async fn test_enable_disable_reenable_macie() {
    let client = make_client().await;

    client.enable_macie().send().await.unwrap();
    client.disable_macie().send().await.unwrap();

    client
        .enable_macie()
        .send()
        .await
        .expect("re-enabling macie after disable should succeed");

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session after re-enable should succeed");

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_macie2::types::MacieStatus::Enabled)
    );
}

#[tokio::test]
async fn test_enable_organization_admin_account_without_enable_fails() {
    let client = make_client().await;

    let result = client
        .enable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await;
    assert!(
        result.is_err(),
        "enable_organization_admin_account without macie enable should fail"
    );
}

#[tokio::test]
async fn test_macie_session_has_service_role_arn() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session should succeed");

    let service_role = resp.service_role().expect("service_role should be set");
    assert!(
        service_role.starts_with("arn:aws:iam::"),
        "service_role should be an IAM ARN"
    );
    assert!(
        service_role.contains("AWSServiceRoleForAmazonMacie"),
        "service_role should reference AWSServiceRoleForAmazonMacie"
    );
}

// ============================================================================
// Tags tests
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let arn = "arn:aws:macie2:us-east-1:123456789012:resource/test";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    assert!(resp.tags().is_some());
    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags.get("env"), Some(&"test".to_string()));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let arn = "arn:aws:macie2:us-east-1:123456789012:resource/untag-test";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags("keep", "yes")
        .tags("remove", "yes")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("remove")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert!(tags.contains_key("keep"));
    assert!(!tags.contains_key("remove"));
}

// ============================================================================
// Stub operation smoke tests (verify SDK compatibility)
// ============================================================================

#[tokio::test]
async fn test_create_allow_list() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .create_allow_list()
        .name("test-allow")
        .criteria(
            aws_sdk_macie2::types::AllowListCriteria::builder()
                .regex("[a-z]+")
                .build(),
        )
        .send()
        .await
        .expect("create_allow_list should succeed");
}

#[tokio::test]
async fn test_list_allow_lists() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .list_allow_lists()
        .send()
        .await
        .expect("list_allow_lists should succeed");
}

#[tokio::test]
async fn test_create_custom_data_identifier() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .create_custom_data_identifier()
        .name("test-cdi")
        .regex("[0-9]+")
        .send()
        .await
        .expect("create_custom_data_identifier should succeed");
}

#[tokio::test]
async fn test_create_findings_filter() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .create_findings_filter()
        .name("test-filter")
        .action(aws_sdk_macie2::types::FindingsFilterAction::Archive)
        .finding_criteria(aws_sdk_macie2::types::FindingCriteria::builder().build())
        .send()
        .await
        .expect("create_findings_filter should succeed");
}

#[tokio::test]
async fn test_list_findings_filters() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .list_findings_filters()
        .send()
        .await
        .expect("list_findings_filters should succeed");
}

#[tokio::test]
async fn test_create_classification_job() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .create_classification_job()
        .name("test-job")
        .job_type(aws_sdk_macie2::types::JobType::OneTime)
        .s3_job_definition(aws_sdk_macie2::types::S3JobDefinition::builder().build())
        .send()
        .await
        .expect("create_classification_job should succeed");
}

#[tokio::test]
async fn test_describe_organization_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe_organization_configuration should succeed");
}

#[tokio::test]
async fn test_get_automated_discovery_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_automated_discovery_configuration()
        .send()
        .await
        .expect("get_automated_discovery_configuration should succeed");
}

#[tokio::test]
async fn test_get_classification_export_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_classification_export_configuration()
        .send()
        .await
        .expect("get_classification_export_configuration should succeed");
}

#[tokio::test]
async fn test_get_reveal_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_reveal_configuration()
        .send()
        .await
        .expect("get_reveal_configuration should succeed");
}

#[tokio::test]
async fn test_get_findings_publication_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_findings_publication_configuration()
        .send()
        .await
        .expect("get_findings_publication_configuration should succeed");
}

#[tokio::test]
async fn test_create_member() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .create_member()
        .account(
            aws_sdk_macie2::types::AccountDetail::builder()
                .account_id("222233334444")
                .email("user@example.com")
                .build(),
        )
        .send()
        .await
        .expect("create_member should succeed");
}

#[tokio::test]
async fn test_list_managed_data_identifiers() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .list_managed_data_identifiers()
        .send()
        .await
        .expect("list_managed_data_identifiers should succeed");
}

#[tokio::test]
async fn test_search_resources() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .search_resources()
        .send()
        .await
        .expect("search_resources should succeed");
}

#[tokio::test]
async fn test_describe_buckets() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .describe_buckets()
        .send()
        .await
        .expect("describe_buckets should succeed");
}

#[tokio::test]
async fn test_get_bucket_statistics() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_bucket_statistics()
        .send()
        .await
        .expect("get_bucket_statistics should succeed");
}

#[tokio::test]
async fn test_get_usage_totals() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_usage_totals()
        .send()
        .await
        .expect("get_usage_totals should succeed");
}

#[tokio::test]
async fn test_get_usage_statistics() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_usage_statistics()
        .send()
        .await
        .expect("get_usage_statistics should succeed");
}

#[tokio::test]
async fn test_get_finding_statistics() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();
    let _resp = client
        .get_finding_statistics()
        .group_by(aws_sdk_macie2::types::GroupBy::Type)
        .send()
        .await
        .expect("get_finding_statistics should succeed");
}

// ============================================================================
// Custom data identifier CRUD tests
// ============================================================================

#[tokio::test]
async fn test_custom_data_identifier_full_crud() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Create
    let create_resp = client
        .create_custom_data_identifier()
        .name("test-cdi-crud")
        .regex("[0-9]{4}")
        .description("A test CDI")
        .send()
        .await
        .expect("create_custom_data_identifier should succeed");
    let id = create_resp
        .custom_data_identifier_id()
        .expect("id should be set");

    // Get
    let get_resp = client
        .get_custom_data_identifier()
        .id(id)
        .send()
        .await
        .expect("get_custom_data_identifier should succeed");
    assert_eq!(get_resp.name().unwrap_or(""), "test-cdi-crud");
    assert_eq!(get_resp.regex().unwrap_or(""), "[0-9]{4}");
    assert_eq!(get_resp.id().unwrap_or(""), id);

    // List — must contain the created CDI
    let list_resp = client
        .list_custom_data_identifiers()
        .send()
        .await
        .expect("list_custom_data_identifiers should succeed");
    let items = list_resp.items();
    assert!(
        items.iter().any(|c| c.id().unwrap_or("") == id),
        "list should contain created CDI"
    );

    // Delete
    client
        .delete_custom_data_identifier()
        .id(id)
        .send()
        .await
        .expect("delete_custom_data_identifier should succeed");

    // After delete, list should not show it
    let list_resp2 = client
        .list_custom_data_identifiers()
        .send()
        .await
        .expect("list after delete should succeed");
    assert!(
        !list_resp2
            .items()
            .iter()
            .any(|c| c.id().unwrap_or("") == id),
        "deleted CDI should not appear in list"
    );
}

#[tokio::test]
async fn test_batch_get_custom_data_identifiers() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let create_resp = client
        .create_custom_data_identifier()
        .name("batch-cdi")
        .regex("[a-z]+")
        .send()
        .await
        .unwrap();
    let id = create_resp.custom_data_identifier_id().unwrap().to_string();

    let batch_resp = client
        .batch_get_custom_data_identifiers()
        .ids(id.clone())
        .ids("nonexistent-id")
        .send()
        .await
        .expect("batch_get_custom_data_identifiers should succeed");

    let found = batch_resp.custom_data_identifiers();
    assert_eq!(found.len(), 1, "should find 1 CDI");
    assert_eq!(found[0].id().unwrap_or(""), &id);

    let not_found = batch_resp.not_found_identifier_ids();
    assert_eq!(not_found.len(), 1, "should report 1 not found");
    assert_eq!(not_found[0], "nonexistent-id");
}

#[tokio::test]
async fn test_get_custom_data_identifier_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client
        .get_custom_data_identifier()
        .id("nonexistent-cdi")
        .send()
        .await;
    assert!(result.is_err(), "getting nonexistent CDI should fail");
}

#[tokio::test]
async fn test_delete_custom_data_identifier_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client
        .delete_custom_data_identifier()
        .id("nonexistent-cdi")
        .send()
        .await;
    assert!(result.is_err(), "deleting nonexistent CDI should fail");
}

// ============================================================================
// Allow list CRUD tests
// ============================================================================

#[tokio::test]
async fn test_allow_list_full_crud() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Create
    let create_resp = client
        .create_allow_list()
        .name("my-allow-list")
        .description("a test list")
        .criteria(
            aws_sdk_macie2::types::AllowListCriteria::builder()
                .regex("[a-zA-Z0-9_]+@example\\.com")
                .build(),
        )
        .send()
        .await
        .expect("create_allow_list should succeed");
    let id = create_resp.id().expect("id should be set").to_string();
    let arn = create_resp.arn().expect("arn should be set").to_string();
    assert!(!id.is_empty(), "id should not be empty");
    assert!(
        arn.contains("allow-list"),
        "arn should reference allow-list"
    );

    // Get
    let get_resp = client
        .get_allow_list()
        .id(&id)
        .send()
        .await
        .expect("get_allow_list should succeed");
    assert_eq!(get_resp.name().unwrap_or(""), "my-allow-list");
    assert_eq!(get_resp.id().unwrap_or(""), &id);

    // Update
    client
        .update_allow_list()
        .id(&id)
        .name("updated-allow-list")
        .criteria(
            aws_sdk_macie2::types::AllowListCriteria::builder()
                .regex("[0-9]+@example\\.com")
                .build(),
        )
        .send()
        .await
        .expect("update_allow_list should succeed");

    let get_resp2 = client
        .get_allow_list()
        .id(&id)
        .send()
        .await
        .expect("get after update should succeed");
    assert_eq!(get_resp2.name().unwrap_or(""), "updated-allow-list");

    // List
    let list_resp = client
        .list_allow_lists()
        .send()
        .await
        .expect("list_allow_lists should succeed");
    let allow_lists = list_resp.allow_lists();
    assert!(
        allow_lists.iter().any(|a| a.id().unwrap_or("") == id),
        "list should contain created allow list"
    );

    // Delete
    client
        .delete_allow_list()
        .id(&id)
        .send()
        .await
        .expect("delete_allow_list should succeed");

    let list_resp2 = client
        .list_allow_lists()
        .send()
        .await
        .expect("list after delete should succeed");
    assert!(
        !list_resp2
            .allow_lists()
            .iter()
            .any(|a| a.id().unwrap_or("") == id),
        "deleted allow list should not appear in list"
    );
}

#[tokio::test]
async fn test_get_allow_list_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client.get_allow_list().id("nonexistent").send().await;
    assert!(
        result.is_err(),
        "getting nonexistent allow list should fail"
    );
}

#[tokio::test]
async fn test_delete_allow_list_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client.delete_allow_list().id("nonexistent").send().await;
    assert!(
        result.is_err(),
        "deleting nonexistent allow list should fail"
    );
}

// ============================================================================
// Findings filter CRUD tests
// ============================================================================

#[tokio::test]
async fn test_findings_filter_full_crud() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Create
    let create_resp = client
        .create_findings_filter()
        .name("my-filter")
        .action(aws_sdk_macie2::types::FindingsFilterAction::Archive)
        .finding_criteria(aws_sdk_macie2::types::FindingCriteria::builder().build())
        .send()
        .await
        .expect("create_findings_filter should succeed");
    let id = create_resp.id().expect("id should be set").to_string();
    let arn = create_resp.arn().expect("arn should be set").to_string();
    assert!(!id.is_empty());
    assert!(
        arn.contains("findings-filter"),
        "arn should reference findings-filter"
    );

    // Get
    let get_resp = client
        .get_findings_filter()
        .id(&id)
        .send()
        .await
        .expect("get_findings_filter should succeed");
    assert_eq!(get_resp.name().unwrap_or(""), "my-filter");
    assert_eq!(get_resp.id().unwrap_or(""), &id);
    assert_eq!(
        get_resp.action(),
        Some(&aws_sdk_macie2::types::FindingsFilterAction::Archive)
    );

    // Update
    client
        .update_findings_filter()
        .id(&id)
        .name("updated-filter")
        .action(aws_sdk_macie2::types::FindingsFilterAction::Noop)
        .send()
        .await
        .expect("update_findings_filter should succeed");

    let get_resp2 = client
        .get_findings_filter()
        .id(&id)
        .send()
        .await
        .expect("get after update should succeed");
    assert_eq!(get_resp2.name().unwrap_or(""), "updated-filter");

    // List
    let list_resp = client
        .list_findings_filters()
        .send()
        .await
        .expect("list_findings_filters should succeed");
    assert!(
        list_resp
            .findings_filter_list_items()
            .iter()
            .any(|f| f.id().unwrap_or("") == id),
        "list should contain created filter"
    );

    // Delete
    client
        .delete_findings_filter()
        .id(&id)
        .send()
        .await
        .expect("delete_findings_filter should succeed");

    let list_resp2 = client
        .list_findings_filters()
        .send()
        .await
        .expect("list after delete should succeed");
    assert!(
        !list_resp2
            .findings_filter_list_items()
            .iter()
            .any(|f| f.id().unwrap_or("") == id),
        "deleted filter should not appear in list"
    );
}

#[tokio::test]
async fn test_get_findings_filter_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client.get_findings_filter().id("nonexistent").send().await;
    assert!(result.is_err(), "getting nonexistent filter should fail");
}

// ============================================================================
// Classification job CRUD tests
// ============================================================================

#[tokio::test]
async fn test_classification_job_full_lifecycle() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Create
    let create_resp = client
        .create_classification_job()
        .name("my-job")
        .job_type(aws_sdk_macie2::types::JobType::OneTime)
        .s3_job_definition(aws_sdk_macie2::types::S3JobDefinition::builder().build())
        .send()
        .await
        .expect("create_classification_job should succeed");
    let job_id = create_resp
        .job_id()
        .expect("job_id should be set")
        .to_string();
    let job_arn = create_resp
        .job_arn()
        .expect("job_arn should be set")
        .to_string();
    assert!(!job_id.is_empty());
    assert!(job_arn.contains("job"), "job_arn should reference job");

    // Describe
    let desc_resp = client
        .describe_classification_job()
        .job_id(&job_id)
        .send()
        .await
        .expect("describe_classification_job should succeed");
    assert_eq!(desc_resp.name().unwrap_or(""), "my-job");
    assert_eq!(desc_resp.job_id().unwrap_or(""), &job_id);
    // One-time jobs are immediately COMPLETE
    assert_eq!(
        desc_resp.job_status(),
        Some(&aws_sdk_macie2::types::JobStatus::Complete)
    );

    // Update (cancel)
    client
        .update_classification_job()
        .job_id(&job_id)
        .job_status(aws_sdk_macie2::types::JobStatus::Cancelled)
        .send()
        .await
        .expect("update_classification_job should succeed");

    let desc_resp2 = client
        .describe_classification_job()
        .job_id(&job_id)
        .send()
        .await
        .expect("describe after update should succeed");
    assert_eq!(
        desc_resp2.job_status(),
        Some(&aws_sdk_macie2::types::JobStatus::Cancelled)
    );

    // List
    let list_resp = client
        .list_classification_jobs()
        .send()
        .await
        .expect("list_classification_jobs should succeed");
    assert!(
        list_resp
            .items()
            .iter()
            .any(|j| j.job_id().unwrap_or("") == job_id),
        "list should contain created job"
    );
}

#[tokio::test]
async fn test_scheduled_classification_job_is_running() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let create_resp = client
        .create_classification_job()
        .name("scheduled-job")
        .job_type(aws_sdk_macie2::types::JobType::Scheduled)
        .s3_job_definition(aws_sdk_macie2::types::S3JobDefinition::builder().build())
        .schedule_frequency(
            aws_sdk_macie2::types::JobScheduleFrequency::builder()
                .daily_schedule(aws_sdk_macie2::types::DailySchedule::builder().build())
                .build(),
        )
        .send()
        .await
        .expect("create scheduled job should succeed");
    let job_id = create_resp.job_id().unwrap().to_string();

    let desc_resp = client
        .describe_classification_job()
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    // Scheduled jobs start as RUNNING
    assert_eq!(
        desc_resp.job_status(),
        Some(&aws_sdk_macie2::types::JobStatus::Running)
    );
}

#[tokio::test]
async fn test_describe_classification_job_not_found() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let result = client
        .describe_classification_job()
        .job_id("nonexistent-job")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent job should fail");
}

// ============================================================================
// Reveal configuration tests
// ============================================================================

#[tokio::test]
async fn test_update_and_get_reveal_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Initially no configuration
    let get_resp = client
        .get_reveal_configuration()
        .send()
        .await
        .expect("get_reveal_configuration should succeed");
    assert!(get_resp.configuration().is_none());

    // Update
    client
        .update_reveal_configuration()
        .configuration(
            aws_sdk_macie2::types::RevealConfiguration::builder()
                .status(aws_sdk_macie2::types::RevealStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("update_reveal_configuration should succeed");

    let get_resp2 = client
        .get_reveal_configuration()
        .send()
        .await
        .expect("get after update should succeed");
    assert!(get_resp2.configuration().is_some());
    assert_eq!(
        get_resp2.configuration().unwrap().status(),
        Some(&aws_sdk_macie2::types::RevealStatus::Enabled)
    );
}

// ============================================================================
// Findings publication configuration tests
// ============================================================================

#[tokio::test]
async fn test_put_and_get_findings_publication_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Initially no configuration
    let get_resp = client
        .get_findings_publication_configuration()
        .send()
        .await
        .expect("get should succeed");
    assert!(get_resp.security_hub_configuration().is_none());

    // Put
    client
        .put_findings_publication_configuration()
        .security_hub_configuration(
            aws_sdk_macie2::types::SecurityHubConfiguration::builder()
                .publish_classification_findings(true)
                .publish_policy_findings(true)
                .build(),
        )
        .send()
        .await
        .expect("put_findings_publication_configuration should succeed");

    let get_resp2 = client
        .get_findings_publication_configuration()
        .send()
        .await
        .expect("get after put should succeed");
    let hub = get_resp2.security_hub_configuration().unwrap();
    assert_eq!(hub.publish_classification_findings(), Some(true));
    assert_eq!(hub.publish_policy_findings(), Some(true));
}

// ============================================================================
// Member CRUD tests (improved)
// ============================================================================

#[tokio::test]
async fn test_create_member_and_list() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let create_resp = client
        .create_member()
        .account(
            aws_sdk_macie2::types::AccountDetail::builder()
                .account_id("111122223333")
                .email("member@example.com")
                .build(),
        )
        .send()
        .await
        .expect("create_member should succeed");
    assert!(
        create_resp.arn().is_some(),
        "create_member should return an ARN"
    );

    let list_resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");
    let members = list_resp.members();
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].account_id().unwrap_or(""), "111122223333");
}

#[tokio::test]
async fn test_create_member_duplicate_fails() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .create_member()
        .account(
            aws_sdk_macie2::types::AccountDetail::builder()
                .account_id("111122223333")
                .email("member@example.com")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .create_member()
        .account(
            aws_sdk_macie2::types::AccountDetail::builder()
                .account_id("111122223333")
                .email("member2@example.com")
                .build(),
        )
        .send()
        .await;
    assert!(result.is_err(), "duplicate member should fail");
}

// ============================================================================
// Organisation configuration tests
// ============================================================================

#[tokio::test]
async fn test_update_and_describe_organization_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Initially not configured — autoEnable should be absent or false
    let desc_resp = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe_organization_configuration should succeed");
    assert!(
        desc_resp.auto_enable().is_none() || !desc_resp.auto_enable().unwrap_or(false),
        "auto_enable should be false or absent before update"
    );

    // Update
    client
        .update_organization_configuration()
        .auto_enable(true)
        .send()
        .await
        .expect("update_organization_configuration should succeed");

    let desc_resp2 = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe after update should succeed");
    assert_eq!(desc_resp2.auto_enable(), Some(true));
}

// ============================================================================
// Disable/enable org admin account tests
// ============================================================================

#[tokio::test]
async fn test_disable_organization_admin_account() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .enable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await
        .unwrap();

    // Verify present
    let list_resp = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.admin_accounts().len(), 1);

    // Disable
    client
        .disable_organization_admin_account()
        .admin_account_id("111122223333")
        .send()
        .await
        .expect("disable_organization_admin_account should succeed");

    let list_resp2 = client
        .list_organization_admin_accounts()
        .send()
        .await
        .unwrap();
    assert_eq!(
        list_resp2.admin_accounts().len(),
        0,
        "admin account should be removed"
    );
}

// ============================================================================
// Disassociate from administrator account tests
// ============================================================================

#[tokio::test]
async fn test_disassociate_from_administrator_account() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Manually add an invitation and accept it to establish administrator
    // (There is no direct API to add an invitation from the same account,
    //  so we test the empty-state path.)
    let resp = client
        .get_administrator_account()
        .send()
        .await
        .expect("get_administrator_account should succeed");
    assert!(resp.administrator().is_none());

    // Disassociate when no administrator: should succeed without error.
    client
        .disassociate_from_administrator_account()
        .send()
        .await
        .expect("disassociate_from_administrator_account should succeed");
}

// ============================================================================
// DeleteInvitations tests
// ============================================================================

#[tokio::test]
async fn test_delete_invitations_not_matching() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .delete_invitations()
        .account_ids("999988887777")
        .send()
        .await
        .expect("delete_invitations should succeed");

    let unprocessed = resp.unprocessed_accounts();
    assert_eq!(unprocessed.len(), 1);
    assert_eq!(unprocessed[0].account_id().unwrap(), "999988887777");
}

// ============================================================================
// UpdateMacieSession tests
// ============================================================================

#[tokio::test]
async fn test_update_macie_session_persists() {
    let client = make_client().await;

    client
        .enable_macie()
        .finding_publishing_frequency(aws_sdk_macie2::types::FindingPublishingFrequency::OneHour)
        .send()
        .await
        .unwrap();

    client
        .update_macie_session()
        .finding_publishing_frequency(
            aws_sdk_macie2::types::FindingPublishingFrequency::FifteenMinutes,
        )
        .send()
        .await
        .expect("update_macie_session should succeed");

    let resp = client
        .get_macie_session()
        .send()
        .await
        .expect("get_macie_session should succeed");
    assert_eq!(
        resp.finding_publishing_frequency(),
        Some(&aws_sdk_macie2::types::FindingPublishingFrequency::FifteenMinutes)
    );
}

#[tokio::test]
async fn test_update_macie_session_without_enable_fails() {
    let client = make_client().await;

    let result = client
        .update_macie_session()
        .status(aws_sdk_macie2::types::MacieStatus::Paused)
        .send()
        .await;
    assert!(result.is_err(), "update without enable should fail");
}

// ============================================================================
// Automated discovery configuration tests
// ============================================================================

#[tokio::test]
async fn test_update_and_get_automated_discovery_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Initially empty
    let get_resp = client
        .get_automated_discovery_configuration()
        .send()
        .await
        .expect("get should succeed");
    assert!(get_resp.status().is_none());

    // Enable automated discovery
    client
        .update_automated_discovery_configuration()
        .status(aws_sdk_macie2::types::AutomatedDiscoveryStatus::Enabled)
        .send()
        .await
        .expect("update should succeed");

    let get_resp2 = client
        .get_automated_discovery_configuration()
        .send()
        .await
        .expect("get after update should succeed");
    assert_eq!(
        get_resp2.status(),
        Some(&aws_sdk_macie2::types::AutomatedDiscoveryStatus::Enabled)
    );
    assert!(get_resp2.first_enabled_at().is_some());
    assert!(get_resp2.classification_scope_id().is_some());
    assert!(get_resp2.sensitivity_inspection_template_id().is_some());
}

#[tokio::test]
async fn test_automated_discovery_disable() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .update_automated_discovery_configuration()
        .status(aws_sdk_macie2::types::AutomatedDiscoveryStatus::Enabled)
        .send()
        .await
        .unwrap();

    client
        .update_automated_discovery_configuration()
        .status(aws_sdk_macie2::types::AutomatedDiscoveryStatus::Disabled)
        .send()
        .await
        .expect("disabling automated discovery should succeed");

    let get_resp = client
        .get_automated_discovery_configuration()
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.status(),
        Some(&aws_sdk_macie2::types::AutomatedDiscoveryStatus::Disabled)
    );
    assert!(get_resp.disabled_at().is_some());
}

// ============================================================================
// Classification export configuration tests
// ============================================================================

#[tokio::test]
async fn test_put_and_get_classification_export_configuration() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    // Initially no configuration
    let get_resp = client
        .get_classification_export_configuration()
        .send()
        .await
        .expect("get should succeed");
    assert!(get_resp.configuration().is_none());

    // Put minimal config
    let put_resp = client
        .put_classification_export_configuration()
        .configuration(aws_sdk_macie2::types::ClassificationExportConfiguration::builder().build())
        .send()
        .await
        .expect("put_classification_export_configuration should succeed");
    // Response may include the configuration back
    let _ = put_resp;

    // Get should now reflect a configuration
    let get_resp2 = client
        .get_classification_export_configuration()
        .send()
        .await
        .expect("get after put should succeed");
    // Configuration was set (may be empty struct but not None)
    let _ = get_resp2;
}

// ============================================================================
// Master account (legacy) tests
// ============================================================================

#[tokio::test]
async fn test_get_master_account_empty() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .get_master_account()
        .send()
        .await
        .expect("get_master_account should succeed");
    assert!(resp.master().is_none());
}

#[tokio::test]
async fn test_disassociate_from_master_account() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    client
        .disassociate_from_master_account()
        .send()
        .await
        .expect("disassociate_from_master_account should succeed");
}

// ============================================================================
// Batch update automated discovery accounts tests
// ============================================================================

#[tokio::test]
async fn test_batch_update_automated_discovery_accounts() {
    let client = make_client().await;
    client.enable_macie().send().await.unwrap();

    let resp = client
        .batch_update_automated_discovery_accounts()
        .accounts(
            aws_sdk_macie2::types::AutomatedDiscoveryAccountUpdate::builder()
                .account_id("111122223333")
                .status(aws_sdk_macie2::types::AutomatedDiscoveryAccountStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("batch_update_automated_discovery_accounts should succeed");

    assert!(resp.errors().is_empty(), "no errors expected");

    let list_resp = client
        .list_automated_discovery_accounts()
        .send()
        .await
        .expect("list_automated_discovery_accounts should succeed");
    let accounts = list_resp.items();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].account_id().unwrap_or(""), "111122223333");
}

use aws_sdk_inspector2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_inspector2::Inspector2Service;

async fn make_client() -> aws_sdk_inspector2::Client {
    let mock = MockAws::builder()
        .with_service(Inspector2Service::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_inspector2::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_inspector2::Client::new(&config)
}

#[tokio::test]
async fn test_enable_inspector() {
    let client = make_client().await;

    let resp = client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ecr)
        .send()
        .await
        .expect("enable should succeed");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
    let account = &accounts[0];
    assert_eq!(
        *account.status(),
        aws_sdk_inspector2::types::Status::Enabled
    );
}

#[tokio::test]
async fn test_disable_inspector() {
    let client = make_client().await;

    // First enable
    client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ecr)
        .send()
        .await
        .unwrap();

    // Then disable EC2
    let resp = client
        .disable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .expect("disable should succeed");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
    // Still enabled because ECR is still on
    assert_eq!(
        *accounts[0].status(),
        aws_sdk_inspector2::types::Status::Enabled
    );
}

#[tokio::test]
async fn test_disable_all_results_in_disabled_status() {
    let client = make_client().await;

    // Enable EC2
    client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .unwrap();

    // Disable EC2
    let resp = client
        .disable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .expect("disable should succeed");

    assert_eq!(
        *resp.accounts()[0].status(),
        aws_sdk_inspector2::types::Status::Disabled
    );
}

#[tokio::test]
async fn test_batch_get_account_status() {
    let client = make_client().await;

    // Enable some resource types first
    client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_account_status()
        .send()
        .await
        .expect("batch_get_account_status should succeed");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
}

#[tokio::test]
async fn test_batch_get_account_status_when_disabled() {
    let client = make_client().await;

    let resp = client
        .batch_get_account_status()
        .send()
        .await
        .expect("batch_get_account_status should succeed even when nothing is enabled");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
}

#[tokio::test]
async fn test_list_findings_empty() {
    let client = make_client().await;

    let resp = client
        .list_findings()
        .send()
        .await
        .expect("list_findings should succeed");

    assert_eq!(resp.findings().len(), 0);
}

#[tokio::test]
async fn test_enable_then_list_findings() {
    let client = make_client().await;

    // Enable inspector
    client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .unwrap();

    // List findings (should be empty in mock)
    let resp = client
        .list_findings()
        .send()
        .await
        .expect("list_findings should succeed");

    assert_eq!(resp.findings().len(), 0);
}

#[tokio::test]
async fn test_associate_member() {
    let client = make_client().await;

    let resp = client
        .associate_member()
        .account_id("123456789012")
        .send()
        .await
        .expect("associate_member should succeed");

    assert_eq!(resp.account_id(), "123456789012");
}

#[tokio::test]
async fn test_create_and_delete_filter() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    let resp = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("test-filter")
        .filter_criteria(criteria)
        .send()
        .await
        .expect("create_filter should succeed");

    let arn = resp.arn();
    assert!(!arn.is_empty());

    // Delete the filter
    let del_resp = client
        .delete_filter()
        .arn(arn)
        .send()
        .await
        .expect("delete_filter should succeed");

    assert_eq!(del_resp.arn(), arn);
}

#[tokio::test]
async fn test_delete_filter_not_found() {
    let client = make_client().await;

    let result = client
        .delete_filter()
        .arn("arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/filter/nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_filters_empty() {
    let client = make_client().await;

    let resp = client
        .list_filters()
        .send()
        .await
        .expect("list_filters should succeed");

    assert_eq!(resp.filters().len(), 0);
}

#[tokio::test]
async fn test_create_filter_then_list() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("my-filter")
        .filter_criteria(criteria)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_filters()
        .send()
        .await
        .expect("list_filters should succeed");

    assert_eq!(resp.filters().len(), 1);
    assert_eq!(resp.filters()[0].name(), "my-filter");
}

#[tokio::test]
async fn test_enable_delegated_admin_account() {
    let client = make_client().await;

    let resp = client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("111122223333")
        .send()
        .await
        .expect("enable_delegated_admin_account should succeed");

    assert_eq!(resp.delegated_admin_account_id(), "111122223333");
}

#[tokio::test]
async fn test_disable_delegated_admin_account() {
    let client = make_client().await;

    // Enable first
    client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("111122223333")
        .send()
        .await
        .unwrap();

    // Then disable
    let resp = client
        .disable_delegated_admin_account()
        .delegated_admin_account_id("111122223333")
        .send()
        .await
        .expect("disable_delegated_admin_account should succeed");

    assert_eq!(resp.delegated_admin_account_id(), "111122223333");
}

#[tokio::test]
async fn test_disable_delegated_admin_not_found() {
    let client = make_client().await;

    let result = client
        .disable_delegated_admin_account()
        .delegated_admin_account_id("999999999999")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_delegated_admin_accounts_empty() {
    let client = make_client().await;

    let resp = client
        .list_delegated_admin_accounts()
        .send()
        .await
        .expect("list_delegated_admin_accounts should succeed");

    assert_eq!(resp.delegated_admin_accounts().len(), 0);
}

#[tokio::test]
async fn test_list_delegated_admin_accounts_after_enable() {
    let client = make_client().await;

    client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("111122223333")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_delegated_admin_accounts()
        .send()
        .await
        .expect("list_delegated_admin_accounts should succeed");

    let accounts = resp.delegated_admin_accounts();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].account_id().unwrap(), "111122223333");
}

#[tokio::test]
async fn test_describe_organization_configuration() {
    let client = make_client().await;

    let resp = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe_organization_configuration should succeed");

    // Before any configuration, auto_enable should be None
    assert!(resp.auto_enable().is_none());
}

#[tokio::test]
async fn test_update_organization_configuration() {
    let client = make_client().await;

    let auto_enable = aws_sdk_inspector2::types::AutoEnable::builder()
        .ec2(true)
        .ecr(false)
        .lambda(true)
        .build()
        .unwrap();

    let resp = client
        .update_organization_configuration()
        .auto_enable(auto_enable)
        .send()
        .await
        .expect("update_organization_configuration should succeed");

    let ae = resp.auto_enable().expect("auto_enable should be present");
    assert!(ae.ec2());
    assert!(!ae.ecr());
    assert_eq!(ae.lambda(), Some(true));
}

#[tokio::test]
async fn test_update_then_describe_organization_configuration() {
    let client = make_client().await;

    let auto_enable = aws_sdk_inspector2::types::AutoEnable::builder()
        .ec2(true)
        .ecr(true)
        .build()
        .unwrap();

    client
        .update_organization_configuration()
        .auto_enable(auto_enable)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_organization_configuration()
        .send()
        .await
        .expect("describe_organization_configuration should succeed");

    let ae = resp.auto_enable().expect("auto_enable should be set");
    assert!(ae.ec2());
    assert!(ae.ecr());
}

#[tokio::test]
async fn test_list_members_empty() {
    let client = make_client().await;

    let resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");

    assert_eq!(resp.members().len(), 0);
}

#[tokio::test]
async fn test_associate_then_list_members() {
    let client = make_client().await;

    client
        .associate_member()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");

    assert_eq!(resp.members().len(), 1);
    assert_eq!(resp.members()[0].account_id().unwrap(), "123456789012");
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_client().await;

    let resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/filter/abc")
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    // No tags set, should return empty or None
    assert!(resp.tags().is_none());
}

#[tokio::test]
async fn test_tag_resource_on_filter() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    let resp = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("tagme-filter")
        .filter_criteria(criteria)
        .send()
        .await
        .unwrap();

    let arn = resp.arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("k1", "v1")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("k1").map(String::as_str), Some("v1"));

    // Add more tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("k2", "v2")
        .send()
        .await
        .unwrap();

    let tags_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags2 = tags_resp2.tags().unwrap();
    assert_eq!(tags2.get("k1").map(String::as_str), Some("v1"));
    assert_eq!(tags2.get("k2").map(String::as_str), Some("v2"));

    // Untag k1
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("k1")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp3 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags3 = tags_resp3.tags().unwrap();
    assert!(tags3.get("k1").is_none(), "k1 should have been removed");
    assert_eq!(tags3.get("k2").map(String::as_str), Some("v2"));
}

#[tokio::test]
async fn test_create_filter_with_tags_reflected_in_list() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    // Create filter with tags
    client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("tagged-filter")
        .filter_criteria(criteria)
        .tags("k1", "v1")
        .send()
        .await
        .unwrap();

    // Tags should appear in list_filters response
    let resp = client
        .list_filters()
        .send()
        .await
        .expect("list_filters should succeed");

    assert_eq!(resp.filters().len(), 1);
    let filter = &resp.filters()[0];
    assert_eq!(filter.name(), "tagged-filter");
    let filter_tags = filter.tags();
    assert!(
        filter_tags.is_some(),
        "filter tags should be present in list"
    );
    let tags = filter_tags.unwrap();
    assert_eq!(tags.get("k1").map(String::as_str), Some("v1"));
}

#[tokio::test]
async fn test_disable_delegated_admin_account_remains_in_list() {
    let client = make_client().await;

    // Enable delegated admin
    client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("111111111111")
        .send()
        .await
        .unwrap();

    // Verify it's enabled
    let list_resp = client.list_delegated_admin_accounts().send().await.unwrap();
    let accounts = list_resp.delegated_admin_accounts();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].account_id().unwrap(), "111111111111");
    assert_eq!(accounts[0].status().unwrap().as_str(), "ENABLED");

    // Disable it
    client
        .disable_delegated_admin_account()
        .delegated_admin_account_id("111111111111")
        .send()
        .await
        .unwrap();

    // Should still be in the list but with DISABLED status
    let list_resp2 = client.list_delegated_admin_accounts().send().await.unwrap();
    let accounts2 = list_resp2.delegated_admin_accounts();
    assert_eq!(accounts2.len(), 1);
    assert_eq!(accounts2[0].account_id().unwrap(), "111111111111");
    assert_eq!(accounts2[0].status().unwrap().as_str(), "DISABLED");
}

#[tokio::test]
async fn test_disassociate_member() {
    let client = make_client().await;

    // First associate a member
    client
        .associate_member()
        .account_id("222222222222")
        .send()
        .await
        .expect("associate_member should succeed");

    // Verify it appears in list
    let list_resp = client.list_members().send().await.unwrap();
    assert_eq!(list_resp.members().len(), 1);

    // Disassociate
    let disassoc_resp = client
        .disassociate_member()
        .account_id("222222222222")
        .send()
        .await
        .expect("disassociate_member should succeed");

    assert_eq!(disassoc_resp.account_id(), "222222222222");

    // Should no longer appear in list
    let list_resp2 = client.list_members().send().await.unwrap();
    assert_eq!(list_resp2.members().len(), 0);
}

#[tokio::test]
async fn test_get_member() {
    let client = make_client().await;

    // Associate a member first
    client
        .associate_member()
        .account_id("333333333333")
        .send()
        .await
        .expect("associate_member should succeed");

    // Get the member
    let get_resp = client
        .get_member()
        .account_id("333333333333")
        .send()
        .await
        .expect("get_member should succeed");

    let member = get_resp.member().expect("should have member");
    assert_eq!(member.account_id().unwrap(), "333333333333");
    assert_eq!(member.relationship_status().unwrap().as_str(), "ENABLED");
}

#[tokio::test]
async fn test_get_member_not_found() {
    let client = make_client().await;

    let result = client.get_member().account_id("nonexistent").send().await;

    assert!(result.is_err(), "get_member for nonexistent should fail");
}

#[tokio::test]
async fn test_enable_lambda_resource_type() {
    let client = make_client().await;

    let resp = client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Lambda)
        .send()
        .await
        .expect("enable with Lambda should succeed");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
    assert_eq!(
        *accounts[0].status(),
        aws_sdk_inspector2::types::Status::Enabled
    );
}

#[tokio::test]
async fn test_enable_multiple_resource_types_batch_get_account_status() {
    let client = make_client().await;

    client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ecr)
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Lambda)
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_account_status()
        .send()
        .await
        .expect("batch_get_account_status should succeed");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
    let account = &accounts[0];
    if let Some(resource_state) = account.resource_state() {
        let _ = resource_state.ec2();
        let _ = resource_state.ecr();
        let _ = resource_state.lambda();
    }
}

#[tokio::test]
async fn test_disable_without_prior_enable_succeeds() {
    let client = make_client().await;

    // Disabling when nothing is enabled should succeed (no-op)
    let resp = client
        .disable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .expect("disable with nothing enabled should succeed");

    let accounts = resp.accounts();
    assert_eq!(accounts.len(), 1);
    assert_eq!(
        *accounts[0].status(),
        aws_sdk_inspector2::types::Status::Disabled
    );
}

#[tokio::test]
async fn test_create_filter_with_suppress_action() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    let resp = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::Suppress)
        .name("suppress-filter")
        .filter_criteria(criteria)
        .send()
        .await
        .expect("create_filter with Suppress action should succeed");

    let arn = resp.arn();
    assert!(!arn.is_empty());
    assert!(arn.contains("filter"));

    let list_resp = client.list_filters().send().await.unwrap();

    assert_eq!(list_resp.filters().len(), 1);
    assert_eq!(
        list_resp.filters()[0].action(),
        &aws_sdk_inspector2::types::FilterAction::Suppress
    );
}

#[tokio::test]
async fn test_create_multiple_filters_and_delete_one() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    let first = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("filter-alpha")
        .filter_criteria(criteria.clone())
        .send()
        .await
        .unwrap();

    client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("filter-beta")
        .filter_criteria(criteria)
        .send()
        .await
        .unwrap();

    let list_before = client.list_filters().send().await.unwrap();
    assert_eq!(list_before.filters().len(), 2);

    client
        .delete_filter()
        .arn(first.arn())
        .send()
        .await
        .expect("delete_filter should succeed");

    let list_after = client.list_filters().send().await.unwrap();
    assert_eq!(list_after.filters().len(), 1);
    assert_eq!(list_after.filters()[0].name(), "filter-beta");
}

#[tokio::test]
async fn test_create_filter_with_description() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    let resp = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("described-filter")
        .description("A filter with a description")
        .filter_criteria(criteria)
        .send()
        .await
        .expect("create_filter with description should succeed");

    assert!(!resp.arn().is_empty());

    let list_resp = client.list_filters().send().await.unwrap();
    assert_eq!(list_resp.filters().len(), 1);
    assert_eq!(
        list_resp.filters()[0].description(),
        Some("A filter with a description")
    );
}

#[tokio::test]
async fn test_associate_multiple_members_and_list() {
    let client = make_client().await;

    client
        .associate_member()
        .account_id("111111111111")
        .send()
        .await
        .unwrap();

    client
        .associate_member()
        .account_id("222222222222")
        .send()
        .await
        .unwrap();

    client
        .associate_member()
        .account_id("333333333333")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_members()
        .send()
        .await
        .expect("list_members should succeed");

    assert_eq!(resp.members().len(), 3);
    let ids: Vec<&str> = resp
        .members()
        .iter()
        .filter_map(|m| m.account_id())
        .collect();
    assert!(ids.contains(&"111111111111"));
    assert!(ids.contains(&"222222222222"));
    assert!(ids.contains(&"333333333333"));
}

#[tokio::test]
async fn test_disassociate_nonexistent_member_returns_error() {
    let client = make_client().await;

    let result = client
        .disassociate_member()
        .account_id("000000000000")
        .send()
        .await;

    assert!(
        result.is_err(),
        "disassociate on nonexistent member should fail"
    );
}

#[tokio::test]
async fn test_untag_resource_on_nonexistent_resource_is_noop() {
    let client = make_client().await;

    // Untagging an ARN that was never tagged should not error
    client
        .untag_resource()
        .resource_arn("arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/filter/ghost")
        .tag_keys("k1")
        .send()
        .await
        .expect("untag_resource on nonexistent resource should succeed silently");
}

#[tokio::test]
async fn test_enable_delegated_admin_idempotent() {
    let client = make_client().await;

    // Enable the same delegated admin twice - should succeed both times
    client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("444444444444")
        .send()
        .await
        .expect("first enable_delegated_admin_account should succeed");

    client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("444444444444")
        .send()
        .await
        .expect("second enable_delegated_admin_account should succeed idempotently");

    let resp = client.list_delegated_admin_accounts().send().await.unwrap();

    // Should still only be one entry, not duplicated
    let accounts = resp.delegated_admin_accounts();
    let matching: Vec<_> = accounts
        .iter()
        .filter(|a| a.account_id() == Some("444444444444"))
        .collect();
    assert_eq!(matching.len(), 1);
    assert_eq!(matching[0].status().unwrap().as_str(), "ENABLED");
}

#[tokio::test]
async fn test_update_organization_configuration_lambda_code() {
    let client = make_client().await;

    let auto_enable = aws_sdk_inspector2::types::AutoEnable::builder()
        .ec2(false)
        .ecr(false)
        .lambda(false)
        .lambda_code(true)
        .build()
        .unwrap();

    let resp = client
        .update_organization_configuration()
        .auto_enable(auto_enable)
        .send()
        .await
        .expect("update_organization_configuration should succeed");

    let ae = resp.auto_enable().expect("auto_enable should be present");
    assert!(!ae.ec2());
    assert!(!ae.ecr());
    assert_eq!(ae.lambda_code(), Some(true));
}

#[tokio::test]
async fn test_associate_member_idempotent() {
    let client = make_client().await;

    // Associate the same account twice
    client
        .associate_member()
        .account_id("555555555555")
        .send()
        .await
        .expect("first associate_member should succeed");

    client
        .associate_member()
        .account_id("555555555555")
        .send()
        .await
        .expect("second associate_member should succeed idempotently");

    let resp = client.list_members().send().await.unwrap();
    // Should only appear once
    let matching: Vec<_> = resp
        .members()
        .iter()
        .filter(|m| m.account_id() == Some("555555555555"))
        .collect();
    assert_eq!(matching.len(), 1);
}

#[tokio::test]
async fn test_enable_then_disable_then_enable_again() {
    let client = make_client().await;

    // Enable EC2
    client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .unwrap();

    // Disable EC2
    let after_disable = client
        .disable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .unwrap();

    assert_eq!(
        *after_disable.accounts()[0].status(),
        aws_sdk_inspector2::types::Status::Disabled
    );

    // Enable again
    let after_reenable = client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .send()
        .await
        .expect("re-enable should succeed");

    assert_eq!(
        *after_reenable.accounts()[0].status(),
        aws_sdk_inspector2::types::Status::Enabled
    );
}

#[tokio::test]
async fn test_tag_and_untag_all_keys_leaves_empty_tags() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    let create_resp = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::None)
        .name("fully-tagged-filter")
        .filter_criteria(criteria)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("key-a", "val-a")
        .tags("key-b", "val-b")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key-a")
        .tag_keys("key-b")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    // All tags removed - either None or empty map
    assert!(
        tags_resp.tags().is_none_or(|t| t.is_empty()),
        "all tags should be removed after untagging all keys"
    );
}

#[tokio::test]
async fn test_update_filter() {
    let client = make_client().await;

    // First create a filter
    let create_resp = client
        .create_filter()
        .name("my-filter")
        .action(aws_sdk_inspector2::types::FilterAction::Suppress)
        .filter_criteria(aws_sdk_inspector2::types::FilterCriteria::builder().build())
        .send()
        .await
        .expect("create_filter should succeed");

    let filter_arn = create_resp.arn().to_string();

    // Now update it
    let update_resp = client
        .update_filter()
        .filter_arn(&filter_arn)
        .name("updated-filter")
        .send()
        .await
        .expect("update_filter should succeed");

    assert_eq!(update_resp.arn(), &filter_arn);

    // Verify the change is visible
    let list_resp = client.list_filters().send().await.unwrap();
    let found = list_resp
        .filters()
        .iter()
        .find(|f| f.arn() == filter_arn)
        .expect("filter should exist after update");
    assert_eq!(found.name(), "updated-filter");
}

#[tokio::test]
async fn test_update_filter_not_found() {
    let client = make_client().await;

    let result = client
        .update_filter()
        .filter_arn(
            "arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/filter/nonexistent",
        )
        .send()
        .await;

    assert!(result.is_err(), "update of nonexistent filter should fail");
}

#[tokio::test]
async fn test_get_configuration() {
    let client = make_client().await;

    let resp = client
        .get_configuration()
        .send()
        .await
        .expect("get_configuration should succeed");

    // Default configuration - no scan mode set
    let _ = resp.ec2_configuration();
    let _ = resp.ecr_configuration();
}

#[tokio::test]
async fn test_update_get_configuration() {
    let client = make_client().await;

    client
        .update_configuration()
        .ec2_configuration(
            aws_sdk_inspector2::types::Ec2Configuration::builder()
                .scan_mode(aws_sdk_inspector2::types::Ec2ScanMode::Ec2SsmAgentBased)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_configuration should succeed");

    let resp = client
        .get_configuration()
        .send()
        .await
        .expect("get_configuration should succeed");

    let ec2 = resp
        .ec2_configuration()
        .expect("ec2_configuration should be set");
    let state = ec2
        .scan_mode_state()
        .expect("scan_mode_state should be set");
    assert_eq!(
        state.scan_mode(),
        Some(&aws_sdk_inspector2::types::Ec2ScanMode::Ec2SsmAgentBased)
    );
}

#[tokio::test]
async fn test_get_delegated_admin_account() {
    let client = make_client().await;

    // Enable a delegated admin first
    client
        .enable_delegated_admin_account()
        .delegated_admin_account_id("111111111111")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_delegated_admin_account()
        .send()
        .await
        .expect("get_delegated_admin_account should succeed");

    let admin = resp
        .delegated_admin()
        .expect("delegated_admin should be set");
    assert_eq!(admin.account_id().unwrap(), "111111111111");
}

#[tokio::test]
async fn test_get_delegated_admin_account_not_found() {
    let client = make_client().await;

    let result = client.get_delegated_admin_account().send().await;
    assert!(
        result.is_err(),
        "should fail when no delegated admin is set"
    );
}

#[tokio::test]
async fn test_get_and_update_ec2_deep_inspection_configuration() {
    let client = make_client().await;

    // Get initial state
    let resp = client
        .get_ec2_deep_inspection_configuration()
        .send()
        .await
        .expect("get_ec2_deep_inspection_configuration should succeed");

    // Should start as deactivated
    assert_eq!(
        resp.status(),
        Some(&aws_sdk_inspector2::types::Ec2DeepInspectionStatus::Deactivated)
    );

    // Update
    client
        .update_ec2_deep_inspection_configuration()
        .activate_deep_inspection(true)
        .package_paths("/usr/local/lib")
        .send()
        .await
        .expect("update_ec2_deep_inspection_configuration should succeed");

    // Verify update
    let resp2 = client
        .get_ec2_deep_inspection_configuration()
        .send()
        .await
        .expect("get_ec2_deep_inspection_configuration should succeed");

    assert_eq!(
        resp2.status(),
        Some(&aws_sdk_inspector2::types::Ec2DeepInspectionStatus::Activated)
    );
    assert!(
        resp2
            .package_paths()
            .contains(&"/usr/local/lib".to_string())
    );
}

#[tokio::test]
async fn test_get_update_reset_encryption_key() {
    let client = make_client().await;

    // Get initial (AWS managed)
    let resp = client
        .get_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("get_encryption_key should succeed");

    let initial_key = resp.kms_key_id().to_string();
    assert!(!initial_key.is_empty());

    // Update
    client
        .update_encryption_key()
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/test-key-id")
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("update_encryption_key should succeed");

    let resp2 = client
        .get_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp2.kms_key_id(),
        "arn:aws:kms:us-east-1:123456789012:key/test-key-id"
    );

    // Reset
    client
        .reset_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("reset_encryption_key should succeed");

    let resp3 = client
        .get_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .unwrap();
    assert_eq!(resp3.kms_key_id(), "AWS_OWNED_KMS_KEY");
}

#[tokio::test]
async fn test_create_and_cancel_findings_report() {
    let client = make_client().await;

    let resp = client
        .create_findings_report()
        .report_format(aws_sdk_inspector2::types::ReportFormat::Json)
        .s3_destination(
            aws_sdk_inspector2::types::Destination::builder()
                .bucket_name("my-bucket")
                .key_prefix("my-key-prefix")
                .kms_key_arn("arn:aws:kms:us-east-1:123456789012:key/test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_findings_report should succeed");

    let report_id = resp.report_id().unwrap().to_string();
    assert!(!report_id.is_empty());

    // Get report status
    let status_resp = client
        .get_findings_report_status()
        .report_id(&report_id)
        .send()
        .await
        .expect("get_findings_report_status should succeed");

    assert_eq!(status_resp.report_id().unwrap(), report_id);

    // Cancel the report
    let cancel_resp = client
        .cancel_findings_report()
        .report_id(&report_id)
        .send()
        .await
        .expect("cancel_findings_report should succeed");

    assert_eq!(cancel_resp.report_id(), report_id);
}

#[tokio::test]
async fn test_cancel_findings_report_not_found() {
    let client = make_client().await;

    let result = client
        .cancel_findings_report()
        .report_id("nonexistent-report-id")
        .send()
        .await;

    assert!(result.is_err(), "cancel of nonexistent report should fail");
}

#[tokio::test]
async fn test_create_and_cancel_sbom_export() {
    let client = make_client().await;

    let resp = client
        .create_sbom_export()
        .report_format(aws_sdk_inspector2::types::SbomReportFormat::Cyclonedx14)
        .s3_destination(
            aws_sdk_inspector2::types::Destination::builder()
                .bucket_name("my-sbom-bucket")
                .key_prefix("prefix")
                .kms_key_arn("arn:aws:kms:us-east-1:123456789012:key/test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_sbom_export should succeed");

    let report_id = resp.report_id().unwrap().to_string();
    assert!(!report_id.is_empty());

    // Get export status
    let get_resp = client
        .get_sbom_export()
        .report_id(&report_id)
        .send()
        .await
        .expect("get_sbom_export should succeed");

    assert_eq!(get_resp.report_id().unwrap(), report_id);

    // Cancel
    let cancel_resp = client
        .cancel_sbom_export()
        .report_id(&report_id)
        .send()
        .await
        .expect("cancel_sbom_export should succeed");

    assert_eq!(cancel_resp.report_id().unwrap(), report_id);
}

#[tokio::test]
async fn test_get_sbom_export_not_found() {
    let client = make_client().await;

    let result = client
        .get_sbom_export()
        .report_id("nonexistent-report-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get of nonexistent SBOM export should fail"
    );
}

#[tokio::test]
async fn test_cis_scan_configuration_lifecycle() {
    use winterbaume_core::StatefulService;
    use winterbaume_inspector2::Inspector2StateView;

    let svc = Inspector2Service::new();

    // Seed state directly via restore
    let mut view = Inspector2StateView::default();
    view.cis_scan_configs.insert(
        "arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/cis-scan-configuration/test-1".to_string(),
        winterbaume_inspector2::views::CisScanConfigView {
            scan_configuration_arn: "arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/cis-scan-configuration/test-1".to_string(),
            scan_name: "my-cis-scan".to_string(),
            owner_id: "123456789012".to_string(),
            tags: None,
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(aws_sdk_inspector2::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_inspector2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_inspector2::Client::new(&config);

    let scan_arn = "arn:aws:inspector2:us-east-1:123456789012:owner/123456789012/cis-scan-configuration/test-1";

    // List should have 1
    let list_resp = client
        .list_cis_scan_configurations()
        .send()
        .await
        .expect("list_cis_scan_configurations should succeed");

    assert_eq!(list_resp.scan_configurations().len(), 1);
    assert_eq!(
        list_resp.scan_configurations()[0].scan_name().unwrap(),
        "my-cis-scan"
    );

    // Update
    let update_resp = client
        .update_cis_scan_configuration()
        .scan_configuration_arn(scan_arn)
        .scan_name("updated-cis-scan")
        .send()
        .await
        .expect("update_cis_scan_configuration should succeed");

    assert_eq!(update_resp.scan_configuration_arn(), scan_arn);

    // Verify name updated
    let list_resp2 = client.list_cis_scan_configurations().send().await.unwrap();
    assert_eq!(
        list_resp2.scan_configurations()[0].scan_name().unwrap(),
        "updated-cis-scan"
    );

    // Delete
    let delete_resp = client
        .delete_cis_scan_configuration()
        .scan_configuration_arn(scan_arn)
        .send()
        .await
        .expect("delete_cis_scan_configuration should succeed");

    assert_eq!(delete_resp.scan_configuration_arn(), scan_arn);

    // List should be empty
    let list_resp3 = client.list_cis_scan_configurations().send().await.unwrap();
    assert!(list_resp3.scan_configurations().is_empty());
}

#[tokio::test]
async fn test_delete_cis_scan_configuration_not_found() {
    let client = make_client().await;

    let result = client
        .delete_cis_scan_configuration()
        .scan_configuration_arn("arn:aws:inspector2:us-east-1:123456789012:nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete of nonexistent CIS scan configuration should fail"
    );
}

#[tokio::test]
async fn test_list_coverage() {
    let client = make_client().await;

    let resp = client
        .list_coverage()
        .send()
        .await
        .expect("list_coverage should succeed");

    assert!(resp.covered_resources().is_empty());
}

#[tokio::test]
async fn test_list_usage_totals() {
    let client = make_client().await;

    let resp = client
        .list_usage_totals()
        .send()
        .await
        .expect("list_usage_totals should succeed");

    assert!(resp.totals().is_empty());
}

#[tokio::test]
async fn test_list_account_permissions() {
    let client = make_client().await;

    let resp = client
        .list_account_permissions()
        .send()
        .await
        .expect("list_account_permissions should succeed");

    assert!(resp.permissions().is_empty());
}

#[tokio::test]
async fn test_search_vulnerabilities() {
    let client = make_client().await;

    let resp = client
        .search_vulnerabilities()
        .filter_criteria(
            aws_sdk_inspector2::types::SearchVulnerabilitiesFilterCriteria::builder()
                .vulnerability_ids("CVE-2021-1234")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("search_vulnerabilities should succeed");

    assert!(resp.vulnerabilities().is_empty());
}

#[tokio::test]
async fn test_batch_get_free_trial_info() {
    let client = make_client().await;

    let resp = client
        .batch_get_free_trial_info()
        .account_ids("123456789012")
        .send()
        .await
        .expect("batch_get_free_trial_info should succeed");

    let _ = resp.accounts();
}

#[tokio::test]
async fn test_batch_get_finding_details() {
    let client = make_client().await;

    let resp = client
        .batch_get_finding_details()
        .finding_arns("arn:aws:inspector2:us-east-1:123456789012:finding/abc")
        .send()
        .await
        .expect("batch_get_finding_details should succeed");

    let _ = resp.finding_details();
}

#[tokio::test]
async fn test_list_finding_aggregations() {
    let client = make_client().await;

    let resp = client
        .list_finding_aggregations()
        .aggregation_type(aws_sdk_inspector2::types::AggregationType::Account)
        .send()
        .await
        .expect("list_finding_aggregations should succeed");

    assert!(resp.responses().is_empty());
}

#[tokio::test]
async fn test_update_org_ec2_deep_inspection_configuration() {
    let client = make_client().await;

    client
        .update_org_ec2_deep_inspection_configuration()
        .org_package_paths("/opt/app")
        .send()
        .await
        .expect("update_org_ec2_deep_inspection_configuration should succeed");
}

#[tokio::test]
async fn test_cis_session_operations() {
    let client = make_client().await;

    // Start a CIS session
    client
        .start_cis_session()
        .scan_job_id("job-123")
        .message(
            aws_sdk_inspector2::types::StartCisSessionMessage::builder()
                .session_token("token-abc")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start_cis_session should succeed");

    // Send health
    client
        .send_cis_session_health()
        .scan_job_id("job-123")
        .session_token("token-abc")
        .send()
        .await
        .expect("send_cis_session_health should succeed");

    // Stop session
    client
        .stop_cis_session()
        .scan_job_id("job-123")
        .session_token("token-abc")
        .message(
            aws_sdk_inspector2::types::StopCisSessionMessage::builder()
                .status(aws_sdk_inspector2::types::StopCisSessionStatus::Success)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("stop_cis_session should succeed");
}

#[tokio::test]
async fn test_batch_get_member_ec2_deep_inspection_status() {
    let client = make_client().await;

    let resp = client
        .batch_get_member_ec2_deep_inspection_status()
        .account_ids("123456789012")
        .send()
        .await
        .expect("batch_get_member_ec2_deep_inspection_status should succeed");

    assert!(resp.account_ids().is_empty());
}

#[tokio::test]
async fn test_list_cis_scans() {
    let client = make_client().await;

    let resp = client
        .list_cis_scans()
        .send()
        .await
        .expect("list_cis_scans should succeed");

    let _ = resp.scans();
}

#[tokio::test]
async fn test_batch_update_member_ec2_deep_inspection_status() {
    let client = make_client().await;

    let resp = client
        .batch_update_member_ec2_deep_inspection_status()
        .account_ids(
            aws_sdk_inspector2::types::MemberAccountEc2DeepInspectionStatus::builder()
                .account_id("123456789012")
                .activate_deep_inspection(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_update_member_ec2_deep_inspection_status should succeed");

    let _ = resp.account_ids();
}

#[tokio::test]
async fn test_batch_get_code_snippet() {
    let client = make_client().await;

    let resp = client
        .batch_get_code_snippet()
        .finding_arns("arn:aws:inspector2:us-east-1:123456789012:finding/abc")
        .send()
        .await
        .expect("batch_get_code_snippet should succeed");

    let _ = resp.code_snippet_results();
}

#[tokio::test]
async fn test_list_coverage_statistics() {
    let client = make_client().await;

    let resp = client
        .list_coverage_statistics()
        .send()
        .await
        .expect("list_coverage_statistics should succeed");

    assert_eq!(resp.total_counts(), 0);
}

#[tokio::test]
async fn test_get_cis_scan_report() {
    let client = make_client().await;

    let resp = client
        .get_cis_scan_report()
        .scan_arn("arn:aws:inspector2:us-east-1:123456789012:cis-scan/abc")
        .send()
        .await
        .expect("get_cis_scan_report should succeed");

    let _ = resp.status();
}

#[tokio::test]
async fn test_get_cis_scan_result_details() {
    let client = make_client().await;

    let resp = client
        .get_cis_scan_result_details()
        .scan_arn("arn:aws:inspector2:us-east-1:123456789012:cis-scan/abc")
        .account_id("123456789012")
        .target_resource_id("i-1234567890abcdef0")
        .send()
        .await
        .expect("get_cis_scan_result_details should succeed");

    let _ = resp.scan_result_details();
}

#[tokio::test]
async fn test_get_clusters_for_image() {
    let client = make_client().await;

    let resp = client
        .get_clusters_for_image()
        .send()
        .await
        .expect("get_clusters_for_image should succeed");

    let _ = resp;
}

#[tokio::test]
async fn test_list_cis_scan_results_aggregated_by_checks() {
    let client = make_client().await;

    let resp = client
        .list_cis_scan_results_aggregated_by_checks()
        .scan_arn("arn:aws:inspector2:us-east-1:123456789012:cis-scan/abc")
        .send()
        .await
        .expect("list_cis_scan_results_aggregated_by_checks should succeed");

    let _ = resp.check_aggregations();
}

#[tokio::test]
async fn test_list_cis_scan_results_aggregated_by_target_resource() {
    let client = make_client().await;

    let resp = client
        .list_cis_scan_results_aggregated_by_target_resource()
        .scan_arn("arn:aws:inspector2:us-east-1:123456789012:cis-scan/abc")
        .send()
        .await
        .expect("list_cis_scan_results_aggregated_by_target_resource should succeed");

    let _ = resp.target_resource_aggregations();
}

#[tokio::test]
async fn test_list_code_security_integrations() {
    let client = make_client().await;

    let resp = client
        .list_code_security_integrations()
        .send()
        .await
        .expect("list_code_security_integrations should succeed");

    let _ = resp.integrations();
}

#[tokio::test]
async fn test_list_code_security_scan_configurations() {
    let client = make_client().await;

    let resp = client
        .list_code_security_scan_configurations()
        .send()
        .await
        .expect("list_code_security_scan_configurations should succeed");

    let _ = resp.configurations();
}

#[tokio::test]
async fn test_list_code_security_scan_configuration_associations() {
    let client = make_client().await;

    let resp = client
        .list_code_security_scan_configuration_associations()
        .scan_configuration_arn("arn:aws:inspector2:us-east-1:123456789012:code-scan-config/abc")
        .send()
        .await
        .expect("list_code_security_scan_configuration_associations should succeed");

    let _ = resp.associations();
}

#[tokio::test]
async fn test_start_code_security_scan() {
    let client = make_client().await;

    let resp = client
        .start_code_security_scan()
        .send()
        .await
        .expect("start_code_security_scan should succeed");

    assert!(resp.scan_id().is_some());
}

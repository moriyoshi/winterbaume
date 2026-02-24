/// Scenario tests for winterbaume-inspector2.
/// Each test exercises a realistic multi-operation workflow, asserting
/// business outcomes rather than per-API response shapes.
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

/// Scenario: Inspector2 scanning lifecycle
///
/// A security team enables Inspector2 scanning for EC2 and ECR, verifies the
/// account status transitions, then disables all scanning and confirms the
/// account is marked DISABLED.  Chains: enable → BatchGetAccountStatus →
/// disable → BatchGetAccountStatus.
#[tokio::test]
async fn test_scanning_lifecycle() {
    let client = make_client().await;

    // Initially, the account should be DISABLED.
    let status_before = client
        .batch_get_account_status()
        .send()
        .await
        .expect("BatchGetAccountStatus should succeed before enable");
    let account_before = &status_before.accounts()[0];
    assert_eq!(
        *account_before
            .state()
            .expect("state must be present")
            .status(),
        aws_sdk_inspector2::types::Status::Disabled,
        "account should start DISABLED"
    );

    // Enable EC2 and ECR scanning.
    let enable_resp = client
        .enable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ecr)
        .send()
        .await
        .expect("Enable should succeed");
    assert_eq!(
        *enable_resp.accounts()[0].status(),
        aws_sdk_inspector2::types::Status::Enabled,
        "account should be ENABLED after enabling EC2+ECR"
    );

    // Confirm via BatchGetAccountStatus.
    let status_mid = client
        .batch_get_account_status()
        .send()
        .await
        .expect("BatchGetAccountStatus should succeed after enable");
    assert_eq!(
        *status_mid.accounts()[0]
            .state()
            .expect("state must be present")
            .status(),
        aws_sdk_inspector2::types::Status::Enabled,
        "BatchGetAccountStatus should reflect ENABLED"
    );

    // Disable all scanning.
    client
        .disable()
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ec2)
        .resource_types(aws_sdk_inspector2::types::ResourceScanType::Ecr)
        .send()
        .await
        .expect("Disable should succeed");

    // Account should now be DISABLED.
    let status_after = client
        .batch_get_account_status()
        .send()
        .await
        .expect("BatchGetAccountStatus should succeed after disable");
    assert_eq!(
        *status_after.accounts()[0]
            .state()
            .expect("state must be present")
            .status(),
        aws_sdk_inspector2::types::Status::Disabled,
        "account should be DISABLED after disabling all resource types"
    );
}

/// Scenario: Filter management workflow
///
/// A security team creates a suppression filter, verifies it appears in the
/// list, updates it, then deletes it and confirms the list is empty.
/// Chains: CreateFilter → ListFilters → UpdateFilter → DeleteFilter →
/// ListFilters.
#[tokio::test]
async fn test_filter_management_workflow() {
    let client = make_client().await;

    let criteria = aws_sdk_inspector2::types::FilterCriteria::builder().build();

    // Create a filter.
    let create_resp = client
        .create_filter()
        .action(aws_sdk_inspector2::types::FilterAction::Suppress)
        .name("suppress-low-severity")
        .filter_criteria(criteria)
        .send()
        .await
        .expect("CreateFilter should succeed");
    let filter_arn = create_resp.arn().to_string();
    assert!(!filter_arn.is_empty(), "filter ARN must be non-empty");

    // Verify it appears in the list.
    let list_resp = client
        .list_filters()
        .send()
        .await
        .expect("ListFilters should succeed");
    assert_eq!(list_resp.filters().len(), 1);
    assert_eq!(list_resp.filters()[0].name(), "suppress-low-severity");

    // Update the filter name.
    let update_resp = client
        .update_filter()
        .filter_arn(&filter_arn)
        .name("suppress-low-severity-v2")
        .send()
        .await
        .expect("UpdateFilter should succeed");
    assert_eq!(update_resp.arn(), filter_arn);

    // Confirm updated name in list.
    let list_after_update = client
        .list_filters()
        .send()
        .await
        .expect("ListFilters should succeed after update");
    assert_eq!(
        list_after_update.filters()[0].name(),
        "suppress-low-severity-v2"
    );

    // Delete the filter.
    client
        .delete_filter()
        .arn(&filter_arn)
        .send()
        .await
        .expect("DeleteFilter should succeed");

    // List should now be empty.
    let list_after_delete = client
        .list_filters()
        .send()
        .await
        .expect("ListFilters should succeed after delete");
    assert_eq!(
        list_after_delete.filters().len(),
        0,
        "filter list should be empty after deletion"
    );
}

/// Scenario: Member account association lifecycle
///
/// An organisation administrator associates a member account, retrieves it,
/// lists all members, then disassociates it and confirms it is gone.
/// Chains: AssociateMember → GetMember → ListMembers → DisassociateMember →
/// GetMember (expect error).
#[tokio::test]
async fn test_member_account_lifecycle() {
    let client = make_client().await;

    let member_account = "111122223333";

    // Associate the member.
    let assoc_resp = client
        .associate_member()
        .account_id(member_account)
        .send()
        .await
        .expect("AssociateMember should succeed");
    assert_eq!(assoc_resp.account_id(), member_account);

    // Retrieve the member.
    let get_resp = client
        .get_member()
        .account_id(member_account)
        .send()
        .await
        .expect("GetMember should succeed");
    let member = get_resp.member().expect("member must be present");
    assert_eq!(member.account_id().unwrap_or_default(), member_account);

    // Confirm it appears in the list.
    let list_resp = client
        .list_members()
        .send()
        .await
        .expect("ListMembers should succeed");
    assert_eq!(list_resp.members().len(), 1);

    // Disassociate the member.
    client
        .disassociate_member()
        .account_id(member_account)
        .send()
        .await
        .expect("DisassociateMember should succeed");

    // Retrieving the member should now fail.
    let get_after = client.get_member().account_id(member_account).send().await;
    assert!(
        get_after.is_err(),
        "GetMember should fail after DisassociateMember"
    );
}

/// Scenario: Encryption key rotation workflow
///
/// A security team retrieves the current encryption key (AWS-managed default),
/// updates it to a custom KMS key, verifies the change, then resets it back
/// to the default.
/// Chains: GetEncryptionKey → UpdateEncryptionKey → GetEncryptionKey →
/// ResetEncryptionKey → GetEncryptionKey.
#[tokio::test]
async fn test_encryption_key_rotation() {
    let client = make_client().await;

    // Default key should be the AWS-owned placeholder.
    let initial = client
        .get_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("GetEncryptionKey should succeed initially");
    assert_eq!(
        initial.kms_key_id(),
        "AWS_OWNED_KMS_KEY",
        "default key should be AWS_OWNED_KMS_KEY"
    );

    // Update to a custom KMS key.
    let custom_key = "arn:aws:kms:us-east-1:123456789012:key/my-custom-key";
    client
        .update_encryption_key()
        .kms_key_id(custom_key)
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("UpdateEncryptionKey should succeed");

    let after_update = client
        .get_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("GetEncryptionKey should succeed after update");
    assert_eq!(
        after_update.kms_key_id(),
        custom_key,
        "key should reflect custom KMS key after update"
    );

    // Reset to AWS-managed default.
    client
        .reset_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("ResetEncryptionKey should succeed");

    let after_reset = client
        .get_encryption_key()
        .resource_type(aws_sdk_inspector2::types::ResourceType::AwsEc2Instance)
        .scan_type(aws_sdk_inspector2::types::ScanType::Network)
        .send()
        .await
        .expect("GetEncryptionKey should succeed after reset");
    assert_eq!(
        after_reset.kms_key_id(),
        "AWS_OWNED_KMS_KEY",
        "key should revert to AWS_OWNED_KMS_KEY after reset"
    );
}

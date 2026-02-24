use aws_sdk_organizations::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_organizations::OrganizationsService;

async fn make_organizations_client() -> aws_sdk_organizations::Client {
    let mock = MockAws::builder()
        .with_service(OrganizationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_organizations::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_organizations::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_organization() {
    let client = make_organizations_client().await;

    let resp = client
        .create_organization()
        .send()
        .await
        .expect("create_organization should succeed");

    let org = resp.organization().expect("should have organization");
    assert!(org.id().is_some());
    assert!(org.arn().unwrap().contains("organization"));
    assert_eq!(org.master_account_id(), Some("123456789012"));

    let desc_resp = client
        .describe_organization()
        .send()
        .await
        .expect("describe_organization should succeed");

    let org = desc_resp.organization().expect("should have organization");
    assert_eq!(org.master_account_id(), Some("123456789012"));
}

#[tokio::test]
async fn test_create_organization_twice_fails() {
    let client = make_organizations_client().await;

    client
        .create_organization()
        .send()
        .await
        .expect("first create should succeed");

    let result = client.create_organization().send().await;
    assert!(result.is_err(), "second create should fail");
}

#[tokio::test]
async fn test_describe_organization_without_org_fails() {
    let client = make_organizations_client().await;

    let result = client.describe_organization().send().await;
    assert!(result.is_err(), "describe without org should fail");
}

#[tokio::test]
async fn test_create_and_describe_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let resp = client
        .create_account()
        .account_name("TestAccount")
        .email("test@example.com")
        .send()
        .await
        .expect("create_account should succeed");

    let status = resp.create_account_status().expect("should have status");
    assert_eq!(
        status.state(),
        Some(&aws_sdk_organizations::types::CreateAccountState::Succeeded)
    );
    let account_id = status.account_id().expect("should have account id");

    let desc_resp = client
        .describe_account()
        .account_id(account_id)
        .send()
        .await
        .expect("describe_account should succeed");

    let account = desc_resp.account().expect("should have account");
    assert_eq!(account.name(), Some("TestAccount"));
    assert_eq!(account.email(), Some("test@example.com"));
}

#[tokio::test]
async fn test_list_accounts() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .create_account()
        .account_name("Account1")
        .email("a1@example.com")
        .send()
        .await
        .unwrap();

    client
        .create_account()
        .account_name("Account2")
        .email("a2@example.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_accounts()
        .send()
        .await
        .expect("list_accounts should succeed");

    // 3 accounts: master + 2 created
    assert_eq!(resp.accounts().len(), 3);
}

#[tokio::test]
async fn test_describe_nonexistent_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .describe_account()
        .account_id("999999999999")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent account should fail");
}

#[tokio::test]
async fn test_list_organizational_units() {
    let client = make_organizations_client().await;

    client
        .create_organization()
        .send()
        .await
        .expect("create_organization should succeed");

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // list_organizational_units_for_parent with the root ID as parent
    let ou_resp = client
        .list_organizational_units_for_parent()
        .parent_id(&root_id)
        .send()
        .await
        .expect("list_organizational_units_for_parent should succeed");

    // No OUs have been created, so the list should be empty
    assert!(
        ou_resp.organizational_units().is_empty(),
        "should have no OUs initially"
    );
}

// ==================== CloseAccount tests ====================

#[tokio::test]
async fn test_close_account_without_org_fails() {
    let client = make_organizations_client().await;

    let result = client
        .close_account()
        .account_id("111111111112")
        .send()
        .await;
    assert!(result.is_err(), "close without org should fail");
}

#[tokio::test]
async fn test_close_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // CloseAccount should succeed and return empty body
    client
        .close_account()
        .account_id(&account_id)
        .send()
        .await
        .expect("close_account should succeed");
}

#[tokio::test]
async fn test_close_account_puts_account_in_suspended_status() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .close_account()
        .account_id(&account_id)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_account()
        .account_id(&account_id)
        .send()
        .await
        .unwrap();

    let account = desc.account().unwrap();
    assert_eq!(
        account.status(),
        Some(&aws_sdk_organizations::types::AccountStatus::Suspended)
    );

    // Closing again should fail with AccountAlreadyClosedException
    let result = client.close_account().account_id(&account_id).send().await;
    assert!(
        result.is_err(),
        "closing already-closed account should fail"
    );
}

#[tokio::test]
async fn test_close_account_not_in_org_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .close_account()
        .account_id("123456789101")
        .send()
        .await;
    assert!(result.is_err(), "closing nonexistent account should fail");
}

// ==================== DescribeCreateAccountStatus tests ====================

#[tokio::test]
async fn test_describe_create_account_status() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let status = create_resp.create_account_status().unwrap();
    let request_id = status.id().unwrap().to_string();
    let account_id = status.account_id().unwrap().to_string();

    let desc_resp = client
        .describe_create_account_status()
        .create_account_request_id(&request_id)
        .send()
        .await
        .expect("describe_create_account_status should succeed");

    let cas = desc_resp.create_account_status().unwrap();
    assert_eq!(cas.id(), Some(request_id.as_str()));
    assert_eq!(cas.account_id(), Some(account_id.as_str()));
    assert_eq!(cas.account_name(), Some("mock-account"));
    assert_eq!(
        cas.state(),
        Some(&aws_sdk_organizations::types::CreateAccountState::Succeeded)
    );
}

// ==================== ListCreateAccountStatus tests ====================

#[tokio::test]
async fn test_list_create_account_status() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    // After creating org, there's 1 account (master) with SUCCEEDED status
    let resp = client
        .list_create_account_status()
        .send()
        .await
        .expect("list_create_account_status should succeed");

    let statuses = resp.create_account_statuses();
    assert_eq!(statuses.len(), 1);

    // Create another account
    client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let resp = client.list_create_account_status().send().await.unwrap();

    let statuses = resp.create_account_statuses();
    assert_eq!(statuses.len(), 2);
    for s in statuses {
        assert_eq!(
            s.state(),
            Some(&aws_sdk_organizations::types::CreateAccountState::Succeeded)
        );
    }
}

#[tokio::test]
async fn test_list_create_account_status_succeeded_filter() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let resp = client
        .list_create_account_status()
        .states(aws_sdk_organizations::types::CreateAccountState::Succeeded)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.create_account_statuses().len(), 1);
}

#[tokio::test]
async fn test_list_create_account_status_in_progress_filter() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let resp = client
        .list_create_account_status()
        .states(aws_sdk_organizations::types::CreateAccountState::InProgress)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.create_account_statuses().len(), 0);
}

// ==================== RegisterDelegatedAdministrator tests ====================

#[tokio::test]
async fn test_register_delegated_administrator() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Register the account as delegated admin
    client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .expect("register should succeed");

    // Verify via list
    let resp = client.list_delegated_administrators().send().await.unwrap();

    let admins = resp.delegated_administrators();
    assert_eq!(admins.len(), 1);
    let admin = &admins[0];
    assert_eq!(admin.id(), Some(account_id.as_str()));
    assert_eq!(admin.email(), Some("mock-account@moto-example.org"));
    assert_eq!(admin.name(), Some("mock-account"));
    assert_eq!(
        admin.status(),
        Some(&aws_sdk_organizations::types::AccountStatus::Active)
    );
    assert_eq!(
        admin.joined_method(),
        Some(&aws_sdk_organizations::types::AccountJoinedMethod::Created)
    );
    assert!(admin.joined_timestamp().is_some());
    assert!(admin.delegation_enabled_date().is_some());
}

#[tokio::test]
async fn test_register_delegated_administrator_master_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .register_delegated_administrator()
        .account_id("123456789012")
        .service_principal("ssm.amazonaws.com")
        .send()
        .await;

    assert!(result.is_err(), "registering master account should fail");
}

#[tokio::test]
async fn test_register_delegated_administrator_nonexistent_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .register_delegated_administrator()
        .account_id("000000000000")
        .service_principal("ssm.amazonaws.com")
        .send()
        .await;

    assert!(
        result.is_err(),
        "registering nonexistent account should fail"
    );
}

#[tokio::test]
async fn test_register_delegated_administrator_unsupported_service_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let result = client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("moto.amazonaws.com")
        .send()
        .await;

    assert!(result.is_err(), "unsupported service principal should fail");
}

#[tokio::test]
async fn test_register_delegated_administrator_already_registered_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .unwrap();

    // Registering same service again should fail
    let result = client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await;

    assert!(result.is_err(), "re-registering same service should fail");
}

// ==================== ListDelegatedAdministrators tests ====================

#[tokio::test]
async fn test_list_delegated_administrators() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp1 = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id_1 = create_resp1
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let create_resp2 = client
        .create_account()
        .account_name("mock-account2")
        .email("mock-account2@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id_2 = create_resp2
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .register_delegated_administrator()
        .account_id(&account_id_1)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .unwrap();

    client
        .register_delegated_administrator()
        .account_id(&account_id_2)
        .service_principal("guardduty.amazonaws.com")
        .send()
        .await
        .unwrap();

    // List all
    let resp = client.list_delegated_administrators().send().await.unwrap();
    assert_eq!(resp.delegated_administrators().len(), 2);

    let mut ids: Vec<String> = resp
        .delegated_administrators()
        .iter()
        .map(|a| a.id().unwrap().to_string())
        .collect();
    ids.sort();

    let mut expected = vec![account_id_1.clone(), account_id_2.clone()];
    expected.sort();
    assert_eq!(ids, expected);

    // Filter by service principal
    let resp = client
        .list_delegated_administrators()
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.delegated_administrators().len(), 1);
    assert_eq!(
        resp.delegated_administrators()[0].id(),
        Some(account_id_1.as_str())
    );
}

#[tokio::test]
async fn test_list_delegated_administrators_unsupported_service_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .list_delegated_administrators()
        .service_principal("moto.amazonaws.com")
        .send()
        .await;

    assert!(result.is_err(), "unsupported service principal should fail");
}

// ==================== ListDelegatedServicesForAccount tests ====================

#[tokio::test]
async fn test_list_delegated_services_for_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .unwrap();

    client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("guardduty.amazonaws.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_delegated_services_for_account()
        .account_id(&account_id)
        .send()
        .await
        .unwrap();

    let services = resp.delegated_services();
    assert_eq!(services.len(), 2);

    let mut principals: Vec<String> = services
        .iter()
        .map(|s| s.service_principal().unwrap().to_string())
        .collect();
    principals.sort();
    assert_eq!(
        principals,
        vec![
            "guardduty.amazonaws.com".to_string(),
            "ssm.amazonaws.com".to_string()
        ]
    );
}

#[tokio::test]
async fn test_list_delegated_services_for_nonexistent_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .list_delegated_services_for_account()
        .account_id("000000000000")
        .send()
        .await;

    assert!(
        result.is_err(),
        "listing services for nonexistent account should fail"
    );
}

#[tokio::test]
async fn test_list_delegated_services_for_unregistered_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    // Master account (123456789012) exists but is not a delegated admin
    let result = client
        .list_delegated_services_for_account()
        .account_id("123456789012")
        .send()
        .await;

    assert!(
        result.is_err(),
        "listing services for non-delegated account should fail"
    );
}

// ==================== DeregisterDelegatedAdministrator tests ====================

#[tokio::test]
async fn test_deregister_delegated_administrator() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .register_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .unwrap();

    // Deregister
    client
        .deregister_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await
        .expect("deregister should succeed");

    // Verify empty
    let resp = client.list_delegated_administrators().send().await.unwrap();
    assert_eq!(resp.delegated_administrators().len(), 0);
}

#[tokio::test]
async fn test_deregister_delegated_administrator_master_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .deregister_delegated_administrator()
        .account_id("123456789012")
        .service_principal("ssm.amazonaws.com")
        .send()
        .await;

    assert!(result.is_err(), "deregistering master account should fail");
}

#[tokio::test]
async fn test_deregister_delegated_administrator_nonexistent_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .deregister_delegated_administrator()
        .account_id("000000000000")
        .service_principal("ssm.amazonaws.com")
        .send()
        .await;

    assert!(
        result.is_err(),
        "deregistering nonexistent account should fail"
    );
}

#[tokio::test]
async fn test_deregister_delegated_administrator_not_registered_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create_resp = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();

    let account_id = create_resp
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let result = client
        .deregister_delegated_administrator()
        .account_id(&account_id)
        .service_principal("ssm.amazonaws.com")
        .send()
        .await;

    assert!(
        result.is_err(),
        "deregistering non-delegated account should fail"
    );
}

// ==================== DeleteOrganization tests ====================

#[tokio::test]
async fn test_delete_organization() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .delete_organization()
        .send()
        .await
        .expect("delete_organization should succeed");

    // Describe should fail now
    let result = client.describe_organization().send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_delete_organization_with_members_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .create_account()
        .account_name("member")
        .email("member@example.com")
        .send()
        .await
        .unwrap();

    let result = client.delete_organization().send().await;
    assert!(result.is_err(), "delete with member accounts should fail");
}

// ==================== CreateOrganizationalUnit tests ====================

#[tokio::test]
async fn test_create_and_describe_organizational_unit() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("Engineering")
        .send()
        .await
        .expect("create_organizational_unit should succeed");

    let ou = ou_resp.organizational_unit().unwrap();
    assert_eq!(ou.name(), Some("Engineering"));
    assert!(ou.id().is_some());
    assert!(ou.arn().unwrap().contains("ou/"));

    let ou_id = ou.id().unwrap().to_string();

    // Describe
    let desc = client
        .describe_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await
        .expect("describe_organizational_unit should succeed");

    let desc_ou = desc.organizational_unit().unwrap();
    assert_eq!(desc_ou.name(), Some("Engineering"));
    assert_eq!(desc_ou.id(), Some(ou_id.as_str()));
}

#[tokio::test]
async fn test_describe_nonexistent_ou_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .describe_organizational_unit()
        .organizational_unit_id("ou-nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent OU should fail");
}

// ==================== UpdateOrganizationalUnit tests ====================

#[tokio::test]
async fn test_update_organizational_unit() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("OldName")
        .send()
        .await
        .unwrap();

    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let updated = client
        .update_organizational_unit()
        .organizational_unit_id(&ou_id)
        .name("NewName")
        .send()
        .await
        .expect("update_organizational_unit should succeed");

    assert_eq!(
        updated.organizational_unit().unwrap().name(),
        Some("NewName")
    );
}

// ==================== DeleteOrganizationalUnit tests ====================

#[tokio::test]
async fn test_delete_organizational_unit() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ToDelete")
        .send()
        .await
        .unwrap();

    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await
        .expect("delete_organizational_unit should succeed");

    // Describe should fail now
    let result = client
        .describe_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ==================== ListRoots tests ====================

#[tokio::test]
async fn test_list_roots() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let resp = client
        .list_roots()
        .send()
        .await
        .expect("list_roots should succeed");

    let roots = resp.roots();
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].name(), Some("Root"));
    assert!(roots[0].id().unwrap().starts_with("r-"));
}

// ==================== CreatePolicy tests ====================

#[tokio::test]
async fn test_create_and_describe_policy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let resp = client
        .create_policy()
        .name("TestSCP")
        .description("A test SCP")
        .content(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"*","Resource":"*"}]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .expect("create_policy should succeed");

    let policy = resp.policy().unwrap();
    let summary = policy.policy_summary().unwrap();
    assert_eq!(summary.name(), Some("TestSCP"));
    assert_eq!(summary.description(), Some("A test SCP"));
    assert!(summary.id().is_some());

    let policy_id = summary.id().unwrap().to_string();

    // Describe
    let desc = client
        .describe_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .expect("describe_policy should succeed");

    let desc_policy = desc.policy().unwrap();
    assert_eq!(
        desc_policy.policy_summary().unwrap().name(),
        Some("TestSCP")
    );
    assert!(desc_policy.content().is_some());
}

// ==================== UpdatePolicy tests ====================

#[tokio::test]
async fn test_update_policy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create = client
        .create_policy()
        .name("OriginalName")
        .description("Original desc")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let updated = client
        .update_policy()
        .policy_id(&policy_id)
        .name("UpdatedName")
        .description("Updated desc")
        .send()
        .await
        .expect("update_policy should succeed");

    let summary = updated.policy().unwrap().policy_summary().unwrap();
    assert_eq!(summary.name(), Some("UpdatedName"));
    assert_eq!(summary.description(), Some("Updated desc"));
}

// ==================== DeletePolicy tests ====================

#[tokio::test]
async fn test_delete_policy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create = client
        .create_policy()
        .name("ToDelete")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .expect("delete_policy should succeed");

    // Describe should fail
    let result = client.describe_policy().policy_id(&policy_id).send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_delete_policy_attached_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let create = client
        .create_policy()
        .name("Attached")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await
        .unwrap();

    let result = client.delete_policy().policy_id(&policy_id).send().await;
    assert!(result.is_err(), "delete attached policy should fail");
}

// ==================== AttachPolicy / DetachPolicy tests ====================

#[tokio::test]
async fn test_attach_and_detach_policy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let create = client
        .create_policy()
        .name("SCP1")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Attach
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await
        .expect("attach_policy should succeed");

    // ListPoliciesForTarget
    let resp = client
        .list_policies_for_target()
        .target_id(&root_id)
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .expect("list_policies_for_target should succeed");

    assert_eq!(resp.policies().len(), 1);
    assert_eq!(resp.policies()[0].name(), Some("SCP1"));

    // ListTargetsForPolicy
    let targets_resp = client
        .list_targets_for_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .expect("list_targets_for_policy should succeed");

    assert_eq!(targets_resp.targets().len(), 1);
    assert_eq!(
        targets_resp.targets()[0].target_id(),
        Some(root_id.as_str())
    );

    // Detach
    client
        .detach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await
        .expect("detach_policy should succeed");

    // Verify detached
    let resp = client
        .list_policies_for_target()
        .target_id(&root_id)
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.policies().len(), 0);
}

#[tokio::test]
async fn test_attach_policy_duplicate_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let create = client
        .create_policy()
        .name("SCP1")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await
        .unwrap();

    let result = client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await;

    assert!(result.is_err(), "duplicate attach should fail");
}

// ==================== ListPolicies tests ====================

#[tokio::test]
async fn test_list_policies() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .create_policy()
        .name("SCP1")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    client
        .create_policy()
        .name("SCP2")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_policies()
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .expect("list_policies should succeed");

    assert_eq!(resp.policies().len(), 2);
}

// ==================== EnablePolicyType / DisablePolicyType tests ====================

#[tokio::test]
async fn test_enable_and_disable_policy_type() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // Enable TAG_POLICY
    let resp = client
        .enable_policy_type()
        .root_id(&root_id)
        .policy_type(aws_sdk_organizations::types::PolicyType::TagPolicy)
        .send()
        .await
        .expect("enable_policy_type should succeed");

    let root = resp.root().unwrap();
    let pt = root.policy_types();
    assert_eq!(pt.len(), 1);

    // Disable TAG_POLICY
    let resp = client
        .disable_policy_type()
        .root_id(&root_id)
        .policy_type(aws_sdk_organizations::types::PolicyType::TagPolicy)
        .send()
        .await
        .expect("disable_policy_type should succeed");

    let root = resp.root().unwrap();
    assert!(root.policy_types().is_empty());
}

#[tokio::test]
async fn test_enable_policy_type_already_enabled_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    client
        .enable_policy_type()
        .root_id(&root_id)
        .policy_type(aws_sdk_organizations::types::PolicyType::TagPolicy)
        .send()
        .await
        .unwrap();

    let result = client
        .enable_policy_type()
        .root_id(&root_id)
        .policy_type(aws_sdk_organizations::types::PolicyType::TagPolicy)
        .send()
        .await;

    assert!(result.is_err(), "enabling already-enabled type should fail");
}

// ==================== EnableAWSServiceAccess / DisableAWSServiceAccess tests ====================

#[tokio::test]
async fn test_enable_and_disable_aws_service_access() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    // Enable
    client
        .enable_aws_service_access()
        .service_principal("guardduty.amazonaws.com")
        .send()
        .await
        .expect("enable_aws_service_access should succeed");

    // List
    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.enabled_service_principals().len(), 1);
    assert_eq!(
        resp.enabled_service_principals()[0].service_principal(),
        Some("guardduty.amazonaws.com")
    );

    // Disable
    client
        .disable_aws_service_access()
        .service_principal("guardduty.amazonaws.com")
        .send()
        .await
        .expect("disable_aws_service_access should succeed");

    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .unwrap();

    assert_eq!(resp.enabled_service_principals().len(), 0);
}

// ==================== ListAccountsForParent tests ====================

#[tokio::test]
async fn test_list_accounts_for_parent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    client
        .create_account()
        .account_name("Child1")
        .email("child1@example.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_accounts_for_parent()
        .parent_id(&root_id)
        .send()
        .await
        .expect("list_accounts_for_parent should succeed");

    // master + Child1
    assert_eq!(resp.accounts().len(), 2);
}

// ==================== ListChildren tests ====================

#[tokio::test]
async fn test_list_children() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("OU1")
        .send()
        .await
        .unwrap();

    // List OU children
    let resp = client
        .list_children()
        .parent_id(&root_id)
        .child_type(aws_sdk_organizations::types::ChildType::OrganizationalUnit)
        .send()
        .await
        .expect("list_children OU should succeed");

    assert_eq!(resp.children().len(), 1);

    // List account children
    let resp = client
        .list_children()
        .parent_id(&root_id)
        .child_type(aws_sdk_organizations::types::ChildType::Account)
        .send()
        .await
        .expect("list_children ACCOUNT should succeed");

    // master account
    assert_eq!(resp.children().len(), 1);
}

// ==================== ListParents tests ====================

#[tokio::test]
async fn test_list_parents() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // Master account parent should be root
    let resp = client
        .list_parents()
        .child_id("123456789012")
        .send()
        .await
        .expect("list_parents should succeed");

    assert_eq!(resp.parents().len(), 1);
    assert_eq!(resp.parents()[0].id(), Some(root_id.as_str()));
    assert_eq!(
        resp.parents()[0].r#type(),
        Some(&aws_sdk_organizations::types::ParentType::Root)
    );
}

// ==================== MoveAccount tests ====================

#[tokio::test]
async fn test_move_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("TargetOU")
        .send()
        .await
        .unwrap();

    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let create = client
        .create_account()
        .account_name("MoveMe")
        .email("moveme@example.com")
        .send()
        .await
        .unwrap();

    let account_id = create
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Move from root to OU
    client
        .move_account()
        .account_id(&account_id)
        .source_parent_id(&root_id)
        .destination_parent_id(&ou_id)
        .send()
        .await
        .expect("move_account should succeed");

    // Verify parent changed
    let parents = client
        .list_parents()
        .child_id(&account_id)
        .send()
        .await
        .unwrap();

    assert_eq!(parents.parents()[0].id(), Some(ou_id.as_str()));
    assert_eq!(
        parents.parents()[0].r#type(),
        Some(&aws_sdk_organizations::types::ParentType::OrganizationalUnit)
    );
}

// ==================== RemoveAccountFromOrganization tests ====================

#[tokio::test]
async fn test_remove_account_from_organization() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create = client
        .create_account()
        .account_name("ToRemove")
        .email("toremove@example.com")
        .send()
        .await
        .unwrap();

    let account_id = create
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .remove_account_from_organization()
        .account_id(&account_id)
        .send()
        .await
        .expect("remove_account should succeed");

    // Account should no longer be found
    let result = client
        .describe_account()
        .account_id(&account_id)
        .send()
        .await;
    assert!(result.is_err(), "describe removed account should fail");
}

#[tokio::test]
async fn test_remove_master_account_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .remove_account_from_organization()
        .account_id("123456789012")
        .send()
        .await;

    assert!(result.is_err(), "removing master account should fail");
}

// ==================== TagResource / UntagResource / ListTagsForResource tests ====================

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let create = client
        .create_account()
        .account_name("Tagged")
        .email("tagged@example.com")
        .send()
        .await
        .unwrap();

    let account_id = create
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Tag
    client
        .tag_resource()
        .resource_id(&account_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List
    let resp = client
        .list_tags_for_resource()
        .resource_id(&account_id)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    let mut keys: Vec<&str> = tags.iter().map(|t| t.key()).collect();
    keys.sort();
    assert_eq!(keys, vec!["env", "team"]);

    // Untag
    client
        .untag_resource()
        .resource_id(&account_id)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(&account_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "team");
}

// ============================================================================
// Ported from moto: test_organizations.py
// ============================================================================

// Ported from moto: test_organizations.py::test_list_organizational_units_for_parent
#[tokio::test]
async fn test_moto_list_organizational_units_for_parent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou02")
        .send()
        .await
        .unwrap();
    client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou03")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_organizational_units_for_parent()
        .parent_id(&root_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.organizational_units().len(), 3);
    for ou in resp.organizational_units() {
        assert!(ou.id().is_some());
        assert!(ou.arn().unwrap().contains("ou/"));
        assert!(ou.name().is_some());
    }
}

// Ported from moto: test_organizations.py::test_list_organizational_units_for_parent_exception
#[tokio::test]
async fn test_moto_list_organizational_units_for_parent_exception() {
    let client = make_organizations_client().await;

    // Without org, listing for a random parent should fail
    let result = client
        .list_organizational_units_for_parent()
        .parent_id("r-0000")
        .send()
        .await;
    assert!(
        result.is_err(),
        "listing OUs for nonexistent parent should fail"
    );
}

// Ported from moto: test_organizations.py::test_list_parents_for_ou
#[tokio::test]
async fn test_moto_list_parents_for_ou() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // Create ou01 under root
    let ou01_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou01_id = ou01_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Parent of ou01 should be root
    let resp = client
        .list_parents()
        .child_id(&ou01_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parents().len(), 1);
    assert_eq!(resp.parents()[0].id(), Some(root_id.as_str()));
    assert_eq!(
        resp.parents()[0].r#type(),
        Some(&aws_sdk_organizations::types::ParentType::Root)
    );

    // Create ou02 under ou01
    let ou02_resp = client
        .create_organizational_unit()
        .parent_id(&ou01_id)
        .name("ou02")
        .send()
        .await
        .unwrap();
    let ou02_id = ou02_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Parent of ou02 should be ou01
    let resp = client
        .list_parents()
        .child_id(&ou02_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parents().len(), 1);
    assert_eq!(resp.parents()[0].id(), Some(ou01_id.as_str()));
    assert_eq!(
        resp.parents()[0].r#type(),
        Some(&aws_sdk_organizations::types::ParentType::OrganizationalUnit)
    );
}

// Ported from moto: test_organizations.py::test_list_parents_for_accounts
#[tokio::test]
async fn test_moto_list_parents_for_accounts() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou01_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou01_id = ou01_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let acct01 = client
        .create_account()
        .account_name("account01")
        .email("account01@moto-example.org")
        .send()
        .await
        .unwrap();
    let account01_id = acct01
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let acct02 = client
        .create_account()
        .account_name("account02")
        .email("account02@moto-example.org")
        .send()
        .await
        .unwrap();
    let account02_id = acct02
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Move account02 to ou01
    client
        .move_account()
        .account_id(&account02_id)
        .source_parent_id(&root_id)
        .destination_parent_id(&ou01_id)
        .send()
        .await
        .unwrap();

    // account01 parent should be root
    let resp = client
        .list_parents()
        .child_id(&account01_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parents()[0].id(), Some(root_id.as_str()));
    assert_eq!(
        resp.parents()[0].r#type(),
        Some(&aws_sdk_organizations::types::ParentType::Root)
    );

    // account02 parent should be ou01
    let resp = client
        .list_parents()
        .child_id(&account02_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.parents()[0].id(), Some(ou01_id.as_str()));
    assert_eq!(
        resp.parents()[0].r#type(),
        Some(&aws_sdk_organizations::types::ParentType::OrganizationalUnit)
    );
}

// Ported from moto: test_organizations.py::test_list_children
#[tokio::test]
async fn test_moto_list_children_complex_hierarchy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // Create ou01 under root
    let ou01_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou01_id = ou01_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Create ou02 under ou01
    let ou02_resp = client
        .create_organizational_unit()
        .parent_id(&ou01_id)
        .name("ou02")
        .send()
        .await
        .unwrap();
    let ou02_id = ou02_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Create two accounts
    let acct01 = client
        .create_account()
        .account_name("account01")
        .email("account01@moto-example.org")
        .send()
        .await
        .unwrap();
    let _account01_id = acct01
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let acct02 = client
        .create_account()
        .account_name("account02")
        .email("account02@moto-example.org")
        .send()
        .await
        .unwrap();
    let account02_id = acct02
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Move account02 to ou01
    client
        .move_account()
        .account_id(&account02_id)
        .source_parent_id(&root_id)
        .destination_parent_id(&ou01_id)
        .send()
        .await
        .unwrap();

    // Root should have master + account01 as ACCOUNT children
    let resp = client
        .list_children()
        .parent_id(&root_id)
        .child_type(aws_sdk_organizations::types::ChildType::Account)
        .send()
        .await
        .unwrap();
    // master account + account01
    assert_eq!(resp.children().len(), 2);
    for child in resp.children() {
        assert_eq!(
            child.r#type(),
            Some(&aws_sdk_organizations::types::ChildType::Account)
        );
    }

    // Root should have ou01 as OU child
    let resp = client
        .list_children()
        .parent_id(&root_id)
        .child_type(aws_sdk_organizations::types::ChildType::OrganizationalUnit)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.children().len(), 1);
    assert_eq!(resp.children()[0].id(), Some(ou01_id.as_str()));

    // ou01 should have account02 as ACCOUNT child
    let resp = client
        .list_children()
        .parent_id(&ou01_id)
        .child_type(aws_sdk_organizations::types::ChildType::Account)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.children().len(), 1);
    assert_eq!(resp.children()[0].id(), Some(account02_id.as_str()));

    // ou01 should have ou02 as OU child
    let resp = client
        .list_children()
        .parent_id(&ou01_id)
        .child_type(aws_sdk_organizations::types::ChildType::OrganizationalUnit)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.children().len(), 1);
    assert_eq!(resp.children()[0].id(), Some(ou02_id.as_str()));
}

// Ported from moto: test_organizations.py::test_list_children_exception
#[tokio::test]
async fn test_moto_list_children_exception() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    // Nonexistent parent
    let result = client
        .list_children()
        .parent_id("r-0000")
        .child_type(aws_sdk_organizations::types::ChildType::Account)
        .send()
        .await;
    assert!(result.is_err(), "nonexistent parent should fail");
}

// Ported from moto: test_organizations.py::test_move_account
#[tokio::test]
async fn test_moto_move_account_and_list_accounts_for_parent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let ou01_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou01_id = ou01_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .move_account()
        .account_id(&account_id)
        .source_parent_id(&root_id)
        .destination_parent_id(&ou01_id)
        .send()
        .await
        .unwrap();

    // Verify the account is now listed under ou01
    let resp = client
        .list_accounts_for_parent()
        .parent_id(&ou01_id)
        .send()
        .await
        .unwrap();

    let ids: Vec<String> = resp
        .accounts()
        .iter()
        .map(|a| a.id().unwrap().to_string())
        .collect();
    assert!(
        ids.contains(&account_id),
        "account should be listed under ou01"
    );
}

// Ported from moto: test_organizations.py::test_create_service_control_policy
#[tokio::test]
async fn test_moto_create_service_control_policy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Sid":"MockPolicyStatement","Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let resp = client
        .create_policy()
        .name("MockServiceControlPolicy")
        .description("A dummy service control policy")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy = resp.policy().unwrap();
    let summary = policy.policy_summary().unwrap();
    assert_eq!(summary.name(), Some("MockServiceControlPolicy"));
    assert_eq!(
        summary.description(),
        Some("A dummy service control policy")
    );
    assert_eq!(policy.content(), Some(policy_doc));
    assert!(summary.id().is_some());
    assert!(summary.arn().unwrap().contains("policy/"));
    assert!(!summary.aws_managed());
}

// Ported from moto: test_organizations.py::test_create_policy_errors
#[tokio::test]
async fn test_moto_create_policy_invalid_type() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    // SDK doesn't have a "MOTO" variant, so we test with TAG_POLICY that is not enabled
    // since that's a valid type but not enabled on root (except SCP).
    // Actually, let's test creating a TAG_POLICY when the type is not enabled on root
    let result = client
        .create_policy()
        .name("moto")
        .description("moto")
        .content("{}")
        .r#type(aws_sdk_organizations::types::PolicyType::TagPolicy)
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating TAG_POLICY without enabling the policy type should fail"
    );
}

// Ported from moto: test_organizations.py::test_attach_service_control_policy
#[tokio::test]
async fn test_moto_attach_policy_to_root_ou_and_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let create = client
        .create_policy()
        .name("MockSCP")
        .description("desc")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Attach to root
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await
        .expect("attach to root should succeed");

    // Attach to OU
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&ou_id)
        .send()
        .await
        .expect("attach to OU should succeed");

    // Attach to account
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&account_id)
        .send()
        .await
        .expect("attach to account should succeed");
}

// Ported from moto: test_organizations.py::test_detach_service_control_policy
#[tokio::test]
async fn test_moto_detach_policy_from_root_ou_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let create = client
        .create_policy()
        .name("MockSCP")
        .description("desc")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Attach/List/Detach for each target type
    for target_id in &[ou_id.as_str(), root_id.as_str(), account_id.as_str()] {
        // Before attach, no non-default policies
        let resp = client
            .list_policies_for_target()
            .target_id(*target_id)
            .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
            .send()
            .await
            .unwrap();
        assert_eq!(resp.policies().len(), 0, "should start with 0 policies");

        // Attach
        client
            .attach_policy()
            .policy_id(&policy_id)
            .target_id(*target_id)
            .send()
            .await
            .unwrap();

        let resp = client
            .list_policies_for_target()
            .target_id(*target_id)
            .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
            .send()
            .await
            .unwrap();
        assert_eq!(
            resp.policies().len(),
            1,
            "should have 1 policy after attach"
        );

        // Detach
        client
            .detach_policy()
            .policy_id(&policy_id)
            .target_id(*target_id)
            .send()
            .await
            .unwrap();

        let resp = client
            .list_policies_for_target()
            .target_id(*target_id)
            .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
            .send()
            .await
            .unwrap();
        assert_eq!(
            resp.policies().len(),
            0,
            "should have 0 policies after detach"
        );
    }
}

// Ported from moto: test_organizations.py::test_list_targets_for_service_control_policy
#[tokio::test]
async fn test_moto_list_targets_for_policy_all_types() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let create = client
        .create_policy()
        .name("MockSCP")
        .description("desc")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await
        .unwrap();
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&ou_id)
        .send()
        .await
        .unwrap();
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&account_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_targets_for_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.targets().len(), 3);
    let mut types: Vec<String> = resp
        .targets()
        .iter()
        .map(|t| format!("{:?}", t.r#type().unwrap()))
        .collect();
    types.sort();
    // Should have ACCOUNT, ORGANIZATIONAL_UNIT, ROOT
    assert!(types.iter().any(|t| t.contains("Account")));
    assert!(types.iter().any(|t| t.contains("OrganizationalUnit")));
    assert!(types.iter().any(|t| t.contains("Root")));

    for target in resp.targets() {
        assert!(target.name().is_some());
        assert!(target.arn().is_some());
        assert!(target.target_id().is_some());
    }
}

// Ported from moto: test_organizations.py::test_list_targets_for_policy_exception
#[tokio::test]
async fn test_moto_list_targets_for_policy_exception() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .list_targets_for_policy()
        .policy_id("p-47fhe9s3")
        .send()
        .await;
    assert!(
        result.is_err(),
        "listing targets for nonexistent policy should fail"
    );
}

// Ported from moto: test_organizations.py::test_list_polices
#[tokio::test]
async fn test_moto_list_policies_multiple() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    for i in 0..4 {
        client
            .create_policy()
            .name(format!("MockSCP{}", i))
            .description("desc")
            .content(policy_doc)
            .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_policies()
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    // 4 policies (winterbaume doesn't create default FullAWSAccess)
    assert_eq!(resp.policies().len(), 4);
    for p in resp.policies() {
        assert!(p.name().is_some());
        assert!(p.id().is_some());
    }
}

// Ported from moto: test_organizations.py::test_delete_service_control_policy
#[tokio::test]
async fn test_moto_delete_policy_and_verify_list() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;

    let base = client
        .list_policies()
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let base_count = base.policies().len();

    let create = client
        .create_policy()
        .name("MockSCP")
        .description("desc")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let after_create = client
        .list_policies()
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    assert_eq!(after_create.policies().len(), base_count + 1);

    client
        .delete_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .unwrap();

    let after_delete = client
        .list_policies()
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    assert_eq!(after_delete.policies().len(), base_count);
}

// Ported from moto: test_organizations.py::test_delete_service_control_policy_exception (non-existent)
#[tokio::test]
async fn test_moto_delete_policy_nonexistent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client.delete_policy().policy_id("p-00000001").send().await;
    assert!(result.is_err(), "deleting nonexistent policy should fail");
}

// Ported from moto: test_organizations.py::test_update_service_control_policy
#[tokio::test]
async fn test_moto_update_policy_description_name_content() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let create = client
        .create_policy()
        .name("MockServiceControlPolicy")
        .description("A dummy service control policy")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Update description
    client
        .update_policy()
        .policy_id(&policy_id)
        .description("foobar")
        .send()
        .await
        .unwrap();
    let desc = client
        .describe_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.policy()
            .unwrap()
            .policy_summary()
            .unwrap()
            .description(),
        Some("foobar")
    );

    // Update name
    client
        .update_policy()
        .policy_id(&policy_id)
        .name("foobar")
        .send()
        .await
        .unwrap();
    let desc = client
        .describe_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.policy().unwrap().policy_summary().unwrap().name(),
        Some("foobar")
    );

    // Update content
    client
        .update_policy()
        .policy_id(&policy_id)
        .content("foobar")
        .send()
        .await
        .unwrap();
    let desc = client
        .describe_policy()
        .policy_id(&policy_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.policy().unwrap().content(), Some("foobar"));
}

// Ported from moto: test_organizations.py::test_update_policy_exception
#[tokio::test]
async fn test_moto_update_policy_nonexistent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client.update_policy().policy_id("p-00000001").send().await;
    assert!(result.is_err(), "updating nonexistent policy should fail");
}

// Ported from moto: test_organizations.py::test_enable_aws_service_access (idempotent)
#[tokio::test]
async fn test_moto_enable_aws_service_access_idempotent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .enable_aws_service_access()
        .service_principal("config.amazonaws.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.enabled_service_principals().len(), 1);

    // Enable again -- should be idempotent
    client
        .enable_aws_service_access()
        .service_principal("config.amazonaws.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.enabled_service_principals().len(), 1);
}

// Ported from moto: test_organizations.py::test_enable_multiple_aws_service_access
#[tokio::test]
async fn test_moto_enable_multiple_aws_service_access() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .enable_aws_service_access()
        .service_principal("config.amazonaws.com")
        .send()
        .await
        .unwrap();
    client
        .enable_aws_service_access()
        .service_principal("ram.amazonaws.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.enabled_service_principals().len(), 2);

    let mut principals: Vec<String> = resp
        .enabled_service_principals()
        .iter()
        .map(|s| s.service_principal().unwrap().to_string())
        .collect();
    principals.sort();
    assert_eq!(
        principals,
        vec![
            "config.amazonaws.com".to_string(),
            "ram.amazonaws.com".to_string()
        ]
    );
}

// Ported from moto: test_organizations.py::test_disable_aws_service_access (idempotent)
#[tokio::test]
async fn test_moto_disable_aws_service_access_idempotent() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    client
        .enable_aws_service_access()
        .service_principal("config.amazonaws.com")
        .send()
        .await
        .unwrap();

    client
        .disable_aws_service_access()
        .service_principal("config.amazonaws.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.enabled_service_principals().len(), 0);

    // Disable again -- should be idempotent
    client
        .disable_aws_service_access()
        .service_principal("config.amazonaws.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_aws_service_access_for_organization()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.enabled_service_principals().len(), 0);
}

// Ported from moto: test_organizations.py::test_tag_resource_organization_organization_root
#[tokio::test]
async fn test_moto_tag_resource_root() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    client
        .tag_resource()
        .resource_id(&root_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag root should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(&root_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");

    // Update tag value
    client
        .tag_resource()
        .resource_id(&root_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("new-value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&root_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].value(), "new-value");

    // Untag
    client
        .untag_resource()
        .resource_id(&root_id)
        .tag_keys("key")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&root_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_organizations.py::test_tag_resource_organization_organizational_unit
#[tokio::test]
async fn test_moto_tag_resource_ou() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_id(&ou_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag OU should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(&ou_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");

    // Update tag
    client
        .tag_resource()
        .resource_id(&ou_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("new-value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&ou_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].value(), "new-value");

    // Untag
    client
        .untag_resource()
        .resource_id(&ou_id)
        .tag_keys("key")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&ou_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_organizations.py::test_tag_resource_policy (for SCP)
#[tokio::test]
async fn test_moto_tag_resource_policy() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let create = client
        .create_policy()
        .name("MockSCP")
        .description("desc")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_id(&policy_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag policy should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(&policy_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");

    // Update
    client
        .tag_resource()
        .resource_id(&policy_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("new-value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&policy_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags()[0].value(), "new-value");

    // Untag
    client
        .untag_resource()
        .resource_id(&policy_id)
        .tag_keys("key")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&policy_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_organizations.py::test_untag_resource (idempotent untag of nonexistent key)
#[tokio::test]
async fn test_moto_untag_resource_nonexistent_key() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_id(&account_id)
        .tags(
            aws_sdk_organizations::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Removing a non-existing tag should not error
    client
        .untag_resource()
        .resource_id(&account_id)
        .tag_keys("not-existing")
        .send()
        .await
        .expect("untagging nonexistent key should succeed");

    // Original tag should still be there
    let resp = client
        .list_tags_for_resource()
        .resource_id(&account_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "key");

    // Now remove the real tag
    client
        .untag_resource()
        .resource_id(&account_id)
        .tag_keys("key")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&account_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_organizations.py::test_delete_organization_with_existing_account
#[tokio::test]
async fn test_moto_delete_organization_lifecycle() {
    let client = make_organizations_client().await;

    // Delete without org should fail
    let result = client.delete_organization().send().await;
    assert!(result.is_err(), "delete without org should fail");

    client.create_organization().send().await.unwrap();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Delete with member account should fail
    let result = client.delete_organization().send().await;
    assert!(result.is_err(), "delete with member should fail");

    // Remove account then delete
    client
        .remove_account_from_organization()
        .account_id(&account_id)
        .send()
        .await
        .unwrap();

    client
        .delete_organization()
        .send()
        .await
        .expect("delete after removing members should succeed");

    // Describe should fail after delete
    let result = client.describe_organization().send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

// Ported from moto: test_organizations.py::test_list_service_control_policies_for_target
#[tokio::test]
async fn test_moto_list_policies_for_target_ou_and_account() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let acct = client
        .create_account()
        .account_name("mock-account")
        .email("mock-account@moto-example.org")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let create = client
        .create_policy()
        .name("MockSCP")
        .description("desc")
        .content(policy_doc)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Attach to OU
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&ou_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_policies_for_target()
        .target_id(&ou_id)
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    assert!(!resp.policies().is_empty());
    assert!(resp.policies().iter().any(|p| p.name() == Some("MockSCP")));

    // Attach to account
    client
        .attach_policy()
        .policy_id(&policy_id)
        .target_id(&account_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_policies_for_target()
        .target_id(&account_id)
        .filter(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();
    assert!(!resp.policies().is_empty());
    assert!(resp.policies().iter().any(|p| p.name() == Some("MockSCP")));
}

// Ported from moto: test_organizations.py::test_describe_organizational_unit_exception
#[tokio::test]
async fn test_moto_describe_ou_exception() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .describe_organizational_unit()
        .organizational_unit_id("ou-zz99-xxxxxxxx")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent OU should fail");
}

// Ported from moto: test_organizations.py::test_delete_organizational_unit (verify via describe)
#[tokio::test]
async fn test_moto_delete_ou_and_verify() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ou01")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await
        .unwrap();

    // Verify the deletion
    let result = client
        .describe_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Tests derived from AWS documentation: Organizations
// ============================================================================

// DescribePolicy with nonexistent ID
#[tokio::test]
async fn test_describe_policy_not_found() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .describe_policy()
        .policy_id("p-00000000")
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("PolicyNotFoundException"),
        "Expected PolicyNotFoundException, got: {err_str}"
    );
}

// UpdateOrganizationalUnit with nonexistent OU
#[tokio::test]
async fn test_update_ou_nonexistent_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .update_organizational_unit()
        .organizational_unit_id("ou-zz00-nonexistent")
        .name("NewName")
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("OrganizationalUnitNotFoundException"),
        "Expected OrganizationalUnitNotFoundException, got: {err_str}"
    );
}

// DeleteOrganizationalUnit with nonexistent OU
#[tokio::test]
async fn test_delete_ou_nonexistent_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let result = client
        .delete_organizational_unit()
        .organizational_unit_id("ou-zz00-nonexistent")
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("OrganizationalUnitNotFoundException"),
        "Expected OrganizationalUnitNotFoundException, got: {err_str}"
    );
}

// DeleteOrganizationalUnit when OU has account children
#[tokio::test]
async fn test_delete_ou_with_account_children_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("OU-with-accounts")
        .send()
        .await
        .unwrap();
    let ou_id = ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Create an account and move it into the OU
    let acct = client
        .create_account()
        .account_name("child-account")
        .email("child-account@example.com")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    client
        .move_account()
        .account_id(&account_id)
        .source_parent_id(&root_id)
        .destination_parent_id(&ou_id)
        .send()
        .await
        .unwrap();

    // Now attempt to delete the OU -- should fail
    let result = client
        .delete_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("OrganizationalUnitNotEmptyException"),
        "Expected OrganizationalUnitNotEmptyException, got: {err_str}"
    );
}

// DeleteOrganizationalUnit when OU has sub-OU children
#[tokio::test]
async fn test_delete_ou_with_ou_children_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let parent_ou_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("ParentOU")
        .send()
        .await
        .unwrap();
    let parent_ou_id = parent_ou_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Create a child OU under the parent OU
    client
        .create_organizational_unit()
        .parent_id(&parent_ou_id)
        .name("ChildOU")
        .send()
        .await
        .unwrap();

    // Now attempt to delete the parent OU -- should fail
    let result = client
        .delete_organizational_unit()
        .organizational_unit_id(&parent_ou_id)
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("OrganizationalUnitNotEmptyException"),
        "Expected OrganizationalUnitNotEmptyException, got: {err_str}"
    );
}

// DetachPolicy when not attached to target
#[tokio::test]
async fn test_detach_policy_not_attached_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    let create = client
        .create_policy()
        .name("SCP-Detach-Test")
        .description("")
        .content(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .r#type(aws_sdk_organizations::types::PolicyType::ServiceControlPolicy)
        .send()
        .await
        .unwrap();

    let policy_id = create
        .policy()
        .unwrap()
        .policy_summary()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Detach without ever attaching -- should fail
    let result = client
        .detach_policy()
        .policy_id(&policy_id)
        .target_id(&root_id)
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("PolicyNotAttachedException"),
        "Expected PolicyNotAttachedException, got: {err_str}"
    );
}

// DisablePolicyType when not enabled
#[tokio::test]
async fn test_disable_policy_type_not_enabled_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // TagPolicy is not enabled by default -- disabling should fail
    let result = client
        .disable_policy_type()
        .root_id(&root_id)
        .policy_type(aws_sdk_organizations::types::PolicyType::TagPolicy)
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("PolicyTypeNotEnabledException"),
        "Expected PolicyTypeNotEnabledException, got: {err_str}"
    );
}

// Full OU lifecycle: create -> describe -> update -> delete -> verify-gone
#[tokio::test]
async fn test_ou_full_lifecycle() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // Create
    let create_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("LifecycleOU")
        .send()
        .await
        .expect("create OU should succeed");

    let ou = create_resp.organizational_unit().unwrap();
    assert_eq!(ou.name(), Some("LifecycleOU"));
    let ou_id = ou.id().unwrap().to_string();

    // Describe
    let desc_resp = client
        .describe_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await
        .expect("describe OU should succeed");
    assert_eq!(
        desc_resp.organizational_unit().unwrap().name(),
        Some("LifecycleOU")
    );

    // Update
    let update_resp = client
        .update_organizational_unit()
        .organizational_unit_id(&ou_id)
        .name("LifecycleOU-Updated")
        .send()
        .await
        .expect("update OU should succeed");
    assert_eq!(
        update_resp.organizational_unit().unwrap().name(),
        Some("LifecycleOU-Updated")
    );

    // Verify updated name via describe
    let desc_after_update = client
        .describe_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_after_update.organizational_unit().unwrap().name(),
        Some("LifecycleOU-Updated")
    );

    // Delete
    client
        .delete_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await
        .expect("delete OU should succeed");

    // Verify gone
    let result = client
        .describe_organizational_unit()
        .organizational_unit_id(&ou_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// MoveAccount with wrong source parent ID
#[tokio::test]
async fn test_move_account_wrong_source_fails() {
    let client = make_organizations_client().await;

    client.create_organization().send().await.unwrap();

    let roots = client.list_roots().send().await.unwrap();
    let root_id = roots.roots()[0].id().unwrap().to_string();

    // Create two OUs
    let ou1_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("OU1")
        .send()
        .await
        .unwrap();
    let ou1_id = ou1_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let ou2_resp = client
        .create_organizational_unit()
        .parent_id(&root_id)
        .name("OU2")
        .send()
        .await
        .unwrap();
    let ou2_id = ou2_resp
        .organizational_unit()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Create an account (it will be under root)
    let acct = client
        .create_account()
        .account_name("MoveWrong")
        .email("movewrong@example.com")
        .send()
        .await
        .unwrap();
    let account_id = acct
        .create_account_status()
        .unwrap()
        .account_id()
        .unwrap()
        .to_string();

    // Try to move with wrong source parent (ou1 instead of root)
    let result = client
        .move_account()
        .account_id(&account_id)
        .source_parent_id(&ou1_id) // account is actually under root
        .destination_parent_id(&ou2_id)
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("SourceParentNotFoundException"),
        "Expected SourceParentNotFoundException, got: {err_str}"
    );
}

// ==================== Handshake / Invite tests ====================

#[tokio::test]
async fn test_invite_account_and_describe_handshake() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let target = aws_sdk_organizations::types::HandshakeParty::builder()
        .id("111222333444")
        .r#type(aws_sdk_organizations::types::HandshakePartyType::Account)
        .build()
        .unwrap();

    let resp = client
        .invite_account_to_organization()
        .target(target)
        .send()
        .await
        .expect("invite_account_to_organization should succeed");

    let hs = resp.handshake().expect("should have handshake");
    let hs_id = hs.id().expect("handshake should have id").to_string();
    assert!(hs.arn().is_some());

    // Describe the handshake
    let desc = client
        .describe_handshake()
        .handshake_id(&hs_id)
        .send()
        .await
        .expect("describe_handshake should succeed");

    let hs2 = desc.handshake().expect("should have handshake");
    assert_eq!(hs2.id(), Some(hs_id.as_str()));
}

#[tokio::test]
async fn test_list_handshakes_for_organization() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    // Initially empty
    let resp = client
        .list_handshakes_for_organization()
        .send()
        .await
        .expect("list_handshakes_for_organization should succeed");
    assert!(resp.handshakes().is_empty());

    // Invite an account
    let target = aws_sdk_organizations::types::HandshakeParty::builder()
        .id("111222333444")
        .r#type(aws_sdk_organizations::types::HandshakePartyType::Account)
        .build()
        .unwrap();
    client
        .invite_account_to_organization()
        .target(target)
        .send()
        .await
        .unwrap();

    // Now list should have one
    let resp = client
        .list_handshakes_for_organization()
        .send()
        .await
        .unwrap();
    assert_eq!(resp.handshakes().len(), 1);
}

#[tokio::test]
async fn test_list_handshakes_for_account() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let resp = client
        .list_handshakes_for_account()
        .send()
        .await
        .expect("list_handshakes_for_account should succeed");
    // May be empty or contain handshakes depending on state
    let _ = resp.handshakes();
}

#[tokio::test]
async fn test_accept_handshake() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let target = aws_sdk_organizations::types::HandshakeParty::builder()
        .id("111222333444")
        .r#type(aws_sdk_organizations::types::HandshakePartyType::Account)
        .build()
        .unwrap();
    let invite_resp = client
        .invite_account_to_organization()
        .target(target)
        .send()
        .await
        .unwrap();
    let hs_id = invite_resp.handshake().unwrap().id().unwrap().to_string();

    let resp = client
        .accept_handshake()
        .handshake_id(&hs_id)
        .send()
        .await
        .expect("accept_handshake should succeed");

    let hs = resp.handshake().expect("should have handshake");
    assert_eq!(
        hs.state(),
        Some(&aws_sdk_organizations::types::HandshakeState::Accepted)
    );
}

#[tokio::test]
async fn test_decline_handshake() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let target = aws_sdk_organizations::types::HandshakeParty::builder()
        .id("222333444555")
        .r#type(aws_sdk_organizations::types::HandshakePartyType::Account)
        .build()
        .unwrap();
    let invite_resp = client
        .invite_account_to_organization()
        .target(target)
        .send()
        .await
        .unwrap();
    let hs_id = invite_resp.handshake().unwrap().id().unwrap().to_string();

    let resp = client
        .decline_handshake()
        .handshake_id(&hs_id)
        .send()
        .await
        .expect("decline_handshake should succeed");

    let hs = resp.handshake().expect("should have handshake");
    assert_eq!(
        hs.state(),
        Some(&aws_sdk_organizations::types::HandshakeState::Declined)
    );
}

#[tokio::test]
async fn test_cancel_handshake() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let target = aws_sdk_organizations::types::HandshakeParty::builder()
        .id("333444555666")
        .r#type(aws_sdk_organizations::types::HandshakePartyType::Account)
        .build()
        .unwrap();
    let invite_resp = client
        .invite_account_to_organization()
        .target(target)
        .send()
        .await
        .unwrap();
    let hs_id = invite_resp.handshake().unwrap().id().unwrap().to_string();

    let resp = client
        .cancel_handshake()
        .handshake_id(&hs_id)
        .send()
        .await
        .expect("cancel_handshake should succeed");

    let hs = resp.handshake().expect("should have handshake");
    assert_eq!(
        hs.state(),
        Some(&aws_sdk_organizations::types::HandshakeState::Canceled)
    );
}

#[tokio::test]
async fn test_describe_handshake_not_found() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let result = client
        .describe_handshake()
        .handshake_id("h-nonexistent")
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe nonexistent handshake should fail"
    );
}

// ==================== EnableAllFeatures test ====================

#[tokio::test]
async fn test_enable_all_features() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let resp = client
        .enable_all_features()
        .send()
        .await
        .expect("enable_all_features should succeed");

    let hs = resp.handshake().expect("should have handshake");
    assert!(hs.id().is_some());
    assert!(hs.arn().is_some());
}

// ==================== CreateGovCloudAccount test ====================

#[tokio::test]
async fn test_create_gov_cloud_account() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let resp = client
        .create_gov_cloud_account()
        .account_name("GovCloudAcct")
        .email("govcloud@example.com")
        .send()
        .await
        .expect("create_gov_cloud_account should succeed");

    let status = resp
        .create_account_status()
        .expect("should have create_account_status");
    assert!(status.account_id().is_some());
    assert!(status.gov_cloud_account_id().is_some());
    assert_eq!(
        status.state(),
        Some(&aws_sdk_organizations::types::CreateAccountState::Succeeded)
    );
}

// ==================== Resource policy tests ====================

#[tokio::test]
async fn test_put_and_describe_resource_policy() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let policy_content = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":"*","Action":"organizations:DescribeOrganization","Resource":"*"}]}"#;

    let put_resp = client
        .put_resource_policy()
        .content(policy_content)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let rp = put_resp
        .resource_policy()
        .expect("should have resource_policy");
    assert_eq!(rp.content(), Some(policy_content));
    let summary = rp.resource_policy_summary().expect("should have summary");
    assert!(summary.id().is_some());
    assert!(summary.arn().is_some());

    // Describe it
    let desc_resp = client
        .describe_resource_policy()
        .send()
        .await
        .expect("describe_resource_policy should succeed");

    let rp2 = desc_resp
        .resource_policy()
        .expect("should have resource_policy");
    assert_eq!(rp2.content(), Some(policy_content));
}

#[tokio::test]
async fn test_delete_resource_policy() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let policy_content = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_resource_policy()
        .content(policy_content)
        .send()
        .await
        .unwrap();

    // Delete it
    client
        .delete_resource_policy()
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    // Describe after delete should fail
    let result = client.describe_resource_policy().send().await;
    assert!(
        result.is_err(),
        "describe_resource_policy after delete should fail"
    );
}

#[tokio::test]
async fn test_describe_resource_policy_without_policy_fails() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let result = client.describe_resource_policy().send().await;
    assert!(
        result.is_err(),
        "describe_resource_policy without policy should fail"
    );
}

// ==================== DescribeEffectivePolicy test ====================

#[tokio::test]
async fn test_describe_effective_policy() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    let resp = client
        .describe_effective_policy()
        .policy_type(aws_sdk_organizations::types::EffectivePolicyType::TagPolicy)
        .send()
        .await
        .expect("describe_effective_policy should succeed");

    let ep = resp
        .effective_policy()
        .expect("should have effective_policy");
    assert!(ep.policy_content().is_some());
    assert!(ep.target_id().is_some());
}

#[tokio::test]
async fn test_describe_effective_policy_without_org_fails() {
    let client = make_organizations_client().await;

    let result = client
        .describe_effective_policy()
        .policy_type(aws_sdk_organizations::types::EffectivePolicyType::TagPolicy)
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe_effective_policy without org should fail"
    );
}

// ==================== LeaveOrganization test ====================

#[tokio::test]
async fn test_leave_organization() {
    let client = make_organizations_client().await;
    client.create_organization().send().await.unwrap();

    client
        .leave_organization()
        .send()
        .await
        .expect("leave_organization should succeed");
}

#[tokio::test]
async fn test_leave_organization_without_org_fails() {
    let client = make_organizations_client().await;

    let result = client.leave_organization().send().await;
    assert!(
        result.is_err(),
        "leave_organization without org should fail"
    );
}

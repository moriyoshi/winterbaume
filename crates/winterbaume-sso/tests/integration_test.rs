use aws_sdk_sso::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sso::SsoService;

async fn setup_sso_service() -> SsoService {
    let service = SsoService::new();

    // Pre-populate state with test data
    {
        let backend = service.backend_state();
        let state = backend.get("123456789012", "us-east-1");
        let mut state = state.write().await;

        // Add a session token
        state.add_session("test-access-token-123");

        // Add accounts
        state.add_account("111111111111", "Development", "dev@example.com");
        state.add_account("222222222222", "Production", "prod@example.com");

        // Add roles
        state.add_role("111111111111", "AdministratorAccess");
        state.add_role("111111111111", "ReadOnlyAccess");
        state.add_role("222222222222", "AdministratorAccess");
    }

    service
}

async fn make_sso_client(service: SsoService) -> aws_sdk_sso::Client {
    let mock = MockAws::builder().with_service(service).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sso::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sso::Client::new(&config)
}

#[tokio::test]
async fn test_list_accounts() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .list_accounts()
        .access_token("test-access-token-123")
        .send()
        .await
        .expect("list_accounts should succeed");

    let accounts = resp.account_list();
    assert_eq!(accounts.len(), 2);

    let account_ids: Vec<&str> = accounts.iter().filter_map(|a| a.account_id()).collect();
    assert!(account_ids.contains(&"111111111111"));
    assert!(account_ids.contains(&"222222222222"));
}

#[tokio::test]
async fn test_list_accounts_checks_account_names() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .list_accounts()
        .access_token("test-access-token-123")
        .send()
        .await
        .expect("list_accounts should succeed");

    let accounts = resp.account_list();
    let dev_account = accounts
        .iter()
        .find(|a| a.account_id() == Some("111111111111"))
        .expect("should find dev account");
    assert_eq!(dev_account.account_name(), Some("Development"));
    assert_eq!(dev_account.email_address(), Some("dev@example.com"));
}

#[tokio::test]
async fn test_list_account_roles() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .list_account_roles()
        .access_token("test-access-token-123")
        .account_id("111111111111")
        .send()
        .await
        .expect("list_account_roles should succeed");

    let roles = resp.role_list();
    assert_eq!(roles.len(), 2);

    let role_names: Vec<&str> = roles.iter().filter_map(|r| r.role_name()).collect();
    assert!(role_names.contains(&"AdministratorAccess"));
    assert!(role_names.contains(&"ReadOnlyAccess"));
}

#[tokio::test]
async fn test_get_role_credentials() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .get_role_credentials()
        .access_token("test-access-token-123")
        .account_id("111111111111")
        .role_name("AdministratorAccess")
        .send()
        .await
        .expect("get_role_credentials should succeed");

    let creds = resp
        .role_credentials()
        .expect("should have role credentials");
    assert!(creds.access_key_id().is_some());
    assert!(creds.secret_access_key().is_some());
    assert!(creds.session_token().is_some());
    assert!(creds.expiration() > 0);
}

#[tokio::test]
async fn test_logout() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    // Logout should succeed
    client
        .logout()
        .access_token("test-access-token-123")
        .send()
        .await
        .expect("logout should succeed");

    // After logout, list_accounts should fail
    let result = client
        .list_accounts()
        .access_token("test-access-token-123")
        .send()
        .await;
    assert!(result.is_err(), "list_accounts after logout should fail");
}

#[tokio::test]
async fn test_invalid_token() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let result = client
        .list_accounts()
        .access_token("invalid-token")
        .send()
        .await;
    assert!(result.is_err(), "invalid token should fail");
}

#[tokio::test]
async fn test_get_role_credentials_nonexistent_role() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let result = client
        .get_role_credentials()
        .access_token("test-access-token-123")
        .account_id("111111111111")
        .role_name("NonexistentRole")
        .send()
        .await;
    assert!(result.is_err(), "nonexistent role should fail");
}

#[tokio::test]
async fn test_list_account_roles_production() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .list_account_roles()
        .access_token("test-access-token-123")
        .account_id("222222222222")
        .send()
        .await
        .expect("list_account_roles for production should succeed");

    let roles = resp.role_list();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0].role_name(), Some("AdministratorAccess"));
}

// ============================================================================
// Tests derived from AWS documentation: IAM Identity Center (SSO) Portal API
// ============================================================================

#[tokio::test]
async fn test_list_accounts_empty() {
    // Service with no pre-populated accounts
    let service = SsoService::new();
    {
        let backend = service.backend_state();
        let state = backend.get("123456789012", "us-east-1");
        let mut state = state.write().await;
        state.add_session("empty-token");
        // No accounts added
    }
    let client = make_sso_client(service).await;

    let resp = client
        .list_accounts()
        .access_token("empty-token")
        .send()
        .await
        .expect("list_accounts on empty state should succeed");

    assert_eq!(
        resp.account_list().len(),
        0,
        "accountList should be empty when no accounts are registered"
    );
}

#[tokio::test]
async fn test_list_account_roles_unknown_account() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let err = client
        .list_account_roles()
        .access_token("test-access-token-123")
        .account_id("999999999999")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for unknown account, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_account_roles_no_roles() {
    // Register an account but no roles for it
    let service = SsoService::new();
    {
        let backend = service.backend_state();
        let state = backend.get("123456789012", "us-east-1");
        let mut state = state.write().await;
        state.add_session("test-token");
        state.add_account("333333333333", "Staging", "staging@example.com");
        // No roles for this account
    }
    let client = make_sso_client(service).await;

    let resp = client
        .list_account_roles()
        .access_token("test-token")
        .account_id("333333333333")
        .send()
        .await
        .expect("list_account_roles for account with no roles should succeed");

    assert_eq!(
        resp.role_list().len(),
        0,
        "roleList should be empty when account has no roles"
    );
}

#[tokio::test]
async fn test_list_account_roles_includes_account_id() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .list_account_roles()
        .access_token("test-access-token-123")
        .account_id("111111111111")
        .send()
        .await
        .expect("list_account_roles should succeed");

    let roles = resp.role_list();
    assert!(!roles.is_empty(), "should have at least one role");
    for role in roles {
        assert_eq!(
            role.account_id(),
            Some("111111111111"),
            "each RoleInfo should carry the account_id"
        );
        assert!(
            role.role_name().is_some(),
            "each RoleInfo should have role_name set"
        );
    }
}

#[tokio::test]
async fn test_get_role_credentials_nonexistent_account() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let err = client
        .get_role_credentials()
        .access_token("test-access-token-123")
        .account_id("000000000000")
        .role_name("AdministratorAccess")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for nonexistent account, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_role_credentials_key_format() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let resp = client
        .get_role_credentials()
        .access_token("test-access-token-123")
        .account_id("111111111111")
        .role_name("AdministratorAccess")
        .send()
        .await
        .expect("get_role_credentials should succeed");

    let creds = resp
        .role_credentials()
        .expect("should have role credentials");

    let access_key_id = creds.access_key_id().unwrap_or_default();
    assert!(
        access_key_id.starts_with("ASIA"),
        "access_key_id should start with 'ASIA' (temporary credentials prefix), got: {access_key_id}"
    );

    let expiration = creds.expiration();
    assert!(
        expiration > 0,
        "expiration should be a positive ms-since-epoch timestamp, got: {expiration}"
    );

    // Secret and session token must be non-empty
    assert!(
        !creds.secret_access_key().unwrap_or_default().is_empty(),
        "secret_access_key should be non-empty"
    );
    assert!(
        !creds.session_token().unwrap_or_default().is_empty(),
        "session_token should be non-empty"
    );
}

#[tokio::test]
async fn test_logout_invalid_token() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    let err = client
        .logout()
        .access_token("completely-unknown-token")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UnauthorizedException"),
        "Logout with unknown token should return UnauthorizedException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_role_credentials_after_logout() {
    let service = setup_sso_service().await;
    let client = make_sso_client(service).await;

    // First logout to invalidate the token
    client
        .logout()
        .access_token("test-access-token-123")
        .send()
        .await
        .expect("logout should succeed");

    // Now GetRoleCredentials should fail with UnauthorizedException
    let err = client
        .get_role_credentials()
        .access_token("test-access-token-123")
        .account_id("111111111111")
        .role_name("AdministratorAccess")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UnauthorizedException"),
        "get_role_credentials after logout should return UnauthorizedException, got: {err_str}"
    );
}

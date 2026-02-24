//! Integration tests for winterbaume STS service.
//!
//! These tests verify that aws-sdk-sts operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_sts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;

/// Helper to create a configured STS client backed by winterbaume.
async fn make_sts_client() -> aws_sdk_sts::Client {
    let mock = MockAws::builder().with_service(StsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sts::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sts::Client::new(&config)
}

#[tokio::test]
async fn test_get_caller_identity() {
    let client = make_sts_client().await;

    let resp = client
        .get_caller_identity()
        .send()
        .await
        .expect("get_caller_identity should succeed");

    assert_eq!(resp.account(), Some("123456789012"));
    assert!(resp.arn().is_some());
    assert!(resp.user_id().is_some());
}

#[tokio::test]
async fn test_assume_role() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/TestRole")
        .role_session_name("test-session")
        .duration_seconds(900)
        .send()
        .await
        .expect("assume_role should succeed");

    let credentials = resp.credentials().expect("should have credentials");
    assert!(credentials.access_key_id().starts_with("ASIA"));
    assert!(!credentials.secret_access_key().is_empty());
    assert!(!credentials.session_token().is_empty());
    // expiration is a DateTime, just verify it's there by accessing it
    let _ = credentials.expiration();

    let assumed_role_user = resp
        .assumed_role_user()
        .expect("should have assumed role user");
    assert!(assumed_role_user.arn().contains("assumed-role"));
    assert!(assumed_role_user.arn().contains("TestRole"));
    assert!(assumed_role_user.arn().contains("test-session"));
    assert!(assumed_role_user.assumed_role_id().contains("test-session"));
}

#[tokio::test]
async fn test_get_session_token() {
    let client = make_sts_client().await;

    let resp = client
        .get_session_token()
        .duration_seconds(900)
        .send()
        .await
        .expect("get_session_token should succeed");

    let credentials = resp.credentials().expect("should have credentials");
    assert!(!credentials.access_key_id().is_empty());
    assert!(!credentials.secret_access_key().is_empty());
    assert!(!credentials.session_token().is_empty());
    let _ = credentials.expiration();
}

#[tokio::test]
async fn test_get_caller_identity_fields() {
    let client = make_sts_client().await;

    let resp = client
        .get_caller_identity()
        .send()
        .await
        .expect("get_caller_identity should succeed");

    let arn = resp.arn().expect("should have arn");
    assert!(
        arn.contains("arn:aws:"),
        "ARN should contain 'arn:aws:', got: {arn}"
    );

    let user_id = resp.user_id().expect("should have user_id");
    assert!(!user_id.is_empty(), "UserId should be non-empty");

    assert_eq!(resp.account(), Some("123456789012"));
}

#[tokio::test]
async fn test_assume_role_credentials_format() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/FormatRole")
        .role_session_name("format-session")
        .duration_seconds(900)
        .send()
        .await
        .expect("assume_role should succeed");

    let credentials = resp.credentials().expect("should have credentials");

    assert!(
        credentials.access_key_id().starts_with("ASIA"),
        "access_key_id should start with ASIA, got: {}",
        credentials.access_key_id()
    );
    assert!(
        credentials.secret_access_key().len() > 10,
        "secret_access_key length should be > 10, got: {}",
        credentials.secret_access_key().len()
    );
    assert!(
        credentials.session_token().len() > 10,
        "session_token length should be > 10, got: {}",
        credentials.session_token().len()
    );
}

#[tokio::test]
async fn test_assume_role_arn_in_response() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/MyRole")
        .role_session_name("my-session")
        .duration_seconds(900)
        .send()
        .await
        .expect("assume_role should succeed");

    let assumed_role_user = resp
        .assumed_role_user()
        .expect("should have assumed role user");

    let expected_arn = "arn:aws:sts::123456789012:assumed-role/MyRole/my-session";
    assert_eq!(
        assumed_role_user.arn(),
        expected_arn,
        "assumed role ARN should match expected pattern"
    );
}

#[tokio::test]
async fn test_get_session_token_credentials_format() {
    let client = make_sts_client().await;

    let resp = client
        .get_session_token()
        .duration_seconds(900)
        .send()
        .await
        .expect("get_session_token should succeed");

    let credentials = resp.credentials().expect("should have credentials");

    let access_key_id = credentials.access_key_id();
    assert!(
        !access_key_id.is_empty(),
        "access_key_id should be a non-empty string"
    );

    let secret_access_key = credentials.secret_access_key();
    assert!(
        !secret_access_key.is_empty(),
        "secret_access_key should be a non-empty string"
    );

    let session_token = credentials.session_token();
    assert!(
        !session_token.is_empty(),
        "session_token should be a non-empty string"
    );
}

#[tokio::test]
async fn test_assume_role_different_sessions() {
    let client = make_sts_client().await;

    let resp1 = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/DiffRole")
        .role_session_name("session-alpha")
        .duration_seconds(900)
        .send()
        .await
        .expect("first assume_role should succeed");

    let resp2 = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/DiffRole")
        .role_session_name("session-beta")
        .duration_seconds(900)
        .send()
        .await
        .expect("second assume_role should succeed");

    let creds1 = resp1.credentials().expect("should have credentials");
    let creds2 = resp2.credentials().expect("should have credentials");

    assert_ne!(
        creds1.access_key_id(),
        creds2.access_key_id(),
        "different session names should produce different access_key_ids"
    );
}

#[tokio::test]
async fn test_assume_role_with_web_identity() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role_with_web_identity()
        .role_arn("arn:aws:iam::123456789012:role/web-id-role")
        .role_session_name("web-session")
        .web_identity_token("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.mock-token")
        .send()
        .await
        .expect("assume_role_with_web_identity should succeed");

    let creds = resp.credentials().expect("should have credentials");
    assert!(creds.access_key_id().starts_with("ASIA"));
    assert!(!creds.secret_access_key().is_empty());
    assert!(!creds.session_token().is_empty());

    let user = resp
        .assumed_role_user()
        .expect("should have assumed role user");
    assert!(user.arn().contains("web-session"));
}

#[tokio::test]
async fn test_assume_role_with_saml() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role_with_saml()
        .role_arn("arn:aws:iam::123456789012:role/saml-role")
        .principal_arn("arn:aws:iam::123456789012:saml-provider/my-provider")
        .saml_assertion("PHNhbWxwOlJlc3BvbnNl...mock-assertion")
        .send()
        .await
        .expect("assume_role_with_saml should succeed");

    let creds = resp.credentials().expect("should have credentials");
    assert!(creds.access_key_id().starts_with("ASIA"));
    assert!(!creds.secret_access_key().is_empty());
    assert!(!creds.session_token().is_empty());

    let user = resp
        .assumed_role_user()
        .expect("should have assumed role user");
    assert!(user.arn().contains("saml-session"));
}

#[tokio::test]
async fn test_get_federation_token() {
    let client = make_sts_client().await;

    let resp = client
        .get_federation_token()
        .name("test-federated-user")
        .send()
        .await
        .expect("get_federation_token should succeed");

    let creds = resp.credentials().expect("should have credentials");
    assert!(creds.access_key_id().starts_with("ASIA"));
    assert!(!creds.secret_access_key().is_empty());
    assert!(!creds.session_token().is_empty());

    let federated_user = resp.federated_user().expect("should have federated user");
    assert!(federated_user.arn().contains("test-federated-user"));
    assert!(
        federated_user
            .federated_user_id()
            .contains("test-federated-user")
    );
}

#[tokio::test]
async fn test_get_caller_identity_default_credentials() {
    let client = make_sts_client().await;

    let resp = client
        .get_caller_identity()
        .send()
        .await
        .expect("get_caller_identity should succeed");

    // Verify the account ID is the default
    assert_eq!(resp.account(), Some("123456789012"));

    // Verify the ARN follows expected pattern
    let arn = resp.arn().expect("should have ARN");
    assert!(
        arn.starts_with("arn:aws:"),
        "ARN should start with 'arn:aws:'"
    );

    // Verify user ID is set
    assert!(resp.user_id().is_some());
    assert!(!resp.user_id().unwrap().is_empty());
}

#[tokio::test]
async fn test_assume_role_credentials_length() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/LenTestRole")
        .role_session_name("len-test-session")
        .duration_seconds(900)
        .send()
        .await
        .expect("assume_role should succeed");

    let creds = resp.credentials().expect("should have credentials");

    // Access key ID should be 20 chars (ASIA + 16)
    assert_eq!(
        creds.access_key_id().len(),
        20,
        "access_key_id should be 20 chars"
    );
    // Secret access key should be 40 chars
    assert_eq!(
        creds.secret_access_key().len(),
        40,
        "secret_access_key should be 40 chars"
    );
    // Session token should be non-empty and start with expected prefix
    assert!(creds.session_token().starts_with("FQoGZXIvYXdzE"));
}

#[tokio::test]
async fn test_assume_role_with_web_identity_credentials_length() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role_with_web_identity()
        .role_arn("arn:aws:iam::123456789012:role/WebIdRole")
        .role_session_name("web-len-session")
        .web_identity_token("mock-token-value")
        .duration_seconds(900)
        .send()
        .await
        .expect("assume_role_with_web_identity should succeed");

    let creds = resp.credentials().expect("should have credentials");
    assert_eq!(creds.access_key_id().len(), 20);
    assert_eq!(creds.secret_access_key().len(), 40);
    assert!(creds.session_token().starts_with("FQoGZXIvYXdzE"));

    let user = resp
        .assumed_role_user()
        .expect("should have assumed_role_user");
    assert!(user.arn().contains("WebIdRole"));
    assert!(user.arn().contains("web-len-session"));
}

#[tokio::test]
async fn test_get_federation_token_arn_format() {
    let client = make_sts_client().await;

    let resp = client
        .get_federation_token()
        .name("Bob")
        .send()
        .await
        .expect("get_federation_token should succeed");

    let fed_user = resp.federated_user().expect("should have federated user");
    assert_eq!(
        fed_user.arn(),
        "arn:aws:sts::123456789012:federated-user/Bob"
    );
    assert_eq!(fed_user.federated_user_id(), "123456789012:Bob");
}

#[tokio::test]
async fn test_assume_role_with_saml_arn_format() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role_with_saml()
        .role_arn("arn:aws:iam::123456789012:role/saml-test-role")
        .principal_arn("arn:aws:iam::123456789012:saml-provider/TestProvider")
        .saml_assertion("bW9jaw==")
        .send()
        .await
        .expect("assume_role_with_saml should succeed");

    let user = resp
        .assumed_role_user()
        .expect("should have assumed_role_user");
    assert!(user.arn().contains("assumed-role"));
    assert!(user.arn().contains("saml-test-role"));
    assert!(user.arn().contains("saml-session"));
}

#[tokio::test]
async fn test_get_session_token_default_duration() {
    let client = make_sts_client().await;

    // Should succeed without specifying duration (use default)
    let resp = client
        .get_session_token()
        .send()
        .await
        .expect("get_session_token without duration should succeed");

    let creds = resp.credentials().expect("should have credentials");
    assert!(!creds.access_key_id().is_empty());
    assert!(!creds.secret_access_key().is_empty());
    assert!(!creds.session_token().is_empty());
}

// ============================================================================
// Tests derived from AWS documentation: AWS Security Token Service (STS)
// ============================================================================

// --- AssumeRole error cases ---

#[tokio::test]
async fn test_assume_role_missing_role_arn() {
    let client = make_sts_client().await;

    let err = client
        .assume_role()
        .role_session_name("some-session")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing RoleArn, got: {err_str}"
    );
}

#[tokio::test]
async fn test_assume_role_missing_session_name() {
    let client = make_sts_client().await;

    let err = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/SomeRole")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing RoleSessionName, got: {err_str}"
    );
}

#[tokio::test]
async fn test_assume_role_expiration_in_future() {
    let client = make_sts_client().await;

    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/ExpiryRole")
        .role_session_name("expiry-session")
        .duration_seconds(3600)
        .send()
        .await
        .expect("assume_role should succeed");

    let creds = resp.credentials().expect("should have credentials");
    let expiration = creds.expiration();
    // expiration should be at least 'now' (i.e., in the future relative to before)
    let expiry_secs = expiration.secs();
    assert!(
        expiry_secs > before as i64,
        "Expiration should be in the future, before={before}, expiry_secs={expiry_secs}"
    );
}

#[tokio::test]
async fn test_assume_role_cross_account() {
    let client = make_sts_client().await;

    // Role ARN from account 999999999999 (different from default 123456789012)
    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::999999999999:role/CrossAccountRole")
        .role_session_name("cross-account-session")
        .duration_seconds(900)
        .send()
        .await
        .expect("cross-account assume_role should succeed");

    let user = resp
        .assumed_role_user()
        .expect("should have assumed_role_user");
    assert!(
        user.arn().contains("999999999999"),
        "ARN should contain the target account 999999999999, got: {}",
        user.arn()
    );
    assert!(
        user.arn().contains(
            "arn:aws:sts::999999999999:assumed-role/CrossAccountRole/cross-account-session"
        ),
        "ARN should follow assumed-role format with correct account, got: {}",
        user.arn()
    );
}

#[tokio::test]
async fn test_assume_role_assumed_role_id_format() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/IdFormatRole")
        .role_session_name("id-format-session")
        .duration_seconds(900)
        .send()
        .await
        .expect("assume_role should succeed");

    let user = resp
        .assumed_role_user()
        .expect("should have assumed_role_user");
    // assumed_role_id format: AROAXXX...XXX:session-name
    let assumed_role_id = user.assumed_role_id();
    assert!(
        assumed_role_id.starts_with("AROA"),
        "assumed_role_id should start with AROA, got: {assumed_role_id}"
    );
    assert!(
        assumed_role_id.contains("id-format-session"),
        "assumed_role_id should contain session name, got: {assumed_role_id}"
    );
}

// --- AssumeRoleWithWebIdentity error cases ---

#[tokio::test]
async fn test_assume_role_with_web_identity_missing_role_arn() {
    let client = make_sts_client().await;

    let err = client
        .assume_role_with_web_identity()
        .role_session_name("web-session")
        .web_identity_token("mock-token")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing RoleArn, got: {err_str}"
    );
}

#[tokio::test]
async fn test_assume_role_with_web_identity_missing_session_name() {
    let client = make_sts_client().await;

    let err = client
        .assume_role_with_web_identity()
        .role_arn("arn:aws:iam::123456789012:role/WebRole")
        .web_identity_token("mock-token")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing RoleSessionName, got: {err_str}"
    );
}

// --- AssumeRoleWithSAML error cases ---

#[tokio::test]
async fn test_assume_role_with_saml_missing_role_arn() {
    let client = make_sts_client().await;

    let err = client
        .assume_role_with_saml()
        .principal_arn("arn:aws:iam::123456789012:saml-provider/MyProvider")
        .saml_assertion("bW9jaw==")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing RoleArn, got: {err_str}"
    );
}

#[tokio::test]
async fn test_assume_role_with_saml_missing_principal_arn() {
    let client = make_sts_client().await;

    let err = client
        .assume_role_with_saml()
        .role_arn("arn:aws:iam::123456789012:role/SamlRole")
        .saml_assertion("bW9jaw==")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing PrincipalArn, got: {err_str}"
    );
}

// --- GetFederationToken error cases ---

#[tokio::test]
async fn test_get_federation_token_missing_name() {
    let client = make_sts_client().await;

    let err = client.get_federation_token().send().await.unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing Name, got: {err_str}"
    );
}

// --- GetSessionToken specific behavior ---

#[tokio::test]
async fn test_get_session_token_fixed_access_key() {
    let client = make_sts_client().await;

    let resp = client
        .get_session_token()
        .duration_seconds(3600)
        .send()
        .await
        .expect("get_session_token should succeed");

    let creds = resp.credentials().expect("should have credentials");
    // winterbaume uses a fixed access_key_id for GetSessionToken
    assert_eq!(
        creds.access_key_id(),
        "ASIAIOSTOKENEXAMPLE",
        "GetSessionToken should return fixed access_key_id"
    );
}

// --- GetFederationToken credential format ---

#[tokio::test]
async fn test_get_federation_token_credentials_format() {
    let client = make_sts_client().await;

    let resp = client
        .get_federation_token()
        .name("alice")
        .send()
        .await
        .expect("get_federation_token should succeed");

    let creds = resp.credentials().expect("should have credentials");
    assert!(
        creds.access_key_id().starts_with("ASIA"),
        "GetFederationToken access_key_id should start with ASIA, got: {}",
        creds.access_key_id()
    );
    assert_eq!(
        creds.access_key_id().len(),
        20,
        "access_key_id should be 20 chars"
    );
    assert_eq!(
        creds.secret_access_key().len(),
        40,
        "secret_access_key should be 40 chars"
    );
}

// ============================================================================
// Tests for previously untested operations
// ============================================================================

// --- AssumeRoot ---

#[tokio::test]
async fn test_assume_root_missing_target_principal() {
    let client = make_sts_client().await;

    let err = client
        .assume_root()
        .task_policy_arn(
            aws_sdk_sts::types::PolicyDescriptorType::builder()
                .arn("arn:aws:iam::aws:policy/root-task/IAMAuditRootUserCredentials")
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing TargetPrincipal, got: {err_str}"
    );
}

#[tokio::test]
async fn test_assume_root_missing_task_policy_arn() {
    let client = make_sts_client().await;

    let err = client
        .assume_root()
        .target_principal("arn:aws:organizations::123456789012:account/o-example/123456789012")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing TaskPolicyArn, got: {err_str}"
    );
}

// --- DecodeAuthorizationMessage ---

#[tokio::test]
async fn test_decode_authorization_message() {
    let client = make_sts_client().await;

    let encoded = "eyJhbGxvd2VkIjp0cnVlfQ==";
    let resp = client
        .decode_authorization_message()
        .encoded_message(encoded)
        .send()
        .await
        .expect("decode_authorization_message should succeed");

    // The mock returns the encoded message as-is for the decoded message
    let decoded = resp.decoded_message().expect("should have decoded_message");
    assert_eq!(
        decoded, encoded,
        "mock should return the encoded message as decoded"
    );
}

#[tokio::test]
async fn test_decode_authorization_message_missing_encoded_message() {
    let client = make_sts_client().await;

    let err = client
        .decode_authorization_message()
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing EncodedMessage, got: {err_str}"
    );
}

// --- GetAccessKeyInfo ---

#[tokio::test]
async fn test_get_access_key_info() {
    let client = make_sts_client().await;

    let resp = client
        .get_access_key_info()
        .access_key_id("AKIAIOSFODNN7EXAMPLE")
        .send()
        .await
        .expect("get_access_key_info should succeed");

    assert_eq!(
        resp.account(),
        Some("123456789012"),
        "account should be the default account ID"
    );
}

#[tokio::test]
async fn test_get_access_key_info_missing_access_key_id() {
    let client = make_sts_client().await;

    let err = client.get_access_key_info().send().await.unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MissingParameter") || err_str.contains("missing"),
        "Expected MissingParameter error for missing AccessKeyId, got: {err_str}"
    );
}

// --- GetWebIdentityToken ---

#[tokio::test]
async fn test_get_web_identity_token() {
    let client = make_sts_client().await;

    let resp = client
        .get_web_identity_token()
        .duration_seconds(3600)
        .send()
        .await
        .expect("get_web_identity_token should succeed");

    let token = resp
        .web_identity_token()
        .expect("should have web_identity_token");
    assert!(!token.is_empty(), "web_identity_token should be non-empty");
    assert!(
        token.starts_with("FQoGZXIvYXdzE"),
        "web_identity_token should start with expected prefix"
    );

    // Should have an expiration
    assert!(resp.expiration().is_some(), "should have expiration");
}

use aws_credential_types::Credentials;
use aws_sdk_sts as sts;
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;

async fn make_sts_client() -> sts::Client {
    let mock = MockAws::builder().with_service(StsService::new()).build();

    let creds = Credentials::new(
        "AKIAIOSFODNN7EXAMPLE",
        "wJalrXUtnFEMI/K7MDENG/bPxRfiCYzEXAMPLEKEY",
        None,
        None,
        "winterbaume-test",
    );

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(creds)
        .region(aws_config::Region::new("us-east-1"))
        .load()
        .await;

    sts::Client::new(&config)
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
    assert!(resp.arn().is_some(), "ARN should be present");
    assert!(resp.user_id().is_some(), "UserId should be present");
}

#[tokio::test]
async fn test_assume_role() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/TestRole")
        .role_session_name("test-session")
        .send()
        .await
        .expect("assume_role should succeed");

    let credentials = resp.credentials().expect("credentials should be present");
    assert!(
        credentials.access_key_id().starts_with("ASIA"),
        "Access key should start with ASIA for temporary credentials"
    );
    assert!(!credentials.secret_access_key().is_empty());
    assert!(!credentials.session_token().is_empty());
    // expiration() returns DateTime directly, just verify credentials exist

    let assumed_role_user = resp
        .assumed_role_user()
        .expect("assumed_role_user should be present");
    assert!(assumed_role_user.arn().contains("assumed-role"));
    assert!(assumed_role_user.arn().contains("TestRole"));
    assert!(assumed_role_user.arn().contains("test-session"));
    assert!(
        assumed_role_user.assumed_role_id().contains("test-session"),
        "AssumedRoleId should contain session name"
    );
}

#[tokio::test]
async fn test_get_session_token() {
    let client = make_sts_client().await;

    let resp = client
        .get_session_token()
        .send()
        .await
        .expect("get_session_token should succeed");

    let credentials = resp.credentials().expect("credentials should be present");
    assert!(!credentials.access_key_id().is_empty());
    assert!(!credentials.secret_access_key().is_empty());
    assert!(!credentials.session_token().is_empty());
}

#[tokio::test]
async fn test_assume_role_with_duration() {
    let client = make_sts_client().await;

    let resp = client
        .assume_role()
        .role_arn("arn:aws:iam::123456789012:role/TestRole")
        .role_session_name("duration-test")
        .duration_seconds(7200)
        .send()
        .await
        .expect("assume_role with custom duration should succeed");

    assert!(resp.credentials().is_some());
}

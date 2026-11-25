//! Tests for the STS relaxations ( `winterbaume_sts::RELAXATIONS` ) and for
//! `GetCallerIdentity` resolving the caller from the request's access key.
//!
//! GitHub issue #77: MinIO's `AssumeRole` accepts a request without `RoleArn`
//! and issues credentials for the caller; AWS answers `MissingParameter`.

use std::sync::Arc;

use aws_sdk_sts::config::{BehaviorVersion, Credentials, Region};
use aws_sdk_sts::error::ProvideErrorMetadata;
use winterbaume_core::{MockAws, Relaxations, StatefulService};
use winterbaume_sts::{ASSUME_ROLE_WITHOUT_ROLE_ARN, StsService};

const ROLE_ARN: &str = "arn:aws:iam::123456789012:role/test-role";

fn relaxed_service() -> StsService {
    StsService::new().with_relaxations(&Relaxations::new().with(ASSUME_ROLE_WITHOUT_ROLE_ARN))
}

/// A client for `mock` that signs with the `MockAws` default credentials.
async fn client(mock: &MockAws) -> aws_sdk_sts::Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_sts::Client::new(&config)
}

/// A client for `mock` that signs with the given temporary credentials.
async fn client_with(
    mock: &MockAws,
    creds: &aws_sdk_sts::types::Credentials,
) -> aws_sdk_sts::Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(Credentials::new(
            creds.access_key_id(),
            creds.secret_access_key(),
            Some(creds.session_token().to_string()),
            None,
            "relaxation-test",
        ))
        .region(Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_sts::Client::new(&config)
}

#[tokio::test]
async fn test_assume_role_without_role_arn_is_refused_by_default() {
    let mock = MockAws::builder().with_service(StsService::new()).build();
    let sts = client(&mock).await;

    let err = sts
        .assume_role()
        .role_session_name("s")
        .send()
        .await
        .expect_err("AWS refuses AssumeRole without RoleArn");
    let err = err.into_service_error();
    assert_eq!(err.code(), Some("MissingParameter"));
    assert_eq!(err.message(), Some("Missing 'RoleArn'"));
}

#[tokio::test]
async fn test_relaxations_default_to_none() {
    assert!(StsService::new().relaxations().is_empty());
    assert!(
        relaxed_service()
            .relaxations()
            .is_enabled(ASSUME_ROLE_WITHOUT_ROLE_ARN)
    );
    // A set without STS relaxations leaves STS strict.
    let other = winterbaume_core::Relaxation {
        name: "other-service-rule",
        summary: "Not an STS rule.",
    };
    assert!(
        StsService::new()
            .with_relaxations(&Relaxations::new().with(other))
            .relaxations()
            .is_empty()
    );
}

#[tokio::test]
async fn test_relaxed_assume_role_without_role_arn_issues_caller_credentials() {
    let mock = MockAws::builder().with_service(relaxed_service()).build();
    let sts = client(&mock).await;
    let caller = sts.get_caller_identity().send().await.unwrap();

    let out = sts
        .assume_role()
        .role_session_name("lakekeeper-sts")
        .duration_seconds(900)
        .send()
        .await
        .expect("relaxed AssumeRole without RoleArn should succeed");

    // No role is assumed, so there is no AssumedRoleUser.
    assert!(out.assumed_role_user().is_none());
    let creds = out.credentials().expect("credentials");
    assert!(creds.access_key_id().starts_with("ASIA"));
    assert_eq!(creds.access_key_id().len(), 20);
    assert_eq!(creds.secret_access_key().len(), 40);
    assert!(!creds.session_token().is_empty());
    let now = aws_sdk_sts::primitives::DateTime::from(std::time::SystemTime::now());
    let lifetime = creds.expiration().secs() - now.secs();
    assert!(
        (890..=900).contains(&lifetime),
        "expiration should honour DurationSeconds, got {lifetime}s"
    );

    // The credentials keep the caller's identity.
    let issued = client_with(&mock, creds).await;
    let identity = issued.get_caller_identity().send().await.unwrap();
    assert_eq!(identity.arn(), caller.arn());
    assert_eq!(identity.user_id(), caller.user_id());
    assert_eq!(identity.account(), caller.account());
}

#[tokio::test]
async fn test_relaxed_assume_role_without_role_arn_keeps_an_assumed_role_identity() {
    let mock = MockAws::builder().with_service(relaxed_service()).build();
    let sts = client(&mock).await;

    let role = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("outer")
        .send()
        .await
        .unwrap();
    let as_role = client_with(&mock, role.credentials().unwrap()).await;

    let out = as_role
        .assume_role()
        .role_session_name("inner")
        .send()
        .await
        .unwrap();
    let issued = client_with(&mock, out.credentials().unwrap()).await;
    let identity = issued.get_caller_identity().send().await.unwrap();
    assert_eq!(
        identity.arn(),
        Some("arn:aws:sts::123456789012:assumed-role/test-role/outer")
    );
    assert_eq!(identity.arn(), role.assumed_role_user().map(|u| u.arn()));
}

#[tokio::test]
async fn test_relaxed_assume_role_still_requires_role_session_name() {
    let mock = MockAws::builder().with_service(relaxed_service()).build();
    let sts = client(&mock).await;

    let err = sts
        .assume_role()
        .send()
        .await
        .expect_err("RoleSessionName is still required");
    let err = err.into_service_error();
    assert_eq!(err.code(), Some("MissingParameter"));
    assert_eq!(err.message(), Some("Missing 'RoleSessionName'"));
}

#[tokio::test]
async fn test_relaxed_assume_role_with_role_arn_is_unchanged() {
    let mock = MockAws::builder().with_service(relaxed_service()).build();
    let sts = client(&mock).await;

    let out = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("s")
        .send()
        .await
        .unwrap();
    let user = out.assumed_role_user().expect("assumed role user");
    assert_eq!(
        user.arn(),
        "arn:aws:sts::123456789012:assumed-role/test-role/s"
    );
}

#[tokio::test]
async fn test_get_caller_identity_with_assumed_role_credentials() {
    // GetCallerIdentity resolves the caller from the request's access key,
    // with or without relaxations.
    let mock = MockAws::builder().with_service(StsService::new()).build();
    let sts = client(&mock).await;

    let out = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("session-1")
        .send()
        .await
        .unwrap();
    let as_role = client_with(&mock, out.credentials().unwrap()).await;
    let identity = as_role.get_caller_identity().send().await.unwrap();
    assert_eq!(
        identity.arn(),
        Some("arn:aws:sts::123456789012:assumed-role/test-role/session-1")
    );
    assert_eq!(
        identity.user_id(),
        out.assumed_role_user().map(|u| u.assumed_role_id())
    );

    // Long-term credentials still get the default identity.
    let identity = sts.get_caller_identity().send().await.unwrap();
    assert_eq!(identity.arn(), Some("arn:aws:sts::123456789012:user/moto"));
}

#[tokio::test]
async fn test_caller_sessions_survive_snapshot_and_restore() {
    let source = Arc::new(relaxed_service());
    let mock = MockAws::builder().with_service(Arc::clone(&source)).build();
    let out = client(&mock)
        .await
        .assume_role()
        .role_session_name("s")
        .send()
        .await
        .unwrap();
    let creds = out.credentials().unwrap().clone();

    let view = source.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view.caller_sessions.len(), 1);
    assert_eq!(view.caller_sessions[0].access_key_id, creds.access_key_id());

    // Restore into a strict service: the relaxation governs new requests,
    // not credentials already issued.
    let target = Arc::new(StsService::new());
    target
        .restore("123456789012", "us-east-1", view)
        .await
        .unwrap();
    let mock = MockAws::builder().with_service(Arc::clone(&target)).build();
    let identity = client_with(&mock, &creds)
        .await
        .get_caller_identity()
        .send()
        .await
        .unwrap();
    assert_eq!(identity.arn(), Some("arn:aws:sts::123456789012:user/moto"));
}

#[test]
fn test_snapshot_without_caller_sessions_still_parses() {
    let view: winterbaume_sts::StsStateView =
        serde_json::from_str(r#"{"assumed_roles": []}"#).unwrap();
    assert!(view.caller_sessions.is_empty());
    // An empty list is not written, so older readers see the old shape.
    let json = serde_json::to_value(&view).unwrap();
    assert!(json.get("caller_sessions").is_none());
}

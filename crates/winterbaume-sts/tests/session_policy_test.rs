//! STS keeps the session policy ( `Policy` ) of the credentials it issues and
//! answers it by access key ( `StsService::session_policy` ), so that S3 can
//! enforce it ( issue #85 ).

use std::sync::Arc;

use aws_sdk_sts::config::{BehaviorVersion, Region};
use winterbaume_core::{MockAws, Relaxations, StatefulService};
use winterbaume_sts::{ASSUME_ROLE_WITHOUT_ROLE_ARN, StsService};

const ROLE_ARN: &str = "arn:aws:iam::123456789012:role/r";
const POLICY: &str = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:GetObject","Resource":"arn:aws:s3:::bkt/allowed/*"}]}"#;

async fn client(mock: &MockAws, region: &str) -> aws_sdk_sts::Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(Region::new(region.to_string()))
        .load()
        .await;
    aws_sdk_sts::Client::new(&config)
}

#[tokio::test]
async fn assume_role_keeps_the_session_policy() {
    let service = Arc::new(StsService::new());
    let mock = MockAws::builder()
        .with_service(Arc::clone(&service))
        .build();
    let sts = client(&mock, "us-east-1").await;

    let scoped = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("scoped")
        .policy(POLICY)
        .send()
        .await
        .unwrap();
    let plain = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("plain")
        .send()
        .await
        .unwrap();

    let scoped_key = scoped.credentials().unwrap().access_key_id();
    let plain_key = plain.credentials().unwrap().access_key_id();
    assert_eq!(
        service.session_policy(scoped_key).await,
        Some(Some(POLICY.to_string()))
    );
    assert_eq!(service.session_policy(plain_key).await, Some(None));
    assert_eq!(service.session_policy("AKIANOTISSUEDHERE").await, None);
}

/// Credentials are looked up across regions: S3 may be called in another
/// region than the STS endpoint that issued them.
#[tokio::test]
async fn lookup_spans_regions() {
    let service = Arc::new(StsService::new());
    let mock = MockAws::builder()
        .with_service(Arc::clone(&service))
        .build();
    let sts = client(&mock, "eu-west-1").await;
    let issued = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("s")
        .policy(POLICY)
        .send()
        .await
        .unwrap();
    let key = issued.credentials().unwrap().access_key_id();
    assert_eq!(
        service.session_policy(key).await,
        Some(Some(POLICY.to_string()))
    );
}

/// The relaxed `AssumeRole` without `RoleArn` ( MinIO's form ) keeps the
/// session policy too: MinIO applies it, and Lakekeeper relies on that.
#[tokio::test]
async fn caller_session_keeps_the_session_policy() {
    let service = Arc::new(
        StsService::new().with_relaxations(&Relaxations::new().with(ASSUME_ROLE_WITHOUT_ROLE_ARN)),
    );
    let mock = MockAws::builder()
        .with_service(Arc::clone(&service))
        .build();
    let sts = client(&mock, "us-east-1").await;
    let issued = sts
        .assume_role()
        .role_session_name("lakekeeper-sts")
        .policy(POLICY)
        .send()
        .await
        .unwrap();
    let key = issued.credentials().unwrap().access_key_id();
    assert_eq!(
        service.session_policy(key).await,
        Some(Some(POLICY.to_string()))
    );
}

#[tokio::test]
async fn web_identity_keeps_the_session_policy() {
    let service = Arc::new(StsService::new());
    let mock = MockAws::builder()
        .with_service(Arc::clone(&service))
        .build();
    let sts = client(&mock, "us-east-1").await;
    let issued = sts
        .assume_role_with_web_identity()
        .role_arn(ROLE_ARN)
        .role_session_name("w")
        .web_identity_token("token")
        .policy(POLICY)
        .send()
        .await
        .unwrap();
    let key = issued.credentials().unwrap().access_key_id();
    assert_eq!(
        service.session_policy(key).await,
        Some(Some(POLICY.to_string()))
    );
}

/// The policy survives a snapshot and restore; views written before it
/// existed still load.
#[tokio::test]
async fn session_policy_round_trips_through_the_state_view() {
    let service = Arc::new(
        StsService::new().with_relaxations(&Relaxations::new().with(ASSUME_ROLE_WITHOUT_ROLE_ARN)),
    );
    let mock = MockAws::builder()
        .with_service(Arc::clone(&service))
        .build();
    let sts = client(&mock, "us-east-1").await;
    let role_key = sts
        .assume_role()
        .role_arn(ROLE_ARN)
        .role_session_name("r")
        .policy(POLICY)
        .send()
        .await
        .unwrap()
        .credentials()
        .unwrap()
        .access_key_id()
        .to_string();
    let caller_key = sts
        .assume_role()
        .role_session_name("c")
        .policy(POLICY)
        .send()
        .await
        .unwrap()
        .credentials()
        .unwrap()
        .access_key_id()
        .to_string();

    let account = winterbaume_core::default_account_id();
    let view = service.snapshot(account, "us-east-1").await;
    let json = serde_json::to_value(&view).unwrap();
    assert_eq!(json["assumed_roles"][0]["session_policy"], POLICY);
    assert_eq!(json["caller_sessions"][0]["session_policy"], POLICY);

    let restored = StsService::new();
    restored
        .restore(account, "us-east-1", serde_json::from_value(json).unwrap())
        .await
        .unwrap();
    for key in [&role_key, &caller_key] {
        assert_eq!(
            restored.session_policy(key).await,
            Some(Some(POLICY.to_string()))
        );
    }

    // A view without `session_policy` ( written by an older build ) loads.
    let old = serde_json::json!({
        "assumed_roles": [{
            "account_id": account, "role_arn": ROLE_ARN, "role_session_name": "old",
            "access_key_id": "ASIAOLDVIEW", "secret_access_key": "s", "session_token": "t",
            "expiration": "2030-01-01T00:00:00Z", "assumed_role_id": "AROAOLD"
        }]
    });
    let old_service = StsService::new();
    old_service
        .restore(account, "us-east-1", serde_json::from_value(old).unwrap())
        .await
        .unwrap();
    assert_eq!(old_service.session_policy("ASIAOLDVIEW").await, Some(None));
}

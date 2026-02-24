use std::sync::{Arc, Mutex};

use aws_sdk_appconfigdata::config::BehaviorVersion;
use winterbaume_appconfigdata::AppConfigDataService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_appconfigdata::Client {
    let mock = MockAws::builder()
        .with_service(AppConfigDataService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appconfigdata::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_appconfigdata::Client::new(&config)
}

#[tokio::test]
async fn test_start_session_returns_token() {
    let client = make_client().await;
    let resp = client
        .start_configuration_session()
        .application_identifier("MyApp")
        .environment_identifier("Prod")
        .configuration_profile_identifier("MyProfile")
        .send()
        .await
        .expect("start_configuration_session");
    let token = resp.initial_configuration_token().expect("token");
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_start_session_with_min_poll() {
    let client = make_client().await;
    let resp = client
        .start_configuration_session()
        .application_identifier("MyApp")
        .environment_identifier("Prod")
        .configuration_profile_identifier("MyProfile")
        .required_minimum_poll_interval_in_seconds(15)
        .send()
        .await
        .expect("start_configuration_session");
    assert!(resp.initial_configuration_token().is_some());
}

#[tokio::test]
async fn test_get_latest_configuration_returns_empty_body() {
    let client = make_client().await;
    let session = client
        .start_configuration_session()
        .application_identifier("MyApp")
        .environment_identifier("Prod")
        .configuration_profile_identifier("MyProfile")
        .send()
        .await
        .expect("start session");
    let token = session.initial_configuration_token().unwrap();

    let resp = client
        .get_latest_configuration()
        .configuration_token(token)
        .send()
        .await
        .expect("get_latest_configuration");

    let next_token = resp.next_poll_configuration_token().expect("next token");
    assert!(!next_token.is_empty());
    assert_ne!(next_token, token);
    assert_eq!(resp.next_poll_interval_in_seconds(), 60);
    let bytes = resp
        .configuration
        .map(|b| b.into_inner())
        .unwrap_or_default();
    assert!(bytes.is_empty(), "configuration should be empty bytes");
}

#[tokio::test]
async fn test_get_latest_configuration_with_invalid_token() {
    let client = make_client().await;
    let err = client
        .get_latest_configuration()
        .configuration_token("not-a-real-token")
        .send()
        .await
        .expect_err("should fail");
    assert!(format!("{:?}", err).contains("BadRequestException"));
}

#[tokio::test]
async fn test_token_rotation_invalidates_previous() {
    let client = make_client().await;
    let session = client
        .start_configuration_session()
        .application_identifier("MyApp")
        .environment_identifier("Prod")
        .configuration_profile_identifier("MyProfile")
        .send()
        .await
        .expect("start session");
    let token1 = session.initial_configuration_token().unwrap().to_string();

    // First poll succeeds and yields a new token.
    let resp1 = client
        .get_latest_configuration()
        .configuration_token(&token1)
        .send()
        .await
        .expect("first poll");
    let token2 = resp1.next_poll_configuration_token().unwrap().to_string();

    // Re-using the original token now fails.
    let err = client
        .get_latest_configuration()
        .configuration_token(&token1)
        .send()
        .await
        .expect_err("re-use should fail");
    assert!(format!("{:?}", err).contains("BadRequestException"));

    // The rotated token works.
    client
        .get_latest_configuration()
        .configuration_token(&token2)
        .send()
        .await
        .expect("rotated token works");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AppConfigDataService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

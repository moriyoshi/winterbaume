use std::sync::{Arc, Mutex};

use aws_sdk_cloudtraildata::config::BehaviorVersion;
use aws_sdk_cloudtraildata::types::AuditEvent;
use winterbaume_cloudtraildata::CloudTrailDataService;
use winterbaume_core::{MockAws, StatefulService};

const CHANNEL_ARN: &str = "arn:aws:cloudtrail:us-east-1:123456789012:channel/test-channel";

async fn make_client() -> aws_sdk_cloudtraildata::Client {
    let mock = MockAws::builder()
        .with_service(CloudTrailDataService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtraildata::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_cloudtraildata::Client::new(&config)
}

#[tokio::test]
async fn test_put_audit_events_success() {
    let client = make_client().await;
    let resp = client
        .put_audit_events()
        .channel_arn(CHANNEL_ARN)
        .audit_events(
            AuditEvent::builder()
                .id("client-event-1")
                .event_data(r#"{"eventName":"TestEvent"}"#)
                .build()
                .unwrap(),
        )
        .audit_events(
            AuditEvent::builder()
                .id("client-event-2")
                .event_data(r#"{"eventName":"AnotherEvent"}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_audit_events");
    assert_eq!(resp.successful().len(), 2);
    assert_eq!(resp.failed().len(), 0);
    assert_eq!(resp.successful()[0].id(), "client-event-1");
    assert!(resp.successful()[0].event_id().starts_with("evt-"));
}

#[tokio::test]
async fn test_put_audit_events_missing_channel_arn() {
    let svc = CloudTrailDataService::new();
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtraildata::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_cloudtraildata::Client::new(&config);
    let err = client
        .put_audit_events()
        .audit_events(
            AuditEvent::builder()
                .id("c1")
                .event_data(r#"{}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("missing");
    // SDK marks channelArn @required so it surfaces as a serialization error before
    // hitting the mock; just assert that the call produced an error of any flavour.
    let msg = format!("{err:?}");
    assert!(msg.contains("channel_arn") || msg.contains("ValidationException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CloudTrailDataService::new();
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
    assert_eq!(events.lock().unwrap().len(), 1);
}

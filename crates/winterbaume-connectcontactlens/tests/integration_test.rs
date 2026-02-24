use std::sync::{Arc, Mutex};

use aws_sdk_connectcontactlens::config::BehaviorVersion;
use winterbaume_connectcontactlens::ConnectContactLensService;
use winterbaume_core::{MockAws, StatefulService};

const INSTANCE_ID: &str = "11111111-1111-1111-1111-111111111111";
const CONTACT_ID: &str = "22222222-2222-2222-2222-222222222222";

async fn make_client() -> aws_sdk_connectcontactlens::Client {
    let mock = MockAws::builder()
        .with_service(ConnectContactLensService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectcontactlens::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_connectcontactlens::Client::new(&config)
}

#[tokio::test]
async fn test_list_segments_empty() {
    let client = make_client().await;
    let resp = client
        .list_realtime_contact_analysis_segments()
        .instance_id(INSTANCE_ID)
        .contact_id(CONTACT_ID)
        .send()
        .await
        .expect("list");
    assert_eq!(resp.segments().len(), 0);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ConnectContactLensService::new();
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

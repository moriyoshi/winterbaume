use std::sync::{Arc, Mutex};

use aws_sdk_controlcatalog::config::BehaviorVersion;
use winterbaume_controlcatalog::ControlCatalogService;
use winterbaume_core::{MockAws, StatefulService};

const CONTROL_ARN: &str = "arn:aws:controlcatalog:::control/AWS-GR_IAM_USER_NO_POLICIES_CHECK";

async fn make_client() -> aws_sdk_controlcatalog::Client {
    let mock = MockAws::builder()
        .with_service(ControlCatalogService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_controlcatalog::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_controlcatalog::Client::new(&config)
}

#[tokio::test]
async fn test_list_seeded_controls() {
    let client = make_client().await;
    let resp = client.list_controls().send().await.expect("list");
    assert_eq!(resp.controls().len(), 1);
}

#[tokio::test]
async fn test_get_control() {
    let client = make_client().await;
    let resp = client
        .get_control()
        .control_arn(CONTROL_ARN)
        .send()
        .await
        .expect("get");
    assert_eq!(resp.arn(), CONTROL_ARN);
    assert_eq!(resp.behavior().as_str(), "DETECTIVE");
}

#[tokio::test]
async fn test_list_domains_objectives_common_controls() {
    let client = make_client().await;
    let domains = client.list_domains().send().await.expect("domains");
    assert_eq!(domains.domains().len(), 1);
    let objectives = client.list_objectives().send().await.expect("objectives");
    assert_eq!(objectives.objectives().len(), 1);
    let common = client.list_common_controls().send().await.expect("common");
    assert_eq!(common.common_controls().len(), 1);
}

#[tokio::test]
async fn test_get_unknown_control() {
    let client = make_client().await;
    let err = client
        .get_control()
        .control_arn("arn:aws:controlcatalog:::control/missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ControlCatalogService::new();
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

use std::sync::{Arc, Mutex};

use aws_sdk_freetier::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_freetier::FreeTierService;

async fn make_client() -> aws_sdk_freetier::Client {
    let mock = MockAws::builder()
        .with_service(FreeTierService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_freetier::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_freetier::Client::new(&config)
}

#[tokio::test]
async fn test_get_account_plan_state_default() {
    let client = make_client().await;
    let resp = client
        .get_account_plan_state()
        .send()
        .await
        .expect("get_account_plan_state");
    assert_eq!(resp.account_plan_type().as_str(), "FREE_TIER");
    assert_eq!(resp.account_plan_status().as_str(), "ACTIVE");
    let credits = resp.account_plan_remaining_credits().expect("credits");
    assert_eq!(credits.amount(), 100.0);
    assert_eq!(credits.unit().as_str(), "USD");
}

#[tokio::test]
async fn test_upgrade_plan() {
    let client = make_client().await;
    let resp = client
        .upgrade_account_plan()
        .account_plan_type("PAID".into())
        .send()
        .await
        .expect("upgrade");
    assert_eq!(resp.account_plan_type().as_str(), "PAID");

    let after = client.get_account_plan_state().send().await.expect("after");
    assert_eq!(after.account_plan_type().as_str(), "PAID");
}

#[tokio::test]
async fn test_list_activities_empty() {
    let client = make_client().await;
    let resp = client.list_account_activities().send().await.expect("list");
    assert_eq!(resp.activities().len(), 0);
}

#[tokio::test]
async fn test_get_activity_not_found() {
    let client = make_client().await;
    let err = client
        .get_account_activity()
        .activity_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_get_free_tier_usage_empty() {
    let client = make_client().await;
    let resp = client
        .get_free_tier_usage()
        .send()
        .await
        .expect("get_usage");
    assert_eq!(resp.free_tier_usages().len(), 0);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = FreeTierService::new();
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

use std::sync::{Arc, Mutex};

use aws_sdk_costoptimizationhub::config::BehaviorVersion;
use aws_sdk_costoptimizationhub::types::{
    EnrollmentStatus, MemberAccountDiscountVisibility, SavingsEstimationMode,
};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_costoptimizationhub::CostOptimizationHubService;

async fn make_client() -> aws_sdk_costoptimizationhub::Client {
    let mock = MockAws::builder()
        .with_service(CostOptimizationHubService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costoptimizationhub::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_costoptimizationhub::Client::new(&config)
}

#[tokio::test]
async fn test_get_preferences_default() {
    let client = make_client().await;
    let resp = client.get_preferences().send().await.expect("get");
    assert_eq!(
        resp.savings_estimation_mode().map(|s| s.as_str()),
        Some("BeforeDiscounts")
    );
}

#[tokio::test]
async fn test_update_preferences_round_trip() {
    let client = make_client().await;
    client
        .update_preferences()
        .savings_estimation_mode(SavingsEstimationMode::AfterDiscounts)
        .member_account_discount_visibility(MemberAccountDiscountVisibility::None)
        .send()
        .await
        .expect("update");
    let got = client.get_preferences().send().await.expect("get");
    assert_eq!(
        got.savings_estimation_mode().map(|s| s.as_str()),
        Some("AfterDiscounts")
    );
    assert_eq!(
        got.member_account_discount_visibility().map(|s| s.as_str()),
        Some("None")
    );
}

#[tokio::test]
async fn test_update_enrollment_then_list() {
    let client = make_client().await;
    let resp = client
        .update_enrollment_status()
        .status(EnrollmentStatus::Active)
        .send()
        .await
        .expect("update");
    assert_eq!(resp.status(), Some("Active"));
    let listed = client
        .list_enrollment_statuses()
        .send()
        .await
        .expect("list");
    assert_eq!(listed.items().len(), 1);
}

#[tokio::test]
async fn test_get_unknown_recommendation() {
    let client = make_client().await;
    let err = client
        .get_recommendation()
        .recommendation_id("ghost")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_recommendations_empty() {
    let client = make_client().await;
    let resp = client.list_recommendations().send().await.expect("list");
    assert_eq!(resp.items().len(), 0);
}

#[tokio::test]
async fn test_list_recommendation_summaries_default() {
    let client = make_client().await;
    let resp = client
        .list_recommendation_summaries()
        .group_by("ResourceType")
        .send()
        .await
        .expect("list");
    assert!(resp.items().is_empty());
    assert_eq!(resp.currency_code(), Some("USD"));
}

#[tokio::test]
async fn test_list_efficiency_metrics_empty() {
    let client = make_client().await;
    let resp = client.list_efficiency_metrics().send().await.expect("list");
    assert_eq!(resp.efficiency_metrics_by_group().len(), 0);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CostOptimizationHubService::new();
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

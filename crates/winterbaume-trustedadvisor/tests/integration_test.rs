use std::sync::{Arc, Mutex};

use aws_sdk_trustedadvisor::config::BehaviorVersion;
use serde_json::json;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_trustedadvisor::{TrustedAdvisorService, TrustedAdvisorStateView};

async fn make_client() -> aws_sdk_trustedadvisor::Client {
    let mock = MockAws::builder()
        .with_service(TrustedAdvisorService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_trustedadvisor::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_trustedadvisor::Client::new(&config)
}

#[tokio::test]
async fn test_list_checks_empty() {
    let client = make_client().await;
    let resp = client.list_checks().send().await.expect("list_checks");
    assert_eq!(resp.check_summaries().len(), 0);
}

#[tokio::test]
async fn test_recommendation_round_trip_via_state() {
    let svc = TrustedAdvisorService::new();
    let mut view = TrustedAdvisorStateView::default();
    view.recommendations.insert(
        "rec-1".to_string(),
        json!({
            "id": "rec-1",
            "arn": "arn:aws:trustedadvisor::123:recommendation/rec-1",
            "name": "Idle EC2 instances",
            "lifecycleStage": "in_progress",
            "status": "warning",
            "type": "standard",
        }),
    );
    svc.restore(winterbaume_core::DEFAULT_ACCOUNT_ID, "us-east-1", view)
        .await
        .expect("restore");
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_trustedadvisor::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_trustedadvisor::Client::new(&config);

    let list = client
        .list_recommendations()
        .send()
        .await
        .expect("list_recommendations");
    assert_eq!(list.recommendation_summaries().len(), 1);

    let got = client
        .get_recommendation()
        .recommendation_identifier("rec-1")
        .send()
        .await
        .expect("get_recommendation");
    assert!(got.recommendation().is_some());

    client
        .update_recommendation_lifecycle()
        .recommendation_identifier("rec-1")
        .lifecycle_stage("resolved".into())
        .update_reason_code("low_priority".into())
        .send()
        .await
        .expect("update");
}

#[tokio::test]
async fn test_get_recommendation_not_found() {
    let client = make_client().await;
    let err = client
        .get_recommendation()
        .recommendation_identifier("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_batch_update_resource_exclusion() {
    let client = make_client().await;
    let resp = client
        .batch_update_recommendation_resource_exclusion()
        .recommendation_resource_exclusions(
            aws_sdk_trustedadvisor::types::RecommendationResourceExclusion::builder()
                .arn("arn:aws:ec2:us-east-1:123:instance/i-abc")
                .is_excluded(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch update");
    assert_eq!(
        resp.batch_update_recommendation_resource_exclusion_errors()
            .len(),
        0
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = TrustedAdvisorService::new();
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

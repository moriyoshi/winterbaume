use std::sync::{Arc, Mutex};

use aws_sdk_bcmrecommendedactions::config::BehaviorVersion;
use winterbaume_bcmrecommendedactions::BcmRecommendedActionsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_bcmrecommendedactions::Client {
    let mock = MockAws::builder()
        .with_service(BcmRecommendedActionsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmrecommendedactions::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_bcmrecommendedactions::Client::new(&config)
}

#[tokio::test]
async fn test_list_recommended_actions_empty() {
    let client = make_client().await;
    let resp = client
        .list_recommended_actions()
        .send()
        .await
        .expect("list");
    assert!(resp.recommended_actions().is_empty());
}

#[tokio::test]
async fn test_state_view_round_trip() {
    use winterbaume_bcmrecommendedactions::views::{
        BcmRecommendedActionsStateView, RecommendedActionView,
    };

    let svc = BcmRecommendedActionsService::new();
    svc.restore(
        "123456789012",
        "us-east-1",
        BcmRecommendedActionsStateView {
            recommended_actions: vec![RecommendedActionView {
                id: Some("rec-1".to_string()),
                action_type: Some("ResourceManagement".to_string()),
                severity: Some("HIGH".to_string()),
                feature: Some("Budgets".to_string()),
                ..Default::default()
            }],
        },
    )
    .await
    .expect("seed");

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.recommended_actions.len(), 1);
    assert_eq!(snap.recommended_actions[0].id, Some("rec-1".to_string()));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BcmRecommendedActionsService::new();
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

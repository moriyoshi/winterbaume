use std::sync::{Arc, Mutex};

use aws_sdk_personalizeruntime::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_personalizeruntime::PersonalizeRuntimeService;

async fn make_client() -> aws_sdk_personalizeruntime::Client {
    let mock = MockAws::builder()
        .with_service(PersonalizeRuntimeService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeruntime::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_personalizeruntime::Client::new(&config)
}

#[tokio::test]
async fn test_get_recommendations_for_user() {
    let client = make_client().await;
    let resp = client
        .get_recommendations()
        .campaign_arn("arn:aws:personalize:us-east-1:123:campaign/c1")
        .user_id("alice")
        .num_results(5)
        .send()
        .await
        .expect("get_recommendations");
    let items = resp.item_list();
    assert_eq!(items.len(), 5);
    let scores: Vec<f64> = items.iter().filter_map(|i| i.score()).collect();
    for w in scores.windows(2) {
        assert!(w[0] >= w[1], "scores must be descending: {scores:?}");
    }
    assert!(resp.recommendation_id().unwrap().starts_with("rec-"));
}

#[tokio::test]
async fn test_get_recommendations_missing_arn() {
    let client = make_client().await;
    let err = client
        .get_recommendations()
        .user_id("alice")
        .send()
        .await
        .expect_err("missing arn");
    assert!(format!("{err:?}").contains("InvalidInputException"));
}

#[tokio::test]
async fn test_get_personalized_ranking() {
    let client = make_client().await;
    let resp = client
        .get_personalized_ranking()
        .campaign_arn("arn:aws:personalize:us-east-1:123:campaign/c1")
        .user_id("alice")
        .input_list("item-A")
        .input_list("item-B")
        .input_list("item-C")
        .send()
        .await
        .expect("rank");
    let ranked = resp.personalized_ranking();
    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0].item_id(), Some("item-A"));
}

#[tokio::test]
async fn test_get_action_recommendations() {
    let client = make_client().await;
    let resp = client
        .get_action_recommendations()
        .campaign_arn("arn:aws:personalize:us-east-1:123:campaign/c1")
        .user_id("alice")
        .num_results(3)
        .send()
        .await
        .expect("actions");
    assert_eq!(resp.action_list().len(), 3);
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = PersonalizeRuntimeService::new();
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

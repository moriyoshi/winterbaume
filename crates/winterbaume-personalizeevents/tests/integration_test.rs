use std::sync::{Arc, Mutex};

use aws_sdk_personalizeevents::config::BehaviorVersion;
use aws_sdk_personalizeevents::types::{Event, Item, User};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_personalizeevents::PersonalizeEventsService;

async fn make_client() -> aws_sdk_personalizeevents::Client {
    let mock = MockAws::builder()
        .with_service(PersonalizeEventsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeevents::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_personalizeevents::Client::new(&config)
}

#[tokio::test]
async fn test_put_events() {
    let client = make_client().await;
    let event = Event::builder()
        .event_type("click")
        .sent_at(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .build()
        .expect("event");
    client
        .put_events()
        .tracking_id("track-1")
        .session_id("session-1")
        .event_list(event)
        .send()
        .await
        .expect("put_events");
}

#[tokio::test]
async fn test_put_events_missing_tracking_id() {
    let svc = PersonalizeEventsService::new();
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeevents::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_personalizeevents::Client::new(&config);
    let event = Event::builder()
        .event_type("click")
        .sent_at(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .build()
        .expect("event");
    let err = client
        .put_events()
        .tracking_id("")
        .session_id("session-1")
        .event_list(event)
        .send()
        .await
        .expect_err("missing tracking id");
    assert!(format!("{err:?}").contains("ValidationException"));
}

#[tokio::test]
async fn test_put_items_and_users() {
    let client = make_client().await;
    let item = Item::builder().item_id("item-1").build().expect("item");
    client
        .put_items()
        .dataset_arn("arn:aws:personalize:us-east-1:123:dataset/items")
        .items(item)
        .send()
        .await
        .expect("put_items");

    let user = User::builder().user_id("user-1").build().expect("user");
    client
        .put_users()
        .dataset_arn("arn:aws:personalize:us-east-1:123:dataset/users")
        .users(user)
        .send()
        .await
        .expect("put_users");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = PersonalizeEventsService::new();
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

#[tokio::test]
async fn test_state_round_trip() {
    let svc = PersonalizeEventsService::new();
    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(view.events.is_empty());
}

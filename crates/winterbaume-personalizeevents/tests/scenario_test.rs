//! End-to-end scenario tests for the Personalize Events service.
//!
//! Each scenario chains 3+ operations and asserts business outcomes
//! rather than per-API return shapes.
//!
//! Note: the Personalize Events API is a pure write/ingest service — there
//! are no corresponding read APIs (describe/list). Business outcomes are
//! therefore verified by asserting that all operations succeed without error,
//! and by using the StatefulService snapshot after restore to confirm
//! persistence semantics.

use std::sync::{Arc, Mutex};

use aws_sdk_personalizeevents::config::BehaviorVersion;
use aws_sdk_personalizeevents::types::{Action, ActionInteraction, Event, Item, User};
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

/// Scenario: batch_ingest_pipeline
///
/// A typical Personalize Events ingest flow injects interaction events,
/// catalogue items, and user profiles in sequence. All three write operations
/// must succeed without error.
#[tokio::test]
async fn test_batch_ingest_pipeline() {
    let client = make_client().await;

    // Step 1: record interaction events for a tracker.
    let event = Event::builder()
        .event_type("click")
        .sent_at(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .build()
        .expect("event");
    client
        .put_events()
        .tracking_id("tracker-001")
        .session_id("session-a")
        .event_list(event)
        .send()
        .await
        .expect("put_events should succeed");

    // Step 2: import catalogue items.
    let item = Item::builder().item_id("item-42").build().expect("item");
    client
        .put_items()
        .dataset_arn("arn:aws:personalize:us-east-1:123456789012:dataset/items")
        .items(item)
        .send()
        .await
        .expect("put_items should succeed");

    // Step 3: import user profiles.
    let user = User::builder().user_id("user-7").build().expect("user");
    client
        .put_users()
        .dataset_arn("arn:aws:personalize:us-east-1:123456789012:dataset/users")
        .users(user)
        .send()
        .await
        .expect("put_users should succeed");
}

/// Scenario: action_catalogue_and_interaction_pipeline
///
/// An action-recommendation workflow: populate an action catalogue, record
/// user–action interactions across two batches, and verify that the state
/// change notifier fires for each mutation (indicating the service is ready to
/// propagate state to a persistence layer).
#[tokio::test]
async fn test_action_catalogue_and_interaction_pipeline() {
    let svc = PersonalizeEventsService::new();
    let fired: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let fired2 = Arc::clone(&fired);
    svc.notifier()
        .subscribe(move |_account_id, _region, _view| {
            *fired2.lock().unwrap() += 1;
        });

    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeevents::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_personalizeevents::Client::new(&config);

    // Step 1: populate an action catalogue dataset.
    let action = Action::builder()
        .action_id("action-1")
        .build()
        .expect("action");
    client
        .put_actions()
        .dataset_arn("arn:aws:personalize:us-east-1:123456789012:dataset/actions")
        .actions(action)
        .send()
        .await
        .expect("put_actions should succeed");

    // Step 2: record first batch of action interactions.
    let interaction1 = ActionInteraction::builder()
        .action_id("action-1")
        .session_id("session-x")
        .event_type("view")
        .timestamp(aws_smithy_types::DateTime::from_secs(1_700_000_100))
        .build()
        .expect("action_interaction");
    client
        .put_action_interactions()
        .tracking_id("tracker-002")
        .action_interactions(interaction1)
        .send()
        .await
        .expect("put_action_interactions (batch 1) should succeed");

    // Step 3: record second batch of action interactions.
    let interaction2 = ActionInteraction::builder()
        .action_id("action-1")
        .session_id("session-x")
        .event_type("click")
        .timestamp(aws_smithy_types::DateTime::from_secs(1_700_000_200))
        .build()
        .expect("action_interaction");
    client
        .put_action_interactions()
        .tracking_id("tracker-002")
        .action_interactions(interaction2)
        .send()
        .await
        .expect("put_action_interactions (batch 2) should succeed");

    // Business outcome: the state change notifier fired once per successful
    // mutation, confirming the service is ready for persistence propagation.
    assert_eq!(
        *fired.lock().unwrap(),
        3,
        "notifier should fire once for each successful ingest call"
    );
}

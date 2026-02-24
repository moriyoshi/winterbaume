//! Example: Personalize Events
//!
//! Demonstrates using aws-sdk-personalizeevents with winterbaume.
//!
//! Run with:
//!   cargo run --example personalizeevents --package winterbaume

use aws_sdk_personalizeevents::config::BehaviorVersion;
use aws_sdk_personalizeevents::types::Event;
use winterbaume_core::MockAws;
use winterbaume_personalizeevents::PersonalizeEventsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PersonalizeEventsService::new())
        .build();

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

    client
        .put_events()
        .tracking_id("track-1")
        .session_id("session-1")
        .event_list(event)
        .send()
        .await
        .expect("put_events should succeed");

    println!("Recorded an event for session session-1");
}

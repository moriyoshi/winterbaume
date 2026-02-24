//! Example: EventBridge
//!
//! Demonstrates using aws-sdk-eventbridge with winterbaume.
//!
//! Run with:
//!   cargo run --example eventbridge --package winterbaume-examples

use aws_sdk_eventbridge::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_eventbridge::EventBridgeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(EventBridgeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_eventbridge::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_eventbridge::Client::new(&config);

    let resp = client
        .list_rules()
        .send()
        .await
        .expect("list_rules should succeed");
    println!("EventBridge rules: {}", resp.rules().len());
}

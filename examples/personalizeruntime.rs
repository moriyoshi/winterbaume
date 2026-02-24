//! Example: Personalize Runtime
//!
//! Demonstrates using aws-sdk-personalizeruntime with winterbaume.
//!
//! Run with:
//!   cargo run --example personalizeruntime --package winterbaume

use aws_sdk_personalizeruntime::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_personalizeruntime::PersonalizeRuntimeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PersonalizeRuntimeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeruntime::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_personalizeruntime::Client::new(&config);
    let resp = client
        .get_recommendations()
        .campaign_arn("arn:aws:personalize:us-east-1:123:campaign/example")
        .user_id("alice")
        .num_results(3)
        .send()
        .await
        .expect("get_recommendations should succeed");

    for item in resp.item_list() {
        println!(
            "Item: {} (score: {})",
            item.item_id().unwrap_or_default(),
            item.score().unwrap_or(0.0)
        );
    }
}

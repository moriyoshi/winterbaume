//! Example: Personalize
//!
//! Demonstrates using aws-sdk-personalize with winterbaume.
//!
//! Run with:
//!   cargo run --example personalize --package winterbaume-examples

use aws_sdk_personalize::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_personalize::PersonalizeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PersonalizeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalize::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_personalize::Client::new(&config);

    let resp = client
        .list_datasets()
        .send()
        .await
        .expect("list_datasets should succeed");
    println!("Personalize datasets: {}", resp.datasets().len());
}

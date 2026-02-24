//! Example: Synthetics
//!
//! Demonstrates using aws-sdk-synthetics with winterbaume.
//!
//! Run with:
//!   cargo run --example synthetics --package winterbaume-examples

use aws_sdk_synthetics::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_synthetics::SyntheticsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SyntheticsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_synthetics::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_synthetics::Client::new(&config);

    let resp = client
        .describe_canaries()
        .send()
        .await
        .expect("describe_canaries should succeed");
    println!("Synthetics canaries: {}", resp.canaries().len());
}

//! Example: Support
//!
//! Demonstrates using aws-sdk-support with winterbaume.
//!
//! Run with:
//!   cargo run --example support --package winterbaume-examples

use aws_sdk_support::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_support::SupportService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SupportService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_support::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_support::Client::new(&config);

    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");
    println!("Support services: {}", resp.services().len());
}

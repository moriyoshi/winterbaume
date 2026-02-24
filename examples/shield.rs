//! Example: Shield
//!
//! Demonstrates using aws-sdk-shield with winterbaume.
//!
//! Run with:
//!   cargo run --example shield --package winterbaume-examples

use aws_sdk_shield::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_shield::ShieldService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ShieldService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_shield::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_shield::Client::new(&config);

    let resp = client
        .list_protections()
        .send()
        .await
        .expect("list_protections should succeed");
    println!("Shield protections: {}", resp.protections().len());
}

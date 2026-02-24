//! Example: RAM (Resource Access Manager)
//!
//! Demonstrates using aws-sdk-ram with winterbaume.
//!
//! Run with:
//!   cargo run --example ram --package winterbaume-examples

use aws_sdk_ram::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ram::RamService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(RamService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ram::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ram::Client::new(&config);

    use aws_sdk_ram::types::ResourceOwner;
    let resp = client
        .list_resources()
        .resource_owner(ResourceOwner::SelfValue)
        .send()
        .await
        .expect("list_resources should succeed");
    println!("RAM resources: {}", resp.resources().len());
}

//! Example: Greengrass
//!
//! Demonstrates using aws-sdk-greengrass with winterbaume.
//!
//! Run with:
//!   cargo run --example greengrass --package winterbaume-examples

use aws_sdk_greengrass::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_greengrass::GreengrassService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(GreengrassService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_greengrass::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_greengrass::Client::new(&config);

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");
    println!("Greengrass groups: {}", resp.groups().len());
}

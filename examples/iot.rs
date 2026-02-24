//! Example: IoT
//!
//! Demonstrates using aws-sdk-iot with winterbaume.
//!
//! Run with:
//!   cargo run --example iot --package winterbaume-examples

use aws_sdk_iot::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_iot::IotService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(IotService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iot::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_iot::Client::new(&config);

    let resp = client
        .list_things()
        .send()
        .await
        .expect("list_things should succeed");
    println!("IoT things: {}", resp.things().len());
}

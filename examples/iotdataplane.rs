//! Example: IoT Data Plane
//!
//! Demonstrates using aws-sdk-iotdataplane with winterbaume.
//!
//! Run with:
//!   cargo run --example iotdataplane --package winterbaume-examples

use aws_sdk_iotdataplane::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_iotdataplane::IotDataPlaneService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(IotDataPlaneService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iotdataplane::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_iotdataplane::Client::new(&config);

    let resp = client
        .list_retained_messages()
        .send()
        .await
        .expect("list_retained_messages should succeed");
    println!("Retained messages: {}", resp.retained_topics().len());
}

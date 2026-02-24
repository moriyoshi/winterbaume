//! Example: Kinesis
//!
//! Demonstrates using aws-sdk-kinesis with winterbaume.
//!
//! Run with:
//!   cargo run --example kinesis --package winterbaume-examples

use aws_sdk_kinesis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesis::KinesisService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesis::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kinesis::Client::new(&config);

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");
    println!("Kinesis streams: {}", resp.stream_names().len());
}

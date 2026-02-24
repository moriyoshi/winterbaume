//! Example: Firehose
//!
//! Demonstrates using aws-sdk-firehose with winterbaume.
//!
//! Run with:
//!   cargo run --example firehose --package winterbaume-examples

use aws_sdk_firehose::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_firehose::FirehoseService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(FirehoseService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_firehose::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_firehose::Client::new(&config);

    let resp = client
        .list_delivery_streams()
        .send()
        .await
        .expect("list_delivery_streams should succeed");
    println!(
        "Firehose delivery streams: {}",
        resp.delivery_stream_names().len()
    );
}

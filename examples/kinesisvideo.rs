//! Example: Kinesis Video
//!
//! Demonstrates using aws-sdk-kinesisvideo with winterbaume.
//!
//! Run with:
//!   cargo run --example kinesisvideo --package winterbaume-examples

use aws_sdk_kinesisvideo::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisvideo::KinesisVideoService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisVideoService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisvideo::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kinesisvideo::Client::new(&config);

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");
    println!("Kinesis Video streams: {}", resp.stream_info_list().len());
}

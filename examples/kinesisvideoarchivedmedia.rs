//! Example: Kinesis Video Archived Media
//!
//! Demonstrates using aws-sdk-kinesisvideoarchivedmedia with winterbaume.
//!
//! Run with:
//!   cargo run --example kinesisvideoarchivedmedia --package winterbaume-examples

use aws_sdk_kinesisvideoarchivedmedia::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisvideoarchivedmedia::KinesisVideoArchivedMediaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisVideoArchivedMediaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisvideoarchivedmedia::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_kinesisvideoarchivedmedia::Client::new(&config);

    // Kinesis Video Archived Media requires a stream ARN or name.
    // This example demonstrates client setup for the KinesisVideoArchivedMedia service.
    println!(
        "KinesisVideoArchivedMedia client ready. Use get_hls_streaming_session_url() to stream."
    );
    let _client = client;
}

//! Example: MediaLive
//!
//! Demonstrates using aws-sdk-medialive with winterbaume.
//!
//! Run with:
//!   cargo run --example medialive --package winterbaume-examples

use aws_sdk_medialive::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_medialive::MediaLiveService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaLiveService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_medialive::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_medialive::Client::new(&config);

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    println!("MediaLive channels: {}", resp.channels().len());
}

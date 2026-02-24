//! Example: MediaPackage v2
//!
//! Demonstrates using aws-sdk-mediapackagev2 with winterbaume.
//!
//! Run with:
//!   cargo run --example mediapackagev2 --package winterbaume-examples

use aws_sdk_mediapackagev2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediapackagev2::MediaPackageV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaPackageV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediapackagev2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediapackagev2::Client::new(&config);

    let resp = client
        .list_channel_groups()
        .send()
        .await
        .expect("list_channel_groups should succeed");
    println!("MediaPackage v2 channel groups: {}", resp.items().len());
}

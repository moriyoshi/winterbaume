//! Example: MediaPackage
//!
//! Demonstrates using aws-sdk-mediapackage with winterbaume.
//!
//! Run with:
//!   cargo run --example mediapackage --package winterbaume-examples

use aws_sdk_mediapackage::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediapackage::MediaPackageService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaPackageService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediapackage::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediapackage::Client::new(&config);

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    println!("MediaPackage channels: {}", resp.channels().len());
}

//! Example: MediaConnect
//!
//! Demonstrates using aws-sdk-mediaconnect with winterbaume.
//!
//! Run with:
//!   cargo run --example mediaconnect --package winterbaume-examples

use aws_sdk_mediaconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediaconnect::MediaConnectService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediaconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediaconnect::Client::new(&config);

    let resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");
    println!("MediaConnect flows: {}", resp.flows().len());
}

//! Example: Braket
//!
//! Demonstrates using aws-sdk-braket with winterbaume.
//!
//! Run with:
//!   cargo run --example braket --package winterbaume

use aws_sdk_braket::config::BehaviorVersion;
use winterbaume_braket::BraketService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BraketService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_braket::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_braket::Client::new(&config);
    let resp = client
        .search_devices()
        .send()
        .await
        .expect("search_devices");
    for d in resp.devices() {
        println!("Device: {} ({})", d.device_arn(), d.device_name());
    }
}

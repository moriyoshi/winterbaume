//! Example: Network Manager
//!
//! Demonstrates using aws-sdk-networkmanager with winterbaume.
//!
//! Run with:
//!   cargo run --example networkmanager --package winterbaume-examples

use aws_sdk_networkmanager::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkmanager::NetworkManagerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(NetworkManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_networkmanager::Client::new(&config);

    let resp = client
        .describe_global_networks()
        .send()
        .await
        .expect("describe_global_networks should succeed");
    println!("Global networks: {}", resp.global_networks().len());
}

//! Example: Backup Gateway
//!
//! Demonstrates using aws-sdk-backupgateway with winterbaume.
//!
//! Run with:
//!   cargo run --example backupgateway --package winterbaume

use aws_sdk_backupgateway::config::BehaviorVersion;
use winterbaume_backupgateway::BackupGatewayService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BackupGatewayService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backupgateway::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_backupgateway::Client::new(&config);

    let resp = client
        .list_gateways()
        .send()
        .await
        .expect("list_gateways should succeed");
    println!("Backup gateways: {}", resp.gateways().len());
}

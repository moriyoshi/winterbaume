//! Example: Transfer
//!
//! Demonstrates using aws-sdk-transfer with winterbaume.
//!
//! Run with:
//!   cargo run --example transfer --package winterbaume-examples

use aws_sdk_transfer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_transfer::TransferService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TransferService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transfer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_transfer::Client::new(&config);

    let resp = client
        .list_servers()
        .send()
        .await
        .expect("list_servers should succeed");
    println!("Transfer servers: {}", resp.servers().len());
}

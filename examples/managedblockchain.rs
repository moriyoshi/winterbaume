//! Example: Managed Blockchain
//!
//! Demonstrates using aws-sdk-managedblockchain with winterbaume.
//!
//! Run with:
//!   cargo run --example managedblockchain --package winterbaume-examples

use aws_sdk_managedblockchain::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_managedblockchain::ManagedBlockchainService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ManagedBlockchainService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_managedblockchain::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_managedblockchain::Client::new(&config);

    let resp = client
        .list_networks()
        .send()
        .await
        .expect("list_networks should succeed");
    println!("Managed Blockchain networks: {}", resp.networks().len());
}

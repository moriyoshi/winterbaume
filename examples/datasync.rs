//! Example: DataSync
//!
//! Demonstrates using aws-sdk-datasync with winterbaume.
//!
//! Run with:
//!   cargo run --example datasync --package winterbaume-examples

use aws_sdk_datasync::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_datasync::DataSyncService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DataSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_datasync::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_datasync::Client::new(&config);

    let resp = client
        .list_agents()
        .send()
        .await
        .expect("list_agents should succeed");
    println!("DataSync agents: {}", resp.agents().len());
}

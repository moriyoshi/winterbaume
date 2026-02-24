//! Example: Connect Campaigns
//!
//! Demonstrates using aws-sdk-connectcampaigns with winterbaume.
//!
//! Run with:
//!   cargo run --example connectcampaigns --package winterbaume-examples

use aws_sdk_connectcampaigns::config::BehaviorVersion;
use winterbaume_connectcampaigns::ConnectCampaignsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectCampaignsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectcampaigns::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connectcampaigns::Client::new(&config);

    // Connect Campaigns requires a Connect instance; create one via winterbaume-connect first.
    // This example demonstrates client setup for the ConnectCampaigns service.
    println!("ConnectCampaigns client ready.");
    let _client = client;
}

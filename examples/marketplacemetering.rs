//! Example: Marketplace Metering
//!
//! Demonstrates using aws-sdk-marketplacemetering with winterbaume.
//!
//! Run with:
//!   cargo run --example marketplacemetering --package winterbaume-examples

use aws_sdk_marketplacemetering::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_marketplacemetering::MarketplaceMeteringService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MarketplaceMeteringService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_marketplacemetering::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_marketplacemetering::Client::new(&config);

    // Marketplace Metering requires a valid product code and customer token.
    // This example demonstrates client setup for the MeteringMarketplace service.
    println!("MeteringMarketplace client ready. Use meter_usage() to report usage.");
    let _client = client;
}

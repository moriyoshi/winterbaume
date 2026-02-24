//! Example: RDS Data
//!
//! Demonstrates using aws-sdk-rdsdata with winterbaume.
//!
//! Run with:
//!   cargo run --example rdsdata --package winterbaume-examples

use aws_sdk_rdsdata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rdsdata::RdsDataService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RdsDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rdsdata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rdsdata::Client::new(&config);

    // RDS Data requires an Aurora cluster ARN and secret ARN.
    // This example demonstrates client setup for the RDS Data service.
    println!("RDS Data client ready. Use execute_statement() to run SQL.");
    let _client = client;
}

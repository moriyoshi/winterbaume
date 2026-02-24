//! Example: EBS (Elastic Block Store)
//!
//! Demonstrates using aws-sdk-ebs with winterbaume.
//!
//! Run with:
//!   cargo run --example ebs --package winterbaume-examples

use aws_sdk_ebs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ebs::EbsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EbsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ebs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ebs::Client::new(&config);

    // EBS Direct APIs operate on snapshot blocks; a snapshot ID is required.
    // This example demonstrates client setup for the EBS service.
    println!("EBS client ready. Use start_snapshot() to begin a new snapshot.");
    let _client = client;
}

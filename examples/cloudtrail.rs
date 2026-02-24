//! Example: CloudTrail
//!
//! Demonstrates using aws-sdk-cloudtrail with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudtrail --package winterbaume-examples

use aws_sdk_cloudtrail::config::BehaviorVersion;
use winterbaume_cloudtrail::CloudTrailService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudTrailService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtrail::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudtrail::Client::new(&config);

    let resp = client
        .describe_trails()
        .send()
        .await
        .expect("describe_trails should succeed");
    println!("Trails: {}", resp.trail_list().len());
}

//! Example: CloudWatch
//!
//! Demonstrates using aws-sdk-cloudwatch with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudwatch --package winterbaume-examples

use aws_sdk_cloudwatch::config::BehaviorVersion;
use winterbaume_cloudwatch::CloudWatchService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudWatchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudwatch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudwatch::Client::new(&config);

    let resp = client
        .list_metrics()
        .send()
        .await
        .expect("list_metrics should succeed");
    println!("Metrics: {}", resp.metrics().len());
}

//! Example: CloudFront
//!
//! Demonstrates using aws-sdk-cloudfront with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudfront --package winterbaume-examples

use aws_sdk_cloudfront::config::BehaviorVersion;
use winterbaume_cloudfront::CloudFrontService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudFrontService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudfront::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudfront::Client::new(&config);

    let resp = client
        .list_distributions()
        .send()
        .await
        .expect("list_distributions should succeed");
    let count = resp
        .distribution_list()
        .map(|l| l.items().len())
        .unwrap_or(0);
    println!("Distributions: {}", count);
}

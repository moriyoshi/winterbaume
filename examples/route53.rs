//! Example: Route 53
//!
//! Demonstrates using aws-sdk-route53 with winterbaume.
//!
//! Run with:
//!   cargo run --example route53 --package winterbaume-examples

use aws_sdk_route53::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53::Route53Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Route53Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_route53::Client::new(&config);

    let resp = client
        .list_hosted_zones()
        .send()
        .await
        .expect("list_hosted_zones should succeed");
    println!("Hosted zones: {}", resp.hosted_zones().len());
}

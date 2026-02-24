//! Example: Route 53 Domains
//!
//! Demonstrates using aws-sdk-route53domains with winterbaume.
//!
//! Run with:
//!   cargo run --example route53domains --package winterbaume-examples

use aws_sdk_route53domains::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53domains::Route53DomainsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Route53DomainsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53domains::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_route53domains::Client::new(&config);

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");
    println!("Route 53 domains: {}", resp.domains().len());
}

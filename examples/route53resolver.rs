//! Example: Route 53 Resolver
//!
//! Demonstrates using aws-sdk-route53resolver with winterbaume.
//!
//! Run with:
//!   cargo run --example route53resolver --package winterbaume-examples

use aws_sdk_route53resolver::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53resolver::Route53ResolverService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Route53ResolverService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53resolver::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_route53resolver::Client::new(&config);

    let resp = client
        .list_resolver_endpoints()
        .send()
        .await
        .expect("list_resolver_endpoints should succeed");
    println!(
        "Route 53 Resolver endpoints: {}",
        resp.resolver_endpoints().len()
    );
}

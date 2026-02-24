//! Example: ELBv2 (Elastic Load Balancing)
//!
//! Demonstrates using aws-sdk-elasticloadbalancingv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example elasticloadbalancingv2 --package winterbaume-examples

use aws_sdk_elasticloadbalancingv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancingv2::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_elasticloadbalancingv2::Client::new(&config);

    let resp = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe_load_balancers should succeed");
    println!("Load balancers: {}", resp.load_balancers().len());
}

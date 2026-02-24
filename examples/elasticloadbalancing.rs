//! Example: Classic Elastic Load Balancing
//!
//! Demonstrates using aws-sdk-elasticloadbalancing with winterbaume.
//!
//! Run with:
//!   cargo run --example elasticloadbalancing --package winterbaume-examples

use aws_sdk_elasticloadbalancing::config::BehaviorVersion;
use aws_sdk_elasticloadbalancing::types::Listener;
use winterbaume_core::MockAws;
use winterbaume_elasticloadbalancing::ElasticLoadBalancingService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElasticLoadBalancingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticloadbalancing::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_elasticloadbalancing::Client::new(&config);

    // Create a classic load balancer
    let create_resp = client
        .create_load_balancer()
        .load_balancer_name("my-classic-lb")
        .availability_zones("us-east-1a")
        .listeners(
            Listener::builder()
                .protocol("HTTP")
                .load_balancer_port(80)
                .instance_port(8080)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_load_balancer should succeed");

    println!(
        "Created load balancer with DNS: {:?}",
        create_resp.dns_name()
    );

    // Describe all load balancers
    let resp = client
        .describe_load_balancers()
        .send()
        .await
        .expect("describe_load_balancers should succeed");

    println!(
        "Classic ELB load balancers: {:?}",
        resp.load_balancer_descriptions()
    );
}

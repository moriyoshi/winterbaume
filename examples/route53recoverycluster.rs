//! Example: Route 53 Application Recovery Controller -- Cluster
//!
//! Demonstrates using aws-sdk-route53recoverycluster with winterbaume.
//!
//! Run with:
//!   cargo run --example route53recoverycluster --package winterbaume

use aws_sdk_route53recoverycluster::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53recoverycluster::RecoveryClusterService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RecoveryClusterService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53recoverycluster::config::Region::new(
            "us-east-1",
        ))
        .endpoint_url("https://route53-recovery-cluster.us-east-1.amazonaws.com")
        .load()
        .await;

    let client = aws_sdk_route53recoverycluster::Client::new(&config);
    let resp = client
        .list_routing_controls()
        .send()
        .await
        .expect("list_routing_controls should succeed");
    for rc in resp.routing_controls() {
        println!(
            "{}: {:?} -> {:?}",
            rc.routing_control_name().unwrap_or_default(),
            rc.routing_control_arn(),
            rc.routing_control_state()
        );
    }
}

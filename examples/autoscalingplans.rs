//! Example: AWS Auto Scaling Plans
//!
//! Demonstrates using aws-sdk-autoscalingplans with winterbaume.
//!
//! Run with:
//!   cargo run --example autoscalingplans --package winterbaume-examples

use aws_sdk_autoscalingplans::config::BehaviorVersion;
use winterbaume_autoscalingplans::AutoScalingPlansService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AutoScalingPlansService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_autoscalingplans::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_autoscalingplans::Client::new(&config);

    let resp = client
        .describe_scaling_plans()
        .send()
        .await
        .expect("describe_scaling_plans should succeed");
    println!("AutoScalingPlans plans: {}", resp.scaling_plans().len());
}

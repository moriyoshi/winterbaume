//! Example: Auto Scaling
//!
//! Demonstrates using aws-sdk-autoscaling with winterbaume.
//!
//! Run with:
//!   cargo run --example autoscaling --package winterbaume-examples

use aws_sdk_autoscaling::config::BehaviorVersion;
use winterbaume_autoscaling::AutoScalingService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AutoScalingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_autoscaling::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_autoscaling::Client::new(&config);

    // Create a launch configuration
    client
        .create_launch_configuration()
        .launch_configuration_name("example-lc")
        .image_id("ami-12345678")
        .instance_type("t2.micro")
        .send()
        .await
        .expect("create_launch_configuration should succeed");

    // Create an Auto Scaling group
    client
        .create_auto_scaling_group()
        .auto_scaling_group_name("example-asg")
        .launch_configuration_name("example-lc")
        .min_size(1)
        .max_size(3)
        .desired_capacity(1)
        .availability_zones("us-east-1a")
        .send()
        .await
        .expect("create_auto_scaling_group should succeed");

    // Describe the Auto Scaling group
    let resp = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("example-asg")
        .send()
        .await
        .expect("describe_auto_scaling_groups should succeed");

    for group in resp.auto_scaling_groups() {
        println!(
            "ASG: {:?} (min={:?}, max={:?}, desired={:?})",
            group.auto_scaling_group_name(),
            group.min_size(),
            group.max_size(),
            group.desired_capacity(),
        );
    }
}

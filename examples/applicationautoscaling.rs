//! Example: Application Auto Scaling
//!
//! Demonstrates using aws-sdk-applicationautoscaling with winterbaume.
//!
//! Run with:
//!   cargo run --example applicationautoscaling --package winterbaume-examples

use aws_sdk_applicationautoscaling::config::BehaviorVersion;
use winterbaume_applicationautoscaling::ApplicationAutoScalingService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationAutoScalingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationautoscaling::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationautoscaling::Client::new(&config);

    use aws_sdk_applicationautoscaling::types::ServiceNamespace;
    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scalable_targets should succeed");
    println!("Scalable targets: {}", resp.scalable_targets().len());
}

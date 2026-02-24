//! Example: Cost Optimization Hub
//!
//! Demonstrates using aws-sdk-costoptimizationhub with winterbaume.
//!
//! Run with:
//!   cargo run --example costoptimizationhub --package winterbaume

use aws_sdk_costoptimizationhub::config::BehaviorVersion;
use aws_sdk_costoptimizationhub::types::EnrollmentStatus;
use winterbaume_core::MockAws;
use winterbaume_costoptimizationhub::CostOptimizationHubService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CostOptimizationHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costoptimizationhub::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_costoptimizationhub::Client::new(&config);
    let resp = client
        .update_enrollment_status()
        .status(EnrollmentStatus::Active)
        .send()
        .await
        .expect("update");
    println!("Enrollment status: {:?}", resp.status());
}

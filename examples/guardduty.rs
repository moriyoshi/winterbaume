//! Example: GuardDuty
//!
//! Demonstrates using aws-sdk-guardduty with winterbaume.
//!
//! Run with:
//!   cargo run --example guardduty --package winterbaume-examples

use aws_sdk_guardduty::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_guardduty::GuardDutyService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(GuardDutyService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_guardduty::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_guardduty::Client::new(&config);

    let resp = client
        .list_detectors()
        .send()
        .await
        .expect("list_detectors should succeed");
    println!("GuardDuty detectors: {}", resp.detector_ids().len());
}

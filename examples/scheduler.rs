//! Example: EventBridge Scheduler
//!
//! Demonstrates using aws-sdk-scheduler with winterbaume.
//!
//! Run with:
//!   cargo run --example scheduler --package winterbaume-examples

use aws_sdk_scheduler::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_scheduler::SchedulerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SchedulerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_scheduler::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_scheduler::Client::new(&config);

    let resp = client
        .list_schedules()
        .send()
        .await
        .expect("list_schedules should succeed");
    println!(
        "EventBridge Scheduler schedules: {}",
        resp.schedules().len()
    );
}

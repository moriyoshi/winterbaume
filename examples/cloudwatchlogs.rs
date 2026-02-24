//! Example: CloudWatch Logs
//!
//! Demonstrates using aws-sdk-cloudwatchlogs with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudwatchlogs --package winterbaume-examples

use aws_sdk_cloudwatchlogs::config::BehaviorVersion;
use winterbaume_cloudwatchlogs::CloudWatchLogsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudWatchLogsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudwatchlogs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudwatchlogs::Client::new(&config);

    let resp = client
        .describe_log_groups()
        .send()
        .await
        .expect("describe_log_groups should succeed");
    println!("CloudWatch log groups: {}", resp.log_groups().len());
}

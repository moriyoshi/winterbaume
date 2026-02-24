//! Example: Security Hub
//!
//! Demonstrates using aws-sdk-securityhub with winterbaume.
//!
//! Run with:
//!   cargo run --example securityhub --package winterbaume-examples

use aws_sdk_securityhub::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_securityhub::SecurityHubService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SecurityHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_securityhub::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_securityhub::Client::new(&config);

    let resp = client
        .describe_hub()
        .send()
        .await
        .expect("describe_hub should succeed");
    println!("Security Hub ARN: {}", resp.hub_arn().unwrap_or("(none)"));
}

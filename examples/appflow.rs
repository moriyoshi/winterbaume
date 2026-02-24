//! Example: AppFlow
//!
//! Demonstrates using aws-sdk-appflow with winterbaume.
//!
//! Run with:
//!   cargo run --example appflow --package winterbaume-examples

use aws_sdk_appflow::config::BehaviorVersion;
use winterbaume_appflow::AppFlowService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppFlowService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appflow::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appflow::Client::new(&config);

    let resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");
    println!("AppFlow flows: {}", resp.flows().len());
}

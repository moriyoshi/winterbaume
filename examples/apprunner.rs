//! Example: App Runner
//!
//! Demonstrates using aws-sdk-apprunner with winterbaume.
//!
//! Run with:
//!   cargo run --example apprunner --package winterbaume-examples

use aws_sdk_apprunner::config::BehaviorVersion;
use winterbaume_apprunner::AppRunnerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppRunnerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apprunner::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_apprunner::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("App Runner services: {}", resp.service_summary_list().len());
}

//! Example: Application Insights
//!
//! Demonstrates using aws-sdk-applicationinsights with winterbaume.
//!
//! Run with:
//!   cargo run --example applicationinsights --package winterbaume

use aws_sdk_applicationinsights::config::BehaviorVersion;
use winterbaume_applicationinsights::ApplicationInsightsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationInsightsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationinsights::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationinsights::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!(
        "Application Insights applications: {}",
        resp.application_info_list().len()
    );
}

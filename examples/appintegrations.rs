//! Example: AppIntegrations
//!
//! Demonstrates using aws-sdk-appintegrations with winterbaume.
//!
//! Run with:
//!   cargo run --example appintegrations --package winterbaume

use aws_sdk_appintegrations::config::BehaviorVersion;
use winterbaume_appintegrations::AppIntegrationsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppIntegrationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appintegrations::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appintegrations::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!(
        "AppIntegrations applications: {}",
        resp.applications().len()
    );
}

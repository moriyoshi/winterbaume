//! Example: AppFabric
//!
//! Demonstrates using aws-sdk-appfabric with winterbaume.
//!
//! Run with:
//!   cargo run --example appfabric --package winterbaume-examples

use aws_sdk_appfabric::config::BehaviorVersion;
use winterbaume_appfabric::AppFabricService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppFabricService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appfabric::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appfabric::Client::new(&config);

    let resp = client
        .list_app_bundles()
        .send()
        .await
        .expect("list_app_bundles should succeed");
    println!(
        "AppFabric bundles: {}",
        resp.app_bundle_summary_list().len()
    );
}

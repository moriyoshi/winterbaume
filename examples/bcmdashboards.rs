//! Example: BCM Dashboards
//!
//! Demonstrates using aws-sdk-bcmdashboards with winterbaume.
//!
//! Run with:
//!   cargo run --example bcmdashboards --package winterbaume

use aws_sdk_bcmdashboards::config::BehaviorVersion;
use winterbaume_bcmdashboards::BcmDashboardsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BcmDashboardsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmdashboards::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bcmdashboards::Client::new(&config);

    let resp = client
        .list_dashboards()
        .send()
        .await
        .expect("list_dashboards should succeed");
    println!("BCM dashboards: {}", resp.dashboards().len());
}

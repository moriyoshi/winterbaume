//! Example: Control Catalog
//!
//! Demonstrates using aws-sdk-controlcatalog with winterbaume.
//!
//! Run with:
//!   cargo run --example controlcatalog --package winterbaume

use aws_sdk_controlcatalog::config::BehaviorVersion;
use winterbaume_controlcatalog::ControlCatalogService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ControlCatalogService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_controlcatalog::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_controlcatalog::Client::new(&config);
    let resp = client.list_controls().send().await.expect("list_controls");
    for c in resp.controls() {
        println!("Control: {:?}", c.arn());
    }
}

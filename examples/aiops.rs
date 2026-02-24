//! Example: AIOps
//!
//! Demonstrates using aws-sdk-aiops with winterbaume.
//!
//! Run with:
//!   cargo run --example aiops --package winterbaume-examples

use aws_sdk_aiops::config::BehaviorVersion;
use winterbaume_aiops::AIOpsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(AIOpsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_aiops::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_aiops::Client::new(&config);

    let resp = client
        .list_investigation_groups()
        .send()
        .await
        .expect("list_investigation_groups should succeed");
    println!(
        "AIOps investigation groups: {}",
        resp.investigation_groups().len()
    );
}

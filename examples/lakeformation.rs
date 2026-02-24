//! Example: Lake Formation
//!
//! Demonstrates using aws-sdk-lakeformation with winterbaume.
//!
//! Run with:
//!   cargo run --example lakeformation --package winterbaume-examples

use aws_sdk_lakeformation::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_lakeformation::LakeFormationService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(LakeFormationService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lakeformation::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_lakeformation::Client::new(&config);

    let resp = client
        .list_resources()
        .send()
        .await
        .expect("list_resources should succeed");
    println!(
        "Lake Formation resources: {}",
        resp.resource_info_list().len()
    );
}

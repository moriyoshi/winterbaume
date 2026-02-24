//! Example: Resource Groups
//!
//! Demonstrates using aws-sdk-resourcegroups with winterbaume.
//!
//! Run with:
//!   cargo run --example resourcegroups --package winterbaume-examples

use aws_sdk_resourcegroups::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_resourcegroups::ResourceGroupsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ResourceGroupsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroups::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_resourcegroups::Client::new(&config);

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");
    println!("Resource groups: {}", resp.group_identifiers().len());
}

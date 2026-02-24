//! Example: Resource Groups Tagging
//!
//! Demonstrates using aws-sdk-resourcegroupstagging with winterbaume.
//!
//! Run with:
//!   cargo run --example tagging --package winterbaume-examples

use aws_sdk_resourcegroupstagging::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_resourcegroupstagging::TaggingService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TaggingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroupstagging::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_resourcegroupstagging::Client::new(&config);

    let resp = client
        .get_resources()
        .send()
        .await
        .expect("get_resources should succeed");
    println!(
        "Tagged resources: {}",
        resp.resource_tag_mapping_list().len()
    );
}

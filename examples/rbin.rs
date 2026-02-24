//! Example: Recycle Bin
//!
//! Demonstrates using aws-sdk-rbin with winterbaume.
//!
//! Run with:
//!   cargo run --example rbin --package winterbaume

use aws_sdk_rbin::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rbin::RbinService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(RbinService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rbin::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rbin::Client::new(&config);

    let resp = client
        .list_rules()
        .resource_type(aws_sdk_rbin::types::ResourceType::EbsSnapshot)
        .send()
        .await
        .expect("list_rules should succeed");
    println!("Recycle Bin rules: {}", resp.rules().len());
}

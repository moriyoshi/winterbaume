//! Example: CloudFormation
//!
//! Demonstrates using aws-sdk-cloudformation with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudformation --package winterbaume-examples

use aws_sdk_cloudformation::config::BehaviorVersion;
use winterbaume_cloudformation::CloudFormationService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudFormationService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudformation::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudformation::Client::new(&config);

    let resp = client
        .list_stacks()
        .send()
        .await
        .expect("list_stacks should succeed");
    println!("CloudFormation stacks: {:?}", resp.stack_summaries());
}

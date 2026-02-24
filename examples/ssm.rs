//! Example: SSM (Systems Manager)
//!
//! Demonstrates using aws-sdk-ssm with winterbaume.
//!
//! Run with:
//!   cargo run --example ssm --package winterbaume-examples

use aws_sdk_ssm::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ssm::SsmService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SsmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssm::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ssm::Client::new(&config);

    let resp = client
        .describe_parameters()
        .send()
        .await
        .expect("describe_parameters should succeed");
    println!("SSM parameters: {}", resp.parameters().len());
}

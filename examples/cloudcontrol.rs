//! Example: Cloud Control API
//!
//! Demonstrates using aws-sdk-cloudcontrol with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudcontrol --package winterbaume-examples

use aws_sdk_cloudcontrol::config::BehaviorVersion;
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudControlService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudcontrol::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudcontrol::Client::new(&config);

    let resp = client
        .list_resources()
        .type_name("AWS::S3::Bucket")
        .send()
        .await
        .expect("list_resources should succeed");
    println!("Cloud Control API: {:?}", resp.resource_descriptions());
}

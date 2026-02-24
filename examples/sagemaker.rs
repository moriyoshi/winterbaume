//! Example: SageMaker
//!
//! Demonstrates using aws-sdk-sagemaker with winterbaume.
//!
//! Run with:
//!   cargo run --example sagemaker --package winterbaume-examples

use aws_sdk_sagemaker::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sagemaker::SageMakerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SageMakerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemaker::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sagemaker::Client::new(&config);

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");
    println!("SageMaker domains: {}", resp.domains().len());
}

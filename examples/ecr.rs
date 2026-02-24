//! Example: ECR
//!
//! Demonstrates using aws-sdk-ecr with winterbaume.
//!
//! Run with:
//!   cargo run --example ecr --package winterbaume-examples

use aws_sdk_ecr::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ecr::EcrService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EcrService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecr::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ecr::Client::new(&config);

    let resp = client
        .describe_repositories()
        .send()
        .await
        .expect("describe_repositories should succeed");
    println!("ECR repositories: {}", resp.repositories().len());
}

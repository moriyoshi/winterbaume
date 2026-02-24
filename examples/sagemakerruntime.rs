//! Example: SageMaker Runtime
//!
//! Demonstrates using aws-sdk-sagemakerruntime with winterbaume.
//!
//! Run with:
//!   cargo run --example sagemakerruntime --package winterbaume-examples

use aws_sdk_sagemakerruntime::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sagemakerruntime::SageMakerRuntimeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SageMakerRuntimeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemakerruntime::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sagemakerruntime::Client::new(&config);

    // SageMaker Runtime invokes deployed endpoints.
    // This example demonstrates client setup for the SageMakerRuntime service.
    println!("SageMakerRuntime client ready. Use invoke_endpoint() to call a deployed model.");
    let _client = client;
}

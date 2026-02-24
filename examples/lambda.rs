//! Example: Lambda
//!
//! Demonstrates using aws-sdk-lambda with winterbaume.
//!
//! Run with:
//!   cargo run --example lambda --package winterbaume-examples

use aws_sdk_lambda::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_lambda::LambdaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(LambdaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lambda::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_lambda::Client::new(&config);

    let resp = client
        .list_functions()
        .send()
        .await
        .expect("list_functions should succeed");
    println!("Lambda functions: {}", resp.functions().len());
}

//! Example: CodeDeploy
//!
//! Demonstrates using aws-sdk-codedeploy with winterbaume.
//!
//! Run with:
//!   cargo run --example codedeploy --package winterbaume-examples

use aws_sdk_codedeploy::config::BehaviorVersion;
use winterbaume_codedeploy::CodeDeployService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeDeployService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codedeploy::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codedeploy::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("Applications: {}", resp.applications().len());
}

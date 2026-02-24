//! Example: CodeBuild
//!
//! Demonstrates using aws-sdk-codebuild with winterbaume.
//!
//! Run with:
//!   cargo run --example codebuild --package winterbaume-examples

use aws_sdk_codebuild::config::BehaviorVersion;
use winterbaume_codebuild::CodeBuildService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeBuildService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codebuild::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codebuild::Client::new(&config);

    let resp = client
        .list_projects()
        .send()
        .await
        .expect("list_projects should succeed");
    println!("Projects: {}", resp.projects().len());
}

//! Example: CodePipeline
//!
//! Demonstrates using aws-sdk-codepipeline with winterbaume.
//!
//! Run with:
//!   cargo run --example codepipeline --package winterbaume-examples

use aws_sdk_codepipeline::config::BehaviorVersion;
use winterbaume_codepipeline::CodePipelineService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodePipelineService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codepipeline::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codepipeline::Client::new(&config);

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");
    println!("Pipelines: {}", resp.pipelines().len());
}

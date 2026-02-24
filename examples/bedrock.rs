//! Example: Bedrock
//!
//! Demonstrates using aws-sdk-bedrock with winterbaume.
//!
//! Run with:
//!   cargo run --example bedrock --package winterbaume-examples

use aws_sdk_bedrock::config::BehaviorVersion;
use winterbaume_bedrock::BedrockService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BedrockService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bedrock::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bedrock::Client::new(&config);

    let resp = client
        .list_foundation_models()
        .send()
        .await
        .expect("list_foundation_models should succeed");
    println!("Foundation models: {}", resp.model_summaries().len());
}

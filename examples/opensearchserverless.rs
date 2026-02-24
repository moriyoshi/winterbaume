//! Example: OpenSearch Serverless
//!
//! Demonstrates using aws-sdk-opensearchserverless with winterbaume.
//!
//! Run with:
//!   cargo run --example opensearchserverless --package winterbaume-examples

use aws_sdk_opensearchserverless::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_opensearchserverless::OpenSearchServerlessService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OpenSearchServerlessService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_opensearchserverless::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_opensearchserverless::Client::new(&config);

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed");
    println!(
        "OpenSearch Serverless collections: {:?}",
        resp.collection_summaries()
    );
}

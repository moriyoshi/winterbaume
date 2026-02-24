//! Example: OpenSearch
//!
//! Demonstrates using aws-sdk-opensearch with winterbaume.
//!
//! Run with:
//!   cargo run --example opensearch --package winterbaume-examples

use aws_sdk_opensearch::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_opensearch::OpenSearchService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OpenSearchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_opensearch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_opensearch::Client::new(&config);

    let resp = client
        .list_domain_names()
        .send()
        .await
        .expect("list_domain_names should succeed");
    println!("OpenSearch domains: {}", resp.domain_names().len());
}

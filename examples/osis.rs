//! Example: OpenSearch Ingestion
//!
//! Demonstrates using aws-sdk-osis with winterbaume.
//!
//! Run with:
//!   cargo run --example osis --package winterbaume-examples

use aws_sdk_osis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_osis::OsisService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(OsisService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_osis::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_osis::Client::new(&config);

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");
    println!("OpenSearch Ingestion pipelines: {}", resp.pipelines().len());
}

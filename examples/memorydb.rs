//! Example: MemoryDB
//!
//! Demonstrates using aws-sdk-memorydb with winterbaume.
//!
//! Run with:
//!   cargo run --example memorydb --package winterbaume-examples

use aws_sdk_memorydb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_memorydb::MemoryDbService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MemoryDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_memorydb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_memorydb::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("MemoryDB clusters: {}", resp.clusters().len());
}

//! Example: EMR (Elastic MapReduce)
//!
//! Demonstrates using aws-sdk-emr with winterbaume.
//!
//! Run with:
//!   cargo run --example emr --package winterbaume-examples

use aws_sdk_emr::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emr::EmrService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EmrService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emr::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_emr::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("EMR clusters: {}", resp.clusters().len());
}

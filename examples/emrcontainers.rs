//! Example: EMR Containers
//!
//! Demonstrates using aws-sdk-emrcontainers with winterbaume.
//!
//! Run with:
//!   cargo run --example emrcontainers --package winterbaume-examples

use aws_sdk_emrcontainers::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emrcontainers::EmrContainersService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(EmrContainersService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emrcontainers::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_emrcontainers::Client::new(&config);

    let resp = client
        .list_virtual_clusters()
        .send()
        .await
        .expect("list_virtual_clusters should succeed");
    println!("EMR virtual clusters: {}", resp.virtual_clusters().len());
}

//! Example: DAX
//!
//! Demonstrates using aws-sdk-dax with winterbaume.
//!
//! Run with:
//!   cargo run --example dax --package winterbaume-examples

use aws_sdk_dax::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dax::DaxService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(DaxService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dax::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dax::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("DAX clusters: {}", resp.clusters().len());
}

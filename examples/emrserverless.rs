//! Example: EMR Serverless
//!
//! Demonstrates using aws-sdk-emrserverless with winterbaume.
//!
//! Run with:
//!   cargo run --example emrserverless --package winterbaume-examples

use aws_sdk_emrserverless::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emrserverless::EmrServerlessService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(EmrServerlessService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emrserverless::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_emrserverless::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("EMR Serverless applications: {}", resp.applications().len());
}

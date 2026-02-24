//! Example: AWS Artifact
//!
//! Demonstrates using aws-sdk-artifact with winterbaume.
//!
//! Run with:
//!   cargo run --example artifact --package winterbaume-examples

use aws_sdk_artifact::config::BehaviorVersion;
use winterbaume_artifact::ArtifactService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ArtifactService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_artifact::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_artifact::Client::new(&config);

    let resp = client
        .list_reports()
        .send()
        .await
        .expect("list_reports should succeed");
    println!("Artifact reports: {}", resp.reports().len());
}

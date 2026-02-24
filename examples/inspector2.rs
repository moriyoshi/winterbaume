//! Example: Inspector2
//!
//! Demonstrates using aws-sdk-inspector2 with winterbaume.
//!
//! Run with:
//!   cargo run --example inspector2 --package winterbaume-examples

use aws_sdk_inspector2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_inspector2::Inspector2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Inspector2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_inspector2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_inspector2::Client::new(&config);

    let resp = client
        .list_coverage()
        .send()
        .await
        .expect("list_coverage should succeed");
    println!(
        "Inspector2 covered resources: {}",
        resp.covered_resources().len()
    );
}

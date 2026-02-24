//! Example: Athena
//!
//! Demonstrates using aws-sdk-athena with winterbaume.
//!
//! Run with:
//!   cargo run --example athena --package winterbaume-examples

use aws_sdk_athena::config::BehaviorVersion;
use winterbaume_athena::AthenaService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AthenaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_athena::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_athena::Client::new(&config);

    let resp = client
        .list_work_groups()
        .send()
        .await
        .expect("list_work_groups should succeed");
    println!("Work groups: {}", resp.work_groups().len());
}

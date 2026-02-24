//! Example: Connect
//!
//! Demonstrates using aws-sdk-connect with winterbaume.
//!
//! Run with:
//!   cargo run --example connect --package winterbaume-examples

use aws_sdk_connect::config::BehaviorVersion;
use winterbaume_connect::ConnectService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connect::Client::new(&config);

    let resp = client
        .list_instances()
        .send()
        .await
        .expect("list_instances should succeed");
    println!("Connect instances: {}", resp.instance_summary_list().len());
}

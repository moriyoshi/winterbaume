//! Example: Direct Connect
//!
//! Demonstrates using aws-sdk-directconnect with winterbaume.
//!
//! Run with:
//!   cargo run --example directconnect --package winterbaume-examples

use aws_sdk_directconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_directconnect::DirectConnectService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DirectConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_directconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_directconnect::Client::new(&config);

    let resp = client
        .describe_connections()
        .send()
        .await
        .expect("describe_connections should succeed");
    println!("Direct Connect connections: {}", resp.connections().len());
}

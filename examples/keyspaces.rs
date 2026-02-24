//! Example: Amazon Keyspaces
//!
//! Demonstrates using aws-sdk-keyspaces with winterbaume.
//!
//! Run with:
//!   cargo run --example keyspaces --package winterbaume-examples

use aws_sdk_keyspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_keyspaces::KeyspacesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KeyspacesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_keyspaces::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_keyspaces::Client::new(&config);

    let resp = client
        .list_keyspaces()
        .send()
        .await
        .expect("list_keyspaces should succeed");
    println!("Keyspaces: {:?}", resp.keyspaces());
}

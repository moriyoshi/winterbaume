//! Example: EventBridge Pipes
//!
//! Demonstrates using aws-sdk-pipes with winterbaume.
//!
//! Run with:
//!   cargo run --example pipes --package winterbaume-examples

use aws_sdk_pipes::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pipes::PipesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(PipesService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pipes::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pipes::Client::new(&config);

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed");
    println!("EventBridge Pipes: {}", resp.pipes().len());
}

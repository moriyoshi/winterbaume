//! Example: QuickSight
//!
//! Demonstrates using aws-sdk-quicksight with winterbaume.
//!
//! Run with:
//!   cargo run --example quicksight --package winterbaume-examples

use aws_sdk_quicksight::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_quicksight::QuickSightService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(QuickSightService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_quicksight::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_quicksight::Client::new(&config);

    let resp = client
        .list_users()
        .aws_account_id("123456789012")
        .namespace("default")
        .send()
        .await
        .expect("list_users should succeed");
    println!("QuickSight users: {}", resp.user_list().len());
}

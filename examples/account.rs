//! Example: Account
//!
//! Demonstrates using aws-sdk-account with winterbaume.
//!
//! Run with:
//!   cargo run --example account --package winterbaume-examples

use aws_sdk_account::config::BehaviorVersion;
use winterbaume_account::AccountService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AccountService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_account::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_account::Client::new(&config);

    let resp = client
        .list_regions()
        .send()
        .await
        .expect("list_regions should succeed");
    println!("Regions: {}", resp.regions().len());
}

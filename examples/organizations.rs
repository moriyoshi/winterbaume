//! Example: Organizations
//!
//! Demonstrates using aws-sdk-organizations with winterbaume.
//!
//! Run with:
//!   cargo run --example organizations --package winterbaume-examples

use aws_sdk_organizations::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_organizations::OrganizationsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OrganizationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_organizations::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_organizations::Client::new(&config);

    let resp = client
        .list_accounts()
        .send()
        .await
        .expect("list_accounts should succeed");
    println!("Organization accounts: {}", resp.accounts().len());
}

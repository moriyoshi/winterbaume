//! Example: SSO
//!
//! Demonstrates using aws-sdk-sso with winterbaume.
//!
//! Run with:
//!   cargo run --example sso --package winterbaume-examples

use aws_sdk_sso::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sso::SsoService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SsoService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sso::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sso::Client::new(&config);

    let resp = client
        .list_accounts()
        .access_token("mock-token")
        .send()
        .await
        .expect("list_accounts should succeed");
    println!("SSO accounts: {}", resp.account_list().len());
}

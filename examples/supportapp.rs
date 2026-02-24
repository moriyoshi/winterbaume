//! Example: Support App
//!
//! Demonstrates using aws-sdk-supportapp with winterbaume.
//!
//! Run with:
//!   cargo run --example supportapp --package winterbaume

use aws_sdk_supportapp::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_supportapp::SupportAppService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SupportAppService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_supportapp::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_supportapp::Client::new(&config);
    client
        .put_account_alias()
        .account_alias("demo-account")
        .send()
        .await
        .expect("put_account_alias should succeed");
    let resp = client
        .get_account_alias()
        .send()
        .await
        .expect("get_account_alias should succeed");
    println!("Account alias: {:?}", resp.account_alias());
}

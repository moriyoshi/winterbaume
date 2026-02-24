//! Example: STS
//!
//! Demonstrates using aws-sdk-sts with winterbaume.
//!
//! Run with:
//!   cargo run --example sts --package winterbaume-examples

use aws_sdk_sts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(StsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sts::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sts::Client::new(&config);

    let resp = client
        .get_caller_identity()
        .send()
        .await
        .expect("get_caller_identity should succeed");
    println!("Account: {}", resp.account().unwrap_or_default());
    println!("User ID: {}", resp.user_id().unwrap_or_default());
    println!("ARN: {}", resp.arn().unwrap_or_default());
}

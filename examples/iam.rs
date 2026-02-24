//! Example: IAM
//!
//! Demonstrates using aws-sdk-iam with winterbaume.
//!
//! Run with:
//!   cargo run --example iam --package winterbaume-examples

use aws_sdk_iam::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_iam::IamService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(IamService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iam::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_iam::Client::new(&config);

    let resp = client
        .list_users()
        .send()
        .await
        .expect("list_users should succeed");
    println!("IAM users: {}", resp.users().len());
}

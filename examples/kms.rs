//! Example: KMS
//!
//! Demonstrates using aws-sdk-kms with winterbaume.
//!
//! Run with:
//!   cargo run --example kms --package winterbaume-examples

use aws_sdk_kms::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kms::KmsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(KmsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kms::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kms::Client::new(&config);

    let resp = client
        .list_keys()
        .send()
        .await
        .expect("list_keys should succeed");
    println!("KMS keys: {}", resp.keys().len());
}

//! Example: Signer
//!
//! Demonstrates using aws-sdk-signer with winterbaume.
//!
//! Run with:
//!   cargo run --example signer --package winterbaume-examples

use aws_sdk_signer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_signer::SignerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SignerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_signer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_signer::Client::new(&config);

    let resp = client
        .list_signing_profiles()
        .send()
        .await
        .expect("list_signing_profiles should succeed");
    println!("Signer signing profiles: {}", resp.profiles().len());
}

//! Example: Glacier
//!
//! Demonstrates using aws-sdk-glacier with winterbaume.
//!
//! Run with:
//!   cargo run --example glacier --package winterbaume-examples

use aws_sdk_glacier::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_glacier::GlacierService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(GlacierService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_glacier::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_glacier::Client::new(&config);

    let resp = client
        .list_vaults()
        .account_id("-")
        .send()
        .await
        .expect("list_vaults should succeed");
    println!("Glacier vaults: {}", resp.vault_list().len());
}

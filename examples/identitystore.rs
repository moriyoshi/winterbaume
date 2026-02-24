//! Example: Identity Store
//!
//! Demonstrates using aws-sdk-identitystore with winterbaume.
//!
//! Run with:
//!   cargo run --example identitystore --package winterbaume-examples

use aws_sdk_identitystore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_identitystore::IdentityStoreService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(IdentityStoreService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_identitystore::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_identitystore::Client::new(&config);

    let resp = client
        .list_users()
        .identity_store_id("d-1234567890")
        .send()
        .await
        .expect("list_users should succeed");
    println!("Identity store users: {}", resp.users().len());
}

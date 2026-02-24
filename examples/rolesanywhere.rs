//! Example: IAM Roles Anywhere
//!
//! Demonstrates using aws-sdk-rolesanywhere with winterbaume.
//!
//! Run with:
//!   cargo run --example rolesanywhere --package winterbaume-examples

use aws_sdk_rolesanywhere::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rolesanywhere::RolesAnywhereService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RolesAnywhereService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rolesanywhere::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rolesanywhere::Client::new(&config);

    let resp = client
        .list_profiles()
        .send()
        .await
        .expect("list_profiles should succeed");
    println!("Roles Anywhere profiles: {:?}", resp.profiles());
}

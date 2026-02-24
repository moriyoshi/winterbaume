//! Example: Systems Manager Quick Setup
//!
//! Demonstrates using aws-sdk-ssmquicksetup with winterbaume.
//!
//! Run with:
//!   cargo run --example ssmquicksetup --package winterbaume

use aws_sdk_ssmquicksetup::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ssmquicksetup::SsmQuickSetupService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SsmQuickSetupService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ssmquicksetup::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ssmquicksetup::Client::new(&config);
    let resp = client
        .list_quick_setup_types()
        .send()
        .await
        .expect("list_quick_setup_types should succeed");
    for t in resp.quick_setup_type_list() {
        println!("Type: {:?} v{:?}", t.r#type(), t.latest_version());
    }
}

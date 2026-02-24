//! Example: BCM Data Exports
//!
//! Demonstrates using aws-sdk-bcmdataexports with winterbaume.
//!
//! Run with:
//!   cargo run --example bcmdataexports --package winterbaume

use aws_sdk_bcmdataexports::config::BehaviorVersion;
use winterbaume_bcmdataexports::BcmDataExportsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BcmDataExportsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmdataexports::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bcmdataexports::Client::new(&config);

    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    println!("BCM data exports: {}", resp.exports().len());
}

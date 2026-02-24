//! Example: ARC Zonal Shift
//!
//! Demonstrates using aws-sdk-arczonalshift with winterbaume.
//!
//! Run with:
//!   cargo run --example arczonalshift --package winterbaume

use aws_sdk_arczonalshift::config::BehaviorVersion;
use winterbaume_arczonalshift::ArcZonalShiftService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ArcZonalShiftService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_arczonalshift::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_arczonalshift::Client::new(&config);

    let resp = client
        .list_managed_resources()
        .send()
        .await
        .expect("list_managed_resources should succeed");
    println!("ARC managed resources: {}", resp.items().len());
}

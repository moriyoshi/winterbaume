//! Example: X-Ray
//!
//! Demonstrates using aws-sdk-xray with winterbaume.
//!
//! Run with:
//!   cargo run --example xray --package winterbaume-examples

use aws_sdk_xray::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_xray::XRayService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(XRayService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_xray::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_xray::Client::new(&config);

    let resp = client
        .get_groups()
        .send()
        .await
        .expect("get_groups should succeed");
    println!("X-Ray groups: {}", resp.groups().len());
}

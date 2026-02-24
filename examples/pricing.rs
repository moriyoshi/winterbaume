//! Example: Pricing
//!
//! Demonstrates using aws-sdk-pricing with winterbaume.
//!
//! Run with:
//!   cargo run --example pricing --package winterbaume

use aws_sdk_pricing::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pricing::PricingService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PricingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pricing::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pricing::Client::new(&config);
    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");
    for svc in resp.services() {
        println!("Service: {:?}", svc.service_code());
    }
}

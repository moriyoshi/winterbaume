//! Example: Billing
//!
//! Demonstrates using aws-sdk-billing with winterbaume.
//!
//! Run with:
//!   cargo run --example billing --package winterbaume

use aws_sdk_billing::config::BehaviorVersion;
use winterbaume_billing::BillingService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BillingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_billing::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_billing::Client::new(&config);
    let resp = client
        .create_billing_view()
        .name("demo-view")
        .send()
        .await
        .expect("create");
    println!("BillingView: {}", resp.arn());
}

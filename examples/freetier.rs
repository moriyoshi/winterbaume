//! Example: Free Tier
//!
//! Demonstrates using aws-sdk-freetier with winterbaume.
//!
//! Run with:
//!   cargo run --example freetier --package winterbaume

use aws_sdk_freetier::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_freetier::FreeTierService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(FreeTierService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_freetier::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_freetier::Client::new(&config);
    let resp = client
        .get_account_plan_state()
        .send()
        .await
        .expect("get_account_plan_state should succeed");
    println!(
        "Plan: {:?} ({:?})",
        resp.account_plan_type(),
        resp.account_plan_status()
    );
}

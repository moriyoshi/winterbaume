//! Example: Savings Plans
//!
//! Demonstrates using aws-sdk-savingsplans with winterbaume.
//!
//! Run with:
//!   cargo run --example savingsplans --package winterbaume

use aws_sdk_savingsplans::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_savingsplans::SavingsPlansService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SavingsPlansService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_savingsplans::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_savingsplans::Client::new(&config);

    let create = client
        .create_savings_plan()
        .savings_plan_offering_id("offering-1")
        .commitment("10.00")
        .send()
        .await
        .expect("create_savings_plan should succeed");
    println!(
        "Created savings plan: {}",
        create.savings_plan_id().unwrap_or_default()
    );

    let resp = client
        .describe_savings_plans()
        .send()
        .await
        .expect("describe_savings_plans should succeed");
    println!("Total savings plans: {}", resp.savings_plans().len());
}

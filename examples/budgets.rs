//! Example: Budgets
//!
//! Demonstrates using aws-sdk-budgets with winterbaume.
//!
//! Run with:
//!   cargo run --example budgets --package winterbaume-examples

use aws_sdk_budgets::config::BehaviorVersion;
use winterbaume_budgets::BudgetsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BudgetsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_budgets::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_budgets::Client::new(&config);

    let resp = client
        .describe_budgets()
        .account_id("123456789012")
        .send()
        .await
        .expect("describe_budgets should succeed");
    println!("Budgets: {}", resp.budgets().len());
}

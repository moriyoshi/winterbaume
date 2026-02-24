//! Example: Trusted Advisor
//!
//! Demonstrates using aws-sdk-trustedadvisor with winterbaume.
//!
//! Run with:
//!   cargo run --example trustedadvisor --package winterbaume

use aws_sdk_trustedadvisor::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_trustedadvisor::TrustedAdvisorService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TrustedAdvisorService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_trustedadvisor::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_trustedadvisor::Client::new(&config);
    let resp = client
        .list_recommendations()
        .send()
        .await
        .expect("list_recommendations should succeed");
    println!(
        "Trusted Advisor recommendations: {}",
        resp.recommendation_summaries().len()
    );
}

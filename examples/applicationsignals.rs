//! Example: CloudWatch Application Signals
//!
//! Demonstrates using aws-sdk-applicationsignals with winterbaume.
//!
//! Run with:
//!   cargo run --example applicationsignals --package winterbaume

use aws_sdk_applicationsignals::config::BehaviorVersion;
use aws_smithy_types::DateTime;
use winterbaume_applicationsignals::ApplicationSignalsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationSignalsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationsignals::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_applicationsignals::Client::new(&config);

    let resp = client
        .list_services()
        .start_time(DateTime::from_secs(0))
        .end_time(DateTime::from_secs(1000))
        .send()
        .await
        .expect("list_services should succeed");
    println!(
        "Application Signals services: {}",
        resp.service_summaries().len()
    );
}

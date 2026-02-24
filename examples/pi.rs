//! Example: Performance Insights
//!
//! Demonstrates using aws-sdk-pi with winterbaume.
//!
//! Run with:
//!   cargo run --example pi --package winterbaume

use aws_sdk_pi::config::BehaviorVersion;
use aws_sdk_pi::types::ServiceType;
use winterbaume_core::MockAws;
use winterbaume_pi::PiService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(PiService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pi::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pi::Client::new(&config);
    let create = client
        .create_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-DEMO")
        .start_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .end_time(aws_smithy_types::DateTime::from_secs(1_700_003_600))
        .send()
        .await
        .expect("create_performance_analysis_report should succeed");
    println!(
        "Created analysis report: {}",
        create.analysis_report_id().unwrap_or_default()
    );
}

//! Example: SageMaker Metrics
//!
//! Demonstrates using aws-sdk-sagemakermetrics with winterbaume.
//!
//! Run with:
//!   cargo run --example sagemakermetrics --package winterbaume-examples

use aws_sdk_sagemakermetrics::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sagemakermetrics::SageMakerMetricsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SageMakerMetricsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemakermetrics::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sagemakermetrics::Client::new(&config);

    // SageMaker Metrics requires training job or experiment run context.
    // This example demonstrates client setup for the SageMakerMetrics service.
    println!("SageMakerMetrics client ready. Use batch_put_metrics() to record training metrics.");
    let _client = client;
}

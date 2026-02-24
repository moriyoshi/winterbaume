//! Example: Application Cost Profiler
//!
//! Demonstrates using aws-sdk-applicationcostprofiler with winterbaume.
//!
//! Run with:
//!   cargo run --example applicationcostprofiler --package winterbaume-examples

use aws_sdk_applicationcostprofiler::config::BehaviorVersion;
use winterbaume_applicationcostprofiler::ApplicationCostProfilerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationCostProfilerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationcostprofiler::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationcostprofiler::Client::new(&config);

    let resp = client
        .list_report_definitions()
        .send()
        .await
        .expect("list_report_definitions should succeed");
    println!(
        "ApplicationCostProfiler reports: {}",
        resp.report_definitions().len()
    );
}

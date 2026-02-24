//! Example: Cost and Usage Report
//!
//! Demonstrates using aws-sdk-costandusagereport with winterbaume.
//!
//! Run with:
//!   cargo run --example costandusagereport --package winterbaume

use aws_sdk_costandusagereport::config::BehaviorVersion;
use aws_sdk_costandusagereport::types::{
    AdditionalArtifact, CompressionFormat, ReportFormat, SchemaElement, TimeUnit,
};
use winterbaume_core::MockAws;
use winterbaume_costandusagereport::CostAndUsageReportService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CostAndUsageReportService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costandusagereport::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_costandusagereport::Client::new(&config);
    let definition = aws_sdk_costandusagereport::types::ReportDefinition::builder()
        .report_name("daily-cur")
        .time_unit(TimeUnit::Daily)
        .format(ReportFormat::Csv)
        .compression(CompressionFormat::Gzip)
        .additional_schema_elements(SchemaElement::Resources)
        .s3_bucket("my-bucket")
        .s3_prefix("reports/")
        .s3_region("us-east-1".into())
        .additional_artifacts(AdditionalArtifact::Athena)
        .build()
        .expect("definition");
    client
        .put_report_definition()
        .report_definition(definition)
        .send()
        .await
        .expect("put_report_definition should succeed");
    let resp = client
        .describe_report_definitions()
        .send()
        .await
        .expect("describe");
    for d in resp.report_definitions() {
        println!("Report: {}", d.report_name());
    }
}

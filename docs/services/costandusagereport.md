# winterbaume-costandusagereport

AWS Cost and Usage Report service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cost and Usage Report Service |
| AWS model | `cost-and-usage-report-service` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 7/7 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/7 operations (0.0%) |
| moto coverage | 0/7 operations (0.0%) |
| floci coverage | 0/7 operations (0.0%) |
| kumo coverage | 0/7 operations (0.0%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws costandusagereport help
```

## Example

```rust
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
```

## Implemented APIs (7)

- `DeleteReportDefinition`
- `DescribeReportDefinitions`
- `ListTagsForResource`
- `ModifyReportDefinition`
- `PutReportDefinition`
- `TagResource`
- `UntagResource`

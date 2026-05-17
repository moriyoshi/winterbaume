# winterbaume-pi

AWS Performance Insights service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Performance Insights |
| AWS model | `pi` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 9/13 operations (69.2%) |
| stubs (routed, returns empty/default) | 4/13 operations (30.8%) |
| moto coverage | 0/13 operations (0.0%) |
| floci coverage | 0/13 operations (0.0%) |
| kumo coverage | 0/13 operations (0.0%) |
| Coverage report date | 2026-05-17 |

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
aws pi help
```

## Example

```rust
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
```

## Implemented APIs (9)

- `CreatePerformanceAnalysisReport`
- `DeletePerformanceAnalysisReport`
- `GetPerformanceAnalysisReport`
- `ListAvailableResourceDimensions`
- `ListAvailableResourceMetrics`
- `ListPerformanceAnalysisReports`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Stubbed APIs (4) &mdash; routed but return an empty/default response</summary>

- `DescribeDimensionKeys`
- `GetDimensionKeyDetails`
- `GetResourceMetadata`
- `GetResourceMetrics`

</details>

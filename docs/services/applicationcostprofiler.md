# winterbaume-applicationcostprofiler

AWS Application Cost Profiler service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Application Cost Profiler |
| AWS model | `applicationcostprofiler` |
| Protocol | restJson1 |
| winterbaume coverage | 6/6 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 0/6 operations (0.0%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
| fakecloud coverage | 0/6 operations (0.0%) |
| Coverage report date | 2026-07-03 |

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
aws applicationcostprofiler help
```

## Example

```rust
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
```

## Implemented APIs (6)

- `DeleteReportDefinition`
- `GetReportDefinition`
- `ImportApplicationUsage`
- `ListReportDefinitions`
- `PutReportDefinition`
- `UpdateReportDefinition`

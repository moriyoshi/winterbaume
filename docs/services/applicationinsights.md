# winterbaume-applicationinsights

AWS Application Insights service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Application Insights |
| AWS model | `application-insights` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 33/33 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/33 operations (0.0%) |
| moto coverage | 0/33 operations (0.0%) |
| floci coverage | 0/33 operations (0.0%) |
| kumo coverage | 0/33 operations (0.0%) |
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
aws applicationinsights help
```

## Example

```rust
use aws_sdk_applicationinsights::config::BehaviorVersion;
use winterbaume_applicationinsights::ApplicationInsightsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationInsightsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationinsights::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationinsights::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!(
        "Application Insights applications: {}",
        resp.application_info_list().len()
    );
}
```

## Implemented APIs (33)

- `AddWorkload`
- `CreateApplication`
- `CreateComponent`
- `CreateLogPattern`
- `DeleteApplication`
- `DeleteComponent`
- `DeleteLogPattern`
- `DescribeApplication`
- `DescribeComponent`
- `DescribeComponentConfiguration`
- `DescribeComponentConfigurationRecommendation`
- `DescribeLogPattern`
- `DescribeObservation`
- `DescribeProblem`
- `DescribeProblemObservations`
- `DescribeWorkload`
- `ListApplications`
- `ListComponents`
- `ListConfigurationHistory`
- `ListLogPatternSets`
- `ListLogPatterns`
- `ListProblems`
- `ListTagsForResource`
- `ListWorkloads`
- `RemoveWorkload`
- `TagResource`
- `UntagResource`
- `UpdateApplication`
- `UpdateComponent`
- `UpdateComponentConfiguration`
- `UpdateLogPattern`
- `UpdateProblem`
- `UpdateWorkload`

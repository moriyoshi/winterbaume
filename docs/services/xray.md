# winterbaume-xray

X-Ray service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | X-Ray |
| AWS model | `xray` |
| Protocol | restJson1 |
| winterbaume coverage | 34/38 operations (89.5%) |
| stubs (routed, returns empty/default) | 4/38 operations (10.5%) |
| moto coverage | 0/38 operations (0.0%) |
| floci coverage | 0/38 operations (0.0%) |
| kumo coverage | 6/38 operations (15.8%) |
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
aws xray get-groups
```

## Example

```rust
use aws_sdk_xray::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_xray::XRayService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(XRayService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_xray::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_xray::Client::new(&config);

    let resp = client
        .get_groups()
        .send()
        .await
        .expect("get_groups should succeed");
    println!("X-Ray groups: {}", resp.groups().len());
}
```

## Implemented APIs (34)

- `BatchGetTraces`
- `CancelTraceRetrieval`
- `CreateGroup`
- `CreateSamplingRule`
- `DeleteGroup`
- `DeleteResourcePolicy`
- `DeleteSamplingRule`
- `GetEncryptionConfig`
- `GetGroup`
- `GetGroups`
- `GetIndexingRules`
- `GetRetrievedTracesGraph`
- `GetSamplingRules`
- `GetSamplingStatisticSummaries`
- `GetSamplingTargets`
- `GetServiceGraph`
- `GetTimeSeriesServiceStatistics`
- `GetTraceGraph`
- `GetTraceSegmentDestination`
- `GetTraceSummaries`
- `ListResourcePolicies`
- `ListRetrievedTraces`
- `ListTagsForResource`
- `PutEncryptionConfig`
- `PutResourcePolicy`
- `PutTelemetryRecords`
- `PutTraceSegments`
- `StartTraceRetrieval`
- `TagResource`
- `UntagResource`
- `UpdateGroup`
- `UpdateIndexingRule`
- `UpdateSamplingRule`
- `UpdateTraceSegmentDestination`

<details><summary>Stubbed APIs (4) &mdash; routed but return an empty/default response</summary>

- `GetInsight`
- `GetInsightEvents`
- `GetInsightImpactGraph`
- `GetInsightSummaries`

</details>

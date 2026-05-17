# winterbaume-sagemakermetrics

SageMaker Metrics service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SageMaker Metrics |
| AWS model | `sagemaker-metrics` |
| Protocol | restJson1 |
| winterbaume coverage | 2/2 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/2 operations (0.0%) |
| moto coverage | 1/2 operations (50.0%) |
| floci coverage | 0/2 operations (0.0%) |
| kumo coverage | 0/2 operations (0.0%) |
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
aws sagemaker-metrics batch-get-metrics --metric-queries MetricName=loss,ResourceArn=arn:aws:sagemaker:us-east-1:123456789012:experiment-trial/my-trial,MetricStat=Max,Period=OneMinute,XAxisType=TIMESTAMP
```

## Example

```rust
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
```

## Implemented APIs (2)

- `BatchGetMetrics`
- `BatchPutMetrics`

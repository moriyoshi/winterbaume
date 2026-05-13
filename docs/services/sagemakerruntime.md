# winterbaume-sagemakerruntime

SageMaker Runtime service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SageMaker Runtime |
| AWS model | `sagemaker-runtime` |
| Protocol | restJson1 |
| winterbaume coverage | 3/3 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/3 operations (0.0%) |
| moto coverage | 2/3 operations (66.7%) |
| floci coverage | 0/3 operations (0.0%) |
| kumo coverage | 0/3 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws sagemaker-runtime invoke-endpoint --endpoint-name my-endpoint --body '{}' /dev/stdout
```

## Example

```rust
use aws_sdk_sagemakerruntime::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sagemakerruntime::SageMakerRuntimeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SageMakerRuntimeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemakerruntime::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sagemakerruntime::Client::new(&config);

    // SageMaker Runtime invokes deployed endpoints.
    // This example demonstrates client setup for the SageMakerRuntime service.
    println!("SageMakerRuntime client ready. Use invoke_endpoint() to call a deployed model.");
    let _client = client;
}
```

## Implemented APIs (3)

- `InvokeEndpoint`
- `InvokeEndpointAsync`
- `InvokeEndpointWithResponseStream`

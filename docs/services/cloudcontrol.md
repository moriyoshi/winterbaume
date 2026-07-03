# winterbaume-cloudcontrol

Cloud Control API service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cloud Control API |
| AWS model | `cloudcontrol` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 8/8 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/8 operations (0.0%) |
| moto coverage | 0/8 operations (0.0%) |
| floci coverage | 0/8 operations (0.0%) |
| kumo coverage | 6/8 operations (75.0%) |
| fakecloud coverage | 8/8 operations (100.0%) |
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
aws cloudcontrol help
```

## Example

```rust
use aws_sdk_cloudcontrol::config::BehaviorVersion;
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudControlService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudcontrol::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudcontrol::Client::new(&config);

    let resp = client
        .list_resources()
        .type_name("AWS::S3::Bucket")
        .send()
        .await
        .expect("list_resources should succeed");
    println!("Cloud Control API: {:?}", resp.resource_descriptions());
}
```

## Implemented APIs (8)

- `CancelResourceRequest`
- `CreateResource`
- `DeleteResource`
- `GetResource`
- `GetResourceRequestStatus`
- `ListResourceRequests`
- `ListResources`
- `UpdateResource`

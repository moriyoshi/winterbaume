# winterbaume-mediastore

MediaStore service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MediaStore |
| AWS model | `mediastore` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 11/21 operations (52.4%) |
| stubs (routed, returns empty/default) | 0/21 operations (0.0%) |
| moto coverage | 11/21 operations (52.4%) |
| floci coverage | 0/21 operations (0.0%) |
| kumo coverage | 0/21 operations (0.0%) |
| fakecloud coverage | 0/21 operations (0.0%) |
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
aws mediastore list-containers
```

## Example

```rust
use aws_sdk_mediastore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediastore::MediaStoreService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaStoreService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediastore::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediastore::Client::new(&config);

    let resp = client
        .list_containers()
        .send()
        .await
        .expect("list_containers should succeed");
    println!("MediaStore containers: {}", resp.containers().len());
}
```

## Implemented APIs (11)

- `CreateContainer`
- `DeleteContainer`
- `DescribeContainer`
- `GetContainerPolicy`
- `GetLifecyclePolicy`
- `GetMetricPolicy`
- `ListContainers`
- `ListTagsForResource`
- `PutContainerPolicy`
- `PutLifecyclePolicy`
- `PutMetricPolicy`

<details><summary>Not yet implemented APIs (10)</summary>

- `DeleteContainerPolicy`
- `DeleteCorsPolicy`
- `DeleteLifecyclePolicy`
- `DeleteMetricPolicy`
- `GetCorsPolicy`
- `PutCorsPolicy`
- `StartAccessLogging`
- `StopAccessLogging`
- `TagResource`
- `UntagResource`

</details>

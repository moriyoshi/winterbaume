# winterbaume-mediastoredata

MediaStore Data Plane service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MediaStore Data |
| AWS model | `mediastore-data` |
| Protocol | restJson1 |
| winterbaume coverage | 5/5 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/5 operations (0.0%) |
| moto coverage | 4/5 operations (80.0%) |
| floci coverage | 0/5 operations (0.0%) |
| kumo coverage | 0/5 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws mediastore-data list-items --endpoint-url http://localhost:5555
```

## Example

```rust
use aws_sdk_mediastoredata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediastoredata::MediaStoreDataService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaStoreDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediastoredata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediastoredata::Client::new(&config);

    // MediaStore Data requires a container endpoint URL.
    // This example demonstrates client setup for the MediaStoreData service.
    println!("MediaStoreData client ready. Use list_items() to list objects in a container.");
    let _client = client;
}
```

## Implemented APIs (5)

- `DeleteObject`
- `DescribeObject`
- `GetObject`
- `ListItems`
- `PutObject`

# winterbaume-keyspaces

Amazon Keyspaces service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Keyspaces |
| AWS model | `keyspaces` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 19/19 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 0/19 operations (0.0%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws keyspaces help
```

## Example

```rust
use aws_sdk_keyspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_keyspaces::KeyspacesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KeyspacesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_keyspaces::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_keyspaces::Client::new(&config);

    let resp = client
        .list_keyspaces()
        .send()
        .await
        .expect("list_keyspaces should succeed");
    println!("Keyspaces: {:?}", resp.keyspaces());
}
```

## Implemented APIs (19)

- `CreateKeyspace`
- `CreateTable`
- `CreateType`
- `DeleteKeyspace`
- `DeleteTable`
- `DeleteType`
- `GetKeyspace`
- `GetTable`
- `GetTableAutoScalingSettings`
- `GetType`
- `ListKeyspaces`
- `ListTables`
- `ListTagsForResource`
- `ListTypes`
- `RestoreTable`
- `TagResource`
- `UntagResource`
- `UpdateKeyspace`
- `UpdateTable`

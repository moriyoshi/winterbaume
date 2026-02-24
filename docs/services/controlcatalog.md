# winterbaume-controlcatalog

AWS Control Catalog service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Control Catalog |
| AWS model | `controlcatalog` |
| Protocol | restJson1 |
| winterbaume coverage | 6/6 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 0/6 operations (0.0%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws controlcatalog help
```

## Example

```rust
use aws_sdk_controlcatalog::config::BehaviorVersion;
use winterbaume_controlcatalog::ControlCatalogService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ControlCatalogService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_controlcatalog::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_controlcatalog::Client::new(&config);
    let resp = client.list_controls().send().await.expect("list_controls");
    for c in resp.controls() {
        println!("Control: {:?}", c.arn());
    }
}
```

## Implemented APIs (6)

- `GetControl`
- `ListCommonControls`
- `ListControlMappings`
- `ListControls`
- `ListDomains`
- `ListObjectives`

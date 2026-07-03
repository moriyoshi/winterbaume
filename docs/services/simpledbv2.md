# winterbaume-simpledbv2

SimpleDB v2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SimpleDB v2 |
| AWS model | `simpledbv2` |
| Protocol | restJson1 |
| winterbaume coverage | 3/3 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/3 operations (0.0%) |
| moto coverage | 0/3 operations (0.0%) |
| floci coverage | 0/3 operations (0.0%) |
| kumo coverage | 0/3 operations (0.0%) |
| fakecloud coverage | 0/3 operations (0.0%) |
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
aws sdb list-domains
```

## Example

```rust
use aws_sdk_simpledbv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_simpledbv2::SimpleDbV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SimpleDbV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simpledbv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simpledbv2::Client::new(&config);

    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    println!(
        "SimpleDB export summaries: {}",
        resp.export_summaries().len()
    );
}
```

## Implemented APIs (3)

- `GetExport`
- `ListExports`
- `StartDomainExport`

# winterbaume-rbin

AWS Recycle Bin service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Recycle Bin |
| AWS model | `rbin` |
| Protocol | restJson1 |
| winterbaume coverage | 9/10 operations (90.0%) |
| stubs (routed, returns empty/default) | 0/10 operations (0.0%) |
| moto coverage | 0/10 operations (0.0%) |
| floci coverage | 0/10 operations (0.0%) |
| kumo coverage | 0/10 operations (0.0%) |
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
aws rbin help
```

## Example

```rust
use aws_sdk_rbin::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rbin::RbinService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(RbinService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rbin::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rbin::Client::new(&config);

    let resp = client
        .list_rules()
        .resource_type(aws_sdk_rbin::types::ResourceType::EbsSnapshot)
        .send()
        .await
        .expect("list_rules should succeed");
    println!("Recycle Bin rules: {}", resp.rules().len());
}
```

## Implemented APIs (9)

- `CreateRule`
- `DeleteRule`
- `GetRule`
- `ListRules`
- `LockRule`
- `TagResource`
- `UnlockRule`
- `UntagResource`
- `UpdateRule`

<details><summary>Not yet implemented APIs (1)</summary>

- `ListTagsForResource`

</details>

# winterbaume-ebs

Amazon EBS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EBS |
| AWS model | `ebs` |
| Protocol | restJson1 |
| winterbaume coverage | 6/6 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 6/6 operations (100.0%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
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
aws ebs list-changed-blocks --first-snapshot-id snap-000000000000 --second-snapshot-id snap-000000000001
```

## Example

```rust
use aws_sdk_ebs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ebs::EbsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EbsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ebs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ebs::Client::new(&config);

    // EBS Direct APIs operate on snapshot blocks; a snapshot ID is required.
    // This example demonstrates client setup for the EBS service.
    println!("EBS client ready. Use start_snapshot() to begin a new snapshot.");
    let _client = client;
}
```

## Implemented APIs (6)

- `CompleteSnapshot`
- `GetSnapshotBlock`
- `ListChangedBlocks`
- `ListSnapshotBlocks`
- `PutSnapshotBlock`
- `StartSnapshot`

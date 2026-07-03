# winterbaume-bcmdataexports

AWS BCM Data Exports service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | BCM Data Exports |
| AWS model | `bcm-data-exports` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 12/12 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 0/12 operations (0.0%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 0/12 operations (0.0%) |
| fakecloud coverage | 0/12 operations (0.0%) |
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
aws bcmdataexports help
```

## Example

```rust
use aws_sdk_bcmdataexports::config::BehaviorVersion;
use winterbaume_bcmdataexports::BcmDataExportsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BcmDataExportsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmdataexports::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bcmdataexports::Client::new(&config);

    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    println!("BCM data exports: {}", resp.exports().len());
}
```

## Implemented APIs (12)

- `CreateExport`
- `DeleteExport`
- `GetExecution`
- `GetExport`
- `GetTable`
- `ListExecutions`
- `ListExports`
- `ListTables`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateExport`

# winterbaume-timestreamwrite

Amazon Timestream Write service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Timestream Write |
| AWS model | `timestream-write` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 19/19 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 15/19 operations (78.9%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
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
aws timestream-write list-databases
```

## Example

```rust
use aws_sdk_timestreamwrite::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreamwrite::TimestreamWriteService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TimestreamWriteService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreamwrite::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_timestreamwrite::Client::new(&config);

    let resp = client
        .list_databases()
        .send()
        .await
        .expect("list_databases should succeed");
    println!("Timestream Write databases: {}", resp.databases().len());
}
```

## Implemented APIs (19)

- `CreateBatchLoadTask`
- `CreateDatabase`
- `CreateTable`
- `DeleteDatabase`
- `DeleteTable`
- `DescribeBatchLoadTask`
- `DescribeDatabase`
- `DescribeEndpoints`
- `DescribeTable`
- `ListBatchLoadTasks`
- `ListDatabases`
- `ListTables`
- `ListTagsForResource`
- `ResumeBatchLoadTask`
- `TagResource`
- `UntagResource`
- `UpdateDatabase`
- `UpdateTable`
- `WriteRecords`

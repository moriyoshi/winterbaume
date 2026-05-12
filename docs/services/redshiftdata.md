# winterbaume-redshiftdata

Amazon Redshift Data API service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Redshift Data |
| AWS model | `redshift-data` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 11/11 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/11 operations (0.0%) |
| moto coverage | 4/11 operations (36.4%) |
| floci coverage | 0/11 operations (0.0%) |
| kumo coverage | 0/11 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws redshift-data list-statements
```

## Example

```rust
use aws_sdk_redshiftdata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_redshiftdata::RedshiftDataService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RedshiftDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_redshiftdata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_redshiftdata::Client::new(&config);

    let resp = client
        .list_databases()
        .workgroup_name("default")
        .database("dev")
        .send()
        .await
        .expect("list_databases should succeed");
    println!("Redshift databases: {}", resp.databases().len());
}
```

## Implemented APIs (11)

- `BatchExecuteStatement`
- `CancelStatement`
- `DescribeStatement`
- `DescribeTable`
- `ExecuteStatement`
- `GetStatementResult`
- `GetStatementResultV2`
- `ListDatabases`
- `ListSchemas`
- `ListStatements`
- `ListTables`

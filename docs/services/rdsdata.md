# winterbaume-rdsdata

RDS Data Service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | RDS Data |
| AWS model | `rds-data` |
| Protocol | restJson1 |
| winterbaume coverage | 6/6 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 1/6 operations (16.7%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws rds-data execute-statement --resource-arn arn:aws:rds:us-east-1:123456789012:cluster:my-cluster --secret-arn arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret --sql 'SELECT 1'
```

## Example

```rust
use aws_sdk_rdsdata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rdsdata::RdsDataService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RdsDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rdsdata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rdsdata::Client::new(&config);

    // RDS Data requires an Aurora cluster ARN and secret ARN.
    // This example demonstrates client setup for the RDS Data service.
    println!("RDS Data client ready. Use execute_statement() to run SQL.");
    let _client = client;
}
```

## Implemented APIs (6)

- `BatchExecuteStatement`
- `BeginTransaction`
- `CommitTransaction`
- `ExecuteSql`
- `ExecuteStatement`
- `RollbackTransaction`

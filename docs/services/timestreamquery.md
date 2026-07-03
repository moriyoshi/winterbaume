# winterbaume-timestreamquery

Timestream Query service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Timestream Query |
| AWS model | `timestream-query` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 15/15 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/15 operations (0.0%) |
| moto coverage | 6/15 operations (40.0%) |
| floci coverage | 0/15 operations (0.0%) |
| kumo coverage | 0/15 operations (0.0%) |
| fakecloud coverage | 0/15 operations (0.0%) |
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
aws timestream-query list-scheduled-queries
```

## Example

```rust
use aws_sdk_timestreamquery::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreamquery::TimestreamQueryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TimestreamQueryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreamquery::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_timestreamquery::Client::new(&config);

    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");
    println!(
        "Timestream scheduled queries: {}",
        resp.scheduled_queries().len()
    );
}
```

## Implemented APIs (15)

- `CancelQuery`
- `CreateScheduledQuery`
- `DeleteScheduledQuery`
- `DescribeAccountSettings`
- `DescribeEndpoints`
- `DescribeScheduledQuery`
- `ExecuteScheduledQuery`
- `ListScheduledQueries`
- `ListTagsForResource`
- `PrepareQuery`
- `Query`
- `TagResource`
- `UntagResource`
- `UpdateAccountSettings`
- `UpdateScheduledQuery`

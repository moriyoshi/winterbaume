# winterbaume-dynamodbstreams

Amazon DynamoDB Streams service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | DynamoDB Streams |
| AWS model | `dynamodb-streams` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 4/4 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/4 operations (0.0%) |
| moto coverage | 0/4 operations (0.0%) |
| floci coverage | 0/4 operations (0.0%) |
| kumo coverage | 0/4 operations (0.0%) |
| fakecloud coverage | 4/4 operations (100.0%) |
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
aws dynamodbstreams list-streams
```

## Example

```rust
use aws_sdk_dynamodbstreams::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dynamodbstreams::DynamoDbStreamsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DynamoDbStreamsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodbstreams::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dynamodbstreams::Client::new(&config);

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");
    println!("DynamoDB streams: {}", resp.streams().len());
}
```

## Implemented APIs (4)

- `DescribeStream`
- `GetRecords`
- `GetShardIterator`
- `ListStreams`

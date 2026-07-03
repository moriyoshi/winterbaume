# winterbaume-kinesis

Kinesis service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Kinesis |
| AWS model | `kinesis` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 38/39 operations (97.4%) |
| stubs (routed, returns empty/default) | 0/39 operations (0.0%) |
| moto coverage | 31/39 operations (79.5%) |
| floci coverage | 0/39 operations (0.0%) |
| kumo coverage | 10/39 operations (25.6%) |
| fakecloud coverage | 39/39 operations (100.0%) |
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
aws kinesis list-streams
```

## Example

```rust
use aws_sdk_kinesis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesis::KinesisService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesis::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kinesis::Client::new(&config);

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");
    println!("Kinesis streams: {}", resp.stream_names().len());
}
```

## Implemented APIs (38)

- `AddTagsToStream`
- `CreateStream`
- `DecreaseStreamRetentionPeriod`
- `DeleteResourcePolicy`
- `DeleteStream`
- `DeregisterStreamConsumer`
- `DescribeAccountSettings`
- `DescribeLimits`
- `DescribeStream`
- `DescribeStreamConsumer`
- `DescribeStreamSummary`
- `DisableEnhancedMonitoring`
- `EnableEnhancedMonitoring`
- `GetRecords`
- `GetResourcePolicy`
- `GetShardIterator`
- `IncreaseStreamRetentionPeriod`
- `ListShards`
- `ListStreamConsumers`
- `ListStreams`
- `ListTagsForResource`
- `ListTagsForStream`
- `MergeShards`
- `PutRecord`
- `PutRecords`
- `PutResourcePolicy`
- `RegisterStreamConsumer`
- `RemoveTagsFromStream`
- `SplitShard`
- `StartStreamEncryption`
- `StopStreamEncryption`
- `TagResource`
- `UntagResource`
- `UpdateAccountSettings`
- `UpdateMaxRecordSize`
- `UpdateShardCount`
- `UpdateStreamMode`
- `UpdateStreamWarmThroughput`

<details><summary>Not yet implemented APIs (1)</summary>

- `SubscribeToShard` (implemented by fakecloud)

</details>

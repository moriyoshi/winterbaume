# winterbaume-firehose

Firehose service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Firehose |
| AWS model | `firehose` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 12/12 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 12/12 operations (100.0%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 7/12 operations (58.3%) |
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
aws firehose list-delivery-streams
```

## Example

```rust
use aws_sdk_firehose::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_firehose::FirehoseService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(FirehoseService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_firehose::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_firehose::Client::new(&config);

    let resp = client
        .list_delivery_streams()
        .send()
        .await
        .expect("list_delivery_streams should succeed");
    println!(
        "Firehose delivery streams: {}",
        resp.delivery_stream_names().len()
    );
}
```

## Implemented APIs (12)

- `CreateDeliveryStream`
- `DeleteDeliveryStream`
- `DescribeDeliveryStream`
- `ListDeliveryStreams`
- `ListTagsForDeliveryStream`
- `PutRecord`
- `PutRecordBatch`
- `StartDeliveryStreamEncryption`
- `StopDeliveryStreamEncryption`
- `TagDeliveryStream`
- `UntagDeliveryStream`
- `UpdateDestination`

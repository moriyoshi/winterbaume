# winterbaume-sqs

SQS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SQS |
| AWS model | `sqs` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 23/23 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/23 operations (0.0%) |
| moto coverage | 20/23 operations (87.0%) |
| floci coverage | 0/23 operations (0.0%) |
| kumo coverage | 14/23 operations (60.9%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws sqs list-queues
```

## Example

```rust
use aws_sdk_sqs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sqs::SqsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SqsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sqs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sqs::Client::new(&config);

    client
        .create_queue()
        .queue_name("example-queue")
        .send()
        .await
        .expect("create_queue should succeed");
    let resp = client
        .list_queues()
        .send()
        .await
        .expect("list_queues should succeed");
    println!("SQS queues: {:?}", resp.queue_urls());
}
```

## Implemented APIs (23)

- `AddPermission`
- `CancelMessageMoveTask`
- `ChangeMessageVisibility`
- `ChangeMessageVisibilityBatch`
- `CreateQueue`
- `DeleteMessage`
- `DeleteMessageBatch`
- `DeleteQueue`
- `GetQueueAttributes`
- `GetQueueUrl`
- `ListDeadLetterSourceQueues`
- `ListMessageMoveTasks`
- `ListQueueTags`
- `ListQueues`
- `PurgeQueue`
- `ReceiveMessage`
- `RemovePermission`
- `SendMessage`
- `SendMessageBatch`
- `SetQueueAttributes`
- `StartMessageMoveTask`
- `TagQueue`
- `UntagQueue`

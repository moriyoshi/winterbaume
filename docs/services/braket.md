# winterbaume-braket

AWS Braket service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Braket |
| AWS model | `braket` |
| Protocol | restJson1 |
| winterbaume coverage | 12/17 operations (70.6%) |
| stubs (routed, returns empty/default) | 0/17 operations (0.0%) |
| moto coverage | 0/17 operations (0.0%) |
| floci coverage | 0/17 operations (0.0%) |
| kumo coverage | 0/17 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws braket help
```

## Example

```rust
use aws_sdk_braket::config::BehaviorVersion;
use winterbaume_braket::BraketService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BraketService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_braket::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_braket::Client::new(&config);
    let resp = client
        .search_devices()
        .send()
        .await
        .expect("search_devices");
    for d in resp.devices() {
        println!("Device: {} ({})", d.device_arn(), d.device_name());
    }
}
```

## Implemented APIs (12)

- `CancelJob`
- `CreateJob`
- `CreateSpendingLimit`
- `DeleteSpendingLimit`
- `GetDevice`
- `GetJob`
- `SearchDevices`
- `SearchJobs`
- `SearchSpendingLimits`
- `TagResource`
- `UntagResource`
- `UpdateSpendingLimit`

<details><summary>Not yet implemented APIs (5)</summary>

- `CancelQuantumTask`
- `CreateQuantumTask`
- `GetQuantumTask`
- `ListTagsForResource`
- `SearchQuantumTasks`

</details>

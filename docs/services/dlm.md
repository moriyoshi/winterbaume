# winterbaume-dlm

AWS Data Lifecycle Manager service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | DLM |
| AWS model | `dlm` |
| Protocol | restJson1 |
| winterbaume coverage | 2/8 operations (25.0%) |
| stubs (routed, returns empty/default) | 0/8 operations (0.0%) |
| moto coverage | 0/8 operations (0.0%) |
| floci coverage | 0/8 operations (0.0%) |
| kumo coverage | 5/8 operations (62.5%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws dlm help
```

## Example

```rust
use aws_sdk_dlm::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dlm::DlmService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(DlmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dlm::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dlm::Client::new(&config);

    let resp = client
        .get_lifecycle_policies()
        .send()
        .await
        .expect("get_lifecycle_policies should succeed");
    println!("DLM lifecycle policies: {}", resp.policies().len());
}
```

## Implemented APIs (2)

- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (6)</summary>

- `CreateLifecyclePolicy` (implemented by kumo)
- `DeleteLifecyclePolicy` (implemented by kumo)
- `GetLifecyclePolicies` (implemented by kumo)
- `GetLifecyclePolicy` (implemented by kumo)
- `ListTagsForResource`
- `UpdateLifecyclePolicy` (implemented by kumo)

</details>

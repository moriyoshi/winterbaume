# winterbaume-aiops

AWS AIOps service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AIOps |
| AWS model | `aiops` |
| Protocol | restJson1 |
| winterbaume coverage | 11/11 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/11 operations (0.0%) |
| moto coverage | 0/11 operations (0.0%) |
| floci coverage | 0/11 operations (0.0%) |
| kumo coverage | 0/11 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws aiops help
```

## Example

```rust
use aws_sdk_aiops::config::BehaviorVersion;
use winterbaume_aiops::AIOpsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(AIOpsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_aiops::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_aiops::Client::new(&config);

    let resp = client
        .list_investigation_groups()
        .send()
        .await
        .expect("list_investigation_groups should succeed");
    println!(
        "AIOps investigation groups: {}",
        resp.investigation_groups().len()
    );
}
```

## Implemented APIs (11)

- `CreateInvestigationGroup`
- `DeleteInvestigationGroup`
- `DeleteInvestigationGroupPolicy`
- `GetInvestigationGroup`
- `GetInvestigationGroupPolicy`
- `ListInvestigationGroups`
- `ListTagsForResource`
- `PutInvestigationGroupPolicy`
- `TagResource`
- `UntagResource`
- `UpdateInvestigationGroup`

# winterbaume-personalizeruntime

AWS Personalize Runtime service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Personalize Runtime |
| AWS model | `personalize-runtime` |
| Protocol | restJson1 |
| winterbaume coverage | 3/3 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/3 operations (0.0%) |
| moto coverage | 0/3 operations (0.0%) |
| floci coverage | 0/3 operations (0.0%) |
| kumo coverage | 0/3 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws personalizeruntime help
```

## Example

```rust
use aws_sdk_personalizeruntime::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_personalizeruntime::PersonalizeRuntimeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PersonalizeRuntimeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeruntime::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_personalizeruntime::Client::new(&config);
    let resp = client
        .get_recommendations()
        .campaign_arn("arn:aws:personalize:us-east-1:123:campaign/example")
        .user_id("alice")
        .num_results(3)
        .send()
        .await
        .expect("get_recommendations should succeed");

    for item in resp.item_list() {
        println!(
            "Item: {} (score: {})",
            item.item_id().unwrap_or_default(),
            item.score().unwrap_or(0.0)
        );
    }
}
```

## Implemented APIs (3)

- `GetActionRecommendations`
- `GetPersonalizedRanking`
- `GetRecommendations`

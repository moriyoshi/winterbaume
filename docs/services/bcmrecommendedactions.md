# winterbaume-bcmrecommendedactions

AWS BCM Recommended Actions service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | BCM Recommended Actions |
| AWS model | `bcm-recommended-actions` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 1/1 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/1 operations (0.0%) |
| moto coverage | 0/1 operations (0.0%) |
| floci coverage | 0/1 operations (0.0%) |
| kumo coverage | 0/1 operations (0.0%) |
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
aws bcmrecommendedactions help
```

## Example

```rust
use aws_sdk_bcmrecommendedactions::config::BehaviorVersion;
use winterbaume_bcmrecommendedactions::BcmRecommendedActionsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BcmRecommendedActionsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmrecommendedactions::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_bcmrecommendedactions::Client::new(&config);

    let resp = client
        .list_recommended_actions()
        .send()
        .await
        .expect("list_recommended_actions should succeed");
    println!(
        "BCM recommended actions: {}",
        resp.recommended_actions().len()
    );
}
```

## Implemented APIs (1)

- `ListRecommendedActions`

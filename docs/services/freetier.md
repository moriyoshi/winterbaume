# winterbaume-freetier

AWS Free Tier service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Free Tier |
| AWS model | `freetier` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 5/5 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/5 operations (0.0%) |
| moto coverage | 0/5 operations (0.0%) |
| floci coverage | 0/5 operations (0.0%) |
| kumo coverage | 0/5 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws freetier help
```

## Example

```rust
use aws_sdk_freetier::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_freetier::FreeTierService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(FreeTierService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_freetier::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_freetier::Client::new(&config);
    let resp = client
        .get_account_plan_state()
        .send()
        .await
        .expect("get_account_plan_state should succeed");
    println!(
        "Plan: {:?} ({:?})",
        resp.account_plan_type(),
        resp.account_plan_status()
    );
}
```

## Implemented APIs (5)

- `GetAccountActivity`
- `GetAccountPlanState`
- `GetFreeTierUsage`
- `ListAccountActivities`
- `UpgradeAccountPlan`

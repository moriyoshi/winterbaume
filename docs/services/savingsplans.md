# winterbaume-savingsplans

AWS Savings Plans service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Savings Plans |
| AWS model | `savingsplans` |
| Protocol | restJson1 |
| winterbaume coverage | 10/10 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/10 operations (0.0%) |
| moto coverage | 0/10 operations (0.0%) |
| floci coverage | 0/10 operations (0.0%) |
| kumo coverage | 0/10 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws savingsplans help
```

## Example

```rust
use aws_sdk_savingsplans::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_savingsplans::SavingsPlansService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SavingsPlansService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_savingsplans::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_savingsplans::Client::new(&config);

    let create = client
        .create_savings_plan()
        .savings_plan_offering_id("offering-1")
        .commitment("10.00")
        .send()
        .await
        .expect("create_savings_plan should succeed");
    println!(
        "Created savings plan: {}",
        create.savings_plan_id().unwrap_or_default()
    );

    let resp = client
        .describe_savings_plans()
        .send()
        .await
        .expect("describe_savings_plans should succeed");
    println!("Total savings plans: {}", resp.savings_plans().len());
}
```

## Implemented APIs (10)

- `CreateSavingsPlan`
- `DeleteQueuedSavingsPlan`
- `DescribeSavingsPlanRates`
- `DescribeSavingsPlans`
- `DescribeSavingsPlansOfferingRates`
- `DescribeSavingsPlansOfferings`
- `ListTagsForResource`
- `ReturnSavingsPlan`
- `TagResource`
- `UntagResource`

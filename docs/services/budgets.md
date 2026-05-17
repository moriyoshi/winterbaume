# winterbaume-budgets

Budgets service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Budgets |
| AWS model | `budgets` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 7/26 operations (26.9%) |
| stubs (routed, returns empty/default) | 0/26 operations (0.0%) |
| moto coverage | 7/26 operations (26.9%) |
| floci coverage | 0/26 operations (0.0%) |
| kumo coverage | 0/26 operations (0.0%) |
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
aws budgets describe-budgets --account-id 123456789012
```

## Example

```rust
use aws_sdk_budgets::config::BehaviorVersion;
use winterbaume_budgets::BudgetsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BudgetsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_budgets::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_budgets::Client::new(&config);

    let resp = client
        .describe_budgets()
        .account_id("123456789012")
        .send()
        .await
        .expect("describe_budgets should succeed");
    println!("Budgets: {}", resp.budgets().len());
}
```

## Implemented APIs (7)

- `CreateBudget`
- `CreateNotification`
- `DeleteBudget`
- `DeleteNotification`
- `DescribeBudget`
- `DescribeBudgets`
- `DescribeNotificationsForBudget`

<details><summary>Not yet implemented APIs (19)</summary>

- `CreateBudgetAction`
- `CreateSubscriber`
- `DeleteBudgetAction`
- `DeleteSubscriber`
- `DescribeBudgetAction`
- `DescribeBudgetActionHistories`
- `DescribeBudgetActionsForAccount`
- `DescribeBudgetActionsForBudget`
- `DescribeBudgetNotificationsForAccount`
- `DescribeBudgetPerformanceHistory`
- `DescribeSubscribersForNotification`
- `ExecuteBudgetAction`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateBudget`
- `UpdateBudgetAction`
- `UpdateNotification`
- `UpdateSubscriber`

</details>

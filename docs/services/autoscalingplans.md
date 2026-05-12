# winterbaume-autoscalingplans

AWS Auto Scaling Plans service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Auto Scaling Plans |
| AWS model | `auto-scaling-plans` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 6/6 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 0/6 operations (0.0%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws autoscalingplans help
```

## Example

```rust
use aws_sdk_autoscalingplans::config::BehaviorVersion;
use winterbaume_autoscalingplans::AutoScalingPlansService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AutoScalingPlansService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_autoscalingplans::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_autoscalingplans::Client::new(&config);

    let resp = client
        .describe_scaling_plans()
        .send()
        .await
        .expect("describe_scaling_plans should succeed");
    println!("AutoScalingPlans plans: {}", resp.scaling_plans().len());
}
```

## Implemented APIs (6)

- `CreateScalingPlan`
- `DeleteScalingPlan`
- `DescribeScalingPlanResources`
- `DescribeScalingPlans`
- `GetScalingPlanResourceForecastData`
- `UpdateScalingPlan`

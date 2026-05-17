# winterbaume-applicationautoscaling

Application Auto Scaling service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Application Auto Scaling |
| AWS model | `application-auto-scaling` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 13/14 operations (92.9%) |
| stubs (routed, returns empty/default) | 1/14 operations (7.1%) |
| moto coverage | 9/14 operations (64.3%) |
| floci coverage | 0/14 operations (0.0%) |
| kumo coverage | 0/14 operations (0.0%) |
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
aws application-autoscaling describe-scalable-targets --service-namespace ecs
```

## Example

```rust
use aws_sdk_applicationautoscaling::config::BehaviorVersion;
use winterbaume_applicationautoscaling::ApplicationAutoScalingService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationAutoScalingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationautoscaling::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationautoscaling::Client::new(&config);

    use aws_sdk_applicationautoscaling::types::ServiceNamespace;
    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scalable_targets should succeed");
    println!("Scalable targets: {}", resp.scalable_targets().len());
}
```

## Implemented APIs (13)

- `DeleteScalingPolicy`
- `DeleteScheduledAction`
- `DeregisterScalableTarget`
- `DescribeScalableTargets`
- `DescribeScalingActivities`
- `DescribeScalingPolicies`
- `DescribeScheduledActions`
- `ListTagsForResource`
- `PutScalingPolicy`
- `PutScheduledAction`
- `RegisterScalableTarget`
- `TagResource`
- `UntagResource`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `GetPredictiveScalingForecast`

</details>

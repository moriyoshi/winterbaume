# winterbaume-costoptimizationhub

AWS Cost Optimization Hub service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cost Optimization Hub |
| AWS model | `cost-optimization-hub` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 8/8 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/8 operations (0.0%) |
| moto coverage | 0/8 operations (0.0%) |
| floci coverage | 0/8 operations (0.0%) |
| kumo coverage | 0/8 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws costoptimizationhub help
```

## Example

```rust
use aws_sdk_costoptimizationhub::config::BehaviorVersion;
use aws_sdk_costoptimizationhub::types::EnrollmentStatus;
use winterbaume_core::MockAws;
use winterbaume_costoptimizationhub::CostOptimizationHubService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CostOptimizationHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costoptimizationhub::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_costoptimizationhub::Client::new(&config);
    let resp = client
        .update_enrollment_status()
        .status(EnrollmentStatus::Active)
        .send()
        .await
        .expect("update");
    println!("Enrollment status: {:?}", resp.status());
}
```

## Implemented APIs (8)

- `GetPreferences`
- `GetRecommendation`
- `ListEfficiencyMetrics`
- `ListEnrollmentStatuses`
- `ListRecommendationSummaries`
- `ListRecommendations`
- `UpdateEnrollmentStatus`
- `UpdatePreferences`

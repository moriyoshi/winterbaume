# winterbaume-trustedadvisor

AWS Trusted Advisor service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Trusted Advisor |
| AWS model | `trustedadvisor` |
| Protocol | restJson1 |
| winterbaume coverage | 6/11 operations (54.5%) |
| stubs (routed, returns empty/default) | 4/11 operations (36.4%) |
| moto coverage | 0/11 operations (0.0%) |
| floci coverage | 0/11 operations (0.0%) |
| kumo coverage | 0/11 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws trustedadvisor help
```

## Example

```rust
use aws_sdk_trustedadvisor::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_trustedadvisor::TrustedAdvisorService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TrustedAdvisorService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_trustedadvisor::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_trustedadvisor::Client::new(&config);
    let resp = client
        .list_recommendations()
        .send()
        .await
        .expect("list_recommendations should succeed");
    println!(
        "Trusted Advisor recommendations: {}",
        resp.recommendation_summaries().len()
    );
}
```

## Implemented APIs (6)

- `GetOrganizationRecommendation`
- `GetRecommendation`
- `ListOrganizationRecommendations`
- `ListRecommendations`
- `UpdateOrganizationRecommendationLifecycle`
- `UpdateRecommendationLifecycle`

<details><summary>Stubbed APIs (4) &mdash; routed but return an empty/default response</summary>

- `ListChecks`
- `ListOrganizationRecommendationAccounts`
- `ListOrganizationRecommendationResources`
- `ListRecommendationResources`

</details>

<details><summary>Not yet implemented APIs (1)</summary>

- `BatchUpdateRecommendationResourceExclusion`

</details>

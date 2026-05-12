# winterbaume-resiliencehub

AWS Resilience Hub service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Resilience Hub |
| AWS model | `resiliencehub` |
| Protocol | restJson1 |
| winterbaume coverage | 22/63 operations (34.9%) |
| stubs (routed, returns empty/default) | 0/63 operations (0.0%) |
| moto coverage | 17/63 operations (27.0%) |
| floci coverage | 0/63 operations (0.0%) |
| kumo coverage | 17/63 operations (27.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws resiliencehub list-apps
```

## Example

```rust
use aws_sdk_resiliencehub::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_resiliencehub::ResilienceHubService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ResilienceHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resiliencehub::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_resiliencehub::Client::new(&config);

    let resp = client
        .list_apps()
        .send()
        .await
        .expect("list_apps should succeed");
    println!("Resilience Hub apps: {}", resp.app_summaries().len());
}
```

## Implemented APIs (22)

- `CreateApp`
- `CreateAppVersionAppComponent`
- `CreateAppVersionResource`
- `CreateResiliencyPolicy`
- `DeleteApp`
- `DeleteResiliencyPolicy`
- `DescribeApp`
- `DescribeAppVersionTemplate`
- `DescribeResiliencyPolicy`
- `ImportResourcesToDraftAppVersion`
- `ListAppAssessments`
- `ListAppVersionAppComponents`
- `ListAppVersionResources`
- `ListAppVersions`
- `ListApps`
- `ListResiliencyPolicies`
- `ListSuggestedResiliencyPolicies`
- `ListTagsForResource`
- `PublishAppVersion`
- `TagResource`
- `UntagResource`
- `UpdateResiliencyPolicy`

<details><summary>Not yet implemented APIs (41)</summary>

- `AcceptResourceGroupingRecommendations`
- `AddDraftAppVersionResourceMappings`
- `BatchUpdateRecommendationStatus`
- `CreateRecommendationTemplate`
- `DeleteAppAssessment` (implemented by kumo)
- `DeleteAppInputSource`
- `DeleteAppVersionAppComponent`
- `DeleteAppVersionResource`
- `DeleteRecommendationTemplate`
- `DescribeAppAssessment` (implemented by kumo)
- `DescribeAppVersion`
- `DescribeAppVersionAppComponent`
- `DescribeAppVersionResource`
- `DescribeAppVersionResourcesResolutionStatus`
- `DescribeDraftAppVersionResourcesImportStatus`
- `DescribeMetricsExport`
- `DescribeResourceGroupingRecommendationTask`
- `ListAlarmRecommendations`
- `ListAppAssessmentComplianceDrifts`
- `ListAppAssessmentResourceDrifts`
- `ListAppComponentCompliances`
- `ListAppComponentRecommendations`
- `ListAppInputSources`
- `ListAppVersionResourceMappings`
- `ListMetrics`
- `ListRecommendationTemplates`
- `ListResourceGroupingRecommendations`
- `ListSopRecommendations`
- `ListTestRecommendations`
- `ListUnsupportedAppVersionResources`
- `PutDraftAppVersionTemplate`
- `RejectResourceGroupingRecommendations`
- `RemoveDraftAppVersionResourceMappings`
- `ResolveAppVersionResources`
- `StartAppAssessment` (implemented by kumo)
- `StartMetricsExport`
- `StartResourceGroupingRecommendationTask`
- `UpdateApp` (implemented by kumo)
- `UpdateAppVersion`
- `UpdateAppVersionAppComponent`
- `UpdateAppVersionResource`

</details>

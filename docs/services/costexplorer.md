# winterbaume-costexplorer

AWS Cost Explorer service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cost Explorer |
| AWS model | `cost-explorer` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 22/47 operations (46.8%) |
| stubs (routed, returns empty/default) | 25/47 operations (53.2%) |
| moto coverage | 0/47 operations (0.0%) |
| floci coverage | 0/47 operations (0.0%) |
| kumo coverage | 8/47 operations (17.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws ce list-cost-category-definitions
```

## Example

```rust
use aws_sdk_costexplorer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_costexplorer::CostExplorerService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CostExplorerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costexplorer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_costexplorer::Client::new(&config);

    let resp = client
        .list_cost_category_definitions()
        .send()
        .await
        .expect("list_cost_category_definitions should succeed");
    println!("Cost categories: {}", resp.cost_category_references().len());
}
```

## Implemented APIs (22)

- `CreateAnomalyMonitor`
- `CreateAnomalySubscription`
- `CreateCostCategoryDefinition`
- `DeleteAnomalyMonitor`
- `DeleteAnomalySubscription`
- `DeleteCostCategoryDefinition`
- `DescribeCostCategoryDefinition`
- `GetAnomalyMonitors`
- `GetAnomalySubscriptions`
- `ListCostAllocationTagBackfillHistory`
- `ListCostAllocationTags`
- `ListCostCategoryDefinitions`
- `ListCostCategoryResourceAssociations`
- `ListTagsForResource`
- `ProvideAnomalyFeedback`
- `StartCostAllocationTagBackfill`
- `TagResource`
- `UntagResource`
- `UpdateAnomalyMonitor`
- `UpdateAnomalySubscription`
- `UpdateCostAllocationTagsStatus`
- `UpdateCostCategoryDefinition`

<details><summary>Stubbed APIs (25) &mdash; routed but return an empty/default response</summary>

- `GetAnomalies`
- `GetApproximateUsageRecords`
- `GetCommitmentPurchaseAnalysis`
- `GetCostAndUsage`
- `GetCostAndUsageComparisons`
- `GetCostAndUsageWithResources`
- `GetCostCategories`
- `GetCostComparisonDrivers`
- `GetCostForecast`
- `GetDimensionValues`
- `GetReservationCoverage`
- `GetReservationPurchaseRecommendation`
- `GetReservationUtilization`
- `GetRightsizingRecommendation`
- `GetSavingsPlanPurchaseRecommendationDetails`
- `GetSavingsPlansCoverage`
- `GetSavingsPlansPurchaseRecommendation`
- `GetSavingsPlansUtilization`
- `GetSavingsPlansUtilizationDetails`
- `GetTags`
- `GetUsageForecast`
- `ListCommitmentPurchaseAnalyses`
- `ListSavingsPlansPurchaseRecommendationGeneration`
- `StartCommitmentPurchaseAnalysis`
- `StartSavingsPlansPurchaseRecommendationGeneration`

</details>

# winterbaume-personalize

Amazon Personalize service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Personalize |
| AWS model | `personalize` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 66/71 operations (93.0%) |
| stubs (routed, returns empty/default) | 5/71 operations (7.0%) |
| moto coverage | 4/71 operations (5.6%) |
| floci coverage | 0/71 operations (0.0%) |
| kumo coverage | 0/71 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws personalize list-datasets
```

## Example

```rust
use aws_sdk_personalize::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_personalize::PersonalizeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PersonalizeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalize::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_personalize::Client::new(&config);

    let resp = client
        .list_datasets()
        .send()
        .await
        .expect("list_datasets should succeed");
    println!("Personalize datasets: {}", resp.datasets().len());
}
```

## Implemented APIs (66)

- `CreateBatchInferenceJob`
- `CreateBatchSegmentJob`
- `CreateCampaign`
- `CreateDataDeletionJob`
- `CreateDataset`
- `CreateDatasetExportJob`
- `CreateDatasetGroup`
- `CreateDatasetImportJob`
- `CreateEventTracker`
- `CreateFilter`
- `CreateMetricAttribution`
- `CreateRecommender`
- `CreateSchema`
- `CreateSolution`
- `CreateSolutionVersion`
- `DeleteCampaign`
- `DeleteDataset`
- `DeleteDatasetGroup`
- `DeleteEventTracker`
- `DeleteFilter`
- `DeleteMetricAttribution`
- `DeleteRecommender`
- `DeleteSchema`
- `DeleteSolution`
- `DescribeBatchInferenceJob`
- `DescribeBatchSegmentJob`
- `DescribeCampaign`
- `DescribeDataDeletionJob`
- `DescribeDataset`
- `DescribeDatasetExportJob`
- `DescribeDatasetGroup`
- `DescribeDatasetImportJob`
- `DescribeEventTracker`
- `DescribeFilter`
- `DescribeMetricAttribution`
- `DescribeRecommender`
- `DescribeSchema`
- `DescribeSolution`
- `DescribeSolutionVersion`
- `ListBatchInferenceJobs`
- `ListBatchSegmentJobs`
- `ListCampaigns`
- `ListDataDeletionJobs`
- `ListDatasetExportJobs`
- `ListDatasetGroups`
- `ListDatasetImportJobs`
- `ListDatasets`
- `ListEventTrackers`
- `ListFilters`
- `ListMetricAttributionMetrics`
- `ListMetricAttributions`
- `ListRecommenders`
- `ListSchemas`
- `ListSolutionVersions`
- `ListSolutions`
- `ListTagsForResource`
- `StartRecommender`
- `StopRecommender`
- `StopSolutionVersionCreation`
- `TagResource`
- `UntagResource`
- `UpdateCampaign`
- `UpdateDataset`
- `UpdateMetricAttribution`
- `UpdateRecommender`
- `UpdateSolution`

<details><summary>Stubbed APIs (5) &mdash; routed but return an empty/default response</summary>

- `DescribeAlgorithm`
- `DescribeFeatureTransformation`
- `DescribeRecipe`
- `GetSolutionMetrics`
- `ListRecipes`

</details>

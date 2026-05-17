# winterbaume-forecast

Amazon Forecast service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Forecast |
| AWS model | `forecast` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 5/63 operations (7.9%) |
| stubs (routed, returns empty/default) | 0/63 operations (0.0%) |
| moto coverage | 5/63 operations (7.9%) |
| floci coverage | 0/63 operations (0.0%) |
| kumo coverage | 17/63 operations (27.0%) |
| Coverage report date | 2026-05-16 |

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
aws forecast list-datasets
```

## Example

```rust
use aws_sdk_forecast::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_forecast::ForecastService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ForecastService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_forecast::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_forecast::Client::new(&config);

    let resp = client
        .list_datasets()
        .send()
        .await
        .expect("list_datasets should succeed");
    println!("Forecast datasets: {}", resp.datasets().len());
}
```

## Implemented APIs (5)

- `CreateDatasetGroup`
- `DeleteDatasetGroup`
- `DescribeDatasetGroup`
- `ListDatasetGroups`
- `UpdateDatasetGroup`

<details><summary>Not yet implemented APIs (58)</summary>

- `CreateAutoPredictor`
- `CreateDataset` (implemented by kumo)
- `CreateDatasetImportJob`
- `CreateExplainability`
- `CreateExplainabilityExport`
- `CreateForecast` (implemented by kumo)
- `CreateForecastExportJob`
- `CreateMonitor`
- `CreatePredictor` (implemented by kumo)
- `CreatePredictorBacktestExportJob`
- `CreateWhatIfAnalysis`
- `CreateWhatIfForecast`
- `CreateWhatIfForecastExport`
- `DeleteDataset` (implemented by kumo)
- `DeleteDatasetImportJob`
- `DeleteExplainability`
- `DeleteExplainabilityExport`
- `DeleteForecast` (implemented by kumo)
- `DeleteForecastExportJob`
- `DeleteMonitor`
- `DeletePredictor` (implemented by kumo)
- `DeletePredictorBacktestExportJob`
- `DeleteResourceTree`
- `DeleteWhatIfAnalysis`
- `DeleteWhatIfForecast`
- `DeleteWhatIfForecastExport`
- `DescribeAutoPredictor`
- `DescribeDataset` (implemented by kumo)
- `DescribeDatasetImportJob`
- `DescribeExplainability`
- `DescribeExplainabilityExport`
- `DescribeForecast` (implemented by kumo)
- `DescribeForecastExportJob`
- `DescribeMonitor`
- `DescribePredictor` (implemented by kumo)
- `DescribePredictorBacktestExportJob`
- `DescribeWhatIfAnalysis`
- `DescribeWhatIfForecast`
- `DescribeWhatIfForecastExport`
- `GetAccuracyMetrics`
- `ListDatasetImportJobs`
- `ListDatasets` (implemented by kumo)
- `ListExplainabilities`
- `ListExplainabilityExports`
- `ListForecastExportJobs`
- `ListForecasts` (implemented by kumo)
- `ListMonitorEvaluations`
- `ListMonitors`
- `ListPredictorBacktestExportJobs`
- `ListPredictors` (implemented by kumo)
- `ListTagsForResource`
- `ListWhatIfAnalyses`
- `ListWhatIfForecastExports`
- `ListWhatIfForecasts`
- `ResumeResource`
- `StopResource`
- `TagResource`
- `UntagResource`

</details>

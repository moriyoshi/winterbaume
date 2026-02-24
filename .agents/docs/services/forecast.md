# Amazon Forecast Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provides APIs for creating and managing Amazon Forecast resources.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-forecast/tests/scenario_test.rs`: build a retail demand-forecasting project with dataset groups, datasets, import jobs, predictors/forecasts, exports, and cleanup.
- Backported from `scenario_test.rs`: keep multiple forecasting projects isolated in list and describe operations.
- Scenario insight from EC2: add full state-machine walks for Amazon Forecast Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent time-series forecasting pipelines, dataset ingestion, predictors, forecasts, forecast exports, what-if analysis resources, tags, and asynchronous job state.

## Service Identity and Protocol

- AWS model slug: `forecast`
- AWS SDK for Rust slug: `forecast`
- Model version: `2018-06-26`
- Model file: `vendor/api-models-aws/models/forecast/service/2018-06-26/forecast-2018-06-26.json`
- SDK ID: `forecast`
- Endpoint prefix: `forecast`
- ARN namespace: `forecast`
- CloudFormation name: `Forecast`
- CloudTrail event source: `forecast.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (15), `Create` (14), `Delete` (14), `Describe` (14), `Get` (1), `Resume` (1), `Stop` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAutoPredictor`, `CreateDataset`, `CreateDatasetGroup`, `CreateDatasetImportJob`, `CreateExplainability`, `CreateExplainabilityExport`, `CreateForecast`, `CreateForecastExportJob`, `CreateMonitor`, `CreatePredictor`, `CreatePredictorBacktestExportJob`, `CreateWhatIfAnalysis`, `CreateWhatIfForecast`, `CreateWhatIfForecastExport`, `DeleteDataset`, `DeleteDatasetGroup`, `DeleteDatasetImportJob`, `DeleteExplainability`, `DeleteExplainabilityExport`, `DeleteForecast`, ... (+12).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAutoPredictor`, `DescribeDataset`, `DescribeDatasetGroup`, `DescribeDatasetImportJob`, `DescribeExplainability`, `DescribeExplainabilityExport`, `DescribeForecast`, `DescribeForecastExportJob`, `DescribeMonitor`, `DescribePredictor`, `DescribePredictorBacktestExportJob`, `DescribeWhatIfAnalysis`, `DescribeWhatIfForecast`, `DescribeWhatIfForecastExport`, `GetAccuracyMetrics`, `ListDatasetGroups`, `ListDatasetImportJobs`, `ListDatasets`, `ListExplainabilities`, `ListExplainabilityExports`, ... (+10).
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 46 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateDatasetImportJob`, `CreateExplainabilityExport`, `CreateForecastExportJob`, `CreatePredictorBacktestExportJob`, `CreateWhatIfAnalysis`, `CreateWhatIfForecastExport`, `DeleteDatasetImportJob`, `DeleteExplainabilityExport`, `DeleteForecastExportJob`, `DeletePredictorBacktestExportJob`, `DeleteWhatIfAnalysis`, `DeleteWhatIfForecastExport`, `DescribeDatasetImportJob`, `DescribeExplainabilityExport`, `DescribeForecastExportJob`, `DescribePredictorBacktestExportJob`, `DescribeWhatIfAnalysis`, `DescribeWhatIfForecastExport`, `ListDatasetImportJobs`, `ListExplainabilityExports`, ... (+4).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 63 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListDatasetGroups`, `ListDatasetImportJobs`, `ListDatasets`, `ListExplainabilities`, `ListExplainabilityExports`, `ListForecastExportJobs`, `ListForecasts`, `ListMonitorEvaluations`, `ListMonitors`, `ListPredictorBacktestExportJobs`, `ListPredictors`, `ListTagsForResource`, `ListWhatIfAnalyses`, `ListWhatIfForecastExports`, `ListWhatIfForecasts`
- Traits: `idempotent` (14), `paginated` (14)
- Common required input members in this group: `MonitorArn`, `ResourceArn`

### Create

- Operations: `CreateAutoPredictor`, `CreateDataset`, `CreateDatasetGroup`, `CreateDatasetImportJob`, `CreateExplainability`, `CreateExplainabilityExport`, `CreateForecast`, `CreateForecastExportJob`, `CreateMonitor`, `CreatePredictor`, `CreatePredictorBacktestExportJob`, `CreateWhatIfAnalysis`, `CreateWhatIfForecast`, `CreateWhatIfForecastExport`
- Common required input members in this group: `DataSource`, `DatasetArn`, `DatasetGroupName`, `DatasetImportJobName`, `DatasetName`, `DatasetType`, `Destination`, `Domain`, `ExplainabilityArn`, `ExplainabilityConfig`, `ExplainabilityExportName`, `ExplainabilityName`, `FeaturizationConfig`, `ForecastArn`, `ForecastExportJobName`, `ForecastHorizon`, `ForecastName`, `InputDataConfig`, `MonitorName`, `PredictorArn`, `PredictorBacktestExportJobName`, `PredictorName`, `ResourceArn`, `Schema`, ... (+5)

### Delete

- Operations: `DeleteDataset`, `DeleteDatasetGroup`, `DeleteDatasetImportJob`, `DeleteExplainability`, `DeleteExplainabilityExport`, `DeleteForecast`, `DeleteForecastExportJob`, `DeleteMonitor`, `DeletePredictor`, `DeletePredictorBacktestExportJob`, `DeleteResourceTree`, `DeleteWhatIfAnalysis`, `DeleteWhatIfForecast`, `DeleteWhatIfForecastExport`
- Traits: `idempotent` (14)
- Common required input members in this group: `DatasetArn`, `DatasetGroupArn`, `DatasetImportJobArn`, `ExplainabilityArn`, `ExplainabilityExportArn`, `ForecastArn`, `ForecastExportJobArn`, `MonitorArn`, `PredictorArn`, `PredictorBacktestExportJobArn`, `ResourceArn`, `WhatIfAnalysisArn`, `WhatIfForecastArn`, `WhatIfForecastExportArn`

### Describe

- Operations: `DescribeAutoPredictor`, `DescribeDataset`, `DescribeDatasetGroup`, `DescribeDatasetImportJob`, `DescribeExplainability`, `DescribeExplainabilityExport`, `DescribeForecast`, `DescribeForecastExportJob`, `DescribeMonitor`, `DescribePredictor`, `DescribePredictorBacktestExportJob`, `DescribeWhatIfAnalysis`, `DescribeWhatIfForecast`, `DescribeWhatIfForecastExport`
- Traits: `idempotent` (14)
- Common required input members in this group: `DatasetArn`, `DatasetGroupArn`, `DatasetImportJobArn`, `ExplainabilityArn`, `ExplainabilityExportArn`, `ForecastArn`, `ForecastExportJobArn`, `MonitorArn`, `PredictorArn`, `PredictorBacktestExportJobArn`, `WhatIfAnalysisArn`, `WhatIfForecastArn`, `WhatIfForecastExportArn`

### Get

- Operations: `GetAccuracyMetrics`
- Traits: `idempotent` (1)
- Common required input members in this group: `PredictorArn`

### Resume

- Operations: `ResumeResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`

### Stop

- Operations: `StopResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateDatasetGroup`
- Traits: `idempotent` (1)
- Common required input members in this group: `DatasetArns`, `DatasetGroupArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAutoPredictor` | - | - | `PredictorName` | - | `CreateAutoPredictorResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Creates an Amazon Forecast predictor. Amazon Forecast creates predictors with AutoPredictor, which involves applying the optimal combination of algorithms to each time series in your datasets. |
| `CreateDataset` | - | - | `DatasetName`, `DatasetType`, `Domain`, `Schema` | - | `CreateDatasetResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates an Amazon Forecast dataset. The information about the dataset that you provide helps Forecast understand how to consume the data for model training. |
| `CreateDatasetGroup` | - | - | `DatasetGroupName`, `Domain` | - | `CreateDatasetGroupResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Creates a dataset group, which holds a collection of related datasets. You can add datasets to the dataset group when you create the dataset group, or later by using the UpdateDatasetGroup operation. |
| `CreateDatasetImportJob` | - | - | `DataSource`, `DatasetArn`, `DatasetImportJobName` | - | `CreateDatasetImportJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Imports your training data to an Amazon Forecast dataset. You provide the location of your training data in an Amazon Simple Storage Service (Amazon S3) bucket and the Amazon Resource Name (ARN) of the dataset that you want to import the data to. |
| `CreateExplainability` | - | - | `ExplainabilityConfig`, `ExplainabilityName`, `ResourceArn` | - | `CreateExplainabilityResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Explainability is only available for Forecasts and Predictors generated from an AutoPredictor (CreateAutoPredictor) Creates an Amazon Forecast Explainability. Explainability helps you better understand how the attributes in your datasets impact forecast. |
| `CreateExplainabilityExport` | - | - | `Destination`, `ExplainabilityArn`, `ExplainabilityExportName` | - | `CreateExplainabilityExportResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Exports an Explainability resource created by the CreateExplainability operation. Exported files are exported to an Amazon Simple Storage Service (Amazon S3) bucket. |
| `CreateForecast` | - | - | `ForecastName`, `PredictorArn` | - | `CreateForecastResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Creates a forecast for each item in the `TARGET_TIME_SERIES` dataset that was used to train the predictor. This is known as inference. |
| `CreateForecastExportJob` | - | - | `Destination`, `ForecastArn`, `ForecastExportJobName` | - | `CreateForecastExportJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Exports a forecast created by the CreateForecast operation to your Amazon Simple Storage Service (Amazon S3) bucket. The forecast file name will match the following conventions: _ _ where the component is in Java SimpleDateFormat (yyyy-MM-ddTHH-mm-ssZ). |
| `CreateMonitor` | - | - | `MonitorName`, `ResourceArn` | - | `CreateMonitorResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Creates a predictor monitor resource for an existing auto predictor. Predictor monitoring allows you to see how your predictor's performance changes over time. |
| `CreatePredictor` | - | - | `FeaturizationConfig`, `ForecastHorizon`, `InputDataConfig`, `PredictorName` | - | `CreatePredictorResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | This operation creates a legacy predictor that does not include all the predictor functionalities provided by Amazon Forecast. To create a predictor that is compatible with all aspects of Forecast, use CreateAutoPredictor. |
| `CreatePredictorBacktestExportJob` | - | - | `Destination`, `PredictorArn`, `PredictorBacktestExportJobName` | - | `CreatePredictorBacktestExportJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Exports backtest forecasts and accuracy metrics generated by the CreateAutoPredictor or CreatePredictor operations. Two folders containing CSV or Parquet files are exported to your specified S3 bucket. |
| `CreateWhatIfAnalysis` | - | - | `ForecastArn`, `WhatIfAnalysisName` | - | `CreateWhatIfAnalysisResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | What-if analysis is a scenario modeling technique where you make a hypothetical change to a time series and compare the forecasts generated by these changes against the baseline, unchanged time series. It is important to remember that the purpose of a what-if... |
| `CreateWhatIfForecast` | - | - | `WhatIfAnalysisArn`, `WhatIfForecastName` | - | `CreateWhatIfForecastResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | A what-if forecast is a forecast that is created from a modified version of the baseline forecast. Each what-if forecast incorporates either a replacement dataset or a set of transformations to the original dataset. |
| `CreateWhatIfForecastExport` | - | - | `Destination`, `WhatIfForecastArns`, `WhatIfForecastExportName` | - | `CreateWhatIfForecastExportResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Exports a forecast created by the CreateWhatIfForecast operation to your Amazon Simple Storage Service (Amazon S3) bucket. The forecast file name will match the following conventions: `≈ _ _ ` The component is in Java SimpleDateFormat (yyyy-MM-ddTHH-mm-ssZ). |
| `DeleteDataset` | - | `idempotent` | `DatasetArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an Amazon Forecast dataset that was created using the CreateDataset operation. You can only delete datasets that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteDatasetGroup` | - | `idempotent` | `DatasetGroupArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a dataset group created using the CreateDatasetGroup operation. You can only delete dataset groups that have a status of `ACTIVE`, `CREATE_FAILED`, or `UPDATE_FAILED`. |
| `DeleteDatasetImportJob` | - | `idempotent` | `DatasetImportJobArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a dataset import job created using the CreateDatasetImportJob operation. You can delete only dataset import jobs that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteExplainability` | - | `idempotent` | `ExplainabilityArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an Explainability resource. You can delete only predictor that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteExplainabilityExport` | - | `idempotent` | `ExplainabilityExportArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an Explainability export. |
| `DeleteForecast` | - | `idempotent` | `ForecastArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a forecast created using the CreateForecast operation. You can delete only forecasts that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteForecastExportJob` | - | `idempotent` | `ForecastExportJobArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a forecast export job created using the CreateForecastExportJob operation. You can delete only export jobs that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteMonitor` | - | `idempotent` | `MonitorArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a monitor resource. You can only delete a monitor resource with a status of `ACTIVE`, `ACTIVE_STOPPED`, `CREATE_FAILED`, or `CREATE_STOPPED`. |
| `DeletePredictor` | - | `idempotent` | `PredictorArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a predictor created using the DescribePredictor or CreatePredictor operations. You can delete only predictor that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeletePredictorBacktestExportJob` | - | `idempotent` | `PredictorBacktestExportJobArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a predictor backtest export job. |
| `DeleteResourceTree` | - | `idempotent` | `ResourceArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes an entire resource tree. This operation will delete the parent resource and its child resources. |
| `DeleteWhatIfAnalysis` | - | `idempotent` | `WhatIfAnalysisArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a what-if analysis created using the CreateWhatIfAnalysis operation. You can delete only what-if analyses that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteWhatIfForecast` | - | `idempotent` | `WhatIfForecastArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a what-if forecast created using the CreateWhatIfForecast operation. You can delete only what-if forecasts that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DeleteWhatIfForecastExport` | - | `idempotent` | `WhatIfForecastExportArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a what-if forecast export created using the CreateWhatIfForecastExport operation. You can delete only what-if forecast exports that have a status of `ACTIVE` or `CREATE_FAILED`. |
| `DescribeAutoPredictor` | - | `idempotent` | `PredictorArn` | - | `DescribeAutoPredictorResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a predictor created using the CreateAutoPredictor operation. |
| `DescribeDataset` | - | `idempotent` | `DatasetArn` | - | `DescribeDatasetResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes an Amazon Forecast dataset created using the CreateDataset operation. In addition to listing the parameters specified in the `CreateDataset` request, this operation includes the following dataset properties: `CreationTime` `LastModificationTime`... |
| `DescribeDatasetGroup` | - | `idempotent` | `DatasetGroupArn` | - | `DescribeDatasetGroupResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a dataset group created using the CreateDatasetGroup operation. In addition to listing the parameters provided in the `CreateDatasetGroup` request, this operation includes the following properties: `DatasetArns` - The datasets belonging to the group. |
| `DescribeDatasetImportJob` | - | `idempotent` | `DatasetImportJobArn` | - | `DescribeDatasetImportJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a dataset import job created using the CreateDatasetImportJob operation. In addition to listing the parameters provided in the `CreateDatasetImportJob` request, this operation includes the following properties: `CreationTime` `LastModificationTime`... |
| `DescribeExplainability` | - | `idempotent` | `ExplainabilityArn` | - | `DescribeExplainabilityResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes an Explainability resource created using the CreateExplainability operation. |
| `DescribeExplainabilityExport` | - | `idempotent` | `ExplainabilityExportArn` | - | `DescribeExplainabilityExportResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes an Explainability export created using the CreateExplainabilityExport operation. |
| `DescribeForecast` | - | `idempotent` | `ForecastArn` | - | `DescribeForecastResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a forecast created using the CreateForecast operation. In addition to listing the properties provided in the `CreateForecast` request, this operation lists the following properties: `DatasetGroupArn` - The dataset group that provided the training... |
| `DescribeForecastExportJob` | - | `idempotent` | `ForecastExportJobArn` | - | `DescribeForecastExportJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a forecast export job created using the CreateForecastExportJob operation. In addition to listing the properties provided by the user in the `CreateForecastExportJob` request, this operation lists the following properties: `CreationTime`... |
| `DescribeMonitor` | - | `idempotent` | `MonitorArn` | - | `DescribeMonitorResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a monitor resource. In addition to listing the properties provided in the CreateMonitor request, this operation lists the following properties: `Baseline` `CreationTime` `LastEvaluationTime` `LastEvaluationState` `LastModificationTime` `Message`... |
| `DescribePredictor` | - | `idempotent` | `PredictorArn` | - | `DescribePredictorResponse` | `InvalidInputException`, `ResourceNotFoundException` | This operation is only valid for legacy predictors created with CreatePredictor. If you are not using a legacy predictor, use DescribeAutoPredictor. |
| `DescribePredictorBacktestExportJob` | - | `idempotent` | `PredictorBacktestExportJobArn` | - | `DescribePredictorBacktestExportJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a predictor backtest export job created using the CreatePredictorBacktestExportJob operation. In addition to listing the properties provided by the user in the `CreatePredictorBacktestExportJob` request, this operation lists the following... |
| `DescribeWhatIfAnalysis` | - | `idempotent` | `WhatIfAnalysisArn` | - | `DescribeWhatIfAnalysisResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the what-if analysis created using the CreateWhatIfAnalysis operation. In addition to listing the properties provided in the `CreateWhatIfAnalysis` request, this operation lists the following properties: `CreationTime` `LastModificationTime`... |
| `DescribeWhatIfForecast` | - | `idempotent` | `WhatIfForecastArn` | - | `DescribeWhatIfForecastResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the what-if forecast created using the CreateWhatIfForecast operation. In addition to listing the properties provided in the `CreateWhatIfForecast` request, this operation lists the following properties: `CreationTime` `LastModificationTime`... |
| `DescribeWhatIfForecastExport` | - | `idempotent` | `WhatIfForecastExportArn` | - | `DescribeWhatIfForecastExportResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the what-if forecast export created using the CreateWhatIfForecastExport operation. In addition to listing the properties provided in the `CreateWhatIfForecastExport` request, this operation lists the following properties: `CreationTime`... |
| `GetAccuracyMetrics` | - | `idempotent` | `PredictorArn` | - | `GetAccuracyMetricsResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Provides metrics on the accuracy of the models that were trained by the CreatePredictor operation. Use metrics to see how well the model performed and to decide whether to use the predictor to generate a forecast. |
| `ListDatasetGroups` | - | `idempotent`, `paginated` | - | - | `ListDatasetGroupsResponse` | `InvalidNextTokenException` | Returns a list of dataset groups created using the CreateDatasetGroup operation. For each dataset group, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListDatasetImportJobs` | - | `idempotent`, `paginated` | - | - | `ListDatasetImportJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of dataset import jobs created using the CreateDatasetImportJob operation. For each import job, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListDatasets` | - | `idempotent`, `paginated` | - | - | `ListDatasetsResponse` | `InvalidNextTokenException` | Returns a list of datasets created using the CreateDataset operation. For each dataset, a summary of its properties, including its Amazon Resource Name (ARN), is returned. |
| `ListExplainabilities` | - | `idempotent`, `paginated` | - | - | `ListExplainabilitiesResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of Explainability resources created using the CreateExplainability operation. This operation returns a summary for each Explainability. |
| `ListExplainabilityExports` | - | `idempotent`, `paginated` | - | - | `ListExplainabilityExportsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of Explainability exports created using the CreateExplainabilityExport operation. This operation returns a summary for each Explainability export. |
| `ListForecastExportJobs` | - | `idempotent`, `paginated` | - | - | `ListForecastExportJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of forecast export jobs created using the CreateForecastExportJob operation. For each forecast export job, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListForecasts` | - | `idempotent`, `paginated` | - | - | `ListForecastsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of forecasts created using the CreateForecast operation. For each forecast, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListMonitorEvaluations` | - | `idempotent`, `paginated` | `MonitorArn` | - | `ListMonitorEvaluationsResponse` | `InvalidInputException`, `InvalidNextTokenException`, `ResourceNotFoundException` | Returns a list of the monitoring evaluation results and predictor events collected by the monitor resource during different windows of time. For information about monitoring see predictor-monitoring. |
| `ListMonitors` | - | `idempotent`, `paginated` | - | - | `ListMonitorsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of monitors created with the CreateMonitor operation and CreateAutoPredictor operation. For each monitor resource, this operation returns of a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListPredictorBacktestExportJobs` | - | `idempotent`, `paginated` | - | - | `ListPredictorBacktestExportJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of predictor backtest export jobs created using the CreatePredictorBacktestExportJob operation. This operation returns a summary for each backtest export job. |
| `ListPredictors` | - | `idempotent`, `paginated` | - | - | `ListPredictorsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of predictors created using the CreateAutoPredictor or CreatePredictor operations. For each predictor, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InvalidInputException`, `ResourceNotFoundException` | Lists the tags for an Amazon Forecast resource. |
| `ListWhatIfAnalyses` | - | `idempotent`, `paginated` | - | - | `ListWhatIfAnalysesResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of what-if analyses created using the CreateWhatIfAnalysis operation. For each what-if analysis, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListWhatIfForecastExports` | - | `idempotent`, `paginated` | - | - | `ListWhatIfForecastExportsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of what-if forecast exports created using the CreateWhatIfForecastExport operation. For each what-if forecast export, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ListWhatIfForecasts` | - | `idempotent`, `paginated` | - | - | `ListWhatIfForecastsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of what-if forecasts created using the CreateWhatIfForecast operation. For each what-if forecast, this operation returns a summary of its properties, including its Amazon Resource Name (ARN). |
| `ResumeResource` | - | `idempotent` | `ResourceArn` | - | `Unit` | `InvalidInputException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Resumes a stopped monitor resource. |
| `StopResource` | - | `idempotent` | `ResourceArn` | - | `Unit` | `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Stops a resource. The resource undergoes the following states: `CREATE_STOPPING` and `CREATE_STOPPED`. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceNotFoundException` | Associates the specified tags to a resource with the specified `resourceArn`. If existing tags on a resource are not specified in the request parameters, they are not changed. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InvalidInputException`, `ResourceNotFoundException` | Deletes the specified tags from a resource. |
| `UpdateDatasetGroup` | - | `idempotent` | `DatasetArns`, `DatasetGroupArn` | - | `UpdateDatasetGroupResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Replaces the datasets in a dataset group with the specified datasets. The `Status` of the dataset group must be `ACTIVE` before you can use the dataset group to create a predictor. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInputException` | `structure` | `Message` | We can't process the request because it includes an invalid value or a value that exceeds the valid range. |
| `ResourceNotFoundException` | `structure` | `Message` | We can't find a resource with that Amazon Resource Name (ARN). |
| `ResourceInUseException` | `structure` | `Message` | The specified resource is in use. |
| `LimitExceededException` | `structure` | `Message` | The limit on the number of resources per account has been exceeded. |
| `ResourceAlreadyExistsException` | `structure` | `Message` | There is already a resource with this name. |
| `InvalidNextTokenException` | `structure` | `Message` | The token is not valid. |
| `CreateAutoPredictorRequest` | `structure` | `DataConfig`, `EncryptionConfig`, `ExplainPredictor`, `ForecastDimensions`, `ForecastFrequency`, `ForecastHorizon`, `ForecastTypes`, `MonitorConfig`, `OptimizationMetric`, `PredictorName`, `ReferencePredictorArn`, `Tags`, ... (+1) | - |
| `CreateAutoPredictorResponse` | `structure` | `PredictorArn` | - |
| `CreateDatasetRequest` | `structure` | `DataFrequency`, `DatasetName`, `DatasetType`, `Domain`, `EncryptionConfig`, `Schema`, `Tags` | - |
| `CreateDatasetResponse` | `structure` | `DatasetArn` | - |
| `CreateDatasetGroupRequest` | `structure` | `DatasetArns`, `DatasetGroupName`, `Domain`, `Tags` | - |
| `CreateDatasetGroupResponse` | `structure` | `DatasetGroupArn` | - |
| `CreateDatasetImportJobRequest` | `structure` | `DataSource`, `DatasetArn`, `DatasetImportJobName`, `Format`, `GeolocationFormat`, `ImportMode`, `Tags`, `TimeZone`, `TimestampFormat`, `UseGeolocationForTimeZone` | - |
| `CreateDatasetImportJobResponse` | `structure` | `DatasetImportJobArn` | - |
| `CreateExplainabilityRequest` | `structure` | `DataSource`, `EnableVisualization`, `EndDateTime`, `ExplainabilityConfig`, `ExplainabilityName`, `ResourceArn`, `Schema`, `StartDateTime`, `Tags` | - |
| `CreateExplainabilityResponse` | `structure` | `ExplainabilityArn` | - |
| `CreateExplainabilityExportRequest` | `structure` | `Destination`, `ExplainabilityArn`, `ExplainabilityExportName`, `Format`, `Tags` | - |
| `CreateExplainabilityExportResponse` | `structure` | `ExplainabilityExportArn` | - |
| `CreateForecastRequest` | `structure` | `ForecastName`, `ForecastTypes`, `PredictorArn`, `Tags`, `TimeSeriesSelector` | - |
| `CreateForecastResponse` | `structure` | `ForecastArn` | - |
| `CreateForecastExportJobRequest` | `structure` | `Destination`, `ForecastArn`, `ForecastExportJobName`, `Format`, `Tags` | - |
| `CreateForecastExportJobResponse` | `structure` | `ForecastExportJobArn` | - |
| `CreateMonitorRequest` | `structure` | `MonitorName`, `ResourceArn`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

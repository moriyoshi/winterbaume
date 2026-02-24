# Amazon Lookout for Equipment

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Lookout for Equipment is a machine learning service that uses advanced analytics to identify anomalies in machines from sensor data for use in predictive maintenance.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Lookout for Equipment resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Lookout for Equipment workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Delete`, `Create`, `Update` operation families, including `ListDataIngestionJobs`, `ListDatasets`, `ListInferenceEvents`, `ListInferenceExecutions`, `DescribeDataIngestionJob`, `DescribeDataset`.

## Service Identity and Protocol

- AWS model slug: `lookoutequipment`
- AWS SDK for Rust slug: `lookoutequipment`
- Model version: `2020-12-15`
- Model file: `vendor/api-models-aws/models/lookoutequipment/service/2020-12-15/lookoutequipment-2020-12-15.json`
- SDK ID: `LookoutEquipment`
- Endpoint prefix: `lookoutequipment`
- ARN namespace: `lookoutequipment`
- CloudFormation name: `LookoutEquipment`
- CloudTrail event source: `lookoutequipment.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Describe` (9), `Delete` (7), `Create` (6), `Update` (5), `Start` (3), `Import` (2), `Stop` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDataset`, `CreateInferenceScheduler`, `CreateLabel`, `CreateLabelGroup`, `CreateModel`, `CreateRetrainingScheduler`, `DeleteDataset`, `DeleteInferenceScheduler`, `DeleteLabel`, `DeleteLabelGroup`, `DeleteModel`, `DeleteResourcePolicy`, `DeleteRetrainingScheduler`, `ImportDataset`, `ImportModelVersion`, `PutResourcePolicy`, `StartDataIngestionJob`, `StartInferenceScheduler`, `StartRetrainingScheduler`, `StopInferenceScheduler`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDataIngestionJob`, `DescribeDataset`, `DescribeInferenceScheduler`, `DescribeLabel`, `DescribeLabelGroup`, `DescribeModel`, `DescribeModelVersion`, `DescribeResourcePolicy`, `DescribeRetrainingScheduler`, `ListDataIngestionJobs`, `ListDatasets`, `ListInferenceEvents`, `ListInferenceExecutions`, `ListInferenceSchedulers`, `ListLabelGroups`, `ListLabels`, `ListModelVersions`, `ListModels`, `ListRetrainingSchedulers`, `ListSensorStatistics`, ... (+1).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 10 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeDataIngestionJob`, `ImportDataset`, `ImportModelVersion`, `ListDataIngestionJobs`, `ListInferenceExecutions`, `StartDataIngestionJob`, `StartInferenceScheduler`, `StartRetrainingScheduler`, `StopInferenceScheduler`, `StopRetrainingScheduler`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 49 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`.

## Operation Groups

### List

- Operations: `ListDataIngestionJobs`, `ListDatasets`, `ListInferenceEvents`, `ListInferenceExecutions`, `ListInferenceSchedulers`, `ListLabelGroups`, `ListLabels`, `ListModelVersions`, `ListModels`, `ListRetrainingSchedulers`, `ListSensorStatistics`, `ListTagsForResource`
- Traits: `paginated` (11)
- Common required input members in this group: `DatasetName`, `InferenceSchedulerName`, `IntervalEndTime`, `IntervalStartTime`, `LabelGroupName`, `ModelName`, `ResourceArn`

### Describe

- Operations: `DescribeDataIngestionJob`, `DescribeDataset`, `DescribeInferenceScheduler`, `DescribeLabel`, `DescribeLabelGroup`, `DescribeModel`, `DescribeModelVersion`, `DescribeResourcePolicy`, `DescribeRetrainingScheduler`
- Common required input members in this group: `DatasetName`, `InferenceSchedulerName`, `JobId`, `LabelGroupName`, `LabelId`, `ModelName`, `ModelVersion`, `ResourceArn`

### Delete

- Operations: `DeleteDataset`, `DeleteInferenceScheduler`, `DeleteLabel`, `DeleteLabelGroup`, `DeleteModel`, `DeleteResourcePolicy`, `DeleteRetrainingScheduler`
- Common required input members in this group: `DatasetName`, `InferenceSchedulerName`, `LabelGroupName`, `LabelId`, `ModelName`, `ResourceArn`

### Create

- Operations: `CreateDataset`, `CreateInferenceScheduler`, `CreateLabel`, `CreateLabelGroup`, `CreateModel`, `CreateRetrainingScheduler`
- Traits: `idempotency-token` (6)
- Common required input members in this group: `ClientToken`, `DataInputConfiguration`, `DataOutputConfiguration`, `DataUploadFrequency`, `DatasetName`, `EndTime`, `InferenceSchedulerName`, `LabelGroupName`, `LookbackWindow`, `ModelName`, `Rating`, `RetrainingFrequency`, `RoleArn`, `StartTime`

### Update

- Operations: `UpdateActiveModelVersion`, `UpdateInferenceScheduler`, `UpdateLabelGroup`, `UpdateModel`, `UpdateRetrainingScheduler`
- Common required input members in this group: `InferenceSchedulerName`, `LabelGroupName`, `ModelName`, `ModelVersion`

### Start

- Operations: `StartDataIngestionJob`, `StartInferenceScheduler`, `StartRetrainingScheduler`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientToken`, `DatasetName`, `InferenceSchedulerName`, `IngestionInputConfiguration`, `ModelName`, `RoleArn`

### Import

- Operations: `ImportDataset`, `ImportModelVersion`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `ClientToken`, `DatasetName`, `SourceDatasetArn`, `SourceModelVersionArn`

### Stop

- Operations: `StopInferenceScheduler`, `StopRetrainingScheduler`
- Common required input members in this group: `InferenceSchedulerName`, `ModelName`

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientToken`, `ResourceArn`, `ResourcePolicy`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateDataset` | - | `idempotency-token` | `ClientToken`, `DatasetName` | `ClientToken` | `CreateDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a container for a collection of data being ingested for analysis. The dataset contains the metadata describing where the data is and what the data actually looks like. |
| `CreateInferenceScheduler` | - | `idempotency-token` | `ClientToken`, `DataInputConfiguration`, `DataOutputConfiguration`, `DataUploadFrequency`, `InferenceSchedulerName`, `ModelName`, `RoleArn` | `ClientToken` | `CreateInferenceSchedulerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a scheduled inference. Scheduling an inference is setting up a continuous real-time inference plan to analyze new measurement data. |
| `CreateLabel` | - | `idempotency-token` | `ClientToken`, `EndTime`, `LabelGroupName`, `Rating`, `StartTime` | `ClientToken` | `CreateLabelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a label for an event. |
| `CreateLabelGroup` | - | `idempotency-token` | `ClientToken`, `LabelGroupName` | `ClientToken` | `CreateLabelGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a group of labels. |
| `CreateModel` | - | `idempotency-token` | `ClientToken`, `DatasetName`, `ModelName` | `ClientToken` | `CreateModelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a machine learning model for data inference. A machine-learning (ML) model is a mathematical model that finds patterns in your data. |
| `CreateRetrainingScheduler` | - | `idempotency-token` | `ClientToken`, `LookbackWindow`, `ModelName`, `RetrainingFrequency` | `ClientToken` | `CreateRetrainingSchedulerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a retraining scheduler on the specified model. |
| `DeleteDataset` | - | - | `DatasetName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a dataset and associated artifacts. The operation will check to see if any inference scheduler or data ingestion job is currently using the dataset, and if there isn't, the dataset, its metadata, and any associated data stored in S3 will be deleted. |
| `DeleteInferenceScheduler` | - | - | `InferenceSchedulerName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an inference scheduler that has been set up. Prior inference results will not be deleted. |
| `DeleteLabel` | - | - | `LabelGroupName`, `LabelId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a label. |
| `DeleteLabelGroup` | - | - | `LabelGroupName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a group of labels. |
| `DeleteModel` | - | - | `ModelName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a machine learning model currently available for Amazon Lookout for Equipment. This will prevent it from being used with an inference scheduler, even one that is already set up. |
| `DeleteResourcePolicy` | - | - | `ResourceArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource policy attached to the resource. |
| `DeleteRetrainingScheduler` | - | - | `ModelName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a retraining scheduler from a model. The retraining scheduler must be in the `STOPPED` status. |
| `DescribeDataIngestionJob` | - | - | `JobId` | - | `DescribeDataIngestionJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides information on a specific data ingestion job such as creation time, dataset ARN, and status. |
| `DescribeDataset` | - | - | `DatasetName` | - | `DescribeDatasetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a JSON description of the data in each time series dataset, including names, column names, and data types. |
| `DescribeInferenceScheduler` | - | - | `InferenceSchedulerName` | - | `DescribeInferenceSchedulerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Specifies information about the inference scheduler being used, including name, model, status, and associated metadata |
| `DescribeLabel` | - | - | `LabelGroupName`, `LabelId` | - | `DescribeLabelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the name of the label. |
| `DescribeLabelGroup` | - | - | `LabelGroupName` | - | `DescribeLabelGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the label group. |
| `DescribeModel` | - | - | `ModelName` | - | `DescribeModelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a JSON containing the overall information about a specific machine learning model, including model name and ARN, dataset, training and evaluation information, status, and so on. |
| `DescribeModelVersion` | - | - | `ModelName`, `ModelVersion` | - | `DescribeModelVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a specific machine learning model version. |
| `DescribeResourcePolicy` | - | - | `ResourceArn` | - | `DescribeResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides the details of a resource policy attached to a resource. |
| `DescribeRetrainingScheduler` | - | - | `ModelName` | - | `DescribeRetrainingSchedulerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a description of the retraining scheduler, including information such as the model name and retraining parameters. |
| `ImportDataset` | - | `idempotency-token` | `ClientToken`, `SourceDatasetArn` | `ClientToken` | `ImportDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Imports a dataset. |
| `ImportModelVersion` | - | `idempotency-token` | `ClientToken`, `DatasetName`, `SourceModelVersionArn` | `ClientToken` | `ImportModelVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Imports a model that has been trained successfully. |
| `ListDataIngestionJobs` | - | `paginated` | - | - | `ListDataIngestionJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provides a list of all data ingestion jobs, including dataset name and ARN, S3 location of the input data, status, and so on. |
| `ListDatasets` | - | `paginated` | - | - | `ListDatasetsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all datasets currently available in your account, filtering on the dataset name. |
| `ListInferenceEvents` | - | `paginated` | `InferenceSchedulerName`, `IntervalEndTime`, `IntervalStartTime` | - | `ListInferenceEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all inference events that have been found for the specified inference scheduler. |
| `ListInferenceExecutions` | - | `paginated` | `InferenceSchedulerName` | - | `ListInferenceExecutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all inference executions that have been performed by the specified inference scheduler. |
| `ListInferenceSchedulers` | - | `paginated` | - | - | `ListInferenceSchedulersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all inference schedulers currently available for your account. |
| `ListLabelGroups` | - | `paginated` | - | - | `ListLabelGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of the label groups. |
| `ListLabels` | - | `paginated` | `LabelGroupName` | - | `ListLabelsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provides a list of labels. |
| `ListModelVersions` | - | `paginated` | `ModelName` | - | `ListModelVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates a list of all model versions for a given model, including the model version, model version ARN, and status. To list a subset of versions, use the `MaxModelVersion` and `MinModelVersion` fields. |
| `ListModels` | - | `paginated` | - | - | `ListModelsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Generates a list of all models in the account, including model name and ARN, dataset, and status. |
| `ListRetrainingSchedulers` | - | `paginated` | - | - | `ListRetrainingSchedulersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all retraining schedulers in your account, filtering by model name prefix and status. |
| `ListSensorStatistics` | - | `paginated` | `DatasetName` | - | `ListSensorStatisticsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists statistics about the data collected for each of the sensors that have been successfully ingested in the particular dataset. Can also be used to retreive Sensor Statistics for a previous ingestion job. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the tags for a specified resource, including key and value. |
| `PutResourcePolicy` | - | `idempotency-token` | `ClientToken`, `ResourceArn`, `ResourcePolicy` | `ClientToken` | `PutResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a resource control policy for a given resource. |
| `StartDataIngestionJob` | - | `idempotency-token` | `ClientToken`, `DatasetName`, `IngestionInputConfiguration`, `RoleArn` | `ClientToken` | `StartDataIngestionJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a data ingestion job. Amazon Lookout for Equipment returns the job status. |
| `StartInferenceScheduler` | - | - | `InferenceSchedulerName` | - | `StartInferenceSchedulerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts an inference scheduler. |
| `StartRetrainingScheduler` | - | - | `ModelName` | - | `StartRetrainingSchedulerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a retraining scheduler. |
| `StopInferenceScheduler` | - | - | `InferenceSchedulerName` | - | `StopInferenceSchedulerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops an inference scheduler. |
| `StopRetrainingScheduler` | - | - | `ModelName` | - | `StopRetrainingSchedulerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a retraining scheduler. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a given tag to a resource in your account. A tag is a key-value pair which can be added to an Amazon Lookout for Equipment resource as metadata. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a specific tag from a given resource. The tag is specified by its key. |
| `UpdateActiveModelVersion` | - | - | `ModelName`, `ModelVersion` | - | `UpdateActiveModelVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets the active model version for a given machine learning model. |
| `UpdateInferenceScheduler` | - | - | `InferenceSchedulerName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an inference scheduler. |
| `UpdateLabelGroup` | - | - | `LabelGroupName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the label group. |
| `UpdateModel` | - | - | `ModelName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a model in the account. |
| `UpdateRetrainingScheduler` | - | - | `ModelName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a retraining scheduler. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | The request could not be completed because you do not have access to the resource. |
| `InternalServerException` | `structure` | `Message` | Processing of the request has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy constraints specified by Amazon Lookout for Equipment or a related Amazon Web Services service that's being utilized. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource requested could not be found. |
| `ConflictException` | `structure` | `Message` | The request could not be completed due to a conflict with the current state of the target resource. |
| `ServiceQuotaExceededException` | `structure` | `Message` | Resource limitations have been exceeded. |
| `CreateDatasetRequest` | `structure` | `ClientToken`, `DatasetName`, `DatasetSchema`, `ServerSideKmsKeyId`, `Tags` | - |
| `CreateDatasetResponse` | `structure` | `DatasetArn`, `DatasetName`, `Status` | - |
| `CreateInferenceSchedulerRequest` | `structure` | `ClientToken`, `DataDelayOffsetInMinutes`, `DataInputConfiguration`, `DataOutputConfiguration`, `DataUploadFrequency`, `InferenceSchedulerName`, `ModelName`, `RoleArn`, `ServerSideKmsKeyId`, `Tags` | - |
| `CreateInferenceSchedulerResponse` | `structure` | `InferenceSchedulerArn`, `InferenceSchedulerName`, `ModelQuality`, `Status` | - |
| `CreateLabelRequest` | `structure` | `ClientToken`, `EndTime`, `Equipment`, `FaultCode`, `LabelGroupName`, `Notes`, `Rating`, `StartTime` | - |
| `CreateLabelResponse` | `structure` | `LabelId` | - |
| `CreateLabelGroupRequest` | `structure` | `ClientToken`, `FaultCodes`, `LabelGroupName`, `Tags` | - |
| `CreateLabelGroupResponse` | `structure` | `LabelGroupArn`, `LabelGroupName` | - |
| `CreateModelRequest` | `structure` | `ClientToken`, `DataPreProcessingConfiguration`, `DatasetName`, `DatasetSchema`, `EvaluationDataEndTime`, `EvaluationDataStartTime`, `LabelsInputConfiguration`, `ModelDiagnosticsOutputConfiguration`, `ModelName`, `OffCondition`, `RoleArn`, `ServerSideKmsKeyId`, ... (+3) | - |
| `CreateModelResponse` | `structure` | `ModelArn`, `Status` | - |
| `CreateRetrainingSchedulerRequest` | `structure` | `ClientToken`, `LookbackWindow`, `ModelName`, `PromoteMode`, `RetrainingFrequency`, `RetrainingStartDate` | - |
| `CreateRetrainingSchedulerResponse` | `structure` | `ModelArn`, `ModelName`, `Status` | - |
| `DeleteDatasetRequest` | `structure` | `DatasetName` | - |
| `DeleteInferenceSchedulerRequest` | `structure` | `InferenceSchedulerName` | - |
| `DeleteLabelRequest` | `structure` | `LabelGroupName`, `LabelId` | - |
| `DeleteLabelGroupRequest` | `structure` | `LabelGroupName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

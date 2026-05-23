# Amazon Machine Learning

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Definition of the public APIs exposed by Amazon Machine Learning

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Machine Learning workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Delete`, `Describe`, `Get`, `Update` operation families, including `CreateBatchPrediction`, `CreateDataSourceFromRDS`, `CreateDataSourceFromRedshift`, `CreateDataSourceFromS3`, `DeleteBatchPrediction`, `DeleteDataSource`.

## Service Identity and Protocol

- AWS model slug: `machine-learning`
- AWS SDK for Rust slug: `machinelearning`
- Model version: `2014-12-12`
- Model file: `vendor/api-models-aws/models/machine-learning/service/2014-12-12/machine-learning-2014-12-12.json`
- SDK ID: `Machine Learning`
- Endpoint prefix: `machinelearning`
- ARN namespace: `machinelearning`
- CloudFormation name: `MachineLearning`
- CloudTrail event source: `machinelearning.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (7), `Delete` (6), `Describe` (5), `Get` (4), `Update` (4), `Add` (1), `Predict` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTags`, `CreateBatchPrediction`, `CreateDataSourceFromRDS`, `CreateDataSourceFromRedshift`, `CreateDataSourceFromS3`, `CreateEvaluation`, `CreateMLModel`, `CreateRealtimeEndpoint`, `DeleteBatchPrediction`, `DeleteDataSource`, `DeleteEvaluation`, `DeleteMLModel`, `DeleteRealtimeEndpoint`, `DeleteTags`, `UpdateBatchPrediction`, `UpdateDataSource`, `UpdateEvaluation`, `UpdateMLModel`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBatchPredictions`, `DescribeDataSources`, `DescribeEvaluations`, `DescribeMLModels`, `DescribeTags`, `GetBatchPrediction`, `GetDataSource`, `GetEvaluation`, `GetMLModel`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 28 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Create

- Operations: `CreateBatchPrediction`, `CreateDataSourceFromRDS`, `CreateDataSourceFromRedshift`, `CreateDataSourceFromS3`, `CreateEvaluation`, `CreateMLModel`, `CreateRealtimeEndpoint`
- Common required input members in this group: `MLModelId`, `DataSourceId`, `RoleARN`, `DataSpec`

### Delete

- Operations: `DeleteBatchPrediction`, `DeleteDataSource`, `DeleteEvaluation`, `DeleteMLModel`, `DeleteRealtimeEndpoint`, `DeleteTags`
- Common required input members in this group: `MLModelId`

### Describe

- Operations: `DescribeBatchPredictions`, `DescribeDataSources`, `DescribeEvaluations`, `DescribeMLModels`, `DescribeTags`
- Traits: `paginated` (4)
- Common required input members in this group: -

### Get

- Operations: `GetBatchPrediction`, `GetDataSource`, `GetEvaluation`, `GetMLModel`
- Common required input members in this group: -

### Update

- Operations: `UpdateBatchPrediction`, `UpdateDataSource`, `UpdateEvaluation`, `UpdateMLModel`
- Common required input members in this group: -

### Add

- Operations: `AddTags`
- Common required input members in this group: -

### Predict

- Operations: `Predict`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTags` | `-` | - | `Tags`, `ResourceId`, `ResourceType` | - | `AddTagsOutput` | `InternalServerException`, `InvalidInputException`, `InvalidTagException`, `ResourceNotFoundException`, `TagLimitExceededException` | Adds one or more tags to an object, up to a limit of 10. Each tag consists of a key and an optional value. If you add a tag using a key that is already associated with the ML object, AddTags updates the tag's value. |
| `CreateBatchPrediction` | `-` | - | `BatchPredictionId`, `MLModelId`, `BatchPredictionDataSourceId`, `OutputUri` | - | `CreateBatchPredictionOutput` | `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidInputException` | Generates predictions for a group of observations. The observations to process exist in one or more data files referenced by a DataSource . This operation creates a new BatchPrediction , and uses an MLModel and the d ... |
| `CreateDataSourceFromRDS` | `-` | - | `DataSourceId`, `RDSData`, `RoleARN` | - | `CreateDataSourceFromRDSOutput` | `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidInputException` | Creates a DataSource object from an Amazon Relational Database Service (Amazon RDS). A DataSource references data that can be used to perform CreateMLModel , CreateEvaluation , or CreateBatchPrediction operations. Cr ... |
| `CreateDataSourceFromRedshift` | `-` | - | `DataSourceId`, `DataSpec`, `RoleARN` | - | `CreateDataSourceFromRedshiftOutput` | `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidInputException` | Creates a DataSource from a database hosted on an Amazon Redshift cluster. A DataSource references data that can be used to perform either CreateMLModel , CreateEvaluation , or CreateBatchPrediction operations. Creat ... |
| `CreateDataSourceFromS3` | `-` | - | `DataSourceId`, `DataSpec` | - | `CreateDataSourceFromS3Output` | `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidInputException` | Creates a DataSource object. A DataSource references data that can be used to perform CreateMLModel , CreateEvaluation , or CreateBatchPrediction operations. CreateDataSourceFromS3 is an asynchronous operation. In re ... |
| `CreateEvaluation` | `-` | - | `EvaluationId`, `MLModelId`, `EvaluationDataSourceId` | - | `CreateEvaluationOutput` | `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidInputException` | Creates a new Evaluation of an MLModel . An MLModel is evaluated on a set of observations associated to a DataSource . Like a DataSource for an MLModel , the DataSource for an Evaluation contains values for the Targe ... |
| `CreateMLModel` | `-` | - | `MLModelId`, `MLModelType`, `TrainingDataSourceId` | - | `CreateMLModelOutput` | `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidInputException` | Creates a new MLModel using the DataSource and the recipe as information sources. An MLModel is nearly immutable. Users can update only the MLModelName and the ScoreThreshold in an MLModel without creating a new MLMo ... |
| `CreateRealtimeEndpoint` | `-` | - | `MLModelId` | - | `CreateRealtimeEndpointOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Creates a real-time endpoint for the MLModel . The endpoint contains the URI of the MLModel ; that is, the location to send real-time prediction requests for the specified MLModel . |
| `DeleteBatchPrediction` | `-` | - | `BatchPredictionId` | - | `DeleteBatchPredictionOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Assigns the DELETED status to a BatchPrediction , rendering it unusable. After using the DeleteBatchPrediction operation, you can use the GetBatchPrediction operation to verify that the status of the BatchPrediction ... |
| `DeleteDataSource` | `-` | - | `DataSourceId` | - | `DeleteDataSourceOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Assigns the DELETED status to a DataSource , rendering it unusable. After using the DeleteDataSource operation, you can use the GetDataSource operation to verify that the status of the DataSource changed to DELETED. ... |
| `DeleteEvaluation` | `-` | - | `EvaluationId` | - | `DeleteEvaluationOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Assigns the DELETED status to an Evaluation , rendering it unusable. After invoking the DeleteEvaluation operation, you can use the GetEvaluation operation to verify that the status of the Evaluation changed to DELET ... |
| `DeleteMLModel` | `-` | - | `MLModelId` | - | `DeleteMLModelOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Assigns the DELETED status to an MLModel , rendering it unusable. After using the DeleteMLModel operation, you can use the GetMLModel operation to verify that the status of the MLModel changed to DELETED. Caution: Th ... |
| `DeleteRealtimeEndpoint` | `-` | - | `MLModelId` | - | `DeleteRealtimeEndpointOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Deletes a real time endpoint of an MLModel . |
| `DeleteTags` | `-` | - | `TagKeys`, `ResourceId`, `ResourceType` | - | `DeleteTagsOutput` | `InternalServerException`, `InvalidInputException`, `InvalidTagException`, `ResourceNotFoundException` | Deletes the specified tags associated with an ML object. After this operation is complete, you can't recover deleted tags. If you specify a tag that doesn't exist, Amazon ML ignores it. |
| `DescribeBatchPredictions` | `-` | `paginated` | - | - | `DescribeBatchPredictionsOutput` | `InternalServerException`, `InvalidInputException` | Returns a list of BatchPrediction operations that match the search criteria in the request. |
| `DescribeDataSources` | `-` | `paginated` | - | - | `DescribeDataSourcesOutput` | `InternalServerException`, `InvalidInputException` | Returns a list of DataSource that match the search criteria in the request. |
| `DescribeEvaluations` | `-` | `paginated` | - | - | `DescribeEvaluationsOutput` | `InternalServerException`, `InvalidInputException` | Returns a list of DescribeEvaluations that match the search criteria in the request. |
| `DescribeMLModels` | `-` | `paginated` | - | - | `DescribeMLModelsOutput` | `InternalServerException`, `InvalidInputException` | Returns a list of MLModel that match the search criteria in the request. |
| `DescribeTags` | `-` | - | `ResourceId`, `ResourceType` | - | `DescribeTagsOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Describes one or more of the tags for your Amazon ML object. |
| `GetBatchPrediction` | `-` | - | `BatchPredictionId` | - | `GetBatchPredictionOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Returns a BatchPrediction that includes detailed metadata, status, and data file information for a Batch Prediction request. |
| `GetDataSource` | `-` | - | `DataSourceId` | - | `GetDataSourceOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Returns a DataSource that includes metadata and data file information, as well as the current status of the DataSource . GetDataSource provides results in normal or verbose format. The verbose format adds the schema ... |
| `GetEvaluation` | `-` | - | `EvaluationId` | - | `GetEvaluationOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Returns an Evaluation that includes metadata as well as the current status of the Evaluation . |
| `GetMLModel` | `-` | - | `MLModelId` | - | `GetMLModelOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Returns an MLModel that includes detailed metadata, data source information, and the current status of the MLModel . GetMLModel provides results in normal or verbose format. |
| `Predict` | `-` | - | `MLModelId`, `Record`, `PredictEndpoint` | - | `PredictOutput` | `InternalServerException`, `InvalidInputException`, `LimitExceededException`, `PredictorNotMountedException`, `ResourceNotFoundException` | Generates a prediction for the observation using the specified ML Model . Note: Not all response parameters will be populated. Whether a response parameter is populated depends on the type of model requested. |
| `UpdateBatchPrediction` | `-` | - | `BatchPredictionId`, `BatchPredictionName` | - | `UpdateBatchPredictionOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Updates the BatchPredictionName of a BatchPrediction . You can use the GetBatchPrediction operation to view the contents of the updated data element. |
| `UpdateDataSource` | `-` | - | `DataSourceId`, `DataSourceName` | - | `UpdateDataSourceOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Updates the DataSourceName of a DataSource . You can use the GetDataSource operation to view the contents of the updated data element. |
| `UpdateEvaluation` | `-` | - | `EvaluationId`, `EvaluationName` | - | `UpdateEvaluationOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Updates the EvaluationName of an Evaluation . You can use the GetEvaluation operation to view the contents of the updated data element. |
| `UpdateMLModel` | `-` | - | `MLModelId` | - | `UpdateMLModelOutput` | `InternalServerException`, `InvalidInputException`, `ResourceNotFoundException` | Updates the MLModelName and the ScoreThreshold of an MLModel . You can use the GetMLModel operation to view the contents of the updated data element. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `IdempotentParameterMismatchException` | `structure` | message, code | A second request to use or change an object was not allowed. This can result from retrying a request using a parameter that was not present in the original ... |
| `InternalServerException` | `structure` | message, code | An error on the server occurred when trying to process a request. |
| `InvalidInputException` | `structure` | message, code | An error on the client occurred. Typically, the cause is an invalid input value. |
| `InvalidTagException` | `structure` | message | - |
| `LimitExceededException` | `structure` | message, code | The subscriber exceeded the maximum number of operations. This exception can occur when listing objects such as DataSource . |
| `PredictorNotMountedException` | `structure` | message | The exception is thrown when a predict request is made to an unmounted MLModel . |
| `ResourceNotFoundException` | `structure` | message, code | A specified resource cannot be located. |
| `TagLimitExceededException` | `structure` | message | - |
| `AddTagsInput` | `structure` | Tags, ResourceId, ResourceType | - |
| `AddTagsOutput` | `structure` | ResourceId, ResourceType | Amazon ML returns the following elements. |
| `CreateBatchPredictionInput` | `structure` | BatchPredictionId, BatchPredictionName, MLModelId, BatchPredictionDataSourceId, OutputUri | - |
| `CreateBatchPredictionOutput` | `structure` | BatchPredictionId | Represents the output of a CreateBatchPrediction operation, and is an acknowledgement that Amazon ML received the request. The CreateBatchPrediction operati ... |
| `CreateDataSourceFromRDSInput` | `structure` | DataSourceId, DataSourceName, RDSData, RoleARN, ComputeStatistics | - |
| `CreateDataSourceFromRDSOutput` | `structure` | DataSourceId | Represents the output of a CreateDataSourceFromRDS operation, and is an acknowledgement that Amazon ML received the request. The CreateDataSourceFromRDS > o ... |
| `CreateDataSourceFromRedshiftInput` | `structure` | DataSourceId, DataSourceName, DataSpec, RoleARN, ComputeStatistics | - |
| `CreateDataSourceFromRedshiftOutput` | `structure` | DataSourceId | Represents the output of a CreateDataSourceFromRedshift operation, and is an acknowledgement that Amazon ML received the request. The CreateDataSourceFromRe ... |
| `CreateDataSourceFromS3Input` | `structure` | DataSourceId, DataSourceName, DataSpec, ComputeStatistics | - |
| `CreateDataSourceFromS3Output` | `structure` | DataSourceId | Represents the output of a CreateDataSourceFromS3 operation, and is an acknowledgement that Amazon ML received the request. The CreateDataSourceFromS3 opera ... |
| `CreateEvaluationInput` | `structure` | EvaluationId, EvaluationName, MLModelId, EvaluationDataSourceId | - |
| `CreateEvaluationOutput` | `structure` | EvaluationId | Represents the output of a CreateEvaluation operation, and is an acknowledgement that Amazon ML received the request. CreateEvaluation operation is asynchro ... |
| `CreateMLModelInput` | `structure` | MLModelId, MLModelName, MLModelType, Parameters, TrainingDataSourceId, Recipe, RecipeUri | - |
| `CreateMLModelOutput` | `structure` | MLModelId | Represents the output of a CreateMLModel operation, and is an acknowledgement that Amazon ML received the request. The CreateMLModel operation is asynchrono ... |
| `CreateRealtimeEndpointInput` | `structure` | MLModelId | - |
| `CreateRealtimeEndpointOutput` | `structure` | MLModelId, RealtimeEndpointInfo | Represents the output of an CreateRealtimeEndpoint operation. The result contains the MLModelId and the endpoint information for the MLModel . Note: The end ... |
| `DeleteBatchPredictionInput` | `structure` | BatchPredictionId | - |
| `DeleteBatchPredictionOutput` | `structure` | BatchPredictionId | Represents the output of a DeleteBatchPrediction operation. You can use the GetBatchPrediction operation and check the value of the Status parameter to see ... |
| `DeleteDataSourceInput` | `structure` | DataSourceId | - |
| `DeleteDataSourceOutput` | `structure` | DataSourceId | Represents the output of a DeleteDataSource operation. |
| `DeleteEvaluationInput` | `structure` | EvaluationId | - |
| `DeleteEvaluationOutput` | `structure` | EvaluationId | Represents the output of a DeleteEvaluation operation. The output indicates that Amazon Machine Learning (Amazon ML) received the request. You can use the G ... |
| `DeleteMLModelInput` | `structure` | MLModelId | - |
| `DeleteMLModelOutput` | `structure` | MLModelId | Represents the output of a DeleteMLModel operation. You can use the GetMLModel operation and check the value of the Status parameter to see whether an MLMod ... |
| `DeleteRealtimeEndpointInput` | `structure` | MLModelId | - |
| `DeleteRealtimeEndpointOutput` | `structure` | MLModelId, RealtimeEndpointInfo | Represents the output of an DeleteRealtimeEndpoint operation. The result contains the MLModelId and the endpoint information for the MLModel . |
| `DeleteTagsInput` | `structure` | TagKeys, ResourceId, ResourceType | - |
| `DeleteTagsOutput` | `structure` | ResourceId, ResourceType | Amazon ML returns the following elements. |
| `DescribeBatchPredictionsInput` | `structure` | FilterVariable, EQ, GT, LT, GE, LE, NE, Prefix, SortOrder, NextToken, Limit | - |
| `DescribeBatchPredictionsOutput` | `structure` | Results, NextToken | Represents the output of a DescribeBatchPredictions operation. The content is essentially a list of BatchPrediction s. |
| `DescribeDataSourcesInput` | `structure` | FilterVariable, EQ, GT, LT, GE, LE, NE, Prefix, SortOrder, NextToken, Limit | - |
| `DescribeDataSourcesOutput` | `structure` | Results, NextToken | Represents the query results from a DescribeDataSources operation. The content is essentially a list of DataSource . |
| `Algorithm` | `enum` | SGD | The function used to train an MLModel . Training choices supported by Amazon ML include the following: SGD - Stochastic Gradient Descent. RandomForest - Ran ... |
| `BatchPredictionFilterVariable` | `enum` | CREATED_AT, LAST_UPDATED_AT, STATUS, NAME, IAM_USER, ML_MODEL_ID, DATASOURCE_ID, DATA_URI | A list of the variables to use in searching or filtering BatchPrediction . CreatedAt - Sets the search criteria to BatchPrediction creation date. Status - S ... |
| `DataSourceFilterVariable` | `enum` | CREATED_AT, LAST_UPDATED_AT, STATUS, NAME, DATA_URI, IAM_USER | A list of the variables to use in searching or filtering DataSource . CreatedAt - Sets the search criteria to DataSource creation date. Status - Sets the se ... |
| `DetailsAttributes` | `enum` | PREDICTIVE_MODEL_TYPE, ALGORITHM | Contains the key values of DetailsMap : PredictiveModelType - Indicates the type of the MLModel . Algorithm - Indicates the algorithm that was used for the ... |
| `EntityStatus` | `enum` | PENDING, INPROGRESS, FAILED, COMPLETED, DELETED | Object status with the following possible values: PENDING INPROGRESS FAILED COMPLETED DELETED |
| `EvaluationFilterVariable` | `enum` | CREATED_AT, LAST_UPDATED_AT, STATUS, NAME, IAM_USER, ML_MODEL_ID, DATASOURCE_ID, DATA_URI | A list of the variables to use in searching or filtering Evaluation . CreatedAt - Sets the search criteria to Evaluation creation date. Status - Sets the se ... |
| `MLModelFilterVariable` | `enum` | CREATED_AT, LAST_UPDATED_AT, STATUS, NAME, IAM_USER, TRAINING_DATASOURCE_ID, REAL_TIME_ENDPOINT_STATUS, ML_MODEL_TYPE, ALGORITHM, TRAINING_DATA_URI | - |
| `MLModelType` | `enum` | REGRESSION, BINARY, MULTICLASS | - |
| `RealtimeEndpointStatus` | `enum` | NONE, READY, UPDATING, FAILED | - |
| `SortOrder` | `enum` | ASC, DSC | The sort order specified in a listing condition. Possible values include the following: asc - Present the information in ascending order (from A-Z). dsc - P ... |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

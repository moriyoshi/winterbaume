# Amazon NeptuneData

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Neptune Data API The Amazon Neptune data API provides SDK support for more than 40 of Neptune's data operations, including data loading, query execution, data inquiry, and machine learning. It supports the Gremlin and openCypher query languages, and is available in all SDK languages. It automatically signs API requests and greatly simplifies integrating Neptune into your applications.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon NeptuneData resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon NeptuneData workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Cancel`, `Execute`, `Start` operation families, including `GetEngineStatus`, `GetGremlinQueryStatus`, `GetLoaderJobStatus`, `GetMLDataProcessingJob`, `ListGremlinQueries`, `ListLoaderJobs`.

## Service Identity and Protocol

- AWS model slug: `neptunedata`
- AWS SDK for Rust slug: `neptunedata`
- Model version: `2023-08-01`
- Model file: `vendor/api-models-aws/models/neptunedata/service/2023-08-01/neptunedata-2023-08-01.json`
- SDK ID: `neptunedata`
- Endpoint prefix: `-`
- ARN namespace: `neptune-db`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (14), `List` (7), `Cancel` (6), `Execute` (6), `Start` (4), `Delete` (3), `Manage` (2), `Create` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelGremlinQuery`, `CancelLoaderJob`, `CancelMLDataProcessingJob`, `CancelMLModelTrainingJob`, `CancelMLModelTransformJob`, `CancelOpenCypherQuery`, `CreateMLEndpoint`, `DeleteMLEndpoint`, `DeletePropertygraphStatistics`, `DeleteSparqlStatistics`, `StartLoaderJob`, `StartMLDataProcessingJob`, `StartMLModelTrainingJob`, `StartMLModelTransformJob`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ExecuteOpenCypherExplainQuery`, `GetEngineStatus`, `GetGremlinQueryStatus`, `GetLoaderJobStatus`, `GetMLDataProcessingJob`, `GetMLEndpoint`, `GetMLModelTrainingJob`, `GetMLModelTransformJob`, `GetOpenCypherQueryStatus`, `GetPropertygraphStatistics`, `GetPropertygraphStream`, `GetPropertygraphSummary`, `GetRDFGraphSummary`, `GetSparqlStatistics`, `GetSparqlStream`, `ListGremlinQueries`, `ListLoaderJobs`, `ListMLDataProcessingJobs`, `ListMLEndpoints`, `ListMLModelTrainingJobs`, ... (+2).
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelGremlinQuery`, `CancelLoaderJob`, `CancelMLDataProcessingJob`, `CancelMLModelTrainingJob`, `CancelMLModelTransformJob`, `CancelOpenCypherQuery`, `GetLoaderJobStatus`, `GetMLDataProcessingJob`, `GetMLModelTrainingJob`, `GetMLModelTransformJob`, `ListLoaderJobs`, `ListMLDataProcessingJobs`, `ListMLModelTrainingJobs`, `ListMLModelTransformJobs`, `StartLoaderJob`, `StartMLDataProcessingJob`, `StartMLModelTrainingJob`, `StartMLModelTransformJob`.
- 43 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetEngineStatus`, `GetGremlinQueryStatus`, `GetLoaderJobStatus`, `GetMLDataProcessingJob`, `GetMLEndpoint`, `GetMLModelTrainingJob`, `GetMLModelTransformJob`, `GetOpenCypherQueryStatus`, `GetPropertygraphStatistics`, `GetPropertygraphStream`, `GetPropertygraphSummary`, `GetRDFGraphSummary`, `GetSparqlStatistics`, `GetSparqlStream`
- Traits: `readonly` (14)
- Common required input members in this group: `id`, `loadId`, `queryId`

### List

- Operations: `ListGremlinQueries`, `ListLoaderJobs`, `ListMLDataProcessingJobs`, `ListMLEndpoints`, `ListMLModelTrainingJobs`, `ListMLModelTransformJobs`, `ListOpenCypherQueries`
- Traits: `readonly` (7)

### Cancel

- Operations: `CancelGremlinQuery`, `CancelLoaderJob`, `CancelMLDataProcessingJob`, `CancelMLModelTrainingJob`, `CancelMLModelTransformJob`, `CancelOpenCypherQuery`
- Traits: `idempotent` (6)
- Common required input members in this group: `id`, `loadId`, `queryId`

### Execute

- Operations: `ExecuteFastReset`, `ExecuteGremlinExplainQuery`, `ExecuteGremlinProfileQuery`, `ExecuteGremlinQuery`, `ExecuteOpenCypherExplainQuery`, `ExecuteOpenCypherQuery`
- Traits: `idempotent` (1), `readonly` (1)
- Common required input members in this group: `action`, `explainMode`, `gremlinQuery`, `openCypherQuery`

### Start

- Operations: `StartLoaderJob`, `StartMLDataProcessingJob`, `StartMLModelTrainingJob`, `StartMLModelTransformJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `dataProcessingJobId`, `format`, `iamRoleArn`, `inputDataS3Location`, `modelTransformOutputS3Location`, `processedDataS3Location`, `s3BucketRegion`, `source`, `trainModelS3Location`

### Delete

- Operations: `DeleteMLEndpoint`, `DeletePropertygraphStatistics`, `DeleteSparqlStatistics`
- Traits: `idempotent` (3)
- Common required input members in this group: `id`

### Manage

- Operations: `ManagePropertygraphStatistics`, `ManageSparqlStatistics`
- Traits: `idempotent` (2)

### Create

- Operations: `CreateMLEndpoint`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelGremlinQuery` | `DELETE /gremlin/status/{queryId}` | `idempotent` | `queryId` | - | `CancelGremlinQueryOutput` | `BadRequestException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, ... (+6) | Cancels a Gremlin query. See Gremlin query cancellation for more information. |
| `CancelLoaderJob` | `DELETE /loader/{loadId}` | `idempotent` | `loadId` | - | `CancelLoaderJobOutput` | `BadRequestException`, `BulkLoadIdNotFoundException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InternalFailureException`, `InvalidArgumentException`, `InvalidParameterException`, ... (+5) | Cancels a specified load job. This is an HTTP `DELETE` request. |
| `CancelMLDataProcessingJob` | `DELETE /ml/dataprocessing/{id}` | `idempotent` | `id` | - | `CancelMLDataProcessingJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Cancels a Neptune ML data processing job. See The `dataprocessing` command. |
| `CancelMLModelTrainingJob` | `DELETE /ml/modeltraining/{id}` | `idempotent` | `id` | - | `CancelMLModelTrainingJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Cancels a Neptune ML model training job. See Model training using the `modeltraining` command. |
| `CancelMLModelTransformJob` | `DELETE /ml/modeltransform/{id}` | `idempotent` | `id` | - | `CancelMLModelTransformJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Cancels a specified model transform job. See Use a trained model to generate new model artifacts. |
| `CancelOpenCypherQuery` | `DELETE /opencypher/status/{queryId}` | `idempotent` | `queryId` | - | `CancelOpenCypherQueryOutput` | `BadRequestException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidNumericDataException`, ... (+7) | Cancels a specified openCypher query. See Neptune openCypher status endpoint for more information. |
| `CreateMLEndpoint` | `POST /ml/endpoints` | - | - | - | `CreateMLEndpointOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Creates a new Neptune ML inference endpoint that lets you query one specific model that the model-training process constructed. See Managing inference endpoints using the endpoints command. |
| `DeleteMLEndpoint` | `DELETE /ml/endpoints/{id}` | `idempotent` | `id` | - | `DeleteMLEndpointOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Cancels the creation of a Neptune ML inference endpoint. See Managing inference endpoints using the endpoints command. |
| `DeletePropertygraphStatistics` | `DELETE /propertygraph/statistics` | `idempotent` | - | - | `DeletePropertygraphStatisticsOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Deletes statistics for Gremlin and openCypher (property graph) data. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the... |
| `DeleteSparqlStatistics` | `DELETE /sparql/statistics` | `idempotent` | - | - | `DeleteSparqlStatisticsOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Deletes SPARQL statistics When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:DeleteStatistics IAM action in that cluster. |
| `ExecuteFastReset` | `POST /system` | `idempotent` | `action` | - | `ExecuteFastResetOutput` | `AccessDeniedException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MethodNotAllowedException`, `MissingParameterException`, ... (+5) | The fast reset REST API lets you reset a Neptune graph quicky and easily, removing all of its data. Neptune fast reset is a two-step process. |
| `ExecuteGremlinExplainQuery` | `POST /gremlin/explain` | - | `gremlinQuery` | - | `ExecuteGremlinExplainQueryOutput` | `BadRequestException`, `CancelledByUserException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+12) | Executes a Gremlin Explain query. Amazon Neptune has added a Gremlin feature named `explain` that provides is a self-service tool for understanding the execution approach being taken by the Neptune engine for the query. |
| `ExecuteGremlinProfileQuery` | `POST /gremlin/profile` | - | `gremlinQuery` | - | `ExecuteGremlinProfileQueryOutput` | `BadRequestException`, `CancelledByUserException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+12) | Executes a Gremlin Profile query, which runs a specified traversal, collects various metrics about the run, and produces a profile report as output. See Gremlin profile API in Neptune for details. |
| `ExecuteGremlinQuery` | `POST /gremlin` | - | `gremlinQuery` | - | `ExecuteGremlinQueryOutput` | `BadRequestException`, `CancelledByUserException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+12) | This commands executes a Gremlin query. Amazon Neptune is compatible with Apache TinkerPop3 and Gremlin, so you can use the Gremlin traversal language to query the graph, as described under The Graph in the Apache TinkerPop3 documentation. |
| `ExecuteOpenCypherExplainQuery` | `POST /opencypher/explain` | `readonly` | `explainMode`, `openCypherQuery` | - | `ExecuteOpenCypherExplainQueryOutput` | `BadRequestException`, `CancelledByUserException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+13) | Executes an openCypher `explain` request. See The openCypher explain feature for more information. |
| `ExecuteOpenCypherQuery` | `POST /opencypher` | - | `openCypherQuery` | - | `ExecuteOpenCypherQueryOutput` | `BadRequestException`, `CancelledByUserException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+13) | Executes an openCypher query. See Accessing the Neptune Graph with openCypher for more information. |
| `GetEngineStatus` | `GET /status` | `readonly` | - | - | `GetEngineStatusOutput` | `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InternalFailureException`, `InvalidArgumentException`, `PreconditionsFailedException`, `TooManyRequestsException`, `UnsupportedOperationException` | Retrieves the status of the graph database on the host. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:GetEngineStatus IAM... |
| `GetGremlinQueryStatus` | `GET /gremlin/status/{queryId}` | `readonly` | `queryId` | - | `GetGremlinQueryStatusOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+8) | Gets the status of a specified Gremlin query. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:GetQueryStatus IAM action in... |
| `GetLoaderJobStatus` | `GET /loader/{loadId}` | `readonly` | `loadId` | - | `GetLoaderJobStatusOutput` | `BadRequestException`, `BulkLoadIdNotFoundException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InternalFailureException`, `InvalidArgumentException`, `InvalidParameterException`, ... (+5) | Gets status information about a specified load job. Neptune keeps track of the most recent 1,024 bulk load jobs, and stores the last 10,000 error details per job. |
| `GetMLDataProcessingJob` | `GET /ml/dataprocessing/{id}` | `readonly` | `id` | - | `GetMLDataProcessingJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Retrieves information about a specified data processing job. See The `dataprocessing` command. |
| `GetMLEndpoint` | `GET /ml/endpoints/{id}` | `readonly` | `id` | - | `GetMLEndpointOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Retrieves details about an inference endpoint. See Managing inference endpoints using the endpoints command. |
| `GetMLModelTrainingJob` | `GET /ml/modeltraining/{id}` | `readonly` | `id` | - | `GetMLModelTrainingJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Retrieves information about a Neptune ML model training job. See Model training using the `modeltraining` command. |
| `GetMLModelTransformJob` | `GET /ml/modeltransform/{id}` | `readonly` | `id` | - | `GetMLModelTransformJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Gets information about a specified model transform job. See Use a trained model to generate new model artifacts. |
| `GetOpenCypherQueryStatus` | `GET /opencypher/status/{queryId}` | `readonly` | `queryId` | - | `GetOpenCypherQueryStatusOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+9) | Retrieves the status of a specified openCypher query. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:GetQueryStatus IAM... |
| `GetPropertygraphStatistics` | `GET /propertygraph/statistics` | `readonly` | - | - | `GetPropertygraphStatisticsOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Gets property graph statistics (Gremlin and openCypher). When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:GetStatisticsStatus... |
| `GetPropertygraphStream` | `GET /propertygraph/stream` | `readonly` | - | - | `GetPropertygraphStreamOutput` | `ClientTimeoutException`, `ConstraintViolationException`, `ExpiredStreamException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MemoryLimitExceededException`, `PreconditionsFailedException`, ... (+4) | Gets a stream for a property graph. With the Neptune Streams feature, you can generate a complete sequence of change-log entries that record every change made to your graph data as it happens. |
| `GetPropertygraphSummary` | `GET /propertygraph/statistics/summary` | `readonly` | - | - | `GetPropertygraphSummaryOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Gets a graph summary for a property graph. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:GetGraphSummary IAM action in that... |
| `GetRDFGraphSummary` | `GET /rdf/statistics/summary` | `readonly` | - | - | `GetRDFGraphSummaryOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Gets a graph summary for an RDF graph. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:GetGraphSummary IAM action in that... |
| `GetSparqlStatistics` | `GET /sparql/statistics` | `readonly` | - | - | `GetSparqlStatisticsOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Gets RDF statistics (SPARQL). |
| `GetSparqlStream` | `GET /sparql/stream` | `readonly` | - | - | `GetSparqlStreamOutput` | `ClientTimeoutException`, `ConstraintViolationException`, `ExpiredStreamException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MemoryLimitExceededException`, `PreconditionsFailedException`, ... (+4) | Gets a stream for an RDF graph. With the Neptune Streams feature, you can generate a complete sequence of change-log entries that record every change made to your graph data as it happens. |
| `ListGremlinQueries` | `GET /gremlin/status` | `readonly` | - | - | `ListGremlinQueriesOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+8) | Lists active Gremlin queries. See Gremlin query status API for details about the output. |
| `ListLoaderJobs` | `GET /loader` | `readonly` | - | - | `ListLoaderJobsOutput` | `BadRequestException`, `BulkLoadIdNotFoundException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InternalFailureException`, `InvalidArgumentException`, `InvalidParameterException`, ... (+4) | Retrieves a list of the `loadIds` for all active loader jobs. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:ListLoaderJobs... |
| `ListMLDataProcessingJobs` | `GET /ml/dataprocessing` | `readonly` | - | - | `ListMLDataProcessingJobsOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Returns a list of Neptune ML data processing jobs. See Listing active data-processing jobs using the Neptune ML dataprocessing command. |
| `ListMLEndpoints` | `GET /ml/endpoints` | `readonly` | - | - | `ListMLEndpointsOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Lists existing inference endpoints. See Managing inference endpoints using the endpoints command. |
| `ListMLModelTrainingJobs` | `GET /ml/modeltraining` | `readonly` | - | - | `ListMLModelTrainingJobsOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Lists Neptune ML model-training jobs. See Model training using the `modeltraining` command. |
| `ListMLModelTransformJobs` | `GET /ml/modeltransform` | `readonly` | - | - | `ListMLModelTransformJobsOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Returns a list of model transform job IDs. See Use a trained model to generate new model artifacts. |
| `ListOpenCypherQueries` | `GET /opencypher/status` | `readonly` | - | - | `ListOpenCypherQueriesOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FailureByQueryException`, `IllegalArgumentException`, `InvalidArgumentException`, ... (+9) | Lists active openCypher queries. See Neptune openCypher status endpoint for more information. |
| `ManagePropertygraphStatistics` | `POST /propertygraph/statistics` | `idempotent` | - | - | `ManagePropertygraphStatisticsOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Manages the generation and use of property graph statistics. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:ManageStatistics... |
| `ManageSparqlStatistics` | `POST /sparql/statistics` | `idempotent` | - | - | `ManageSparqlStatisticsOutput` | `AccessDeniedException`, `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MissingParameterException`, ... (+5) | Manages the generation and use of RDF graph statistics. When invoking this operation in a Neptune cluster that has IAM authentication enabled, the IAM user or role making the request must have a policy attached that allows the neptune-db:ManageStatistics IAM... |
| `StartLoaderJob` | `POST /loader` | `idempotent` | `format`, `iamRoleArn`, `s3BucketRegion`, `source` | - | `StartLoaderJobOutput` | `BadRequestException`, `BulkLoadIdNotFoundException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InternalFailureException`, `InvalidArgumentException`, `InvalidParameterException`, ... (+6) | Starts a Neptune bulk loader job to load data from an Amazon S3 bucket into a Neptune DB instance. See Using the Amazon Neptune Bulk Loader to Ingest Data. |
| `StartMLDataProcessingJob` | `POST /ml/dataprocessing` | - | `inputDataS3Location`, `processedDataS3Location` | - | `StartMLDataProcessingJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Creates a new Neptune ML data processing job for processing the graph data exported from Neptune for training. See The `dataprocessing` command. |
| `StartMLModelTrainingJob` | `POST /ml/modeltraining` | - | `dataProcessingJobId`, `trainModelS3Location` | - | `StartMLModelTrainingJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Creates a new Neptune ML model training job. See Model training using the `modeltraining` command. |
| `StartMLModelTransformJob` | `POST /ml/modeltransform` | - | `modelTransformOutputS3Location` | - | `StartMLModelTransformJobOutput` | `BadRequestException`, `ClientTimeoutException`, `ConstraintViolationException`, `IllegalArgumentException`, `InvalidArgumentException`, `InvalidParameterException`, `MLResourceNotFoundException`, `MissingParameterException`, ... (+3) | Creates a new model transform job. See Use a trained model to generate new model artifacts. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClientTimeoutException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request timed out in the client. |
| `ConstraintViolationException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a value in a request field did not satisfy required constraints. |
| `IllegalArgumentException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when an argument in a request is not supported. |
| `InvalidArgumentException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when an argument in a request has an invalid value. |
| `PreconditionsFailedException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a precondition for processing a request is not satisfied. |
| `TooManyRequestsException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when the number of requests being processed exceeds the limit. |
| `UnsupportedOperationException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request attempts to initiate an operation that is not supported. |
| `InvalidParameterException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a parameter value is not valid. |
| `BadRequestException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request is submitted that cannot be processed. |
| `MissingParameterException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a required parameter is missing. |
| `MLResourceNotFoundException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a specified machine-learning resource could not be found. |
| `AccessDeniedException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised in case of an authentication or authorization failure. |
| `ReadOnlyViolationException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request attempts to write to a read-only resource. |
| `ConcurrentModificationException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request attempts to modify data that is concurrently being modified by another process. |
| `FailureByQueryException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request fails. |
| `ParsingException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a parsing issue is encountered. |
| `TimeLimitExceededException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when the an operation exceeds the time limit allowed for it. |
| `StatisticsNotAvailableException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when statistics needed to satisfy a request are not available. |
| `MemoryLimitExceededException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a request fails because of insufficient memory resources. |
| `InternalFailureException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when the processing of the request failed unexpectedly. |
| `InvalidNumericDataException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when invalid numerical data is encountered when servicing a request. |
| `CancelledByUserException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a user cancelled a request. |
| `MalformedQueryException` | `structure` | `code`, `detailedMessage`, `requestId` | Raised when a query is submitted that is syntactically incorrect or does not pass additional validation. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

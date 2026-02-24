# Amazon Personalize

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Personalize is a machine learning service that makes it easy to add individualized recommendations to customers.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Personalize resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Personalize workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Create`, `Delete`, `Update` operation families, including `DescribeAlgorithm`, `DescribeBatchInferenceJob`, `DescribeBatchSegmentJob`, `DescribeCampaign`, `ListBatchInferenceJobs`, `ListBatchSegmentJobs`.

## Service Identity and Protocol

- AWS model slug: `personalize`
- AWS SDK for Rust slug: `personalize`
- Model version: `2018-05-22`
- Model file: `vendor/api-models-aws/models/personalize/service/2018-05-22/personalize-2018-05-22.json`
- SDK ID: `Personalize`
- Endpoint prefix: `personalize`
- ARN namespace: `personalize`
- CloudFormation name: `Personalize`
- CloudTrail event source: `personalize.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (18), `List` (18), `Create` (15), `Delete` (9), `Update` (5), `Stop` (2), `Get` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBatchInferenceJob`, `CreateBatchSegmentJob`, `CreateCampaign`, `CreateDataDeletionJob`, `CreateDataset`, `CreateDatasetExportJob`, `CreateDatasetGroup`, `CreateDatasetImportJob`, `CreateEventTracker`, `CreateFilter`, `CreateMetricAttribution`, `CreateRecommender`, `CreateSchema`, `CreateSolution`, `CreateSolutionVersion`, `DeleteCampaign`, `DeleteDataset`, `DeleteDatasetGroup`, `DeleteEventTracker`, `DeleteFilter`, ... (+14).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAlgorithm`, `DescribeBatchInferenceJob`, `DescribeBatchSegmentJob`, `DescribeCampaign`, `DescribeDataDeletionJob`, `DescribeDataset`, `DescribeDatasetExportJob`, `DescribeDatasetGroup`, `DescribeDatasetImportJob`, `DescribeEventTracker`, `DescribeFeatureTransformation`, `DescribeFilter`, `DescribeMetricAttribution`, `DescribeRecipe`, `DescribeRecommender`, `DescribeSchema`, `DescribeSolution`, `DescribeSolutionVersion`, `GetSolutionMetrics`, `ListBatchInferenceJobs`, ... (+17).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 54 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateBatchInferenceJob`, `CreateBatchSegmentJob`, `CreateDataDeletionJob`, `CreateDatasetExportJob`, `CreateDatasetImportJob`, `DescribeBatchInferenceJob`, `DescribeBatchSegmentJob`, `DescribeDataDeletionJob`, `DescribeDatasetExportJob`, `DescribeDatasetImportJob`, `ListBatchInferenceJobs`, `ListBatchSegmentJobs`, `ListDataDeletionJobs`, `ListDatasetExportJobs`, `ListDatasetImportJobs`, `StartRecommender`, `StopRecommender`, `StopSolutionVersionCreation`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 71 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAlgorithm`, `DescribeBatchInferenceJob`, `DescribeBatchSegmentJob`, `DescribeCampaign`, `DescribeDataDeletionJob`, `DescribeDataset`, `DescribeDatasetExportJob`, `DescribeDatasetGroup`, `DescribeDatasetImportJob`, `DescribeEventTracker`, `DescribeFeatureTransformation`, `DescribeFilter`, `DescribeMetricAttribution`, `DescribeRecipe`, `DescribeRecommender`, `DescribeSchema`, `DescribeSolution`, `DescribeSolutionVersion`
- Traits: `idempotent` (17)
- Common required input members in this group: `algorithmArn`, `batchInferenceJobArn`, `batchSegmentJobArn`, `campaignArn`, `dataDeletionJobArn`, `datasetArn`, `datasetExportJobArn`, `datasetGroupArn`, `datasetImportJobArn`, `eventTrackerArn`, `featureTransformationArn`, `filterArn`, `metricAttributionArn`, `recipeArn`, `recommenderArn`, `schemaArn`, `solutionArn`, `solutionVersionArn`

### List

- Operations: `ListBatchInferenceJobs`, `ListBatchSegmentJobs`, `ListCampaigns`, `ListDataDeletionJobs`, `ListDatasetExportJobs`, `ListDatasetGroups`, `ListDatasetImportJobs`, `ListDatasets`, `ListEventTrackers`, `ListFilters`, `ListMetricAttributionMetrics`, `ListMetricAttributions`, `ListRecipes`, `ListRecommenders`, `ListSchemas`, `ListSolutionVersions`, `ListSolutions`, `ListTagsForResource`
- Traits: `idempotent` (17), `paginated` (16)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateBatchInferenceJob`, `CreateBatchSegmentJob`, `CreateCampaign`, `CreateDataDeletionJob`, `CreateDataset`, `CreateDatasetExportJob`, `CreateDatasetGroup`, `CreateDatasetImportJob`, `CreateEventTracker`, `CreateFilter`, `CreateMetricAttribution`, `CreateRecommender`, `CreateSchema`, `CreateSolution`, `CreateSolutionVersion`
- Traits: `idempotent` (6)
- Common required input members in this group: `dataSource`, `datasetArn`, `datasetGroupArn`, `datasetType`, `filterExpression`, `jobInput`, `jobName`, `jobOutput`, `metrics`, `metricsOutputConfig`, `name`, `recipeArn`, `roleArn`, `schema`, `schemaArn`, `solutionArn`, `solutionVersionArn`

### Delete

- Operations: `DeleteCampaign`, `DeleteDataset`, `DeleteDatasetGroup`, `DeleteEventTracker`, `DeleteFilter`, `DeleteMetricAttribution`, `DeleteRecommender`, `DeleteSchema`, `DeleteSolution`
- Traits: `idempotent` (8)
- Common required input members in this group: `campaignArn`, `datasetArn`, `datasetGroupArn`, `eventTrackerArn`, `filterArn`, `metricAttributionArn`, `recommenderArn`, `schemaArn`, `solutionArn`

### Update

- Operations: `UpdateCampaign`, `UpdateDataset`, `UpdateMetricAttribution`, `UpdateRecommender`, `UpdateSolution`
- Traits: `idempotent` (3)
- Common required input members in this group: `campaignArn`, `datasetArn`, `recommenderArn`, `recommenderConfig`, `schemaArn`, `solutionArn`

### Stop

- Operations: `StopRecommender`, `StopSolutionVersionCreation`
- Traits: `idempotent` (2)
- Common required input members in this group: `recommenderArn`, `solutionVersionArn`

### Get

- Operations: `GetSolutionMetrics`
- Common required input members in this group: `solutionVersionArn`

### Start

- Operations: `StartRecommender`
- Traits: `idempotent` (1)
- Common required input members in this group: `recommenderArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateBatchInferenceJob` | - | - | `jobInput`, `jobName`, `jobOutput`, `roleArn`, `solutionVersionArn` | - | `CreateBatchInferenceJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Generates batch recommendations based on a list of items or users stored in Amazon S3 and exports the recommendations to an Amazon S3 bucket. To generate batch recommendations, specify the ARN of a solution version and an Amazon S3 URI for the input and... |
| `CreateBatchSegmentJob` | - | - | `jobInput`, `jobName`, `jobOutput`, `roleArn`, `solutionVersionArn` | - | `CreateBatchSegmentJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates a batch segment job. The operation can handle up to 50 million records and the input file must be in JSON format. |
| `CreateCampaign` | - | `idempotent` | `name`, `solutionVersionArn` | - | `CreateCampaignResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | You incur campaign costs while it is active. To avoid unnecessary costs, make sure to delete the campaign when you are finished. |
| `CreateDataDeletionJob` | - | - | `dataSource`, `datasetGroupArn`, `jobName`, `roleArn` | - | `CreateDataDeletionJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates a batch job that deletes all references to specific users from an Amazon Personalize dataset group in batches. You specify the users to delete in a CSV file of userIds in an Amazon S3 bucket. |
| `CreateDataset` | - | `idempotent` | `datasetGroupArn`, `datasetType`, `name`, `schemaArn` | - | `CreateDatasetResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates an empty dataset and adds it to the specified dataset group. Use CreateDatasetImportJob to import your training data to a dataset. |
| `CreateDatasetExportJob` | - | `idempotent` | `datasetArn`, `jobName`, `jobOutput`, `roleArn` | - | `CreateDatasetExportJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates a job that exports data from your dataset to an Amazon S3 bucket. To allow Amazon Personalize to export the training data, you must specify an service-linked IAM role that gives Amazon Personalize `PutObject` permissions for your Amazon S3 bucket. |
| `CreateDatasetGroup` | - | - | `name` | - | `CreateDatasetGroupResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `TooManyTagsException` | Creates an empty dataset group. A dataset group is a container for Amazon Personalize resources. |
| `CreateDatasetImportJob` | - | - | `dataSource`, `datasetArn`, `jobName` | - | `CreateDatasetImportJobResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates a job that imports training data from your data source (an Amazon S3 bucket) to an Amazon Personalize dataset. To allow Amazon Personalize to import the training data, you must specify an IAM service role that has permission to read from the data... |
| `CreateEventTracker` | - | `idempotent` | `datasetGroupArn`, `name` | - | `CreateEventTrackerResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates an event tracker that you use when adding event data to a specified dataset group using the PutEvents API. Only one event tracker can be associated with a dataset group. |
| `CreateFilter` | - | - | `datasetGroupArn`, `filterExpression`, `name` | - | `CreateFilterResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates a recommendation filter. For more information, see Filtering recommendations and user segments. |
| `CreateMetricAttribution` | - | - | `datasetGroupArn`, `metrics`, `metricsOutputConfig`, `name` | - | `CreateMetricAttributionResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Creates a metric attribution. A metric attribution creates reports on the data that you import into Amazon Personalize. |
| `CreateRecommender` | - | `idempotent` | `datasetGroupArn`, `name`, `recipeArn` | - | `CreateRecommenderResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Creates a recommender with the recipe (a Domain dataset group use case) you specify. You create recommenders for a Domain dataset group and specify the recommender's Amazon Resource Name (ARN) when you make a GetRecommendations request. |
| `CreateSchema` | - | `idempotent` | `name`, `schema` | - | `CreateSchemaResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates an Amazon Personalize schema from the specified schema string. The schema you create must be in Avro JSON format. |
| `CreateSolution` | - | - | `datasetGroupArn`, `name` | - | `CreateSolutionResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | By default, all new solutions use automatic training. With automatic training, you incur training costs while your solution is active. |
| `CreateSolutionVersion` | - | - | `solutionArn` | - | `CreateSolutionVersionResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Trains or retrains an active solution in a Custom dataset group. A solution is created using the CreateSolution operation and must be in the ACTIVE state before calling `CreateSolutionVersion`. |
| `DeleteCampaign` | - | `idempotent` | `campaignArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Removes a campaign by deleting the solution deployment. The solution that the campaign is based on is not deleted and can be redeployed when needed. |
| `DeleteDataset` | - | `idempotent` | `datasetArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a dataset. You can't delete a dataset if an associated `DatasetImportJob` or `SolutionVersion` is in the CREATE PENDING or IN PROGRESS state. |
| `DeleteDatasetGroup` | - | `idempotent` | `datasetGroupArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a dataset group. Before you delete a dataset group, you must delete the following: All associated event trackers. |
| `DeleteEventTracker` | - | `idempotent` | `eventTrackerArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the event tracker. Does not delete the dataset from the dataset group. |
| `DeleteFilter` | - | - | `filterArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a filter. |
| `DeleteMetricAttribution` | - | `idempotent` | `metricAttributionArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a metric attribution. |
| `DeleteRecommender` | - | `idempotent` | `recommenderArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deactivates and removes a recommender. A deleted recommender can no longer be specified in a GetRecommendations request. |
| `DeleteSchema` | - | `idempotent` | `schemaArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a schema. Before deleting a schema, you must delete all datasets referencing the schema. |
| `DeleteSolution` | - | `idempotent` | `solutionArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes all versions of a solution and the `Solution` object itself. Before deleting a solution, you must delete all campaigns based on the solution. |
| `DescribeAlgorithm` | - | `idempotent` | `algorithmArn` | - | `DescribeAlgorithmResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the given algorithm. |
| `DescribeBatchInferenceJob` | - | `idempotent` | `batchInferenceJobArn` | - | `DescribeBatchInferenceJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Gets the properties of a batch inference job including name, Amazon Resource Name (ARN), status, input and output configurations, and the ARN of the solution version used to generate the recommendations. |
| `DescribeBatchSegmentJob` | - | `idempotent` | `batchSegmentJobArn` | - | `DescribeBatchSegmentJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Gets the properties of a batch segment job including name, Amazon Resource Name (ARN), status, input and output configurations, and the ARN of the solution version used to generate segments. |
| `DescribeCampaign` | - | `idempotent` | `campaignArn` | - | `DescribeCampaignResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the given campaign, including its status. A campaign can be in one of the following states: CREATE PENDING > CREATE IN_PROGRESS > ACTIVE -or- CREATE FAILED DELETE PENDING > DELETE IN_PROGRESS When the `status` is `CREATE FAILED`, the response... |
| `DescribeDataDeletionJob` | - | `idempotent` | `dataDeletionJobArn` | - | `DescribeDataDeletionJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the data deletion job created by CreateDataDeletionJob, including the job status. |
| `DescribeDataset` | - | `idempotent` | `datasetArn` | - | `DescribeDatasetResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the given dataset. For more information on datasets, see CreateDataset. |
| `DescribeDatasetExportJob` | - | `idempotent` | `datasetExportJobArn` | - | `DescribeDatasetExportJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the dataset export job created by CreateDatasetExportJob, including the export job status. |
| `DescribeDatasetGroup` | - | `idempotent` | `datasetGroupArn` | - | `DescribeDatasetGroupResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the given dataset group. For more information on dataset groups, see CreateDatasetGroup. |
| `DescribeDatasetImportJob` | - | `idempotent` | `datasetImportJobArn` | - | `DescribeDatasetImportJobResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the dataset import job created by CreateDatasetImportJob, including the import job status. |
| `DescribeEventTracker` | - | `idempotent` | `eventTrackerArn` | - | `DescribeEventTrackerResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes an event tracker. The response includes the `trackingId` and `status` of the event tracker. |
| `DescribeFeatureTransformation` | - | `idempotent` | `featureTransformationArn` | - | `DescribeFeatureTransformationResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the given feature transformation. |
| `DescribeFilter` | - | `idempotent` | `filterArn` | - | `DescribeFilterResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a filter's properties. |
| `DescribeMetricAttribution` | - | - | `metricAttributionArn` | - | `DescribeMetricAttributionResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a metric attribution. |
| `DescribeRecipe` | - | `idempotent` | `recipeArn` | - | `DescribeRecipeResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a recipe. A recipe contains three items: An algorithm that trains a model. |
| `DescribeRecommender` | - | `idempotent` | `recommenderArn` | - | `DescribeRecommenderResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes the given recommender, including its status. A recommender can be in one of the following states: CREATE PENDING > CREATE IN_PROGRESS > ACTIVE -or- CREATE FAILED STOP PENDING > STOP IN_PROGRESS > INACTIVE > START PENDING > START IN_PROGRESS > ACTIVE... |
| `DescribeSchema` | - | `idempotent` | `schemaArn` | - | `DescribeSchemaResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a schema. For more information on schemas, see CreateSchema. |
| `DescribeSolution` | - | `idempotent` | `solutionArn` | - | `DescribeSolutionResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a solution. For more information on solutions, see CreateSolution. |
| `DescribeSolutionVersion` | - | `idempotent` | `solutionVersionArn` | - | `DescribeSolutionVersionResponse` | `InvalidInputException`, `ResourceNotFoundException` | Describes a specific version of a solution. For more information on solutions, see CreateSolution |
| `GetSolutionMetrics` | - | - | `solutionVersionArn` | - | `GetSolutionMetricsResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Gets the metrics for the specified solution version. |
| `ListBatchInferenceJobs` | - | `idempotent`, `paginated` | - | - | `ListBatchInferenceJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Gets a list of the batch inference jobs that have been performed off of a solution version. |
| `ListBatchSegmentJobs` | - | `idempotent`, `paginated` | - | - | `ListBatchSegmentJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Gets a list of the batch segment jobs that have been performed off of a solution version that you specify. |
| `ListCampaigns` | - | `idempotent`, `paginated` | - | - | `ListCampaignsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of campaigns that use the given solution. When a solution is not specified, all the campaigns associated with the account are listed. |
| `ListDataDeletionJobs` | - | `idempotent` | - | - | `ListDataDeletionJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of data deletion jobs for a dataset group ordered by creation time, with the most recent first. When a dataset group is not specified, all the data deletion jobs associated with the account are listed. |
| `ListDatasetExportJobs` | - | `idempotent`, `paginated` | - | - | `ListDatasetExportJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of dataset export jobs that use the given dataset. When a dataset is not specified, all the dataset export jobs associated with the account are listed. |
| `ListDatasetGroups` | - | `idempotent`, `paginated` | - | - | `ListDatasetGroupsResponse` | `InvalidNextTokenException` | Returns a list of dataset groups. The response provides the properties for each dataset group, including the Amazon Resource Name (ARN). |
| `ListDatasetImportJobs` | - | `idempotent`, `paginated` | - | - | `ListDatasetImportJobsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of dataset import jobs that use the given dataset. When a dataset is not specified, all the dataset import jobs associated with the account are listed. |
| `ListDatasets` | - | `idempotent`, `paginated` | - | - | `ListDatasetsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns the list of datasets contained in the given dataset group. The response provides the properties for each dataset, including the Amazon Resource Name (ARN). |
| `ListEventTrackers` | - | `idempotent`, `paginated` | - | - | `ListEventTrackersResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns the list of event trackers associated with the account. The response provides the properties for each event tracker, including the Amazon Resource Name (ARN) and tracking ID. |
| `ListFilters` | - | `idempotent`, `paginated` | - | - | `ListFiltersResponse` | `InvalidInputException`, `InvalidNextTokenException` | Lists all filters that belong to a given dataset group. |
| `ListMetricAttributionMetrics` | - | `idempotent`, `paginated` | - | - | `ListMetricAttributionMetricsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Lists the metrics for the metric attribution. |
| `ListMetricAttributions` | - | `idempotent`, `paginated` | - | - | `ListMetricAttributionsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Lists metric attributions. |
| `ListRecipes` | - | `idempotent`, `paginated` | - | - | `ListRecipesResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of available recipes. The response provides the properties for each recipe, including the recipe's Amazon Resource Name (ARN). |
| `ListRecommenders` | - | `idempotent`, `paginated` | - | - | `ListRecommendersResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of recommenders in a given Domain dataset group. When a Domain dataset group is not specified, all the recommenders associated with the account are listed. |
| `ListSchemas` | - | `idempotent`, `paginated` | - | - | `ListSchemasResponse` | `InvalidNextTokenException` | Returns the list of schemas associated with the account. The response provides the properties for each schema, including the Amazon Resource Name (ARN). |
| `ListSolutionVersions` | - | `idempotent`, `paginated` | - | - | `ListSolutionVersionsResponse` | `InvalidInputException`, `InvalidNextTokenException`, `ResourceNotFoundException` | Returns a list of solution versions for the given solution. When a solution is not specified, all the solution versions associated with the account are listed. |
| `ListSolutions` | - | `idempotent`, `paginated` | - | - | `ListSolutionsResponse` | `InvalidInputException`, `InvalidNextTokenException` | Returns a list of solutions in a given dataset group. When a dataset group is not specified, all the solutions associated with the account are listed. |
| `ListTagsForResource` | - | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Get a list of tags attached to a resource. |
| `StartRecommender` | - | `idempotent` | `recommenderArn` | - | `StartRecommenderResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Starts a recommender that is INACTIVE. Starting a recommender does not create any new models, but resumes billing and automatic retraining for the recommender. |
| `StopRecommender` | - | `idempotent` | `recommenderArn` | - | `StopRecommenderResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Stops a recommender that is ACTIVE. Stopping a recommender halts billing and automatic retraining for the recommender. |
| `StopSolutionVersionCreation` | - | `idempotent` | `solutionVersionArn` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Stops creating a solution version that is in a state of CREATE_PENDING or CREATE IN_PROGRESS. Depending on the current state of the solution version, the solution version state changes as follows: CREATE_PENDING > CREATE_STOPPED or CREATE_IN_PROGRESS >... |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagsException` | Add a list of tags to a resource. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException`, `TooManyTagKeysException` | Removes the specified tags that are attached to a resource. For more information, see Removing tags from Amazon Personalize resources. |
| `UpdateCampaign` | - | `idempotent` | `campaignArn` | - | `UpdateCampaignResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates a campaign to deploy a retrained solution version with an existing campaign, change your campaign's `minProvisionedTPS`, or modify your campaign's configuration. For example, you can set `enableMetadataWithRecommendations` to true for an existing... |
| `UpdateDataset` | - | `idempotent` | `datasetArn`, `schemaArn` | - | `UpdateDatasetResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Update a dataset to replace its schema with a new or existing one. For more information, see Replacing a dataset's schema. |
| `UpdateMetricAttribution` | - | - | - | - | `UpdateMetricAttributionResponse` | `InvalidInputException`, `ResourceAlreadyExistsException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates a metric attribution. |
| `UpdateRecommender` | - | `idempotent` | `recommenderArn`, `recommenderConfig` | - | `UpdateRecommenderResponse` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the recommender to modify the recommender configuration. If you update the recommender to modify the columns used in training, Amazon Personalize automatically starts a full retraining of the models backing your recommender. |
| `UpdateSolution` | - | - | `solutionArn` | - | `UpdateSolutionResponse` | `InvalidInputException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates an Amazon Personalize solution to use a different automatic training configuration. When you update a solution, you can change whether the solution uses automatic training, and you can change the training frequency. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInputException` | `structure` | `message` | Provide a valid value for the field or parameter. |
| `ResourceNotFoundException` | `structure` | `message` | Could not find the specified resource. |
| `ResourceInUseException` | `structure` | `message` | The specified resource is in use. |
| `LimitExceededException` | `structure` | `message` | The limit on the number of requests per second has been exceeded. |
| `InvalidNextTokenException` | `structure` | `message` | The token is not valid. |
| `ResourceAlreadyExistsException` | `structure` | `message` | The specified resource already exists. |
| `TooManyTagsException` | `structure` | `message` | You have exceeded the maximum number of tags you can apply to this resource. |
| `CreateBatchInferenceJobRequest` | `structure` | `batchInferenceJobConfig`, `batchInferenceJobMode`, `filterArn`, `jobInput`, `jobName`, `jobOutput`, `numResults`, `roleArn`, `solutionVersionArn`, `tags`, `themeGenerationConfig` | - |
| `CreateBatchInferenceJobResponse` | `structure` | `batchInferenceJobArn` | - |
| `CreateBatchSegmentJobRequest` | `structure` | `filterArn`, `jobInput`, `jobName`, `jobOutput`, `numResults`, `roleArn`, `solutionVersionArn`, `tags` | - |
| `CreateBatchSegmentJobResponse` | `structure` | `batchSegmentJobArn` | - |
| `CreateCampaignRequest` | `structure` | `campaignConfig`, `minProvisionedTPS`, `name`, `solutionVersionArn`, `tags` | - |
| `CreateCampaignResponse` | `structure` | `campaignArn` | - |
| `CreateDataDeletionJobRequest` | `structure` | `dataSource`, `datasetGroupArn`, `jobName`, `roleArn`, `tags` | - |
| `CreateDataDeletionJobResponse` | `structure` | `dataDeletionJobArn` | - |
| `CreateDatasetRequest` | `structure` | `datasetGroupArn`, `datasetType`, `name`, `schemaArn`, `tags` | - |
| `CreateDatasetResponse` | `structure` | `datasetArn` | - |
| `CreateDatasetExportJobRequest` | `structure` | `datasetArn`, `ingestionMode`, `jobName`, `jobOutput`, `roleArn`, `tags` | - |
| `CreateDatasetExportJobResponse` | `structure` | `datasetExportJobArn` | - |
| `CreateDatasetGroupRequest` | `structure` | `domain`, `kmsKeyArn`, `name`, `roleArn`, `tags` | - |
| `CreateDatasetGroupResponse` | `structure` | `datasetGroupArn`, `domain` | - |
| `CreateDatasetImportJobRequest` | `structure` | `dataSource`, `datasetArn`, `importMode`, `jobName`, `publishAttributionMetricsToS3`, `roleArn`, `tags` | - |
| `CreateDatasetImportJobResponse` | `structure` | `datasetImportJobArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

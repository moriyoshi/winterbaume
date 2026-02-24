# AWS Glue DataBrew

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Glue DataBrew is a visual, cloud-scale data-preparation service. DataBrew simplifies data preparation tasks, targeting data issues that are hard to spot and time-consuming to fix. DataBrew empowers users of all technical levels to visualize the data and perform one-click data transformations, with no coding required.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Glue DataBrew resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Glue DataBrew workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Describe`, `Update`, `Delete` operation families, including `ListDatasets`, `ListJobRuns`, `ListJobs`, `ListProjects`, `CreateDataset`, `CreateProfileJob`.

## Service Identity and Protocol

- AWS model slug: `databrew`
- AWS SDK for Rust slug: `databrew`
- Model version: `2017-07-25`
- Model file: `vendor/api-models-aws/models/databrew/service/2017-07-25/databrew-2017-07-25.json`
- SDK ID: `DataBrew`
- Endpoint prefix: `databrew`
- ARN namespace: `databrew`
- CloudFormation name: `DataBrew`
- CloudTrail event source: `databrew.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (9), `Create` (7), `Describe` (7), `Update` (7), `Delete` (6), `Start` (2), `Batch` (1), `Publish` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDeleteRecipeVersion`, `CreateDataset`, `CreateProfileJob`, `CreateProject`, `CreateRecipe`, `CreateRecipeJob`, `CreateRuleset`, `CreateSchedule`, `DeleteDataset`, `DeleteJob`, `DeleteProject`, `DeleteRecipeVersion`, `DeleteRuleset`, `DeleteSchedule`, `StartJobRun`, `StartProjectSession`, `StopJobRun`, `TagResource`, `UntagResource`, `UpdateDataset`, ... (+6).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDataset`, `DescribeJob`, `DescribeJobRun`, `DescribeProject`, `DescribeRecipe`, `DescribeRuleset`, `DescribeSchedule`, `ListDatasets`, `ListJobRuns`, `ListJobs`, `ListProjects`, `ListRecipeVersions`, `ListRecipes`, `ListRulesets`, `ListSchedules`, `ListTagsForResource`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateProfileJob`, `CreateRecipeJob`, `DeleteJob`, `DescribeJob`, `DescribeJobRun`, `ListJobRuns`, `ListJobs`, `StartJobRun`, `StartProjectSession`, `StopJobRun`, `UpdateProfileJob`, `UpdateRecipeJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 44 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `Glue`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/databrew/latest/dg/what-is.html
- https://docs.aws.amazon.com/databrew/latest/dg/getting-started.html
- https://docs.aws.amazon.com/cli/latest/reference/databrew/

Research outcomes:
- AWS Glue DataBrew is a visual data preparation service for cleaning, normalising, profiling, and transforming datasets.
- Core resources include datasets, projects, recipes, recipe versions, jobs, schedules, rulesets, profiles, and outputs.
- Projects provide interactive data preparation over a dataset and build recipes from transformation steps.
- Recipe jobs apply saved recipes to datasets and write transformed data to destinations such as S3.
- Profile jobs analyse datasets and generate data quality/statistical profiles.
- DataBrew is serverless from the user's perspective and tracks job status asynchronously.

Parity implications:
- Model datasets, projects, recipes, recipe versions, jobs, schedules, rulesets, profiles, outputs, and job runs separately.
- Job execution should be asynchronous and should depend on dataset connectivity and recipe validity.
- Recipe versioning should preserve historical transformation steps.

## Operation Groups

### List

- Operations: `ListDatasets`, `ListJobRuns`, `ListJobs`, `ListProjects`, `ListRecipeVersions`, `ListRecipes`, `ListRulesets`, `ListSchedules`, `ListTagsForResource`
- Traits: `paginated` (8)
- Common required input members in this group: `Name`, `ResourceArn`

### Create

- Operations: `CreateDataset`, `CreateProfileJob`, `CreateProject`, `CreateRecipe`, `CreateRecipeJob`, `CreateRuleset`, `CreateSchedule`
- Common required input members in this group: `CronExpression`, `DatasetName`, `Input`, `Name`, `OutputLocation`, `RecipeName`, `RoleArn`, `Rules`, `Steps`, `TargetArn`

### Describe

- Operations: `DescribeDataset`, `DescribeJob`, `DescribeJobRun`, `DescribeProject`, `DescribeRecipe`, `DescribeRuleset`, `DescribeSchedule`
- Common required input members in this group: `Name`, `RunId`

### Update

- Operations: `UpdateDataset`, `UpdateProfileJob`, `UpdateProject`, `UpdateRecipe`, `UpdateRecipeJob`, `UpdateRuleset`, `UpdateSchedule`
- Common required input members in this group: `CronExpression`, `Input`, `Name`, `OutputLocation`, `RoleArn`, `Rules`

### Delete

- Operations: `DeleteDataset`, `DeleteJob`, `DeleteProject`, `DeleteRecipeVersion`, `DeleteRuleset`, `DeleteSchedule`
- Common required input members in this group: `Name`, `RecipeVersion`

### Start

- Operations: `StartJobRun`, `StartProjectSession`
- Common required input members in this group: `Name`

### Batch

- Operations: `BatchDeleteRecipeVersion`
- Common required input members in this group: `Name`, `RecipeVersions`

### Publish

- Operations: `PublishRecipe`
- Common required input members in this group: `Name`

### Send

- Operations: `SendProjectSessionAction`
- Common required input members in this group: `Name`

### Stop

- Operations: `StopJobRun`
- Common required input members in this group: `Name`, `RunId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchDeleteRecipeVersion` | `POST /recipes/{Name}/batchDeleteRecipeVersion` | - | `Name`, `RecipeVersions` | - | `BatchDeleteRecipeVersionResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes one or more versions of a recipe at a time. The entire request will be rejected if: The recipe does not exist. |
| `CreateDataset` | `POST /datasets` | - | `Input`, `Name` | - | `CreateDatasetResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new DataBrew dataset. |
| `CreateProfileJob` | `POST /profileJobs` | - | `DatasetName`, `Name`, `OutputLocation`, `RoleArn` | - | `CreateProfileJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new job to analyze a dataset and create its data profile. |
| `CreateProject` | `POST /projects` | - | `DatasetName`, `Name`, `RecipeName`, `RoleArn` | - | `CreateProjectResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new DataBrew project. |
| `CreateRecipe` | `POST /recipes` | - | `Name`, `Steps` | - | `CreateRecipeResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new DataBrew recipe. |
| `CreateRecipeJob` | `POST /recipeJobs` | - | `Name`, `RoleArn` | - | `CreateRecipeJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new job to transform input data, using steps defined in an existing Glue DataBrew recipe |
| `CreateRuleset` | `POST /rulesets` | - | `Name`, `Rules`, `TargetArn` | - | `CreateRulesetResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new ruleset that can be used in a profile job to validate the data quality of a dataset. |
| `CreateSchedule` | `POST /schedules` | - | `CronExpression`, `Name` | - | `CreateScheduleResponse` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new schedule for one or more DataBrew jobs. Jobs can be run at a specific date and time, or at regular intervals. |
| `DeleteDataset` | `DELETE /datasets/{Name}` | - | `Name` | - | `DeleteDatasetResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes a dataset from DataBrew. |
| `DeleteJob` | `DELETE /jobs/{Name}` | - | `Name` | - | `DeleteJobResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified DataBrew job. |
| `DeleteProject` | `DELETE /projects/{Name}` | - | `Name` | - | `DeleteProjectResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes an existing DataBrew project. |
| `DeleteRecipeVersion` | `DELETE /recipes/{Name}/recipeVersion/{RecipeVersion}` | - | `Name`, `RecipeVersion` | - | `DeleteRecipeVersionResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes a single version of a DataBrew recipe. |
| `DeleteRuleset` | `DELETE /rulesets/{Name}` | - | `Name` | - | `DeleteRulesetResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes a ruleset. |
| `DeleteSchedule` | `DELETE /schedules/{Name}` | - | `Name` | - | `DeleteScheduleResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes the specified DataBrew schedule. |
| `DescribeDataset` | `GET /datasets/{Name}` | - | `Name` | - | `DescribeDatasetResponse` | `ResourceNotFoundException`, `ValidationException` | Returns the definition of a specific DataBrew dataset. |
| `DescribeJob` | `GET /jobs/{Name}` | - | `Name` | - | `DescribeJobResponse` | `ResourceNotFoundException`, `ValidationException` | Returns the definition of a specific DataBrew job. |
| `DescribeJobRun` | `GET /jobs/{Name}/jobRun/{RunId}` | - | `Name`, `RunId` | - | `DescribeJobRunResponse` | `ResourceNotFoundException`, `ValidationException` | Represents one run of a DataBrew job. |
| `DescribeProject` | `GET /projects/{Name}` | - | `Name` | - | `DescribeProjectResponse` | `ResourceNotFoundException`, `ValidationException` | Returns the definition of a specific DataBrew project. |
| `DescribeRecipe` | `GET /recipes/{Name}` | - | `Name` | - | `DescribeRecipeResponse` | `ResourceNotFoundException`, `ValidationException` | Returns the definition of a specific DataBrew recipe corresponding to a particular version. |
| `DescribeRuleset` | `GET /rulesets/{Name}` | - | `Name` | - | `DescribeRulesetResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves detailed information about the ruleset. |
| `DescribeSchedule` | `GET /schedules/{Name}` | - | `Name` | - | `DescribeScheduleResponse` | `ResourceNotFoundException`, `ValidationException` | Returns the definition of a specific DataBrew schedule. |
| `ListDatasets` | `GET /datasets` | `paginated` | - | - | `ListDatasetsResponse` | `ValidationException` | Lists all of the DataBrew datasets. |
| `ListJobRuns` | `GET /jobs/{Name}/jobRuns` | `paginated` | `Name` | - | `ListJobRunsResponse` | `ResourceNotFoundException`, `ValidationException` | Lists all of the previous runs of a particular DataBrew job. |
| `ListJobs` | `GET /jobs` | `paginated` | - | - | `ListJobsResponse` | `ValidationException` | Lists all of the DataBrew jobs that are defined. |
| `ListProjects` | `GET /projects` | `paginated` | - | - | `ListProjectsResponse` | `ValidationException` | Lists all of the DataBrew projects that are defined. |
| `ListRecipeVersions` | `GET /recipeVersions` | `paginated` | `Name` | - | `ListRecipeVersionsResponse` | `ValidationException` | Lists the versions of a particular DataBrew recipe, except for `LATEST_WORKING`. |
| `ListRecipes` | `GET /recipes` | `paginated` | - | - | `ListRecipesResponse` | `ValidationException` | Lists all of the DataBrew recipes that are defined. |
| `ListRulesets` | `GET /rulesets` | `paginated` | - | - | `ListRulesetsResponse` | `ResourceNotFoundException`, `ValidationException` | List all rulesets available in the current account or rulesets associated with a specific resource (dataset). |
| `ListSchedules` | `GET /schedules` | `paginated` | - | - | `ListSchedulesResponse` | `ValidationException` | Lists the DataBrew schedules that are defined. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all the tags for a DataBrew resource. |
| `PublishRecipe` | `POST /recipes/{Name}/publishRecipe` | - | `Name` | - | `PublishRecipeResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Publishes a new version of a DataBrew recipe. |
| `SendProjectSessionAction` | `PUT /projects/{Name}/sendProjectSessionAction` | - | `Name` | - | `SendProjectSessionActionResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Performs a recipe step within an interactive DataBrew session that's currently open. |
| `StartJobRun` | `POST /jobs/{Name}/startJobRun` | - | `Name` | - | `StartJobRunResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Runs a DataBrew job. |
| `StartProjectSession` | `PUT /projects/{Name}/startProjectSession` | - | `Name` | - | `StartProjectSessionResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an interactive session, enabling you to manipulate data in a DataBrew project. |
| `StopJobRun` | `POST /jobs/{Name}/jobRun/{RunId}/stopJobRun` | - | `Name`, `RunId` | - | `StopJobRunResponse` | `ResourceNotFoundException`, `ValidationException` | Stops a particular run of a job. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds metadata tags to a DataBrew resource, such as a dataset, project, recipe, job, or schedule. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes metadata tags from a DataBrew resource. |
| `UpdateDataset` | `PUT /datasets/{Name}` | - | `Input`, `Name` | - | `UpdateDatasetResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Modifies the definition of an existing DataBrew dataset. |
| `UpdateProfileJob` | `PUT /profileJobs/{Name}` | - | `Name`, `OutputLocation`, `RoleArn` | - | `UpdateProfileJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Modifies the definition of an existing profile job. |
| `UpdateProject` | `PUT /projects/{Name}` | - | `Name`, `RoleArn` | - | `UpdateProjectResponse` | `ResourceNotFoundException`, `ValidationException` | Modifies the definition of an existing DataBrew project. |
| `UpdateRecipe` | `PUT /recipes/{Name}` | - | `Name` | - | `UpdateRecipeResponse` | `ResourceNotFoundException`, `ValidationException` | Modifies the definition of the `LATEST_WORKING` version of a DataBrew recipe. |
| `UpdateRecipeJob` | `PUT /recipeJobs/{Name}` | - | `Name`, `RoleArn` | - | `UpdateRecipeJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Modifies the definition of an existing DataBrew recipe job. |
| `UpdateRuleset` | `PUT /rulesets/{Name}` | - | `Name`, `Rules` | - | `UpdateRulesetResponse` | `ResourceNotFoundException`, `ValidationException` | Updates specified ruleset. |
| `UpdateSchedule` | `PUT /schedules/{Name}` | - | `CronExpression`, `Name` | - | `UpdateScheduleResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Modifies the definition of an existing DataBrew schedule. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | The input parameters for this request failed validation. |
| `ResourceNotFoundException` | `structure` | `Message` | One or more resources can't be found. |
| `ConflictException` | `structure` | `Message` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `Message` | A service quota is exceeded. |
| `AccessDeniedException` | `structure` | `Message` | Access to the specified resource was denied. |
| `InternalServerException` | `structure` | `Message` | An internal service failure occurred. |
| `BatchDeleteRecipeVersionRequest` | `structure` | `Name`, `RecipeVersions` | - |
| `BatchDeleteRecipeVersionResponse` | `structure` | `Errors`, `Name` | - |
| `CreateDatasetRequest` | `structure` | `Format`, `FormatOptions`, `Input`, `Name`, `PathOptions`, `Tags` | - |
| `CreateDatasetResponse` | `structure` | `Name` | - |
| `CreateProfileJobRequest` | `structure` | `Configuration`, `DatasetName`, `EncryptionKeyArn`, `EncryptionMode`, `JobSample`, `LogSubscription`, `MaxCapacity`, `MaxRetries`, `Name`, `OutputLocation`, `RoleArn`, `Tags`, ... (+2) | - |
| `CreateProfileJobResponse` | `structure` | `Name` | - |
| `CreateProjectRequest` | `structure` | `DatasetName`, `Name`, `RecipeName`, `RoleArn`, `Sample`, `Tags` | - |
| `CreateProjectResponse` | `structure` | `Name` | - |
| `CreateRecipeRequest` | `structure` | `Description`, `Name`, `Steps`, `Tags` | - |
| `CreateRecipeResponse` | `structure` | `Name` | - |
| `CreateRecipeJobRequest` | `structure` | `DataCatalogOutputs`, `DatabaseOutputs`, `DatasetName`, `EncryptionKeyArn`, `EncryptionMode`, `LogSubscription`, `MaxCapacity`, `MaxRetries`, `Name`, `Outputs`, `ProjectName`, `RecipeReference`, ... (+3) | - |
| `CreateRecipeJobResponse` | `structure` | `Name` | - |
| `CreateRulesetRequest` | `structure` | `Description`, `Name`, `Rules`, `Tags`, `TargetArn` | - |
| `CreateRulesetResponse` | `structure` | `Name` | - |
| `CreateScheduleRequest` | `structure` | `CronExpression`, `JobNames`, `Name`, `Tags` | - |
| `CreateScheduleResponse` | `structure` | `Name` | - |
| `DeleteDatasetRequest` | `structure` | `Name` | - |
| `DeleteDatasetResponse` | `structure` | `Name` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

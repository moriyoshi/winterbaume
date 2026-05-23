# AWS Data Pipeline

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Data Pipeline configures and manages a data-driven workflow called a pipeline. AWS Data Pipeline handles the details of scheduling and ensuring that data dependencies are met so that your application can focus on processing the data. AWS Data Pipeline provides a JAR implementation of a task runner called AWS Data Pipeline Task Runner. AWS Data Pipeline Task Runner provides logic for common data management scenarios, such as performing database queries and running data analysis using Amazon Elastic MapReduce (Amazon EMR). You can use AWS Data Pipeline Task Runner as your task runner, or you can write your own task runner to provide custom data management.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Data Pipeline resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Data Pipeline workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Describe`, `Report`, `Set`, `Activate`, `Add` operation families, including `DescribeObjects`, `DescribePipelines`, `ReportTaskProgress`, `ReportTaskRunnerHeartbeat`, `SetStatus`, `SetTaskStatus`.

## Service Identity and Protocol

- AWS model slug: `data-pipeline`
- AWS SDK for Rust slug: `datapipeline`
- Model version: `2012-10-29`
- Model file: `vendor/api-models-aws/models/data-pipeline/service/2012-10-29/data-pipeline-2012-10-29.json`
- SDK ID: `Data Pipeline`
- Endpoint prefix: `datapipeline`
- ARN namespace: `datapipeline`
- CloudFormation name: `DataPipeline`
- CloudTrail event source: `datapipeline.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (2), `Report` (2), `Set` (2), `Activate` (1), `Add` (1), `Create` (1), `Deactivate` (1), `Delete` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTags`, `CreatePipeline`, `DeletePipeline`, `PutPipelineDefinition`, `RemoveTags`, `SetStatus`, `SetTaskStatus`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeObjects`, `DescribePipelines`, `GetPipelineDefinition`, `ListPipelines`, `ValidatePipelineDefinition`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `PollForTask`, `ReportTaskProgress`, `ReportTaskRunnerHeartbeat`, `SetTaskStatus`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`.

## Operation Groups

### Describe

- Operations: `DescribeObjects`, `DescribePipelines`
- Traits: `paginated` (1)
- Common required input members in this group: `objectIds`, `pipelineId`, `pipelineIds`

### Report

- Operations: `ReportTaskProgress`, `ReportTaskRunnerHeartbeat`
- Common required input members in this group: `taskId`, `taskrunnerId`

### Set

- Operations: `SetStatus`, `SetTaskStatus`
- Common required input members in this group: `objectIds`, `pipelineId`, `status`, `taskId`, `taskStatus`

### Activate

- Operations: `ActivatePipeline`
- Common required input members in this group: `pipelineId`

### Add

- Operations: `AddTags`
- Common required input members in this group: `pipelineId`, `tags`

### Create

- Operations: `CreatePipeline`
- Common required input members in this group: `name`, `uniqueId`

### Deactivate

- Operations: `DeactivatePipeline`
- Common required input members in this group: `pipelineId`

### Delete

- Operations: `DeletePipeline`
- Common required input members in this group: `pipelineId`

### Evaluate

- Operations: `EvaluateExpression`
- Common required input members in this group: `expression`, `objectId`, `pipelineId`

### Get

- Operations: `GetPipelineDefinition`
- Common required input members in this group: `pipelineId`

### List

- Operations: `ListPipelines`
- Traits: `paginated` (1)

### Poll

- Operations: `PollForTask`
- Common required input members in this group: `workerGroup`

### Put

- Operations: `PutPipelineDefinition`
- Common required input members in this group: `pipelineId`, `pipelineObjects`

### Query

- Operations: `QueryObjects`
- Traits: `paginated` (1)
- Common required input members in this group: `pipelineId`, `sphere`

### Remove

- Operations: `RemoveTags`
- Common required input members in this group: `pipelineId`, `tagKeys`

### Validate

- Operations: `ValidatePipelineDefinition`
- Common required input members in this group: `pipelineId`, `pipelineObjects`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivatePipeline` | - | - | `pipelineId` | - | `ActivatePipelineOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Validates the specified pipeline and starts processing pipeline tasks. If the pipeline does not pass validation, activation fails. |
| `AddTags` | - | - | `pipelineId`, `tags` | - | `AddTagsOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Adds or modifies tags for the specified pipeline. |
| `CreatePipeline` | - | - | `name`, `uniqueId` | - | `CreatePipelineOutput` | `InternalServiceError`, `InvalidRequestException` | Creates a new, empty pipeline. Use PutPipelineDefinition to populate the pipeline. |
| `DeactivatePipeline` | - | - | `pipelineId` | - | `DeactivatePipelineOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Deactivates the specified running pipeline. The pipeline is set to the `DEACTIVATING` state until the deactivation process completes. |
| `DeletePipeline` | - | - | `pipelineId` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `PipelineNotFoundException` | Deletes a pipeline, its pipeline definition, and its run history. AWS Data Pipeline attempts to cancel instances associated with the pipeline that are currently being processed by task runners. |
| `DescribeObjects` | - | `paginated` | `objectIds`, `pipelineId` | - | `DescribeObjectsOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Gets the object definitions for a set of objects associated with the pipeline. Object definitions are composed of a set of fields that define the properties of the object. |
| `DescribePipelines` | - | - | `pipelineIds` | - | `DescribePipelinesOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Retrieves metadata about one or more pipelines. The information retrieved includes the name of the pipeline, the pipeline identifier, its current state, and the user account that owns the pipeline. |
| `EvaluateExpression` | - | - | `expression`, `objectId`, `pipelineId` | - | `EvaluateExpressionOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException`, `TaskNotFoundException` | Task runners call `EvaluateExpression` to evaluate a string in the context of the specified object. For example, a task runner can evaluate SQL queries stored in Amazon S3. |
| `GetPipelineDefinition` | - | - | `pipelineId` | - | `GetPipelineDefinitionOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Gets the definition of the specified pipeline. You can call `GetPipelineDefinition` to retrieve the pipeline definition that you provided using PutPipelineDefinition. |
| `ListPipelines` | - | `paginated` | - | - | `ListPipelinesOutput` | `InternalServiceError`, `InvalidRequestException` | Lists the pipeline identifiers for all active pipelines that you have permission to access. POST / HTTP/1.1 Content-Type: application/x-amz-json-1.1 X-Amz-Target: DataPipeline.ListPipelines Content-Length: 14 Host: datapipeline.us-east-1.amazonaws.com... |
| `PollForTask` | - | - | `workerGroup` | - | `PollForTaskOutput` | `InternalServiceError`, `InvalidRequestException`, `TaskNotFoundException` | Task runners call `PollForTask` to receive a task to perform from AWS Data Pipeline. The task runner specifies which tasks it can perform by setting a value for the `workerGroup` parameter. |
| `PutPipelineDefinition` | - | - | `pipelineId`, `pipelineObjects` | - | `PutPipelineDefinitionOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Adds tasks, schedules, and preconditions to the specified pipeline. You can use `PutPipelineDefinition` to populate a new pipeline. |
| `QueryObjects` | - | `paginated` | `pipelineId`, `sphere` | - | `QueryObjectsOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Queries the specified pipeline for the names of objects that match the specified set of conditions. POST / HTTP/1.1 Content-Type: application/x-amz-json-1.1 X-Amz-Target: DataPipeline.QueryObjects Content-Length: 123 Host: datapipeline.us-east-1.amazonaws.com... |
| `RemoveTags` | - | - | `pipelineId`, `tagKeys` | - | `RemoveTagsOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Removes existing tags from the specified pipeline. |
| `ReportTaskProgress` | - | - | `taskId` | - | `ReportTaskProgressOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException`, `TaskNotFoundException` | Task runners call `ReportTaskProgress` when assigned a task to acknowledge that it has the task. If the web service does not receive this acknowledgement within 2 minutes, it assigns the task in a subsequent PollForTask call. |
| `ReportTaskRunnerHeartbeat` | - | - | `taskrunnerId` | - | `ReportTaskRunnerHeartbeatOutput` | `InternalServiceError`, `InvalidRequestException` | Task runners call `ReportTaskRunnerHeartbeat` every 15 minutes to indicate that they are operational. If the AWS Data Pipeline Task Runner is launched on a resource managed by AWS Data Pipeline, the web service can use this call to detect when the task runner... |
| `SetStatus` | - | - | `objectIds`, `pipelineId`, `status` | - | `Unit` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Requests that the status of the specified physical or logical pipeline objects be updated in the specified pipeline. This update might not occur immediately, but is eventually consistent. |
| `SetTaskStatus` | - | - | `taskId`, `taskStatus` | - | `SetTaskStatusOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException`, `TaskNotFoundException` | Task runners call `SetTaskStatus` to notify AWS Data Pipeline that a task is completed and provide information about the final status. A task runner makes this call regardless of whether the task was sucessful. |
| `ValidatePipelineDefinition` | - | - | `pipelineId`, `pipelineObjects` | - | `ValidatePipelineDefinitionOutput` | `InternalServiceError`, `InvalidRequestException`, `PipelineDeletedException`, `PipelineNotFoundException` | Validates the specified pipeline definition to ensure that it is well formed and can be run without error. Example 1 This example sets an valid pipeline configuration and returns success. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceError` | `structure` | `message` | An internal service error occurred. |
| `InvalidRequestException` | `structure` | `message` | The request was not valid. |
| `PipelineNotFoundException` | `structure` | `message` | The specified pipeline was not found. |
| `PipelineDeletedException` | `structure` | `message` | The specified pipeline has been deleted. |
| `TaskNotFoundException` | `structure` | `message` | The specified task was not found. |
| `ActivatePipelineInput` | `structure` | `parameterValues`, `pipelineId`, `startTimestamp` | Contains the parameters for ActivatePipeline. |
| `ActivatePipelineOutput` | `structure` | - | Contains the output of ActivatePipeline. |
| `AddTagsInput` | `structure` | `pipelineId`, `tags` | Contains the parameters for AddTags. |
| `AddTagsOutput` | `structure` | - | Contains the output of AddTags. |
| `CreatePipelineInput` | `structure` | `description`, `name`, `tags`, `uniqueId` | Contains the parameters for CreatePipeline. |
| `CreatePipelineOutput` | `structure` | `pipelineId` | Contains the output of CreatePipeline. |
| `DeactivatePipelineInput` | `structure` | `cancelActive`, `pipelineId` | Contains the parameters for DeactivatePipeline. |
| `DeactivatePipelineOutput` | `structure` | - | Contains the output of DeactivatePipeline. |
| `DeletePipelineInput` | `structure` | `pipelineId` | Contains the parameters for DeletePipeline. |
| `DescribeObjectsInput` | `structure` | `evaluateExpressions`, `marker`, `objectIds`, `pipelineId` | Contains the parameters for DescribeObjects. |
| `DescribeObjectsOutput` | `structure` | `hasMoreResults`, `marker`, `pipelineObjects` | Contains the output of DescribeObjects. |
| `DescribePipelinesInput` | `structure` | `pipelineIds` | Contains the parameters for DescribePipelines. |
| `DescribePipelinesOutput` | `structure` | `pipelineDescriptionList` | Contains the output of DescribePipelines. |
| `EvaluateExpressionInput` | `structure` | `expression`, `objectId`, `pipelineId` | Contains the parameters for EvaluateExpression. |
| `EvaluateExpressionOutput` | `structure` | `evaluatedExpression` | Contains the output of EvaluateExpression. |
| `GetPipelineDefinitionInput` | `structure` | `pipelineId`, `version` | Contains the parameters for GetPipelineDefinition. |
| `GetPipelineDefinitionOutput` | `structure` | `parameterObjects`, `parameterValues`, `pipelineObjects` | Contains the output of GetPipelineDefinition. |
| `ListPipelinesInput` | `structure` | `marker` | Contains the parameters for ListPipelines. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

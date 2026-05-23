# AmazonMWAAServerless

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Workflows for Apache Airflow Serverless provides a managed workflow orchestration platform for running Apache Airflow workflows in a serverless environment. You can use Amazon Managed Workflows for Apache Airflow Serverless to create, manage, and run data processing workflows without managing the underlying infrastructure, Airflow clusters, metadata databases, or scheduling overhead. The service provides secure multi-tenant run environments with automatic scaling, comprehensive logging, and integration with multiple Amazon Web Services services for orchestrating complex analytics workloads.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AmazonMWAAServerless resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AmazonMWAAServerless workflows in the local mock. Key resources include `TaskInstanceResource`, `WorkflowResource`, `WorkflowRunResource`, `WorkflowVersionResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Start` operation families, including `ListTagsForResource`, `ListTaskInstances`, `ListWorkflowRuns`, `ListWorkflowVersions`, `GetTaskInstance`, `GetWorkflow`.

## Service Identity and Protocol

- AWS model slug: `mwaa-serverless`
- AWS SDK for Rust slug: `mwaaserverless`
- Model version: `2024-07-26`
- Model file: `vendor/api-models-aws/models/mwaa-serverless/service/2024-07-26/mwaa-serverless-2024-07-26.json`
- SDK ID: `MWAA Serverless`
- Endpoint prefix: `airflow-serverless`
- ARN namespace: `airflow-serverless`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Get` (3), `Create` (1), `Delete` (1), `Start` (1), `Stop` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateWorkflow`, `DeleteWorkflow`, `StartWorkflowRun`, `StopWorkflowRun`, `TagResource`, `UntagResource`, `UpdateWorkflow`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetTaskInstance`, `GetWorkflow`, `GetWorkflowRun`, `ListTagsForResource`, `ListTaskInstances`, `ListWorkflowRuns`, `ListWorkflowVersions`, `ListWorkflows`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetTaskInstance`, `ListTaskInstances`, `StartWorkflowRun`, `StopWorkflowRun`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 15 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `TaskInstanceResource` | `RunId`, `TaskInstanceId`, `WorkflowArn` | read: `GetTaskInstance`; list: `ListTaskInstances` | - | - |
| `WorkflowResource` | `WorkflowArn` | create: `CreateWorkflow`; read: `GetWorkflow`; update: `UpdateWorkflow`; delete: `DeleteWorkflow`; list: `ListWorkflows` | - | - |
| `WorkflowRunResource` | `RunId` | create: `StartWorkflowRun`; read: `GetWorkflowRun`; update: `StopWorkflowRun`; list: `ListWorkflowRuns` | - | - |
| `WorkflowVersionResource` | `WorkflowVersionArn` | list: `ListWorkflowVersions` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`, `ListTaskInstances`, `ListWorkflowRuns`, `ListWorkflowVersions`, `ListWorkflows`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `ResourceArn`, `RunId`, `WorkflowArn`

### Get

- Operations: `GetTaskInstance`, `GetWorkflow`, `GetWorkflowRun`
- Traits: `readonly` (3)
- Common required input members in this group: `RunId`, `TaskInstanceId`, `WorkflowArn`

### Create

- Operations: `CreateWorkflow`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `DefinitionS3Location`, `Name`, `RoleArn`

### Delete

- Operations: `DeleteWorkflow`
- Traits: `idempotent` (1)
- Common required input members in this group: `WorkflowArn`

### Start

- Operations: `StartWorkflowRun`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `WorkflowArn`

### Stop

- Operations: `StopWorkflowRun`
- Traits: `idempotent` (1)
- Common required input members in this group: `RunId`, `WorkflowArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateWorkflow`
- Common required input members in this group: `DefinitionS3Location`, `RoleArn`, `WorkflowArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateWorkflow` | `POST /workflows` | `idempotent`, `idempotency-token` | `DefinitionS3Location`, `Name`, `RoleArn` | `ClientToken` | `CreateWorkflowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `OperationTimeoutException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new workflow in Amazon Managed Workflows for Apache Airflow Serverless. This operation initializes a workflow with the specified configuration including the workflow definition, execution role, and optional settings for encryption, logging, and... |
| `DeleteWorkflow` | `DELETE /workflows/{WorkflowArn}` | `idempotent` | `WorkflowArn` | - | `DeleteWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a workflow and all its versions. This operation permanently removes the workflow and cannot be undone. |
| `GetTaskInstance` | `GET /workflows/{WorkflowArn}/runs/{RunId}/tasks/{TaskInstanceId}` | `readonly` | `RunId`, `TaskInstanceId`, `WorkflowArn` | - | `GetTaskInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific task instance within a workflow run. Task instances represent individual tasks that are executed as part of a workflow in the Amazon Managed Workflows for Apache Airflow Serverless environment. |
| `GetWorkflow` | `GET /workflows/{WorkflowArn}` | `readonly` | `WorkflowArn` | - | `GetWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a workflow, including its configuration, status, and metadata. |
| `GetWorkflowRun` | `GET /workflows/{WorkflowArn}/runs/{RunId}` | `readonly` | `RunId`, `WorkflowArn` | - | `GetWorkflowRunResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific workflow run, including its status, execution details, and task instances. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags that are associated with a specified Amazon Managed Workflows for Apache Airflow Serverless resource. |
| `ListTaskInstances` | `GET /workflows/{WorkflowArn}/runs/{RunId}/tasks` | `readonly`, `paginated` | `RunId`, `WorkflowArn` | - | `ListTaskInstancesResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ThrottlingException`, `ValidationException` | Lists all task instances for a specific workflow run, with optional pagination support. |
| `ListWorkflowRuns` | `GET /workflows/{WorkflowArn}/runs` | `readonly`, `paginated` | `WorkflowArn` | - | `ListWorkflowRunsResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ThrottlingException`, `ValidationException` | Lists all runs for a specified workflow, with optional pagination and filtering support. |
| `ListWorkflowVersions` | `GET /workflows/{WorkflowArn}/versions` | `readonly`, `paginated` | `WorkflowArn` | - | `ListWorkflowVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ThrottlingException`, `ValidationException` | Lists all versions of a specified workflow, with optional pagination support. |
| `ListWorkflows` | `GET /workflows` | `readonly`, `paginated` | - | - | `ListWorkflowsResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ThrottlingException`, `ValidationException` | Lists all workflows in your account, with optional pagination support. This operation returns summary information for workflows, showing only the most recently created version of each workflow. |
| `StartWorkflowRun` | `POST /workflows/{WorkflowArn}/runs` | `idempotent`, `idempotency-token` | `WorkflowArn` | `ClientToken` | `StartWorkflowRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a new execution of a workflow. This operation creates a workflow run that executes the tasks that are defined in the workflow. |
| `StopWorkflowRun` | `DELETE /workflows/{WorkflowArn}/runs/{RunId}` | `idempotent` | `RunId`, `WorkflowArn` | - | `StopWorkflowRunResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a running workflow execution. This operation terminates all running tasks and prevents new tasks from starting. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to an Amazon Managed Workflows for Apache Airflow Serverless resource. Tags are key-value pairs that help you organize and categorize your resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from an Amazon Managed Workflows for Apache Airflow Serverless resource. This operation removes the specified tags from the resource. |
| `UpdateWorkflow` | `PUT /workflows/{WorkflowArn}` | - | `DefinitionS3Location`, `RoleArn`, `WorkflowArn` | - | `UpdateWorkflowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing workflow with new configuration settings. This operation allows you to modify the workflow definition, role, and other settings. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient permission to perform this action. |
| `InternalServerException` | `structure` | `Message`, `RetryAfterSeconds` | An unexpected server-side error occurred during request processing. |
| `OperationTimeoutException` | `structure` | `Message` | The operation timed out. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `RetryAfterSeconds`, `ServiceCode` | The request was denied because too many requests were made in a short period, exceeding the service rate limits. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | The specified request parameters are invalid, missing, or inconsistent with Amazon Managed Workflows for Apache Airflow Serverless service requirements. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The specified resource was not found. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | You cannot create a resource that already exists, or the resource is in a state that prevents the requested operation. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | The request exceeds the service quota for Amazon Managed Workflows for Apache Airflow Serverless resources. |
| `CreateWorkflowRequest` | `structure` | `ClientToken`, `DefinitionS3Location`, `Description`, `EncryptionConfiguration`, `EngineVersion`, `LoggingConfiguration`, `Name`, `NetworkConfiguration`, `RoleArn`, `Tags`, `TriggerMode` | - |
| `CreateWorkflowResponse` | `structure` | `CreatedAt`, `IsLatestVersion`, `RevisionId`, `Warnings`, `WorkflowArn`, `WorkflowStatus`, `WorkflowVersion` | - |
| `DeleteWorkflowRequest` | `structure` | `WorkflowArn`, `WorkflowVersion` | - |
| `DeleteWorkflowResponse` | `structure` | `WorkflowArn`, `WorkflowVersion` | - |
| `GetTaskInstanceRequest` | `structure` | `RunId`, `TaskInstanceId`, `WorkflowArn` | - |
| `GetTaskInstanceResponse` | `structure` | `AttemptNumber`, `DurationInSeconds`, `EndedAt`, `ErrorMessage`, `LogStream`, `ModifiedAt`, `OperatorName`, `RunId`, `StartedAt`, `Status`, `TaskId`, `TaskInstanceId`, ... (+3) | - |
| `GetWorkflowRequest` | `structure` | `WorkflowArn`, `WorkflowVersion` | - |
| `GetWorkflowResponse` | `structure` | `CreatedAt`, `DefinitionS3Location`, `Description`, `EncryptionConfiguration`, `EngineVersion`, `LoggingConfiguration`, `ModifiedAt`, `Name`, `NetworkConfiguration`, `RoleArn`, `ScheduleConfiguration`, `TriggerMode`, ... (+4) | - |
| `GetWorkflowRunRequest` | `structure` | `RunId`, `WorkflowArn` | - |
| `GetWorkflowRunResponse` | `structure` | `OverrideParameters`, `RunDetail`, `RunId`, `RunType`, `WorkflowArn`, `WorkflowVersion` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `Tags` | - |
| `ListTaskInstancesRequest` | `structure` | `MaxResults`, `NextToken`, `RunId`, `WorkflowArn` | - |
| `ListTaskInstancesResponse` | `structure` | `NextToken`, `TaskInstances` | - |
| `ListWorkflowRunsRequest` | `structure` | `MaxResults`, `NextToken`, `WorkflowArn`, `WorkflowVersion` | - |
| `ListWorkflowRunsResponse` | `structure` | `NextToken`, `WorkflowRuns` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

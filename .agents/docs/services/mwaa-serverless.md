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

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags that are associated with a specified Amazon Managed Workflows for Apache Airflow Serverless resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to an Amazon Managed Workflows for Apache Airflow Serverless resource. Tags are key-value pairs that help you organize and categorize your resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `OperationTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from an Amazon Managed Workflows for Apache Airflow Serverless resource. This operation removes the specified tags from the resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient permission to perform this action. |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType | You cannot create a resource that already exists, or the resource is in a state that prevents the requested operation. |
| `InternalServerException` | `structure` | Message, RetryAfterSeconds | An unexpected server-side error occurred during request processing. |
| `OperationTimeoutException` | `structure` | Message | The operation timed out. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The specified resource was not found. You can only access or modify a resource that already exists. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, ServiceCode, QuotaCode | The request exceeds the service quota for Amazon Managed Workflows for Apache Airflow Serverless resources. This can occur when you attempt to create more w ... |
| `ThrottlingException` | `structure` | Message, ServiceCode, QuotaCode, RetryAfterSeconds | The request was denied because too many requests were made in a short period, exceeding the service rate limits. Amazon Managed Workflows for Apache Airflow ... |
| `ValidationException` | `structure` | Message, Reason, FieldList | The specified request parameters are invalid, missing, or inconsistent with Amazon Managed Workflows for Apache Airflow Serverless service requirements. Thi ... |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `EncryptionType` | `enum` | AWS_MANAGED_KEY, CUSTOMER_MANAGED_KEY | - |
| `EngineVersion` | `intEnum` | ONE | - |
| `RunType` | `enum` | ON_DEMAND, SCHEDULED | - |
| `TaskInstanceStatus` | `enum` | QUEUED, FAILED, SCHEDULED, RUNNING, SUCCESS, UP_FOR_RESCHEDULE, UP_FOR_RETRY, UPSTREAM_FAILED, REMOVED, RESTARTING, DEFERRED, NONE, ... (+2) | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
| `WorkflowRunStatus` | `enum` | STARTING, QUEUED, RUNNING, SUCCESS, FAILED, TIMEOUT, STOPPING, STOPPED | - |
| `WorkflowStatus` | `enum` | READY, DELETING | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

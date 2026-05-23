# Nova Act Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Nova Act service provides a REST API for managing AI-powered workflow automation. It enables users to create workflow definitions, execute workflow runs, manage sessions, and orchestrate acts (individual AI tasks) with tool integrations.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Nova Act Service workflows in the local mock. Key resources include `ActResource`, `ModelResource`, `ServiceLinkedRoleResource`, `SessionResource`, `WorkflowDefinitionResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Update` operation families, including `ListActs`, `ListModels`, `ListSessions`, `ListWorkflowDefinitions`, `CreateAct`, `CreateSession`.

## Service Identity and Protocol

- AWS model slug: `nova-act`
- AWS SDK for Rust slug: `novaact`
- Model version: `2025-08-22`
- Model file: `vendor/api-models-aws/models/nova-act/service/2025-08-22/nova-act-2025-08-22.json`
- SDK ID: `Nova Act`
- Endpoint prefix: `nova-act`
- ARN namespace: `nova-act`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Create` (4), `Delete` (2), `Get` (2), `Update` (2), `Invoke` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAct`, `CreateSession`, `CreateWorkflowDefinition`, `CreateWorkflowRun`, `DeleteWorkflowDefinition`, `DeleteWorkflowRun`, `UpdateAct`, `UpdateWorkflowRun`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetWorkflowDefinition`, `GetWorkflowRun`, `ListActs`, `ListModels`, `ListSessions`, `ListWorkflowDefinitions`, `ListWorkflowRuns`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`, `ECS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ActResource` | `actId`, `sessionId`, `workflowDefinitionName`, `workflowRunId` | create: `CreateAct`; list: `ListActs` | `InvokeActStep`, `UpdateAct` | - |
| `ModelResource` | `modelId` | list: `ListModels` | - | - |
| `ServiceLinkedRoleResource` | - | - | - | - |
| `SessionResource` | `sessionId`, `workflowDefinitionName`, `workflowRunId` | create: `CreateSession`; list: `ListSessions` | - | - |
| `WorkflowDefinitionResource` | `workflowDefinitionName` | create: `CreateWorkflowDefinition`; read: `GetWorkflowDefinition`; delete: `DeleteWorkflowDefinition`; list: `ListWorkflowDefinitions` | - | - |
| `WorkflowRunResource` | `workflowDefinitionName`, `workflowRunId` | create: `CreateWorkflowRun`; read: `GetWorkflowRun`; update: `UpdateWorkflowRun`; delete: `DeleteWorkflowRun`; list: `ListWorkflowRuns` | - | - |
## Operation Groups

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permissions to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request could not be completed due to a conflict with the current state of the resource. |
| `InternalServerException` | `structure` | message, retryAfterSeconds, reason | An internal server error occurred. Please try again later. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The requested resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request would exceed a service quota limit. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was throttled due to too many requests. Please try again later. |
| `ValidationException` | `structure` | message, reason, fieldList | The input parameters for the request are invalid. |
| `ActStatus` | `enum` | RUNNING, PENDING_CLIENT_ACTION, PENDING_HUMAN_ACTION, SUCCEEDED, FAILED, TIMED_OUT | - |
| `InternalServerExceptionReason` | `enum` | INVALID_MODEL_GENERATION, TOKEN_LIMIT_EXCEEDED | - |
| `ModelStatus` | `enum` | ACTIVE, LEGACY, DEPRECATED, PREVIEW | - |
| `SortOrder` | `enum` | ASC, DESC | - |
| `TraceLocationType` | `enum` | S3 | - |
| `ValidationExceptionReason` | `enum` | FIELD_VALIDATION_FAILED, INVALID_STATUS, GUARDRAIL_INTERVENED | - |
| `WorkflowDefinitionStatus` | `enum` | ACTIVE, DELETING | - |
| `WorkflowRunStatus` | `enum` | RUNNING, SUCCEEDED, FAILED, TIMED_OUT, DELETING | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

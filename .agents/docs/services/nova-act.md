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

### List

- Operations: `ListActs`, `ListModels`, `ListSessions`, `ListWorkflowDefinitions`, `ListWorkflowRuns`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `clientCompatibilityVersion`, `workflowDefinitionName`, `workflowRunId`

### Create

- Operations: `CreateAct`, `CreateSession`, `CreateWorkflowDefinition`, `CreateWorkflowRun`
- Traits: `idempotency-token` (4), `idempotent` (4)
- Common required input members in this group: `clientInfo`, `modelId`, `name`, `sessionId`, `task`, `workflowDefinitionName`, `workflowRunId`

### Delete

- Operations: `DeleteWorkflowDefinition`, `DeleteWorkflowRun`
- Traits: `idempotent` (2)
- Common required input members in this group: `workflowDefinitionName`, `workflowRunId`

### Get

- Operations: `GetWorkflowDefinition`, `GetWorkflowRun`
- Traits: `readonly` (2)
- Common required input members in this group: `workflowDefinitionName`, `workflowRunId`

### Update

- Operations: `UpdateAct`, `UpdateWorkflowRun`
- Traits: `idempotent` (2)
- Common required input members in this group: `actId`, `sessionId`, `status`, `workflowDefinitionName`, `workflowRunId`

### Invoke

- Operations: `InvokeActStep`
- Common required input members in this group: `actId`, `callResults`, `sessionId`, `workflowDefinitionName`, `workflowRunId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAct` | `PUT /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}/sessions/{sessionId}/acts` | `idempotent`, `idempotency-token` | `sessionId`, `task`, `workflowDefinitionName`, `workflowRunId` | `clientToken` | `CreateActResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new AI task (act) within a session that can interact with tools and perform specific actions. |
| `CreateSession` | `PUT /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}/sessions` | `idempotent`, `idempotency-token` | `workflowDefinitionName`, `workflowRunId` | `clientToken` | `CreateSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new session context within a workflow run to manage conversation state and acts. |
| `CreateWorkflowDefinition` | `PUT /workflow-definitions` | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateWorkflowDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new workflow definition template that can be used to execute multiple workflow runs. |
| `CreateWorkflowRun` | `PUT /workflow-definitions/{workflowDefinitionName}/workflow-runs` | `idempotent`, `idempotency-token` | `clientInfo`, `modelId`, `workflowDefinitionName` | `clientToken` | `CreateWorkflowRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new execution instance of a workflow definition with specified parameters. |
| `DeleteWorkflowDefinition` | `DELETE /workflow-definitions/{workflowDefinitionName}` | `idempotent` | `workflowDefinitionName` | - | `DeleteWorkflowDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a workflow definition and all associated resources. This operation cannot be undone. |
| `DeleteWorkflowRun` | `DELETE /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}` | `idempotent` | `workflowDefinitionName`, `workflowRunId` | - | `DeleteWorkflowRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Terminates and cleans up a workflow run, stopping all associated acts and sessions. |
| `GetWorkflowDefinition` | `GET /workflow-definitions/{workflowDefinitionName}` | `readonly` | `workflowDefinitionName` | - | `GetWorkflowDefinitionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details and configuration of a specific workflow definition. |
| `GetWorkflowRun` | `GET /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}` | `readonly` | `workflowDefinitionName`, `workflowRunId` | - | `GetWorkflowRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the current state, configuration, and execution details of a workflow run. |
| `InvokeActStep` | `PUT /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}/sessions/{sessionId}/acts/{actId}/invoke-step/` | - | `actId`, `callResults`, `sessionId`, `workflowDefinitionName`, `workflowRunId` | - | `InvokeActStepResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Executes the next step of an act, processing tool call results and returning new tool calls if needed. |
| `ListActs` | `POST /workflow-definitions/{workflowDefinitionName}/acts` | `readonly`, `paginated` | `workflowDefinitionName` | - | `ListActsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all acts within a specific session with their current status and execution details. |
| `ListModels` | `POST /models` | `readonly` | `clientCompatibilityVersion` | - | `ListModelsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | Lists all available AI models that can be used for workflow execution, including their status and compatibility information. |
| `ListSessions` | `POST /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}` | `readonly`, `paginated` | `workflowDefinitionName`, `workflowRunId` | - | `ListSessionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all sessions within a specific workflow run. |
| `ListWorkflowDefinitions` | `POST /workflow-definitions` | `readonly`, `paginated` | - | - | `ListWorkflowDefinitionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all workflow definitions in your account with optional filtering and pagination. |
| `ListWorkflowRuns` | `POST /workflow-definitions/{workflowDefinitionName}/workflow-runs` | `readonly`, `paginated` | `workflowDefinitionName` | - | `ListWorkflowRunsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all workflow runs for a specific workflow definition with optional filtering and pagination. |
| `UpdateAct` | `PUT /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}/sessions/{sessionId}/acts/{actId}` | `idempotent` | `actId`, `sessionId`, `status`, `workflowDefinitionName`, `workflowRunId` | - | `UpdateActResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing act's configuration, status, or error information. |
| `UpdateWorkflowRun` | `PUT /workflow-definitions/{workflowDefinitionName}/workflow-runs/{workflowRunId}` | `idempotent` | `status`, `workflowDefinitionName`, `workflowRunId` | - | `UpdateWorkflowRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration or state of an active workflow run. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permissions to perform this action. |
| `InternalServerException` | `structure` | `message`, `reason`, `retryAfterSeconds` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was throttled due to too many requests. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input parameters for the request are invalid. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request could not be completed due to a conflict with the current state of the resource. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The requested resource was not found. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request would exceed a service quota limit. |
| `CreateActRequest` | `structure` | `clientToken`, `sessionId`, `task`, `toolSpecs`, `workflowDefinitionName`, `workflowRunId` | - |
| `CreateActResponse` | `structure` | `actId`, `status` | - |
| `CreateSessionRequest` | `structure` | `clientToken`, `workflowDefinitionName`, `workflowRunId` | - |
| `CreateSessionResponse` | `structure` | `sessionId` | - |
| `CreateWorkflowDefinitionRequest` | `structure` | `clientToken`, `description`, `exportConfig`, `name` | - |
| `CreateWorkflowDefinitionResponse` | `structure` | `status` | - |
| `CreateWorkflowRunRequest` | `structure` | `clientInfo`, `clientToken`, `logGroupName`, `modelId`, `workflowDefinitionName` | - |
| `CreateWorkflowRunResponse` | `structure` | `status`, `workflowRunId` | - |
| `DeleteWorkflowDefinitionRequest` | `structure` | `workflowDefinitionName` | - |
| `DeleteWorkflowDefinitionResponse` | `structure` | `status` | - |
| `DeleteWorkflowRunRequest` | `structure` | `workflowDefinitionName`, `workflowRunId` | - |
| `DeleteWorkflowRunResponse` | `structure` | `status` | - |
| `GetWorkflowDefinitionRequest` | `structure` | `workflowDefinitionName` | - |
| `GetWorkflowDefinitionResponse` | `structure` | `arn`, `createdAt`, `description`, `exportConfig`, `name`, `status` | - |
| `GetWorkflowRunRequest` | `structure` | `workflowDefinitionName`, `workflowRunId` | - |
| `GetWorkflowRunResponse` | `structure` | `endedAt`, `logGroupName`, `modelId`, `startedAt`, `status`, `workflowRunArn`, `workflowRunId` | - |
| `InvokeActStepRequest` | `structure` | `actId`, `callResults`, `previousStepId`, `sessionId`, `workflowDefinitionName`, `workflowRunId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

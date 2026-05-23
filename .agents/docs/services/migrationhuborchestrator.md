# AWS Migration Hub Orchestrator

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This API reference provides descriptions, syntax, and other details about each of the actions and data types for AWS Migration Hub Orchestrator. The topic for each action shows the API request parameters and responses. Alternatively, you can use one of the AWS SDKs to access an API that is tailored to the programming language or platform that you're using.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Migration Hub Orchestrator workflows in the local mock. Key resources include `MigrationWorkflow`, `MigrationWorkflowTemplate`, `Plugin`, `TemplateStep`, `TemplateStepGroup`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListPlugins`, `ListTagsForResource`, `ListTemplateStepGroups`, `ListTemplateSteps`, `GetTemplate`, `GetTemplateStep`.

## Service Identity and Protocol

- AWS model slug: `migrationhuborchestrator`
- AWS SDK for Rust slug: `migrationhuborchestrator`
- Model version: `2021-08-28`
- Model file: `vendor/api-models-aws/models/migrationhuborchestrator/service/2021-08-28/migrationhuborchestrator-2021-08-28.json`
- SDK ID: `MigrationHubOrchestrator`
- Endpoint prefix: `migrationhub-orchestrator`
- ARN namespace: `migrationhub-orchestrator`
- CloudFormation name: `-`
- CloudTrail event source: `migrationhub-orchestrator.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (6), `Create` (4), `Delete` (4), `Update` (4), `Retry` (1), `Start` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateTemplate`, `CreateWorkflow`, `CreateWorkflowStep`, `CreateWorkflowStepGroup`, `DeleteTemplate`, `DeleteWorkflow`, `DeleteWorkflowStep`, `DeleteWorkflowStepGroup`, `StartWorkflow`, `StopWorkflow`, `TagResource`, `UntagResource`, `UpdateTemplate`, `UpdateWorkflow`, `UpdateWorkflowStep`, `UpdateWorkflowStepGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetTemplate`, `GetTemplateStep`, `GetTemplateStepGroup`, `GetWorkflow`, `GetWorkflowStep`, `GetWorkflowStepGroup`, `ListPlugins`, `ListTagsForResource`, `ListTemplateStepGroups`, `ListTemplateSteps`, `ListTemplates`, `ListWorkflowStepGroups`, `ListWorkflowSteps`, `ListWorkflows`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartWorkflow`, `StopWorkflow`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `SNS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `MigrationWorkflow` | `id` | create: `CreateWorkflow`; read: `GetWorkflow`; update: `UpdateWorkflow`; delete: `DeleteWorkflow`; list: `ListWorkflows` | `StartWorkflow`, `StopWorkflow` | - |
| `MigrationWorkflowTemplate` | `id` | create: `CreateTemplate`; read: `GetTemplate`; update: `UpdateTemplate`; delete: `DeleteTemplate`; list: `ListTemplates` | - | - |
| `Plugin` | `pluginId` | list: `ListPlugins` | - | - |
| `TemplateStep` | `id` | read: `GetTemplateStep`; list: `ListTemplateSteps` | - | - |
| `TemplateStepGroup` | `id`, `templateId` | read: `GetTemplateStepGroup` | - | - |
| `TemplateStepGroups` | `templateId` | read: `ListTemplateStepGroups` | - | - |
| `WorkflowStep` | `id` | create: `CreateWorkflowStep`; read: `GetWorkflowStep`; update: `UpdateWorkflowStep`; delete: `DeleteWorkflowStep`; list: `ListWorkflowSteps` | `RetryWorkflowStep` | - |
| `WorkflowStepGroup` | `id` | create: `CreateWorkflowStepGroup`; read: `GetWorkflowStepGroup`; update: `UpdateWorkflowStepGroup`; delete: `DeleteWorkflowStepGroup`; list: `ListWorkflowStepGroups` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | List the tags added to a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Tag a resource by specifying its Amazon Resource Name (ARN). |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes the tags for a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | This exception is thrown when an attempt to update or delete a resource would cause an inconsistent state. |
| `InternalServerException` | `structure` | message | An internal error has occurred. |
| `ResourceNotFoundException` | `structure` | message | The resource is not available. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an AWS service. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

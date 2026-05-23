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

- Operations: `ListPlugins`, `ListTagsForResource`, `ListTemplateStepGroups`, `ListTemplateSteps`, `ListTemplates`, `ListWorkflowStepGroups`, `ListWorkflowSteps`, `ListWorkflows`
- Traits: `paginated` (7), `readonly` (8)
- Common required input members in this group: `resourceArn`, `stepGroupId`, `templateId`, `workflowId`

### Get

- Operations: `GetTemplate`, `GetTemplateStep`, `GetTemplateStepGroup`, `GetWorkflow`, `GetWorkflowStep`, `GetWorkflowStepGroup`
- Traits: `readonly` (6)
- Common required input members in this group: `id`, `stepGroupId`, `templateId`, `workflowId`

### Create

- Operations: `CreateTemplate`, `CreateWorkflow`, `CreateWorkflowStep`, `CreateWorkflowStepGroup`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `inputParameters`, `name`, `stepActionType`, `stepGroupId`, `templateId`, `templateName`, `templateSource`, `workflowId`

### Delete

- Operations: `DeleteTemplate`, `DeleteWorkflow`, `DeleteWorkflowStep`, `DeleteWorkflowStepGroup`
- Traits: `idempotent` (4)
- Common required input members in this group: `id`, `stepGroupId`, `workflowId`

### Update

- Operations: `UpdateTemplate`, `UpdateWorkflow`, `UpdateWorkflowStep`, `UpdateWorkflowStepGroup`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `id`, `stepGroupId`, `workflowId`

### Retry

- Operations: `RetryWorkflowStep`
- Common required input members in this group: `id`, `stepGroupId`, `workflowId`

### Start

- Operations: `StartWorkflow`
- Common required input members in this group: `id`

### Stop

- Operations: `StopWorkflow`
- Common required input members in this group: `id`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateTemplate` | `POST /template` | `idempotency-token` | `templateName`, `templateSource` | `clientToken` | `CreateTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a migration workflow template. |
| `CreateWorkflow` | `POST /migrationworkflow/` | - | `inputParameters`, `name`, `templateId` | - | `CreateMigrationWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Create a workflow to orchestrate your migrations. |
| `CreateWorkflowStep` | `POST /workflowstep` | - | `name`, `stepActionType`, `stepGroupId`, `workflowId` | - | `CreateWorkflowStepResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Create a step in the migration workflow. |
| `CreateWorkflowStepGroup` | `POST /workflowstepgroups` | - | `name`, `workflowId` | - | `CreateWorkflowStepGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Create a step group in a migration workflow. |
| `DeleteTemplate` | `DELETE /template/{id}` | `idempotent` | `id` | - | `DeleteTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a migration workflow template. |
| `DeleteWorkflow` | `DELETE /migrationworkflow/{id}` | `idempotent` | `id` | - | `DeleteMigrationWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a migration workflow. You must pause a running workflow in Migration Hub Orchestrator console to delete it. |
| `DeleteWorkflowStep` | `DELETE /workflowstep/{id}` | `idempotent` | `id`, `stepGroupId`, `workflowId` | - | `DeleteWorkflowStepResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a step in a migration workflow. Pause the workflow to delete a running step. |
| `DeleteWorkflowStepGroup` | `DELETE /workflowstepgroup/{id}` | `idempotent` | `id`, `workflowId` | - | `DeleteWorkflowStepGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a step group in a migration workflow. |
| `GetTemplate` | `GET /migrationworkflowtemplate/{id}` | `readonly` | `id` | - | `GetMigrationWorkflowTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get the template you want to use for creating a migration workflow. |
| `GetTemplateStep` | `GET /templatestep/{id}` | `readonly` | `id`, `stepGroupId`, `templateId` | - | `GetTemplateStepResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a specific step in a template. |
| `GetTemplateStepGroup` | `GET /templates/{templateId}/stepgroups/{id}` | `readonly` | `id`, `templateId` | - | `GetTemplateStepGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a step group in a template. |
| `GetWorkflow` | `GET /migrationworkflow/{id}` | `readonly` | `id` | - | `GetMigrationWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get migration workflow. |
| `GetWorkflowStep` | `GET /workflowstep/{id}` | `readonly` | `id`, `stepGroupId`, `workflowId` | - | `GetWorkflowStepResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Get a step in the migration workflow. |
| `GetWorkflowStepGroup` | `GET /workflowstepgroup/{id}` | `readonly` | `id`, `workflowId` | - | `GetWorkflowStepGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the step group of a migration workflow. |
| `ListPlugins` | `GET /plugins` | `readonly`, `paginated` | - | - | `ListPluginsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | List AWS Migration Hub Orchestrator plugins. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | List the tags added to a resource. |
| `ListTemplateStepGroups` | `GET /templatestepgroups/{templateId}` | `readonly`, `paginated` | `templateId` | - | `ListTemplateStepGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | List the step groups in a template. |
| `ListTemplateSteps` | `GET /templatesteps` | `readonly`, `paginated` | `stepGroupId`, `templateId` | - | `ListTemplateStepsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the steps in a template. |
| `ListTemplates` | `GET /migrationworkflowtemplates` | `readonly`, `paginated` | - | - | `ListMigrationWorkflowTemplatesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | List the templates available in Migration Hub Orchestrator to create a migration workflow. |
| `ListWorkflowStepGroups` | `GET /workflowstepgroups` | `readonly`, `paginated` | `workflowId` | - | `ListWorkflowStepGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the step groups in a migration workflow. |
| `ListWorkflowSteps` | `GET /workflow/{workflowId}/workflowstepgroups/{stepGroupId}/workflowsteps` | `readonly`, `paginated` | `stepGroupId`, `workflowId` | - | `ListWorkflowStepsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List the steps in a workflow. |
| `ListWorkflows` | `GET /migrationworkflows` | `readonly`, `paginated` | - | - | `ListMigrationWorkflowsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the migration workflows. |
| `RetryWorkflowStep` | `POST /retryworkflowstep/{id}` | - | `id`, `stepGroupId`, `workflowId` | - | `RetryWorkflowStepResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retry a failed step in a migration workflow. |
| `StartWorkflow` | `POST /migrationworkflow/{id}/start` | - | `id` | - | `StartMigrationWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Start a migration workflow. |
| `StopWorkflow` | `POST /migrationworkflow/{id}/stop` | - | `id` | - | `StopMigrationWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stop an ongoing migration workflow. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Tag a resource by specifying its Amazon Resource Name (ARN). |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes the tags for a resource. |
| `UpdateTemplate` | `POST /template/{id}` | `idempotency-token` | `id` | `clientToken` | `UpdateTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a migration workflow template. |
| `UpdateWorkflow` | `POST /migrationworkflow/{id}` | - | `id` | - | `UpdateMigrationWorkflowResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a migration workflow. |
| `UpdateWorkflowStep` | `POST /workflowstep/{id}` | - | `id`, `stepGroupId`, `workflowId` | - | `UpdateWorkflowStepResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update a step in a migration workflow. |
| `UpdateWorkflowStepGroup` | `POST /workflowstepgroup/{id}` | `idempotent` | `id`, `workflowId` | - | `UpdateWorkflowStepGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the step group in a migration workflow. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | An internal error has occurred. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by an AWS service. |
| `ResourceNotFoundException` | `structure` | `message` | The resource is not available. |
| `CreateTemplateRequest` | `structure` | `clientToken`, `tags`, `templateDescription`, `templateName`, `templateSource` | - |
| `CreateTemplateResponse` | `structure` | `tags`, `templateArn`, `templateId` | - |
| `ConflictException` | `structure` | `message` | This exception is thrown when an attempt to update or delete a resource would cause an inconsistent state. |
| `CreateMigrationWorkflowRequest` | `structure` | `applicationConfigurationId`, `description`, `inputParameters`, `name`, `stepTargets`, `tags`, `templateId` | - |
| `CreateMigrationWorkflowResponse` | `structure` | `adsApplicationConfigurationId`, `arn`, `creationTime`, `description`, `id`, `name`, `status`, `stepTargets`, `tags`, `templateId`, `workflowInputs` | - |
| `CreateWorkflowStepRequest` | `structure` | `description`, `name`, `next`, `outputs`, `previous`, `stepActionType`, `stepGroupId`, `stepTarget`, `workflowId`, `workflowStepAutomationConfiguration` | - |
| `CreateWorkflowStepResponse` | `structure` | `id`, `name`, `stepGroupId`, `workflowId` | - |
| `CreateWorkflowStepGroupRequest` | `structure` | `description`, `name`, `next`, `previous`, `workflowId` | - |
| `CreateWorkflowStepGroupResponse` | `structure` | `creationTime`, `description`, `id`, `name`, `next`, `previous`, `tools`, `workflowId` | - |
| `DeleteTemplateRequest` | `structure` | `id` | - |
| `DeleteTemplateResponse` | `structure` | - | - |
| `DeleteMigrationWorkflowRequest` | `structure` | `id` | - |
| `DeleteMigrationWorkflowResponse` | `structure` | `arn`, `id`, `status` | - |
| `DeleteWorkflowStepRequest` | `structure` | `id`, `stepGroupId`, `workflowId` | - |
| `DeleteWorkflowStepResponse` | `structure` | - | - |
| `DeleteWorkflowStepGroupRequest` | `structure` | `id`, `workflowId` | - |
| `DeleteWorkflowStepGroupResponse` | `structure` | - | - |
| `GetMigrationWorkflowTemplateRequest` | `structure` | `id` | - |
| `GetMigrationWorkflowTemplateResponse` | `structure` | `creationTime`, `description`, `id`, `inputs`, `name`, `owner`, `status`, `statusMessage`, `tags`, `templateArn`, `templateClass`, `tools` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

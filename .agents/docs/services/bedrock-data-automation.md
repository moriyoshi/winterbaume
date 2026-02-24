# Data Automation for Amazon Bedrock

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Bedrock Data Automation BuildTime

## Possible Usage Scenarios
- From the AWS documentation and model: create data automation projects, blueprints, and related assets that define extraction and processing behaviour.
- From the operation surface: support project versioning, blueprint publishing, validation, tagging, and lifecycle management before runtime invocation.

## Service Identity and Protocol

- AWS model slug: `bedrock-data-automation`
- AWS SDK for Rust slug: `bedrock`
- Model version: `2023-07-26`
- Model file: `vendor/api-models-aws/models/bedrock-data-automation/service/2023-07-26/bedrock-data-automation-2023-07-26.json`
- SDK ID: `Bedrock Data Automation`
- Endpoint prefix: `bedrock-data-automation`
- ARN namespace: `bedrock`
- CloudFormation name: `-`
- CloudTrail event source: `bedrock.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (3), `Get` (3), `List` (3), `Delete` (2), `Update` (2), `Copy` (1), `Invoke` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBlueprint`, `CreateBlueprintVersion`, `CreateDataAutomationProject`, `DeleteBlueprint`, `DeleteDataAutomationProject`, `TagResource`, `UntagResource`, `UpdateBlueprint`, `UpdateDataAutomationProject`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBlueprint`, `GetBlueprintOptimizationStatus`, `GetDataAutomationProject`, `ListBlueprints`, `ListDataAutomationProjects`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BlueprintOptimizationJobResource` | `invocationArn` | create: `InvokeBlueprintOptimizationAsync`; read: `GetBlueprintOptimizationStatus` | - | - |
| `BlueprintResource` | `blueprintArn` | create: `CreateBlueprint`; read: `GetBlueprint`; update: `UpdateBlueprint`; delete: `DeleteBlueprint`; list: `ListBlueprints` | - | - |
| `DataAutomationProjectResource` | `projectArn` | create: `CreateDataAutomationProject`; read: `GetDataAutomationProject`; update: `UpdateDataAutomationProject`; delete: `DeleteDataAutomationProject`; list: `ListDataAutomationProjects` | - | - |
## Operation Groups

### Create

- Operations: `CreateBlueprint`, `CreateBlueprintVersion`, `CreateDataAutomationProject`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `blueprintArn`, `blueprintName`, `projectName`, `schema`, `standardOutputConfiguration`, `type`

### Get

- Operations: `GetBlueprint`, `GetBlueprintOptimizationStatus`, `GetDataAutomationProject`
- Traits: `readonly` (3)
- Common required input members in this group: `blueprintArn`, `invocationArn`, `projectArn`

### List

- Operations: `ListBlueprints`, `ListDataAutomationProjects`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (2)
- Common required input members in this group: `resourceARN`

### Delete

- Operations: `DeleteBlueprint`, `DeleteDataAutomationProject`
- Traits: `idempotent` (2)
- Common required input members in this group: `blueprintArn`, `projectArn`

### Update

- Operations: `UpdateBlueprint`, `UpdateDataAutomationProject`
- Traits: `idempotent` (2)
- Common required input members in this group: `blueprintArn`, `projectArn`, `schema`, `standardOutputConfiguration`

### Copy

- Operations: `CopyBlueprintStage`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `blueprintArn`, `sourceStage`, `targetStage`

### Invoke

- Operations: `InvokeBlueprintOptimizationAsync`
- Traits: `idempotent` (1)
- Common required input members in this group: `blueprint`, `dataAutomationProfileArn`, `outputConfiguration`, `samples`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceARN`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceARN`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CopyBlueprintStage` | `PUT /blueprints/{blueprintArn}/copy-stage` | `idempotent`, `idempotency-token` | `blueprintArn`, `sourceStage`, `targetStage` | `clientToken` | `CopyBlueprintStageResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Copies a Blueprint from one stage to another |
| `CreateBlueprint` | `PUT /blueprints/` | `idempotent`, `idempotency-token` | `blueprintName`, `schema`, `type` | `clientToken` | `CreateBlueprintResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Bedrock Data Automation Blueprint |
| `CreateBlueprintVersion` | `POST /blueprints/{blueprintArn}/versions/` | `idempotent`, `idempotency-token` | `blueprintArn` | `clientToken` | `CreateBlueprintVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new version of an existing Amazon Bedrock Data Automation Blueprint |
| `CreateDataAutomationProject` | `PUT /data-automation-projects/` | `idempotent`, `idempotency-token` | `projectName`, `standardOutputConfiguration` | `clientToken` | `CreateDataAutomationProjectResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Bedrock Data Automation Project |
| `DeleteBlueprint` | `DELETE /blueprints/{blueprintArn}/` | `idempotent` | `blueprintArn` | - | `DeleteBlueprintResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing Amazon Bedrock Data Automation Blueprint |
| `DeleteDataAutomationProject` | `DELETE /data-automation-projects/{projectArn}/` | `idempotent` | `projectArn` | - | `DeleteDataAutomationProjectResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing Amazon Bedrock Data Automation Project |
| `GetBlueprint` | `POST /blueprints/{blueprintArn}/` | `readonly` | `blueprintArn` | - | `GetBlueprintResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an existing Amazon Bedrock Data Automation Blueprint |
| `GetBlueprintOptimizationStatus` | `POST /getBlueprintOptimizationStatus/{invocationArn}` | `readonly` | `invocationArn` | - | `GetBlueprintOptimizationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | API used to get blueprint optimization status. |
| `GetDataAutomationProject` | `POST /data-automation-projects/{projectArn}/` | `readonly` | `projectArn` | - | `GetDataAutomationProjectResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an existing Amazon Bedrock Data Automation Project |
| `InvokeBlueprintOptimizationAsync` | `POST /invokeBlueprintOptimizationAsync` | `idempotent` | `blueprint`, `dataAutomationProfileArn`, `outputConfiguration`, `samples` | - | `InvokeBlueprintOptimizationAsyncResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Invoke an async job to perform Blueprint Optimization |
| `ListBlueprints` | `POST /blueprints/` | `readonly`, `paginated` | - | - | `ListBlueprintsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all existing Amazon Bedrock Data Automation Blueprints |
| `ListDataAutomationProjects` | `POST /data-automation-projects/` | `readonly`, `paginated` | - | - | `ListDataAutomationProjectsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all existing Amazon Bedrock Data Automation Projects |
| `ListTagsForResource` | `POST /listTagsForResource` | - | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for an Amazon Bedrock Data Automation resource |
| `TagResource` | `POST /tagResource` | - | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tag an Amazon Bedrock Data Automation resource |
| `UntagResource` | `POST /untagResource` | - | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untag an Amazon Bedrock Data Automation resource |
| `UpdateBlueprint` | `PUT /blueprints/{blueprintArn}/` | `idempotent` | `blueprintArn`, `schema` | - | `UpdateBlueprintResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing Amazon Bedrock Data Automation Blueprint |
| `UpdateDataAutomationProject` | `PUT /data-automation-projects/{projectArn}/` | `idempotent` | `projectArn`, `standardOutputConfiguration` | - | `UpdateDataAutomationProjectResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing Amazon Bedrock Data Automation Project |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | This exception is thrown when a request is denied per access permissions |
| `InternalServerException` | `structure` | `message` | This exception is thrown if there was an unexpected error during processing of request |
| `ThrottlingException` | `structure` | `message` | This exception is thrown when the number of requests exceeds the limit |
| `ValidationException` | `structure` | `fieldList`, `message` | This exception is thrown when the request's input validation fails |
| `ResourceNotFoundException` | `structure` | `message` | This exception is thrown when a resource referenced by the operation does not exist |
| `ServiceQuotaExceededException` | `structure` | `message` | This exception is thrown when a request is made beyond the service quota |
| `ConflictException` | `structure` | `message` | This exception is thrown when there is a conflict performing an operation |
| `CopyBlueprintStageRequest` | `structure` | `blueprintArn`, `clientToken`, `sourceStage`, `targetStage` | CopyBlueprintStage Request |
| `CopyBlueprintStageResponse` | `structure` | - | CopyBlueprintStage Response |
| `CreateBlueprintRequest` | `structure` | `blueprintName`, `blueprintStage`, `clientToken`, `encryptionConfiguration`, `schema`, `tags`, `type` | Create Blueprint Request |
| `CreateBlueprintResponse` | `structure` | `blueprint` | Create Blueprint Response |
| `CreateBlueprintVersionRequest` | `structure` | `blueprintArn`, `clientToken` | Create Blueprint Version Request |
| `CreateBlueprintVersionResponse` | `structure` | `blueprint` | Create Blueprint Version Response |
| `CreateDataAutomationProjectRequest` | `structure` | `clientToken`, `customOutputConfiguration`, `encryptionConfiguration`, `overrideConfiguration`, `projectDescription`, `projectName`, `projectStage`, `projectType`, `standardOutputConfiguration`, `tags` | Create DataAutomationProject Request |
| `CreateDataAutomationProjectResponse` | `structure` | `projectArn`, `projectStage`, `status` | Create DataAutomationProject Response |
| `DeleteBlueprintRequest` | `structure` | `blueprintArn`, `blueprintVersion` | Delete Blueprint Request |
| `DeleteBlueprintResponse` | `structure` | - | Delete Blueprint Response |
| `DeleteDataAutomationProjectRequest` | `structure` | `projectArn` | Delete DataAutomationProject Request |
| `DeleteDataAutomationProjectResponse` | `structure` | `projectArn`, `status` | Delete DataAutomationProject Response |
| `GetBlueprintRequest` | `structure` | `blueprintArn`, `blueprintStage`, `blueprintVersion` | Get Blueprint Request |
| `GetBlueprintResponse` | `structure` | `blueprint` | Get Blueprint Response |
| `GetBlueprintOptimizationStatusRequest` | `structure` | `invocationArn` | Structure for request of GetBlueprintOptimizationStatus API. |
| `GetBlueprintOptimizationStatusResponse` | `structure` | `errorMessage`, `errorType`, `outputConfiguration`, `status` | Response of GetBlueprintOptimizationStatus API. |
| `GetDataAutomationProjectRequest` | `structure` | `projectArn`, `projectStage` | Get DataAutomationProject Request |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

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

### List

- Operations: `ListDataAutomationLibraryEntities`, `ListTagsForResource`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Copy

- Operations: `CopyBlueprintStage`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateBlueprintVersion`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Get

- Operations: `GetDataAutomationLibraryEntity`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CopyBlueprintStage` | `PUT /blueprints/{blueprintArn}/copy-stage` | `idempotent`, `idempotency-token` | `blueprintArn`, `sourceStage`, `targetStage` | `clientToken` | `CopyBlueprintStageResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Copies a Blueprint from one stage to another |
| `CreateBlueprintVersion` | `POST /blueprints/{blueprintArn}/versions/` | `idempotent`, `idempotency-token` | `blueprintArn` | `clientToken` | `CreateBlueprintVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new version of an existing Amazon Bedrock Data Automation Blueprint |
| `GetDataAutomationLibraryEntity` | `POST /data-automation-libraries/{libraryArn}/entityType/{entityType}/entities/{entityId}` | `readonly` | `libraryArn`, `entityType`, `entityId` | - | `GetDataAutomationLibraryEntityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an existing entity based on entity type from the library |
| `ListDataAutomationLibraryEntities` | `POST /data-automation-libraries/{libraryArn}/entityType/{entityType}/entities/` | `readonly`, `paginated` | `libraryArn`, `entityType` | - | `ListDataAutomationLibraryEntitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all stored entities in the library |
| `ListTagsForResource` | `POST /listTagsForResource` | - | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for an Amazon Bedrock Data Automation resource |
| `TagResource` | `POST /tagResource` | - | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tag an Amazon Bedrock Data Automation resource |
| `UntagResource` | `POST /untagResource` | - | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untag an Amazon Bedrock Data Automation resource |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | This exception is thrown when a request is denied per access permissions |
| `ConflictException` | `structure` | message | This exception is thrown when there is a conflict performing an operation |
| `InternalServerException` | `structure` | message | This exception is thrown if there was an unexpected error during processing of request |
| `ResourceNotFoundException` | `structure` | message | This exception is thrown when a resource referenced by the operation does not exist |
| `ServiceQuotaExceededException` | `structure` | message | This exception is thrown when a request is made beyond the service quota |
| `ThrottlingException` | `structure` | message | This exception is thrown when the number of requests exceeds the limit |
| `ValidationException` | `structure` | message, fieldList | This exception is thrown when the request's input validation fails |
| `CopyBlueprintStageRequest` | `structure` | blueprintArn, sourceStage, targetStage, clientToken | CopyBlueprintStage Request |
| `CopyBlueprintStageResponse` | `structure` | **empty (no members)** | CopyBlueprintStage Response |
| `CreateBlueprintVersionRequest` | `structure` | blueprintArn, clientToken | Create Blueprint Version Request |
| `CreateBlueprintVersionResponse` | `structure` | blueprint | Create Blueprint Version Response |
| `GetDataAutomationLibraryEntityRequest` | `structure` | libraryArn, entityType, entityId | Get DataAutomationLibraryEntity Request |
| `GetDataAutomationLibraryEntityResponse` | `structure` | entity | Get DataAutomationLibraryEntity Response |
| `ListDataAutomationLibraryEntitiesRequest` | `structure` | libraryArn, entityType, maxResults, nextToken | List DataAutomationLibraryEntities Request |
| `ListDataAutomationLibraryEntitiesResponse` | `structure` | entities, nextToken | List DataAutomationLibraryEntities Response |
| `ListTagsForResourceRequest` | `structure` | resourceARN | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceARN, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceARN, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AudioExtractionCategoryType` | `enum` | AUDIO_CONTENT_MODERATION, TRANSCRIPT, TOPIC_CONTENT_MODERATION | - |
| `AudioGenerativeOutputLanguage` | `enum` | DEFAULT, EN | Configuration for Audio output language |
| `AudioStandardGenerativeFieldType` | `enum` | AUDIO_SUMMARY, IAB, TOPIC_SUMMARY | - |
| `BlueprintOptimizationJobStatus` | `enum` | CREATED, IN_PROGRESS, SUCCESS, SERVICE_ERROR, CLIENT_ERROR | List of status supported by optimization jobs |
| `BlueprintStage` | `enum` | DEVELOPMENT, LIVE | Stage of the Blueprint |
| `BlueprintStageFilter` | `enum` | DEVELOPMENT, LIVE, ALL | Blueprint Stage filter |
| `DataAutomationLibraryStatus` | `enum` | ACTIVE, DELETING | Status of DataAutomationLibrary |
| `DataAutomationProjectStage` | `enum` | DEVELOPMENT, LIVE | Stage of the Project |
| `DataAutomationProjectStageFilter` | `enum` | DEVELOPMENT, LIVE, ALL | Project Stage filter |
| `DataAutomationProjectStatus` | `enum` | COMPLETED, IN_PROGRESS, FAILED | Status of Data Automation Project |
| `DataAutomationProjectType` | `enum` | ASYNC, SYNC | Type of the DataAutomationProject |
| `DesiredModality` | `enum` | IMAGE, DOCUMENT, AUDIO, VIDEO | Desired Modality types |
| `DocumentExtractionGranularityType` | `enum` | DOCUMENT, PAGE, ELEMENT, WORD, LINE | - |
| `DocumentOutputTextFormatType` | `enum` | PLAIN_TEXT, MARKDOWN, HTML, CSV | - |
| `EntityType` | `enum` | VOCABULARY | Entity types supported in DataAutomationLibraries |
| `ImageExtractionCategoryType` | `enum` | CONTENT_MODERATION, TEXT_DETECTION, LOGOS | - |
| `ImageStandardGenerativeFieldType` | `enum` | IMAGE_SUMMARY, IAB | - |
| `Language` | `enum` | EN, DE, ES, FR, IT, PT, JA, KO, CN, TW, HK | Supported input languages |
| `LibraryIngestionJobOperationType` | `enum` | UPSERT, DELETE | DataAutomationLibraryIngestionJob operation type |
| `LibraryIngestionJobStatus` | `enum` | IN_PROGRESS, COMPLETED, COMPLETED_WITH_ERRORS, FAILED | Status of DataAutomationLibraryIngestionJob |
| `PIIEntityType` | `enum` | ALL, ADDRESS, AGE, NAME, EMAIL, PHONE, USERNAME, PASSWORD, DRIVER_ID, LICENSE_PLATE, VEHICLE_IDENTIFICATION_NUMBER, CREDIT_DEBIT_CARD_CVV, ... (+20) | Types of PII entities that can be detected, we will support every types that Guardrails can support |
| `PIIRedactionMaskMode` | `enum` | PII, ENTITY_TYPE | Mode for redacting detected PII |
| `ResourceOwner` | `enum` | SERVICE, ACCOUNT | Resource Owner |
| `SensitiveDataDetectionMode` | `enum` | DETECTION, DETECTION_AND_REDACTION | Mode for sensitive data detection |
| `SensitiveDataDetectionScopeType` | `enum` | STANDARD, CUSTOM | Types of sensitive data detection scope |
| `State` | `enum` | ENABLED, DISABLED | State |
| `Type` | `enum` | DOCUMENT, IMAGE, AUDIO, VIDEO | Type |
| `VideoExtractionCategoryType` | `enum` | CONTENT_MODERATION, TEXT_DETECTION, TRANSCRIPT, LOGOS | - |
| `VideoStandardGenerativeFieldType` | `enum` | VIDEO_SUMMARY, IAB, CHAPTER_SUMMARY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon Omics

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services HealthOmics is a service that helps users such as bioinformaticians, researchers, and scientists to store, query, analyze, and generate insights from genomics and other biological data. It simplifies and accelerates the process of storing and analyzing genomic information for Amazon Web Services. For an introduction to the service, see What is Amazon Web Services HealthOmics? in the Amazon Web Services HealthOmics User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Omics resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Omics workflows in the local mock. Key resources include `AnnotationImportJob`, `AnnotationStore`, `AnnotationStoreVersion`, `ReadSetResource`, `ReferenceResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Create`, `Update` operation families, including `GetAnnotationImportJob`, `GetAnnotationStore`, `GetAnnotationStoreVersion`, `GetReadSet`, `ListAnnotationImportJobs`, `ListAnnotationStoreVersions`.

## Service Identity and Protocol

- AWS model slug: `omics`
- AWS SDK for Rust slug: `omics`
- Model version: `2022-11-28`
- Model file: `vendor/api-models-aws/models/omics/service/2022-11-28/omics-2022-11-28.json`
- SDK ID: `Omics`
- Endpoint prefix: `-`
- ARN namespace: `omics`
- CloudFormation name: `-`
- CloudTrail event source: `omics.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (23), `List` (23), `Delete` (13), `Create` (11), `Update` (8), `Start` (7), `Cancel` (3), `Abort` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptShare`, `BatchDeleteReadSet`, `CancelAnnotationImportJob`, `CancelRun`, `CancelVariantImportJob`, `CreateAnnotationStore`, `CreateAnnotationStoreVersion`, `CreateMultipartReadSetUpload`, `CreateReferenceStore`, `CreateRunCache`, `CreateRunGroup`, `CreateSequenceStore`, `CreateShare`, `CreateVariantStore`, `CreateWorkflow`, `CreateWorkflowVersion`, `DeleteAnnotationStore`, `DeleteAnnotationStoreVersions`, `DeleteReference`, `DeleteReferenceStore`, ... (+27).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAnnotationImportJob`, `GetAnnotationStore`, `GetAnnotationStoreVersion`, `GetReadSet`, `GetReadSetActivationJob`, `GetReadSetExportJob`, `GetReadSetImportJob`, `GetReadSetMetadata`, `GetReference`, `GetReferenceImportJob`, `GetReferenceMetadata`, `GetReferenceStore`, `GetRun`, `GetRunCache`, `GetRunGroup`, `GetRunTask`, `GetS3AccessPolicy`, `GetSequenceStore`, `GetShare`, `GetVariantImportJob`, ... (+26).
- Pagination is modelled for 22 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 24 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelAnnotationImportJob`, `CancelRun`, `CancelVariantImportJob`, `GetAnnotationImportJob`, `GetReadSetActivationJob`, `GetReadSetExportJob`, `GetReadSetImportJob`, `GetReferenceImportJob`, `GetRunTask`, `GetVariantImportJob`, `ListAnnotationImportJobs`, `ListReadSetActivationJobs`, `ListReadSetExportJobs`, `ListReadSetImportJobs`, `ListReferenceImportJobs`, `ListRunTasks`, `ListVariantImportJobs`, `StartAnnotationImportJob`, `StartReadSetActivationJob`, `StartReadSetExportJob`, ... (+4).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 96 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECR`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AnnotationImportJob` | `jobId` | create: `StartAnnotationImportJob`; read: `GetAnnotationImportJob`; delete: `CancelAnnotationImportJob`; list: `ListAnnotationImportJobs` | - | - |
| `AnnotationStore` | `name` | create: `CreateAnnotationStore`; read: `GetAnnotationStore`; update: `UpdateAnnotationStore`; delete: `DeleteAnnotationStore`; list: `ListAnnotationStores` | - | Represents a resource that stores annotation data |
| `AnnotationStoreVersion` | `name`, `versionName` | create: `CreateAnnotationStoreVersion`; read: `GetAnnotationStoreVersion`; update: `UpdateAnnotationStoreVersion`; list: `ListAnnotationStoreVersions` | `DeleteAnnotationStoreVersions` | - |
| `ReadSetResource` | `readSetId`, `sequenceStoreId` | read: `GetReadSetMetadata`; list: `ListReadSets` | `BatchDeleteReadSet`, `GetReadSet` | Represents a resource that stores a single genomic sequence file |
| `ReferenceResource` | `referenceId`, `referenceStoreId` | read: `GetReferenceMetadata`; delete: `DeleteReference`; list: `ListReferences` | `GetReference` | Represents a resource that stores a single genomic reference file |
| `ReferenceStoreResource` | `referenceStoreId` | create: `CreateReferenceStore`; read: `GetReferenceStore`; delete: `DeleteReferenceStore`; list: `ListReferenceStores` | `GetReferenceImportJob`, `ListReferenceImportJobs`, `StartReferenceImportJob` | Represents a resource that stores genomic reference data |
| `RunCacheResource` | `id` | create: `CreateRunCache`; read: `GetRunCache`; update: `UpdateRunCache`; delete: `DeleteRunCache`; list: `ListRunCaches` | - | - |
| `RunGroupResource` | `id` | create: `CreateRunGroup`; read: `GetRunGroup`; update: `UpdateRunGroup`; delete: `DeleteRunGroup`; list: `ListRunGroups` | - | - |
| `RunResource` | `id` | create: `StartRun`; read: `GetRun`; delete: `DeleteRun`; list: `ListRuns` | `CancelRun` | - |
| `SequenceStoreResource` | `sequenceStoreId` | create: `CreateSequenceStore`; read: `GetSequenceStore`; update: `UpdateSequenceStore`; delete: `DeleteSequenceStore`; list: `ListSequenceStores` | `AbortMultipartReadSetUpload`, `CompleteMultipartReadSetUpload`, `CreateMultipartReadSetUpload`, `GetReadSetActivationJob`, `GetReadSetExportJob`, `GetReadSetImportJob`, `ListMultipartReadSetUploads`, `ListReadSetActivationJobs`, `ListReadSetExportJobs`, `ListReadSetImportJobs`, ... (+5) | Represents a resource that stores genomic sequence data |
| `Share` | `shareId` | create: `CreateShare`; read: `GetShare`; update: `AcceptShare`; delete: `DeleteShare`; list: `ListShares` | - | - |
| `TaggingResource` | `tagKey` | list: `ListTagsForResource` | `TagResource`, `UntagResource` | - |
| `TaskResource` | `id`, `taskId` | read: `GetRunTask`; list: `ListRunTasks` | - | - |
| `VariantImportJob` | `jobId` | create: `StartVariantImportJob`; read: `GetVariantImportJob`; delete: `CancelVariantImportJob`; list: `ListVariantImportJobs` | - | - |
| `VariantStore` | `name` | create: `CreateVariantStore`; read: `GetVariantStore`; update: `UpdateVariantStore`; delete: `DeleteVariantStore`; list: `ListVariantStores` | - | Represents a resource that stores variant data |
| `WorkflowResource` | `id` | create: `CreateWorkflow`; read: `GetWorkflow`; update: `UpdateWorkflow`; delete: `DeleteWorkflow`; list: `ListWorkflows` | - | - |
| `WorkflowVersion` | `id`, `versionName` | put: `CreateWorkflowVersion`; read: `GetWorkflowVersion`; update: `UpdateWorkflowVersion`; delete: `DeleteWorkflowVersion`; list: `ListWorkflowVersions` | - | - |
## Operation Groups

### Delete

- Operations: `DeleteS3AccessPolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetS3AccessPolicy`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutS3AccessPolicy`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteS3AccessPolicy` | `DELETE /s3accesspolicy/{s3AccessPointArn}` | `idempotent` | `s3AccessPointArn` | - | `DeleteS3AccessPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `NotSupportedOperationException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an access policy for the specified store. |
| `GetS3AccessPolicy` | `GET /s3accesspolicy/{s3AccessPointArn}` | `readonly` | `s3AccessPointArn` | - | `GetS3AccessPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `NotSupportedOperationException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves details about an access policy on a given store. |
| `PutS3AccessPolicy` | `PUT /s3accesspolicy/{s3AccessPointArn}` | - | `s3AccessPointArn`, `s3AccessPolicy` | - | `PutS3AccessPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `NotSupportedOperationException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds an access policy to the specified store. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The request cannot be applied to the target resource in its current state. |
| `InternalServerException` | `structure` | message | An unexpected error occurred. Try the request again. |
| `NotSupportedOperationException` | `structure` | message | The operation is not supported by Amazon Omics, or the API does not exist. |
| `RangeNotSatisfiableException` | `structure` | message | The ranges specified in the request are not valid. |
| `RequestTimeoutException` | `structure` | message | The request timed out. |
| `ResourceNotFoundException` | `structure` | message | The target resource was not found in the current Region. |
| `ServiceQuotaExceededException` | `structure` | message | The request exceeds a service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an AWS service. |
| `DeleteS3AccessPolicyRequest` | `structure` | s3AccessPointArn | - |
| `DeleteS3AccessPolicyResponse` | `structure` | **empty (no members)** | - |
| `GetS3AccessPolicyRequest` | `structure` | s3AccessPointArn | - |
| `GetS3AccessPolicyResponse` | `structure` | s3AccessPointArn, storeId, storeType, updateTime, s3AccessPolicy | - |
| `PutS3AccessPolicyRequest` | `structure` | s3AccessPointArn, s3AccessPolicy | - |
| `PutS3AccessPolicyResponse` | `structure` | s3AccessPointArn, storeId, storeType | - |
| `StoreType` | `enum` | SEQUENCE_STORE, REFERENCE_STORE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

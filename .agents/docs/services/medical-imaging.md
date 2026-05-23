# AWS Health Imaging

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the AWS HealthImaging API Reference . For an introduction to the service, see What is AWS HealthImaging? in the AWS HealthImaging Developer Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Health Imaging resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Health Imaging workflows in the local mock. Key resources include `DatastoreResource`, `ImageSetResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Copy`, `Create` operation families, including `GetDICOMImportJob`, `GetDatastore`, `GetImageFrame`, `GetImageSet`, `ListDICOMImportJobs`, `ListDatastores`.

## Service Identity and Protocol

- AWS model slug: `medical-imaging`
- AWS SDK for Rust slug: `medicalimaging`
- Model version: `2023-07-19`
- Model file: `vendor/api-models-aws/models/medical-imaging/service/2023-07-19/medical-imaging-2023-07-19.json`
- SDK ID: `Medical Imaging`
- Endpoint prefix: `medical-imaging`
- ARN namespace: `medical-imaging`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (4), `Delete` (2), `Copy` (1), `Create` (1), `Search` (1), `Start` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDatastore`, `DeleteDatastore`, `DeleteImageSet`, `StartDICOMImportJob`, `TagResource`, `UntagResource`, `UpdateImageSetMetadata`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDICOMImportJob`, `GetDatastore`, `GetImageFrame`, `GetImageSet`, `GetImageSetMetadata`, `ListDICOMImportJobs`, `ListDatastores`, `ListImageSetVersions`, `ListTagsForResource`, `SearchImageSets`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetDICOMImportJob`, `ListDICOMImportJobs`, `StartDICOMImportJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 18 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DatastoreResource` | `datastoreId` | create: `CreateDatastore`; read: `GetDatastore`; delete: `DeleteDatastore`; list: `ListDatastores` | - | - |
| `ImageSetResource` | `datastoreId`, `imageSetId` | - | - | - |
## Operation Groups

### Get

- Operations: `GetDICOMImportJob`, `GetImageFrame`, `GetImageSet`, `GetImageSetMetadata`
- Traits: `readonly` (4)
- Common required input members in this group: `datastoreId`, `imageSetId`

### List

- Operations: `ListDICOMImportJobs`, `ListImageSetVersions`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: `datastoreId`

### Copy

- Operations: `CopyImageSet`
- Common required input members in this group: -

### Delete

- Operations: `DeleteImageSet`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Search

- Operations: `SearchImageSets`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartDICOMImportJob`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateImageSetMetadata`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CopyImageSet` | `POST /datastore/{datastoreId}/imageSet/{sourceImageSetId}/copyImageSet` | - | `datastoreId`, `sourceImageSetId`, `copyImageSetInformation` | - | `CopyImageSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Copy an image set. |
| `DeleteImageSet` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/deleteImageSet` | `idempotent` | `datastoreId`, `imageSetId` | - | `DeleteImageSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an image set. |
| `GetDICOMImportJob` | `GET /getDICOMImportJob/datastore/{datastoreId}/job/{jobId}` | `readonly` | `datastoreId`, `jobId` | - | `GetDICOMImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the import job properties to learn more about the job or job progress. The jobStatus refers to the execution of the import job. Therefore, an import job can return a jobStatus as COMPLETED even if validation issu ... |
| `GetImageFrame` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/getImageFrame` | `readonly` | `datastoreId`, `imageSetId`, `imageFrameInformation` | - | `GetImageFrameResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `NotAcceptableException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an image frame (pixel data) for an image set. |
| `GetImageSet` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/getImageSet` | `readonly` | `datastoreId`, `imageSetId` | - | `GetImageSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get image set properties. |
| `GetImageSetMetadata` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/getImageSetMetadata` | `readonly` | `datastoreId`, `imageSetId` | - | `GetImageSetMetadataResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get metadata attributes for an image set. |
| `ListDICOMImportJobs` | `GET /listDICOMImportJobs/datastore/{datastoreId}` | `readonly`, `paginated` | `datastoreId` | - | `ListDICOMImportJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List import jobs created for a specific data store. |
| `ListImageSetVersions` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/listImageSetVersions` | `readonly`, `paginated` | `datastoreId`, `imageSetId` | - | `ListImageSetVersionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List image set versions. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with a medical imaging resource. |
| `SearchImageSets` | `POST /datastore/{datastoreId}/searchImageSets` | `paginated` | `datastoreId` | - | `SearchImageSetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Search image sets based on defined input attributes. SearchImageSets accepts a single search query parameter and returns a paginated response of all image sets that have the matching criteria. All date range queries ... |
| `StartDICOMImportJob` | `POST /startDICOMImportJob/datastore/{datastoreId}` | `idempotent`, `idempotency-token` | `dataAccessRoleArn`, `clientToken`, `datastoreId`, `inputS3Uri`, `outputS3Uri` | `clientToken` | `StartDICOMImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Start importing bulk data into an ACTIVE data store. The import job imports DICOM P10 files found in the S3 prefix specified by the inputS3Uri parameter. The import job stores processing results in the file specified ... |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a user-specifed key and value tag to a medical imaging resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a medical imaging resource. |
| `UpdateImageSetMetadata` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/updateImageSetMetadata` | - | `datastoreId`, `imageSetId`, `latestVersionId`, `updateImageSetMetadataUpdates` | - | `UpdateImageSetMetadataResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update image set metadata attributes. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CopyImageSet` | - | `force -> force`, `promoteToPrimary -> promoteToPrimary` | - | `copyImageSetInformation` |
| `GetImageFrame` | - | - | - | `imageFrameInformation` |
| `GetImageSet` | - | `versionId -> version` | - | - |
| `GetImageSetMetadata` | - | `versionId -> version` | - | - |
| `ListDICOMImportJobs` | - | `jobStatus -> jobStatus`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListImageSetVersions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `SearchImageSets` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | `searchCriteria` |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |
| `UpdateImageSetMetadata` | - | `latestVersionId -> latestVersion`, `force -> force`, `includeStudyImageSets -> includeStudyImageSets` | - | `updateImageSetMetadataUpdates` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The user does not have sufficient access to perform this action. |
| `BadRequestException` | `structure` | message | The request is invalid or malformed. |
| `ConflictException` | `structure` | message | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message | An unexpected error occurred during processing of the request. |
| `NotAcceptableException` | `structure` | message | The request content type or accept header is not supported. |
| `ResourceNotFoundException` | `structure` | message | The request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | The request caused a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message | The request was denied due to throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints set by the service. |
| `CopyImageSetRequest` | `structure` | datastoreId, sourceImageSetId, copyImageSetInformation, force, promoteToPrimary | - |
| `CopyImageSetResponse` | `structure` | datastoreId, sourceImageSetProperties, destinationImageSetProperties | - |
| `DeleteImageSetRequest` | `structure` | datastoreId, imageSetId | - |
| `DeleteImageSetResponse` | `structure` | datastoreId, imageSetId, imageSetState, imageSetWorkflowStatus | - |
| `GetDICOMImportJobRequest` | `structure` | datastoreId, jobId | - |
| `GetDICOMImportJobResponse` | `structure` | jobProperties | - |
| `GetImageFrameRequest` | `structure` | datastoreId, imageSetId, imageFrameInformation | - |
| `GetImageFrameResponse` | `structure` | imageFrameBlob, contentType | - |
| `GetImageSetRequest` | `structure` | datastoreId, imageSetId, versionId | - |
| `GetImageSetResponse` | `structure` | datastoreId, imageSetId, versionId, imageSetState, imageSetWorkflowStatus, createdAt, updatedAt, deletedAt, message, imageSetArn, overrides, isPrimary, ... (+2) | - |
| `GetImageSetMetadataRequest` | `structure` | datastoreId, imageSetId, versionId | - |
| `GetImageSetMetadataResponse` | `structure` | imageSetMetadataBlob, contentType, contentEncoding | - |
| `ListDICOMImportJobsRequest` | `structure` | datastoreId, jobStatus, nextToken, maxResults | - |
| `ListDICOMImportJobsResponse` | `structure` | jobSummaries, nextToken | - |
| `ListImageSetVersionsRequest` | `structure` | datastoreId, imageSetId, nextToken, maxResults | - |
| `ListImageSetVersionsResponse` | `structure` | imageSetPropertiesList, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `SearchImageSetsRequest` | `structure` | datastoreId, searchCriteria, maxResults, nextToken | - |
| `SearchImageSetsResponse` | `structure` | imageSetsMetadataSummaries, sort, nextToken | - |
| `StartDICOMImportJobRequest` | `structure` | jobName, dataAccessRoleArn, clientToken, datastoreId, inputS3Uri, outputS3Uri, inputOwnerAccountId | - |
| `StartDICOMImportJobResponse` | `structure` | datastoreId, jobId, jobStatus, submittedAt | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateImageSetMetadataRequest` | `structure` | datastoreId, imageSetId, latestVersionId, force, includeStudyImageSets, updateImageSetMetadataUpdates | - |
| `UpdateImageSetMetadataResponse` | `structure` | datastoreId, imageSetId, latestVersionId, imageSetState, imageSetWorkflowStatus, createdAt, updatedAt, message | - |
| `DatastoreStatus` | `enum` | CREATING, CREATE_FAILED, ACTIVE, DELETING, DELETED | - |
| `ImageSetState` | `enum` | ACTIVE, LOCKED, DELETED | - |
| `ImageSetWorkflowStatus` | `enum` | CREATED, COPIED, COPYING, COPYING_WITH_READ_ONLY_ACCESS, COPY_FAILED, UPDATING, UPDATING_FOR_STUDY_CONSISTENCY, UPDATED, UPDATE_FAILED, DELETING, DELETED, IMPORTING, ... (+2) | - |
| `JobStatus` | `enum` | SUBMITTED, IN_PROGRESS, COMPLETED, FAILED | - |
| `LosslessStorageFormat` | `enum` | HTJ2K, JPEG_2000_LOSSLESS | - |
| `Operator` | `enum` | EQUAL, BETWEEN | - |
| `SortField` | `enum` | updatedAt, createdAt, DICOMStudyDateAndTime | - |
| `SortOrder` | `enum` | ASC, DESC | - |
| `StorageTier` | `enum` | FREQUENT_ACCESS, ARCHIVE_INSTANT_ACCESS | Storage tier for image sets |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

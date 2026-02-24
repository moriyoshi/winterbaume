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

- Operations: `GetDICOMImportJob`, `GetDatastore`, `GetImageFrame`, `GetImageSet`, `GetImageSetMetadata`
- Traits: `endpoint-bound` (3), `readonly` (5)
- Common required input members in this group: `datastoreId`, `imageFrameInformation`, `imageSetId`, `jobId`

### List

- Operations: `ListDICOMImportJobs`, `ListDatastores`, `ListImageSetVersions`, `ListTagsForResource`
- Traits: `endpoint-bound` (1), `paginated` (3), `readonly` (4)
- Common required input members in this group: `datastoreId`, `imageSetId`, `resourceArn`

### Delete

- Operations: `DeleteDatastore`, `DeleteImageSet`
- Traits: `endpoint-bound` (1), `idempotent` (2)
- Common required input members in this group: `datastoreId`, `imageSetId`

### Copy

- Operations: `CopyImageSet`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `copyImageSetInformation`, `datastoreId`, `sourceImageSetId`

### Create

- Operations: `CreateDatastore`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `clientToken`

### Search

- Operations: `SearchImageSets`
- Traits: `endpoint-bound` (1), `paginated` (1)
- Common required input members in this group: `datastoreId`

### Start

- Operations: `StartDICOMImportJob`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `clientToken`, `dataAccessRoleArn`, `datastoreId`, `inputS3Uri`, `outputS3Uri`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateImageSetMetadata`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `datastoreId`, `imageSetId`, `latestVersionId`, `updateImageSetMetadataUpdates`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CopyImageSet` | `POST /datastore/{datastoreId}/imageSet/{sourceImageSetId}/copyImageSet` | `endpoint-bound` | `copyImageSetInformation`, `datastoreId`, `sourceImageSetId` | - | `CopyImageSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Copy an image set. |
| `CreateDatastore` | `POST /datastore` | `idempotent`, `idempotency-token` | `clientToken` | `clientToken` | `CreateDatastoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a data store. |
| `DeleteDatastore` | `DELETE /datastore/{datastoreId}` | `idempotent` | `datastoreId` | - | `DeleteDatastoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a data store. Before a data store can be deleted, you must first delete all image sets within it. |
| `DeleteImageSet` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/deleteImageSet` | `idempotent`, `endpoint-bound` | `datastoreId`, `imageSetId` | - | `DeleteImageSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an image set. |
| `GetDICOMImportJob` | `GET /getDICOMImportJob/datastore/{datastoreId}/job/{jobId}` | `readonly` | `datastoreId`, `jobId` | - | `GetDICOMImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the import job properties to learn more about the job or job progress. The `jobStatus` refers to the execution of the import job. |
| `GetDatastore` | `GET /datastore/{datastoreId}` | `readonly` | `datastoreId` | - | `GetDatastoreResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get data store properties. |
| `GetImageFrame` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/getImageFrame` | `readonly`, `endpoint-bound` | `datastoreId`, `imageFrameInformation`, `imageSetId` | - | `GetImageFrameResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an image frame (pixel data) for an image set. |
| `GetImageSet` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/getImageSet` | `readonly`, `endpoint-bound` | `datastoreId`, `imageSetId` | - | `GetImageSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get image set properties. |
| `GetImageSetMetadata` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/getImageSetMetadata` | `readonly`, `endpoint-bound` | `datastoreId`, `imageSetId` | - | `GetImageSetMetadataResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get metadata attributes for an image set. |
| `ListDICOMImportJobs` | `GET /listDICOMImportJobs/datastore/{datastoreId}` | `readonly`, `paginated` | `datastoreId` | - | `ListDICOMImportJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List import jobs created for a specific data store. |
| `ListDatastores` | `GET /datastore` | `readonly`, `paginated` | - | - | `ListDatastoresResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List data stores. |
| `ListImageSetVersions` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/listImageSetVersions` | `readonly`, `paginated`, `endpoint-bound` | `datastoreId`, `imageSetId` | - | `ListImageSetVersionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List image set versions. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with a medical imaging resource. |
| `SearchImageSets` | `POST /datastore/{datastoreId}/searchImageSets` | `paginated`, `endpoint-bound` | `datastoreId` | - | `SearchImageSetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Search image sets based on defined input attributes. `SearchImageSets` accepts a single search query parameter and returns a paginated response of all image sets that have the matching criteria. |
| `StartDICOMImportJob` | `POST /startDICOMImportJob/datastore/{datastoreId}` | `idempotent`, `idempotency-token` | `clientToken`, `dataAccessRoleArn`, `datastoreId`, `inputS3Uri`, `outputS3Uri` | `clientToken` | `StartDICOMImportJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Start importing bulk data into an `ACTIVE` data store. The import job imports DICOM P10 files found in the S3 prefix specified by the `inputS3Uri` parameter. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a user-specifed key and value tag to a medical imaging resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a medical imaging resource. |
| `UpdateImageSetMetadata` | `POST /datastore/{datastoreId}/imageSet/{imageSetId}/updateImageSetMetadata` | `endpoint-bound` | `datastoreId`, `imageSetId`, `latestVersionId`, `updateImageSetMetadataUpdates` | - | `UpdateImageSetMetadataResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update image set metadata attributes. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The user does not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | An unexpected error occurred during processing of the request. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to throttling. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints set by the service. |
| `ResourceNotFoundException` | `structure` | `message` | The request references a resource which does not exist. |
| `ConflictException` | `structure` | `message` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request caused a service quota to be exceeded. |
| `CopyImageSetRequest` | `structure` | `copyImageSetInformation`, `datastoreId`, `force`, `promoteToPrimary`, `sourceImageSetId` | - |
| `CopyImageSetResponse` | `structure` | `datastoreId`, `destinationImageSetProperties`, `sourceImageSetProperties` | - |
| `CreateDatastoreRequest` | `structure` | `clientToken`, `datastoreName`, `kmsKeyArn`, `lambdaAuthorizerArn`, `losslessStorageFormat`, `tags` | - |
| `CreateDatastoreResponse` | `structure` | `datastoreId`, `datastoreStatus` | - |
| `DeleteDatastoreRequest` | `structure` | `datastoreId` | - |
| `DeleteDatastoreResponse` | `structure` | `datastoreId`, `datastoreStatus` | - |
| `DeleteImageSetRequest` | `structure` | `datastoreId`, `imageSetId` | - |
| `DeleteImageSetResponse` | `structure` | `datastoreId`, `imageSetId`, `imageSetState`, `imageSetWorkflowStatus` | - |
| `GetDICOMImportJobRequest` | `structure` | `datastoreId`, `jobId` | - |
| `GetDICOMImportJobResponse` | `structure` | `jobProperties` | - |
| `GetDatastoreRequest` | `structure` | `datastoreId` | - |
| `GetDatastoreResponse` | `structure` | `datastoreProperties` | - |
| `GetImageFrameRequest` | `structure` | `datastoreId`, `imageFrameInformation`, `imageSetId` | - |
| `GetImageFrameResponse` | `structure` | `contentType`, `imageFrameBlob` | - |
| `GetImageSetRequest` | `structure` | `datastoreId`, `imageSetId`, `versionId` | - |
| `GetImageSetResponse` | `structure` | `createdAt`, `datastoreId`, `deletedAt`, `imageSetArn`, `imageSetId`, `imageSetState`, `imageSetWorkflowStatus`, `isPrimary`, `lastAccessedAt`, `message`, `overrides`, `storageTier`, ... (+2) | - |
| `GetImageSetMetadataRequest` | `structure` | `datastoreId`, `imageSetId`, `versionId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

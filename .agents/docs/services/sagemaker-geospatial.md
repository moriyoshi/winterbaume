# Amazon SageMaker geospatial capabilities

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provides APIs for creating and managing SageMaker geospatial resources.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon SageMaker geospatial capabilities resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon SageMaker geospatial capabilities workflows in the local mock. Key resources include `EarthObservationJob`, `RasterDataCollection`, `VectorEnrichmentJob`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Delete`, `Export`, `Start` operation families, including `GetEarthObservationJob`, `GetRasterDataCollection`, `GetTile`, `GetVectorEnrichmentJob`, `ListEarthObservationJobs`, `ListRasterDataCollections`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-geospatial`
- AWS SDK for Rust slug: `sagemakergeospatial`
- Model version: `2020-05-27`
- Model file: `vendor/api-models-aws/models/sagemaker-geospatial/service/2020-05-27/sagemaker-geospatial-2020-05-27.json`
- SDK ID: `SageMaker Geospatial`
- Endpoint prefix: `-`
- ARN namespace: `sagemaker-geospatial`
- CloudFormation name: `-`
- CloudTrail event source: `sagemaker-geospatial.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (4), `Delete` (2), `Export` (2), `Start` (2), `Stop` (2), `Search` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteEarthObservationJob`, `DeleteVectorEnrichmentJob`, `StartEarthObservationJob`, `StartVectorEnrichmentJob`, `StopEarthObservationJob`, `StopVectorEnrichmentJob`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ExportVectorEnrichmentJob`, `GetEarthObservationJob`, `GetRasterDataCollection`, `GetTile`, `GetVectorEnrichmentJob`, `ListEarthObservationJobs`, `ListRasterDataCollections`, `ListTagsForResource`, `ListVectorEnrichmentJobs`, `SearchRasterDataCollection`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteEarthObservationJob`, `DeleteVectorEnrichmentJob`, `ExportEarthObservationJob`, `ExportVectorEnrichmentJob`, `GetEarthObservationJob`, `GetVectorEnrichmentJob`, `ListEarthObservationJobs`, `ListVectorEnrichmentJobs`, `StartEarthObservationJob`, `StartVectorEnrichmentJob`, `StopEarthObservationJob`, `StopVectorEnrichmentJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `EarthObservationJob` | `Arn` | create: `StartEarthObservationJob`; read: `GetEarthObservationJob`; delete: `DeleteEarthObservationJob`; list: `ListEarthObservationJobs` | `ExportEarthObservationJob`, `GetTile`, `StopEarthObservationJob` | - |
| `RasterDataCollection` | `Arn` | read: `GetRasterDataCollection`; list: `ListRasterDataCollections` | `SearchRasterDataCollection` | - |
| `VectorEnrichmentJob` | `Arn` | create: `StartVectorEnrichmentJob`; read: `GetVectorEnrichmentJob`; delete: `DeleteVectorEnrichmentJob`; list: `ListVectorEnrichmentJobs` | `ExportVectorEnrichmentJob`, `StopVectorEnrichmentJob` | - |
## Operation Groups

### Get

- Operations: `GetEarthObservationJob`, `GetRasterDataCollection`, `GetTile`, `GetVectorEnrichmentJob`
- Traits: `readonly` (4)
- Common required input members in this group: `Arn`, `ImageAssets`, `Target`, `x`, `y`, `z`

### List

- Operations: `ListEarthObservationJobs`, `ListRasterDataCollections`, `ListTagsForResource`, `ListVectorEnrichmentJobs`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `ResourceArn`

### Delete

- Operations: `DeleteEarthObservationJob`, `DeleteVectorEnrichmentJob`
- Traits: `idempotent` (2)
- Common required input members in this group: `Arn`

### Export

- Operations: `ExportEarthObservationJob`, `ExportVectorEnrichmentJob`
- Traits: `idempotency-token` (2), `readonly` (1)
- Common required input members in this group: `Arn`, `ExecutionRoleArn`, `OutputConfig`

### Start

- Operations: `StartEarthObservationJob`, `StartVectorEnrichmentJob`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `ExecutionRoleArn`, `InputConfig`, `JobConfig`, `Name`

### Stop

- Operations: `StopEarthObservationJob`, `StopVectorEnrichmentJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `Arn`

### Search

- Operations: `SearchRasterDataCollection`
- Traits: `paginated` (1)
- Common required input members in this group: `Arn`, `RasterDataCollectionQuery`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteEarthObservationJob` | `DELETE /earth-observation-jobs/{Arn}` | `idempotent` | `Arn` | - | `DeleteEarthObservationJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to delete an Earth Observation job. |
| `DeleteVectorEnrichmentJob` | `DELETE /vector-enrichment-jobs/{Arn}` | `idempotent` | `Arn` | - | `DeleteVectorEnrichmentJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to delete a Vector Enrichment job. |
| `ExportEarthObservationJob` | `POST /export-earth-observation-job` | `idempotency-token` | `Arn`, `ExecutionRoleArn`, `OutputConfig` | `ClientToken` | `ExportEarthObservationJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Use this operation to export results of an Earth Observation job and optionally source images used as input to the EOJ to an Amazon S3 location. |
| `ExportVectorEnrichmentJob` | `POST /export-vector-enrichment-jobs` | `readonly`, `idempotency-token` | `Arn`, `ExecutionRoleArn`, `OutputConfig` | `ClientToken` | `ExportVectorEnrichmentJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Use this operation to copy results of a Vector Enrichment job to an Amazon S3 location. |
| `GetEarthObservationJob` | `GET /earth-observation-jobs/{Arn}` | `readonly` | `Arn` | - | `GetEarthObservationJobOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the details for a previously initiated Earth Observation job. |
| `GetRasterDataCollection` | `GET /raster-data-collection/{Arn}` | `readonly` | `Arn` | - | `GetRasterDataCollectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to get details of a specific raster data collection. |
| `GetTile` | `GET /tile/{z}/{x}/{y}` | `readonly` | `Arn`, `ImageAssets`, `Target`, `x`, `y`, `z` | - | `GetTileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a web mercator tile for the given Earth Observation job. |
| `GetVectorEnrichmentJob` | `GET /vector-enrichment-jobs/{Arn}` | `readonly` | `Arn` | - | `GetVectorEnrichmentJobOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details of a Vector Enrichment Job for a given job Amazon Resource Name (ARN). |
| `ListEarthObservationJobs` | `POST /list-earth-observation-jobs` | `readonly`, `paginated` | - | - | `ListEarthObservationJobOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to get a list of the Earth Observation jobs associated with the calling Amazon Web Services account. |
| `ListRasterDataCollections` | `GET /raster-data-collections` | `readonly`, `paginated` | - | - | `ListRasterDataCollectionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to get raster data collections. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags attached to the resource. |
| `ListVectorEnrichmentJobs` | `POST /list-vector-enrichment-jobs` | `readonly`, `paginated` | - | - | `ListVectorEnrichmentJobOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of vector enrichment jobs. |
| `SearchRasterDataCollection` | `POST /search-raster-data-collection` | `paginated` | `Arn`, `RasterDataCollectionQuery` | - | `SearchRasterDataCollectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows you run image query on a specific raster data collection to get a list of the satellite imagery matching the selected filters. |
| `StartEarthObservationJob` | `POST /earth-observation-jobs` | `idempotent`, `idempotency-token` | `ExecutionRoleArn`, `InputConfig`, `JobConfig`, `Name` | `ClientToken` | `StartEarthObservationJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Use this operation to create an Earth observation job. |
| `StartVectorEnrichmentJob` | `POST /vector-enrichment-jobs` | `idempotent`, `idempotency-token` | `ExecutionRoleArn`, `InputConfig`, `JobConfig`, `Name` | `ClientToken` | `StartVectorEnrichmentJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a Vector Enrichment job for the supplied job type. Currently, there are two supported job types: reverse geocoding and map matching. |
| `StopEarthObservationJob` | `POST /earth-observation-jobs/stop` | - | `Arn` | - | `StopEarthObservationJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to stop an existing earth observation job. |
| `StopVectorEnrichmentJob` | `POST /vector-enrichment-jobs/stop` | `idempotent` | `Arn` | - | `StopVectorEnrichmentJobOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops the Vector Enrichment job for a given job ARN. |
| `TagResource` | `PUT /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The resource you want to tag. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The resource you want to untag. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message`, `ResourceId` | The request processing has failed because of an unknown error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId` | The request references a resource which does not exist. |
| `ThrottlingException` | `structure` | `Message`, `ResourceId` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message`, `ResourceId` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ConflictException` | `structure` | `Message`, `ResourceId` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `ResourceId` | You have exceeded the service quota. |
| `DeleteEarthObservationJobInput` | `structure` | `Arn` | - |
| `DeleteEarthObservationJobOutput` | `structure` | - | - |
| `DeleteVectorEnrichmentJobInput` | `structure` | `Arn` | - |
| `DeleteVectorEnrichmentJobOutput` | `structure` | - | - |
| `ExportEarthObservationJobInput` | `structure` | `Arn`, `ClientToken`, `ExecutionRoleArn`, `ExportSourceImages`, `OutputConfig` | - |
| `ExportEarthObservationJobOutput` | `structure` | `Arn`, `CreationTime`, `ExecutionRoleArn`, `ExportSourceImages`, `ExportStatus`, `OutputConfig` | - |
| `ExportVectorEnrichmentJobInput` | `structure` | `Arn`, `ClientToken`, `ExecutionRoleArn`, `OutputConfig` | - |
| `ExportVectorEnrichmentJobOutput` | `structure` | `Arn`, `CreationTime`, `ExecutionRoleArn`, `ExportStatus`, `OutputConfig` | - |
| `GetEarthObservationJobInput` | `structure` | `Arn` | - |
| `GetEarthObservationJobOutput` | `structure` | `Arn`, `CreationTime`, `DurationInSeconds`, `ErrorDetails`, `ExecutionRoleArn`, `ExportErrorDetails`, `ExportStatus`, `InputConfig`, `JobConfig`, `KmsKeyId`, `Name`, `OutputBands`, ... (+2) | - |
| `GetRasterDataCollectionInput` | `structure` | `Arn` | - |
| `GetRasterDataCollectionOutput` | `structure` | `Arn`, `Description`, `DescriptionPageUrl`, `ImageSourceBands`, `Name`, `SupportedFilters`, `Tags`, `Type` | - |
| `GetTileInput` | `structure` | `Arn`, `ExecutionRoleArn`, `ImageAssets`, `ImageMask`, `OutputDataType`, `OutputFormat`, `PropertyFilters`, `Target`, `TimeRangeFilter`, `x`, `y`, `z` | - |
| `GetTileOutput` | `structure` | `BinaryFile` | - |
| `GetVectorEnrichmentJobInput` | `structure` | `Arn` | - |
| `GetVectorEnrichmentJobOutput` | `structure` | `Arn`, `CreationTime`, `DurationInSeconds`, `ErrorDetails`, `ExecutionRoleArn`, `ExportErrorDetails`, `ExportStatus`, `InputConfig`, `JobConfig`, `KmsKeyId`, `Name`, `Status`, ... (+2) | - |
| `ListEarthObservationJobInput` | `structure` | `MaxResults`, `NextToken`, `SortBy`, `SortOrder`, `StatusEquals` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

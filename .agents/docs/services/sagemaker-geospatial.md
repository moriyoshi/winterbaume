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
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags attached to the resource. |
| `TagResource` | `PUT /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The resource you want to tag. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The resource you want to untag. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message, ResourceId | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | Message, ResourceId | The request processing has failed because of an unknown error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId | The request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId | You have exceeded the service quota. |
| `ThrottlingException` | `structure` | Message, ResourceId | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message, ResourceId | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

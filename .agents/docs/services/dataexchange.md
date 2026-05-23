# AWS Data Exchange

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Data Exchange is a service that makes it easy for AWS customers to exchange data in the cloud. You can use the AWS Data Exchange APIs to create, update, manage, and access file-based data set in the AWS Cloud. As a subscriber, you can view and access the data sets that you have an entitlement to through a subscription. You can use the APIs to download or copy your entitled data sets to Amazon Simple Storage Service (Amazon S3) for use across a variety of AWS analytics and machine learning services. As a provider, you can create and manage your data sets that you would like to publish to a product.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Data Exchange resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Data Exchange workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListDataGrants`, `ListDataSetRevisions`, `ListDataSets`, `ListEventActions`, `GetAsset`, `GetDataGrant`.

## Service Identity and Protocol

- AWS model slug: `dataexchange`
- AWS SDK for Rust slug: `dataexchange`
- Model version: `2017-07-25`
- Model file: `vendor/api-models-aws/models/dataexchange/service/2017-07-25/dataexchange-2017-07-25.json`
- SDK ID: `DataExchange`
- Endpoint prefix: `dataexchange`
- ARN namespace: `dataexchange`
- CloudFormation name: `-`
- CloudTrail event source: `dataexchange.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (7), `Create` (5), `Delete` (5), `Update` (4), `Send` (2), `Accept` (1), `Cancel` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptDataGrant`, `CancelJob`, `CreateDataGrant`, `CreateDataSet`, `CreateEventAction`, `CreateJob`, `CreateRevision`, `DeleteAsset`, `DeleteDataGrant`, `DeleteDataSet`, `DeleteEventAction`, `DeleteRevision`, `RevokeRevision`, `StartJob`, `TagResource`, `UntagResource`, `UpdateAsset`, `UpdateDataSet`, `UpdateEventAction`, `UpdateRevision`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAsset`, `GetDataGrant`, `GetDataSet`, `GetEventAction`, `GetJob`, `GetReceivedDataGrant`, `GetRevision`, `ListDataGrants`, `ListDataSetRevisions`, `ListDataSets`, `ListEventActions`, `ListJobs`, `ListReceivedDataGrants`, `ListRevisionAssets`, `ListTagsForResource`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelJob`, `CreateJob`, `GetJob`, `ListJobs`, `StartJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 34 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `Redshift`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/data-exchange/latest/userguide/what-is.html
- https://docs.aws.amazon.com/data-exchange/latest/userguide/jobs.html
- https://docs.aws.amazon.com/data-exchange/latest/userguide/product-subscriptions.html

Research outcomes:
- AWS Data Exchange manages data grants, entitlements, Marketplace data product subscriptions, and supported data sets.
- Data sets contain revisions, and revisions contain assets.
- Jobs are asynchronous import/export operations for assets and have states such as waiting, in progress, completed, cancelled, or failed.
- File-based subscriptions can export revisions to S3 and can configure auto-export.
- Data products can include data dictionaries and samples.
- Data sets can represent file-based data, API products, Amazon Redshift data shares, Lake Formation data, and other supported types.

Parity implications:
- Model data sets, revisions, assets, jobs, grants, entitlements, subscriptions, product metadata, and export destinations separately.
- Import/export operations should create jobs and require status polling.
- Subscription entitlement should control access to revisions and assets without copying provider-owned metadata unnecessarily.

## Operation Groups

### List

- Operations: `ListDataGrants`, `ListDataSetRevisions`, `ListDataSets`, `ListEventActions`, `ListJobs`, `ListReceivedDataGrants`, `ListRevisionAssets`, `ListTagsForResource`
- Traits: `paginated` (7), `readonly` (8)
- Common required input members in this group: `DataSetId`, `ResourceArn`, `RevisionId`

### Get

- Operations: `GetAsset`, `GetDataGrant`, `GetDataSet`, `GetEventAction`, `GetJob`, `GetReceivedDataGrant`, `GetRevision`
- Traits: `readonly` (7)
- Common required input members in this group: `AssetId`, `DataGrantArn`, `DataGrantId`, `DataSetId`, `EventActionId`, `JobId`, `RevisionId`

### Create

- Operations: `CreateDataGrant`, `CreateDataSet`, `CreateEventAction`, `CreateJob`, `CreateRevision`
- Common required input members in this group: `Action`, `AssetType`, `DataSetId`, `Description`, `Details`, `Event`, `GrantDistributionScope`, `Name`, `ReceiverPrincipal`, `SourceDataSetId`, `Type`

### Delete

- Operations: `DeleteAsset`, `DeleteDataGrant`, `DeleteDataSet`, `DeleteEventAction`, `DeleteRevision`
- Common required input members in this group: `AssetId`, `DataGrantId`, `DataSetId`, `EventActionId`, `RevisionId`

### Update

- Operations: `UpdateAsset`, `UpdateDataSet`, `UpdateEventAction`, `UpdateRevision`
- Common required input members in this group: `AssetId`, `DataSetId`, `EventActionId`, `Name`, `RevisionId`

### Send

- Operations: `SendApiAsset`, `SendDataSetNotification`
- Traits: `endpoint-bound` (1), `idempotency-token` (1)
- Common required input members in this group: `AssetId`, `DataSetId`, `RevisionId`, `Type`

### Accept

- Operations: `AcceptDataGrant`
- Common required input members in this group: `DataGrantArn`

### Cancel

- Operations: `CancelJob`
- Common required input members in this group: `JobId`

### Revoke

- Operations: `RevokeRevision`
- Common required input members in this group: `DataSetId`, `RevisionId`, `RevocationComment`

### Start

- Operations: `StartJob`
- Common required input members in this group: `JobId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptDataGrant` | `POST /v1/data-grants/{DataGrantArn}/accept` | - | `DataGrantArn` | - | `AcceptDataGrantResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation accepts a data grant. |
| `CancelJob` | `DELETE /v1/jobs/{JobId}` | - | `JobId` | - | `Unit` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation cancels a job. Jobs can be cancelled only when they are in the WAITING state. |
| `CreateDataGrant` | `POST /v1/data-grants` | - | `GrantDistributionScope`, `Name`, `ReceiverPrincipal`, `SourceDataSetId` | - | `CreateDataGrantResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | This operation creates a data grant. |
| `CreateDataSet` | `POST /v1/data-sets` | - | `AssetType`, `Description`, `Name` | - | `CreateDataSetResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | This operation creates a data set. |
| `CreateEventAction` | `POST /v1/event-actions` | - | `Action`, `Event` | - | `CreateEventActionResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | This operation creates an event action. |
| `CreateJob` | `POST /v1/jobs` | - | `Details`, `Type` | - | `CreateJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation creates a job. |
| `CreateRevision` | `POST /v1/data-sets/{DataSetId}/revisions` | - | `DataSetId` | - | `CreateRevisionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation creates a revision for a data set. |
| `DeleteAsset` | `DELETE /v1/data-sets/{DataSetId}/revisions/{RevisionId}/assets/{AssetId}` | - | `AssetId`, `DataSetId`, `RevisionId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation deletes an asset. |
| `DeleteDataGrant` | `DELETE /v1/data-grants/{DataGrantId}` | - | `DataGrantId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation deletes a data grant. |
| `DeleteDataSet` | `DELETE /v1/data-sets/{DataSetId}` | - | `DataSetId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation deletes a data set. |
| `DeleteEventAction` | `DELETE /v1/event-actions/{EventActionId}` | - | `EventActionId` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation deletes the event action. |
| `DeleteRevision` | `DELETE /v1/data-sets/{DataSetId}/revisions/{RevisionId}` | - | `DataSetId`, `RevisionId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation deletes a revision. |
| `GetAsset` | `GET /v1/data-sets/{DataSetId}/revisions/{RevisionId}/assets/{AssetId}` | `readonly` | `AssetId`, `DataSetId`, `RevisionId` | - | `GetAssetResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about an asset. |
| `GetDataGrant` | `GET /v1/data-grants/{DataGrantId}` | `readonly` | `DataGrantId` | - | `GetDataGrantResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about a data grant. |
| `GetDataSet` | `GET /v1/data-sets/{DataSetId}` | `readonly` | `DataSetId` | - | `GetDataSetResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about a data set. |
| `GetEventAction` | `GET /v1/event-actions/{EventActionId}` | `readonly` | `EventActionId` | - | `GetEventActionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation retrieves information about an event action. |
| `GetJob` | `GET /v1/jobs/{JobId}` | `readonly` | `JobId` | - | `GetJobResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about a job. |
| `GetReceivedDataGrant` | `GET /v1/received-data-grants/{DataGrantArn}` | `readonly` | `DataGrantArn` | - | `GetReceivedDataGrantResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about a received data grant. |
| `GetRevision` | `GET /v1/data-sets/{DataSetId}/revisions/{RevisionId}` | `readonly` | `DataSetId`, `RevisionId` | - | `GetRevisionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about a revision. |
| `ListDataGrants` | `GET /v1/data-grants` | `readonly`, `paginated` | - | - | `ListDataGrantsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about all data grants. |
| `ListDataSetRevisions` | `GET /v1/data-sets/{DataSetId}/revisions` | `readonly`, `paginated` | `DataSetId` | - | `ListDataSetRevisionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation lists a data set's revisions sorted by CreatedAt in descending order. |
| `ListDataSets` | `GET /v1/data-sets` | `readonly`, `paginated` | - | - | `ListDataSetsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation lists your data sets. When listing by origin OWNED, results are sorted by CreatedAt in descending order. |
| `ListEventActions` | `GET /v1/event-actions` | `readonly`, `paginated` | - | - | `ListEventActionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation lists your event actions. |
| `ListJobs` | `GET /v1/jobs` | `readonly`, `paginated` | - | - | `ListJobsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation lists your jobs sorted by CreatedAt in descending order. |
| `ListReceivedDataGrants` | `GET /v1/received-data-grants` | `readonly`, `paginated` | - | - | `ListReceivedDataGrantsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation returns information about all received data grants. |
| `ListRevisionAssets` | `GET /v1/data-sets/{DataSetId}/revisions/{RevisionId}/assets` | `readonly`, `paginated` | `DataSetId`, `RevisionId` | - | `ListRevisionAssetsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation lists a revision's assets sorted alphabetically in descending order. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | - | This operation lists the tags on the resource. |
| `RevokeRevision` | `POST /v1/data-sets/{DataSetId}/revisions/{RevisionId}/revoke` | - | `DataSetId`, `RevisionId`, `RevocationComment` | - | `RevokeRevisionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation revokes subscribers' access to a revision. |
| `SendApiAsset` | `POST /v1` | `endpoint-bound` | `AssetId`, `DataSetId`, `RevisionId` | - | `SendApiAssetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation invokes an API Gateway API asset. The request is proxied to the provider’s API Gateway API. |
| `SendDataSetNotification` | `POST /v1/data-sets/{DataSetId}/notification` | `idempotency-token` | `DataSetId`, `Type` | `ClientToken` | `SendDataSetNotificationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The type of event associated with the data set. |
| `StartJob` | `PATCH /v1/jobs/{JobId}` | - | `JobId` | - | `StartJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation starts a job. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | - | This operation tags a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | - | This operation removes one or more tags from a resource. |
| `UpdateAsset` | `PATCH /v1/data-sets/{DataSetId}/revisions/{RevisionId}/assets/{AssetId}` | - | `AssetId`, `DataSetId`, `Name`, `RevisionId` | - | `UpdateAssetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation updates an asset. |
| `UpdateDataSet` | `PATCH /v1/data-sets/{DataSetId}` | - | `DataSetId` | - | `UpdateDataSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation updates a data set. |
| `UpdateEventAction` | `PATCH /v1/event-actions/{EventActionId}` | - | `EventActionId` | - | `UpdateEventActionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation updates the event action. |
| `UpdateRevision` | `PATCH /v1/data-sets/{DataSetId}/revisions/{RevisionId}` | - | `DataSetId`, `RevisionId` | - | `UpdateRevisionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation updates a revision. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListDataGrants` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListDataSetRevisions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListDataSets` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `Origin -> origin` | - | - |
| `ListEventActions` | - | `EventSourceId -> eventSourceId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListJobs` | - | `DataSetId -> dataSetId`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `RevisionId -> revisionId` | - | - |
| `ListReceivedDataGrants` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `AcceptanceState -> acceptanceState` | - | - |
| `ListRevisionAssets` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `SendApiAsset` | `AssetId -> x-amzn-dataexchange-asset-id`, `DataSetId -> x-amzn-dataexchange-data-set-id`, `Method -> x-amzn-dataexchange-http-method`, `Path -> x-amzn-dataexchange-path`, `RevisionId -> x-amzn-dataexchange-revision-id` | - | `RequestHeaders -> x-amzn-dataexchange-header-*` | `Body` |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An exception occurred with the service. |
| `ThrottlingException` | `structure` | `Message` | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | `ExceptionCause`, `Message` | The request was invalid. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The resource couldn't be found. |
| `AccessDeniedException` | `structure` | `Message` | Access to the resource is denied. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The request couldn't be completed because it conflicted with the current state of the resource. |
| `ServiceLimitExceededException` | `structure` | `LimitName`, `LimitValue`, `Message` | The request has exceeded the quotas imposed by the service. |
| `AcceptDataGrantRequest` | `structure` | `DataGrantArn` | - |
| `AcceptDataGrantResponse` | `structure` | `AcceptanceState`, `AcceptedAt`, `Arn`, `CreatedAt`, `DataSetId`, `Description`, `EndsAt`, `GrantDistributionScope`, `Id`, `Name`, `ReceiverPrincipal`, `SenderPrincipal`, ... (+1) | - |
| `CancelJobRequest` | `structure` | `JobId` | - |
| `CreateDataGrantRequest` | `structure` | `Description`, `EndsAt`, `GrantDistributionScope`, `Name`, `ReceiverPrincipal`, `SourceDataSetId`, `Tags` | - |
| `CreateDataGrantResponse` | `structure` | `AcceptanceState`, `AcceptedAt`, `Arn`, `CreatedAt`, `DataSetId`, `Description`, `EndsAt`, `GrantDistributionScope`, `Id`, `Name`, `ReceiverPrincipal`, `SenderPrincipal`, ... (+3) | - |
| `CreateDataSetRequest` | `structure` | `AssetType`, `Description`, `Name`, `Tags` | - |
| `CreateDataSetResponse` | `structure` | `Arn`, `AssetType`, `CreatedAt`, `Description`, `Id`, `Name`, `Origin`, `OriginDetails`, `SourceId`, `Tags`, `UpdatedAt` | - |
| `CreateEventActionRequest` | `structure` | `Action`, `Event`, `Tags` | - |
| `CreateEventActionResponse` | `structure` | `Action`, `Arn`, `CreatedAt`, `Event`, `Id`, `Tags`, `UpdatedAt` | - |
| `CreateJobRequest` | `structure` | `Details`, `Type` | - |
| `CreateJobResponse` | `structure` | `Arn`, `CreatedAt`, `Details`, `Errors`, `Id`, `State`, `Type`, `UpdatedAt` | - |
| `CreateRevisionRequest` | `structure` | `Comment`, `DataSetId`, `Tags` | - |
| `CreateRevisionResponse` | `structure` | `Arn`, `Comment`, `CreatedAt`, `DataSetId`, `Finalized`, `Id`, `RevocationComment`, `Revoked`, `RevokedAt`, `SourceId`, `Tags`, `UpdatedAt` | - |
| `DeleteAssetRequest` | `structure` | `AssetId`, `DataSetId`, `RevisionId` | - |
| `DeleteDataGrantRequest` | `structure` | `DataGrantId` | - |
| `DeleteDataSetRequest` | `structure` | `DataSetId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

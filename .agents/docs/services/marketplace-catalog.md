# AWS Marketplace Catalog Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Catalog API actions allow you to manage your entities through list, describe, and update capabilities. An entity can be a product or an offer on AWS Marketplace. You can automate your entity update process by integrating the AWS Marketplace Catalog API with your AWS Marketplace product build or deployment pipelines. You can also create your own applications on top of the Catalog API to manage your products on AWS Marketplace.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Marketplace Catalog Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Describe`, `Batch`, `Cancel`, `Delete` operation families, including `ListChangeSets`, `ListEntities`, `ListTagsForResource`, `DescribeChangeSet`, `DescribeEntity`, `BatchDescribeEntities`.

## Service Identity and Protocol

- AWS model slug: `marketplace-catalog`
- AWS SDK for Rust slug: `marketplacecatalog`
- Model version: `2018-09-17`
- Model file: `vendor/api-models-aws/models/marketplace-catalog/service/2018-09-17/marketplace-catalog-2018-09-17.json`
- SDK ID: `Marketplace Catalog`
- Endpoint prefix: `catalog.marketplace`
- ARN namespace: `aws-marketplace`
- CloudFormation name: `MarketplaceCatalog`
- CloudTrail event source: `marketplacecatalog.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (3), `Describe` (2), `Batch` (1), `Cancel` (1), `Delete` (1), `Get` (1), `Put` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDescribeEntities`, `CancelChangeSet`, `DeleteResourcePolicy`, `PutResourcePolicy`, `StartChangeSet`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeChangeSet`, `DescribeEntity`, `GetResourcePolicy`, `ListChangeSets`, `ListEntities`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelChangeSet`, `StartChangeSet`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Operation Groups

### List

- Operations: `ListChangeSets`, `ListEntities`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: `Catalog`

### Describe

- Operations: `DescribeChangeSet`, `DescribeEntity`
- Common required input members in this group: `Catalog`

### Batch

- Operations: `BatchDescribeEntities`
- Common required input members in this group: -

### Cancel

- Operations: `CancelChangeSet`
- Common required input members in this group: -

### Delete

- Operations: `DeleteResourcePolicy`
- Common required input members in this group: -

### Get

- Operations: `GetResourcePolicy`
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: -

### Start

- Operations: `StartChangeSet`
- Traits: `idempotency-token` (1)
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
| `BatchDescribeEntities` | `POST /BatchDescribeEntities` | - | `EntityRequestList` | - | `BatchDescribeEntitiesResponse` | `AccessDeniedException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Returns metadata and content for multiple entities. This is the Batch version of the DescribeEntity API and uses the same IAM permission action as DescribeEntity API. |
| `CancelChangeSet` | `PATCH /CancelChangeSet` | - | `Catalog`, `ChangeSetId` | - | `CancelChangeSetResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Used to cancel an open change request. Must be sent before the status of the request changes to APPLYING , the final stage of completing your change request. You can describe a change during the 60-day request histor ... |
| `DeleteResourcePolicy` | `DELETE /DeleteResourcePolicy` | - | `ResourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a resource-based policy on an entity that is identified by its resource ARN. |
| `DescribeChangeSet` | `GET /DescribeChangeSet` | - | `Catalog`, `ChangeSetId` | - | `DescribeChangeSetResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides information about a given change set. |
| `DescribeEntity` | `GET /DescribeEntity` | - | `Catalog`, `EntityId` | - | `DescribeEntityResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ResourceNotSupportedException`, `ThrottlingException`, `ValidationException` | Returns the metadata and content of the entity. |
| `GetResourcePolicy` | `GET /GetResourcePolicy` | - | `ResourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a resource-based policy of an entity that is identified by its resource ARN. |
| `ListChangeSets` | `POST /ListChangeSets` | `paginated` | `Catalog` | - | `ListChangeSetsResponse` | `AccessDeniedException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Returns the list of change sets owned by the account being used to make the call. You can filter this list by providing any combination of entityId , ChangeSetName , and status. If you provide more than one filter, t ... |
| `ListEntities` | `POST /ListEntities` | `paginated` | `Catalog`, `EntityType` | - | `ListEntitiesResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides the list of entities of a given type. |
| `ListTagsForResource` | `POST /ListTagsForResource` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags that have been added to a resource (either an entity or change set ). |
| `PutResourcePolicy` | `POST /PutResourcePolicy` | - | `ResourceArn`, `Policy` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a resource-based policy to an entity. Examples of an entity include: AmiProduct and ContainerProduct . |
| `StartChangeSet` | `POST /StartChangeSet` | `idempotency-token` | `Catalog`, `ChangeSet` | `ClientRequestToken` | `StartChangeSetResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Allows you to request changes for your entities. Within a single ChangeSet , you can't start the same change type against the same entity multiple times. Additionally, when a ChangeSet is running, all the entities ta ... |
| `TagResource` | `POST /TagResource` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags a resource (either an entity or change set ). |
| `UntagResource` | `POST /UntagResource` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or list of tags from a resource (either an entity or change set ). |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CancelChangeSet` | - | `Catalog -> catalog`, `ChangeSetId -> changeSetId` | - | - |
| `DeleteResourcePolicy` | - | `ResourceArn -> resourceArn` | - | - |
| `DescribeChangeSet` | - | `Catalog -> catalog`, `ChangeSetId -> changeSetId` | - | - |
| `DescribeEntity` | - | `Catalog -> catalog`, `EntityId -> entityId` | - | - |
| `GetResourcePolicy` | - | `ResourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Access is denied. HTTP status code: 403 |
| `InternalServiceException` | `structure` | Message | There was an internal service exception. HTTP status code: 500 |
| `ResourceInUseException` | `structure` | Message | The resource is currently in use. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource wasn't found. HTTP status code: 404 |
| `ResourceNotSupportedException` | `structure` | Message | Currently, the specified resource is not supported. |
| `ServiceQuotaExceededException` | `structure` | Message | The maximum number of open requests per account has been exceeded. |
| `ThrottlingException` | `structure` | Message | Too many requests. HTTP status code: 429 |
| `ValidationException` | `structure` | Message | An error occurred during validation. HTTP status code: 422 |
| `BatchDescribeEntitiesRequest` | `structure` | EntityRequestList | - |
| `BatchDescribeEntitiesResponse` | `structure` | EntityDetails, Errors | - |
| `CancelChangeSetRequest` | `structure` | Catalog, ChangeSetId | - |
| `CancelChangeSetResponse` | `structure` | ChangeSetId, ChangeSetArn | - |
| `DeleteResourcePolicyRequest` | `structure` | ResourceArn | - |
| `DeleteResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `DescribeChangeSetRequest` | `structure` | Catalog, ChangeSetId | - |
| `DescribeChangeSetResponse` | `structure` | ChangeSetId, ChangeSetArn, ChangeSetName, Intent, StartTime, EndTime, Status, FailureCode, FailureDescription, ChangeSet | - |
| `DescribeEntityRequest` | `structure` | Catalog, EntityId | - |
| `DescribeEntityResponse` | `structure` | EntityType, EntityIdentifier, EntityArn, LastModifiedDate, Details, DetailsDocument | - |
| `GetResourcePolicyRequest` | `structure` | ResourceArn | - |
| `GetResourcePolicyResponse` | `structure` | Policy | - |
| `ListChangeSetsRequest` | `structure` | Catalog, FilterList, Sort, MaxResults, NextToken | - |
| `ListChangeSetsResponse` | `structure` | ChangeSetSummaryList, NextToken | - |
| `ListEntitiesRequest` | `structure` | Catalog, EntityType, FilterList, Sort, NextToken, MaxResults, OwnershipType, EntityTypeFilters, EntityTypeSort | - |
| `ListEntitiesResponse` | `structure` | EntitySummaryList, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | ResourceArn, Tags | - |
| `PutResourcePolicyRequest` | `structure` | ResourceArn, Policy | - |
| `PutResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `StartChangeSetRequest` | `structure` | Catalog, ChangeSet, ChangeSetName, ClientRequestToken, ChangeSetTags, Intent | - |
| `StartChangeSetResponse` | `structure` | ChangeSetId, ChangeSetArn | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AmiProductSortBy` | `enum` | EntityId, LastModifiedDate, ProductTitle, Visibility | - |
| `AmiProductVisibilityString` | `enum` | Limited, Public, Restricted, Draft | - |
| `ChangeStatus` | `enum` | PREPARING, APPLYING, SUCCEEDED, CANCELLED, FAILED | - |
| `ContainerProductSortBy` | `enum` | EntityId, LastModifiedDate, ProductTitle, Visibility, CompatibleAWSServices | - |
| `ContainerProductVisibilityString` | `enum` | Limited, Public, Restricted, Draft | - |
| `DataProductSortBy` | `enum` | EntityId, ProductTitle, Visibility, LastModifiedDate | - |
| `DataProductVisibilityString` | `enum` | Limited, Public, Restricted, Unavailable, Draft | - |
| `FailureCode` | `enum` | ClientError, ServerFault | - |
| `Intent` | `enum` | VALIDATE, APPLY | - |
| `MachineLearningProductSortBy` | `enum` | EntityId, LastModifiedDate, ProductTitle, Visibility | The fields that you can sort machine learning products by. |
| `MachineLearningProductVisibilityString` | `enum` | Limited, Public, Restricted, Draft | The visibility status of a machine learning product. Valid values are: Limited - The product is available to a limited set of buyers. Public - The product i ... |
| `OfferSetSortBy` | `enum` | Name, State, ReleaseDate, SolutionId, EntityId, LastModifiedDate | - |
| `OfferSetStateString` | `enum` | Draft, Released | - |
| `OfferSortBy` | `enum` | EntityId, Name, ProductId, ResaleAuthorizationId, ReleaseDate, AvailabilityEndDate, BuyerAccounts, State, Targeting, LastModifiedDate, OfferSetId | - |
| `OfferStateString` | `enum` | Draft, Released | - |
| `OfferTargetingString` | `enum` | BuyerAccounts, ParticipatingPrograms, CountryCodes, None | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

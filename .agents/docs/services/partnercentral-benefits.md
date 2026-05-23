# Partner Central Benefits API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Partner Central Benefits Service provides APIs for managing partner benefits, applications, and allocations within the AWS Partner Network ecosystem.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Partner Central Benefits API where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Partner Central Benefits API by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Partner Central Benefits API resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Partner Central Benefits API workflows in the local mock. Key resources include `Benefit`, `BenefitAllocation`, `BenefitApplication`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Amend`, `Associate`, `Cancel` operation families, including `ListBenefitAllocations`, `ListBenefitApplications`, `ListBenefits`, `ListTagsForResource`, `GetBenefit`, `GetBenefitAllocation`.

## Service Identity and Protocol

- AWS model slug: `partnercentral-benefits`
- AWS SDK for Rust slug: `partnercentralbenefits`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/partnercentral-benefits/service/2018-05-10/partnercentral-benefits-2018-05-10.json`
- SDK ID: `PartnerCentral Benefits`
- Endpoint prefix: `partnercentral-benefits`
- ARN namespace: `partnercentral`
- CloudFormation name: `-`
- CloudTrail event source: `partnercentral-benefits.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Amend` (1), `Associate` (1), `Cancel` (1), `Create` (1), `Disassociate` (1), `Recall` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateBenefitApplicationResource`, `CancelBenefitApplication`, `CreateBenefitApplication`, `DisassociateBenefitApplicationResource`, `SubmitBenefitApplication`, `TagResource`, `UntagResource`, `UpdateBenefitApplication`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBenefit`, `GetBenefitAllocation`, `GetBenefitApplication`, `ListBenefitAllocations`, `ListBenefitApplications`, `ListBenefits`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelBenefitApplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Benefit` | `BenefitId`, `CatalogName` | - | - | - |
| `BenefitAllocation` | `BenefitAllocationId`, `CatalogName` | - | - | - |
| `BenefitApplication` | `BenefitApplicationId`, `CatalogName` | - | - | - |
## Operation Groups

### List

- Operations: `ListBenefitAllocations`, `ListBenefitApplications`, `ListBenefits`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (1)
- Common required input members in this group: `Catalog`

### Get

- Operations: `GetBenefit`, `GetBenefitAllocation`, `GetBenefitApplication`
- Common required input members in this group: `Catalog`, `Identifier`

### Amend

- Operations: `AmendBenefitApplication`
- Common required input members in this group: -

### Associate

- Operations: `AssociateBenefitApplicationResource`
- Common required input members in this group: -

### Cancel

- Operations: `CancelBenefitApplication`
- Common required input members in this group: -

### Create

- Operations: `CreateBenefitApplication`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateBenefitApplicationResource`
- Common required input members in this group: -

### Recall

- Operations: `RecallBenefitApplication`
- Common required input members in this group: -

### Submit

- Operations: `SubmitBenefitApplication`
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

- Operations: `UpdateBenefitApplication`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AmendBenefitApplication` | `POST /AmendBenefitApplication` | - | `Catalog`, `ClientToken`, `Revision`, `Identifier`, `AmendmentReason`, `Amendments` | - | `AmendBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing benefit application by applying amendments to specific fields while maintaining revision control. |
| `AssociateBenefitApplicationResource` | `POST /AssociateBenefitApplicationResource` | - | `Catalog`, `BenefitApplicationIdentifier`, `ResourceArn` | - | `AssociateBenefitApplicationResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Links an AWS resource to an existing benefit application for tracking and management purposes. |
| `CancelBenefitApplication` | `POST /CancelBenefitApplication` | - | `Catalog`, `ClientToken`, `Identifier` | - | `CancelBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a benefit application that is currently in progress, preventing further processing. |
| `CreateBenefitApplication` | `POST /CreateBenefitApplication` | `idempotent` | `Catalog`, `ClientToken`, `BenefitIdentifier` | - | `CreateBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new benefit application for a partner to request access to AWS benefits and programs. |
| `DisassociateBenefitApplicationResource` | `POST /DisassociateBenefitApplicationResource` | - | `Catalog`, `BenefitApplicationIdentifier`, `ResourceArn` | - | `DisassociateBenefitApplicationResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between an AWS resource and a benefit application. |
| `GetBenefit` | `POST /GetBenefit` | - | `Catalog`, `Identifier` | - | `GetBenefitOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific benefit available in the partner catalog. |
| `GetBenefitAllocation` | `POST /GetBenefitAllocation` | - | `Catalog`, `Identifier` | - | `GetBenefitAllocationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific benefit allocation that has been granted to a partner. |
| `GetBenefitApplication` | `POST /GetBenefitApplication` | - | `Catalog`, `Identifier` | - | `GetBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific benefit application. |
| `ListBenefitAllocations` | `POST /ListBenefitAllocations` | `paginated` | `Catalog` | - | `ListBenefitAllocationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of benefit allocations based on specified filter criteria. |
| `ListBenefitApplications` | `POST /ListBenefitApplications` | `paginated` | `Catalog` | - | `ListBenefitApplicationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of benefit applications based on specified filter criteria. |
| `ListBenefits` | `POST /ListBenefits` | `paginated` | `Catalog` | - | `ListBenefitsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of available benefits based on specified filter criteria. |
| `ListTagsForResource` | `POST /ListTagsForResource` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves all tags associated with a specific resource. |
| `RecallBenefitApplication` | `POST /RecallBenefitApplication` | - | `Catalog`, `Identifier`, `Reason` | - | `RecallBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Recalls a submitted benefit application, returning it to draft status for further modifications. |
| `SubmitBenefitApplication` | `POST /SubmitBenefitApplication` | - | `Catalog`, `Identifier` | - | `SubmitBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Submits a benefit application for review and processing by AWS. |
| `TagResource` | `POST /TagResource` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified resource. |
| `UntagResource` | `POST /UntagResource` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes specified tags from a resource. |
| `UpdateBenefitApplication` | `POST /UpdateBenefitApplication` | - | `Catalog`, `ClientToken`, `Identifier`, `Revision` | - | `UpdateBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing benefit application with new information while maintaining revision control. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Thrown when the caller does not have sufficient permissions to perform the requested operation. |
| `ConflictException` | `structure` | Message | Thrown when the request conflicts with the current state of the resource, such as attempting to modify a resource that has been changed by another process. |
| `InternalServerException` | `structure` | Message | Thrown when an unexpected error occurs on the server side during request processing. |
| `ResourceNotFoundException` | `structure` | Message | Thrown when the requested resource cannot be found or does not exist. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, QuotaCode | Thrown when the request would exceed the service quotas or limits for the account. |
| `ThrottlingException` | `structure` | Message | Thrown when the request rate exceeds the allowed limits and the request is being throttled. |
| `ValidationException` | `structure` | Message, Reason, FieldList | Thrown when the request contains invalid parameters or fails input validation requirements. |
| `AmendBenefitApplicationInput` | `structure` | Catalog, ClientToken, Revision, Identifier, AmendmentReason, Amendments | - |
| `AmendBenefitApplicationOutput` | `structure` | **empty (no members)** | - |
| `AssociateBenefitApplicationResourceInput` | `structure` | Catalog, BenefitApplicationIdentifier, ResourceArn | - |
| `AssociateBenefitApplicationResourceOutput` | `structure` | Id, Arn, Revision | - |
| `CancelBenefitApplicationInput` | `structure` | Catalog, ClientToken, Identifier, Reason | - |
| `CancelBenefitApplicationOutput` | `structure` | **empty (no members)** | - |
| `CreateBenefitApplicationInput` | `structure` | Catalog, ClientToken, Name, Description, BenefitIdentifier, FulfillmentTypes, BenefitApplicationDetails, Tags, AssociatedResources, PartnerContacts, FileDetails | - |
| `CreateBenefitApplicationOutput` | `structure` | Id, Arn, Revision | - |
| `DisassociateBenefitApplicationResourceInput` | `structure` | Catalog, BenefitApplicationIdentifier, ResourceArn | - |
| `DisassociateBenefitApplicationResourceOutput` | `structure` | Id, Arn, Revision | - |
| `GetBenefitInput` | `structure` | Catalog, Identifier | - |
| `GetBenefitOutput` | `structure` | Id, Catalog, Arn, Name, Description, Programs, FulfillmentTypes, BenefitRequestSchema, Status | - |
| `GetBenefitAllocationInput` | `structure` | Catalog, Identifier | - |
| `GetBenefitAllocationOutput` | `structure` | Id, Catalog, Arn, Name, Description, Status, StatusReason, BenefitApplicationId, BenefitId, FulfillmentType, ApplicableBenefitIds, FulfillmentDetail, ... (+4) | - |
| `GetBenefitApplicationInput` | `structure` | Catalog, Identifier | - |
| `GetBenefitApplicationOutput` | `structure` | Id, Arn, Catalog, BenefitId, Name, Description, FulfillmentTypes, BenefitApplicationDetails, Programs, Status, Stage, StatusReason, ... (+8) | - |
| `ListBenefitAllocationsInput` | `structure` | Catalog, FulfillmentTypes, BenefitIdentifiers, BenefitApplicationIdentifiers, Status, MaxResults, NextToken | - |
| `ListBenefitAllocationsOutput` | `structure` | BenefitAllocationSummaries, NextToken | - |
| `ListBenefitApplicationsInput` | `structure` | Catalog, Programs, FulfillmentTypes, BenefitIdentifiers, Status, Stages, AssociatedResources, AssociatedResourceArns, MaxResults, NextToken | - |
| `ListBenefitApplicationsOutput` | `structure` | BenefitApplicationSummaries, NextToken | - |
| `ListBenefitsInput` | `structure` | Catalog, Programs, FulfillmentTypes, Status, MaxResults, NextToken | - |
| `ListBenefitsOutput` | `structure` | BenefitSummaries, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `RecallBenefitApplicationInput` | `structure` | Catalog, ClientToken, Identifier, Reason | - |
| `RecallBenefitApplicationOutput` | `structure` | **empty (no members)** | - |
| `SubmitBenefitApplicationInput` | `structure` | Catalog, Identifier | - |
| `SubmitBenefitApplicationOutput` | `structure` | **empty (no members)** | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateBenefitApplicationInput` | `structure` | Catalog, ClientToken, Name, Description, Identifier, Revision, BenefitApplicationDetails, PartnerContacts, FileDetails | - |
| `BenefitAllocationStatus` | `enum` | ACTIVE, INACTIVE, FULFILLED | - |
| `BenefitApplicationStatus` | `enum` | PENDING_SUBMISSION, IN_REVIEW, ACTION_REQUIRED, APPROVED, REJECTED, CANCELED | - |
| `BenefitStatus` | `enum` | ACTIVE, INACTIVE | - |
| `CurrencyCode` | `enum` | AED, AMD, ARS, AUD, AWG, AZN, BBD, BDT, BGN, BMD, BND, BOB, ... (+83) | - |
| `FileType` | `enum` | DOC, DOCX, XLSX, PPTX, PDF, PNG, JPG, SVG, CSV | - |
| `FulfillmentType` | `enum` | CREDITS, CASH, ACCESS | - |
| `ResourceType` | `enum` | OPPORTUNITY, BENEFIT_ALLOCATION | - |
| `ValidationExceptionErrorCode` | `enum` | REQUIRED_FIELD_MISSING, INVALID_ENUM_VALUE, INVALID_STRING_FORMAT, INVALID_VALUE, NOT_ENOUGH_VALUES, TOO_MANY_VALUES, INVALID_RESOURCE_STATE, DUPLICATE_KEY_VALUE, VALUE_OUT_OF_RANGE, ACTION_NOT_PERMITTED | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER, BUSINESS_VALIDATION_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

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
- Common required input members in this group: `Catalog`, `resourceArn`

### Get

- Operations: `GetBenefit`, `GetBenefitAllocation`, `GetBenefitApplication`
- Common required input members in this group: `Catalog`, `Identifier`

### Amend

- Operations: `AmendBenefitApplication`
- Common required input members in this group: `AmendmentReason`, `Amendments`, `Catalog`, `ClientToken`, `Identifier`, `Revision`

### Associate

- Operations: `AssociateBenefitApplicationResource`
- Common required input members in this group: `BenefitApplicationIdentifier`, `Catalog`, `ResourceArn`

### Cancel

- Operations: `CancelBenefitApplication`
- Common required input members in this group: `Catalog`, `ClientToken`, `Identifier`

### Create

- Operations: `CreateBenefitApplication`
- Traits: `idempotent` (1)
- Common required input members in this group: `BenefitIdentifier`, `Catalog`, `ClientToken`

### Disassociate

- Operations: `DisassociateBenefitApplicationResource`
- Common required input members in this group: `BenefitApplicationIdentifier`, `Catalog`, `ResourceArn`

### Recall

- Operations: `RecallBenefitApplication`
- Common required input members in this group: `Catalog`, `Identifier`, `Reason`

### Submit

- Operations: `SubmitBenefitApplication`
- Common required input members in this group: `Catalog`, `Identifier`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateBenefitApplication`
- Common required input members in this group: `Catalog`, `ClientToken`, `Identifier`, `Revision`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AmendBenefitApplication` | `POST /AmendBenefitApplication` | - | `AmendmentReason`, `Amendments`, `Catalog`, `ClientToken`, `Identifier`, `Revision` | - | `AmendBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing benefit application by applying amendments to specific fields while maintaining revision control. |
| `AssociateBenefitApplicationResource` | `POST /AssociateBenefitApplicationResource` | - | `BenefitApplicationIdentifier`, `Catalog`, `ResourceArn` | - | `AssociateBenefitApplicationResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Links an AWS resource to an existing benefit application for tracking and management purposes. |
| `CancelBenefitApplication` | `POST /CancelBenefitApplication` | - | `Catalog`, `ClientToken`, `Identifier` | - | `CancelBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a benefit application that is currently in progress, preventing further processing. |
| `CreateBenefitApplication` | `POST /CreateBenefitApplication` | `idempotent` | `BenefitIdentifier`, `Catalog`, `ClientToken` | - | `CreateBenefitApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new benefit application for a partner to request access to AWS benefits and programs. |
| `DisassociateBenefitApplicationResource` | `POST /DisassociateBenefitApplicationResource` | - | `BenefitApplicationIdentifier`, `Catalog`, `ResourceArn` | - | `DisassociateBenefitApplicationResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between an AWS resource and a benefit application. |
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

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | Thrown when the caller does not have sufficient permissions to perform the requested operation. |
| `InternalServerException` | `structure` | `Message` | Thrown when an unexpected error occurs on the server side during request processing. |
| `ResourceNotFoundException` | `structure` | `Message` | Thrown when the requested resource cannot be found or does not exist. |
| `ThrottlingException` | `structure` | `Message` | Thrown when the request rate exceeds the allowed limits and the request is being throttled. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | Thrown when the request contains invalid parameters or fails input validation requirements. |
| `ConflictException` | `structure` | `Message` | Thrown when the request conflicts with the current state of the resource, such as attempting to modify a resource that has been changed by another process. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType` | Thrown when the request would exceed the service quotas or limits for the account. |
| `AmendBenefitApplicationInput` | `structure` | `AmendmentReason`, `Amendments`, `Catalog`, `ClientToken`, `Identifier`, `Revision` | - |
| `AmendBenefitApplicationOutput` | `structure` | - | - |
| `AssociateBenefitApplicationResourceInput` | `structure` | `BenefitApplicationIdentifier`, `Catalog`, `ResourceArn` | - |
| `AssociateBenefitApplicationResourceOutput` | `structure` | `Arn`, `Id`, `Revision` | - |
| `CancelBenefitApplicationInput` | `structure` | `Catalog`, `ClientToken`, `Identifier`, `Reason` | - |
| `CancelBenefitApplicationOutput` | `structure` | - | - |
| `CreateBenefitApplicationInput` | `structure` | `AssociatedResources`, `BenefitApplicationDetails`, `BenefitIdentifier`, `Catalog`, `ClientToken`, `Description`, `FileDetails`, `FulfillmentTypes`, `Name`, `PartnerContacts`, `Tags` | - |
| `CreateBenefitApplicationOutput` | `structure` | `Arn`, `Id`, `Revision` | - |
| `DisassociateBenefitApplicationResourceInput` | `structure` | `BenefitApplicationIdentifier`, `Catalog`, `ResourceArn` | - |
| `DisassociateBenefitApplicationResourceOutput` | `structure` | `Arn`, `Id`, `Revision` | - |
| `GetBenefitInput` | `structure` | `Catalog`, `Identifier` | - |
| `GetBenefitOutput` | `structure` | `Arn`, `BenefitRequestSchema`, `Catalog`, `Description`, `FulfillmentTypes`, `Id`, `Name`, `Programs`, `Status` | - |
| `GetBenefitAllocationInput` | `structure` | `Catalog`, `Identifier` | - |
| `GetBenefitAllocationOutput` | `structure` | `ApplicableBenefitIds`, `Arn`, `BenefitApplicationId`, `BenefitId`, `Catalog`, `CreatedAt`, `Description`, `ExpiresAt`, `FulfillmentDetail`, `FulfillmentType`, `Id`, `Name`, ... (+4) | - |
| `GetBenefitApplicationInput` | `structure` | `Catalog`, `Identifier` | - |
| `GetBenefitApplicationOutput` | `structure` | `Arn`, `AssociatedResources`, `BenefitApplicationDetails`, `BenefitId`, `Catalog`, `CreatedAt`, `Description`, `FileDetails`, `FulfillmentTypes`, `Id`, `Name`, `PartnerContacts`, ... (+8) | - |
| `ListBenefitAllocationsInput` | `structure` | `BenefitApplicationIdentifiers`, `BenefitIdentifiers`, `Catalog`, `FulfillmentTypes`, `MaxResults`, `NextToken`, `Status` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

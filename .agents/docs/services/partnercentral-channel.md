# Partner Central Channel API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Partner Central Channel service for managing partner relationships, handshakes, and program management accounts.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Partner Central Channel API workflows in the local mock. Key resources include `ChannelHandshakeResource`, `ProgramManagementAccountResource`, `RelationshipResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Update`, `Accept` operation families, including `ListChannelHandshakes`, `ListProgramManagementAccounts`, `ListRelationships`, `ListTagsForResource`, `CreateChannelHandshake`, `CreateProgramManagementAccount`.

## Service Identity and Protocol

- AWS model slug: `partnercentral-channel`
- AWS SDK for Rust slug: `partnercentralchannel`
- Model version: `2024-03-18`
- Model file: `vendor/api-models-aws/models/partnercentral-channel/service/2024-03-18/partnercentral-channel-2024-03-18.json`
- SDK ID: `PartnerCentral Channel`
- Endpoint prefix: `partnercentral-channel`
- ARN namespace: `partnercentral`
- CloudFormation name: `-`
- CloudTrail event source: `partnercentral-channel.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`, `sigv4a`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (3), `Delete` (2), `Update` (2), `Accept` (1), `Cancel` (1), `Get` (1), `Reject` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptChannelHandshake`, `CancelChannelHandshake`, `CreateChannelHandshake`, `CreateProgramManagementAccount`, `CreateRelationship`, `DeleteProgramManagementAccount`, `DeleteRelationship`, `RejectChannelHandshake`, `TagResource`, `UntagResource`, `UpdateProgramManagementAccount`, `UpdateRelationship`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRelationship`, `ListChannelHandshakes`, `ListProgramManagementAccounts`, `ListRelationships`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelChannelHandshake`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ChannelHandshakeResource` | `identifier` | create: `CreateChannelHandshake`; list: `ListChannelHandshakes` | `AcceptChannelHandshake`, `CancelChannelHandshake`, `RejectChannelHandshake` | - |
| `ProgramManagementAccountResource` | `identifier` | create: `CreateProgramManagementAccount`; update: `UpdateProgramManagementAccount`; delete: `DeleteProgramManagementAccount`; list: `ListProgramManagementAccounts` | - | - |
| `RelationshipResource` | `identifier` | create: `CreateRelationship`; read: `GetRelationship`; update: `UpdateRelationship`; delete: `DeleteRelationship`; list: `ListRelationships` | - | - |
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
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `POST /ListTagsForResource` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags associated with a specific resource. |
| `TagResource` | `POST /TagResource` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified resource. |
| `UntagResource` | `POST /UntagResource` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, reason | The request was denied due to insufficient permissions. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request could not be completed due to a conflict with the current state of the resource. |
| `InternalServerException` | `structure` | message | An internal server error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, quotaCode | The request would exceed a service quota limit. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode | The request was throttled due to too many requests being sent in a short period. |
| `ValidationException` | `structure` | message, reason, fieldList | The request failed validation due to invalid input parameters. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AssociationType` | `enum` | DOWNSTREAM_SELLER, END_CUSTOMER, INTERNAL | - |
| `Coverage` | `enum` | ENTIRE_ORGANIZATION, MANAGEMENT_ACCOUNT_ONLY | - |
| `HandshakeStatus` | `enum` | PENDING, ACCEPTED, REJECTED, CANCELED, EXPIRED | - |
| `HandshakeType` | `enum` | START_SERVICE_PERIOD, REVOKE_SERVICE_PERIOD, PROGRAM_MANAGEMENT_ACCOUNT | - |
| `ListProgramManagementAccountsSortName` | `enum` | UPDATED_AT | - |
| `ListRelationshipsSortName` | `enum` | UPDATED_AT | - |
| `ParticipantType` | `enum` | SENDER, RECEIVER | - |
| `Program` | `enum` | SOLUTION_PROVIDER, DISTRIBUTION, DISTRIBUTION_SELLER | - |
| `ProgramManagementAccountStatus` | `enum` | PENDING, ACTIVE, INACTIVE | - |
| `ProgramManagementAccountTypeSortName` | `enum` | UPDATED_AT | - |
| `Provider` | `enum` | DISTRIBUTOR, DISTRIBUTION_SELLER | - |
| `ResaleAccountModel` | `enum` | DISTRIBUTOR, END_CUSTOMER, SOLUTION_PROVIDER | - |
| `RevokeServicePeriodTypeSortName` | `enum` | UPDATED_AT | - |
| `Sector` | `enum` | COMMERCIAL, GOVERNMENT, GOVERNMENT_EXCEPTION | - |
| `ServicePeriodType` | `enum` | MINIMUM_NOTICE_PERIOD, FIXED_COMMITMENT_PERIOD | - |
| `SortOrder` | `enum` | ASCENDING, DESCENDING | - |
| `StartServicePeriodTypeSortName` | `enum` | UPDATED_AT | - |
| `ValidationExceptionReason` | `enum` | REQUEST_VALIDATION_FAILED, BUSINESS_VALIDATION_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

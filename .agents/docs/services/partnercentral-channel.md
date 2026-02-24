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

- Operations: `ListChannelHandshakes`, `ListProgramManagementAccounts`, `ListRelationships`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `catalog`, `handshakeType`, `participantType`, `resourceArn`

### Create

- Operations: `CreateChannelHandshake`, `CreateProgramManagementAccount`, `CreateRelationship`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `accountId`, `associatedAccountId`, `associatedResourceIdentifier`, `associationType`, `catalog`, `displayName`, `handshakeType`, `program`, `programManagementAccountIdentifier`, `sector`

### Delete

- Operations: `DeleteProgramManagementAccount`, `DeleteRelationship`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `catalog`, `identifier`, `programManagementAccountIdentifier`

### Update

- Operations: `UpdateProgramManagementAccount`, `UpdateRelationship`
- Traits: `idempotent` (1)
- Common required input members in this group: `catalog`, `identifier`, `programManagementAccountIdentifier`

### Accept

- Operations: `AcceptChannelHandshake`
- Traits: `idempotent` (1)
- Common required input members in this group: `catalog`, `identifier`

### Cancel

- Operations: `CancelChannelHandshake`
- Traits: `idempotent` (1)
- Common required input members in this group: `catalog`, `identifier`

### Get

- Operations: `GetRelationship`
- Traits: `readonly` (1)
- Common required input members in this group: `catalog`, `identifier`, `programManagementAccountIdentifier`

### Reject

- Operations: `RejectChannelHandshake`
- Traits: `idempotent` (1)
- Common required input members in this group: `catalog`, `identifier`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptChannelHandshake` | `POST /AcceptChannelHandshake` | `idempotent` | `catalog`, `identifier` | - | `AcceptChannelHandshakeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Accepts a pending channel handshake request from another AWS account. |
| `CancelChannelHandshake` | `POST /CancelChannelHandshake` | `idempotent` | `catalog`, `identifier` | - | `CancelChannelHandshakeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a pending channel handshake request. |
| `CreateChannelHandshake` | `POST /CreateChannelHandshake` | `idempotent`, `idempotency-token` | `associatedResourceIdentifier`, `catalog`, `handshakeType` | `clientToken` | `CreateChannelHandshakeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new channel handshake request to establish a partnership with another AWS account. |
| `CreateProgramManagementAccount` | `POST /CreateProgramManagementAccount` | `idempotent`, `idempotency-token` | `accountId`, `catalog`, `displayName`, `program` | `clientToken` | `CreateProgramManagementAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new program management account for managing partner relationships. |
| `CreateRelationship` | `POST /CreateRelationship` | `idempotent`, `idempotency-token` | `associatedAccountId`, `associationType`, `catalog`, `displayName`, `programManagementAccountIdentifier`, `sector` | `clientToken` | `CreateRelationshipResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new partner relationship between accounts. |
| `DeleteProgramManagementAccount` | `POST /DeleteProgramManagementAccount` | `idempotent`, `idempotency-token` | `catalog`, `identifier` | `clientToken` | `DeleteProgramManagementAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a program management account. |
| `DeleteRelationship` | `POST /DeleteRelationship` | `idempotent`, `idempotency-token` | `catalog`, `identifier`, `programManagementAccountIdentifier` | `clientToken` | `DeleteRelationshipResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a partner relationship. |
| `GetRelationship` | `POST /GetRelationship` | `readonly` | `catalog`, `identifier`, `programManagementAccountIdentifier` | - | `GetRelationshipResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details of a specific partner relationship. |
| `ListChannelHandshakes` | `POST /ListChannelHandshakes` | `readonly`, `paginated` | `catalog`, `handshakeType`, `participantType` | - | `ListChannelHandshakesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists channel handshakes based on specified criteria. |
| `ListProgramManagementAccounts` | `POST /ListProgramManagementAccounts` | `readonly`, `paginated` | `catalog` | - | `ListProgramManagementAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists program management accounts based on specified criteria. |
| `ListRelationships` | `POST /ListRelationships` | `readonly`, `paginated` | `catalog` | - | `ListRelationshipsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists partner relationships based on specified criteria. |
| `ListTagsForResource` | `POST /ListTagsForResource` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags associated with a specific resource. |
| `RejectChannelHandshake` | `POST /RejectChannelHandshake` | `idempotent` | `catalog`, `identifier` | - | `RejectChannelHandshakeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects a pending channel handshake request. |
| `TagResource` | `POST /TagResource` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified resource. |
| `UntagResource` | `POST /UntagResource` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a specified resource. |
| `UpdateProgramManagementAccount` | `POST /UpdateProgramManagementAccount` | `idempotent` | `catalog`, `identifier` | - | `UpdateProgramManagementAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the properties of a program management account. |
| `UpdateRelationship` | `POST /UpdateRelationship` | - | `catalog`, `identifier`, `programManagementAccountIdentifier` | - | `UpdateRelationshipResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the properties of a partner relationship. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message`, `reason` | The request was denied due to insufficient permissions. |
| `InternalServerException` | `structure` | `message` | An internal server error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The specified resource was not found. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `serviceCode` | The request was throttled due to too many requests being sent in a short period. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The request failed validation due to invalid input parameters. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request could not be completed due to a conflict with the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType` | The request would exceed a service quota limit. |
| `AcceptChannelHandshakeRequest` | `structure` | `catalog`, `identifier` | - |
| `AcceptChannelHandshakeResponse` | `structure` | `channelHandshakeDetail` | - |
| `CancelChannelHandshakeRequest` | `structure` | `catalog`, `identifier` | - |
| `CancelChannelHandshakeResponse` | `structure` | `channelHandshakeDetail` | - |
| `CreateChannelHandshakeRequest` | `structure` | `associatedResourceIdentifier`, `catalog`, `clientToken`, `handshakeType`, `payload`, `tags` | - |
| `CreateChannelHandshakeResponse` | `structure` | `channelHandshakeDetail` | - |
| `CreateProgramManagementAccountRequest` | `structure` | `accountId`, `catalog`, `clientToken`, `displayName`, `program`, `tags` | - |
| `CreateProgramManagementAccountResponse` | `structure` | `programManagementAccountDetail` | - |
| `CreateRelationshipRequest` | `structure` | `associatedAccountId`, `associationType`, `catalog`, `clientToken`, `displayName`, `programManagementAccountIdentifier`, `requestedSupportPlan`, `resaleAccountModel`, `sector`, `tags` | - |
| `CreateRelationshipResponse` | `structure` | `relationshipDetail` | - |
| `DeleteProgramManagementAccountRequest` | `structure` | `catalog`, `clientToken`, `identifier` | - |
| `DeleteProgramManagementAccountResponse` | `structure` | - | - |
| `DeleteRelationshipRequest` | `structure` | `catalog`, `clientToken`, `identifier`, `programManagementAccountIdentifier` | - |
| `DeleteRelationshipResponse` | `structure` | - | - |
| `GetRelationshipRequest` | `structure` | `catalog`, `identifier`, `programManagementAccountIdentifier` | - |
| `GetRelationshipResponse` | `structure` | `relationshipDetail` | - |
| `ListChannelHandshakesRequest` | `structure` | `associatedResourceIdentifiers`, `catalog`, `handshakeType`, `handshakeTypeFilters`, `handshakeTypeSort`, `maxResults`, `nextToken`, `participantType`, `statuses` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

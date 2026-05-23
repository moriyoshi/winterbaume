# AWS re:Post Private

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS re:Post Private is a private version of AWS re:Post for enterprises with Enterprise Support or Enterprise On-Ramp Support plans. It provides access to knowledge and experts to accelerate cloud adoption and increase developer productivity. With your organization-specific private re:Post, you can build an organization-specific developer community that drives efficiencies at scale and provides access to valuable knowledge resources. Additionally, re:Post Private centralizes trusted AWS technical content and offers private discussion forums to improve how your teams collaborate internally and with AWS to remove technical obstacles, accelerate innovation, and scale more efficiently in the cloud.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS re:Post Private workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Batch`, `List`, `Create`, `Get`, `Update` operation families, including `BatchAddChannelRoleToAccessors`, `BatchAddRole`, `BatchRemoveChannelRoleFromAccessors`, `BatchRemoveRole`, `ListChannels`, `ListSpaces`.

## Service Identity and Protocol

- AWS model slug: `repostspace`
- AWS SDK for Rust slug: `repostspace`
- Model version: `2022-05-13`
- Model file: `vendor/api-models-aws/models/repostspace/service/2022-05-13/repostspace-2022-05-13.json`
- SDK ID: `repostspace`
- Endpoint prefix: `-`
- ARN namespace: `repostspace`
- CloudFormation name: `-`
- CloudTrail event source: `repostspace.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Batch` (4), `List` (3), `Create` (2), `Get` (2), `Update` (2), `Delete` (1), `Deregister` (1), `Register` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchAddChannelRoleToAccessors`, `BatchAddRole`, `BatchRemoveChannelRoleFromAccessors`, `BatchRemoveRole`, `CreateChannel`, `CreateSpace`, `DeleteSpace`, `DeregisterAdmin`, `RegisterAdmin`, `TagResource`, `UntagResource`, `UpdateChannel`, `UpdateSpace`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetChannel`, `GetSpace`, `ListChannels`, `ListSpaces`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.

## Operation Groups

### Batch

- Operations: `BatchAddChannelRoleToAccessors`, `BatchAddRole`, `BatchRemoveChannelRoleFromAccessors`, `BatchRemoveRole`
- Traits: `idempotent` (4)
- Common required input members in this group: `spaceId`, `channelId`, `accessorIds`, `channelRole`, `role`

### List

- Operations: `ListChannels`, `ListSpaces`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateChannel`, `CreateSpace`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Get

- Operations: `GetChannel`, `GetSpace`
- Traits: `readonly` (2)
- Common required input members in this group: `spaceId`

### Update

- Operations: `UpdateChannel`, `UpdateSpace`
- Traits: `idempotent` (2)
- Common required input members in this group: `spaceId`

### Delete

- Operations: `DeleteSpace`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterAdmin`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterAdmin`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Send

- Operations: `SendInvites`
- Traits: `idempotent` (1)
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
| `BatchAddChannelRoleToAccessors` | `POST /spaces/{spaceId}/channels/{channelId}/roles` | `idempotent` | `spaceId`, `channelId`, `accessorIds`, `channelRole` | - | `BatchAddChannelRoleToAccessorsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add role to multiple users or groups in a private re:Post channel. |
| `BatchAddRole` | `POST /spaces/{spaceId}/roles` | `idempotent` | `spaceId`, `accessorIds`, `role` | - | `BatchAddRoleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add a role to multiple users or groups in a private re:Post. |
| `BatchRemoveChannelRoleFromAccessors` | `PATCH /spaces/{spaceId}/channels/{channelId}/roles` | `idempotent` | `spaceId`, `channelId`, `accessorIds`, `channelRole` | - | `BatchRemoveChannelRoleFromAccessorsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a role from multiple users or groups in a private re:Post channel. |
| `BatchRemoveRole` | `PATCH /spaces/{spaceId}/roles` | `idempotent` | `spaceId`, `accessorIds`, `role` | - | `BatchRemoveRoleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a role from multiple users or groups in a private re:Post. |
| `CreateChannel` | `POST /spaces/{spaceId}/channels` | `idempotent` | `spaceId`, `channelName` | - | `CreateChannelOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a channel in an AWS re:Post Private private re:Post. |
| `CreateSpace` | `POST /spaces` | `idempotent` | `name`, `subdomain`, `tier` | - | `CreateSpaceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an AWS re:Post Private private re:Post. |
| `DeleteSpace` | `DELETE /spaces/{spaceId}` | `idempotent` | `spaceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an AWS re:Post Private private re:Post. |
| `DeregisterAdmin` | `DELETE /spaces/{spaceId}/admins/{adminId}` | `idempotent` | `spaceId`, `adminId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the user or group from the list of administrators of the private re:Post. |
| `GetChannel` | `GET /spaces/{spaceId}/channels/{channelId}` | `readonly` | `spaceId`, `channelId` | - | `GetChannelOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays information about a channel in a private re:Post. |
| `GetSpace` | `GET /spaces/{spaceId}` | `readonly` | `spaceId` | - | `GetSpaceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays information about the AWS re:Post Private private re:Post. |
| `ListChannels` | `GET /spaces/{spaceId}/channels` | `readonly`, `paginated` | `spaceId` | - | `ListChannelsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the list of channel within a private re:Post with some information about each channel. |
| `ListSpaces` | `GET /spaces` | `readonly`, `paginated` | - | - | `ListSpacesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of AWS re:Post Private private re:Posts in the account with some information about each private re:Post. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the tags that are associated with the AWS re:Post Private resource specified by the resourceArn. The only resource that can be tagged is a private re:Post. |
| `RegisterAdmin` | `POST /spaces/{spaceId}/admins/{adminId}` | `idempotent` | `spaceId`, `adminId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a user or group to the list of administrators of the private re:Post. |
| `SendInvites` | `POST /spaces/{spaceId}/invite` | `idempotent` | `spaceId`, `accessorIds`, `title`, `body` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends an invitation email to selected users and groups. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates tags with an AWS re:Post Private resource. Currently, the only resource that can be tagged is the private re:Post. If you specify a new tag key for the resource, the tag is appended to the list of tags tha ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association of the tag with the AWS re:Post Private resource. |
| `UpdateChannel` | `PUT /spaces/{spaceId}/channels/{channelId}` | `idempotent` | `spaceId`, `channelId`, `channelName` | - | `UpdateChannelOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing channel. |
| `UpdateSpace` | `PUT /spaces/{spaceId}` | `idempotent` | `spaceId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing AWS re:Post Private private re:Post. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListChannels` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListSpaces` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | User does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | Unexpected error during processing of request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | Request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | Request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | Request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the constraints specified by an AWS service. |
| `BatchAddChannelRoleToAccessorsInput` | `structure` | spaceId, channelId, accessorIds, channelRole | - |
| `BatchAddChannelRoleToAccessorsOutput` | `structure` | addedAccessorIds, errors | - |
| `BatchAddRoleInput` | `structure` | spaceId, accessorIds, role | - |
| `BatchAddRoleOutput` | `structure` | addedAccessorIds, errors | - |
| `BatchRemoveChannelRoleFromAccessorsInput` | `structure` | spaceId, channelId, accessorIds, channelRole | - |
| `BatchRemoveChannelRoleFromAccessorsOutput` | `structure` | removedAccessorIds, errors | - |
| `BatchRemoveRoleInput` | `structure` | spaceId, accessorIds, role | - |
| `BatchRemoveRoleOutput` | `structure` | removedAccessorIds, errors | - |
| `CreateChannelInput` | `structure` | spaceId, channelName, channelDescription | - |
| `CreateChannelOutput` | `structure` | channelId | - |
| `CreateSpaceInput` | `structure` | name, subdomain, tier, description, userKMSKey, tags, roleArn, supportedEmailDomains | - |
| `CreateSpaceOutput` | `structure` | spaceId | - |
| `DeleteSpaceInput` | `structure` | spaceId | - |
| `DeregisterAdminInput` | `structure` | spaceId, adminId | - |
| `GetChannelInput` | `structure` | spaceId, channelId | - |
| `GetChannelOutput` | `structure` | spaceId, channelId, channelName, channelDescription, createDateTime, deleteDateTime, channelRoles, channelStatus | - |
| `GetSpaceInput` | `structure` | spaceId | - |
| `GetSpaceOutput` | `structure` | spaceId, arn, name, status, configurationStatus, clientId, identityStoreId, applicationArn, description, vanityDomainStatus, vanityDomain, randomDomain, ... (+12) | - |
| `ListChannelsInput` | `structure` | spaceId, nextToken, maxResults | - |
| `ListChannelsOutput` | `structure` | channels, nextToken | - |
| `ListSpacesInput` | `structure` | nextToken, maxResults | - |
| `ListSpacesOutput` | `structure` | spaces, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `RegisterAdminInput` | `structure` | spaceId, adminId | - |
| `SendInvitesInput` | `structure` | spaceId, accessorIds, title, body | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateChannelInput` | `structure` | spaceId, channelId, channelName, channelDescription | - |
| `UpdateChannelOutput` | `structure` | **empty (no members)** | - |
| `UpdateSpaceInput` | `structure` | spaceId, description, tier, roleArn, supportedEmailDomains | - |
| `ChannelRole` | `enum` | ASKER, EXPERT, MODERATOR, SUPPORTREQUESTOR | - |
| `ChannelStatus` | `enum` | CREATED, CREATING, CREATE_FAILED, DELETED, DELETING, DELETE_FAILED | - |
| `ConfigurationStatus` | `enum` | CONFIGURED, UNCONFIGURED | - |
| `FeatureEnableParameter` | `enum` | ENABLED, DISABLED | - |
| `FeatureEnableStatus` | `enum` | ENABLED, DISABLED, NOT_ALLOWED | - |
| `Role` | `enum` | EXPERT, MODERATOR, ADMINISTRATOR, SUPPORTREQUESTOR | - |
| `TierLevel` | `enum` | BASIC, STANDARD | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
| `VanityDomainStatus` | `enum` | PENDING, APPROVED, UNAPPROVED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

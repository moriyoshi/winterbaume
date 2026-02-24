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
- Common required input members in this group: `accessorIds`, `channelId`, `channelRole`, `role`, `spaceId`

### List

- Operations: `ListChannels`, `ListSpaces`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `resourceArn`, `spaceId`

### Create

- Operations: `CreateChannel`, `CreateSpace`
- Traits: `idempotent` (2)
- Common required input members in this group: `channelName`, `name`, `spaceId`, `subdomain`, `tier`

### Get

- Operations: `GetChannel`, `GetSpace`
- Traits: `readonly` (2)
- Common required input members in this group: `channelId`, `spaceId`

### Update

- Operations: `UpdateChannel`, `UpdateSpace`
- Traits: `idempotent` (2)
- Common required input members in this group: `channelId`, `channelName`, `spaceId`

### Delete

- Operations: `DeleteSpace`
- Traits: `idempotent` (1)
- Common required input members in this group: `spaceId`

### Deregister

- Operations: `DeregisterAdmin`
- Traits: `idempotent` (1)
- Common required input members in this group: `adminId`, `spaceId`

### Register

- Operations: `RegisterAdmin`
- Traits: `idempotent` (1)
- Common required input members in this group: `adminId`, `spaceId`

### Send

- Operations: `SendInvites`
- Traits: `idempotent` (1)
- Common required input members in this group: `accessorIds`, `body`, `spaceId`, `title`

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
| `BatchAddChannelRoleToAccessors` | `POST /spaces/{spaceId}/channels/{channelId}/roles` | `idempotent` | `accessorIds`, `channelId`, `channelRole`, `spaceId` | - | `BatchAddChannelRoleToAccessorsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add role to multiple users or groups in a private re:Post channel. |
| `BatchAddRole` | `POST /spaces/{spaceId}/roles` | `idempotent` | `accessorIds`, `role`, `spaceId` | - | `BatchAddRoleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add a role to multiple users or groups in a private re:Post. |
| `BatchRemoveChannelRoleFromAccessors` | `PATCH /spaces/{spaceId}/channels/{channelId}/roles` | `idempotent` | `accessorIds`, `channelId`, `channelRole`, `spaceId` | - | `BatchRemoveChannelRoleFromAccessorsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a role from multiple users or groups in a private re:Post channel. |
| `BatchRemoveRole` | `PATCH /spaces/{spaceId}/roles` | `idempotent` | `accessorIds`, `role`, `spaceId` | - | `BatchRemoveRoleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a role from multiple users or groups in a private re:Post. |
| `CreateChannel` | `POST /spaces/{spaceId}/channels` | `idempotent` | `channelName`, `spaceId` | - | `CreateChannelOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a channel in an AWS re:Post Private private re:Post. |
| `CreateSpace` | `POST /spaces` | `idempotent` | `name`, `subdomain`, `tier` | - | `CreateSpaceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an AWS re:Post Private private re:Post. |
| `DeleteSpace` | `DELETE /spaces/{spaceId}` | `idempotent` | `spaceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an AWS re:Post Private private re:Post. |
| `DeregisterAdmin` | `DELETE /spaces/{spaceId}/admins/{adminId}` | `idempotent` | `adminId`, `spaceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the user or group from the list of administrators of the private re:Post. |
| `GetChannel` | `GET /spaces/{spaceId}/channels/{channelId}` | `readonly` | `channelId`, `spaceId` | - | `GetChannelOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays information about a channel in a private re:Post. |
| `GetSpace` | `GET /spaces/{spaceId}` | `readonly` | `spaceId` | - | `GetSpaceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays information about the AWS re:Post Private private re:Post. |
| `ListChannels` | `GET /spaces/{spaceId}/channels` | `readonly`, `paginated` | `spaceId` | - | `ListChannelsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the list of channel within a private re:Post with some information about each channel. |
| `ListSpaces` | `GET /spaces` | `readonly`, `paginated` | - | - | `ListSpacesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of AWS re:Post Private private re:Posts in the account with some information about each private re:Post. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the tags that are associated with the AWS re:Post Private resource specified by the resourceArn. The only resource that can be tagged is a private re:Post. |
| `RegisterAdmin` | `POST /spaces/{spaceId}/admins/{adminId}` | `idempotent` | `adminId`, `spaceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a user or group to the list of administrators of the private re:Post. |
| `SendInvites` | `POST /spaces/{spaceId}/invite` | `idempotent` | `accessorIds`, `body`, `spaceId`, `title` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends an invitation email to selected users and groups. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates tags with an AWS re:Post Private resource. Currently, the only resource that can be tagged is the private re:Post. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association of the tag with the AWS re:Post Private resource. |
| `UpdateChannel` | `PUT /spaces/{spaceId}/channels/{channelId}` | `idempotent` | `channelId`, `channelName`, `spaceId` | - | `UpdateChannelOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing channel. |
| `UpdateSpace` | `PUT /spaces/{spaceId}` | `idempotent` | `spaceId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing AWS re:Post Private private re:Post. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | User does not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | Unexpected error during processing of request. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | Request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by an AWS service. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | Request references a resource which does not exist. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | Request would cause a service quota to be exceeded. |
| `BatchAddChannelRoleToAccessorsInput` | `structure` | `accessorIds`, `channelId`, `channelRole`, `spaceId` | - |
| `BatchAddChannelRoleToAccessorsOutput` | `structure` | `addedAccessorIds`, `errors` | - |
| `BatchAddRoleInput` | `structure` | `accessorIds`, `role`, `spaceId` | - |
| `BatchAddRoleOutput` | `structure` | `addedAccessorIds`, `errors` | - |
| `BatchRemoveChannelRoleFromAccessorsInput` | `structure` | `accessorIds`, `channelId`, `channelRole`, `spaceId` | - |
| `BatchRemoveChannelRoleFromAccessorsOutput` | `structure` | `errors`, `removedAccessorIds` | - |
| `BatchRemoveRoleInput` | `structure` | `accessorIds`, `role`, `spaceId` | - |
| `BatchRemoveRoleOutput` | `structure` | `errors`, `removedAccessorIds` | - |
| `CreateChannelInput` | `structure` | `channelDescription`, `channelName`, `spaceId` | - |
| `CreateChannelOutput` | `structure` | `channelId` | - |
| `CreateSpaceInput` | `structure` | `description`, `name`, `roleArn`, `subdomain`, `supportedEmailDomains`, `tags`, `tier`, `userKMSKey` | - |
| `CreateSpaceOutput` | `structure` | `spaceId` | - |
| `DeleteSpaceInput` | `structure` | `spaceId` | - |
| `DeregisterAdminInput` | `structure` | `adminId`, `spaceId` | - |
| `GetChannelInput` | `structure` | `channelId`, `spaceId` | - |
| `GetChannelOutput` | `structure` | `channelDescription`, `channelId`, `channelName`, `channelRoles`, `channelStatus`, `createDateTime`, `deleteDateTime`, `spaceId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

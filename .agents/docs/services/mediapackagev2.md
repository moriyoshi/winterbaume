# AWS Elemental MediaPackage v2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This guide is intended for creating AWS Elemental MediaPackage resources in MediaPackage Version 2 (v2) starting from May 2023. To get started with MediaPackage v2, create your MediaPackage resources. There isn't an automated process to migrate your resources from MediaPackage v1 to MediaPackage v2. The names of the entities that you use to access this API, like URLs and ARNs, all have the versioning information added, like "v2", to distinguish from the prior version. If you used MediaPackage prior to this release, you can't use the MediaPackage v2 CLI or the MediaPackage v2 API to access any MediaPackage v1 resources.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Elemental MediaPackage v2 resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Elemental MediaPackage v2 workflows in the local mock. Key resources include `ChannelGroupResource`, `ChannelPolicyResource`, `ChannelResource`, `HarvestJobResource`, `OriginEndpointPolicyResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Delete`, `List`, `Create`, `Update` operation families, including `GetChannel`, `GetChannelGroup`, `GetChannelPolicy`, `GetHarvestJob`, `DeleteChannel`, `DeleteChannelGroup`.

## Service Identity and Protocol

- AWS model slug: `mediapackagev2`
- AWS SDK for Rust slug: `mediapackagev2`
- Model version: `2022-12-25`
- Model file: `vendor/api-models-aws/models/mediapackagev2/service/2022-12-25/mediapackagev2-2022-12-25.json`
- SDK ID: `MediaPackageV2`
- Endpoint prefix: `mediapackagev2`
- ARN namespace: `mediapackagev2`
- CloudFormation name: `MediaPackageV2`
- CloudTrail event source: `mediapackagev2.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `Delete` (5), `List` (5), `Create` (4), `Update` (3), `Put` (2), `Reset` (2), `Cancel` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelHarvestJob`, `CreateChannel`, `CreateChannelGroup`, `CreateHarvestJob`, `CreateOriginEndpoint`, `DeleteChannel`, `DeleteChannelGroup`, `DeleteChannelPolicy`, `DeleteOriginEndpoint`, `DeleteOriginEndpointPolicy`, `PutChannelPolicy`, `PutOriginEndpointPolicy`, `TagResource`, `UntagResource`, `UpdateChannel`, `UpdateChannelGroup`, `UpdateOriginEndpoint`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetChannel`, `GetChannelGroup`, `GetChannelPolicy`, `GetHarvestJob`, `GetOriginEndpoint`, `GetOriginEndpointPolicy`, `ListChannelGroups`, `ListChannels`, `ListHarvestJobs`, `ListOriginEndpoints`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 18 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelHarvestJob`, `CreateHarvestJob`, `GetHarvestJob`, `ListHarvestJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 30 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`.

## v1/v2 State Coherence

- **Paired with `mediapackage` ( different SDK slug, different resource model ).** Despite the v1/v2 naming, AWS treats MediaPackage v2 as a **separate service** from MediaPackage v1 — the v2 service overview explicitly says: "There isn't an automated process to migrate your resources from MediaPackage v1 to MediaPackage v2 [...] you can't use the MediaPackage v2 CLI or the MediaPackage v2 API to access any MediaPackage v1 resources."
- v2 introduces `ChannelGroup` as the new top-level container, plus channel and origin-endpoint resource policies and harvest jobs that v1 does not have. Channel and origin-endpoint identifiers are scoped under a channel group, so even names that look the same as v1 names live in a different namespace.
- **Current Winterbaume status: correctly separate.** `winterbaume-mediapackagev2` and `winterbaume-mediapackage` each own their own state, mirroring real AWS. No dependency between the crates is needed; do not introduce cross-API visibility.

## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ChannelGroupResource` | `ChannelGroupName` | put: `CreateChannelGroup`; read: `GetChannelGroup`; update: `UpdateChannelGroup`; delete: `DeleteChannelGroup`; list: `ListChannelGroups` | - | Represents a channel group that facilitates the grouping of multiple channels. |
| `ChannelPolicyResource` | `ChannelGroupName`, `ChannelName` | put: `PutChannelPolicy`; read: `GetChannelPolicy`; delete: `DeleteChannelPolicy` | - | Represents a resource-based policy that allows or denies access to a channel. |
| `ChannelResource` | `ChannelGroupName`, `ChannelName` | put: `CreateChannel`; read: `GetChannel`; update: `UpdateChannel`; delete: `DeleteChannel`; list: `ListChannels` | `ResetChannelState` | Represents an entry point into AWS Elemental MediaPackage for an ABR video content stream sent from an upstream encoder such as AWS Elemental MediaLive. |
| `HarvestJobResource` | `ChannelGroupName`, `ChannelName`, `HarvestJobName`, `OriginEndpointName` | create: `CreateHarvestJob`; read: `GetHarvestJob`; update: `CancelHarvestJob`; list: `ListHarvestJobs` | - | A HarvestJob resource represents a request to export content from a MediaPackage v2 channel to an S3 bucket. |
| `OriginEndpointPolicyResource` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | put: `PutOriginEndpointPolicy`; read: `GetOriginEndpointPolicy`; delete: `DeleteOriginEndpointPolicy` | - | Represents a resource policy that allows or denies access to an origin endpoint. |
| `OriginEndpointResource` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | put: `CreateOriginEndpoint`; read: `GetOriginEndpoint`; update: `UpdateOriginEndpoint`; delete: `DeleteOriginEndpoint`; list: `ListOriginEndpoints` | `ResetOriginEndpointState` | Represents an origin endpoint that is associated with a channel, offering a dynamically repackaged version of its content through various streaming media protocols. |
## Operation Groups

### Get

- Operations: `GetChannel`, `GetChannelGroup`, `GetChannelPolicy`, `GetHarvestJob`, `GetOriginEndpoint`, `GetOriginEndpointPolicy`
- Traits: `readonly` (6)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `HarvestJobName`, `OriginEndpointName`

### Delete

- Operations: `DeleteChannel`, `DeleteChannelGroup`, `DeleteChannelPolicy`, `DeleteOriginEndpoint`, `DeleteOriginEndpointPolicy`
- Traits: `idempotent` (5)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `OriginEndpointName`

### List

- Operations: `ListChannelGroups`, `ListChannels`, `ListHarvestJobs`, `ListOriginEndpoints`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `ResourceArn`

### Create

- Operations: `CreateChannel`, `CreateChannelGroup`, `CreateHarvestJob`, `CreateOriginEndpoint`
- Traits: `idempotency-token` (4), `idempotent` (4)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `ContainerType`, `Destination`, `HarvestedManifests`, `OriginEndpointName`, `ScheduleConfiguration`

### Update

- Operations: `UpdateChannel`, `UpdateChannelGroup`, `UpdateOriginEndpoint`
- Traits: `idempotent` (3)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `ContainerType`, `OriginEndpointName`

### Put

- Operations: `PutChannelPolicy`, `PutOriginEndpointPolicy`
- Traits: `idempotent` (2)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `OriginEndpointName`, `Policy`

### Reset

- Operations: `ResetChannelState`, `ResetOriginEndpointState`
- Traits: `idempotent` (2)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `OriginEndpointName`

### Cancel

- Operations: `CancelHarvestJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChannelGroupName`, `ChannelName`, `HarvestJobName`, `OriginEndpointName`

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
| `CancelHarvestJob` | `PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob/{HarvestJobName}` | `idempotent` | `ChannelGroupName`, `ChannelName`, `HarvestJobName`, `OriginEndpointName` | - | `CancelHarvestJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an in-progress harvest job. |
| `CreateChannel` | `POST /channelGroup/{ChannelGroupName}/channel` | `idempotent`, `idempotency-token` | `ChannelGroupName`, `ChannelName` | `ClientToken` | `CreateChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a channel to start receiving content streams. The channel represents the input to MediaPackage for incoming live content from an encoder such as AWS Elemental MediaLive. |
| `CreateChannelGroup` | `POST /channelGroup` | `idempotent`, `idempotency-token` | `ChannelGroupName` | `ClientToken` | `CreateChannelGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a channel group to group your channels and origin endpoints. A channel group is the top-level resource that consists of channels and origin endpoints that are associated with it and that provides predictable URLs for stream delivery. |
| `CreateHarvestJob` | `POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob` | `idempotent`, `idempotency-token` | `ChannelGroupName`, `ChannelName`, `Destination`, `HarvestedManifests`, `OriginEndpointName`, `ScheduleConfiguration` | `ClientToken` | `CreateHarvestJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new harvest job to export content from a MediaPackage v2 channel to an S3 bucket. |
| `CreateOriginEndpoint` | `POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint` | `idempotent`, `idempotency-token` | `ChannelGroupName`, `ChannelName`, `ContainerType`, `OriginEndpointName` | `ClientToken` | `CreateOriginEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The endpoint is attached to a channel, and represents the output of the live content. You can associate multiple endpoints to a single channel. |
| `DeleteChannel` | `DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/` | `idempotent` | `ChannelGroupName`, `ChannelName` | - | `DeleteChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Delete a channel to stop AWS Elemental MediaPackage from receiving further content. You must delete the channel's origin endpoints before you can delete the channel. |
| `DeleteChannelGroup` | `DELETE /channelGroup/{ChannelGroupName}` | `idempotent` | `ChannelGroupName` | - | `DeleteChannelGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Delete a channel group. You must delete the channel group's channels and origin endpoints before you can delete the channel group. |
| `DeleteChannelPolicy` | `DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/policy` | `idempotent` | `ChannelGroupName`, `ChannelName` | - | `DeleteChannelPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Delete a channel policy. |
| `DeleteOriginEndpoint` | `DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}` | `idempotent` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | - | `DeleteOriginEndpointResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Origin endpoints can serve content until they're deleted. Delete the endpoint if it should no longer respond to playback requests. |
| `DeleteOriginEndpointPolicy` | `DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/policy` | `idempotent` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | - | `DeleteOriginEndpointPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Delete an origin endpoint policy. |
| `GetChannel` | `GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/` | `readonly` | `ChannelGroupName`, `ChannelName` | - | `GetChannelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified channel that's configured in AWS Elemental MediaPackage. |
| `GetChannelGroup` | `GET /channelGroup/{ChannelGroupName}` | `readonly` | `ChannelGroupName` | - | `GetChannelGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified channel group that's configured in AWS Elemental MediaPackage. |
| `GetChannelPolicy` | `GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/policy` | `readonly` | `ChannelGroupName`, `ChannelName` | - | `GetChannelPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified channel policy that's configured in AWS Elemental MediaPackage. With policies, you can specify who has access to AWS resources and what actions they can perform on those resources. |
| `GetHarvestJob` | `GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob/{HarvestJobName}` | `readonly` | `ChannelGroupName`, `ChannelName`, `HarvestJobName`, `OriginEndpointName` | - | `GetHarvestJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of a specific harvest job. |
| `GetOriginEndpoint` | `GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}` | `readonly` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | - | `GetOriginEndpointResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified origin endpoint that's configured in AWS Elemental MediaPackage to obtain its playback URL and to view the packaging settings that it's currently using. |
| `GetOriginEndpointPolicy` | `GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/policy` | `readonly` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | - | `GetOriginEndpointPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified origin endpoint policy that's configured in AWS Elemental MediaPackage. |
| `ListChannelGroups` | `GET /channelGroup` | `readonly`, `paginated` | - | - | `ListChannelGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves all channel groups that are configured in Elemental MediaPackage. |
| `ListChannels` | `GET /channelGroup/{ChannelGroupName}/channel` | `readonly`, `paginated` | `ChannelGroupName` | - | `ListChannelsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves all channels in a specific channel group that are configured in AWS Elemental MediaPackage. |
| `ListHarvestJobs` | `GET /channelGroup/{ChannelGroupName}/harvestJob` | `readonly`, `paginated` | `ChannelGroupName` | - | `ListHarvestJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of harvest jobs that match the specified criteria. |
| `ListOriginEndpoints` | `GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint` | `readonly`, `paginated` | `ChannelGroupName`, `ChannelName` | - | `ListOriginEndpointsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves all origin endpoints in a specific channel that are configured in AWS Elemental MediaPackage. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ValidationException` | Lists the tags assigned to a resource. |
| `PutChannelPolicy` | `PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/policy` | `idempotent` | `ChannelGroupName`, `ChannelName`, `Policy` | - | `PutChannelPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches an IAM policy to the specified channel. With policies, you can specify who has access to AWS resources and what actions they can perform on those resources. |
| `PutOriginEndpointPolicy` | `POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/policy` | `idempotent` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName`, `Policy` | - | `PutOriginEndpointPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches an IAM policy to the specified origin endpoint. You can attach only one policy with each request. |
| `ResetChannelState` | `POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/reset` | `idempotent` | `ChannelGroupName`, `ChannelName` | - | `ResetChannelStateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Resetting the channel can help to clear errors from misconfigurations in the encoder. A reset refreshes the ingest stream and removes previous content. |
| `ResetOriginEndpointState` | `POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/reset` | `idempotent` | `ChannelGroupName`, `ChannelName`, `OriginEndpointName` | - | `ResetOriginEndpointStateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Resetting the origin endpoint can help to resolve unexpected behavior and other content packaging issues. It also helps to preserve special events when you don't want the previous content to be available for viewing. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `ValidationException` | Assigns one of more tags (key-value pairs) to the specified MediaPackage resource. Tags can help you organize and categorize your resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `ValidationException` | Removes one or more tags from the specified resource. |
| `UpdateChannel` | `PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/` | `idempotent` | `ChannelGroupName`, `ChannelName` | - | `UpdateChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the specified channel. You can edit if MediaPackage sends ingest or egress access logs to the CloudWatch log group, if content will be encrypted, the description on a channel, and your channel's policy settings. |
| `UpdateChannelGroup` | `PUT /channelGroup/{ChannelGroupName}` | `idempotent` | `ChannelGroupName` | - | `UpdateChannelGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the specified channel group. You can edit the description on a channel group for easier identification later from the AWS Elemental MediaPackage console. |
| `UpdateOriginEndpoint` | `PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}` | `idempotent` | `ChannelGroupName`, `ChannelName`, `ContainerType`, `OriginEndpointName` | - | `UpdateOriginEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update the specified origin endpoint. Edit the packaging preferences on an endpoint to optimize the viewing experience. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message`, `ValidationExceptionType` | The input failed to meet the constraints specified by the AWS service. |
| `AccessDeniedException` | `structure` | `Message` | Access is denied because either you don't have permissions to perform the requested operation or MediaPackage is getting throttling errors with CDN authorization. |
| `InternalServerException` | `structure` | `Message` | Indicates that an error from the service occurred while trying to process a request. |
| `ThrottlingException` | `structure` | `Message` | The request throughput limit was exceeded. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceTypeNotFound` | The specified resource doesn't exist. |
| `ConflictException` | `structure` | `ConflictExceptionType`, `Message` | Updating or deleting this resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The request would cause a service quota to be exceeded. |
| `CancelHarvestJobRequest` | `structure` | `ChannelGroupName`, `ChannelName`, `ETag`, `HarvestJobName`, `OriginEndpointName` | - |
| `CancelHarvestJobResponse` | `structure` | - | - |
| `CreateChannelRequest` | `structure` | `ChannelGroupName`, `ChannelName`, `ClientToken`, `Description`, `InputSwitchConfiguration`, `InputType`, `OutputHeaderConfiguration`, `Tags` | - |
| `CreateChannelResponse` | `structure` | `Arn`, `ChannelGroupName`, `ChannelName`, `CreatedAt`, `Description`, `ETag`, `IngestEndpoints`, `InputSwitchConfiguration`, `InputType`, `ModifiedAt`, `OutputHeaderConfiguration`, `Tags` | - |
| `CreateChannelGroupRequest` | `structure` | `ChannelGroupName`, `ClientToken`, `Description`, `Tags` | - |
| `CreateChannelGroupResponse` | `structure` | `Arn`, `ChannelGroupName`, `CreatedAt`, `Description`, `ETag`, `EgressDomain`, `ModifiedAt`, `Tags` | - |
| `CreateHarvestJobRequest` | `structure` | `ChannelGroupName`, `ChannelName`, `ClientToken`, `Description`, `Destination`, `HarvestJobName`, `HarvestedManifests`, `OriginEndpointName`, `ScheduleConfiguration`, `Tags` | The request object for creating a new harvest job. |
| `CreateHarvestJobResponse` | `structure` | `Arn`, `ChannelGroupName`, `ChannelName`, `CreatedAt`, `Description`, `Destination`, `ETag`, `ErrorMessage`, `HarvestJobName`, `HarvestedManifests`, `ModifiedAt`, `OriginEndpointName`, ... (+3) | The response object returned after creating a harvest job. |
| `CreateOriginEndpointRequest` | `structure` | `ChannelGroupName`, `ChannelName`, `ClientToken`, `ContainerType`, `DashManifests`, `Description`, `ForceEndpointErrorConfiguration`, `HlsManifests`, `LowLatencyHlsManifests`, `MssManifests`, `OriginEndpointName`, `Segment`, ... (+2) | - |
| `CreateOriginEndpointResponse` | `structure` | `Arn`, `ChannelGroupName`, `ChannelName`, `ContainerType`, `CreatedAt`, `DashManifests`, `Description`, `ETag`, `ForceEndpointErrorConfiguration`, `HlsManifests`, `LowLatencyHlsManifests`, `ModifiedAt`, ... (+5) | - |
| `DeleteChannelRequest` | `structure` | `ChannelGroupName`, `ChannelName` | - |
| `DeleteChannelResponse` | `structure` | - | - |
| `DeleteChannelGroupRequest` | `structure` | `ChannelGroupName` | - |
| `DeleteChannelGroupResponse` | `structure` | - | - |
| `DeleteChannelPolicyRequest` | `structure` | `ChannelGroupName`, `ChannelName` | - |
| `DeleteChannelPolicyResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

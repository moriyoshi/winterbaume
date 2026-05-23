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

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ValidationException` | Lists the tags assigned to a resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `ValidationException` | Assigns one of more tags (key-value pairs) to the specified MediaPackage resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permiss ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `ValidationException` | Removes one or more tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Access is denied because either you don't have permissions to perform the requested operation or MediaPackage is getting throttling errors with CDN authoriz ... |
| `ConflictException` | `structure` | Message, ConflictExceptionType | Updating or deleting this resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | Message | Indicates that an error from the service occurred while trying to process a request. |
| `ResourceNotFoundException` | `structure` | Message, ResourceTypeNotFound | The specified resource doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | Message | The request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | Message | The request throughput limit was exceeded. |
| `ValidationException` | `structure` | Message, ValidationExceptionType | The input failed to meet the constraints specified by the AWS service. |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `AdMarkerDash` | `enum` | BINARY, XML | - |
| `AdMarkerHls` | `enum` | DATERANGE, SCTE35_ENHANCED | - |
| `CmafEncryptionMethod` | `enum` | CENC, CBCS | - |
| `ConflictExceptionType` | `enum` | RESOURCE_IN_USE, RESOURCE_ALREADY_EXISTS, IDEMPOTENT_PARAMETER_MISMATCH, CONFLICTING_OPERATION | - |
| `ContainerType` | `enum` | TS, CMAF, ISM | - |
| `CustomAdType` | `enum` | PROGRAM, CHAPTER, UNSCHEDULED_EVENT, ALTERNATE_CONTENT_OPPORTUNITY, NETWORK | - |
| `DashCompactness` | `enum` | STANDARD, NONE | - |
| `DashDrmSignaling` | `enum` | INDIVIDUAL, REFERENCED | - |
| `DashPeriodTrigger` | `enum` | AVAILS, DRM_KEY_ROTATION, SOURCE_CHANGES, SOURCE_DISRUPTIONS, NONE | - |
| `DashProfile` | `enum` | DVB_DASH | - |
| `DashSegmentTemplateFormat` | `enum` | NUMBER_WITH_TIMELINE | - |
| `DashTtmlProfile` | `enum` | IMSC_1, EBU_TT_D_101 | - |
| `DashUtcTimingMode` | `enum` | HTTP_HEAD, HTTP_ISO, HTTP_XSDATE, UTC_DIRECT | - |
| `DrmSystem` | `enum` | CLEAR_KEY_AES_128, FAIRPLAY, PLAYREADY, WIDEVINE, IRDETO | - |
| `EndpointErrorCondition` | `enum` | STALE_MANIFEST, INCOMPLETE_MANIFEST, MISSING_DRM_KEY, SLATE_INPUT | - |
| `HarvestJobStatus` | `enum` | QUEUED, IN_PROGRESS, CANCELLED, COMPLETED, FAILED | - |
| `InputType` | `enum` | HLS, CMAF | - |
| `IsmEncryptionMethod` | `enum` | CENC | - |
| `MssManifestLayout` | `enum` | FULL, COMPACT | - |
| `PresetSpeke20Audio` | `enum` | PRESET_AUDIO_1, PRESET_AUDIO_2, PRESET_AUDIO_3, SHARED, UNENCRYPTED | - |
| `PresetSpeke20Video` | `enum` | PRESET_VIDEO_1, PRESET_VIDEO_2, PRESET_VIDEO_3, PRESET_VIDEO_4, PRESET_VIDEO_5, PRESET_VIDEO_6, PRESET_VIDEO_7, PRESET_VIDEO_8, SHARED, UNENCRYPTED | - |
| `ResourceTypeNotFound` | `enum` | CHANNEL_GROUP, CHANNEL, ORIGIN_ENDPOINT, HARVEST_JOB | - |
| `ScteFilter` | `enum` | SPLICE_INSERT, BREAK, PROVIDER_ADVERTISEMENT, DISTRIBUTOR_ADVERTISEMENT, PROVIDER_PLACEMENT_OPPORTUNITY, DISTRIBUTOR_PLACEMENT_OPPORTUNITY, PROVIDER_OVERLAY_PLACEMENT_OPPORTUNITY, DISTRIBUTOR_OVERLAY_PLACEMENT_OPPORTUNITY, PROGRAM, CHAPTER, UNSCHEDULED_EVENT, ALTERNATE_CONTENT_OPPORTUNITY, ... (+5) | - |
| `ScteInManifests` | `enum` | ALL, MATCHES_FILTER | - |
| `ScteInSegments` | `enum` | NONE, ALL, MATCHES_FILTER | - |
| `TsEncryptionMethod` | `enum` | AES_128, SAMPLE_AES | - |
| `UriPathType` | `enum` | LEAF, ROOT | - |
| `UriSeparator` | `enum` | UNDERSCORE, HYPHEN | - |
| `ValidationExceptionType` | `enum` | CONTAINER_TYPE_IMMUTABLE, INVALID_PAGINATION_TOKEN, INVALID_PAGINATION_MAX_RESULTS, INVALID_POLICY, INVALID_ROLE_ARN, MANIFEST_NAME_COLLISION, ENCRYPTION_METHOD_CONTAINER_TYPE_MISMATCH, CENC_IV_INCOMPATIBLE, ENCRYPTION_CONTRACT_WITHOUT_AUDIO_RENDITION_INCOMPATIBLE, ENCRYPTION_CONTRACT_WITH_ISM_CONTAINER_INCOMPATIBLE, ENCRYPTION_CONTRACT_UNENCRYPTED, ENCRYPTION_CONTRACT_SHARED, ... (+88) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Elemental MediaPackage

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Elemental MediaPackage

## Possible Usage Scenarios
- Backported from `crates/winterbaume-mediapackage/tests/scenario_test.rs`: set up a live-streaming ingest pipeline with channels, origin endpoints, describe/list, and cleanup.
- Backported from `scenario_test.rs`: keep multiple channels and endpoint pools isolated for independent content distribution.
- From the AWS documentation and model: model packaging channels, origin endpoints, harvest jobs, access logging, encryption/CDN settings, and live-content distribution lifecycle.

## Service Identity and Protocol

- AWS model slug: `mediapackage`
- AWS SDK for Rust slug: `mediapackage`
- Model version: `2017-10-12`
- Model file: `vendor/api-models-aws/models/mediapackage/service/2017-10-12/mediapackage-2017-10-12.json`
- SDK ID: `MediaPackage`
- Endpoint prefix: `mediapackage`
- ARN namespace: `mediapackage`
- CloudFormation name: `MediaPackage`
- CloudTrail event source: `mediapackage.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (3), `Describe` (3), `Delete` (2), `Rotate` (2), `Update` (2), `Configure` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateChannel`, `CreateHarvestJob`, `CreateOriginEndpoint`, `DeleteChannel`, `DeleteOriginEndpoint`, `TagResource`, `UntagResource`, `UpdateChannel`, `UpdateOriginEndpoint`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeChannel`, `DescribeHarvestJob`, `DescribeOriginEndpoint`, `ListChannels`, `ListHarvestJobs`, `ListOriginEndpoints`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateHarvestJob`, `DescribeHarvestJob`, `ListHarvestJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## v1/v2 State Coherence

- **Paired with `mediapackagev2` ( different SDK slug, different resource model ).** Despite the v1/v2 naming, AWS treats MediaPackage and MediaPackage v2 as **separate services with no shared backend**. The v2 docs explicitly state: "There isn't an automated process to migrate your resources from MediaPackage v1 to MediaPackage v2 [...] you can't use the MediaPackage v2 CLI or the MediaPackage v2 API to access any MediaPackage v1 resources."
- v1 owns `Channel` and `OriginEndpoint` keyed by name; v2 introduces `ChannelGroup` as the new top-level container plus channel/endpoint policies and harvest jobs. The two resource models do not overlap.
- **Current Winterbaume status: correctly separate.** `winterbaume-mediapackage` and `winterbaume-mediapackagev2` each own their own state, mirroring real AWS. No dependency between the crates is needed; do not introduce cross-API visibility.

## Operation Groups

### List

- Operations: `ListChannels`, `ListHarvestJobs`, `ListOriginEndpoints`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Create

- Operations: `CreateChannel`, `CreateHarvestJob`, `CreateOriginEndpoint`
- Common required input members in this group: `Id`

### Describe

- Operations: `DescribeChannel`, `DescribeHarvestJob`, `DescribeOriginEndpoint`
- Common required input members in this group: `Id`

### Delete

- Operations: `DeleteChannel`, `DeleteOriginEndpoint`
- Common required input members in this group: `Id`

### Rotate

- Operations: `RotateChannelCredentials`, `RotateIngestEndpointCredentials`
- Common required input members in this group: `Id`

### Update

- Operations: `UpdateChannel`, `UpdateOriginEndpoint`
- Common required input members in this group: `Id`

### Configure

- Operations: `ConfigureLogs`
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
| `ConfigureLogs` | `PUT /channels/{Id}/configure_logs` | - | `Id` | - | `ConfigureLogsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Changes the Channel's properities to configure log subscription |
| `CreateChannel` | `POST /channels` | - | `Id` | - | `CreateChannelResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new Channel. |
| `CreateHarvestJob` | `POST /harvest_jobs` | - | `EndTime`, `Id`, `OriginEndpointId`, `S3Destination`, `StartTime` | - | `CreateHarvestJobResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new HarvestJob record. |
| `CreateOriginEndpoint` | `POST /origin_endpoints` | - | `ChannelId`, `Id` | - | `CreateOriginEndpointResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new OriginEndpoint record. |
| `DeleteChannel` | `DELETE /channels/{Id}` | - | `Id` | - | `DeleteChannelResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Deletes an existing Channel. |
| `DeleteOriginEndpoint` | `DELETE /origin_endpoints/{Id}` | - | `Id` | - | `DeleteOriginEndpointResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Deletes an existing OriginEndpoint. |
| `DescribeChannel` | `GET /channels/{Id}` | - | `Id` | - | `DescribeChannelResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Gets details about a Channel. |
| `DescribeHarvestJob` | `GET /harvest_jobs/{Id}` | - | `Id` | - | `DescribeHarvestJobResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Gets details about an existing HarvestJob. |
| `DescribeOriginEndpoint` | `GET /origin_endpoints/{Id}` | - | `Id` | - | `DescribeOriginEndpointResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Gets details about an existing OriginEndpoint. |
| `ListChannels` | `GET /channels` | `paginated` | - | - | `ListChannelsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a collection of Channels. |
| `ListHarvestJobs` | `GET /harvest_jobs` | `paginated` | - | - | `ListHarvestJobsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a collection of HarvestJob records. |
| `ListOriginEndpoints` | `GET /origin_endpoints` | `paginated` | - | - | `ListOriginEndpointsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Returns a collection of OriginEndpoint records. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | - | - |
| `RotateChannelCredentials` | `PUT /channels/{Id}/credentials` | - | `Id` | - | `RotateChannelCredentialsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Changes the Channel's first IngestEndpoint's username and password. WARNING - This API is deprecated. Please use RotateIngestEndpointCredentials instead |
| `RotateIngestEndpointCredentials` | `PUT /channels/{Id}/ingest_endpoints/{IngestEndpointId}/credentials` | - | `Id`, `IngestEndpointId` | - | `RotateIngestEndpointCredentialsResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Rotate the IngestEndpoint's username and password, as specified by the IngestEndpoint's id. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | - | - |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | - | - |
| `UpdateChannel` | `PUT /channels/{Id}` | - | `Id` | - | `UpdateChannelResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Updates an existing Channel. |
| `UpdateOriginEndpoint` | `PUT /origin_endpoints/{Id}` | - | `Id` | - | `UpdateOriginEndpointResponse` | `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnprocessableEntityException` | Updates an existing OriginEndpoint. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListChannels` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListHarvestJobs` | - | `IncludeChannelId -> includeChannelId`, `IncludeStatus -> includeStatus`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListOriginEndpoints` | - | `ChannelId -> channelId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ForbiddenException` | `structure` | Message | The client is not authorized to access the requested resource. |
| `InternalServerErrorException` | `structure` | Message | An unexpected error occurred. |
| `NotFoundException` | `structure` | Message | The requested resource does not exist. |
| `ServiceUnavailableException` | `structure` | Message | An unexpected error occurred. |
| `TooManyRequestsException` | `structure` | Message | The client has exceeded their resource or throttling limits. |
| `UnprocessableEntityException` | `structure` | Message | The parameters sent in the request are not valid. |
| `ConfigureLogsRequest` | `structure` | EgressAccessLogs, Id, IngressAccessLogs | the option to configure log subscription. |
| `ConfigureLogsResponse` | `structure` | Arn, CreatedAt, Description, EgressAccessLogs, HlsIngest, Id, IngressAccessLogs, Tags | - |
| `CreateChannelRequest` | `structure` | Description, Id, Tags | A new Channel configuration. |
| `CreateChannelResponse` | `structure` | Arn, CreatedAt, Description, EgressAccessLogs, HlsIngest, Id, IngressAccessLogs, Tags | - |
| `CreateHarvestJobRequest` | `structure` | EndTime, Id, OriginEndpointId, S3Destination, StartTime | Configuration parameters used to create a new HarvestJob. |
| `CreateHarvestJobResponse` | `structure` | Arn, ChannelId, CreatedAt, EndTime, Id, OriginEndpointId, S3Destination, StartTime, Status | - |
| `CreateOriginEndpointRequest` | `structure` | Authorization, ChannelId, CmafPackage, DashPackage, Description, HlsPackage, Id, ManifestName, MssPackage, Origination, StartoverWindowSeconds, Tags, ... (+2) | Configuration parameters used to create a new OriginEndpoint. |
| `CreateOriginEndpointResponse` | `structure` | Arn, Authorization, ChannelId, CmafPackage, CreatedAt, DashPackage, Description, HlsPackage, Id, ManifestName, MssPackage, Origination, ... (+5) | - |
| `DeleteChannelRequest` | `structure` | Id | - |
| `DeleteChannelResponse` | `structure` | **empty (no members)** | - |
| `DeleteOriginEndpointRequest` | `structure` | Id | - |
| `DeleteOriginEndpointResponse` | `structure` | **empty (no members)** | - |
| `DescribeChannelRequest` | `structure` | Id | - |
| `DescribeChannelResponse` | `structure` | Arn, CreatedAt, Description, EgressAccessLogs, HlsIngest, Id, IngressAccessLogs, Tags | - |
| `DescribeHarvestJobRequest` | `structure` | Id | - |
| `DescribeHarvestJobResponse` | `structure` | Arn, ChannelId, CreatedAt, EndTime, Id, OriginEndpointId, S3Destination, StartTime, Status | - |
| `DescribeOriginEndpointRequest` | `structure` | Id | - |
| `DescribeOriginEndpointResponse` | `structure` | Arn, Authorization, ChannelId, CmafPackage, CreatedAt, DashPackage, Description, HlsPackage, Id, ManifestName, MssPackage, Origination, ... (+5) | - |
| `ListChannelsRequest` | `structure` | MaxResults, NextToken | - |
| `ListChannelsResponse` | `structure` | Channels, NextToken | - |
| `ListHarvestJobsRequest` | `structure` | IncludeChannelId, IncludeStatus, MaxResults, NextToken | - |
| `ListHarvestJobsResponse` | `structure` | HarvestJobs, NextToken | - |
| `ListOriginEndpointsRequest` | `structure` | ChannelId, MaxResults, NextToken | - |
| `ListOriginEndpointsResponse` | `structure` | NextToken, OriginEndpoints | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `RotateChannelCredentialsRequest` | `structure` | Id | - |
| `RotateChannelCredentialsResponse` | `structure` | Arn, CreatedAt, Description, EgressAccessLogs, HlsIngest, Id, IngressAccessLogs, Tags | - |
| `RotateIngestEndpointCredentialsRequest` | `structure` | Id, IngestEndpointId | - |
| `RotateIngestEndpointCredentialsResponse` | `structure` | Arn, CreatedAt, Description, EgressAccessLogs, HlsIngest, Id, IngressAccessLogs, Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UpdateChannelRequest` | `structure` | Description, Id | Configuration parameters used to update the Channel. |
| `UpdateChannelResponse` | `structure` | Arn, CreatedAt, Description, EgressAccessLogs, HlsIngest, Id, IngressAccessLogs, Tags | - |
| `AdMarkers` | `enum` | NONE, SCTE35_ENHANCED, PASSTHROUGH, DATERANGE | - |
| `AdsOnDeliveryRestrictions` | `enum` | NONE, RESTRICTED, UNRESTRICTED, BOTH | This setting allows the delivery restriction flags on SCTE-35 segmentation descriptors to determine whether a message signals an ad. Choosing "NONE" means n ... |
| `CmafEncryptionMethod` | `enum` | SAMPLE_AES, AES_CTR | The encryption method to use. |
| `EncryptionMethod` | `enum` | AES_128, SAMPLE_AES | - |
| `ManifestLayout` | `enum` | FULL, COMPACT, DRM_TOP_LEVEL_COMPACT | - |
| `Origination` | `enum` | ALLOW, DENY | - |
| `PlaylistType` | `enum` | NONE, EVENT, VOD | - |
| `PresetSpeke20Audio` | `enum` | PRESET_AUDIO_1, PRESET_AUDIO_2, PRESET_AUDIO_3, SHARED, UNENCRYPTED | - |
| `PresetSpeke20Video` | `enum` | PRESET_VIDEO_1, PRESET_VIDEO_2, PRESET_VIDEO_3, PRESET_VIDEO_4, PRESET_VIDEO_5, PRESET_VIDEO_6, PRESET_VIDEO_7, PRESET_VIDEO_8, SHARED, UNENCRYPTED | - |
| `Profile` | `enum` | NONE, HBBTV_1_5, HYBRIDCAST, DVB_DASH_2014 | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

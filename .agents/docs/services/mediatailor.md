# AWS MediaTailor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use the AWS Elemental MediaTailor SDKs and CLI to configure scalable ad insertion and linear channels. With MediaTailor, you can assemble existing content into a linear stream and serve targeted ads to viewers while maintaining broadcast quality in over-the-top (OTT) video applications. For information about using the service, including detailed information about the settings covered in this guide, see the AWS Elemental MediaTailor User Guide. Through the SDKs and the CLI you manage AWS Elemental MediaTailor configurations and channels the same as you do through the console. For example, you specify ad insertion behavior and mapping information for the origin server and the ad decision server (ADS).

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS MediaTailor resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS MediaTailor workflows in the local mock. Key resources include `ChannelPolicy`, `ChannelResource`, `LiveSourceResource`, `PlaybackConfigurationResource`, `PrefetchScheduleResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `List`, `Create`, `Describe`, `Update` operation families, including `DeleteChannel`, `DeleteChannelPolicy`, `DeleteLiveSource`, `DeletePlaybackConfiguration`, `ListAlerts`, `ListChannels`.

## Service Identity and Protocol

- AWS model slug: `mediatailor`
- AWS SDK for Rust slug: `mediatailor`
- Model version: `2018-04-23`
- Model file: `vendor/api-models-aws/models/mediatailor/service/2018-04-23/mediatailor-2018-04-23.json`
- SDK ID: `MediaTailor`
- Endpoint prefix: `api.mediatailor`
- ARN namespace: `mediatailor`
- CloudFormation name: `MediaTailor`
- CloudTrail event source: `mediatailor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (8), `List` (8), `Create` (6), `Describe` (5), `Update` (5), `Get` (4), `Configure` (2), `Put` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateChannel`, `CreateLiveSource`, `CreatePrefetchSchedule`, `CreateProgram`, `CreateSourceLocation`, `CreateVodSource`, `DeleteChannel`, `DeleteChannelPolicy`, `DeleteLiveSource`, `DeletePlaybackConfiguration`, `DeletePrefetchSchedule`, `DeleteProgram`, `DeleteSourceLocation`, `DeleteVodSource`, `PutChannelPolicy`, `PutPlaybackConfiguration`, `StartChannel`, `StopChannel`, `TagResource`, `UntagResource`, ... (+5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeChannel`, `DescribeLiveSource`, `DescribeProgram`, `DescribeSourceLocation`, `DescribeVodSource`, `GetChannelPolicy`, `GetChannelSchedule`, `GetPlaybackConfiguration`, `GetPrefetchSchedule`, `ListAlerts`, `ListChannels`, `ListLiveSources`, `ListPlaybackConfigurations`, `ListPrefetchSchedules`, `ListSourceLocations`, `ListTagsForResource`, `ListVodSources`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 25 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartChannel`, `StopChannel`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ChannelPolicy` | `ChannelName` | put: `PutChannelPolicy`; read: `GetChannelPolicy`; delete: `DeleteChannelPolicy` | - | - |
| `ChannelResource` | `ChannelName` | put: `CreateChannel`; read: `DescribeChannel`; update: `UpdateChannel`; delete: `DeleteChannel`; list: `ListChannels` | `ConfigureLogsForChannel`, `CreateChannel`, `GetChannelSchedule`, `StartChannel`, `StopChannel` | - |
| `LiveSourceResource` | `LiveSourceName`, `SourceLocationName` | put: `CreateLiveSource`; read: `DescribeLiveSource`; update: `UpdateLiveSource`; delete: `DeleteLiveSource`; list: `ListLiveSources` | - | - |
| `PlaybackConfigurationResource` | `Name` | put: `PutPlaybackConfiguration`; read: `GetPlaybackConfiguration`; delete: `DeletePlaybackConfiguration`; list: `ListPlaybackConfigurations` | - | - |
| `PrefetchScheduleResource` | `Name`, `PlaybackConfigurationName` | put: `CreatePrefetchSchedule`; read: `GetPrefetchSchedule`; delete: `DeletePrefetchSchedule`; list: `ListPrefetchSchedules` | - | - |
| `ProgramResource` | `ChannelName`, `ProgramName` | put: `CreateProgram`; read: `DescribeProgram`; update: `UpdateProgram`; delete: `DeleteProgram` | - | - |
| `SourceLocationResource` | `SourceLocationName` | put: `CreateSourceLocation`; read: `DescribeSourceLocation`; update: `UpdateSourceLocation`; delete: `DeleteSourceLocation`; list: `ListSourceLocations` | - | - |
| `VodSourceResource` | `SourceLocationName`, `VodSourceName` | put: `CreateVodSource`; read: `DescribeVodSource`; update: `UpdateVodSource`; delete: `DeleteVodSource`; list: `ListVodSources` | - | - |
## Operation Groups

### List

- Operations: `ListAlerts`, `ListTagsForResource`
- Traits: `readonly` (2), `paginated` (1)
- Common required input members in this group: `ResourceArn`

### Configure

- Operations: `ConfigureLogsForPlaybackConfiguration`
- Traits: `idempotent` (1)
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
| `ConfigureLogsForPlaybackConfiguration` | `PUT /configureLogs/playbackConfiguration` | `idempotent` | `PercentEnabled`, `PlaybackConfigurationName` | - | `ConfigureLogsForPlaybackConfigurationResponse` | - | Defines where AWS Elemental MediaTailor sends logs for the playback configuration. |
| `ListAlerts` | `GET /alerts` | `readonly`, `paginated` | `ResourceArn` | - | `ListAlertsResponse` | - | Lists the alerts that are associated with a MediaTailor channel assembly resource. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException` | A list of tags that are associated with this resource. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. For more information, see Tag ... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException` | The resource to tag. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. For more information, see Tagging AWS Elemental MediaTailor Res ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException` | The resource to untag. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListAlerts` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `ResourceArn -> resourceArn` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Message | A request contains unexpected data. |
| `ConfigureLogsForPlaybackConfigurationRequest` | `structure` | PercentEnabled, PlaybackConfigurationName, EnabledLoggingStrategies, AdsInteractionLog, ManifestServiceInteractionLog | Configures Amazon CloudWatch log settings for a playback configuration. |
| `ConfigureLogsForPlaybackConfigurationResponse` | `structure` | PercentEnabled, PlaybackConfigurationName, EnabledLoggingStrategies, AdsInteractionLog, ManifestServiceInteractionLog | - |
| `ListAlertsRequest` | `structure` | MaxResults, NextToken, ResourceArn | - |
| `ListAlertsResponse` | `structure` | Items, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `AccessType` | `enum` | S3_SIGV4, SECRETS_MANAGER_ACCESS_TOKEN, AUTODETECT_SIGV4 | - |
| `AdMarkupType` | `enum` | DATERANGE, SCTE35_ENHANCED | - |
| `AdsInteractionExcludeEventType` | `enum` | AD_MARKER_FOUND, NON_AD_MARKER_FOUND, MAKING_ADS_REQUEST, MODIFIED_TARGET_URL, VAST_REDIRECT, EMPTY_VAST_RESPONSE, EMPTY_VMAP_RESPONSE, VAST_RESPONSE, REDIRECTED_VAST_RESPONSE, FILLED_AVAIL, FILLED_OVERLAY_AVAIL, BEACON_FIRED, ... (+29) | - |
| `AdsInteractionPublishOptInEventType` | `enum` | RAW_ADS_RESPONSE, RAW_ADS_REQUEST | - |
| `AlertCategory` | `enum` | SCHEDULING_ERROR, PLAYBACK_WARNING, INFO | - |
| `ChannelState` | `enum` | RUNNING, STOPPED | - |
| `CompressionMethod` | `enum` | NONE, GZIP | - |
| `FillPolicy` | `enum` | FULL_AVAIL_ONLY, PARTIAL_AVAIL | - |
| `InsertionMode` | `enum` | STITCHED_ONLY, PLAYER_SELECT | Insertion Mode controls whether players can use stitched or guided ad insertion. |
| `ListPrefetchScheduleType` | `enum` | SINGLE, RECURRING, ALL | - |
| `LogType` | `enum` | AS_RUN | - |
| `LoggingStrategy` | `enum` | VENDED_LOGS, LEGACY_CLOUDWATCH | - |
| `ManifestServiceExcludeEventType` | `enum` | GENERATED_MANIFEST, ORIGIN_MANIFEST, SESSION_INITIALIZED, TRACKING_RESPONSE, CONFIG_SYNTAX_ERROR, CONFIG_SECURITY_ERROR, UNKNOWN_HOST, TIMEOUT_ERROR, CONNECTION_ERROR, IO_ERROR, UNKNOWN_ERROR, HOST_DISALLOWED, ... (+20) | - |
| `MessageType` | `enum` | SPLICE_INSERT, TIME_SIGNAL | - |
| `Method` | `enum` | GET, POST | - |
| `Mode` | `enum` | OFF, BEHIND_LIVE_EDGE, AFTER_LIVE_EDGE | - |
| `Operator` | `enum` | EQUALS | - |
| `OriginManifestType` | `enum` | SINGLE_PERIOD, MULTI_PERIOD | - |
| `PlaybackMode` | `enum` | LOOP, LINEAR | - |
| `PrefetchScheduleType` | `enum` | SINGLE, RECURRING | - |
| `RelativePosition` | `enum` | BEFORE_PROGRAM, AFTER_PROGRAM | - |
| `ScheduleEntryType` | `enum` | PROGRAM, FILLER_SLATE, ALTERNATE_MEDIA | - |
| `StreamingMediaFileConditioning` | `enum` | TRANSCODE, NONE | - |
| `Tier` | `enum` | BASIC, STANDARD | - |
| `TrafficShapingType` | `enum` | RETRIEVAL_WINDOW, TPS | - |
| `Type` | `enum` | DASH, HLS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

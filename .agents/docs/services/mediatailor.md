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

### Delete

- Operations: `DeleteChannel`, `DeleteChannelPolicy`, `DeleteLiveSource`, `DeletePlaybackConfiguration`, `DeletePrefetchSchedule`, `DeleteProgram`, `DeleteSourceLocation`, `DeleteVodSource`
- Traits: `idempotent` (8)
- Common required input members in this group: `ChannelName`, `LiveSourceName`, `Name`, `PlaybackConfigurationName`, `ProgramName`, `SourceLocationName`, `VodSourceName`

### List

- Operations: `ListAlerts`, `ListChannels`, `ListLiveSources`, `ListPlaybackConfigurations`, `ListPrefetchSchedules`, `ListSourceLocations`, `ListTagsForResource`, `ListVodSources`
- Traits: `paginated` (7), `readonly` (8)
- Common required input members in this group: `PlaybackConfigurationName`, `ResourceArn`, `SourceLocationName`

### Create

- Operations: `CreateChannel`, `CreateLiveSource`, `CreatePrefetchSchedule`, `CreateProgram`, `CreateSourceLocation`, `CreateVodSource`
- Traits: `idempotent` (6)
- Common required input members in this group: `ChannelName`, `HttpConfiguration`, `HttpPackageConfigurations`, `LiveSourceName`, `Name`, `Outputs`, `PlaybackConfigurationName`, `PlaybackMode`, `ProgramName`, `ScheduleConfiguration`, `SourceLocationName`, `VodSourceName`

### Describe

- Operations: `DescribeChannel`, `DescribeLiveSource`, `DescribeProgram`, `DescribeSourceLocation`, `DescribeVodSource`
- Traits: `readonly` (5)
- Common required input members in this group: `ChannelName`, `LiveSourceName`, `ProgramName`, `SourceLocationName`, `VodSourceName`

### Update

- Operations: `UpdateChannel`, `UpdateLiveSource`, `UpdateProgram`, `UpdateSourceLocation`, `UpdateVodSource`
- Traits: `idempotent` (5)
- Common required input members in this group: `ChannelName`, `HttpConfiguration`, `HttpPackageConfigurations`, `LiveSourceName`, `Outputs`, `ProgramName`, `ScheduleConfiguration`, `SourceLocationName`, `VodSourceName`

### Get

- Operations: `GetChannelPolicy`, `GetChannelSchedule`, `GetPlaybackConfiguration`, `GetPrefetchSchedule`
- Traits: `paginated` (1), `readonly` (4)
- Common required input members in this group: `ChannelName`, `Name`, `PlaybackConfigurationName`

### Configure

- Operations: `ConfigureLogsForChannel`, `ConfigureLogsForPlaybackConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChannelName`, `LogTypes`, `PercentEnabled`, `PlaybackConfigurationName`

### Put

- Operations: `PutChannelPolicy`, `PutPlaybackConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `ChannelName`, `Name`, `Policy`

### Start

- Operations: `StartChannel`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChannelName`

### Stop

- Operations: `StopChannel`
- Traits: `idempotent` (1)
- Common required input members in this group: `ChannelName`

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
| `ConfigureLogsForChannel` | `PUT /configureLogs/channel` | - | `ChannelName`, `LogTypes` | - | `ConfigureLogsForChannelResponse` | - | Configures Amazon CloudWatch log settings for a channel. |
| `ConfigureLogsForPlaybackConfiguration` | `PUT /configureLogs/playbackConfiguration` | `idempotent` | `PercentEnabled`, `PlaybackConfigurationName` | - | `ConfigureLogsForPlaybackConfigurationResponse` | - | Defines where AWS Elemental MediaTailor sends logs for the playback configuration. |
| `CreateChannel` | `POST /channel/{ChannelName}` | `idempotent` | `ChannelName`, `Outputs`, `PlaybackMode` | - | `CreateChannelResponse` | - | Creates a channel. For information about MediaTailor channels, see Working with channels in the MediaTailor User Guide . |
| `CreateLiveSource` | `POST /sourceLocation/{SourceLocationName}/liveSource/{LiveSourceName}` | `idempotent` | `HttpPackageConfigurations`, `LiveSourceName`, `SourceLocationName` | - | `CreateLiveSourceResponse` | - | The live source configuration. |
| `CreatePrefetchSchedule` | `POST /prefetchSchedule/{PlaybackConfigurationName}/{Name}` | `idempotent` | `Name`, `PlaybackConfigurationName` | - | `CreatePrefetchScheduleResponse` | - | Creates a prefetch schedule for a playback configuration. A prefetch schedule allows you to tell MediaTailor to fetch and prepare certain ads before an ad break happens. |
| `CreateProgram` | `POST /channel/{ChannelName}/program/{ProgramName}` | `idempotent` | `ChannelName`, `ProgramName`, `ScheduleConfiguration`, `SourceLocationName` | - | `CreateProgramResponse` | - | Creates a program within a channel. For information about programs, see Working with programs in the MediaTailor User Guide . |
| `CreateSourceLocation` | `POST /sourceLocation/{SourceLocationName}` | `idempotent` | `HttpConfiguration`, `SourceLocationName` | - | `CreateSourceLocationResponse` | - | Creates a source location. A source location is a container for sources. |
| `CreateVodSource` | `POST /sourceLocation/{SourceLocationName}/vodSource/{VodSourceName}` | `idempotent` | `HttpPackageConfigurations`, `SourceLocationName`, `VodSourceName` | - | `CreateVodSourceResponse` | - | The VOD source configuration parameters. |
| `DeleteChannel` | `DELETE /channel/{ChannelName}` | `idempotent` | `ChannelName` | - | `DeleteChannelResponse` | - | Deletes a channel. For information about MediaTailor channels, see Working with channels in the MediaTailor User Guide . |
| `DeleteChannelPolicy` | `DELETE /channel/{ChannelName}/policy` | `idempotent` | `ChannelName` | - | `DeleteChannelPolicyResponse` | - | The channel policy to delete. |
| `DeleteLiveSource` | `DELETE /sourceLocation/{SourceLocationName}/liveSource/{LiveSourceName}` | `idempotent` | `LiveSourceName`, `SourceLocationName` | - | `DeleteLiveSourceResponse` | - | The live source to delete. |
| `DeletePlaybackConfiguration` | `DELETE /playbackConfiguration/{Name}` | `idempotent` | `Name` | - | `DeletePlaybackConfigurationResponse` | - | Deletes a playback configuration. For information about MediaTailor configurations, see Working with configurations in AWS Elemental MediaTailor. |
| `DeletePrefetchSchedule` | `DELETE /prefetchSchedule/{PlaybackConfigurationName}/{Name}` | `idempotent` | `Name`, `PlaybackConfigurationName` | - | `DeletePrefetchScheduleResponse` | - | Deletes a prefetch schedule for a specific playback configuration. If you call `DeletePrefetchSchedule` on an expired prefetch schedule, MediaTailor returns an HTTP 404 status code. |
| `DeleteProgram` | `DELETE /channel/{ChannelName}/program/{ProgramName}` | `idempotent` | `ChannelName`, `ProgramName` | - | `DeleteProgramResponse` | - | Deletes a program within a channel. For information about programs, see Working with programs in the MediaTailor User Guide . |
| `DeleteSourceLocation` | `DELETE /sourceLocation/{SourceLocationName}` | `idempotent` | `SourceLocationName` | - | `DeleteSourceLocationResponse` | - | Deletes a source location. A source location is a container for sources. |
| `DeleteVodSource` | `DELETE /sourceLocation/{SourceLocationName}/vodSource/{VodSourceName}` | `idempotent` | `SourceLocationName`, `VodSourceName` | - | `DeleteVodSourceResponse` | - | The video on demand (VOD) source to delete. |
| `DescribeChannel` | `GET /channel/{ChannelName}` | `readonly` | `ChannelName` | - | `DescribeChannelResponse` | - | Describes a channel. For information about MediaTailor channels, see Working with channels in the MediaTailor User Guide . |
| `DescribeLiveSource` | `GET /sourceLocation/{SourceLocationName}/liveSource/{LiveSourceName}` | `readonly` | `LiveSourceName`, `SourceLocationName` | - | `DescribeLiveSourceResponse` | - | The live source to describe. |
| `DescribeProgram` | `GET /channel/{ChannelName}/program/{ProgramName}` | `readonly` | `ChannelName`, `ProgramName` | - | `DescribeProgramResponse` | - | Describes a program within a channel. For information about programs, see Working with programs in the MediaTailor User Guide . |
| `DescribeSourceLocation` | `GET /sourceLocation/{SourceLocationName}` | `readonly` | `SourceLocationName` | - | `DescribeSourceLocationResponse` | - | Describes a source location. A source location is a container for sources. |
| `DescribeVodSource` | `GET /sourceLocation/{SourceLocationName}/vodSource/{VodSourceName}` | `readonly` | `SourceLocationName`, `VodSourceName` | - | `DescribeVodSourceResponse` | - | Provides details about a specific video on demand (VOD) source in a specific source location. |
| `GetChannelPolicy` | `GET /channel/{ChannelName}/policy` | `readonly` | `ChannelName` | - | `GetChannelPolicyResponse` | - | Returns the channel's IAM policy. IAM policies are used to control access to your channel. |
| `GetChannelSchedule` | `GET /channel/{ChannelName}/schedule` | `readonly`, `paginated` | `ChannelName` | - | `GetChannelScheduleResponse` | - | Retrieves information about your channel's schedule. |
| `GetPlaybackConfiguration` | `GET /playbackConfiguration/{Name}` | `readonly` | `Name` | - | `GetPlaybackConfigurationResponse` | - | Retrieves a playback configuration. For information about MediaTailor configurations, see Working with configurations in AWS Elemental MediaTailor. |
| `GetPrefetchSchedule` | `GET /prefetchSchedule/{PlaybackConfigurationName}/{Name}` | `readonly` | `Name`, `PlaybackConfigurationName` | - | `GetPrefetchScheduleResponse` | - | Retrieves a prefetch schedule for a playback configuration. A prefetch schedule allows you to tell MediaTailor to fetch and prepare certain ads before an ad break happens. |
| `ListAlerts` | `GET /alerts` | `readonly`, `paginated` | `ResourceArn` | - | `ListAlertsResponse` | - | Lists the alerts that are associated with a MediaTailor channel assembly resource. |
| `ListChannels` | `GET /channels` | `readonly`, `paginated` | - | - | `ListChannelsResponse` | - | Retrieves information about the channels that are associated with the current AWS account. |
| `ListLiveSources` | `GET /sourceLocation/{SourceLocationName}/liveSources` | `readonly`, `paginated` | `SourceLocationName` | - | `ListLiveSourcesResponse` | - | Lists the live sources contained in a source location. A source represents a piece of content. |
| `ListPlaybackConfigurations` | `GET /playbackConfigurations` | `readonly`, `paginated` | - | - | `ListPlaybackConfigurationsResponse` | - | Retrieves existing playback configurations. For information about MediaTailor configurations, see Working with Configurations in AWS Elemental MediaTailor. |
| `ListPrefetchSchedules` | `POST /prefetchSchedule/{PlaybackConfigurationName}` | `readonly`, `paginated` | `PlaybackConfigurationName` | - | `ListPrefetchSchedulesResponse` | - | Lists the prefetch schedules for a playback configuration. |
| `ListSourceLocations` | `GET /sourceLocations` | `readonly`, `paginated` | - | - | `ListSourceLocationsResponse` | - | Lists the source locations for a channel. A source location defines the host server URL, and contains a list of sources. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException` | A list of tags that are associated with this resource. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. |
| `ListVodSources` | `GET /sourceLocation/{SourceLocationName}/vodSources` | `readonly`, `paginated` | `SourceLocationName` | - | `ListVodSourcesResponse` | - | Lists the VOD sources contained in a source location. A source represents a piece of content. |
| `PutChannelPolicy` | `PUT /channel/{ChannelName}/policy` | `idempotent` | `ChannelName`, `Policy` | - | `PutChannelPolicyResponse` | - | Creates an IAM policy for the channel. IAM policies are used to control access to your channel. |
| `PutPlaybackConfiguration` | `PUT /playbackConfiguration` | `idempotent` | `Name` | - | `PutPlaybackConfigurationResponse` | - | Creates a playback configuration. For information about MediaTailor configurations, see Working with configurations in AWS Elemental MediaTailor. |
| `StartChannel` | `PUT /channel/{ChannelName}/start` | `idempotent` | `ChannelName` | - | `StartChannelResponse` | - | Starts a channel. For information about MediaTailor channels, see Working with channels in the MediaTailor User Guide . |
| `StopChannel` | `PUT /channel/{ChannelName}/stop` | `idempotent` | `ChannelName` | - | `StopChannelResponse` | - | Stops a channel. For information about MediaTailor channels, see Working with channels in the MediaTailor User Guide . |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException` | The resource to tag. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException` | The resource to untag. |
| `UpdateChannel` | `PUT /channel/{ChannelName}` | `idempotent` | `ChannelName`, `Outputs` | - | `UpdateChannelResponse` | - | Updates a channel. For information about MediaTailor channels, see Working with channels in the MediaTailor User Guide . |
| `UpdateLiveSource` | `PUT /sourceLocation/{SourceLocationName}/liveSource/{LiveSourceName}` | `idempotent` | `HttpPackageConfigurations`, `LiveSourceName`, `SourceLocationName` | - | `UpdateLiveSourceResponse` | - | Updates a live source's configuration. |
| `UpdateProgram` | `PUT /channel/{ChannelName}/program/{ProgramName}` | `idempotent` | `ChannelName`, `ProgramName`, `ScheduleConfiguration` | - | `UpdateProgramResponse` | - | Updates a program within a channel. |
| `UpdateSourceLocation` | `PUT /sourceLocation/{SourceLocationName}` | `idempotent` | `HttpConfiguration`, `SourceLocationName` | - | `UpdateSourceLocationResponse` | - | Updates a source location. A source location is a container for sources. |
| `UpdateVodSource` | `PUT /sourceLocation/{SourceLocationName}/vodSource/{VodSourceName}` | `idempotent` | `HttpPackageConfigurations`, `SourceLocationName`, `VodSourceName` | - | `UpdateVodSourceResponse` | - | Updates a VOD source's configuration. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | A request contains unexpected data. |
| `ConfigureLogsForChannelRequest` | `structure` | `ChannelName`, `LogTypes` | - |
| `ConfigureLogsForChannelResponse` | `structure` | `ChannelName`, `LogTypes` | - |
| `ConfigureLogsForPlaybackConfigurationRequest` | `structure` | `AdsInteractionLog`, `EnabledLoggingStrategies`, `ManifestServiceInteractionLog`, `PercentEnabled`, `PlaybackConfigurationName` | Configures Amazon CloudWatch log settings for a playback configuration. |
| `ConfigureLogsForPlaybackConfigurationResponse` | `structure` | `AdsInteractionLog`, `EnabledLoggingStrategies`, `ManifestServiceInteractionLog`, `PercentEnabled`, `PlaybackConfigurationName` | - |
| `CreateChannelRequest` | `structure` | `Audiences`, `ChannelName`, `FillerSlate`, `Outputs`, `PlaybackMode`, `Tags`, `Tier`, `TimeShiftConfiguration` | - |
| `CreateChannelResponse` | `structure` | `Arn`, `Audiences`, `ChannelName`, `ChannelState`, `CreationTime`, `FillerSlate`, `LastModifiedTime`, `Outputs`, `PlaybackMode`, `Tags`, `Tier`, `TimeShiftConfiguration` | - |
| `CreateLiveSourceRequest` | `structure` | `HttpPackageConfigurations`, `LiveSourceName`, `SourceLocationName`, `Tags` | - |
| `CreateLiveSourceResponse` | `structure` | `Arn`, `CreationTime`, `HttpPackageConfigurations`, `LastModifiedTime`, `LiveSourceName`, `SourceLocationName`, `Tags` | - |
| `CreatePrefetchScheduleRequest` | `structure` | `Consumption`, `Name`, `PlaybackConfigurationName`, `RecurringPrefetchConfiguration`, `Retrieval`, `ScheduleType`, `StreamId` | - |
| `CreatePrefetchScheduleResponse` | `structure` | `Arn`, `Consumption`, `Name`, `PlaybackConfigurationName`, `RecurringPrefetchConfiguration`, `Retrieval`, `ScheduleType`, `StreamId` | - |
| `CreateProgramRequest` | `structure` | `AdBreaks`, `AudienceMedia`, `ChannelName`, `LiveSourceName`, `ProgramName`, `ScheduleConfiguration`, `SourceLocationName`, `VodSourceName` | - |
| `CreateProgramResponse` | `structure` | `AdBreaks`, `Arn`, `AudienceMedia`, `ChannelName`, `ClipRange`, `CreationTime`, `DurationMillis`, `LiveSourceName`, `ProgramName`, `ScheduledStartTime`, `SourceLocationName`, `VodSourceName` | - |
| `CreateSourceLocationRequest` | `structure` | `AccessConfiguration`, `DefaultSegmentDeliveryConfiguration`, `HttpConfiguration`, `SegmentDeliveryConfigurations`, `SourceLocationName`, `Tags` | - |
| `CreateSourceLocationResponse` | `structure` | `AccessConfiguration`, `Arn`, `CreationTime`, `DefaultSegmentDeliveryConfiguration`, `HttpConfiguration`, `LastModifiedTime`, `SegmentDeliveryConfigurations`, `SourceLocationName`, `Tags` | - |
| `CreateVodSourceRequest` | `structure` | `HttpPackageConfigurations`, `SourceLocationName`, `Tags`, `VodSourceName` | - |
| `CreateVodSourceResponse` | `structure` | `Arn`, `CreationTime`, `HttpPackageConfigurations`, `LastModifiedTime`, `SourceLocationName`, `Tags`, `VodSourceName` | - |
| `DeleteChannelRequest` | `structure` | `ChannelName` | - |
| `DeleteChannelResponse` | `structure` | - | - |
| `DeleteChannelPolicyRequest` | `structure` | `ChannelName` | - |
| `DeleteChannelPolicyResponse` | `structure` | - | - |
| `DeleteLiveSourceRequest` | `structure` | `LiveSourceName`, `SourceLocationName` | - |
| `DeleteLiveSourceResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

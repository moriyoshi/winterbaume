# Amazon Chime SDK Media Pipelines

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Chime SDK media pipeline APIs in this section allow software developers to create Amazon Chime SDK media pipelines that capture, concatenate, or stream your Amazon Chime SDK meetings. For more information about media pipelines, see Amazon Chime SDK media pipelines.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Chime SDK Media Pipelines resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Chime SDK Media Pipelines workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Get`, `List`, `Delete`, `Update` operation families, including `CreateMediaCapturePipeline`, `CreateMediaConcatenationPipeline`, `CreateMediaInsightsPipeline`, `CreateMediaInsightsPipelineConfiguration`, `GetMediaCapturePipeline`, `GetMediaInsightsPipelineConfiguration`.

## Service Identity and Protocol

- AWS model slug: `chime-sdk-media-pipelines`
- AWS SDK for Rust slug: `chimesdkmediapipelines`
- Model version: `2021-07-15`
- Model file: `vendor/api-models-aws/models/chime-sdk-media-pipelines/service/2021-07-15/chime-sdk-media-pipelines-2021-07-15.json`
- SDK ID: `Chime SDK Media Pipelines`
- Endpoint prefix: `media-pipelines-chime`
- ARN namespace: `chime`
- CloudFormation name: `ChimeSDKMediaPipelines`
- CloudTrail event source: `chimesdkmediapipelines.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (7), `Get` (6), `List` (5), `Delete` (4), `Update` (3), `Start` (2), `Stop` (2), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateMediaCapturePipeline`, `CreateMediaConcatenationPipeline`, `CreateMediaInsightsPipeline`, `CreateMediaInsightsPipelineConfiguration`, `CreateMediaLiveConnectorPipeline`, `CreateMediaPipelineKinesisVideoStreamPool`, `CreateMediaStreamPipeline`, `DeleteMediaCapturePipeline`, `DeleteMediaInsightsPipelineConfiguration`, `DeleteMediaPipeline`, `DeleteMediaPipelineKinesisVideoStreamPool`, `StartSpeakerSearchTask`, `StartVoiceToneAnalysisTask`, `StopSpeakerSearchTask`, `StopVoiceToneAnalysisTask`, `TagResource`, `UntagResource`, `UpdateMediaInsightsPipelineConfiguration`, `UpdateMediaInsightsPipelineStatus`, `UpdateMediaPipelineKinesisVideoStreamPool`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetMediaCapturePipeline`, `GetMediaInsightsPipelineConfiguration`, `GetMediaPipeline`, `GetMediaPipelineKinesisVideoStreamPool`, `GetSpeakerSearchTask`, `GetVoiceToneAnalysisTask`, `ListMediaCapturePipelines`, `ListMediaInsightsPipelineConfigurations`, `ListMediaPipelineKinesisVideoStreamPools`, `ListMediaPipelines`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetSpeakerSearchTask`, `GetVoiceToneAnalysisTask`, `StartSpeakerSearchTask`, `StartVoiceToneAnalysisTask`, `StopSpeakerSearchTask`, `StopVoiceToneAnalysisTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`, `SQS`, `Lambda`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/chime-sdk/latest/dg/creating-media-pipelines-considerations.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/create-media-stream-pipeline.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/media-stream-tips-tricks.html

Research outcomes:
- Chime SDK media pipelines capture, concatenate, stream, or process media from Chime SDK meetings and related sources.
- Media capture pipelines can save meeting media artefacts to configured destinations such as S3.
- Media stream pipelines can capture individual attendee audio or mixed audio and write to Kinesis Video Streams.
- Kinesis Video Streams output includes stream ARNs, fragment metadata, MKV containers, and audio encoding details.
- Pipeline availability and feature support depend on the Chime SDK namespace and pipeline type.
- Pipeline creation requires valid source and sink configuration and then progresses through lifecycle state.

Parity implications:
- Model media capture, concatenation, live connector, and media stream pipelines separately with source/sink configuration.
- Pipeline status should be asynchronous and should depend on meeting/source availability and destination permissions.
- Kinesis Video Streams outputs should be represented as downstream artefacts rather than inline API results.

## Operation Groups

### Create

- Operations: `CreateMediaCapturePipeline`, `CreateMediaConcatenationPipeline`, `CreateMediaInsightsPipeline`, `CreateMediaInsightsPipelineConfiguration`, `CreateMediaLiveConnectorPipeline`, `CreateMediaPipelineKinesisVideoStreamPool`, `CreateMediaStreamPipeline`
- Traits: `idempotency-token` (7)
- Common required input members in this group: `Elements`, `MediaInsightsPipelineConfigurationArn`, `MediaInsightsPipelineConfigurationName`, `PoolName`, `ResourceAccessRoleArn`, `SinkArn`, `SinkType`, `Sinks`, `SourceArn`, `SourceType`, `Sources`, `StreamConfiguration`

### Get

- Operations: `GetMediaCapturePipeline`, `GetMediaInsightsPipelineConfiguration`, `GetMediaPipeline`, `GetMediaPipelineKinesisVideoStreamPool`, `GetSpeakerSearchTask`, `GetVoiceToneAnalysisTask`
- Common required input members in this group: `Identifier`, `MediaPipelineId`, `SpeakerSearchTaskId`, `VoiceToneAnalysisTaskId`

### List

- Operations: `ListMediaCapturePipelines`, `ListMediaInsightsPipelineConfigurations`, `ListMediaPipelineKinesisVideoStreamPools`, `ListMediaPipelines`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `ResourceARN`

### Delete

- Operations: `DeleteMediaCapturePipeline`, `DeleteMediaInsightsPipelineConfiguration`, `DeleteMediaPipeline`, `DeleteMediaPipelineKinesisVideoStreamPool`
- Common required input members in this group: `Identifier`, `MediaPipelineId`

### Update

- Operations: `UpdateMediaInsightsPipelineConfiguration`, `UpdateMediaInsightsPipelineStatus`, `UpdateMediaPipelineKinesisVideoStreamPool`
- Common required input members in this group: `Elements`, `Identifier`, `ResourceAccessRoleArn`, `UpdateStatus`

### Start

- Operations: `StartSpeakerSearchTask`, `StartVoiceToneAnalysisTask`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `Identifier`, `LanguageCode`, `VoiceProfileDomainArn`

### Stop

- Operations: `StopSpeakerSearchTask`, `StopVoiceToneAnalysisTask`
- Common required input members in this group: `Identifier`, `SpeakerSearchTaskId`, `VoiceToneAnalysisTaskId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateMediaCapturePipeline` | `POST /sdk-media-capture-pipelines` | `idempotency-token` | `SinkArn`, `SinkType`, `SourceArn`, `SourceType` | `ClientRequestToken` | `CreateMediaCapturePipelineResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a media pipeline. |
| `CreateMediaConcatenationPipeline` | `POST /sdk-media-concatenation-pipelines` | `idempotency-token` | `Sinks`, `Sources` | `ClientRequestToken` | `CreateMediaConcatenationPipelineResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a media concatenation pipeline. |
| `CreateMediaInsightsPipeline` | `POST /media-insights-pipelines` | `idempotency-token` | `MediaInsightsPipelineConfigurationArn` | `ClientRequestToken` | `CreateMediaInsightsPipelineResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a media insights pipeline. |
| `CreateMediaInsightsPipelineConfiguration` | `POST /media-insights-pipeline-configurations` | `idempotency-token` | `Elements`, `MediaInsightsPipelineConfigurationName`, `ResourceAccessRoleArn` | `ClientRequestToken` | `CreateMediaInsightsPipelineConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | A structure that contains the static configurations for a media insights pipeline. |
| `CreateMediaLiveConnectorPipeline` | `POST /sdk-media-live-connector-pipelines` | `idempotency-token` | `Sinks`, `Sources` | `ClientRequestToken` | `CreateMediaLiveConnectorPipelineResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a media live connector pipeline in an Amazon Chime SDK meeting. |
| `CreateMediaPipelineKinesisVideoStreamPool` | `POST /media-pipeline-kinesis-video-stream-pools` | `idempotency-token` | `PoolName`, `StreamConfiguration` | `ClientRequestToken` | `CreateMediaPipelineKinesisVideoStreamPoolResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates an Amazon Kinesis Video Stream pool for use with media stream pipelines. If a meeting uses an opt-in Region as its MediaRegion, the KVS stream must be in that same Region. |
| `CreateMediaStreamPipeline` | `POST /sdk-media-stream-pipelines` | `idempotency-token` | `Sinks`, `Sources` | `ClientRequestToken` | `CreateMediaStreamPipelineResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a streaming media pipeline. |
| `DeleteMediaCapturePipeline` | `DELETE /sdk-media-capture-pipelines/{MediaPipelineId}` | - | `MediaPipelineId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes the media pipeline. |
| `DeleteMediaInsightsPipelineConfiguration` | `DELETE /media-insights-pipeline-configurations/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes the specified configuration settings. |
| `DeleteMediaPipeline` | `DELETE /sdk-media-pipelines/{MediaPipelineId}` | - | `MediaPipelineId` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes the media pipeline. |
| `DeleteMediaPipelineKinesisVideoStreamPool` | `DELETE /media-pipeline-kinesis-video-stream-pools/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes an Amazon Kinesis Video Stream pool. |
| `GetMediaCapturePipeline` | `GET /sdk-media-capture-pipelines/{MediaPipelineId}` | - | `MediaPipelineId` | - | `GetMediaCapturePipelineResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets an existing media pipeline. |
| `GetMediaInsightsPipelineConfiguration` | `GET /media-insights-pipeline-configurations/{Identifier}` | - | `Identifier` | - | `GetMediaInsightsPipelineConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets the configuration settings for a media insights pipeline. |
| `GetMediaPipeline` | `GET /sdk-media-pipelines/{MediaPipelineId}` | - | `MediaPipelineId` | - | `GetMediaPipelineResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets an existing media pipeline. |
| `GetMediaPipelineKinesisVideoStreamPool` | `GET /media-pipeline-kinesis-video-stream-pools/{Identifier}` | - | `Identifier` | - | `GetMediaPipelineKinesisVideoStreamPoolResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets an Kinesis video stream pool. |
| `GetSpeakerSearchTask` | `GET /media-insights-pipelines/{Identifier}/speaker-search-tasks/{SpeakerSearchTaskId}` | - | `Identifier`, `SpeakerSearchTaskId` | - | `GetSpeakerSearchTaskResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves the details of the specified speaker search task. |
| `GetVoiceToneAnalysisTask` | `GET /media-insights-pipelines/{Identifier}/voice-tone-analysis-tasks/{VoiceToneAnalysisTaskId}` | - | `Identifier`, `VoiceToneAnalysisTaskId` | - | `GetVoiceToneAnalysisTaskResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves the details of a voice tone analysis task. |
| `ListMediaCapturePipelines` | `GET /sdk-media-capture-pipelines` | `paginated` | - | - | `ListMediaCapturePipelinesResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns a list of media pipelines. |
| `ListMediaInsightsPipelineConfigurations` | `GET /media-insights-pipeline-configurations` | `paginated` | - | - | `ListMediaInsightsPipelineConfigurationsResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the available media insights pipeline configurations. |
| `ListMediaPipelineKinesisVideoStreamPools` | `GET /media-pipeline-kinesis-video-stream-pools` | `paginated` | - | - | `ListMediaPipelineKinesisVideoStreamPoolsResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the video stream pools in the media pipeline. |
| `ListMediaPipelines` | `GET /sdk-media-pipelines` | `paginated` | - | - | `ListMediaPipelinesResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns a list of media pipelines. |
| `ListTagsForResource` | `GET /tags` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the tags available for a media pipeline. |
| `StartSpeakerSearchTask` | `POST /media-insights-pipelines/{Identifier}/speaker-search-tasks?operation=start` | `idempotency-token` | `Identifier`, `VoiceProfileDomainArn` | `ClientRequestToken` | `StartSpeakerSearchTaskResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Starts a speaker search task. Before starting any speaker search tasks, you must provide all notices and obtain all consents from the speaker as required under applicable privacy and biometrics laws, and as required under the AWS service terms for the Amazon... |
| `StartVoiceToneAnalysisTask` | `POST /media-insights-pipelines/{Identifier}/voice-tone-analysis-tasks?operation=start` | `idempotency-token` | `Identifier`, `LanguageCode` | `ClientRequestToken` | `StartVoiceToneAnalysisTaskResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Starts a voice tone analysis task. For more information about voice tone analysis, see Using Amazon Chime SDK voice analytics in the Amazon Chime SDK Developer Guide . |
| `StopSpeakerSearchTask` | `POST /media-insights-pipelines/{Identifier}/speaker-search-tasks/{SpeakerSearchTaskId}?operation=stop` | - | `Identifier`, `SpeakerSearchTaskId` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Stops a speaker search task. |
| `StopVoiceToneAnalysisTask` | `POST /media-insights-pipelines/{Identifier}/voice-tone-analysis-tasks/{VoiceToneAnalysisTaskId}?operation=stop` | - | `Identifier`, `VoiceToneAnalysisTaskId` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Stops a voice tone analysis task. |
| `TagResource` | `POST /tags?operation=tag-resource` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | The ARN of the media pipeline that you want to tag. Consists of the pipeline's endpoint region, resource ID, and pipeline ID. |
| `UntagResource` | `POST /tags?operation=untag-resource` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes any tags from a media pipeline. |
| `UpdateMediaInsightsPipelineConfiguration` | `PUT /media-insights-pipeline-configurations/{Identifier}` | - | `Elements`, `Identifier`, `ResourceAccessRoleArn` | - | `UpdateMediaInsightsPipelineConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the media insights pipeline's configuration settings. |
| `UpdateMediaInsightsPipelineStatus` | `PUT /media-insights-pipeline-status/{Identifier}` | - | `Identifier`, `UpdateStatus` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the status of a media insights pipeline. |
| `UpdateMediaPipelineKinesisVideoStreamPool` | `PUT /media-pipeline-kinesis-video-stream-pools/{Identifier}` | - | `Identifier` | - | `UpdateMediaPipelineKinesisVideoStreamPoolResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates an Amazon Kinesis Video Stream pool in a media pipeline. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListMediaCapturePipelines` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListMediaInsightsPipelineConfigurations` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListMediaPipelineKinesisVideoStreamPools` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListMediaPipelines` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListTagsForResource` | - | `ResourceARN -> arn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Code`, `Message`, `RequestId` | The input parameters don't match the service's restrictions. |
| `ForbiddenException` | `structure` | `Code`, `Message`, `RequestId` | The client is permanently forbidden from making the request. |
| `ServiceFailureException` | `structure` | `Code`, `Message`, `RequestId` | The service encountered an unexpected error. |
| `ServiceUnavailableException` | `structure` | `Code`, `Message`, `RequestId` | The service is currently unavailable. |
| `ThrottledClientException` | `structure` | `Code`, `Message`, `RequestId` | The client exceeded its request rate limit. |
| `UnauthorizedClientException` | `structure` | `Code`, `Message`, `RequestId` | The client is not currently authorized to make the request. |
| `NotFoundException` | `structure` | `Code`, `Message`, `RequestId` | One or more of the resources in the request does not exist in the system. |
| `ResourceLimitExceededException` | `structure` | `Code`, `Message`, `RequestId` | The request exceeds the resource limit. |
| `ConflictException` | `structure` | `Code`, `Message`, `RequestId` | The request could not be processed because of conflict in the current state of the resource. |
| `CreateMediaCapturePipelineRequest` | `structure` | `ChimeSdkMeetingConfiguration`, `ClientRequestToken`, `SinkArn`, `SinkIamRoleArn`, `SinkType`, `SourceArn`, `SourceType`, `SseAwsKeyManagementParams`, `Tags` | - |
| `CreateMediaCapturePipelineResponse` | `structure` | `MediaCapturePipeline` | - |
| `CreateMediaConcatenationPipelineRequest` | `structure` | `ClientRequestToken`, `Sinks`, `Sources`, `Tags` | - |
| `CreateMediaConcatenationPipelineResponse` | `structure` | `MediaConcatenationPipeline` | - |
| `CreateMediaInsightsPipelineRequest` | `structure` | `ClientRequestToken`, `KinesisVideoStreamRecordingSourceRuntimeConfiguration`, `KinesisVideoStreamSourceRuntimeConfiguration`, `MediaInsightsPipelineConfigurationArn`, `MediaInsightsRuntimeMetadata`, `S3RecordingSinkRuntimeConfiguration`, `Tags` | - |
| `CreateMediaInsightsPipelineResponse` | `structure` | `MediaInsightsPipeline` | - |
| `CreateMediaInsightsPipelineConfigurationRequest` | `structure` | `ClientRequestToken`, `Elements`, `MediaInsightsPipelineConfigurationName`, `RealTimeAlertConfiguration`, `ResourceAccessRoleArn`, `Tags` | - |
| `CreateMediaInsightsPipelineConfigurationResponse` | `structure` | `MediaInsightsPipelineConfiguration` | - |
| `CreateMediaLiveConnectorPipelineRequest` | `structure` | `ClientRequestToken`, `Sinks`, `Sources`, `Tags` | - |
| `CreateMediaLiveConnectorPipelineResponse` | `structure` | `MediaLiveConnectorPipeline` | - |
| `CreateMediaPipelineKinesisVideoStreamPoolRequest` | `structure` | `ClientRequestToken`, `PoolName`, `StreamConfiguration`, `Tags` | - |
| `CreateMediaPipelineKinesisVideoStreamPoolResponse` | `structure` | `KinesisVideoStreamPoolConfiguration` | - |
| `CreateMediaStreamPipelineRequest` | `structure` | `ClientRequestToken`, `Sinks`, `Sources`, `Tags` | - |
| `CreateMediaStreamPipelineResponse` | `structure` | `MediaStreamPipeline` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

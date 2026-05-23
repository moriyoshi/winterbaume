# Amazon Kinesis Video Streams

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

No high-level service documentation is embedded in the AWS API model.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon Kinesis Video Streams by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon Kinesis Video Streams resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Kinesis Video Streams workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Update`, `List`, `Delete`, `Create` operation families, including `DescribeEdgeConfiguration`, `DescribeImageGenerationConfiguration`, `DescribeMappedResourceConfiguration`, `DescribeMediaStorageConfiguration`, `UpdateDataRetention`, `UpdateImageGenerationConfiguration`.

## Service Identity and Protocol

- AWS model slug: `kinesis-video`
- AWS SDK for Rust slug: `kinesisvideo`
- Model version: `2017-09-30`
- Model file: `vendor/api-models-aws/models/kinesis-video/service/2017-09-30/kinesis-video-2017-09-30.json`
- SDK ID: `Kinesis Video`
- Endpoint prefix: `kinesisvideo`
- ARN namespace: `kinesisvideo`
- CloudFormation name: `KinesisVideo`
- CloudTrail event source: `kinesisvideo.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (8), `Update` (7), `List` (5), `Delete` (3), `Create` (2), `Get` (2), `Tag` (2), `Untag` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSignalingChannel`, `CreateStream`, `DeleteEdgeConfiguration`, `DeleteSignalingChannel`, `DeleteStream`, `StartEdgeConfigurationUpdate`, `TagResource`, `TagStream`, `UntagResource`, `UntagStream`, `UpdateDataRetention`, `UpdateImageGenerationConfiguration`, `UpdateMediaStorageConfiguration`, `UpdateNotificationConfiguration`, `UpdateSignalingChannel`, `UpdateStream`, `UpdateStreamStorageConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeEdgeConfiguration`, `DescribeImageGenerationConfiguration`, `DescribeMappedResourceConfiguration`, `DescribeMediaStorageConfiguration`, `DescribeNotificationConfiguration`, `DescribeSignalingChannel`, `DescribeStream`, `DescribeStreamStorageConfiguration`, `GetDataEndpoint`, `GetSignalingChannelEndpoint`, `ListEdgeAgentConfigurations`, `ListSignalingChannels`, `ListStreams`, `ListTagsForResource`, `ListTagsForStream`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartEdgeConfigurationUpdate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 32 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/what-is-kinesis-video.html
- https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/how-it-works-kinesis-video-api-producer-sdk.html
- https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_GetMedia.html

Research outcomes:
- Kinesis Video Streams ingests, stores, and serves time-encoded media fragments from producers to consumers.
- Streams are control-plane resources, while media ingestion and retrieval use data-plane endpoints.
- Clients discover stream-specific data endpoints before calling media APIs.
- Producer libraries send media fragments and metadata to streams.
- Consumers can use APIs such as GetMedia over a persistent HTTP connection to receive chunks of media.
- GetMedia requires an endpoint from GetDataEndpoint and has concurrency limits such as a limited number of concurrent sessions.
- Archived media APIs and signalling APIs are distinct service surfaces for playback and WebRTC signalling.

Parity implications:
- Model streams, stream ARN/name mapping, data endpoints, fragments, producer state, consumer sessions, retention, tags, and signalling channels separately.
- Media APIs should require endpoint discovery and enforce stream/session limits.
- Control-plane stream state should be distinct from fragment storage and archived-media retrieval.

## Control-Plane / Data-Plane Coherence

- **Paired with `kinesisvideoarchivedmedia` ( and the future `kinesisvideomedia`, `kinesisvideosignaling`, `kinesisvideowebrtcstorage` ).** All four data-plane services share the SDK slug `kinesisvideo`. `GetMediaForFragmentList`, `GetClip`, `GetHLSStreamingSessionURL`, `GetDASHStreamingSessionURL`, and `ListFragments` all target streams created by this control plane via `CreateStream`. In real AWS, calling them with an unknown stream name or ARN returns `ResourceNotFoundException`.
- **Current Winterbaume status: divergent ( explicitly so ).** `winterbaume-kinesisvideoarchivedmedia` does not depend on `winterbaume-kinesisvideo`; it carries its own `streams: HashMap<String, StreamData>` and an `ensure_stream` method that auto-creates the stream with mock fragments on first request, regardless of whether the stream exists in this control plane.
- **What needs to change:** `winterbaume-kinesisvideoarchivedmedia` ( and any future `kinesisvideo*` data-plane crates ) should observe this crate's `streams` and reject unknown identifiers. Fragment payload storage can stay in the data-plane crate ( fragments are an artefact of producer ingest, not control-plane state ), but the stream identity must come from here.

## Operation Groups

### Describe

- Operations: `DescribeEdgeConfiguration`, `DescribeImageGenerationConfiguration`, `DescribeMappedResourceConfiguration`, `DescribeMediaStorageConfiguration`, `DescribeNotificationConfiguration`, `DescribeSignalingChannel`, `DescribeStream`, `DescribeStreamStorageConfiguration`
- Traits: `paginated` (1)

### Update

- Operations: `UpdateDataRetention`, `UpdateImageGenerationConfiguration`, `UpdateMediaStorageConfiguration`, `UpdateNotificationConfiguration`, `UpdateSignalingChannel`, `UpdateStream`, `UpdateStreamStorageConfiguration`
- Common required input members in this group: `ChannelARN`, `CurrentVersion`, `DataRetentionChangeInHours`, `MediaStorageConfiguration`, `Operation`, `StreamStorageConfiguration`

### List

- Operations: `ListEdgeAgentConfigurations`, `ListSignalingChannels`, `ListStreams`, `ListTagsForResource`, `ListTagsForStream`
- Traits: `paginated` (3)
- Common required input members in this group: `HubDeviceArn`, `ResourceARN`

### Delete

- Operations: `DeleteEdgeConfiguration`, `DeleteSignalingChannel`, `DeleteStream`
- Common required input members in this group: `ChannelARN`, `StreamARN`

### Create

- Operations: `CreateSignalingChannel`, `CreateStream`
- Common required input members in this group: `ChannelName`, `StreamName`

### Get

- Operations: `GetDataEndpoint`, `GetSignalingChannelEndpoint`
- Common required input members in this group: `APIName`, `ChannelARN`

### Tag

- Operations: `TagResource`, `TagStream`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`, `UntagStream`
- Common required input members in this group: `ResourceARN`, `TagKeyList`

### Start

- Operations: `StartEdgeConfigurationUpdate`
- Common required input members in this group: `EdgeConfig`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateSignalingChannel` | `POST /createSignalingChannel` | - | `ChannelName` | - | `CreateSignalingChannelOutput` | `AccessDeniedException`, `AccountChannelLimitExceededException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceInUseException`, `TagsPerResourceExceededLimitException` | Creates a signaling channel. `CreateSignalingChannel` is an asynchronous operation. |
| `CreateStream` | `POST /createStream` | - | `StreamName` | - | `CreateStreamOutput` | `AccountStreamLimitExceededException`, `ClientLimitExceededException`, `DeviceStreamLimitExceededException`, `InvalidArgumentException`, `InvalidDeviceException`, `ResourceInUseException`, `TagsPerResourceExceededLimitException` | Creates a new Kinesis video stream. When you create a new stream, Kinesis Video Streams assigns it a version number. |
| `DeleteEdgeConfiguration` | `POST /deleteEdgeConfiguration` | - | - | - | `DeleteEdgeConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException`, `StreamEdgeConfigurationNotFoundException` | An asynchronous API that deletes a stream’s existing edge configuration, as well as the corresponding media from the Edge Agent. When you invoke this API, the sync status is set to `DELETING`. |
| `DeleteSignalingChannel` | `POST /deleteSignalingChannel` | - | `ChannelARN` | - | `DeleteSignalingChannelOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `VersionMismatchException` | Deletes a specified signaling channel. `DeleteSignalingChannel` is an asynchronous operation. |
| `DeleteStream` | `POST /deleteStream` | - | `StreamARN` | - | `DeleteStreamOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceInUseException`, `ResourceNotFoundException`, `VersionMismatchException` | Deletes a Kinesis video stream and the data contained in the stream. This method marks the stream for deletion, and makes the data in the stream inaccessible immediately. |
| `DescribeEdgeConfiguration` | `POST /describeEdgeConfiguration` | - | - | - | `DescribeEdgeConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException`, `StreamEdgeConfigurationNotFoundException` | Describes a stream’s edge configuration that was set using the `StartEdgeConfigurationUpdate` API and the latest status of the edge agent's recorder and uploader jobs. Use this API to get the status of the configuration to determine if the configuration is in... |
| `DescribeImageGenerationConfiguration` | `POST /describeImageGenerationConfiguration` | - | - | - | `DescribeImageGenerationConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Gets the `ImageGenerationConfiguration` for a given Kinesis video stream. |
| `DescribeMappedResourceConfiguration` | `POST /describeMappedResourceConfiguration` | `paginated` | - | - | `DescribeMappedResourceConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Returns the most current information about the stream. The `streamName` or `streamARN` should be provided in the input. |
| `DescribeMediaStorageConfiguration` | `POST /describeMediaStorageConfiguration` | - | - | - | `DescribeMediaStorageConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Returns the most current information about the channel. Specify the `ChannelName` or `ChannelARN` in the input. |
| `DescribeNotificationConfiguration` | `POST /describeNotificationConfiguration` | - | - | - | `DescribeNotificationConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Gets the `NotificationConfiguration` for a given Kinesis video stream. |
| `DescribeSignalingChannel` | `POST /describeSignalingChannel` | - | - | - | `DescribeSignalingChannelOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Returns the most current information about the signaling channel. You must specify either the name or the Amazon Resource Name (ARN) of the channel that you want to describe. |
| `DescribeStream` | `POST /describeStream` | - | - | - | `DescribeStreamOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceNotFoundException` | Returns the most current information about the specified stream. You must specify either the `StreamName` or the `StreamARN`. |
| `DescribeStreamStorageConfiguration` | `POST /describeStreamStorageConfiguration` | - | - | - | `DescribeStreamStorageConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Retrieves the current storage configuration for the specified Kinesis video stream. In the request, you must specify either the `StreamName` or the `StreamARN`. |
| `GetDataEndpoint` | `POST /getDataEndpoint` | - | `APIName` | - | `GetDataEndpointOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceNotFoundException` | Gets an endpoint for a specified stream for either reading or writing. Use this endpoint in your application to read from the specified stream (using the `GetMedia` or `GetMediaForFragmentList` operations) or write to it (using the `PutMedia` operation). |
| `GetSignalingChannelEndpoint` | `POST /getSignalingChannelEndpoint` | - | `ChannelARN` | - | `GetSignalingChannelEndpointOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException` | Provides an endpoint for the specified signaling channel to send and receive messages. This API uses the `SingleMasterChannelEndpointConfiguration` input parameter, which consists of the `Protocols` and `Role` properties. |
| `ListEdgeAgentConfigurations` | `POST /listEdgeAgentConfigurations` | `paginated` | `HubDeviceArn` | - | `ListEdgeAgentConfigurationsOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException` | Returns an array of edge configurations associated with the specified Edge Agent. In the request, you must specify the Edge Agent `HubDeviceArn`. |
| `ListSignalingChannels` | `POST /listSignalingChannels` | `paginated` | - | - | `ListSignalingChannelsOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException` | Returns an array of `ChannelInfo` objects. Each object describes a signaling channel. |
| `ListStreams` | `POST /listStreams` | `paginated` | - | - | `ListStreamsOutput` | `ClientLimitExceededException`, `InvalidArgumentException` | Returns an array of `StreamInfo` objects. Each object describes a stream. |
| `ListTagsForResource` | `POST /ListTagsForResource` | - | `ResourceARN` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Returns a list of tags associated with the specified signaling channel. |
| `ListTagsForStream` | `POST /listTagsForStream` | - | - | - | `ListTagsForStreamOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidResourceFormatException`, `NotAuthorizedException`, `ResourceNotFoundException` | Returns a list of tags associated with the specified stream. In the request, you must specify either the `StreamName` or the `StreamARN`. |
| `StartEdgeConfigurationUpdate` | `POST /startEdgeConfigurationUpdate` | - | `EdgeConfig` | - | `StartEdgeConfigurationUpdateOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `NoDataRetentionException`, `ResourceInUseException`, `ResourceNotFoundException` | An asynchronous API that updates a stream’s existing edge configuration. The Kinesis Video Stream will sync the stream’s edge configuration with the Edge Agent IoT Greengrass component that runs on an IoT Hub Device, setup at your premise. |
| `TagResource` | `POST /TagResource` | - | `ResourceARN`, `Tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException`, `TagsPerResourceExceededLimitException` | Adds one or more tags to a signaling channel. A tag is a key-value pair (the value is optional) that you can define and assign to Amazon Web Services resources. |
| `TagStream` | `POST /tagStream` | - | `Tags` | - | `TagStreamOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidResourceFormatException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TagsPerResourceExceededLimitException` | Adds one or more tags to a stream. A tag is a key-value pair (the value is optional) that you can define and assign to Amazon Web Services resources. |
| `UntagResource` | `POST /UntagResource` | - | `ResourceARN`, `TagKeyList` | - | `UntagResourceOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Removes one or more tags from a signaling channel. In the request, specify only a tag key or keys; don't specify the value. |
| `UntagStream` | `POST /untagStream` | - | `TagKeyList` | - | `UntagStreamOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidResourceFormatException`, `NotAuthorizedException`, `ResourceNotFoundException` | Removes one or more tags from a stream. In the request, specify only a tag key or keys; don't specify the value. |
| `UpdateDataRetention` | `POST /updateDataRetention` | - | `CurrentVersion`, `DataRetentionChangeInHours`, `Operation` | - | `UpdateDataRetentionOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceInUseException`, `ResourceNotFoundException`, `VersionMismatchException` | Increases or decreases the stream's data retention period by the value that you specify. To indicate whether you want to increase or decrease the data retention period, specify the `Operation` parameter in the request body. |
| `UpdateImageGenerationConfiguration` | `POST /updateImageGenerationConfiguration` | - | - | - | `UpdateImageGenerationConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `NoDataRetentionException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the `StreamInfo` and `ImageProcessingConfiguration` fields. |
| `UpdateMediaStorageConfiguration` | `POST /updateMediaStorageConfiguration` | - | `ChannelARN`, `MediaStorageConfiguration` | - | `UpdateMediaStorageConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `NoDataRetentionException`, `ResourceInUseException`, `ResourceNotFoundException` | Associates a `SignalingChannel` to a stream to store the media. There are two signaling modes that you can specify : If `StorageStatus` is enabled, the data will be stored in the `StreamARN` provided. |
| `UpdateNotificationConfiguration` | `POST /updateNotificationConfiguration` | - | - | - | `UpdateNotificationConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `NoDataRetentionException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the notification information for a stream. |
| `UpdateSignalingChannel` | `POST /updateSignalingChannel` | - | `ChannelARN`, `CurrentVersion` | - | `UpdateSignalingChannelOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `VersionMismatchException` | Updates the existing signaling channel. This is an asynchronous operation and takes time to complete. |
| `UpdateStream` | `POST /updateStream` | - | `CurrentVersion` | - | `UpdateStreamOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceInUseException`, `ResourceNotFoundException`, `VersionMismatchException` | Updates stream metadata, such as the device name and media type. You must provide the stream name or the Amazon Resource Name (ARN) of the stream. |
| `UpdateStreamStorageConfiguration` | `POST /updateStreamStorageConfiguration` | - | `CurrentVersion`, `StreamStorageConfiguration` | - | `UpdateStreamStorageConfigurationOutput` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException`, `VersionMismatchException` | Updates the storage configuration for an existing Kinesis video stream. This operation allows you to modify the storage tier settings for a stream, enabling you to optimize storage costs and performance based on your access patterns. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClientLimitExceededException` | `structure` | `Message` | Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. |
| `InvalidArgumentException` | `structure` | `Message` | The value for this input parameter is invalid. |
| `ResourceNotFoundException` | `structure` | `Message` | Amazon Kinesis Video Streams can't find the stream that you specified. |
| `AccessDeniedException` | `structure` | `Message` | You do not have required permissions to perform this operation. |
| `ResourceInUseException` | `structure` | `Message` | When the input `StreamARN` or `ChannelARN` in `CLOUD_STORAGE_MODE` is already mapped to a different Kinesis Video Stream resource, or if the provided input `StreamARN` or... |
| `NotAuthorizedException` | `structure` | `Message` | The caller is not authorized to perform this operation. |
| `VersionMismatchException` | `structure` | `Message` | The stream version that you specified is not the latest version. |
| `TagsPerResourceExceededLimitException` | `structure` | `Message` | You have exceeded the limit of tags that you can associate with the resource. |
| `NoDataRetentionException` | `structure` | `Message` | The Stream data retention in hours is equal to zero. |
| `InvalidResourceFormatException` | `structure` | `Message` | The format of the `StreamARN` is invalid. |
| `StreamEdgeConfigurationNotFoundException` | `structure` | `Message` | The Exception rendered when the Amazon Kinesis Video Stream can't find a stream's edge configuration that you specified. |
| `CreateSignalingChannelInput` | `structure` | `ChannelName`, `ChannelType`, `SingleMasterConfiguration`, `Tags` | - |
| `CreateSignalingChannelOutput` | `structure` | `ChannelARN` | - |
| `AccountChannelLimitExceededException` | `structure` | `Message` | You have reached the maximum limit of active signaling channels for this Amazon Web Services account in this region. |
| `CreateStreamInput` | `structure` | `DataRetentionInHours`, `DeviceName`, `KmsKeyId`, `MediaType`, `StreamName`, `StreamStorageConfiguration`, `Tags` | - |
| `CreateStreamOutput` | `structure` | `StreamARN` | - |
| `AccountStreamLimitExceededException` | `structure` | `Message` | The number of streams created for the account is too high. |
| `DeviceStreamLimitExceededException` | `structure` | `Message` | Not implemented. |
| `InvalidDeviceException` | `structure` | `Message` | Not implemented. |
| `DeleteEdgeConfigurationInput` | `structure` | `StreamARN`, `StreamName` | - |
| `DeleteEdgeConfigurationOutput` | `structure` | - | - |
| `DeleteSignalingChannelInput` | `structure` | `ChannelARN`, `CurrentVersion` | - |
| `DeleteSignalingChannelOutput` | `structure` | - | - |
| `DeleteStreamInput` | `structure` | `CurrentVersion`, `StreamARN` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

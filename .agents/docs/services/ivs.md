# Amazon Interactive Video Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Introduction The Amazon Interactive Video Service (IVS) API is REST compatible, using a standard HTTP API and an Amazon Web Services EventBridge event stream for responses. JSON is used for both requests and responses, including errors. The API is an Amazon Web Services regional service. For a list of supported regions and Amazon IVS HTTPS service endpoints, see the Amazon IVS page in the Amazon Web Services General Reference . All API request parameters and URLs are case sensitive.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Interactive Video Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Interactive Video Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Delete`, `Create`, `Batch` operation families, including `ListChannels`, `ListPlaybackKeyPairs`, `ListPlaybackRestrictionPolicies`, `ListRecordingConfigurations`, `GetChannel`, `GetPlaybackKeyPair`.

## Service Identity and Protocol

- AWS model slug: `ivs`
- AWS SDK for Rust slug: `ivs`
- Model version: `2020-07-14`
- Model file: `vendor/api-models-aws/models/ivs/service/2020-07-14/ivs-2020-07-14.json`
- SDK ID: `ivs`
- Endpoint prefix: `-`
- ARN namespace: `ivs`
- CloudFormation name: `IVS`
- CloudTrail event source: `ivs.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (7), `Delete` (5), `Create` (4), `Batch` (3), `Update` (2), `Import` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetChannel`, `BatchGetStreamKey`, `BatchStartViewerSessionRevocation`, `CreateChannel`, `CreatePlaybackRestrictionPolicy`, `CreateRecordingConfiguration`, `CreateStreamKey`, `DeleteChannel`, `DeletePlaybackKeyPair`, `DeletePlaybackRestrictionPolicy`, `DeleteRecordingConfiguration`, `DeleteStreamKey`, `ImportPlaybackKeyPair`, `PutMetadata`, `StartViewerSessionRevocation`, `StopStream`, `TagResource`, `UntagResource`, `UpdateChannel`, `UpdatePlaybackRestrictionPolicy`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetChannel`, `BatchGetStreamKey`, `BatchStartViewerSessionRevocation`, `GetChannel`, `GetPlaybackKeyPair`, `GetPlaybackRestrictionPolicy`, `GetRecordingConfiguration`, `GetStream`, `GetStreamKey`, `GetStreamSession`, `ListChannels`, `ListPlaybackKeyPairs`, `ListPlaybackRestrictionPolicies`, `ListRecordingConfigurations`, `ListStreamKeys`, `ListStreamSessions`, `ListStreams`, `ListTagsForResource`, `StartViewerSessionRevocation`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportPlaybackKeyPair`, `StartViewerSessionRevocation`, `StopStream`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListChannels`, `ListPlaybackKeyPairs`, `ListPlaybackRestrictionPolicies`, `ListRecordingConfigurations`, `ListStreamKeys`, `ListStreamSessions`, `ListStreams`, `ListTagsForResource`
- Traits: `paginated` (7), `readonly` (8)
- Common required input members in this group: `channelArn`, `resourceArn`

### Get

- Operations: `GetChannel`, `GetPlaybackKeyPair`, `GetPlaybackRestrictionPolicy`, `GetRecordingConfiguration`, `GetStream`, `GetStreamKey`, `GetStreamSession`
- Traits: `readonly` (7)
- Common required input members in this group: `arn`, `channelArn`

### Delete

- Operations: `DeleteChannel`, `DeletePlaybackKeyPair`, `DeletePlaybackRestrictionPolicy`, `DeleteRecordingConfiguration`, `DeleteStreamKey`
- Common required input members in this group: `arn`

### Create

- Operations: `CreateChannel`, `CreatePlaybackRestrictionPolicy`, `CreateRecordingConfiguration`, `CreateStreamKey`
- Common required input members in this group: `channelArn`, `destinationConfiguration`

### Batch

- Operations: `BatchGetChannel`, `BatchGetStreamKey`, `BatchStartViewerSessionRevocation`
- Traits: `readonly` (3)
- Common required input members in this group: `arns`, `viewerSessions`

### Update

- Operations: `UpdateChannel`, `UpdatePlaybackRestrictionPolicy`
- Common required input members in this group: `arn`

### Import

- Operations: `ImportPlaybackKeyPair`
- Common required input members in this group: `publicKeyMaterial`

### Put

- Operations: `PutMetadata`
- Common required input members in this group: `channelArn`, `metadata`

### Start

- Operations: `StartViewerSessionRevocation`
- Traits: `readonly` (1)
- Common required input members in this group: `channelArn`, `viewerId`

### Stop

- Operations: `StopStream`
- Common required input members in this group: `channelArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetChannel` | `POST /BatchGetChannel` | `readonly` | `arns` | - | `BatchGetChannelResponse` | - | Performs GetChannel on multiple ARNs simultaneously. |
| `BatchGetStreamKey` | `POST /BatchGetStreamKey` | `readonly` | `arns` | - | `BatchGetStreamKeyResponse` | - | Performs GetStreamKey on multiple ARNs simultaneously. |
| `BatchStartViewerSessionRevocation` | `POST /BatchStartViewerSessionRevocation` | `readonly` | `viewerSessions` | - | `BatchStartViewerSessionRevocationResponse` | `AccessDeniedException`, `PendingVerification`, `ThrottlingException`, `ValidationException` | Performs StartViewerSessionRevocation on multiple channel ARN and viewer ID pairs simultaneously. |
| `CreateChannel` | `POST /CreateChannel` | - | - | - | `CreateChannelResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new channel and an associated stream key to start streaming. |
| `CreatePlaybackRestrictionPolicy` | `POST /CreatePlaybackRestrictionPolicy` | - | - | - | `CreatePlaybackRestrictionPolicyResponse` | `AccessDeniedException`, `PendingVerification`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new playback restriction policy, for constraining playback by countries and/or origins. |
| `CreateRecordingConfiguration` | `POST /CreateRecordingConfiguration` | - | `destinationConfiguration` | - | `CreateRecordingConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PendingVerification`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new recording configuration, used to enable recording to Amazon S3. Known issue: In the us-east-1 region, if you use the Amazon Web Services CLI to create a recording configuration, it returns success even if the S3 bucket is in a different region. |
| `CreateStreamKey` | `POST /CreateStreamKey` | - | `channelArn` | - | `CreateStreamKeyResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a stream key, used to initiate a stream, for the specified channel ARN. Note that CreateChannel creates a stream key. |
| `DeleteChannel` | `POST /DeleteChannel` | - | `arn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified channel and its associated stream keys. If you try to delete a live channel, you will get an error (409 ConflictException). |
| `DeletePlaybackKeyPair` | `POST /DeletePlaybackKeyPair` | - | `arn` | - | `DeletePlaybackKeyPairResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes a specified authorization key pair. This invalidates future viewer tokens generated using the key pair’s `privateKey`. |
| `DeletePlaybackRestrictionPolicy` | `POST /DeletePlaybackRestrictionPolicy` | - | `arn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified playback restriction policy. |
| `DeleteRecordingConfiguration` | `POST /DeleteRecordingConfiguration` | - | `arn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the recording configuration for the specified ARN. If you try to delete a recording configuration that is associated with a channel, you will get an error (409 ConflictException). |
| `DeleteStreamKey` | `POST /DeleteStreamKey` | - | `arn` | - | `Unit` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes the stream key for the specified ARN, so it can no longer be used to stream. |
| `GetChannel` | `POST /GetChannel` | `readonly` | `arn` | - | `GetChannelResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets the channel configuration for the specified channel ARN. See also BatchGetChannel. |
| `GetPlaybackKeyPair` | `POST /GetPlaybackKeyPair` | `readonly` | `arn` | - | `GetPlaybackKeyPairResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets a specified playback authorization key pair and returns the `arn` and `fingerprint`. The `privateKey` held by the caller can be used to generate viewer authorization tokens, to grant viewers access to private channels. |
| `GetPlaybackRestrictionPolicy` | `POST /GetPlaybackRestrictionPolicy` | `readonly` | `arn` | - | `GetPlaybackRestrictionPolicyResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Gets the specified playback restriction policy. |
| `GetRecordingConfiguration` | `POST /GetRecordingConfiguration` | `readonly` | `arn` | - | `GetRecordingConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets the recording configuration for the specified ARN. |
| `GetStream` | `POST /GetStream` | `readonly` | `channelArn` | - | `GetStreamResponse` | `AccessDeniedException`, `ChannelNotBroadcasting`, `ResourceNotFoundException`, `ValidationException` | Gets information about the active (live) stream on a specified channel. |
| `GetStreamKey` | `POST /GetStreamKey` | `readonly` | `arn` | - | `GetStreamKeyResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets stream-key information for a specified ARN. |
| `GetStreamSession` | `POST /GetStreamSession` | `readonly` | `channelArn` | - | `GetStreamSessionResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets metadata on a specified stream. |
| `ImportPlaybackKeyPair` | `POST /ImportPlaybackKeyPair` | - | `publicKeyMaterial` | - | `ImportPlaybackKeyPairResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ServiceQuotaExceededException`, `ValidationException` | Imports the public portion of a new key pair and returns its `arn` and `fingerprint`. The `privateKey` can then be used to generate viewer authorization tokens, to grant viewers access to private channels. |
| `ListChannels` | `POST /ListChannels` | `readonly`, `paginated` | - | - | `ListChannelsResponse` | `AccessDeniedException`, `ConflictException`, `ValidationException` | Gets summary information about all channels in your account, in the Amazon Web Services region where the API request is processed. This list can be filtered to match a specified name or recording-configuration ARN. |
| `ListPlaybackKeyPairs` | `POST /ListPlaybackKeyPairs` | `readonly`, `paginated` | - | - | `ListPlaybackKeyPairsResponse` | `AccessDeniedException`, `ValidationException` | Gets summary information about playback key pairs. For more information, see Setting Up Private Channels in the Amazon IVS User Guide . |
| `ListPlaybackRestrictionPolicies` | `POST /ListPlaybackRestrictionPolicies` | `readonly`, `paginated` | - | - | `ListPlaybackRestrictionPoliciesResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ValidationException` | Gets summary information about playback restriction policies. |
| `ListRecordingConfigurations` | `POST /ListRecordingConfigurations` | `readonly`, `paginated` | - | - | `ListRecordingConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Gets summary information about all recording configurations in your account, in the Amazon Web Services region where the API request is processed. |
| `ListStreamKeys` | `POST /ListStreamKeys` | `readonly`, `paginated` | `channelArn` | - | `ListStreamKeysResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets summary information about stream keys for the specified channel. |
| `ListStreamSessions` | `POST /ListStreamSessions` | `readonly`, `paginated` | `channelArn` | - | `ListStreamSessionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets a summary of current and previous streams for a specified channel in your account, in the AWS region where the API request is processed. |
| `ListStreams` | `POST /ListStreams` | `readonly`, `paginated` | - | - | `ListStreamsResponse` | `AccessDeniedException`, `ValidationException` | Gets summary information about live streams in your account, in the Amazon Web Services region where the API request is processed. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about Amazon Web Services tags for the specified ARN. |
| `PutMetadata` | `POST /PutMetadata` | - | `channelArn`, `metadata` | - | `Unit` | `AccessDeniedException`, `ChannelNotBroadcasting`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Inserts metadata into the active stream of the specified channel. At most 5 requests per second per channel are allowed, each with a maximum 1 KB payload. |
| `StartViewerSessionRevocation` | `POST /StartViewerSessionRevocation` | `readonly` | `channelArn`, `viewerId` | - | `StartViewerSessionRevocationResponse` | `AccessDeniedException`, `InternalServerException`, `PendingVerification`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the process of revoking the viewer session associated with a specified channel ARN and viewer ID. Optionally, you can provide a version to revoke viewer sessions less than and including that version. |
| `StopStream` | `POST /StopStream` | - | `channelArn` | - | `StopStreamResponse` | `AccessDeniedException`, `ChannelNotBroadcasting`, `ResourceNotFoundException`, `StreamUnavailable`, `ValidationException` | Disconnects the incoming RTMPS stream for the specified channel. Can be used in conjunction with DeleteStreamKey to prevent further streaming to a channel. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds or updates tags for the Amazon Web Services resource with the specified ARN. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from the resource with the specified ARN. |
| `UpdateChannel` | `POST /UpdateChannel` | - | `arn` | - | `UpdateChannelResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Updates a channel's configuration. Live channels cannot be updated. |
| `UpdatePlaybackRestrictionPolicy` | `POST /UpdatePlaybackRestrictionPolicy` | - | `arn` | - | `UpdatePlaybackRestrictionPolicyResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Updates a specified playback restriction policy. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `exceptionMessage` | - |
| `AccessDeniedException` | `structure` | `exceptionMessage` | - |
| `ResourceNotFoundException` | `structure` | `exceptionMessage` | - |
| `PendingVerification` | `structure` | `exceptionMessage` | - |
| `ConflictException` | `structure` | `exceptionMessage` | - |
| `InternalServerException` | `structure` | `exceptionMessage` | - |
| `ServiceQuotaExceededException` | `structure` | `exceptionMessage` | - |
| `ThrottlingException` | `structure` | `exceptionMessage` | - |
| `ChannelNotBroadcasting` | `structure` | `exceptionMessage` | - |
| `BatchGetChannelRequest` | `structure` | `arns` | - |
| `BatchGetChannelResponse` | `structure` | `channels`, `errors` | - |
| `BatchGetStreamKeyRequest` | `structure` | `arns` | - |
| `BatchGetStreamKeyResponse` | `structure` | `errors`, `streamKeys` | - |
| `BatchStartViewerSessionRevocationRequest` | `structure` | `viewerSessions` | - |
| `BatchStartViewerSessionRevocationResponse` | `structure` | `errors` | - |
| `CreateChannelRequest` | `structure` | `authorized`, `containerFormat`, `insecureIngest`, `latencyMode`, `multitrackInputConfiguration`, `name`, `playbackRestrictionPolicyArn`, `preset`, `recordingConfigurationArn`, `tags`, `type` | - |
| `CreateChannelResponse` | `structure` | `channel`, `streamKey` | - |
| `CreatePlaybackRestrictionPolicyRequest` | `structure` | `allowedCountries`, `allowedOrigins`, `enableStrictOriginEnforcement`, `name`, `tags` | - |
| `CreatePlaybackRestrictionPolicyResponse` | `structure` | `playbackRestrictionPolicy` | - |
| `CreateRecordingConfigurationRequest` | `structure` | `destinationConfiguration`, `name`, `recordingReconnectWindowSeconds`, `renditionConfiguration`, `tags`, `thumbnailConfiguration` | - |
| `CreateRecordingConfigurationResponse` | `structure` | `recordingConfiguration` | - |
| `CreateStreamKeyRequest` | `structure` | `channelArn`, `tags` | - |
| `CreateStreamKeyResponse` | `structure` | `streamKey` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

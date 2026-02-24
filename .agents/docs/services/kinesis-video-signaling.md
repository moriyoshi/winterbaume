# Amazon Kinesis Video Signaling Channels

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Kinesis Video Streams Signaling Service is a intermediate service that establishes a communication channel for discovering peers, transmitting offers and answers in order to establish peer-to-peer connection in webRTC technology.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Kinesis Video Signaling Channels workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `Send` operation families, including `GetIceServerConfig`, `SendAlexaOfferToMaster`.

## Service Identity and Protocol

- AWS model slug: `kinesis-video-signaling`
- AWS SDK for Rust slug: `kinesisvideo`
- Model version: `2019-12-04`
- Model file: `vendor/api-models-aws/models/kinesis-video-signaling/service/2019-12-04/kinesis-video-signaling-2019-12-04.json`
- SDK ID: `Kinesis Video Signaling`
- Endpoint prefix: `kinesisvideo`
- ARN namespace: `kinesisvideo`
- CloudFormation name: `KinesisVideoSignaling`
- CloudTrail event source: `kinesisvideosignaling.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1), `Send` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetIceServerConfig`.
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Get

- Operations: `GetIceServerConfig`
- Common required input members in this group: `ChannelARN`

### Send

- Operations: `SendAlexaOfferToMaster`
- Common required input members in this group: `ChannelARN`, `MessagePayload`, `SenderClientId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetIceServerConfig` | `POST /v1/get-ice-server-config` | - | `ChannelARN` | - | `GetIceServerConfigResponse` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidClientException`, `NotAuthorizedException`, `ResourceNotFoundException`, `SessionExpiredException` | Gets the Interactive Connectivity Establishment (ICE) server configuration information, including URIs, username, and password which can be used to configure the WebRTC connection. The ICE component uses this configuration information to setup the WebRTC... |
| `SendAlexaOfferToMaster` | `POST /v1/send-alexa-offer-to-master` | - | `ChannelARN`, `MessagePayload`, `SenderClientId` | - | `SendAlexaOfferToMasterResponse` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceNotFoundException` | This API allows you to connect WebRTC-enabled devices with Alexa display devices. When invoked, it sends the Alexa Session Description Protocol (SDP) offer to the master peer. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClientLimitExceededException` | `structure` | `Message` | Your request was throttled because you have exceeded the limit of allowed client calls. |
| `InvalidArgumentException` | `structure` | `Message` | The value for this input parameter is invalid. |
| `NotAuthorizedException` | `structure` | `Message` | The caller is not authorized to perform this operation. |
| `ResourceNotFoundException` | `structure` | `Message` | The specified resource is not found. |
| `GetIceServerConfigRequest` | `structure` | `ChannelARN`, `ClientId`, `Service`, `Username` | - |
| `GetIceServerConfigResponse` | `structure` | `IceServerList` | - |
| `InvalidClientException` | `structure` | `message` | The specified client is invalid. |
| `SessionExpiredException` | `structure` | `message` | If the client session is expired. |
| `SendAlexaOfferToMasterRequest` | `structure` | `ChannelARN`, `MessagePayload`, `SenderClientId` | - |
| `SendAlexaOfferToMasterResponse` | `structure` | `Answer` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon Kinesis Video WebRTC Storage

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

webrtc

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Kinesis Video WebRTC Storage workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Join` operation families, including `JoinStorageSession`, `JoinStorageSessionAsViewer`.

## Service Identity and Protocol

- AWS model slug: `kinesis-video-webrtc-storage`
- AWS SDK for Rust slug: `kinesisvideo`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/kinesis-video-webrtc-storage/service/2018-05-10/kinesis-video-webrtc-storage-2018-05-10.json`
- SDK ID: `Kinesis Video WebRTC Storage`
- Endpoint prefix: `kinesisvideo`
- ARN namespace: `kinesisvideo`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Join` (2).
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Join

- Operations: `JoinStorageSession`, `JoinStorageSessionAsViewer`
- Common required input members in this group: `channelArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `JoinStorageSession` | `POST /joinStorageSession` | - | `channelArn` | - | `Unit` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Before using this API, you must call the GetSignalingChannelEndpoint API to request the WEBRTC endpoint. You then specify the endpoint and region in your JoinStorageSession API request. Join the ongoing one way-video ... |
| `JoinStorageSessionAsViewer` | `POST /joinStorageSessionAsViewer` | - | `channelArn`, `clientId` | - | `Unit` | `AccessDeniedException`, `ClientLimitExceededException`, `InvalidArgumentException`, `ResourceNotFoundException` | Join the ongoing one way-video and/or multi-way audio WebRTC session as a viewer for an input channel. If there’s no existing session for the channel, create a new streaming session and provide the Amazon Resource Na ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have required permissions to perform this operation. |
| `ClientLimitExceededException` | `structure` | message | Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later. |
| `InvalidArgumentException` | `structure` | message | The value for this input parameter is invalid. |
| `ResourceNotFoundException` | `structure` | message | The specified resource is not found. |
| `JoinStorageSessionInput` | `structure` | channelArn | - |
| `JoinStorageSessionAsViewerInput` | `structure` | channelArn, clientId | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

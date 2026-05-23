# Amazon Kinesis Video Streams Archived Media

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

No high-level service documentation is embedded in the AWS API model.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Kinesis Video Streams Archived Media workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `List` operation families, including `GetClip`, `GetDASHStreamingSessionURL`, `GetHLSStreamingSessionURL`, `GetImages`, `ListFragments`.

## Service Identity and Protocol

- AWS model slug: `kinesis-video-archived-media`
- AWS SDK for Rust slug: `kinesisvideoarchivedmedia`
- Model version: `2017-09-30`
- Model file: `vendor/api-models-aws/models/kinesis-video-archived-media/service/2017-09-30/kinesis-video-archived-media-2017-09-30.json`
- SDK ID: `Kinesis Video Archived Media`
- Endpoint prefix: `kinesisvideo`
- ARN namespace: `kinesisvideo`
- CloudFormation name: `KinesisVideoArchivedMedia`
- CloudTrail event source: `kinesisvideoarchivedmedia.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetClip`, `GetDASHStreamingSessionURL`, `GetHLSStreamingSessionURL`, `GetImages`, `GetMediaForFragmentList`, `ListFragments`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Control-Plane / Data-Plane Coherence

- **Paired with `kinesisvideo` ( same SDK slug `kinesisvideo` ).** `GetMediaForFragmentList`, `GetClip`, `GetHLSStreamingSessionURL`, `GetDASHStreamingSessionURL`, and `ListFragments` all target streams created by the Kinesis Video control plane ( `winterbaume-kinesisvideo` ) via `CreateStream`. In real AWS, calling them with an unknown stream name or ARN returns `ResourceNotFoundException`.
- **Current Winterbaume status: divergent ( explicitly so ).** This crate does not depend on `winterbaume-kinesisvideo`; it carries its own `streams: HashMap<String, StreamData>` and an `ensure_stream` method that auto-creates the stream with mock fragments on first request, regardless of whether the stream exists in the control plane.
- **What needs to change:** observe `winterbaume-kinesisvideo`'s `streams` and reject unknown identifiers. Fragment payload storage can stay here ( fragments are an artefact of producer ingest, not control-plane state ), but the stream identity must come from there.

## Operation Groups

### Get

- Operations: `GetClip`, `GetDASHStreamingSessionURL`, `GetHLSStreamingSessionURL`, `GetImages`, `GetMediaForFragmentList`
- Traits: `paginated` (1)
- Common required input members in this group: `ClipFragmentSelector`, `EndTimestamp`, `Format`, `Fragments`, `ImageSelectorType`, `StartTimestamp`

### List

- Operations: `ListFragments`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetClip` | `POST /getClip` | - | `ClipFragmentSelector` | - | `GetClipOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidCodecPrivateDataException`, `InvalidMediaFrameException`, `MissingCodecPrivateDataException`, `NoDataRetentionException`, `NotAuthorizedException`, `ResourceNotFoundException`, ... (+1) | Downloads an MP4 file (clip) containing the archived, on-demand media from the specified video stream over the specified time range. Both the StreamName and the StreamARN parameters are optional, but you must specify either the StreamName or the StreamARN... |
| `GetDASHStreamingSessionURL` | `POST /getDASHStreamingSessionURL` | - | - | - | `GetDASHStreamingSessionURLOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidCodecPrivateDataException`, `MissingCodecPrivateDataException`, `NoDataRetentionException`, `NotAuthorizedException`, `ResourceNotFoundException`, `UnsupportedStreamMediaTypeException` | Retrieves an MPEG Dynamic Adaptive Streaming over HTTP (DASH) URL for the stream. You can then open the URL in a media player to view the stream contents. |
| `GetHLSStreamingSessionURL` | `POST /getHLSStreamingSessionURL` | - | - | - | `GetHLSStreamingSessionURLOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `InvalidCodecPrivateDataException`, `MissingCodecPrivateDataException`, `NoDataRetentionException`, `NotAuthorizedException`, `ResourceNotFoundException`, `UnsupportedStreamMediaTypeException` | Retrieves an HTTP Live Streaming (HLS) URL for the stream. You can then open the URL in a browser or media player to view the stream contents. |
| `GetImages` | `POST /getImages` | `paginated` | `EndTimestamp`, `Format`, `ImageSelectorType`, `StartTimestamp` | - | `GetImagesOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NoDataRetentionException`, `NotAuthorizedException`, `ResourceNotFoundException` | Retrieves a list of images corresponding to each timestamp for a given time range, sampling interval, and image format configuration. |
| `GetMediaForFragmentList` | `POST /getMediaForFragmentList` | - | `Fragments` | - | `GetMediaForFragmentListOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceNotFoundException` | Gets media for a list of fragments (specified by fragment number) from the archived data in an Amazon Kinesis video stream. You must first call the `GetDataEndpoint` API to get an endpoint. |
| `ListFragments` | `POST /listFragments` | `paginated` | - | - | `ListFragmentsOutput` | `ClientLimitExceededException`, `InvalidArgumentException`, `NotAuthorizedException`, `ResourceNotFoundException` | Returns a list of Fragment objects from the specified stream and timestamp range within the archived data. Listing fragments is eventually consistent. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClientLimitExceededException` | `structure` | `Message` | Kinesis Video Streams has throttled the request because you have exceeded a limit. |
| `InvalidArgumentException` | `structure` | `Message` | A specified parameter exceeds its restrictions, is not supported, or can't be used. |
| `NotAuthorizedException` | `structure` | `Message` | Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired. |
| `ResourceNotFoundException` | `structure` | `Message` | `GetImages` will throw this error when Kinesis Video Streams can't find the stream that you specified. |
| `NoDataRetentionException` | `structure` | `Message` | `GetImages` was requested for a stream that does not retain data (that is, has a `DataRetentionInHours` of 0). |
| `InvalidCodecPrivateDataException` | `structure` | `Message` | The codec private data in at least one of the tracks of the video stream is not valid for this operation. |
| `MissingCodecPrivateDataException` | `structure` | `Message` | No codec private data was found in at least one of tracks of the video stream. |
| `UnsupportedStreamMediaTypeException` | `structure` | `Message` | The type of the media (for example, h.264 or h.265 video or ACC or G.711 audio) could not be determined from the codec IDs of the tracks in the first fragment for a playback... |
| `GetClipInput` | `structure` | `ClipFragmentSelector`, `StreamARN`, `StreamName` | - |
| `GetClipOutput` | `structure` | `ContentType`, `Payload` | - |
| `InvalidMediaFrameException` | `structure` | `Message` | One or more frames in the requested clip could not be parsed based on the specified codec. |
| `GetDASHStreamingSessionURLInput` | `structure` | `DASHFragmentSelector`, `DisplayFragmentNumber`, `DisplayFragmentTimestamp`, `Expires`, `MaxManifestFragmentResults`, `PlaybackMode`, `StreamARN`, `StreamName` | - |
| `GetDASHStreamingSessionURLOutput` | `structure` | `DASHStreamingSessionURL` | - |
| `GetHLSStreamingSessionURLInput` | `structure` | `ContainerFormat`, `DiscontinuityMode`, `DisplayFragmentTimestamp`, `Expires`, `HLSFragmentSelector`, `MaxMediaPlaylistFragmentResults`, `PlaybackMode`, `StreamARN`, `StreamName` | - |
| `GetHLSStreamingSessionURLOutput` | `structure` | `HLSStreamingSessionURL` | - |
| `GetImagesInput` | `structure` | `EndTimestamp`, `Format`, `FormatConfig`, `HeightPixels`, `ImageSelectorType`, `MaxResults`, `NextToken`, `SamplingInterval`, `StartTimestamp`, `StreamARN`, `StreamName`, `WidthPixels` | - |
| `GetImagesOutput` | `structure` | `Images`, `NextToken` | - |
| `GetMediaForFragmentListInput` | `structure` | `Fragments`, `StreamARN`, `StreamName` | - |
| `GetMediaForFragmentListOutput` | `structure` | `ContentType`, `Payload` | - |
| `ListFragmentsInput` | `structure` | `FragmentSelector`, `MaxResults`, `NextToken`, `StreamARN`, `StreamName` | - |
| `ListFragmentsOutput` | `structure` | `Fragments`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

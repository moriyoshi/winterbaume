# Amazon Chime SDK Meetings

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Chime SDK meetings APIs in this section allow software developers to create Amazon Chime SDK meetings, set the Amazon Web Services Regions for meetings, create and manage users, and send and receive meeting notifications. For more information about the meeting APIs, see Amazon Chime SDK meetings.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-chimesdkmeetings/tests/scenario_test.rs`: create a full meeting collaboration session, add attendees, inspect meeting and attendee details, and delete the meeting.
- Backported from `scenario_test.rs`: exercise batch attendee creation and deletion for meeting participant management.
- Scenario insight from EC2: add full state-machine walks for Amazon Chime SDK Meetings resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support meeting-room provisioning, attendee identity lifecycle, media placement information, batch participant operations, and cleanup of meeting resources.

## Service Identity and Protocol

- AWS model slug: `chime-sdk-meetings`
- AWS SDK for Rust slug: `chimesdkmeetings`
- Model version: `2021-07-15`
- Model file: `vendor/api-models-aws/models/chime-sdk-meetings/service/2021-07-15/chime-sdk-meetings-2021-07-15.json`
- SDK ID: `Chime SDK Meetings`
- Endpoint prefix: `meetings-chime`
- ARN namespace: `chime`
- CloudFormation name: `ChimeSDKMeetings`
- CloudTrail event source: `chimesdkmeetings.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (3), `Batch` (2), `Delete` (2), `Get` (2), `List` (2), `Start` (1), `Stop` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateAttendee`, `BatchUpdateAttendeeCapabilitiesExcept`, `CreateAttendee`, `CreateMeeting`, `CreateMeetingWithAttendees`, `DeleteAttendee`, `DeleteMeeting`, `StartMeetingTranscription`, `StopMeetingTranscription`, `TagResource`, `UntagResource`, `UpdateAttendeeCapabilities`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAttendee`, `GetMeeting`, `ListAttendees`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartMeetingTranscription`, `StopMeetingTranscription`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/chime-sdk/latest/dg/meetings-sdk.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/create-meeting.html
- https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_meeting-chime_CreateMeetingWithAttendees.html

Research outcomes:
- A meeting is an ephemeral resource identified by a `MeetingId` and assigned to a region-specific media service group.
- `CreateMeeting` requires a `ClientRequestToken` for uniqueness context and can include `MediaRegion`, `MeetingHostId`, and `NotificationsConfiguration`.
- `CreateMeeting` returns a meeting object containing `MeetingId`, `MediaRegion`, and `MediaPlacement` URLs.
- Lifecycle events are delivered through EventBridge by default, or optionally to an SQS queue ARN or SNS topic ARN in `NotificationsConfiguration`.
- Media placement is the set of regionalised media URLs attendees use to connect to the media service group.
- An attendee has a unique `AttendeeId` and a join token. The join token is an opaque secret that the server must transfer securely to the authorised client.
- Clients authenticate to the media service group with the attendee join token and use secure WebSockets plus DTLS for signalling and media.
- The media service mixes audio and acts as a selective forwarding unit for video.
- Each attendee can publish one video source, with up to 25 simultaneous videos per meeting.
- Meeting data messages can carry up to 2 KB each for application-specific real-time signalling.

Parity implications:
- Model meetings, attendees, join tokens, media Region, media placement URLs, notification configuration, and feature configuration separately.
- CreateMeeting and CreateMeetingWithAttendees should preserve idempotency through `ClientRequestToken` and produce stable meeting/attendee identities.
- Attendee authentication and join-token secrecy are core service semantics, even when the media plane itself is out of scope.

## Operation Groups

### Create

- Operations: `CreateAttendee`, `CreateMeeting`, `CreateMeetingWithAttendees`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `ClientRequestToken`, `MediaRegion`, `ExternalMeetingId`

### Batch

- Operations: `BatchCreateAttendee`, `BatchUpdateAttendeeCapabilitiesExcept`
- Common required input members in this group: `MeetingId`

### Delete

- Operations: `DeleteAttendee`, `DeleteMeeting`
- Common required input members in this group: `MeetingId`

### Get

- Operations: `GetAttendee`, `GetMeeting`
- Common required input members in this group: `MeetingId`

### List

- Operations: `ListAttendees`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartMeetingTranscription`
- Common required input members in this group: -

### Stop

- Operations: `StopMeetingTranscription`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateAttendeeCapabilities`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateAttendee` | `POST /meetings/{MeetingId}/attendees?operation=batch-create` | - | `MeetingId`, `Attendees` | - | `BatchCreateAttendeeResponse` | `BadRequestException`, `ForbiddenException`, `LimitExceededException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `UnprocessableEntityException` | Creates up to 100 attendees for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see Using the Amazon Chime SDK in the Amazon Chime Developer Guide . |
| `BatchUpdateAttendeeCapabilitiesExcept` | `PUT /meetings/{MeetingId}/attendees/capabilities?operation=batch-update-except` | - | `MeetingId`, `ExcludedAttendeeIds`, `Capabilities` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Updates AttendeeCapabilities except the capabilities listed in an ExcludedAttendeeIds table. You use the capabilities with a set of values that control what the capabilities can do, such as SendReceive data. For more ... |
| `CreateAttendee` | `POST /meetings/{MeetingId}/attendees` | - | `MeetingId`, `ExternalUserId` | - | `CreateAttendeeResponse` | `BadRequestException`, `ForbiddenException`, `LimitExceededException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `UnprocessableEntityException` | Creates a new attendee for an active Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see Using the Amazon Chime SDK in the Amazon Chime Developer Guide . |
| `CreateMeeting` | `POST /meetings` | `idempotency-token` | `ClientRequestToken`, `MediaRegion`, `ExternalMeetingId` | `ClientRequestToken` | `CreateMeetingResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `LimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Creates a new Amazon Chime SDK meeting in the specified media Region with no initial attendees. For more information about specifying media Regions, see Available Regions and Using meeting Regions , both in the Amazo ... |
| `CreateMeetingWithAttendees` | `POST /meetings?operation=create-attendees` | `idempotency-token` | `ClientRequestToken`, `MediaRegion`, `ExternalMeetingId`, `Attendees` | `ClientRequestToken` | `CreateMeetingWithAttendeesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `LimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Creates a new Amazon Chime SDK meeting in the specified media Region, with attendees. For more information about specifying media Regions, see Available Regions and Using meeting Regions , both in the Amazon Chime SD ... |
| `DeleteAttendee` | `DELETE /meetings/{MeetingId}/attendees/{AttendeeId}` | - | `MeetingId`, `AttendeeId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Deletes an attendee from the specified Amazon Chime SDK meeting and deletes their JoinToken . Attendees are automatically deleted when a Amazon Chime SDK meeting is deleted. For more information about the Amazon Chim ... |
| `DeleteMeeting` | `DELETE /meetings/{MeetingId}` | - | `MeetingId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Deletes the specified Amazon Chime SDK meeting. The operation deletes all attendees, disconnects all clients, and prevents new clients from joining the meeting. For more information about the Amazon Chime SDK, see Us ... |
| `GetAttendee` | `GET /meetings/{MeetingId}/attendees/{AttendeeId}` | - | `MeetingId`, `AttendeeId` | - | `GetAttendeeResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Gets the Amazon Chime SDK attendee details for a specified meeting ID and attendee ID. For more information about the Amazon Chime SDK, see Using the Amazon Chime SDK in the Amazon Chime Developer Guide . |
| `GetMeeting` | `GET /meetings/{MeetingId}` | - | `MeetingId` | - | `GetMeetingResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Gets the Amazon Chime SDK meeting details for the specified meeting ID. For more information about the Amazon Chime SDK, see Using the Amazon Chime SDK in the Amazon Chime Developer Guide . |
| `ListAttendees` | `GET /meetings/{MeetingId}/attendees` | `paginated` | `MeetingId` | - | `ListAttendeesResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Lists the attendees for the specified Amazon Chime SDK meeting. For more information about the Amazon Chime SDK, see Using the Amazon Chime SDK in the Amazon Chime Developer Guide . |
| `ListTagsForResource` | `GET /tags` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Returns a list of the tags available for the specified resource. |
| `StartMeetingTranscription` | `POST /meetings/{MeetingId}/transcription?operation=start` | - | `MeetingId`, `TranscriptionConfiguration` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `LimitExceededException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `UnprocessableEntityException` | Starts transcription for the specified meetingId . For more information, refer to Using Amazon Chime SDK live transcription in the Amazon Chime SDK Developer Guide . If you specify an invalid configuration, a Transcr ... |
| `StopMeetingTranscription` | `POST /meetings/{MeetingId}/transcription?operation=stop` | - | `MeetingId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `UnprocessableEntityException` | Stops transcription for the specified meetingId . For more information, refer to Using Amazon Chime SDK live transcription in the Amazon Chime SDK Developer Guide . By default, Amazon Transcribe may use and store aud ... |
| `TagResource` | `POST /tags?operation=tag-resource` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ForbiddenException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `TooManyTagsException`, `UnauthorizedException` | The resource that supports tags. |
| `UntagResource` | `POST /tags?operation=untag-resource` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ForbiddenException`, `LimitExceededException`, `ResourceNotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resourc ... |
| `UpdateAttendeeCapabilities` | `PUT /meetings/{MeetingId}/attendees/{AttendeeId}/capabilities` | - | `MeetingId`, `AttendeeId`, `Capabilities` | - | `UpdateAttendeeCapabilitiesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException` | The capabilities that you want to update. You use the capabilities with a set of values that control what the capabilities can do, such as SendReceive data. For more information about those values, see . When using c ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListAttendees` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListTagsForResource` | - | `ResourceARN -> arn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Code, Message, RequestId | The input parameters don't match the service's restrictions. |
| `ConflictException` | `structure` | Code, Message, RequestId | Multiple instances of the same request have been made simultaneously. |
| `ForbiddenException` | `structure` | Code, Message, RequestId | The client is permanently forbidden from making the request. |
| `LimitExceededException` | `structure` | Code, Message, RequestId | The request exceeds the resource limit. |
| `NotFoundException` | `structure` | Code, Message, RequestId | One or more of the resources in the request does not exist in the system. |
| `ResourceNotFoundException` | `structure` | Code, Message, RequestId, ResourceName | The resource that you want to tag couldn't be found. |
| `ServiceFailureException` | `structure` | Code, Message, RequestId | The service encountered an unexpected error. |
| `ServiceUnavailableException` | `structure` | Code, Message, RequestId, RetryAfterSeconds | The service is currently unavailable. |
| `ThrottlingException` | `structure` | Code, Message, RequestId | The number of customer requests exceeds the request rate limit. |
| `TooManyTagsException` | `structure` | Code, Message, RequestId, ResourceName | Too many tags were added to the specified resource. |
| `UnauthorizedException` | `structure` | Code, Message, RequestId | The user isn't authorized to request a resource. |
| `UnprocessableEntityException` | `structure` | Code, Message, RequestId | The request was well-formed but was unable to be followed due to semantic errors. |
| `BatchCreateAttendeeRequest` | `structure` | MeetingId, Attendees | - |
| `BatchCreateAttendeeResponse` | `structure` | Attendees, Errors | - |
| `BatchUpdateAttendeeCapabilitiesExceptRequest` | `structure` | MeetingId, ExcludedAttendeeIds, Capabilities | - |
| `CreateAttendeeRequest` | `structure` | MeetingId, ExternalUserId, Capabilities | - |
| `CreateAttendeeResponse` | `structure` | Attendee | - |
| `CreateMeetingRequest` | `structure` | ClientRequestToken, MediaRegion, MeetingHostId, ExternalMeetingId, NotificationsConfiguration, MeetingFeatures, PrimaryMeetingId, TenantIds, Tags, MediaPlacementNetworkType | - |
| `CreateMeetingResponse` | `structure` | Meeting | - |
| `CreateMeetingWithAttendeesRequest` | `structure` | ClientRequestToken, MediaRegion, MeetingHostId, ExternalMeetingId, MeetingFeatures, NotificationsConfiguration, Attendees, PrimaryMeetingId, TenantIds, Tags, MediaPlacementNetworkType | - |
| `CreateMeetingWithAttendeesResponse` | `structure` | Meeting, Attendees, Errors | - |
| `DeleteAttendeeRequest` | `structure` | MeetingId, AttendeeId | - |
| `DeleteMeetingRequest` | `structure` | MeetingId | - |
| `GetAttendeeRequest` | `structure` | MeetingId, AttendeeId | - |
| `GetAttendeeResponse` | `structure` | Attendee | - |
| `GetMeetingRequest` | `structure` | MeetingId | - |
| `GetMeetingResponse` | `structure` | Meeting | - |
| `ListAttendeesRequest` | `structure` | MeetingId, NextToken, MaxResults | - |
| `ListAttendeesResponse` | `structure` | Attendees, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `StartMeetingTranscriptionRequest` | `structure` | MeetingId, TranscriptionConfiguration | - |
| `StopMeetingTranscriptionRequest` | `structure` | MeetingId | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateAttendeeCapabilitiesRequest` | `structure` | MeetingId, AttendeeId, Capabilities | - |
| `UpdateAttendeeCapabilitiesResponse` | `structure` | Attendee | - |
| `ContentResolution` | `enum` | NONE, FHD, UHD | - |
| `MediaCapabilities` | `enum` | SEND_RECEIVE, SEND, RECEIVE, NONE | - |
| `MediaPlacementNetworkType` | `enum` | IPV4_ONLY, DUAL_STACK | - |
| `MeetingFeatureStatus` | `enum` | AVAILABLE, UNAVAILABLE | - |
| `TranscribeContentIdentificationType` | `enum` | PII | - |
| `TranscribeContentRedactionType` | `enum` | PII | - |
| `TranscribeLanguageCode` | `enum` | EN_US, EN_GB, ES_US, FR_CA, FR_FR, EN_AU, IT_IT, DE_DE, PT_BR, JA_JP, KO_KR, ZH_CN, ... (+2) | - |
| `TranscribeMedicalContentIdentificationType` | `enum` | PHI | - |
| `TranscribeMedicalLanguageCode` | `enum` | EN_US | - |
| `TranscribeMedicalRegion` | `enum` | US_EAST_1, US_EAST_2, US_WEST_2, AP_SOUTHEAST_2, CA_CENTRAL_1, EU_WEST_1, AUTO | - |
| `TranscribeMedicalSpecialty` | `enum` | PRIMARYCARE, CARDIOLOGY, NEUROLOGY, ONCOLOGY, RADIOLOGY, UROLOGY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

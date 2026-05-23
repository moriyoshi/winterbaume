# Amazon Connect Participant Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Participant Service actions Participant Service data types Amazon Connect is an easy-to-use omnichannel cloud contact center service that enables companies of any size to deliver superior customer service at a lower cost. Amazon Connect communications capabilities make it easy for companies to deliver personalized interactions across communication channels, including chat. Use the Amazon Connect Participant Service to manage participants (for example, agents, customers, and managers listening in), and to send messages and events within a chat contact. The APIs in the service enable the following: sending chat messages, attachment sharing, managing a participant's connection state and message events, and retrieving chat transcripts.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Connect Participant Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Send`, `Cancel`, `Complete`, `Create` operation families, including `GetAttachment`, `GetAuthenticationUrl`, `GetTranscript`, `SendEvent`, `SendMessage`, `CancelParticipantAuthentication`.

## Service Identity and Protocol

- AWS model slug: `connectparticipant`
- AWS SDK for Rust slug: `connectparticipant`
- Model version: `2018-09-07`
- Model file: `vendor/api-models-aws/models/connectparticipant/service/2018-09-07/connectparticipant-2018-09-07.json`
- SDK ID: `ConnectParticipant`
- Endpoint prefix: `participant.connect`
- ARN namespace: `connect`
- CloudFormation name: `ConnectParticipant`
- CloudTrail event source: `connectparticipant.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `Send` (2), `Cancel` (1), `Complete` (1), `Create` (1), `Describe` (1), `Disconnect` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelParticipantAuthentication`, `CreateParticipantConnection`, `StartAttachmentUpload`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeView`, `GetAttachment`, `GetAuthenticationUrl`, `GetTranscript`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelParticipantAuthentication`, `StartAttachmentUpload`.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAttachment`, `GetAuthenticationUrl`, `GetTranscript`
- Traits: `paginated` (1)
- Common required input members in this group: `AttachmentId`, `ConnectionToken`, `RedirectUri`, `SessionId`

### Send

- Operations: `SendEvent`, `SendMessage`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `ConnectionToken`, `Content`, `ContentType`

### Cancel

- Operations: `CancelParticipantAuthentication`
- Common required input members in this group: `ConnectionToken`, `SessionId`

### Complete

- Operations: `CompleteAttachmentUpload`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AttachmentIds`, `ClientToken`, `ConnectionToken`

### Create

- Operations: `CreateParticipantConnection`
- Common required input members in this group: `ParticipantToken`

### Describe

- Operations: `DescribeView`
- Common required input members in this group: `ConnectionToken`, `ViewToken`

### Disconnect

- Operations: `DisconnectParticipant`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ConnectionToken`

### Start

- Operations: `StartAttachmentUpload`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AttachmentName`, `AttachmentSizeInBytes`, `ClientToken`, `ConnectionToken`, `ContentType`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelParticipantAuthentication` | `POST /participant/cancel-authentication` | - | `ConnectionToken`, `SessionId` | - | `CancelParticipantAuthenticationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Cancels the authentication session. The opted out branch of the Authenticate Customer flow block will be taken. |
| `CompleteAttachmentUpload` | `POST /participant/complete-attachment-upload` | `idempotency-token` | `AttachmentIds`, `ClientToken`, `ConnectionToken` | `ClientToken` | `CompleteAttachmentUploadResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Allows you to confirm that the attachment has been uploaded using the pre-signed URL provided in StartAttachmentUpload API. A conflict exception is thrown when an attachment with that identifier is already being uploaded. |
| `CreateParticipantConnection` | `POST /participant/connection` | - | `ParticipantToken` | - | `CreateParticipantConnectionResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates the participant's connection. For security recommendations, see Amazon Connect Chat security best practices. |
| `DescribeView` | `GET /participant/views/{ViewToken}` | - | `ConnectionToken`, `ViewToken` | - | `DescribeViewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the view for the specified view token. For security recommendations, see Amazon Connect Chat security best practices. |
| `DisconnectParticipant` | `POST /participant/disconnect` | `idempotency-token` | `ConnectionToken` | `ClientToken` | `DisconnectParticipantResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Disconnects a participant. For security recommendations, see Amazon Connect Chat security best practices. |
| `GetAttachment` | `POST /participant/attachment` | - | `AttachmentId`, `ConnectionToken` | - | `GetAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provides a pre-signed URL for download of a completed attachment. This is an asynchronous API for use with active contacts. |
| `GetAuthenticationUrl` | `POST /participant/authentication-url` | - | `ConnectionToken`, `RedirectUri`, `SessionId` | - | `GetAuthenticationUrlResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the AuthenticationUrl for the current authentication session for the AuthenticateCustomer flow block. For security recommendations, see Amazon Connect Chat security best practices. |
| `GetTranscript` | `POST /participant/transcript` | `paginated` | `ConnectionToken` | - | `GetTranscriptResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a transcript of the session, including details about any attachments. For information about accessing past chat contact transcripts for a persistent chat, see Enable persistent chat. |
| `SendEvent` | `POST /participant/event` | `idempotency-token` | `ConnectionToken`, `ContentType` | `ClientToken` | `SendEventResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | The `application/vnd.amazonaws.connect.event.connection.acknowledged` ContentType is no longer maintained since December 31, 2024. This event has been migrated to the CreateParticipantConnection API using the `ConnectParticipant` field. |
| `SendMessage` | `POST /participant/message` | `idempotency-token` | `ConnectionToken`, `Content`, `ContentType` | `ClientToken` | `SendMessageResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sends a message. For security recommendations, see Amazon Connect Chat security best practices. |
| `StartAttachmentUpload` | `POST /participant/start-attachment-upload` | `idempotency-token` | `AttachmentName`, `AttachmentSizeInBytes`, `ClientToken`, `ConnectionToken`, `ContentType` | `ClientToken` | `StartAttachmentUploadResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Provides a pre-signed Amazon S3 URL in response for uploading the file directly to S3. For security recommendations, see Amazon Connect Chat security best practices. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CancelParticipantAuthentication` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `CompleteAttachmentUpload` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `CreateParticipantConnection` | `ParticipantToken -> X-Amz-Bearer` | - | - | - |
| `DescribeView` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `DisconnectParticipant` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `GetAttachment` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `GetAuthenticationUrl` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `GetTranscript` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `SendEvent` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `SendMessage` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |
| `StartAttachmentUpload` | `ConnectionToken -> X-Amz-Bearer` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | This exception occurs when there is an internal failure in the Amazon Connect service. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the constraints specified by Amazon Connect. |
| `ConflictException` | `structure` | `Message` | The requested operation conflicts with the current state of a service resource associated with the request. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The number of attachments per contact exceeds the quota. |
| `CancelParticipantAuthenticationRequest` | `structure` | `ConnectionToken`, `SessionId` | - |
| `CancelParticipantAuthenticationResponse` | `structure` | - | - |
| `CompleteAttachmentUploadRequest` | `structure` | `AttachmentIds`, `ClientToken`, `ConnectionToken` | - |
| `CompleteAttachmentUploadResponse` | `structure` | - | - |
| `CreateParticipantConnectionRequest` | `structure` | `ConnectParticipant`, `ParticipantToken`, `Type` | - |
| `CreateParticipantConnectionResponse` | `structure` | `ConnectionCredentials`, `WebRTCConnection`, `Websocket` | - |
| `DescribeViewRequest` | `structure` | `ConnectionToken`, `ViewToken` | - |
| `DescribeViewResponse` | `structure` | `View` | - |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The resource was not found. |
| `DisconnectParticipantRequest` | `structure` | `ClientToken`, `ConnectionToken` | - |
| `DisconnectParticipantResponse` | `structure` | - | - |
| `GetAttachmentRequest` | `structure` | `AttachmentId`, `ConnectionToken`, `UrlExpiryInSeconds` | - |
| `GetAttachmentResponse` | `structure` | `AttachmentSizeInBytes`, `Url`, `UrlExpiry` | - |
| `GetAuthenticationUrlRequest` | `structure` | `ConnectionToken`, `RedirectUri`, `SessionId` | - |
| `GetAuthenticationUrlResponse` | `structure` | `AuthenticationUrl` | - |
| `GetTranscriptRequest` | `structure` | `ConnectionToken`, `ContactId`, `MaxResults`, `NextToken`, `ScanDirection`, `SortOrder`, `StartPosition` | - |
| `GetTranscriptResponse` | `structure` | `InitialContactId`, `NextToken`, `Transcript` | - |
| `SendEventRequest` | `structure` | `ClientToken`, `ConnectionToken`, `Content`, `ContentType` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

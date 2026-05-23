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
- Common required input members in this group: `ConnectionToken`

### Send

- Operations: `SendEvent`, `SendMessage`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `ContentType`, `ConnectionToken`

### Cancel

- Operations: `CancelParticipantAuthentication`
- Common required input members in this group: -

### Complete

- Operations: `CompleteAttachmentUpload`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateParticipantConnection`
- Common required input members in this group: -

### Describe

- Operations: `DescribeView`
- Common required input members in this group: -

### Disconnect

- Operations: `DisconnectParticipant`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Start

- Operations: `StartAttachmentUpload`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelParticipantAuthentication` | `POST /participant/cancel-authentication` | - | `SessionId`, `ConnectionToken` | - | `CancelParticipantAuthenticationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Cancels the authentication session. The opted out branch of the Authenticate Customer flow block will be taken. The current supported channel is chat. This API is not supported for Apple Messages for Business, WhatsA ... |
| `CompleteAttachmentUpload` | `POST /participant/complete-attachment-upload` | `idempotency-token` | `AttachmentIds`, `ClientToken`, `ConnectionToken` | `ClientToken` | `CompleteAttachmentUploadResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Allows you to confirm that the attachment has been uploaded using the pre-signed URL provided in StartAttachmentUpload API. A conflict exception is thrown when an attachment with that identifier is already being uplo ... |
| `CreateParticipantConnection` | `POST /participant/connection` | - | `ParticipantToken` | - | `CreateParticipantConnectionResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates the participant's connection. For security recommendations, see Amazon Connect Chat security best practices . For WebRTC security recommendations, see Amazon Connect WebRTC security best practices . Participa ... |
| `DescribeView` | `GET /participant/views/{ViewToken}` | - | `ViewToken`, `ConnectionToken` | - | `DescribeViewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the view for the specified view token. For security recommendations, see Amazon Connect Chat security best practices . |
| `DisconnectParticipant` | `POST /participant/disconnect` | `idempotency-token` | `ConnectionToken` | `ClientToken` | `DisconnectParticipantResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Disconnects a participant. For security recommendations, see Amazon Connect Chat security best practices . ConnectionToken is used for invoking this API instead of ParticipantToken . The Amazon Connect Participant Se ... |
| `GetAttachment` | `POST /participant/attachment` | - | `AttachmentId`, `ConnectionToken` | - | `GetAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provides a pre-signed URL for download of a completed attachment. This is an asynchronous API for use with active contacts. For security recommendations, see Amazon Connect Chat security best practices . The particip ... |
| `GetAuthenticationUrl` | `POST /participant/authentication-url` | - | `SessionId`, `RedirectUri`, `ConnectionToken` | - | `GetAuthenticationUrlResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the AuthenticationUrl for the current authentication session for the AuthenticateCustomer flow block. For security recommendations, see Amazon Connect Chat security best practices . This API can only be cal ... |
| `GetTranscript` | `POST /participant/transcript` | `paginated` | `ConnectionToken` | - | `GetTranscriptResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a transcript of the session, including details about any attachments. For information about accessing past chat contact transcripts for a persistent chat, see Enable persistent chat . For security recommend ... |
| `SendEvent` | `POST /participant/event` | `idempotency-token` | `ContentType`, `ConnectionToken` | `ClientToken` | `SendEventResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | The application/vnd.amazonaws.connect.event.connection.acknowledged ContentType is no longer maintained since December 31, 2024. This event has been migrated to the CreateParticipantConnection API using the ConnectPa ... |
| `SendMessage` | `POST /participant/message` | `idempotency-token` | `ContentType`, `Content`, `ConnectionToken` | `ClientToken` | `SendMessageResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sends a message. For security recommendations, see Amazon Connect Chat security best practices . ConnectionToken is used for invoking this API instead of ParticipantToken . The Amazon Connect Participant Service APIs ... |
| `StartAttachmentUpload` | `POST /participant/start-attachment-upload` | `idempotency-token` | `ContentType`, `AttachmentSizeInBytes`, `AttachmentName`, `ClientToken`, `ConnectionToken` | `ClientToken` | `StartAttachmentUploadResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Provides a pre-signed Amazon S3 URL in response for uploading the file directly to S3. For security recommendations, see Amazon Connect Chat security best practices . ConnectionToken is used for invoking this API ins ... |

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
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message | The requested operation conflicts with the current state of a service resource associated with the request. |
| `InternalServerException` | `structure` | Message | This exception occurs when there is an internal failure in the Amazon Connect service. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The resource was not found. |
| `ServiceQuotaExceededException` | `structure` | Message | The number of attachments per contact exceeds the quota. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message | The input fails to satisfy the constraints specified by Amazon Connect. |
| `CancelParticipantAuthenticationRequest` | `structure` | SessionId, ConnectionToken | - |
| `CancelParticipantAuthenticationResponse` | `structure` | **empty (no members)** | - |
| `CompleteAttachmentUploadRequest` | `structure` | AttachmentIds, ClientToken, ConnectionToken | - |
| `CompleteAttachmentUploadResponse` | `structure` | **empty (no members)** | - |
| `CreateParticipantConnectionRequest` | `structure` | Type, ParticipantToken, ConnectParticipant | - |
| `CreateParticipantConnectionResponse` | `structure` | Websocket, ConnectionCredentials, WebRTCConnection | - |
| `DescribeViewRequest` | `structure` | ViewToken, ConnectionToken | - |
| `DescribeViewResponse` | `structure` | View | - |
| `DisconnectParticipantRequest` | `structure` | ClientToken, ConnectionToken | - |
| `DisconnectParticipantResponse` | `structure` | **empty (no members)** | - |
| `GetAttachmentRequest` | `structure` | AttachmentId, ConnectionToken, UrlExpiryInSeconds | - |
| `GetAttachmentResponse` | `structure` | Url, UrlExpiry, AttachmentSizeInBytes | - |
| `GetAuthenticationUrlRequest` | `structure` | SessionId, RedirectUri, ConnectionToken | - |
| `GetAuthenticationUrlResponse` | `structure` | AuthenticationUrl | - |
| `GetTranscriptRequest` | `structure` | ContactId, MaxResults, NextToken, ScanDirection, SortOrder, StartPosition, ConnectionToken | - |
| `GetTranscriptResponse` | `structure` | InitialContactId, Transcript, NextToken | - |
| `SendEventRequest` | `structure` | ContentType, Content, ClientToken, ConnectionToken | - |
| `SendEventResponse` | `structure` | Id, AbsoluteTime | - |
| `SendMessageRequest` | `structure` | ContentType, Content, ClientToken, ConnectionToken | - |
| `SendMessageResponse` | `structure` | Id, AbsoluteTime, MessageMetadata | - |
| `StartAttachmentUploadRequest` | `structure` | ContentType, AttachmentSizeInBytes, AttachmentName, ClientToken, ConnectionToken | - |
| `StartAttachmentUploadResponse` | `structure` | AttachmentId, UploadMetadata | - |
| `ArtifactStatus` | `enum` | APPROVED, REJECTED, IN_PROGRESS | - |
| `ChatItemType` | `enum` | TYPING, PARTICIPANT_JOINED, PARTICIPANT_LEFT, CHAT_ENDED, TRANSFER_SUCCEEDED, TRANSFER_FAILED, MESSAGE, EVENT, ATTACHMENT, CONNECTION_ACK, MESSAGE_DELIVERED, MESSAGE_READ | - |
| `ConnectionType` | `enum` | WEBSOCKET, CONNECTION_CREDENTIALS, WEBRTC_CONNECTION | - |
| `MeetingFeatureStatus` | `enum` | AVAILABLE, UNAVAILABLE | - |
| `MessageProcessingStatus` | `enum` | PROCESSING, FAILED, REJECTED | - |
| `ParticipantRole` | `enum` | AGENT, CUSTOMER, SYSTEM, CUSTOM_BOT, SUPERVISOR | - |
| `ResourceType` | `enum` | CONTACT, CONTACT_FLOW, INSTANCE, PARTICIPANT, HIERARCHY_LEVEL, HIERARCHY_GROUP, USER, PHONE_NUMBER | - |
| `ScanDirection` | `enum` | FORWARD, BACKWARD | - |
| `SortKey` | `enum` | DESCENDING, ASCENDING | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

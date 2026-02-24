# Amazon Interactive Video Service Chat

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Introduction The Amazon IVS Chat control-plane API enables you to create and manage Amazon IVS Chat resources. You also need to integrate with the Amazon IVS Chat Messaging API, to enable users to interact with chat rooms in real time. The API is an AWS regional service. For a list of supported regions and Amazon IVS Chat HTTPS service endpoints, see the Amazon IVS Chat information on the Amazon IVS page in the AWS General Reference . This document describes HTTP operations.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Interactive Video Service Chat workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Delete`, `List`, `Get`, `Update` operation families, including `CreateChatToken`, `CreateLoggingConfiguration`, `CreateRoom`, `DeleteLoggingConfiguration`, `DeleteMessage`, `DeleteRoom`.

## Service Identity and Protocol

- AWS model slug: `ivschat`
- AWS SDK for Rust slug: `ivschat`
- Model version: `2020-07-14`
- Model file: `vendor/api-models-aws/models/ivschat/service/2020-07-14/ivschat-2020-07-14.json`
- SDK ID: `ivschat`
- Endpoint prefix: `-`
- ARN namespace: `ivschat`
- CloudFormation name: `IVSChat`
- CloudTrail event source: `ivschat.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (3), `Delete` (3), `List` (3), `Get` (2), `Update` (2), `Disconnect` (1), `Send` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateChatToken`, `CreateLoggingConfiguration`, `CreateRoom`, `DeleteLoggingConfiguration`, `DeleteMessage`, `DeleteRoom`, `TagResource`, `UntagResource`, `UpdateLoggingConfiguration`, `UpdateRoom`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetLoggingConfiguration`, `GetRoom`, `ListLoggingConfigurations`, `ListRooms`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`.

## Operation Groups

### Create

- Operations: `CreateChatToken`, `CreateLoggingConfiguration`, `CreateRoom`
- Common required input members in this group: `destinationConfiguration`, `roomIdentifier`, `userId`

### Delete

- Operations: `DeleteLoggingConfiguration`, `DeleteMessage`, `DeleteRoom`
- Common required input members in this group: `id`, `identifier`, `roomIdentifier`

### List

- Operations: `ListLoggingConfigurations`, `ListRooms`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `resourceArn`

### Get

- Operations: `GetLoggingConfiguration`, `GetRoom`
- Traits: `readonly` (1)
- Common required input members in this group: `identifier`

### Update

- Operations: `UpdateLoggingConfiguration`, `UpdateRoom`
- Common required input members in this group: `identifier`

### Disconnect

- Operations: `DisconnectUser`
- Common required input members in this group: `roomIdentifier`, `userId`

### Send

- Operations: `SendEvent`
- Common required input members in this group: `eventName`, `roomIdentifier`

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
| `CreateChatToken` | `POST /CreateChatToken` | - | `roomIdentifier`, `userId` | - | `CreateChatTokenResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Creates an encrypted token that is used by a chat participant to establish an individual WebSocket chat connection to a room. When the token is used to connect to chat, the connection is valid for the session duration specified in the request. |
| `CreateLoggingConfiguration` | `POST /CreateLoggingConfiguration` | - | `destinationConfiguration` | - | `CreateLoggingConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a logging configuration that allows clients to store and record sent messages. |
| `CreateRoom` | `POST /CreateRoom` | - | - | - | `CreateRoomResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a room that allows clients to connect and pass messages. |
| `DeleteLoggingConfiguration` | `POST /DeleteLoggingConfiguration` | - | `identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified logging configuration. |
| `DeleteMessage` | `POST /DeleteMessage` | - | `id`, `roomIdentifier` | - | `DeleteMessageResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends an event to a specific room which directs clients to delete a specific message; that is, unrender it from view and delete it from the client’s chat history. This event’s `EventName` is `aws:DELETE_MESSAGE`. |
| `DeleteRoom` | `POST /DeleteRoom` | - | `identifier` | - | `Unit` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified room. |
| `DisconnectUser` | `POST /DisconnectUser` | - | `roomIdentifier`, `userId` | - | `DisconnectUserResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disconnects all connections using a specified user ID from a room. This replicates the DisconnectUser WebSocket operation in the Amazon IVS Chat Messaging API. |
| `GetLoggingConfiguration` | `POST /GetLoggingConfiguration` | - | `identifier` | - | `GetLoggingConfigurationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets the specified logging configuration. |
| `GetRoom` | `POST /GetRoom` | `readonly` | `identifier` | - | `GetRoomResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets the specified room. |
| `ListLoggingConfigurations` | `POST /ListLoggingConfigurations` | `readonly`, `paginated` | - | - | `ListLoggingConfigurationsResponse` | `AccessDeniedException`, `ValidationException` | Gets summary information about all your logging configurations in the AWS region where the API request is processed. |
| `ListRooms` | `POST /ListRooms` | `readonly`, `paginated` | - | - | `ListRoomsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Gets summary information about all your rooms in the AWS region where the API request is processed. Results are sorted in descending order of `updateTime`. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about AWS tags for the specified ARN. |
| `SendEvent` | `POST /SendEvent` | - | `eventName`, `roomIdentifier` | - | `SendEventResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends an event to a room. Use this within your application’s business logic to send events to clients of a room; e.g., to notify clients to change the way the chat UI is rendered. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds or updates tags for the AWS resource with the specified ARN. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from the resource with the specified ARN. |
| `UpdateLoggingConfiguration` | `POST /UpdateLoggingConfiguration` | - | `identifier` | - | `UpdateLoggingConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Updates a specified logging configuration. |
| `UpdateRoom` | `POST /UpdateRoom` | - | `identifier` | - | `UpdateRoomResponse` | `AccessDeniedException`, `PendingVerification`, `ResourceNotFoundException`, `ValidationException` | Updates a room’s configuration. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | - |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | - |
| `AccessDeniedException` | `structure` | `message` | - |
| `PendingVerification` | `structure` | `message` | - |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | - |
| `ThrottlingException` | `structure` | `limit`, `message`, `resourceId`, `resourceType` | - |
| `InternalServerException` | `structure` | `message` | - |
| `ServiceQuotaExceededException` | `structure` | `limit`, `message`, `resourceId`, `resourceType` | - |
| `CreateChatTokenRequest` | `structure` | `attributes`, `capabilities`, `roomIdentifier`, `sessionDurationInMinutes`, `userId` | - |
| `CreateChatTokenResponse` | `structure` | `sessionExpirationTime`, `token`, `tokenExpirationTime` | - |
| `CreateLoggingConfigurationRequest` | `structure` | `destinationConfiguration`, `name`, `tags` | - |
| `CreateLoggingConfigurationResponse` | `structure` | `arn`, `createTime`, `destinationConfiguration`, `id`, `name`, `state`, `tags`, `updateTime` | - |
| `CreateRoomRequest` | `structure` | `loggingConfigurationIdentifiers`, `maximumMessageLength`, `maximumMessageRatePerSecond`, `messageReviewHandler`, `name`, `tags` | - |
| `CreateRoomResponse` | `structure` | `arn`, `createTime`, `id`, `loggingConfigurationIdentifiers`, `maximumMessageLength`, `maximumMessageRatePerSecond`, `messageReviewHandler`, `name`, `tags`, `updateTime` | - |
| `DeleteLoggingConfigurationRequest` | `structure` | `identifier` | - |
| `DeleteMessageRequest` | `structure` | `id`, `reason`, `roomIdentifier` | - |
| `DeleteMessageResponse` | `structure` | `id` | - |
| `DeleteRoomRequest` | `structure` | `identifier` | - |
| `DisconnectUserRequest` | `structure` | `reason`, `roomIdentifier`, `userId` | - |
| `DisconnectUserResponse` | `structure` | - | - |
| `GetLoggingConfigurationRequest` | `structure` | `identifier` | - |
| `GetLoggingConfigurationResponse` | `structure` | `arn`, `createTime`, `destinationConfiguration`, `id`, `name`, `state`, `tags`, `updateTime` | - |
| `GetRoomRequest` | `structure` | `identifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

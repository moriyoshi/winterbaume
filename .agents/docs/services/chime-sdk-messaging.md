# Amazon Chime SDK Messaging

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Chime SDK messaging APIs in this section allow software developers to send and receive messages in custom messaging applications. These APIs depend on the frameworks provided by the Amazon Chime SDK identity APIs. For more information about the messaging APIs, see Amazon Chime SDK messaging.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Chime SDK Messaging where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Chime SDK Messaging by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Chime SDK Messaging by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Chime SDK Messaging workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Describe`, `Create`, `Get` operation families, including `ListChannelBans`, `ListChannelFlows`, `ListChannelMemberships`, `ListChannelMembershipsForAppInstanceUser`, `DeleteChannel`, `DeleteChannelBan`.

## Service Identity and Protocol

- AWS model slug: `chime-sdk-messaging`
- AWS SDK for Rust slug: `chimesdkmessaging`
- Model version: `2021-05-15`
- Model file: `vendor/api-models-aws/models/chime-sdk-messaging/service/2021-05-15/chime-sdk-messaging-2021-05-15.json`
- SDK ID: `Chime SDK Messaging`
- Endpoint prefix: `messaging-chime`
- ARN namespace: `chime`
- CloudFormation name: `ChimeSDKMessaging`
- CloudTrail event source: `chimesdkmessaging.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Delete` (7), `Describe` (7), `Create` (5), `Get` (5), `Update` (4), `Put` (3), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateChannelFlow`, `BatchCreateChannelMembership`, `CreateChannel`, `CreateChannelBan`, `CreateChannelFlow`, `CreateChannelMembership`, `CreateChannelModerator`, `DeleteChannel`, `DeleteChannelBan`, `DeleteChannelFlow`, `DeleteChannelMembership`, `DeleteChannelMessage`, `DeleteChannelModerator`, `DeleteMessagingStreamingConfigurations`, `DisassociateChannelFlow`, `PutChannelExpirationSettings`, `PutChannelMembershipPreferences`, `PutMessagingStreamingConfigurations`, `TagResource`, `UntagResource`, ... (+4).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeChannel`, `DescribeChannelBan`, `DescribeChannelFlow`, `DescribeChannelMembership`, `DescribeChannelMembershipForAppInstanceUser`, `DescribeChannelModeratedByAppInstanceUser`, `DescribeChannelModerator`, `GetChannelMembershipPreferences`, `GetChannelMessage`, `GetChannelMessageStatus`, `GetMessagingSessionEndpoint`, `GetMessagingStreamingConfigurations`, `ListChannelBans`, `ListChannelFlows`, `ListChannelMemberships`, `ListChannelMembershipsForAppInstanceUser`, `ListChannelMessages`, `ListChannelModerators`, `ListChannels`, `ListChannelsAssociatedWithChannelFlow`, ... (+4).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 51 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Lambda`, `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/chime-sdk/latest/dg/using-the-messaging-sdk.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/using-channel-flows.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/integrate-client-library.html

Research outcomes:
- Chime SDK messaging provides app instances, channels, channel memberships, messages, push notifications, elastic channels, bots, retention, and channel flows.
- Channels organise messages and memberships under an app instance.
- Channel flows process in-flight messages with business logic before delivery, such as filtering or transformation.
- Client libraries use AWS SDK APIs plus WebSocket connections for real-time messaging features.
- Messaging supports push notifications and retention configuration.
- Elastic channels and namespace migration introduce feature differences from older messaging namespaces.

Parity implications:
- Model channels, memberships, moderators, messages, message statuses, channel flows, processors, push preferences, and retention separately.
- SendMessage should evaluate membership/authorisation, channel flow processing, and retention state.
- WebSocket session state should be distinct from persisted channel/message control-plane state.

## Operation Groups

### List

- Operations: `ListChannelBans`, `ListChannelFlows`, `ListChannelMemberships`, `ListChannelMembershipsForAppInstanceUser`, `ListChannelMessages`, `ListChannelModerators`, `ListChannels`, `ListChannelsAssociatedWithChannelFlow`, `ListChannelsModeratedByAppInstanceUser`, `ListSubChannels`, `ListTagsForResource`
- Traits: `paginated` (10)
- Common required input members in this group: `ChannelArn`, `ChimeBearer`, `AppInstanceArn`

### Delete

- Operations: `DeleteChannel`, `DeleteChannelBan`, `DeleteChannelFlow`, `DeleteChannelMembership`, `DeleteChannelMessage`, `DeleteChannelModerator`, `DeleteMessagingStreamingConfigurations`
- Common required input members in this group: `ChannelArn`, `ChimeBearer`, `MemberArn`

### Describe

- Operations: `DescribeChannel`, `DescribeChannelBan`, `DescribeChannelFlow`, `DescribeChannelMembership`, `DescribeChannelMembershipForAppInstanceUser`, `DescribeChannelModeratedByAppInstanceUser`, `DescribeChannelModerator`
- Common required input members in this group: `ChannelArn`, `ChimeBearer`, `MemberArn`, `AppInstanceUserArn`

### Create

- Operations: `CreateChannel`, `CreateChannelBan`, `CreateChannelFlow`, `CreateChannelMembership`, `CreateChannelModerator`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `AppInstanceArn`, `Name`, `ClientRequestToken`, `ChimeBearer`, `ChannelArn`, `MemberArn`

### Get

- Operations: `GetChannelMembershipPreferences`, `GetChannelMessage`, `GetChannelMessageStatus`, `GetMessagingSessionEndpoint`, `GetMessagingStreamingConfigurations`
- Common required input members in this group: `ChannelArn`, `ChimeBearer`, `MessageId`

### Update

- Operations: `UpdateChannel`, `UpdateChannelFlow`, `UpdateChannelMessage`, `UpdateChannelReadMarker`
- Common required input members in this group: `ChannelArn`, `ChimeBearer`

### Put

- Operations: `PutChannelExpirationSettings`, `PutChannelMembershipPreferences`, `PutMessagingStreamingConfigurations`
- Common required input members in this group: `ChannelArn`

### Associate

- Operations: `AssociateChannelFlow`
- Common required input members in this group: -

### Batch

- Operations: `BatchCreateChannelMembership`
- Common required input members in this group: -

### Channel

- Operations: `ChannelFlowCallback`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateChannelFlow`
- Common required input members in this group: -

### Redact

- Operations: `RedactChannelMessage`
- Common required input members in this group: -

### Search

- Operations: `SearchChannels`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Send

- Operations: `SendChannelMessage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateChannelFlow` | `PUT /channels/{ChannelArn}/channel-flow` | - | `ChannelArn`, `ChannelFlowArn`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Associates a channel flow with a channel. Once associated, all messages to that channel go through channel flow processors. To stop processing, use the DisassociateChannelFlow API. Only administrators or channel mode ... |
| `BatchCreateChannelMembership` | `POST /channels/{ChannelArn}/memberships?operation=batch-create` | - | `ChannelArn`, `MemberArns`, `ChimeBearer` | - | `BatchCreateChannelMembershipResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Adds a specified number of users and bots to a channel. |
| `ChannelFlowCallback` | `POST /channels/{ChannelArn}?operation=channel-flow-callback` | `idempotency-token` | `CallbackId`, `ChannelArn`, `ChannelMessage` | `CallbackId` | `ChannelFlowCallbackResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Calls back Amazon Chime SDK messaging with a processing response message. This should be invoked from the processor Lambda. This is a developer API. You can return one of the following processing responses: Update me ... |
| `CreateChannel` | `POST /channels` | `idempotency-token` | `AppInstanceArn`, `Name`, `ClientRequestToken`, `ChimeBearer` | `ClientRequestToken` | `CreateChannelResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a channel to which you can add users and send messages. Restriction : You can't change a channel's privacy. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanc ... |
| `CreateChannelBan` | `POST /channels/{ChannelArn}/bans` | - | `ChannelArn`, `MemberArn`, `ChimeBearer` | - | `CreateChannelBanResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Permanently bans a member from a channel. Moderators can't add banned members to a channel. To undo a ban, you first have to DeleteChannelBan , and then CreateChannelMembership . Bans are cleaned up when you delete u ... |
| `CreateChannelFlow` | `POST /channel-flows` | `idempotency-token` | `AppInstanceArn`, `Processors`, `Name`, `ClientRequestToken` | `ClientRequestToken` | `CreateChannelFlowResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a channel flow, a container for processors. Processors are AWS Lambda functions that perform actions on chat messages, such as stripping out profanity. You can associate channel flows with channels, and the p ... |
| `CreateChannelMembership` | `POST /channels/{ChannelArn}/memberships` | - | `ChannelArn`, `MemberArn`, `Type`, `ChimeBearer` | - | `CreateChannelMembershipResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Adds a member to a channel. The InvitedBy field in ChannelMembership is derived from the request header. A channel member can: List messages Send messages Receive messages Edit their own messages Leave the channel Pr ... |
| `CreateChannelModerator` | `POST /channels/{ChannelArn}/moderators` | - | `ChannelArn`, `ChannelModeratorArn`, `ChimeBearer` | - | `CreateChannelModeratorResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a new ChannelModerator . A channel moderator can: Add and remove other members of the channel. Add and remove other moderators of the channel. Add and remove user bans for the channel. Redact messages in the ... |
| `DeleteChannel` | `DELETE /channels/{ChannelArn}` | - | `ChannelArn`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Immediately makes a channel and its memberships inaccessible and marks them for deletion. This is an irreversible process. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUserArn or ... |
| `DeleteChannelBan` | `DELETE /channels/{ChannelArn}/bans/{MemberArn}` | - | `ChannelArn`, `MemberArn`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes a member from a channel's ban list. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `DeleteChannelFlow` | `DELETE /channel-flows/{ChannelFlowArn}` | - | `ChannelFlowArn` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes a channel flow, an irreversible process. This is a developer API. This API works only when the channel flow is not associated with any channel. To get a list of all channels that a channel flow is associated ... |
| `DeleteChannelMembership` | `DELETE /channels/{ChannelArn}/memberships/{MemberArn}` | - | `ChannelArn`, `MemberArn`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes a member from a channel. The x-amz-chime-bearer request header is mandatory. Use the AppInstanceUserArn of the user that makes the API call as the value in the header. |
| `DeleteChannelMessage` | `DELETE /channels/{ChannelArn}/messages/{MessageId}` | - | `ChannelArn`, `MessageId`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes a channel message. Only admins can perform this action. Deletion makes messages inaccessible immediately. A background process deletes any revisions created by UpdateChannelMessage . The x-amz-chime-bearer re ... |
| `DeleteChannelModerator` | `DELETE /channels/{ChannelArn}/moderators/{ChannelModeratorArn}` | - | `ChannelArn`, `ChannelModeratorArn`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes a channel moderator. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `DeleteMessagingStreamingConfigurations` | `DELETE /app-instances/{AppInstanceArn}/streaming-configurations` | - | `AppInstanceArn` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes the streaming configurations for an AppInstance . For more information, see Streaming messaging data in the Amazon Chime SDK Developer Guide . |
| `DescribeChannel` | `GET /channels/{ChannelArn}` | - | `ChannelArn`, `ChimeBearer` | - | `DescribeChannelResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of a channel in an Amazon Chime AppInstance . The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in ... |
| `DescribeChannelBan` | `GET /channels/{ChannelArn}/bans/{MemberArn}` | - | `ChannelArn`, `MemberArn`, `ChimeBearer` | - | `DescribeChannelBanResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of a channel ban. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `DescribeChannelFlow` | `GET /channel-flows/{ChannelFlowArn}` | - | `ChannelFlowArn` | - | `DescribeChannelFlowResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of a channel flow in an Amazon Chime AppInstance . This is a developer API. |
| `DescribeChannelMembership` | `GET /channels/{ChannelArn}/memberships/{MemberArn}` | - | `ChannelArn`, `MemberArn`, `ChimeBearer` | - | `DescribeChannelMembershipResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of a user's channel membership. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `DescribeChannelMembershipForAppInstanceUser` | `GET /channels/{ChannelArn}?scope=app-instance-user-membership` | - | `ChannelArn`, `AppInstanceUserArn`, `ChimeBearer` | - | `DescribeChannelMembershipForAppInstanceUserResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the details of a channel based on the membership of the specified AppInstanceUser or AppInstanceBot . The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot t ... |
| `DescribeChannelModeratedByAppInstanceUser` | `GET /channels/{ChannelArn}?scope=app-instance-user-moderated-channel` | - | `ChannelArn`, `AppInstanceUserArn`, `ChimeBearer` | - | `DescribeChannelModeratedByAppInstanceUserResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of a channel moderated by the specified AppInstanceUser or AppInstanceBot . The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes ... |
| `DescribeChannelModerator` | `GET /channels/{ChannelArn}/moderators/{ChannelModeratorArn}` | - | `ChannelArn`, `ChannelModeratorArn`, `ChimeBearer` | - | `DescribeChannelModeratorResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of a single ChannelModerator. The x-amz-chime-bearer request header is mandatory. Use the AppInstanceUserArn of the user that makes the API call as the value in the header. |
| `DisassociateChannelFlow` | `DELETE /channels/{ChannelArn}/channel-flow/{ChannelFlowArn}` | - | `ChannelArn`, `ChannelFlowArn`, `ChimeBearer` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Disassociates a channel flow from all its channels. Once disassociated, all messages to that channel stop going through the channel flow processor. Only administrators or channel moderators can disassociate a channel ... |
| `GetChannelMembershipPreferences` | `GET /channels/{ChannelArn}/memberships/{MemberArn}/preferences` | - | `ChannelArn`, `MemberArn`, `ChimeBearer` | - | `GetChannelMembershipPreferencesResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets the membership preferences of an AppInstanceUser or AppInstanceBot for the specified channel. A user or a bot must be a member of the channel and own the membership in order to retrieve membership preferences. U ... |
| `GetChannelMessage` | `GET /channels/{ChannelArn}/messages/{MessageId}` | - | `ChannelArn`, `MessageId`, `ChimeBearer` | - | `GetChannelMessageResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets the full details of a channel message. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `GetChannelMessageStatus` | `GET /channels/{ChannelArn}/messages/{MessageId}?scope=message-status` | - | `ChannelArn`, `MessageId`, `ChimeBearer` | - | `GetChannelMessageStatusResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets message status for a specified messageId . Use this API to determine the intermediate status of messages going through channel flow processing. The API provides an alternative to retrieving message status if the ... |
| `GetMessagingSessionEndpoint` | `GET /endpoints/messaging-session` | - | - | - | `GetMessagingSessionEndpointResponse` | `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | The details of the endpoint for the messaging session. |
| `GetMessagingStreamingConfigurations` | `GET /app-instances/{AppInstanceArn}/streaming-configurations` | - | `AppInstanceArn` | - | `GetMessagingStreamingConfigurationsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves the data streaming configuration for an AppInstance . For more information, see Streaming messaging data in the Amazon Chime SDK Developer Guide . |
| `ListChannelBans` | `GET /channels/{ChannelArn}/bans` | `paginated` | `ChannelArn`, `ChimeBearer` | - | `ListChannelBansResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all the users and bots banned from a particular channel. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `ListChannelFlows` | `GET /channel-flows` | `paginated` | `AppInstanceArn` | - | `ListChannelFlowsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns a paginated lists of all the channel flows created under a single Chime. This is a developer API. |
| `ListChannelMemberships` | `GET /channels/{ChannelArn}/memberships` | `paginated` | `ChannelArn`, `ChimeBearer` | - | `ListChannelMembershipsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all channel memberships in a channel. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. If you want to ... |
| `ListChannelMembershipsForAppInstanceUser` | `GET /channels?scope=app-instance-user-memberships` | `paginated` | `ChimeBearer` | - | `ListChannelMembershipsForAppInstanceUserResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all channels that an AppInstanceUser or AppInstanceBot is a part of. Only an AppInstanceAdmin can call the API with a user ARN that is not their own. The x-amz-chime-bearer request header is mandatory. Use the ... |
| `ListChannelMessages` | `GET /channels/{ChannelArn}/messages` | `paginated` | `ChannelArn`, `ChimeBearer` | - | `ListChannelMessagesResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | List all the messages in a channel. Returns a paginated list of ChannelMessages . By default, sorted by creation timestamp in descending order. Redacted messages appear in the results as empty, since they are only re ... |
| `ListChannelModerators` | `GET /channels/{ChannelArn}/moderators` | `paginated` | `ChannelArn`, `ChimeBearer` | - | `ListChannelModeratorsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all the moderators for a channel. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `ListChannels` | `GET /channels` | `paginated` | `AppInstanceArn`, `ChimeBearer` | - | `ListChannelsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all Channels created under a single Chime App as a paginated list. You can specify filters to narrow results. Functionality & restrictions Use privacy = PUBLIC to retrieve all public channels in the account. On ... |
| `ListChannelsAssociatedWithChannelFlow` | `GET /channels?scope=channel-flow-associations` | `paginated` | `ChannelFlowArn` | - | `ListChannelsAssociatedWithChannelFlowResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all channels associated with a specified channel flow. You can associate a channel flow with multiple channels, but you can only associate a channel with one channel flow. This is a developer API. |
| `ListChannelsModeratedByAppInstanceUser` | `GET /channels?scope=app-instance-user-moderated-channels` | `paginated` | `ChimeBearer` | - | `ListChannelsModeratedByAppInstanceUserResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | A list of the channels moderated by an AppInstanceUser . The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `ListSubChannels` | `GET /channels/{ChannelArn}/subchannels` | `paginated` | `ChannelArn`, `ChimeBearer` | - | `ListSubChannelsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all the SubChannels in an elastic channel when given a channel ID. Available only to the app instance admins and channel moderators of elastic channels. |
| `ListTagsForResource` | `GET /tags` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the tags applied to an Amazon Chime SDK messaging resource. |
| `PutChannelExpirationSettings` | `PUT /channels/{ChannelArn}/expiration-settings` | - | `ChannelArn` | - | `PutChannelExpirationSettingsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sets the number of days before the channel is automatically deleted. A background process deletes expired channels within 6 hours of expiration. Actual deletion times may vary. Expired channels that have not yet been ... |
| `PutChannelMembershipPreferences` | `PUT /channels/{ChannelArn}/memberships/{MemberArn}/preferences` | - | `ChannelArn`, `MemberArn`, `ChimeBearer`, `Preferences` | - | `PutChannelMembershipPreferencesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sets the membership preferences of an AppInstanceUser or AppInstanceBot for the specified channel. The user or bot must be a member of the channel. Only the user or bot who owns the membership can set preferences. Us ... |
| `PutMessagingStreamingConfigurations` | `PUT /app-instances/{AppInstanceArn}/streaming-configurations` | - | `AppInstanceArn`, `StreamingConfigurations` | - | `PutMessagingStreamingConfigurationsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sets the data streaming configuration for an AppInstance . For more information, see Streaming messaging data in the Amazon Chime SDK Developer Guide . |
| `RedactChannelMessage` | `POST /channels/{ChannelArn}/messages/{MessageId}?operation=redact` | - | `ChannelArn`, `MessageId`, `ChimeBearer` | - | `RedactChannelMessageResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Redacts message content and metadata. The message exists in the back end, but the action returns null content, and the state shows as redacted. The x-amz-chime-bearer request header is mandatory. Use the ARN of the A ... |
| `SearchChannels` | `POST /channels?operation=search` | `paginated` | `Fields` | - | `SearchChannelsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Allows the ChimeBearer to search channels by channel members. Users or bots can search across the channels that they belong to. Users in the AppInstanceAdmin role can search across all channels. The x-amz-chime-beare ... |
| `SendChannelMessage` | `POST /channels/{ChannelArn}/messages` | `idempotency-token` | `ChannelArn`, `Content`, `Type`, `Persistence`, `ClientRequestToken`, `ChimeBearer` | `ClientRequestToken` | `SendChannelMessageResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sends a message to a particular channel that the member is a part of. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in t ... |
| `TagResource` | `POST /tags?operation=tag-resource` | - | `ResourceARN`, `Tags` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Applies the specified tags to the specified Amazon Chime SDK messaging resource. |
| `UntagResource` | `POST /tags?operation=untag-resource` | - | `ResourceARN`, `TagKeys` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes the specified tags from the specified Amazon Chime SDK messaging resource. |
| `UpdateChannel` | `PUT /channels/{ChannelArn}` | - | `ChannelArn`, `ChimeBearer` | - | `UpdateChannelResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Update a channel's attributes. Restriction : You can't change a channel's privacy. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as t ... |
| `UpdateChannelFlow` | `PUT /channel-flows/{ChannelFlowArn}` | - | `ChannelFlowArn`, `Processors`, `Name` | - | `UpdateChannelFlowResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates channel flow attributes. This is a developer API. |
| `UpdateChannelMessage` | `PUT /channels/{ChannelArn}/messages/{MessageId}` | - | `ChannelArn`, `MessageId`, `Content`, `ChimeBearer` | - | `UpdateChannelMessageResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the content of a message. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in the header. |
| `UpdateChannelReadMarker` | `PUT /channels/{ChannelArn}/readMarker` | - | `ChannelArn`, `ChimeBearer` | - | `UpdateChannelReadMarkerResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | The details of the time when a user last read messages in a channel. The x-amz-chime-bearer request header is mandatory. Use the ARN of the AppInstanceUser or AppInstanceBot that makes the API call as the value in th ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AssociateChannelFlow` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `BatchCreateChannelMembership` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `CreateChannel` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `CreateChannelBan` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `CreateChannelMembership` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `CreateChannelModerator` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DeleteChannel` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DeleteChannelBan` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DeleteChannelMembership` | `ChimeBearer -> x-amz-chime-bearer` | `SubChannelId -> sub-channel-id` | - | - |
| `DeleteChannelMessage` | `ChimeBearer -> x-amz-chime-bearer` | `SubChannelId -> sub-channel-id` | - | - |
| `DeleteChannelModerator` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DescribeChannel` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DescribeChannelBan` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DescribeChannelMembership` | `ChimeBearer -> x-amz-chime-bearer` | `SubChannelId -> sub-channel-id` | - | - |
| `DescribeChannelMembershipForAppInstanceUser` | `ChimeBearer -> x-amz-chime-bearer` | `AppInstanceUserArn -> app-instance-user-arn` | - | - |
| `DescribeChannelModeratedByAppInstanceUser` | `ChimeBearer -> x-amz-chime-bearer` | `AppInstanceUserArn -> app-instance-user-arn` | - | - |
| `DescribeChannelModerator` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `DisassociateChannelFlow` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `GetChannelMembershipPreferences` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `GetChannelMessage` | `ChimeBearer -> x-amz-chime-bearer` | `SubChannelId -> sub-channel-id` | - | - |
| `GetChannelMessageStatus` | `ChimeBearer -> x-amz-chime-bearer` | `SubChannelId -> sub-channel-id` | - | - |
| `GetMessagingSessionEndpoint` | - | `NetworkType -> network-type` | - | - |
| `ListChannelBans` | `ChimeBearer -> x-amz-chime-bearer` | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListChannelFlows` | - | `AppInstanceArn -> app-instance-arn`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListChannelMemberships` | `ChimeBearer -> x-amz-chime-bearer` | `Type -> type`, `MaxResults -> max-results`, `NextToken -> next-token`, `SubChannelId -> sub-channel-id` | - | - |
| `ListChannelMembershipsForAppInstanceUser` | `ChimeBearer -> x-amz-chime-bearer` | `AppInstanceUserArn -> app-instance-user-arn`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListChannelMessages` | `ChimeBearer -> x-amz-chime-bearer` | `SortOrder -> sort-order`, `NotBefore -> not-before`, `NotAfter -> not-after`, `MaxResults -> max-results`, `NextToken -> next-token`, `SubChannelId -> sub-channel-id` | - | - |
| `ListChannelModerators` | `ChimeBearer -> x-amz-chime-bearer` | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListChannels` | `ChimeBearer -> x-amz-chime-bearer` | `AppInstanceArn -> app-instance-arn`, `Privacy -> privacy`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListChannelsAssociatedWithChannelFlow` | - | `ChannelFlowArn -> channel-flow-arn`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListChannelsModeratedByAppInstanceUser` | `ChimeBearer -> x-amz-chime-bearer` | `AppInstanceUserArn -> app-instance-user-arn`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListSubChannels` | `ChimeBearer -> x-amz-chime-bearer` | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListTagsForResource` | - | `ResourceARN -> arn` | - | - |
| `PutChannelExpirationSettings` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `PutChannelMembershipPreferences` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `RedactChannelMessage` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `SearchChannels` | `ChimeBearer -> x-amz-chime-bearer` | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `SendChannelMessage` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `UpdateChannel` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `UpdateChannelMessage` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |
| `UpdateChannelReadMarker` | `ChimeBearer -> x-amz-chime-bearer` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Code, Message | The input parameters don't match the service's restrictions. |
| `ConflictException` | `structure` | Code, Message | The request could not be processed because of conflict in the current state of the resource. |
| `ForbiddenException` | `structure` | Code, Message | The client is permanently forbidden from making the request. |
| `NotFoundException` | `structure` | Code, Message | One or more of the resources in the request does not exist in the system. |
| `ResourceLimitExceededException` | `structure` | Code, Message | The request exceeds the resource limit. |
| `ServiceFailureException` | `structure` | Code, Message | The service encountered an unexpected error. |
| `ServiceUnavailableException` | `structure` | Code, Message | The service is currently unavailable. |
| `ThrottledClientException` | `structure` | Code, Message | The client exceeded its request rate limit. |
| `UnauthorizedClientException` | `structure` | Code, Message | The client is not currently authorized to make the request. |
| `AssociateChannelFlowRequest` | `structure` | ChannelArn, ChannelFlowArn, ChimeBearer | - |
| `BatchCreateChannelMembershipRequest` | `structure` | ChannelArn, Type, MemberArns, ChimeBearer, SubChannelId | - |
| `BatchCreateChannelMembershipResponse` | `structure` | BatchChannelMemberships, Errors | - |
| `ChannelFlowCallbackRequest` | `structure` | CallbackId, ChannelArn, DeleteResource, ChannelMessage | - |
| `ChannelFlowCallbackResponse` | `structure` | ChannelArn, CallbackId | - |
| `CreateChannelRequest` | `structure` | AppInstanceArn, Name, Mode, Privacy, Metadata, ClientRequestToken, Tags, ChimeBearer, ChannelId, MemberArns, ModeratorArns, ElasticChannelConfiguration, ... (+1) | - |
| `CreateChannelResponse` | `structure` | ChannelArn | - |
| `CreateChannelBanRequest` | `structure` | ChannelArn, MemberArn, ChimeBearer | - |
| `CreateChannelBanResponse` | `structure` | ChannelArn, Member | - |
| `CreateChannelFlowRequest` | `structure` | AppInstanceArn, Processors, Name, Tags, ClientRequestToken | - |
| `CreateChannelFlowResponse` | `structure` | ChannelFlowArn | - |
| `CreateChannelMembershipRequest` | `structure` | ChannelArn, MemberArn, Type, ChimeBearer, SubChannelId | - |
| `CreateChannelMembershipResponse` | `structure` | ChannelArn, Member, SubChannelId | - |
| `CreateChannelModeratorRequest` | `structure` | ChannelArn, ChannelModeratorArn, ChimeBearer | - |
| `CreateChannelModeratorResponse` | `structure` | ChannelArn, ChannelModerator | - |
| `DeleteChannelRequest` | `structure` | ChannelArn, ChimeBearer | - |
| `DeleteChannelBanRequest` | `structure` | ChannelArn, MemberArn, ChimeBearer | - |
| `DeleteChannelFlowRequest` | `structure` | ChannelFlowArn | - |
| `DeleteChannelMembershipRequest` | `structure` | ChannelArn, MemberArn, ChimeBearer, SubChannelId | - |
| `DeleteChannelMessageRequest` | `structure` | ChannelArn, MessageId, ChimeBearer, SubChannelId | - |
| `DeleteChannelModeratorRequest` | `structure` | ChannelArn, ChannelModeratorArn, ChimeBearer | - |
| `DeleteMessagingStreamingConfigurationsRequest` | `structure` | AppInstanceArn | - |
| `DescribeChannelRequest` | `structure` | ChannelArn, ChimeBearer | - |
| `DescribeChannelResponse` | `structure` | Channel | - |
| `DescribeChannelBanRequest` | `structure` | ChannelArn, MemberArn, ChimeBearer | - |
| `DescribeChannelBanResponse` | `structure` | ChannelBan | - |
| `DescribeChannelFlowRequest` | `structure` | ChannelFlowArn | - |
| `DescribeChannelFlowResponse` | `structure` | ChannelFlow | - |
| `DescribeChannelMembershipRequest` | `structure` | ChannelArn, MemberArn, ChimeBearer, SubChannelId | - |
| `DescribeChannelMembershipResponse` | `structure` | ChannelMembership | - |
| `DescribeChannelMembershipForAppInstanceUserRequest` | `structure` | ChannelArn, AppInstanceUserArn, ChimeBearer | - |
| `AllowNotifications` | `enum` | ALL, NONE, FILTERED | - |
| `ChannelMembershipType` | `enum` | DEFAULT, HIDDEN | - |
| `ChannelMessagePersistenceType` | `enum` | PERSISTENT, NON_PERSISTENT | - |
| `ChannelMessageStatus` | `enum` | SENT, PENDING, FAILED, DENIED | - |
| `ChannelMessageType` | `enum` | STANDARD, CONTROL | - |
| `ChannelMode` | `enum` | UNRESTRICTED, RESTRICTED | - |
| `ChannelPrivacy` | `enum` | PUBLIC, PRIVATE | - |
| `ErrorCode` | `enum` | BadRequest, Conflict, Forbidden, NotFound, PreconditionFailed, ResourceLimitExceeded, ServiceFailure, AccessDenied, ServiceUnavailable, Throttled, Throttling, Unauthorized, ... (+3) | - |
| `ExpirationCriterion` | `enum` | CREATED_TIMESTAMP, LAST_MESSAGE_TIMESTAMP | - |
| `FallbackAction` | `enum` | CONTINUE, ABORT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

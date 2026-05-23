# Amazon Chime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Most of these APIs are no longer supported and will not be updated. We recommend using the latest versions in the Amazon Chime SDK API reference, in the Amazon Chime SDK. Using the latest versions requires migrating to dedicated namespaces. For more information, refer to Migrating from the Amazon Chime namespace in the Amazon Chime SDK Developer Guide . The Amazon Chime application programming interface (API) is designed so administrators can perform key tasks, such as creating and managing Amazon Chime accounts, users, and Voice Connectors.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Chime where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Chime by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Chime by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Chime workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Update`, `List`, `Create`, `Batch` operation families, including `GetAccount`, `GetAccountSettings`, `GetBot`, `GetEventsConfiguration`, `UpdateAccount`, `UpdateAccountSettings`.

## Service Identity and Protocol

- AWS model slug: `chime`
- AWS SDK for Rust slug: `chime`
- Model version: `2018-05-01`
- Model file: `vendor/api-models-aws/models/chime/service/2018-05-01/chime-2018-05-01.json`
- SDK ID: `Chime`
- Endpoint prefix: `chime`
- ARN namespace: `chime`
- CloudFormation name: `Chime`
- CloudTrail event source: `chime.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (12), `Update` (10), `List` (8), `Create` (7), `Batch` (6), `Delete` (5), `Associate` (2), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociatePhoneNumberWithUser`, `AssociateSigninDelegateGroupsWithAccount`, `BatchCreateRoomMembership`, `BatchDeletePhoneNumber`, `BatchSuspendUser`, `BatchUnsuspendUser`, `BatchUpdatePhoneNumber`, `BatchUpdateUser`, `CreateAccount`, `CreateBot`, `CreateMeetingDialOut`, `CreatePhoneNumberOrder`, `CreateRoom`, `CreateRoomMembership`, `CreateUser`, `DeleteAccount`, `DeleteEventsConfiguration`, `DeletePhoneNumber`, `DeleteRoom`, `DeleteRoomMembership`, ... (+15).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccount`, `GetAccountSettings`, `GetBot`, `GetEventsConfiguration`, `GetGlobalSettings`, `GetPhoneNumber`, `GetPhoneNumberOrder`, `GetPhoneNumberSettings`, `GetRetentionSettings`, `GetRoom`, `GetUser`, `GetUserSettings`, `ListAccounts`, `ListBots`, `ListPhoneNumberOrders`, `ListPhoneNumbers`, `ListRoomMemberships`, `ListRooms`, `ListSupportedPhoneNumberCountries`, `ListUsers`, ... (+1).
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 62 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SQS`, `Lambda`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/chime/latest/ag/what-is-chime.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/meetings-sdk.html

Research outcomes:
- The original Amazon Chime service manages business calling, meetings, users, accounts, phone numbers, and Voice Connector resources.
- Amazon Chime SDK is the newer programmable real-time communications service surface for meetings, messaging, media pipelines, and voice.
- Chime accounts and users are administrative resources distinct from Chime SDK meeting attendees.
- Voice resources such as phone numbers and Voice Connectors have telecom-specific lifecycle and configuration.
- Some Chime APIs are legacy surfaces while new application development often uses Chime SDK service APIs.

Parity implications:
- Model legacy Chime accounts/users/phone resources separately from Chime SDK meetings, messaging, voice, and media pipeline resources.
- Avoid assuming Chime SDK attendee semantics apply to legacy Chime users.
- Telecom resources should retain phone-number assignment and voice connector state independently from meeting state.

## Operation Groups

### Get

- Operations: `GetAccount`, `GetAccountSettings`, `GetBot`, `GetEventsConfiguration`, `GetGlobalSettings`, `GetPhoneNumber`, `GetPhoneNumberOrder`, `GetPhoneNumberSettings`, `GetRetentionSettings`, `GetRoom`, `GetUser`, `GetUserSettings`
- Common required input members in this group: `AccountId`, `BotId`, `PhoneNumberId`, `PhoneNumberOrderId`, `RoomId`, `UserId`

### Update

- Operations: `UpdateAccount`, `UpdateAccountSettings`, `UpdateBot`, `UpdateGlobalSettings`, `UpdatePhoneNumber`, `UpdatePhoneNumberSettings`, `UpdateRoom`, `UpdateRoomMembership`, `UpdateUser`, `UpdateUserSettings`
- Common required input members in this group: `AccountId`, `AccountSettings`, `BotId`, `CallingName`, `MemberId`, `PhoneNumberId`, `RoomId`, `UserId`, `UserSettings`

### List

- Operations: `ListAccounts`, `ListBots`, `ListPhoneNumberOrders`, `ListPhoneNumbers`, `ListRoomMemberships`, `ListRooms`, `ListSupportedPhoneNumberCountries`, `ListUsers`
- Traits: `paginated` (7)
- Common required input members in this group: `AccountId`, `ProductType`, `RoomId`

### Create

- Operations: `CreateAccount`, `CreateBot`, `CreateMeetingDialOut`, `CreatePhoneNumberOrder`, `CreateRoom`, `CreateRoomMembership`, `CreateUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AccountId`, `DisplayName`, `E164PhoneNumbers`, `FromPhoneNumber`, `JoinToken`, `MeetingId`, `MemberId`, `Name`, `ProductType`, `RoomId`, `ToPhoneNumber`

### Batch

- Operations: `BatchCreateRoomMembership`, `BatchDeletePhoneNumber`, `BatchSuspendUser`, `BatchUnsuspendUser`, `BatchUpdatePhoneNumber`, `BatchUpdateUser`
- Common required input members in this group: `AccountId`, `MembershipItemList`, `PhoneNumberIds`, `RoomId`, `UpdatePhoneNumberRequestItems`, `UpdateUserRequestItems`, `UserIdList`

### Delete

- Operations: `DeleteAccount`, `DeleteEventsConfiguration`, `DeletePhoneNumber`, `DeleteRoom`, `DeleteRoomMembership`
- Common required input members in this group: `AccountId`, `BotId`, `MemberId`, `PhoneNumberId`, `RoomId`

### Associate

- Operations: `AssociatePhoneNumberWithUser`, `AssociateSigninDelegateGroupsWithAccount`
- Common required input members in this group: `AccountId`, `E164PhoneNumber`, `SigninDelegateGroups`, `UserId`

### Disassociate

- Operations: `DisassociatePhoneNumberFromUser`, `DisassociateSigninDelegateGroupsFromAccount`
- Common required input members in this group: `AccountId`, `GroupNames`, `UserId`

### Put

- Operations: `PutEventsConfiguration`, `PutRetentionSettings`
- Common required input members in this group: `AccountId`, `BotId`, `RetentionSettings`

### Redact

- Operations: `RedactConversationMessage`, `RedactRoomMessage`
- Common required input members in this group: `AccountId`, `ConversationId`, `MessageId`, `RoomId`

### Invite

- Operations: `InviteUsers`
- Common required input members in this group: `AccountId`, `UserEmailList`

### Logout

- Operations: `LogoutUser`
- Common required input members in this group: `AccountId`, `UserId`

### Regenerate

- Operations: `RegenerateSecurityToken`
- Common required input members in this group: `AccountId`, `BotId`

### Reset

- Operations: `ResetPersonalPIN`
- Common required input members in this group: `AccountId`, `UserId`

### Restore

- Operations: `RestorePhoneNumber`
- Common required input members in this group: `PhoneNumberId`

### Search

- Operations: `SearchAvailablePhoneNumbers`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociatePhoneNumberWithUser` | `POST /accounts/{AccountId}/users/{UserId}?operation=associate-phone-number` | - | `AccountId`, `E164PhoneNumber`, `UserId` | - | `AssociatePhoneNumberWithUserResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Associates a phone number with the specified Amazon Chime user. |
| `AssociateSigninDelegateGroupsWithAccount` | `POST /accounts/{AccountId}?operation=associate-signin-delegate-groups` | - | `AccountId`, `SigninDelegateGroups` | - | `AssociateSigninDelegateGroupsWithAccountResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Associates the specified sign-in delegate groups with the specified Amazon Chime account. |
| `BatchCreateRoomMembership` | `POST /accounts/{AccountId}/rooms/{RoomId}/memberships?operation=batch-create` | - | `AccountId`, `MembershipItemList`, `RoomId` | - | `BatchCreateRoomMembershipResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Adds up to 50 members to a chat room in an Amazon Chime Enterprise account. Members can be users or bots. |
| `BatchDeletePhoneNumber` | `POST /phone-numbers?operation=batch-delete` | - | `PhoneNumberIds` | - | `BatchDeletePhoneNumberResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Moves phone numbers into the Deletion queue . Phone numbers must be disassociated from any users or Amazon Chime Voice Connectors before they can be deleted. |
| `BatchSuspendUser` | `POST /accounts/{AccountId}/users?operation=suspend` | - | `AccountId`, `UserIdList` | - | `BatchSuspendUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Suspends up to 50 users from a `Team` or `EnterpriseLWA` Amazon Chime account. For more information about different account types, see Managing Your Amazon Chime Accounts in the Amazon Chime Administration Guide . |
| `BatchUnsuspendUser` | `POST /accounts/{AccountId}/users?operation=unsuspend` | - | `AccountId`, `UserIdList` | - | `BatchUnsuspendUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes the suspension from up to 50 previously suspended users for the specified Amazon Chime `EnterpriseLWA` account. Only users on `EnterpriseLWA` accounts can be unsuspended using this action. |
| `BatchUpdatePhoneNumber` | `POST /phone-numbers?operation=batch-update` | - | `UpdatePhoneNumberRequestItems` | - | `BatchUpdatePhoneNumberResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates phone number product types or calling names. You can update one attribute at a time for each `UpdatePhoneNumberRequestItem`. |
| `BatchUpdateUser` | `POST /accounts/{AccountId}/users` | - | `AccountId`, `UpdateUserRequestItems` | - | `BatchUpdateUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates user details within the UpdateUserRequestItem object for up to 20 users for the specified Amazon Chime account. Currently, only `LicenseType` updates are supported for this action. |
| `CreateAccount` | `POST /accounts` | - | `Name` | - | `CreateAccountResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates an Amazon Chime account under the administrator's AWS account. Only `Team` account types are currently supported for this action. |
| `CreateBot` | `POST /accounts/{AccountId}/bots` | - | `AccountId`, `DisplayName` | - | `CreateBotResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a bot for an Amazon Chime Enterprise account. |
| `CreateMeetingDialOut` | `POST /meetings/{MeetingId}/dial-outs` | - | `FromPhoneNumber`, `JoinToken`, `MeetingId`, `ToPhoneNumber` | - | `CreateMeetingDialOutResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Uses the join token and call metadata in a meeting request (From number, To number, and so forth) to initiate an outbound call to a public switched telephone network (PSTN) and join them into a Chime meeting. Also ensures that the From number belongs to the... |
| `CreatePhoneNumberOrder` | `POST /phone-number-orders` | - | `E164PhoneNumbers`, `ProductType` | - | `CreatePhoneNumberOrderResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates an order for phone numbers to be provisioned. For toll-free numbers, you cannot use the Amazon Chime Business Calling product type. |
| `CreateRoom` | `POST /accounts/{AccountId}/rooms` | `idempotency-token` | `AccountId`, `Name` | `ClientRequestToken` | `CreateRoomResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a chat room for the specified Amazon Chime Enterprise account. |
| `CreateRoomMembership` | `POST /accounts/{AccountId}/rooms/{RoomId}/memberships` | - | `AccountId`, `MemberId`, `RoomId` | - | `CreateRoomMembershipResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, ... (+1) | Adds a member to a chat room in an Amazon Chime Enterprise account. A member can be either a user or a bot. |
| `CreateUser` | `POST /accounts/{AccountId}/users?operation=create` | - | `AccountId` | - | `CreateUserResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a user under the specified Amazon Chime account. |
| `DeleteAccount` | `DELETE /accounts/{AccountId}` | - | `AccountId` | - | `DeleteAccountResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException`, `UnprocessableEntityException` | Deletes the specified Amazon Chime account. You must suspend all users before deleting `Team` account. |
| `DeleteEventsConfiguration` | `DELETE /accounts/{AccountId}/bots/{BotId}/events-configuration` | - | `AccountId`, `BotId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `UnauthorizedClientException` | Deletes the events configuration that allows a bot to receive outgoing events. |
| `DeletePhoneNumber` | `DELETE /phone-numbers/{PhoneNumberId}` | - | `PhoneNumberId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Moves the specified phone number into the Deletion queue . A phone number must be disassociated from any users or Amazon Chime Voice Connectors before it can be deleted. |
| `DeleteRoom` | `DELETE /accounts/{AccountId}/rooms/{RoomId}` | - | `AccountId`, `RoomId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes a chat room in an Amazon Chime Enterprise account. |
| `DeleteRoomMembership` | `DELETE /accounts/{AccountId}/rooms/{RoomId}/memberships/{MemberId}` | - | `AccountId`, `MemberId`, `RoomId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes a member from a chat room in an Amazon Chime Enterprise account. |
| `DisassociatePhoneNumberFromUser` | `POST /accounts/{AccountId}/users/{UserId}?operation=disassociate-phone-number` | - | `AccountId`, `UserId` | - | `DisassociatePhoneNumberFromUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Disassociates the primary provisioned phone number from the specified Amazon Chime user. |
| `DisassociateSigninDelegateGroupsFromAccount` | `POST /accounts/{AccountId}?operation=disassociate-signin-delegate-groups` | - | `AccountId`, `GroupNames` | - | `DisassociateSigninDelegateGroupsFromAccountResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Disassociates the specified sign-in delegate groups from the specified Amazon Chime account. |
| `GetAccount` | `GET /accounts/{AccountId}` | - | `AccountId` | - | `GetAccountResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves details for the specified Amazon Chime account, such as account type and supported licenses. |
| `GetAccountSettings` | `GET /accounts/{AccountId}/settings` | - | `AccountId` | - | `GetAccountSettingsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves account settings for the specified Amazon Chime account ID, such as remote control and dialout settings. For more information about these settings, see Use the Policies Page in the Amazon Chime Administration Guide . |
| `GetBot` | `GET /accounts/{AccountId}/bots/{BotId}` | - | `AccountId`, `BotId` | - | `GetBotResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves details for the specified bot, such as bot email address, bot type, status, and display name. |
| `GetEventsConfiguration` | `GET /accounts/{AccountId}/bots/{BotId}/events-configuration` | - | `AccountId`, `BotId` | - | `GetEventsConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `UnauthorizedClientException` | Gets details for an events configuration that allows a bot to receive outgoing events, such as an HTTPS endpoint or Lambda function ARN. |
| `GetGlobalSettings` | `GET /settings` | - | - | - | `GetGlobalSettingsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings. |
| `GetPhoneNumber` | `GET /phone-numbers/{PhoneNumberId}` | - | `PhoneNumberId` | - | `GetPhoneNumberResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves details for the specified phone number ID, such as associations, capabilities, and product type. |
| `GetPhoneNumberOrder` | `GET /phone-number-orders/{PhoneNumberOrderId}` | - | `PhoneNumberOrderId` | - | `GetPhoneNumberOrderResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves details for the specified phone number order, such as the order creation timestamp, phone numbers in E.164 format, product type, and order status. |
| `GetPhoneNumberSettings` | `GET /settings/phone-number` | - | - | - | `GetPhoneNumberSettingsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves the phone number settings for the administrator's AWS account, such as the default outbound calling name. |
| `GetRetentionSettings` | `GET /accounts/{AccountId}/retention-settings` | - | `AccountId` | - | `GetRetentionSettingsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets the retention settings for the specified Amazon Chime Enterprise account. For more information about retention settings, see Managing Chat Retention Policies in the Amazon Chime Administration Guide . |
| `GetRoom` | `GET /accounts/{AccountId}/rooms/{RoomId}` | - | `AccountId`, `RoomId` | - | `GetRoomResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves room details, such as the room name, for a room in an Amazon Chime Enterprise account. |
| `GetUser` | `GET /accounts/{AccountId}/users/{UserId}` | - | `AccountId`, `UserId` | - | `GetUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves details for the specified user ID, such as primary email address, license type,and personal meeting PIN. To retrieve user details with an email address instead of a user ID, use the ListUsers action, and then filter by email address. |
| `GetUserSettings` | `GET /accounts/{AccountId}/users/{UserId}/settings` | - | `AccountId`, `UserId` | - | `GetUserSettingsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Retrieves settings for the specified user ID, such as any associated phone number settings. |
| `InviteUsers` | `POST /accounts/{AccountId}/users?operation=add` | - | `AccountId`, `UserEmailList` | - | `InviteUsersResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sends email to a maximum of 50 users, inviting them to the specified Amazon Chime `Team` account. Only `Team` account types are currently supported for this action. |
| `ListAccounts` | `GET /accounts` | `paginated` | - | - | `ListAccountsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the Amazon Chime accounts under the administrator's AWS account. You can filter accounts by account name prefix. |
| `ListBots` | `GET /accounts/{AccountId}/bots` | `paginated` | `AccountId` | - | `ListBotsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the bots associated with the administrator's Amazon Chime Enterprise account ID. |
| `ListPhoneNumberOrders` | `GET /phone-number-orders` | `paginated` | - | - | `ListPhoneNumberOrdersResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the phone number orders for the administrator's Amazon Chime account. |
| `ListPhoneNumbers` | `GET /phone-numbers` | `paginated` | - | - | `ListPhoneNumbersResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the phone numbers for the specified Amazon Chime account, Amazon Chime user, Amazon Chime Voice Connector, or Amazon Chime Voice Connector group. |
| `ListRoomMemberships` | `GET /accounts/{AccountId}/rooms/{RoomId}/memberships` | `paginated` | `AccountId`, `RoomId` | - | `ListRoomMembershipsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the membership details for the specified room in an Amazon Chime Enterprise account, such as the members' IDs, email addresses, and names. |
| `ListRooms` | `GET /accounts/{AccountId}/rooms` | `paginated` | `AccountId` | - | `ListRoomsResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the room details for the specified Amazon Chime Enterprise account. Optionally, filter the results by a member ID (user ID or bot ID) to see a list of rooms that the member belongs to. |
| `ListSupportedPhoneNumberCountries` | `GET /phone-number-countries` | - | `ProductType` | - | `ListSupportedPhoneNumberCountriesResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists supported phone number countries. |
| `ListUsers` | `GET /accounts/{AccountId}/users` | `paginated` | `AccountId` | - | `ListUsersResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the users that belong to the specified Amazon Chime account. You can specify an email address to list only the user that the email address belongs to. |
| `LogoutUser` | `POST /accounts/{AccountId}/users/{UserId}?operation=logout` | - | `AccountId`, `UserId` | - | `LogoutUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Logs out the specified user from all of the devices they are currently logged into. |
| `PutEventsConfiguration` | `PUT /accounts/{AccountId}/bots/{BotId}/events-configuration` | - | `AccountId`, `BotId` | - | `PutEventsConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `UnauthorizedClientException` | Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. |
| `PutRetentionSettings` | `PUT /accounts/{AccountId}/retention-settings` | - | `AccountId`, `RetentionSettings` | - | `PutRetentionSettingsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Puts retention settings for the specified Amazon Chime Enterprise account. We recommend using AWS CloudTrail to monitor usage of this API for your account. |
| `RedactConversationMessage` | `POST /accounts/{AccountId}/conversations/{ConversationId}/messages/{MessageId}?operation=redact` | - | `AccountId`, `ConversationId`, `MessageId` | - | `RedactConversationMessageResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Redacts the specified message from the specified Amazon Chime conversation. |
| `RedactRoomMessage` | `POST /accounts/{AccountId}/rooms/{RoomId}/messages/{MessageId}?operation=redact` | - | `AccountId`, `MessageId`, `RoomId` | - | `RedactRoomMessageResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Redacts the specified message from the specified Amazon Chime channel. |
| `RegenerateSecurityToken` | `POST /accounts/{AccountId}/bots/{BotId}?operation=regenerate-security-token` | - | `AccountId`, `BotId` | - | `RegenerateSecurityTokenResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Regenerates the security token for a bot. |
| `ResetPersonalPIN` | `POST /accounts/{AccountId}/users/{UserId}?operation=reset-personal-pin` | - | `AccountId`, `UserId` | - | `ResetPersonalPINResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Resets the personal meeting PIN for the specified user on an Amazon Chime account. Returns the User object with the updated personal meeting PIN. |
| `RestorePhoneNumber` | `POST /phone-numbers/{PhoneNumberId}?operation=restore` | - | `PhoneNumberId` | - | `RestorePhoneNumberResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Moves a phone number from the Deletion queue back into the phone number Inventory . |
| `SearchAvailablePhoneNumbers` | `GET /search?type=phone-numbers` | `paginated` | - | - | `SearchAvailablePhoneNumbersResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Searches for phone numbers that can be ordered. For US numbers, provide at least one of the following search filters: `AreaCode`, `City`, `State`, or `TollFreePrefix`. |
| `UpdateAccount` | `POST /accounts/{AccountId}` | - | `AccountId` | - | `UpdateAccountResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates account details for the specified Amazon Chime account. Currently, only account name and default license updates are supported for this action. |
| `UpdateAccountSettings` | `PUT /accounts/{AccountId}/settings` | - | `AccountId`, `AccountSettings` | - | `UpdateAccountSettingsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the settings for the specified Amazon Chime account. You can update settings for remote control of shared screens, or for the dial-out option. |
| `UpdateBot` | `POST /accounts/{AccountId}/bots/{BotId}` | - | `AccountId`, `BotId` | - | `UpdateBotResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account. |
| `UpdateGlobalSettings` | `PUT /settings` | - | - | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates global settings for the administrator's AWS account, such as Amazon Chime Business Calling and Amazon Chime Voice Connector settings. |
| `UpdatePhoneNumber` | `POST /phone-numbers/{PhoneNumberId}` | - | `PhoneNumberId` | - | `UpdatePhoneNumberResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates phone number details, such as product type or calling name, for the specified phone number ID. You can update one phone number detail at a time. |
| `UpdatePhoneNumberSettings` | `PUT /settings/phone-number` | - | `CallingName` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the phone number settings for the administrator's AWS account, such as the default outbound calling name. You can update the default outbound calling name once every seven days. |
| `UpdateRoom` | `POST /accounts/{AccountId}/rooms/{RoomId}` | - | `AccountId`, `RoomId` | - | `UpdateRoomResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates room details, such as the room name, for a room in an Amazon Chime Enterprise account. |
| `UpdateRoomMembership` | `POST /accounts/{AccountId}/rooms/{RoomId}/memberships/{MemberId}` | - | `AccountId`, `MemberId`, `RoomId` | - | `UpdateRoomMembershipResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates room membership details, such as the member role, for a room in an Amazon Chime Enterprise account. The member role designates whether the member is a chat room administrator or a general chat room member. |
| `UpdateUser` | `POST /accounts/{AccountId}/users/{UserId}` | - | `AccountId`, `UserId` | - | `UpdateUserResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates user details for a specified user ID. Currently, only `LicenseType` updates are supported for this action. |
| `UpdateUserSettings` | `PUT /accounts/{AccountId}/users/{UserId}/settings` | - | `AccountId`, `UserId`, `UserSettings` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the settings for the specified user, such as phone number settings. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListAccounts` | - | `Name -> name`, `UserEmail -> user-email`, `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListBots` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListPhoneNumberOrders` | - | `NextToken -> next-token`, `MaxResults -> max-results` | - | - |
| `ListPhoneNumbers` | - | `Status -> status`, `ProductType -> product-type`, `FilterName -> filter-name`, `FilterValue -> filter-value`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListRoomMemberships` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListRooms` | - | `MemberId -> member-id`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListSupportedPhoneNumberCountries` | - | `ProductType -> product-type` | - | - |
| `ListUsers` | - | `UserEmail -> user-email`, `UserType -> user-type`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `SearchAvailablePhoneNumbers` | - | `AreaCode -> area-code`, `City -> city`, `Country -> country`, `State -> state`, `TollFreePrefix -> toll-free-prefix`, `PhoneNumberType -> phone-number-type`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Code`, `Message` | The input parameters don't match the service's restrictions. |
| `ForbiddenException` | `structure` | `Code`, `Message` | The client is permanently forbidden from making the request. |
| `ServiceFailureException` | `structure` | `Code`, `Message` | The service encountered an unexpected error. |
| `ServiceUnavailableException` | `structure` | `Code`, `Message` | The service is currently unavailable. |
| `UnauthorizedClientException` | `structure` | `Code`, `Message` | The client is not currently authorized to make the request. |
| `ThrottledClientException` | `structure` | `Code`, `Message` | The client exceeded its request rate limit. |
| `NotFoundException` | `structure` | `Code`, `Message` | One or more of the resources in the request does not exist in the system. |
| `ResourceLimitExceededException` | `structure` | `Code`, `Message` | The request exceeds the resource limit. |
| `AccessDeniedException` | `structure` | `Code`, `Message` | You don't have permissions to perform the requested operation. |
| `ConflictException` | `structure` | `Code`, `Message` | The request could not be processed because of conflict in the current state of the resource. |
| `AssociatePhoneNumberWithUserRequest` | `structure` | `AccountId`, `E164PhoneNumber`, `UserId` | - |
| `AssociatePhoneNumberWithUserResponse` | `structure` | - | - |
| `AssociateSigninDelegateGroupsWithAccountRequest` | `structure` | `AccountId`, `SigninDelegateGroups` | - |
| `AssociateSigninDelegateGroupsWithAccountResponse` | `structure` | - | - |
| `BatchCreateRoomMembershipRequest` | `structure` | `AccountId`, `MembershipItemList`, `RoomId` | - |
| `BatchCreateRoomMembershipResponse` | `structure` | `Errors` | - |
| `BatchDeletePhoneNumberRequest` | `structure` | `PhoneNumberIds` | - |
| `BatchDeletePhoneNumberResponse` | `structure` | `PhoneNumberErrors` | - |
| `BatchSuspendUserRequest` | `structure` | `AccountId`, `UserIdList` | - |
| `BatchSuspendUserResponse` | `structure` | `UserErrors` | - |
| `BatchUnsuspendUserRequest` | `structure` | `AccountId`, `UserIdList` | - |
| `BatchUnsuspendUserResponse` | `structure` | `UserErrors` | - |
| `BatchUpdatePhoneNumberRequest` | `structure` | `UpdatePhoneNumberRequestItems` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

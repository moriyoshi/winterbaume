# AWS Wickr Admin API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon Web Services Wickr API Reference . The Amazon Web Services Wickr application programming interface (API) is designed for administrators to perform key tasks, such as creating and managing Amazon Web Services Wickr, networks, users, security groups, bots and more. This guide provides detailed information about the Amazon Web Services Wickr API, including operations, types, inputs and outputs, and error codes. You can use an Amazon Web Services SDK, the Amazon Web Services Command Line Interface (Amazon Web Services CLI, or the REST API to make API calls for Amazon Web Services Wickr. Using Amazon Web Services SDK The SDK clients authenticate your requests by using access keys that you provide.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Wickr Admin API by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS Wickr Admin API workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Update`, `Batch`, `Create` operation families, including `GetBot`, `GetBotsCount`, `GetDataRetentionBot`, `GetGuestUserHistoryCount`, `ListBlockedGuestUsers`, `ListBots`.

## Service Identity and Protocol

- AWS model slug: `wickr`
- AWS SDK for Rust slug: `wickr`
- Model version: `2024-02-01`
- Model file: `vendor/api-models-aws/models/wickr/service/2024-02-01/wickr-2024-02-01.json`
- SDK ID: `Wickr`
- Endpoint prefix: `admin.wickr`
- ARN namespace: `wickr`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (11), `List` (8), `Update` (7), `Batch` (6), `Create` (5), `Delete` (4), `Register` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateUser`, `BatchDeleteUser`, `BatchLookupUserUname`, `BatchReinviteUser`, `BatchResetDevicesForUser`, `BatchToggleUserSuspendStatus`, `CreateBot`, `CreateDataRetentionBot`, `CreateDataRetentionBotChallenge`, `CreateNetwork`, `CreateSecurityGroup`, `DeleteBot`, `DeleteDataRetentionBot`, `DeleteNetwork`, `DeleteSecurityGroup`, `RegisterOidcConfig`, `RegisterOidcConfigTest`, `RegisterOpentdfConfig`, `UpdateBot`, `UpdateDataRetention`, ... (+5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBot`, `GetBotsCount`, `GetDataRetentionBot`, `GetGuestUserHistoryCount`, `GetNetwork`, `GetNetworkSettings`, `GetOidcInfo`, `GetOpentdfConfig`, `GetSecurityGroup`, `GetUser`, `GetUsersCount`, `ListBlockedGuestUsers`, `ListBots`, `ListDevicesForUser`, `ListGuestUsers`, `ListNetworks`, `ListSecurityGroupUsers`, `ListSecurityGroups`, `ListUsers`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 18 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 44 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `KMS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetBot`, `GetBotsCount`, `GetDataRetentionBot`, `GetGuestUserHistoryCount`, `GetNetwork`, `GetNetworkSettings`, `GetOidcInfo`, `GetOpentdfConfig`, `GetSecurityGroup`, `GetUser`, `GetUsersCount`
- Traits: `readonly` (11)
- Common required input members in this group: `networkId`

### List

- Operations: `ListBlockedGuestUsers`, `ListBots`, `ListDevicesForUser`, `ListGuestUsers`, `ListNetworks`, `ListSecurityGroups`, `ListSecurityGroupUsers`, `ListUsers`
- Traits: `readonly` (8), `paginated` (8)
- Common required input members in this group: `networkId`

### Update

- Operations: `UpdateBot`, `UpdateDataRetention`, `UpdateGuestUser`, `UpdateNetwork`, `UpdateNetworkSettings`, `UpdateSecurityGroup`, `UpdateUser`
- Traits: `idempotent` (6), `idempotency-token` (1)
- Common required input members in this group: `networkId`

### Batch

- Operations: `BatchCreateUser`, `BatchDeleteUser`, `BatchLookupUserUname`, `BatchReinviteUser`, `BatchResetDevicesForUser`, `BatchToggleUserSuspendStatus`
- Traits: `idempotency-token` (6), `idempotent` (3)
- Common required input members in this group: `networkId`, `userIds`

### Create

- Operations: `CreateBot`, `CreateDataRetentionBot`, `CreateDataRetentionBotChallenge`, `CreateNetwork`, `CreateSecurityGroup`
- Traits: `idempotent` (2), `idempotency-token` (1)
- Common required input members in this group: `networkId`

### Delete

- Operations: `DeleteBot`, `DeleteDataRetentionBot`, `DeleteNetwork`, `DeleteSecurityGroup`
- Traits: `idempotent` (4), `idempotency-token` (1)
- Common required input members in this group: `networkId`

### Register

- Operations: `RegisterOidcConfig`, `RegisterOidcConfigTest`, `RegisterOpentdfConfig`
- Common required input members in this group: `networkId`, `issuer`, `scopes`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateUser` | `POST /networks/{networkId}/users` | `idempotency-token` | `networkId`, `users` | `clientToken` | `BatchCreateUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Creates multiple users in a specified Wickr network. This operation allows you to provision multiple user accounts simultaneously, optionally specifying security groups, and validation requirements for each user. cod ... |
| `BatchDeleteUser` | `POST /networks/{networkId}/users/batch-delete` | `idempotent`, `idempotency-token` | `networkId`, `userIds` | `clientToken` | `BatchDeleteUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Deletes multiple users from a specified Wickr network. This operation permanently removes user accounts and their associated data from the network. |
| `BatchLookupUserUname` | `POST /networks/{networkId}/users/uname-lookup` | `idempotency-token` | `networkId`, `unames` | `clientToken` | `BatchLookupUserUnameResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Looks up multiple user usernames from their unique username hashes (unames). This operation allows you to retrieve the email addresses associated with a list of username hashes. |
| `BatchReinviteUser` | `PATCH /networks/{networkId}/users/re-invite` | `idempotency-token` | `networkId`, `userIds` | `clientToken` | `BatchReinviteUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Resends invitation codes to multiple users who have pending invitations in a Wickr network. This operation is useful when users haven't accepted their initial invitations or when invitations have expired. |
| `BatchResetDevicesForUser` | `PATCH /networks/{networkId}/users/{userId}/devices` | `idempotent`, `idempotency-token` | `networkId`, `userId`, `appIds` | `clientToken` | `BatchResetDevicesForUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Resets multiple devices for a specific user in a Wickr network. This operation forces the selected devices to log out and requires users to re-authenticate, which is useful for security purposes or when devices need ... |
| `BatchToggleUserSuspendStatus` | `PATCH /networks/{networkId}/users/toggleSuspend` | `idempotent`, `idempotency-token` | `networkId`, `suspend`, `userIds` | `clientToken` | `BatchToggleUserSuspendStatusResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Suspends or unsuspends multiple users in a Wickr network. Suspended users cannot access the network until they are unsuspended. This operation is useful for temporarily restricting access without deleting user accounts. |
| `CreateBot` | `POST /networks/{networkId}/bots` | - | `networkId`, `username`, `groupId`, `challenge` | - | `CreateBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Creates a new bot in a specified Wickr network. Bots are automated accounts that can send and receive messages, enabling integration with external systems and automation of tasks. |
| `CreateDataRetentionBot` | `POST /networks/{networkId}/data-retention-bots` | - | `networkId` | - | `CreateDataRetentionBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Creates a data retention bot in a Wickr network. Data retention bots are specialized bots that handle message archiving and compliance by capturing and storing messages for regulatory or organizational requirements. |
| `CreateDataRetentionBotChallenge` | `POST /networks/{networkId}/data-retention-bots/challenge` | - | `networkId` | - | `CreateDataRetentionBotChallengeResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Creates a new challenge password for the data retention bot. This password is used for authentication when the bot connects to the network. |
| `CreateNetwork` | `POST /networks` | `idempotent` | `networkName`, `accessLevel` | - | `CreateNetworkResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Creates a new Wickr network with specified access level and configuration. This operation provisions a new communication network for your organization. |
| `CreateSecurityGroup` | `POST /networks/{networkId}/security-groups` | `idempotent`, `idempotency-token` | `networkId`, `name`, `securityGroupSettings` | `clientToken` | `CreateSecurityGroupResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Creates a new security group in a Wickr network. Security groups allow you to organize users and control their permissions, features, and security settings. |
| `DeleteBot` | `DELETE /networks/{networkId}/bots/{botId}` | `idempotent` | `networkId`, `botId` | - | `DeleteBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Deletes a bot from a specified Wickr network. This operation permanently removes the bot account and its associated data from the network. |
| `DeleteDataRetentionBot` | `DELETE /networks/{networkId}/data-retention-bots` | `idempotent` | `networkId` | - | `DeleteDataRetentionBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Deletes the data retention bot from a Wickr network. This operation permanently removes the bot and all its associated data from the database. |
| `DeleteNetwork` | `DELETE /networks/{networkId}` | `idempotent`, `idempotency-token` | `networkId` | `clientToken` | `DeleteNetworkResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Deletes a Wickr network and all its associated resources, including users, bots, security groups, and settings. This operation is permanent and cannot be undone. |
| `DeleteSecurityGroup` | `DELETE /networks/{networkId}/security-groups/{groupId}` | `idempotent` | `networkId`, `groupId` | - | `DeleteSecurityGroupResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Deletes a security group from a Wickr network. This operation cannot be performed on the default security group. |
| `GetBot` | `GET /networks/{networkId}/bots/{botId}` | `readonly` | `networkId`, `botId` | - | `GetBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves detailed information about a specific bot in a Wickr network, including its status, group membership, and authentication details. |
| `GetBotsCount` | `GET /networks/{networkId}/bots/count` | `readonly` | `networkId` | - | `GetBotsCountResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves the count of bots in a Wickr network, categorized by their status (pending, active, and total). |
| `GetDataRetentionBot` | `GET /networks/{networkId}/data-retention-bots` | `readonly` | `networkId` | - | `GetDataRetentionBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves information about the data retention bot in a Wickr network, including its status and whether the data retention service is enabled. |
| `GetGuestUserHistoryCount` | `GET /networks/{networkId}/guest-users/count` | `readonly` | `networkId` | - | `GetGuestUserHistoryCountResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves historical guest user count data for a Wickr network, showing the number of guest users per billing period over the past 90 days. |
| `GetNetwork` | `GET /networks/{networkId}` | `readonly` | `networkId` | - | `GetNetworkResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves detailed information about a specific Wickr network, including its configuration, access level, and status. |
| `GetNetworkSettings` | `GET /networks/{networkId}/settings` | `readonly` | `networkId` | - | `GetNetworkSettingsResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves all network-level settings for a Wickr network, including client metrics, data retention, and other configuration options. |
| `GetOidcInfo` | `GET /networks/{networkId}/oidc` | `readonly` | `networkId` | - | `GetOidcInfoResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves the OpenID Connect (OIDC) configuration for a Wickr network, including SSO settings and optional token information if access token parameters are provided. |
| `GetOpentdfConfig` | `GET /networks/{networkId}/tdf` | `readonly` | `networkId` | - | `GetOpentdfConfigResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves the OpenTDF integration configuration for a Wickr network. |
| `GetSecurityGroup` | `GET /networks/{networkId}/security-groups/{groupId}` | `readonly` | `networkId`, `groupId` | - | `GetSecurityGroupResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves detailed information about a specific security group in a Wickr network, including its settings, member counts, and configuration. |
| `GetUser` | `GET /networks/{networkId}/users/{userId}` | `readonly` | `networkId`, `userId` | - | `GetUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves detailed information about a specific user in a Wickr network, including their profile, status, and activity history. |
| `GetUsersCount` | `GET /networks/{networkId}/users/count` | `readonly` | `networkId` | - | `GetUsersCountResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves the count of users in a Wickr network, categorized by their status (pending, active, rejected) and showing how many users can still be added. |
| `ListBlockedGuestUsers` | `GET /networks/{networkId}/guest-users/blocklist` | `readonly`, `paginated` | `networkId` | - | `ListBlockedGuestUsersResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of guest users who have been blocked from a Wickr network. You can filter and sort the results. |
| `ListBots` | `GET /networks/{networkId}/bots` | `readonly`, `paginated` | `networkId` | - | `ListBotsResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of bots in a specified Wickr network. You can filter and sort the results based on various criteria. |
| `ListDevicesForUser` | `GET /networks/{networkId}/users/{userId}/devices` | `readonly`, `paginated` | `networkId`, `userId` | - | `ListDevicesForUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of devices associated with a specific user in a Wickr network. This operation returns information about all devices where the user has logged into Wickr. |
| `ListGuestUsers` | `GET /networks/{networkId}/guest-users` | `readonly`, `paginated` | `networkId` | - | `ListGuestUsersResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of guest users who have communicated with your Wickr network. Guest users are external users from federated networks who can communicate with network members. |
| `ListNetworks` | `GET /networks` | `readonly`, `paginated` | - | - | `ListNetworksResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of all Wickr networks associated with your Amazon Web Services account. You can sort the results by network ID or name. |
| `ListSecurityGroups` | `GET /networks/{networkId}/security-groups` | `readonly`, `paginated` | `networkId` | - | `ListSecurityGroupsResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of security groups in a specified Wickr network. You can sort the results by various criteria. |
| `ListSecurityGroupUsers` | `GET /networks/{networkId}/security-groups/{groupId}/users` | `readonly`, `paginated` | `networkId`, `groupId` | - | `ListSecurityGroupUsersResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of users who belong to a specific security group in a Wickr network. |
| `ListUsers` | `GET /networks/{networkId}/users` | `readonly`, `paginated` | `networkId` | - | `ListUsersResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Retrieves a paginated list of users in a specified Wickr network. You can filter and sort the results based on various criteria such as name, status, or security group membership. |
| `RegisterOidcConfig` | `POST /networks/{networkId}/oidc/save` | - | `networkId`, `companyId`, `issuer`, `scopes` | - | `RegisterOidcConfigResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Registers and saves an OpenID Connect (OIDC) configuration for a Wickr network, enabling Single Sign-On (SSO) authentication through an identity provider. |
| `RegisterOidcConfigTest` | `POST /networks/{networkId}/oidc/test` | - | `networkId`, `issuer`, `scopes` | - | `RegisterOidcConfigTestResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Tests an OpenID Connect (OIDC) configuration for a Wickr network by validating the connection to the identity provider and retrieving its supported capabilities. |
| `RegisterOpentdfConfig` | `POST /networks/{networkId}/tdf` | - | `networkId`, `clientId`, `clientSecret`, `domain`, `provider` | - | `RegisterOpentdfConfigResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Registers and saves OpenTDF configuration for a Wickr network, enabling attribute-based access control for Wickr through an OpenTDF provider. |
| `UpdateBot` | `PATCH /networks/{networkId}/bots/{botId}` | `idempotent` | `networkId`, `botId` | - | `UpdateBotResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates the properties of an existing bot in a Wickr network. This operation allows you to modify the bot's display name, security group, password, or suspension status. |
| `UpdateDataRetention` | `PATCH /networks/{networkId}/data-retention-bots` | `idempotent` | `networkId`, `actionType` | - | `UpdateDataRetentionResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates the data retention bot settings, allowing you to enable or disable the data retention service, or acknowledge the public key message. |
| `UpdateGuestUser` | `PATCH /networks/{networkId}/guest-users/{usernameHash}` | - | `networkId`, `usernameHash`, `block` | - | `UpdateGuestUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates the block status of a guest user in a Wickr network. This operation allows you to block or unblock a guest user from accessing the network. |
| `UpdateNetwork` | `PATCH /networks/{networkId}` | `idempotent`, `idempotency-token` | `networkId`, `networkName` | `clientToken` | `UpdateNetworkResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates the properties of an existing Wickr network, such as its name or encryption key configuration. |
| `UpdateNetworkSettings` | `PATCH /networks/{networkId}/settings` | `idempotent` | `networkId`, `settings` | - | `UpdateNetworkSettingsResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates network-level settings for a Wickr network. You can modify settings such as client metrics, data retention, and other network-wide options. |
| `UpdateSecurityGroup` | `PATCH /networks/{networkId}/security-groups/{groupId}` | `idempotent` | `networkId`, `groupId` | - | `UpdateSecurityGroupResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates the properties of an existing security group in a Wickr network, such as its name or settings. |
| `UpdateUser` | `PATCH /networks/{networkId}/users` | `idempotent` | `networkId`, `userId` | - | `UpdateUserResponse` | `BadRequestError`, `ForbiddenError`, `InternalServerError`, `RateLimitError`, `ResourceNotFoundError`, `UnauthorizedError`, `ValidationError` | Updates the properties of an existing user in a Wickr network. This operation allows you to modify the user's name, password, security group membership, and invite code settings. codeValidation , inviteCode , and inv ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `BatchCreateUser` | `clientToken -> X-Client-Token` | - | - | - |
| `BatchDeleteUser` | `clientToken -> X-Client-Token` | - | - | - |
| `BatchLookupUserUname` | `clientToken -> X-Client-Token` | - | - | - |
| `BatchReinviteUser` | `clientToken -> X-Client-Token` | - | - | - |
| `BatchResetDevicesForUser` | `clientToken -> X-Client-Token` | - | - | - |
| `BatchToggleUserSuspendStatus` | `clientToken -> X-Client-Token` | `suspend -> suspend` | - | - |
| `CreateSecurityGroup` | `clientToken -> X-Client-Token` | - | - | - |
| `DeleteNetwork` | `clientToken -> X-Client-Token` | - | - | - |
| `GetOidcInfo` | - | `clientId -> clientId`, `code -> code`, `grantType -> grantType`, `redirectUri -> redirectUri`, `url -> url`, `clientSecret -> clientSecret`, `codeVerifier -> codeVerifier`, `certificate -> certificate` | - | - |
| `GetUser` | - | `startTime -> startTime`, `endTime -> endTime` | - | - |
| `ListBlockedGuestUsers` | - | `maxResults -> maxResults`, `sortDirection -> sortDirection`, `sortFields -> sortFields`, `username -> username`, `admin -> admin`, `nextToken -> nextToken` | - | - |
| `ListBots` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `sortFields -> sortFields`, `sortDirection -> sortDirection`, `displayName -> displayName`, `username -> username`, `status -> status`, `groupId -> groupId` | - | - |
| `ListDevicesForUser` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `sortFields -> sortFields`, `sortDirection -> sortDirection` | - | - |
| `ListGuestUsers` | - | `maxResults -> maxResults`, `sortDirection -> sortDirection`, `sortFields -> sortFields`, `username -> username`, `billingPeriod -> billingPeriod`, `nextToken -> nextToken` | - | - |
| `ListNetworks` | - | `maxResults -> maxResults`, `sortFields -> sortFields`, `sortDirection -> sortDirection`, `nextToken -> nextToken` | - | - |
| `ListSecurityGroups` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `sortFields -> sortFields`, `sortDirection -> sortDirection` | - | - |
| `ListSecurityGroupUsers` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `sortFields -> sortFields`, `sortDirection -> sortDirection` | - | - |
| `ListUsers` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `sortFields -> sortFields`, `sortDirection -> sortDirection`, `firstName -> firstName`, `lastName -> lastName`, `username -> username`, `status -> status`, `groupId -> groupId` | - | - |
| `RegisterOpentdfConfig` | - | `dryRun -> dryRun` | - | - |
| `UpdateNetwork` | `clientToken -> X-Client-Token` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestError` | `structure` | message | The request was invalid or malformed. This error occurs when the request parameters do not meet the API requirements, such as invalid field values, missing ... |
| `ForbiddenError` | `structure` | message | Access to the requested resource is forbidden. This error occurs when the authenticated user does not have the necessary permissions to perform the requeste ... |
| `InternalServerError` | `structure` | message | An unexpected error occurred on the server while processing the request. This indicates a problem with the Wickr service itself rather than with the request ... |
| `RateLimitError` | `structure` | message | The request was throttled because too many requests were sent in a short period of time. Wait a moment and retry the request. Consider implementing exponent ... |
| `ResourceNotFoundError` | `structure` | message | The requested resource could not be found. This error occurs when you try to access or modify a network, user, bot, security group, or other resource that d ... |
| `UnauthorizedError` | `structure` | message | The request was not authenticated or the authentication credentials were invalid. This error occurs when the request lacks valid authentication credentials ... |
| `ValidationError` | `structure` | reasons, message | One or more fields in the request failed validation. This error provides detailed information about which fields were invalid and why, allowing you to corre ... |
| `BatchCreateUserRequest` | `structure` | networkId, users, clientToken | - |
| `BatchCreateUserResponse` | `structure` | message, successful, failed | - |
| `BatchDeleteUserRequest` | `structure` | networkId, userIds, clientToken | - |
| `BatchDeleteUserResponse` | `structure` | message, successful, failed | - |
| `BatchLookupUserUnameRequest` | `structure` | networkId, unames, clientToken | - |
| `BatchLookupUserUnameResponse` | `structure` | message, successful, failed | - |
| `BatchReinviteUserRequest` | `structure` | networkId, userIds, clientToken | - |
| `BatchReinviteUserResponse` | `structure` | message, successful, failed | - |
| `BatchResetDevicesForUserRequest` | `structure` | networkId, userId, appIds, clientToken | - |
| `BatchResetDevicesForUserResponse` | `structure` | message, successful, failed | - |
| `BatchToggleUserSuspendStatusRequest` | `structure` | networkId, suspend, userIds, clientToken | - |
| `BatchToggleUserSuspendStatusResponse` | `structure` | message, successful, failed | - |
| `CreateBotRequest` | `structure` | networkId, username, displayName, groupId, challenge | - |
| `CreateBotResponse` | `structure` | message, botId, networkId, username, displayName, groupId | - |
| `CreateDataRetentionBotRequest` | `structure` | networkId | - |
| `CreateDataRetentionBotResponse` | `structure` | message | - |
| `CreateDataRetentionBotChallengeRequest` | `structure` | networkId | - |
| `CreateDataRetentionBotChallengeResponse` | `structure` | challenge | - |
| `CreateNetworkRequest` | `structure` | networkName, accessLevel, enablePremiumFreeTrial, encryptionKeyArn | - |
| `CreateNetworkResponse` | `structure` | networkId, networkName, encryptionKeyArn | - |
| `CreateSecurityGroupRequest` | `structure` | networkId, name, securityGroupSettings, clientToken | - |
| `CreateSecurityGroupResponse` | `structure` | securityGroup | - |
| `DeleteBotRequest` | `structure` | networkId, botId | - |
| `DeleteBotResponse` | `structure` | message | - |
| `DeleteDataRetentionBotRequest` | `structure` | networkId | - |
| `DeleteDataRetentionBotResponse` | `structure` | message | - |
| `DeleteNetworkRequest` | `structure` | networkId, clientToken | - |
| `DeleteNetworkResponse` | `structure` | message | - |
| `DeleteSecurityGroupRequest` | `structure` | networkId, groupId | - |
| `DeleteSecurityGroupResponse` | `structure` | message, networkId, groupId | - |
| `GetBotRequest` | `structure` | networkId, botId | - |
| `GetBotResponse` | `structure` | botId, displayName, username, uname, pubkey, status, groupId, hasChallenge, suspended, lastLogin | - |
| `GetBotsCountRequest` | `structure` | networkId | - |
| `AccessLevel` | `enum` | STANDARD, PREMIUM | - |
| `BotStatus` | `intEnum` | PENDING, ACTIVE | - |
| `DataRetentionActionType` | `enum` | ENABLE, DISABLE, PUBKEY_MSG_ACK | - |
| `SortDirection` | `enum` | ASC, DESC | - |
| `Status` | `enum` | DISABLED, ENABLED, FORCE_ENABLED | - |
| `UserStatus` | `intEnum` | PENDING, ACTIVE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

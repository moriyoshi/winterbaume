# Amazon Chime SDK Identity

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Chime SDK Identity APIs in this section allow software developers to create and manage unique instances of their messaging applications. These APIs provide the overarching framework for creating and sending messages. For more information about the identity APIs, refer to Amazon Chime SDK identity.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Chime SDK Identity where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented Amazon Chime SDK Identity workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Update` operation families, including `ListAppInstanceAdmins`, `ListAppInstanceBots`, `ListAppInstanceUserEndpoints`, `ListAppInstanceUsers`, `DescribeAppInstance`, `DescribeAppInstanceAdmin`.

## Service Identity and Protocol

- AWS model slug: `chime-sdk-identity`
- AWS SDK for Rust slug: `chimesdkidentity`
- Model version: `2021-04-20`
- Model file: `vendor/api-models-aws/models/chime-sdk-identity/service/2021-04-20/chime-sdk-identity-2021-04-20.json`
- SDK ID: `Chime SDK Identity`
- Endpoint prefix: `identity-chime`
- ARN namespace: `chime`
- CloudFormation name: `ChimeSDKIdentity`
- CloudTrail event source: `chimesdkidentity.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Describe` (5), `Create` (4), `Delete` (4), `Update` (4), `Put` (2), `Deregister` (1), `Get` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAppInstance`, `CreateAppInstanceAdmin`, `CreateAppInstanceBot`, `CreateAppInstanceUser`, `DeleteAppInstance`, `DeleteAppInstanceAdmin`, `DeleteAppInstanceBot`, `DeleteAppInstanceUser`, `DeregisterAppInstanceUserEndpoint`, `PutAppInstanceRetentionSettings`, `PutAppInstanceUserExpirationSettings`, `RegisterAppInstanceUserEndpoint`, `TagResource`, `UntagResource`, `UpdateAppInstance`, `UpdateAppInstanceBot`, `UpdateAppInstanceUser`, `UpdateAppInstanceUserEndpoint`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAppInstance`, `DescribeAppInstanceAdmin`, `DescribeAppInstanceBot`, `DescribeAppInstanceUser`, `DescribeAppInstanceUserEndpoint`, `GetAppInstanceRetentionSettings`, `ListAppInstanceAdmins`, `ListAppInstanceBots`, `ListAppInstanceUserEndpoints`, `ListAppInstanceUsers`, `ListAppInstances`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 30 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cli/latest/reference/chime-sdk-identity/list-app-instance-admins.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/using-the-messaging-sdk.html
- https://docs.aws.amazon.com/chime-sdk/latest/dg/messaging-namespace-migration.html

Research outcomes:
- Chime SDK Identity manages app instances, app instance users, and app instance administrators for Chime SDK messaging applications.
- App instance users are identities inside an app instance and are distinct from legacy Amazon Chime user accounts.
- App instance administrators have administrative permissions within the app instance.
- Chime SDK messaging channels, memberships, and messages reference app instance user identities.
- Namespace migration documentation distinguishes older Amazon Chime messaging namespaces from the newer Chime SDK namespace.

Parity implications:
- Model app instances, app instance users, app instance admins, tags, and identity references separately from messaging channel resources.
- User/admin lifecycle should affect messaging authorisation without deleting message history automatically.
- Do not conflate legacy Chime accounts with Chime SDK app instance identities.

## Operation Groups

### List

- Operations: `ListAppInstanceAdmins`, `ListAppInstanceBots`, `ListAppInstanceUserEndpoints`, `ListAppInstanceUsers`, `ListAppInstances`, `ListTagsForResource`
- Traits: `paginated` (5)
- Common required input members in this group: `AppInstanceArn`, `AppInstanceUserArn`, `ResourceARN`

### Describe

- Operations: `DescribeAppInstance`, `DescribeAppInstanceAdmin`, `DescribeAppInstanceBot`, `DescribeAppInstanceUser`, `DescribeAppInstanceUserEndpoint`
- Common required input members in this group: `AppInstanceAdminArn`, `AppInstanceArn`, `AppInstanceBotArn`, `AppInstanceUserArn`, `EndpointId`

### Create

- Operations: `CreateAppInstance`, `CreateAppInstanceAdmin`, `CreateAppInstanceBot`, `CreateAppInstanceUser`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `AppInstanceAdminArn`, `AppInstanceArn`, `AppInstanceUserId`, `ClientRequestToken`, `Configuration`, `Name`

### Delete

- Operations: `DeleteAppInstance`, `DeleteAppInstanceAdmin`, `DeleteAppInstanceBot`, `DeleteAppInstanceUser`
- Common required input members in this group: `AppInstanceAdminArn`, `AppInstanceArn`, `AppInstanceBotArn`, `AppInstanceUserArn`

### Update

- Operations: `UpdateAppInstance`, `UpdateAppInstanceBot`, `UpdateAppInstanceUser`, `UpdateAppInstanceUserEndpoint`
- Common required input members in this group: `AppInstanceArn`, `AppInstanceBotArn`, `AppInstanceUserArn`, `EndpointId`, `Metadata`, `Name`

### Put

- Operations: `PutAppInstanceRetentionSettings`, `PutAppInstanceUserExpirationSettings`
- Common required input members in this group: `AppInstanceArn`, `AppInstanceRetentionSettings`, `AppInstanceUserArn`

### Deregister

- Operations: `DeregisterAppInstanceUserEndpoint`
- Common required input members in this group: `AppInstanceUserArn`, `EndpointId`

### Get

- Operations: `GetAppInstanceRetentionSettings`
- Common required input members in this group: `AppInstanceArn`

### Register

- Operations: `RegisterAppInstanceUserEndpoint`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AppInstanceUserArn`, `ClientRequestToken`, `EndpointAttributes`, `ResourceArn`, `Type`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAppInstance` | `POST /app-instances` | `idempotency-token` | `ClientRequestToken`, `Name` | `ClientRequestToken` | `CreateAppInstanceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates an Amazon Chime SDK messaging `AppInstance` under an AWS account. Only SDK messaging customers use this API. |
| `CreateAppInstanceAdmin` | `POST /app-instances/{AppInstanceArn}/admins` | - | `AppInstanceAdminArn`, `AppInstanceArn` | - | `CreateAppInstanceAdminResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Promotes an `AppInstanceUser` or `AppInstanceBot` to an `AppInstanceAdmin`. The promoted entity can perform the following actions. |
| `CreateAppInstanceBot` | `POST /app-instance-bots` | `idempotency-token` | `AppInstanceArn`, `ClientRequestToken`, `Configuration` | `ClientRequestToken` | `CreateAppInstanceBotResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a bot under an Amazon Chime `AppInstance`. The request consists of a unique `Configuration` and `Name` for that bot. |
| `CreateAppInstanceUser` | `POST /app-instance-users` | `idempotency-token` | `AppInstanceArn`, `AppInstanceUserId`, `ClientRequestToken`, `Name` | `ClientRequestToken` | `CreateAppInstanceUserResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Creates a user under an Amazon Chime `AppInstance`. The request consists of a unique `appInstanceUserId` and `Name` for that user. |
| `DeleteAppInstance` | `DELETE /app-instances/{AppInstanceArn}` | - | `AppInstanceArn` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes an `AppInstance` and all associated data asynchronously. |
| `DeleteAppInstanceAdmin` | `DELETE /app-instances/{AppInstanceArn}/admins/{AppInstanceAdminArn}` | - | `AppInstanceAdminArn`, `AppInstanceArn` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Demotes an `AppInstanceAdmin` to an `AppInstanceUser` or `AppInstanceBot`. This action does not delete the user. |
| `DeleteAppInstanceBot` | `DELETE /app-instance-bots/{AppInstanceBotArn}` | - | `AppInstanceBotArn` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes an `AppInstanceBot`. |
| `DeleteAppInstanceUser` | `DELETE /app-instance-users/{AppInstanceUserArn}` | - | `AppInstanceUserArn` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deletes an `AppInstanceUser`. |
| `DeregisterAppInstanceUserEndpoint` | `DELETE /app-instance-users/{AppInstanceUserArn}/endpoints/{EndpointId}` | - | `AppInstanceUserArn`, `EndpointId` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Deregisters an `AppInstanceUserEndpoint`. |
| `DescribeAppInstance` | `GET /app-instances/{AppInstanceArn}` | - | `AppInstanceArn` | - | `DescribeAppInstanceResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of an `AppInstance`. |
| `DescribeAppInstanceAdmin` | `GET /app-instances/{AppInstanceArn}/admins/{AppInstanceAdminArn}` | - | `AppInstanceAdminArn`, `AppInstanceArn` | - | `DescribeAppInstanceAdminResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of an `AppInstanceAdmin`. |
| `DescribeAppInstanceBot` | `GET /app-instance-bots/{AppInstanceBotArn}` | - | `AppInstanceBotArn` | - | `DescribeAppInstanceBotResponse` | `BadRequestException`, `ForbiddenException`, `NotFoundException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | The `AppInstanceBot's` information. |
| `DescribeAppInstanceUser` | `GET /app-instance-users/{AppInstanceUserArn}` | - | `AppInstanceUserArn` | - | `DescribeAppInstanceUserResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of an `AppInstanceUser`. |
| `DescribeAppInstanceUserEndpoint` | `GET /app-instance-users/{AppInstanceUserArn}/endpoints/{EndpointId}` | - | `AppInstanceUserArn`, `EndpointId` | - | `DescribeAppInstanceUserEndpointResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns the full details of an `AppInstanceUserEndpoint`. |
| `GetAppInstanceRetentionSettings` | `GET /app-instances/{AppInstanceArn}/retention-settings` | - | `AppInstanceArn` | - | `GetAppInstanceRetentionSettingsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Gets the retention settings for an `AppInstance`. |
| `ListAppInstanceAdmins` | `GET /app-instances/{AppInstanceArn}/admins` | `paginated` | `AppInstanceArn` | - | `ListAppInstanceAdminsResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Returns a list of the administrators in the `AppInstance`. |
| `ListAppInstanceBots` | `GET /app-instance-bots` | `paginated` | `AppInstanceArn` | - | `ListAppInstanceBotsResponse` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all `AppInstanceBots` created under a single `AppInstance`. |
| `ListAppInstanceUserEndpoints` | `GET /app-instance-users/{AppInstanceUserArn}/endpoints` | `paginated` | `AppInstanceUserArn` | - | `ListAppInstanceUserEndpointsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all the `AppInstanceUserEndpoints` created under a single `AppInstanceUser`. |
| `ListAppInstanceUsers` | `GET /app-instance-users` | `paginated` | `AppInstanceArn` | - | `ListAppInstanceUsersResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | List all `AppInstanceUsers` created under a single `AppInstance`. |
| `ListAppInstances` | `GET /app-instances` | `paginated` | - | - | `ListAppInstancesResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists all Amazon Chime `AppInstance`s created under a single AWS account. |
| `ListTagsForResource` | `GET /tags` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Lists the tags applied to an Amazon Chime SDK identity resource. |
| `PutAppInstanceRetentionSettings` | `PUT /app-instances/{AppInstanceArn}/retention-settings` | - | `AppInstanceArn`, `AppInstanceRetentionSettings` | - | `PutAppInstanceRetentionSettingsResponse` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sets the amount of time in days that a given `AppInstance` retains data. |
| `PutAppInstanceUserExpirationSettings` | `PUT /app-instance-users/{AppInstanceUserArn}/expiration-settings` | - | `AppInstanceUserArn` | - | `PutAppInstanceUserExpirationSettingsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Sets the number of days before the `AppInstanceUser` is automatically deleted. A background process deletes expired `AppInstanceUsers` within 6 hours of expiration. |
| `RegisterAppInstanceUserEndpoint` | `POST /app-instance-users/{AppInstanceUserArn}/endpoints` | `idempotency-token` | `AppInstanceUserArn`, `ClientRequestToken`, `EndpointAttributes`, `ResourceArn`, `Type` | `ClientRequestToken` | `RegisterAppInstanceUserEndpointResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Registers an endpoint under an Amazon Chime `AppInstanceUser`. The endpoint receives messages for a user. |
| `TagResource` | `POST /tags?operation=tag-resource` | - | `ResourceARN`, `Tags` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Applies the specified tags to the specified Amazon Chime SDK identity resource. |
| `UntagResource` | `POST /tags?operation=untag-resource` | - | `ResourceARN`, `TagKeys` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Removes the specified tags from the specified Amazon Chime SDK identity resource. |
| `UpdateAppInstance` | `PUT /app-instances/{AppInstanceArn}` | - | `AppInstanceArn`, `Metadata`, `Name` | - | `UpdateAppInstanceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates `AppInstance` metadata. |
| `UpdateAppInstanceBot` | `PUT /app-instance-bots/{AppInstanceBotArn}` | - | `AppInstanceBotArn`, `Metadata`, `Name` | - | `UpdateAppInstanceBotResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the name and metadata of an `AppInstanceBot`. |
| `UpdateAppInstanceUser` | `PUT /app-instance-users/{AppInstanceUserArn}` | - | `AppInstanceUserArn`, `Metadata`, `Name` | - | `UpdateAppInstanceUserResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ResourceLimitExceededException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the details of an `AppInstanceUser`. You can update names and metadata. |
| `UpdateAppInstanceUserEndpoint` | `PUT /app-instance-users/{AppInstanceUserArn}/endpoints/{EndpointId}` | - | `AppInstanceUserArn`, `EndpointId` | - | `UpdateAppInstanceUserEndpointResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `ServiceFailureException`, `ServiceUnavailableException`, `ThrottledClientException`, `UnauthorizedClientException` | Updates the details of an `AppInstanceUserEndpoint`. You can update the name and `AllowMessage` values. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListAppInstanceAdmins` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListAppInstanceBots` | - | `AppInstanceArn -> app-instance-arn`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListAppInstances` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListAppInstanceUserEndpoints` | - | `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListAppInstanceUsers` | - | `AppInstanceArn -> app-instance-arn`, `MaxResults -> max-results`, `NextToken -> next-token` | - | - |
| `ListTagsForResource` | - | `ResourceARN -> arn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Code`, `Message` | The input parameters don't match the service's restrictions. |
| `ForbiddenException` | `structure` | `Code`, `Message` | The client is permanently forbidden from making the request. |
| `ServiceFailureException` | `structure` | `Code`, `Message` | The service encountered an unexpected error. |
| `ServiceUnavailableException` | `structure` | `Code`, `Message` | The service is currently unavailable. |
| `ThrottledClientException` | `structure` | `Code`, `Message` | The client exceeded its request rate limit. |
| `UnauthorizedClientException` | `structure` | `Code`, `Message` | The client is not currently authorized to make the request. |
| `ResourceLimitExceededException` | `structure` | `Code`, `Message` | The request exceeds the resource limit. |
| `ConflictException` | `structure` | `Code`, `Message` | The request could not be processed because of conflict in the current state of the resource. |
| `CreateAppInstanceRequest` | `structure` | `ClientRequestToken`, `Metadata`, `Name`, `Tags` | - |
| `CreateAppInstanceResponse` | `structure` | `AppInstanceArn` | - |
| `CreateAppInstanceAdminRequest` | `structure` | `AppInstanceAdminArn`, `AppInstanceArn` | - |
| `CreateAppInstanceAdminResponse` | `structure` | `AppInstanceAdmin`, `AppInstanceArn` | - |
| `CreateAppInstanceBotRequest` | `structure` | `AppInstanceArn`, `ClientRequestToken`, `Configuration`, `Metadata`, `Name`, `Tags` | - |
| `CreateAppInstanceBotResponse` | `structure` | `AppInstanceBotArn` | - |
| `CreateAppInstanceUserRequest` | `structure` | `AppInstanceArn`, `AppInstanceUserId`, `ClientRequestToken`, `ExpirationSettings`, `Metadata`, `Name`, `Tags` | - |
| `CreateAppInstanceUserResponse` | `structure` | `AppInstanceUserArn` | - |
| `DeleteAppInstanceRequest` | `structure` | `AppInstanceArn` | - |
| `DeleteAppInstanceAdminRequest` | `structure` | `AppInstanceAdminArn`, `AppInstanceArn` | - |
| `DeleteAppInstanceBotRequest` | `structure` | `AppInstanceBotArn` | - |
| `DeleteAppInstanceUserRequest` | `structure` | `AppInstanceUserArn` | - |
| `DeregisterAppInstanceUserEndpointRequest` | `structure` | `AppInstanceUserArn`, `EndpointId` | - |
| `DescribeAppInstanceRequest` | `structure` | `AppInstanceArn` | - |
| `DescribeAppInstanceResponse` | `structure` | `AppInstance` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

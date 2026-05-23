# Amazon Security Lake

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Security Lake is a fully managed security data lake service. You can use Security Lake to automatically centralize security data from cloud, on-premises, and custom sources into a data lake that's stored in your Amazon Web Services account. Amazon Web Services Organizations is an account management service that lets you consolidate multiple Amazon Web Services accounts into an organization that you create and centrally manage. With Organizations, you can create member accounts and invite existing accounts to join your organization. Security Lake helps you analyze security data for a more complete understanding of your security posture across the entire organization.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Security Lake where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented Amazon Security Lake workflows in the local mock. Key resources include `DataLake`, `Subscriber`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Delete`, `List`, `Get`, `Update` operation families, including `CreateAwsLogSource`, `CreateCustomLogSource`, `CreateDataLake`, `CreateDataLakeExceptionSubscription`, `DeleteAwsLogSource`, `DeleteCustomLogSource`.

## Service Identity and Protocol

- AWS model slug: `securitylake`
- AWS SDK for Rust slug: `securitylake`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/securitylake/service/2018-05-10/securitylake-2018-05-10.json`
- SDK ID: `SecurityLake`
- Endpoint prefix: `-`
- ARN namespace: `securitylake`
- CloudFormation name: `-`
- CloudTrail event source: `securitylake.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (7), `Delete` (7), `List` (5), `Get` (4), `Update` (4), `Deregister` (1), `Register` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAwsLogSource`, `CreateCustomLogSource`, `CreateDataLake`, `CreateDataLakeExceptionSubscription`, `CreateDataLakeOrganizationConfiguration`, `CreateSubscriber`, `CreateSubscriberNotification`, `DeleteAwsLogSource`, `DeleteCustomLogSource`, `DeleteDataLake`, `DeleteDataLakeExceptionSubscription`, `DeleteDataLakeOrganizationConfiguration`, `DeleteSubscriber`, `DeleteSubscriberNotification`, `DeregisterDataLakeDelegatedAdministrator`, `RegisterDataLakeDelegatedAdministrator`, `TagResource`, `UntagResource`, `UpdateDataLake`, `UpdateDataLakeExceptionSubscription`, ... (+2).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDataLakeExceptionSubscription`, `GetDataLakeOrganizationConfiguration`, `GetDataLakeSources`, `GetSubscriber`, `ListDataLakeExceptions`, `ListDataLakes`, `ListLogSources`, `ListSubscribers`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`, `SNS`, `SQS`, `Glue`, `EC2/VPC`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DataLake` | - | - | `CreateAwsLogSource`, `CreateCustomLogSource`, `CreateDataLake`, `CreateDataLakeOrganizationConfiguration`, `DeleteAwsLogSource`, `DeleteCustomLogSource`, `DeleteDataLake`, `DeleteDataLakeOrganizationConfiguration`, `GetDataLakeOrganizationConfiguration`, `GetDataLakeSources`, ... (+3) | Represents a data lake resource that can collect logs from multiple aws or custom sources |
| `Subscriber` | `subscriberId` | create: `CreateSubscriber`; read: `GetSubscriber`; update: `UpdateSubscriber`; delete: `DeleteSubscriber`; list: `ListSubscribers` | `CreateSubscriberNotification`, `DeleteSubscriberNotification`, `UpdateSubscriberNotification` | Represents a subscriber resource that allows access to sources |
## Operation Groups

### Create

- Operations: `CreateAwsLogSource`, `CreateCustomLogSource`, `CreateDataLake`, `CreateDataLakeExceptionSubscription`, `CreateDataLakeOrganizationConfiguration`, `CreateSubscriber`, `CreateSubscriberNotification`
- Traits: `idempotent` (1)
- Common required input members in this group: `configuration`, `configurations`, `metaStoreManagerRoleArn`, `notificationEndpoint`, `sourceName`, `sources`, `subscriberId`, `subscriberIdentity`, `subscriberName`, `subscriptionProtocol`

### Delete

- Operations: `DeleteAwsLogSource`, `DeleteCustomLogSource`, `DeleteDataLake`, `DeleteDataLakeExceptionSubscription`, `DeleteDataLakeOrganizationConfiguration`, `DeleteSubscriber`, `DeleteSubscriberNotification`
- Traits: `idempotent` (5)
- Common required input members in this group: `regions`, `sourceName`, `sources`, `subscriberId`

### List

- Operations: `ListDataLakeExceptions`, `ListDataLakes`, `ListLogSources`, `ListSubscribers`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (5)
- Common required input members in this group: `resourceArn`

### Get

- Operations: `GetDataLakeExceptionSubscription`, `GetDataLakeOrganizationConfiguration`, `GetDataLakeSources`, `GetSubscriber`
- Traits: `paginated` (1), `readonly` (3)
- Common required input members in this group: `subscriberId`

### Update

- Operations: `UpdateDataLake`, `UpdateDataLakeExceptionSubscription`, `UpdateSubscriber`, `UpdateSubscriberNotification`
- Traits: `idempotent` (4)
- Common required input members in this group: `configuration`, `configurations`, `notificationEndpoint`, `subscriberId`, `subscriptionProtocol`

### Deregister

- Operations: `DeregisterDataLakeDelegatedAdministrator`
- Traits: `idempotent` (1)

### Register

- Operations: `RegisterDataLakeDelegatedAdministrator`
- Common required input members in this group: `accountId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAwsLogSource` | `POST /v1/datalake/logsources/aws` | - | `sources` | - | `CreateAwsLogSourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds a natively supported Amazon Web Services service as an Amazon Security Lake source. Enables source types for member accounts in required Amazon Web Services Regions, based on the parameters you specify. |
| `CreateCustomLogSource` | `POST /v1/datalake/logsources/custom` | `idempotent` | `configuration`, `sourceName` | - | `CreateCustomLogSourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds a third-party custom source in Amazon Security Lake, from the Amazon Web Services Region where you want to create a custom source. Security Lake can collect logs and events from third-party custom sources. |
| `CreateDataLake` | `POST /v1/datalake` | - | `configurations`, `metaStoreManagerRoleArn` | - | `CreateDataLakeResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Initializes an Amazon Security Lake instance with the provided (or default) configuration. You can enable Security Lake in Amazon Web Services Regions with customized settings before enabling log collection in Regions. |
| `CreateDataLakeExceptionSubscription` | `POST /v1/datalake/exceptions/subscription` | - | `notificationEndpoint`, `subscriptionProtocol` | - | `CreateDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates the specified notification subscription in Amazon Security Lake for the organization you specify. The notification subscription is created for exceptions that cannot be resolved by Security Lake automatically. |
| `CreateDataLakeOrganizationConfiguration` | `POST /v1/datalake/organization/configuration` | - | - | - | `CreateDataLakeOrganizationConfigurationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Automatically enables Amazon Security Lake for new member accounts in your organization. Security Lake is not automatically enabled for any existing member accounts in your organization. |
| `CreateSubscriber` | `POST /v1/subscribers` | - | `sources`, `subscriberIdentity`, `subscriberName` | - | `CreateSubscriberResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a subscriber for accounts that are already enabled in Amazon Security Lake. You can create a subscriber with access to data in the current Amazon Web Services Region. |
| `CreateSubscriberNotification` | `POST /v1/subscribers/{subscriberId}/notification` | - | `configuration`, `subscriberId` | - | `CreateSubscriberNotificationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Notifies the subscriber when new data is written to the data lake for the sources that the subscriber consumes in Security Lake. You can create only one subscriber notification per subscriber. |
| `DeleteAwsLogSource` | `POST /v1/datalake/logsources/aws/delete` | - | `sources` | - | `DeleteAwsLogSourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a natively supported Amazon Web Services service as an Amazon Security Lake source. You can remove a source for one or more Regions. |
| `DeleteCustomLogSource` | `DELETE /v1/datalake/logsources/custom/{sourceName}` | `idempotent` | `sourceName` | - | `DeleteCustomLogSourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes a custom log source from Amazon Security Lake, to stop sending data from the custom source to Security Lake. |
| `DeleteDataLake` | `POST /v1/datalake/delete` | `idempotent` | `regions` | - | `DeleteDataLakeResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | When you disable Amazon Security Lake from your account, Security Lake is disabled in all Amazon Web Services Regions and it stops collecting data from your sources. Also, this API automatically takes steps to remove the account from Security Lake. |
| `DeleteDataLakeExceptionSubscription` | `DELETE /v1/datalake/exceptions/subscription` | `idempotent` | - | - | `DeleteDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified notification subscription in Amazon Security Lake for the organization you specify. |
| `DeleteDataLakeOrganizationConfiguration` | `POST /v1/datalake/organization/configuration/delete` | - | - | - | `DeleteDataLakeOrganizationConfigurationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Turns off automatic enablement of Amazon Security Lake for member accounts that are added to an organization in Organizations. Only the delegated Security Lake administrator for an organization can perform this operation. |
| `DeleteSubscriber` | `DELETE /v1/subscribers/{subscriberId}` | `idempotent` | `subscriberId` | - | `DeleteSubscriberResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the subscription permission and all notification settings for accounts that are already enabled in Amazon Security Lake. When you run `DeleteSubscriber`, the subscriber will no longer consume data from Security Lake and the subscriber is removed. |
| `DeleteSubscriberNotification` | `DELETE /v1/subscribers/{subscriberId}/notification` | `idempotent` | `subscriberId` | - | `DeleteSubscriberNotificationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified subscription notification in Amazon Security Lake for the organization you specify. |
| `DeregisterDataLakeDelegatedAdministrator` | `DELETE /v1/datalake/delegate` | `idempotent` | - | - | `DeregisterDataLakeDelegatedAdministratorResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the Amazon Security Lake delegated administrator account for the organization. This API can only be called by the organization management account. |
| `GetDataLakeExceptionSubscription` | `GET /v1/datalake/exceptions/subscription` | `readonly` | - | - | `GetDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the protocol and endpoint that were provided when subscribing to Amazon SNS topics for exception notifications. |
| `GetDataLakeOrganizationConfiguration` | `GET /v1/datalake/organization/configuration` | `readonly` | - | - | `GetDataLakeOrganizationConfigurationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the configuration that will be automatically set up for accounts added to the organization after the organization has onboarded to Amazon Security Lake. This API does not take input parameters. |
| `GetDataLakeSources` | `POST /v1/datalake/sources` | `paginated` | - | - | `GetDataLakeSourcesResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves a snapshot of the current Region, including whether Amazon Security Lake is enabled for those accounts and which sources Security Lake is collecting data from. |
| `GetSubscriber` | `GET /v1/subscribers/{subscriberId}` | `readonly` | `subscriberId` | - | `GetSubscriberResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the subscription information for the specified subscription ID. You can get information about a specific subscriber. |
| `ListDataLakeExceptions` | `POST /v1/datalake/exceptions` | `readonly`, `paginated` | - | - | `ListDataLakeExceptionsResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the Amazon Security Lake exceptions that you can use to find the source of problems and fix them. |
| `ListDataLakes` | `GET /v1/datalakes` | `readonly` | - | - | `ListDataLakesResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the Amazon Security Lake configuration object for the specified Amazon Web Services Regions. You can use this operation to determine whether Security Lake is enabled for a Region. |
| `ListLogSources` | `POST /v1/datalake/logsources/list` | `readonly`, `paginated` | - | - | `ListLogSourcesResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the log sources. |
| `ListSubscribers` | `GET /v1/subscribers` | `readonly`, `paginated` | - | - | `ListSubscribersResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists all subscribers for the specific Amazon Security Lake account ID. You can retrieve a list of subscriptions associated with a specific organization or Amazon Web Services account. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the tags (keys and values) that are associated with an Amazon Security Lake resource: a subscriber, or the data lake configuration for your Amazon Web Services account in a particular Amazon Web Services Region. |
| `RegisterDataLakeDelegatedAdministrator` | `POST /v1/datalake/delegate` | - | `accountId` | - | `RegisterDataLakeDelegatedAdministratorResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Designates the Amazon Security Lake delegated administrator account for the organization. This API can only be called by the organization management account. |
| `TagResource` | `POST /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds or updates one or more tags that are associated with an Amazon Security Lake resource: a subscriber, or the data lake configuration for your Amazon Web Services account in a particular Amazon Web Services Region. A tag is a label that you can define and... |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags (keys and values) from an Amazon Security Lake resource: a subscriber, or the data lake configuration for your Amazon Web Services account in a particular Amazon Web Services Region. |
| `UpdateDataLake` | `PUT /v1/datalake` | `idempotent` | `configurations` | - | `UpdateDataLakeResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | You can use `UpdateDataLake` to specify where to store your security data, how it should be encrypted at rest and for how long. You can add a Rollup Region to consolidate data from multiple Amazon Web Services Regions, replace default encryption (SSE-S3) with... |
| `UpdateDataLakeExceptionSubscription` | `PUT /v1/datalake/exceptions/subscription` | `idempotent` | `notificationEndpoint`, `subscriptionProtocol` | - | `UpdateDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the specified notification subscription in Amazon Security Lake for the organization you specify. |
| `UpdateSubscriber` | `PUT /v1/subscribers/{subscriberId}` | `idempotent` | `subscriberId` | - | `UpdateSubscriberResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an existing subscription for the given Amazon Security Lake account ID. You can update a subscriber by changing the sources that the subscriber consumes data from. |
| `UpdateSubscriberNotification` | `PUT /v1/subscribers/{subscriberId}/notification` | `idempotent` | `configuration`, `subscriberId` | - | `UpdateSubscriberNotificationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates an existing notification method for the subscription (SQS or HTTPs endpoint) or switches the notification subscription endpoint for a subscriber. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `errorCode`, `message` | You do not have sufficient access to perform this action. |
| `BadRequestException` | `structure` | `message` | The request is malformed or contains an error such as an invalid parameter value or a missing required parameter. |
| `ConflictException` | `structure` | `message`, `resourceName`, `resourceType` | Occurs when a conflict with a previous successful write is detected. |
| `InternalServerException` | `structure` | `message` | Internal service exceptions are sometimes caused by transient issues. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceName`, `resourceType` | The resource could not be found. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The limit on the number of requests per second was exceeded. |
| `CreateAwsLogSourceRequest` | `structure` | `sources` | - |
| `CreateAwsLogSourceResponse` | `structure` | `failed` | - |
| `CreateCustomLogSourceRequest` | `structure` | `configuration`, `eventClasses`, `sourceName`, `sourceVersion` | - |
| `CreateCustomLogSourceResponse` | `structure` | `source` | - |
| `CreateDataLakeRequest` | `structure` | `configurations`, `metaStoreManagerRoleArn`, `tags` | - |
| `CreateDataLakeResponse` | `structure` | `dataLakes` | - |
| `CreateDataLakeExceptionSubscriptionRequest` | `structure` | `exceptionTimeToLive`, `notificationEndpoint`, `subscriptionProtocol` | - |
| `CreateDataLakeExceptionSubscriptionResponse` | `structure` | - | - |
| `CreateDataLakeOrganizationConfigurationRequest` | `structure` | `autoEnableNewAccount` | - |
| `CreateDataLakeOrganizationConfigurationResponse` | `structure` | - | - |
| `CreateSubscriberRequest` | `structure` | `accessTypes`, `sources`, `subscriberDescription`, `subscriberIdentity`, `subscriberName`, `tags` | - |
| `CreateSubscriberResponse` | `structure` | `subscriber` | - |
| `CreateSubscriberNotificationRequest` | `structure` | `configuration`, `subscriberId` | - |
| `CreateSubscriberNotificationResponse` | `structure` | `subscriberEndpoint` | - |
| `DeleteAwsLogSourceRequest` | `structure` | `sources` | - |
| `DeleteAwsLogSourceResponse` | `structure` | `failed` | - |
| `DeleteCustomLogSourceRequest` | `structure` | `sourceName`, `sourceVersion` | - |
| `DeleteCustomLogSourceResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

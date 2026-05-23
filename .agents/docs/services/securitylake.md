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

### List

- Operations: `ListDataLakeExceptions`, `ListTagsForResource`
- Traits: `readonly` (2), `paginated` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateDataLakeExceptionSubscription`
- Common required input members in this group: -

### Delete

- Operations: `DeleteDataLakeExceptionSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterDataLakeDelegatedAdministrator`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetDataLakeExceptionSubscription`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterDataLakeDelegatedAdministrator`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateDataLakeExceptionSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateDataLakeExceptionSubscription` | `POST /v1/datalake/exceptions/subscription` | - | `subscriptionProtocol`, `notificationEndpoint` | - | `CreateDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Creates the specified notification subscription in Amazon Security Lake for the organization you specify. The notification subscription is created for exceptions that cannot be resolved by Security Lake automatically. |
| `DeleteDataLakeExceptionSubscription` | `DELETE /v1/datalake/exceptions/subscription` | `idempotent` | - | - | `DeleteDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the specified notification subscription in Amazon Security Lake for the organization you specify. |
| `DeregisterDataLakeDelegatedAdministrator` | `DELETE /v1/datalake/delegate` | `idempotent` | - | - | `DeregisterDataLakeDelegatedAdministratorResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes the Amazon Security Lake delegated administrator account for the organization. This API can only be called by the organization management account. The organization management account cannot be the delegated a ... |
| `GetDataLakeExceptionSubscription` | `GET /v1/datalake/exceptions/subscription` | `readonly` | - | - | `GetDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the protocol and endpoint that were provided when subscribing to Amazon SNS topics for exception notifications. |
| `ListDataLakeExceptions` | `POST /v1/datalake/exceptions` | `readonly`, `paginated` | - | - | `ListDataLakeExceptionsResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Lists the Amazon Security Lake exceptions that you can use to find the source of problems and fix them. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the tags (keys and values) that are associated with an Amazon Security Lake resource: a subscriber, or the data lake configuration for your Amazon Web Services account in a particular Amazon Web Services Re ... |
| `RegisterDataLakeDelegatedAdministrator` | `POST /v1/datalake/delegate` | - | `accountId` | - | `RegisterDataLakeDelegatedAdministratorResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Designates the Amazon Security Lake delegated administrator account for the organization. This API can only be called by the organization management account. The organization management account cannot be the delegate ... |
| `TagResource` | `POST /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Adds or updates one or more tags that are associated with an Amazon Security Lake resource: a subscriber, or the data lake configuration for your Amazon Web Services account in a particular Amazon Web Services Region ... |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags (keys and values) from an Amazon Security Lake resource: a subscriber, or the data lake configuration for your Amazon Web Services account in a particular Amazon Web Services Region. |
| `UpdateDataLakeExceptionSubscription` | `PUT /v1/datalake/exceptions/subscription` | `idempotent` | `subscriptionProtocol`, `notificationEndpoint` | - | `UpdateDataLakeExceptionSubscriptionResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Updates the specified notification subscription in Amazon Security Lake for the organization you specify. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, errorCode | You do not have sufficient access to perform this action. Access denied errors appear when Amazon Security Lake explicitly or implicitly denies an authoriza ... |
| `BadRequestException` | `structure` | message | The request is malformed or contains an error such as an invalid parameter value or a missing required parameter. |
| `ConflictException` | `structure` | message, resourceName, resourceType | Occurs when a conflict with a previous successful write is detected. This generally occurs when the previous write did not have time to propagate to the hos ... |
| `InternalServerException` | `structure` | message | Internal service exceptions are sometimes caused by transient issues. Before you start troubleshooting, perform the operation again. |
| `ResourceNotFoundException` | `structure` | message, resourceName, resourceType | The resource could not be found. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The limit on the number of requests per second was exceeded. |
| `CreateDataLakeExceptionSubscriptionRequest` | `structure` | subscriptionProtocol, notificationEndpoint, exceptionTimeToLive | - |
| `CreateDataLakeExceptionSubscriptionResponse` | `structure` | **empty (no members)** | - |
| `DeleteDataLakeExceptionSubscriptionRequest` | `structure` | **empty (no members)** | - |
| `DeleteDataLakeExceptionSubscriptionResponse` | `structure` | **empty (no members)** | - |
| `DeregisterDataLakeDelegatedAdministratorRequest` | `structure` | **empty (no members)** | - |
| `DeregisterDataLakeDelegatedAdministratorResponse` | `structure` | **empty (no members)** | - |
| `GetDataLakeExceptionSubscriptionRequest` | `structure` | **empty (no members)** | - |
| `GetDataLakeExceptionSubscriptionResponse` | `structure` | subscriptionProtocol, notificationEndpoint, exceptionTimeToLive | - |
| `ListDataLakeExceptionsRequest` | `structure` | regions, maxResults, nextToken | - |
| `ListDataLakeExceptionsResponse` | `structure` | exceptions, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `RegisterDataLakeDelegatedAdministratorRequest` | `structure` | accountId | - |
| `RegisterDataLakeDelegatedAdministratorResponse` | `structure` | **empty (no members)** | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateDataLakeExceptionSubscriptionRequest` | `structure` | subscriptionProtocol, notificationEndpoint, exceptionTimeToLive | - |
| `UpdateDataLakeExceptionSubscriptionResponse` | `structure` | **empty (no members)** | - |
| `AccessType` | `enum` | LAKEFORMATION, S3 | - |
| `AwsLogSourceName` | `enum` | ROUTE53, VPC_FLOW, SH_FINDINGS, CLOUD_TRAIL_MGMT, LAMBDA_EXECUTION, S3_DATA, EKS_AUDIT, WAF | - |
| `DataLakeStatus` | `enum` | INITIALIZED, PENDING, COMPLETED, FAILED | - |
| `HttpMethod` | `enum` | POST, PUT | - |
| `SourceCollectionStatus` | `enum` | COLLECTING, MISCONFIGURED, NOT_COLLECTING | - |
| `SubscriberStatus` | `enum` | ACTIVE, DEACTIVATED, PENDING, READY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

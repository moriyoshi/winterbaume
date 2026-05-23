# AWS User Notifications

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The User Notifications API Reference provides descriptions, API request parameters, and the JSON response for each of the User Notifications API actions. User Notification control plane APIs are currently available in US East (Virginia) - `us-east-1`. GetNotificationEvent and ListNotificationEvents APIs are currently available in commercial partition Regions and only return notifications stored in the same Region in which they're called. The User Notifications console can only be used in US East (Virginia). Your data however, is stored in each Region chosen as a notification hub in addition to US East (Virginia).

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS User Notifications where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS User Notifications by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS User Notifications by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS User Notifications workflows in the local mock. Key resources include `Channel`, `EventRule`, `ManagedNotificationAccountContactAssociation`, `ManagedNotificationAdditionalChannelAssociation`, `ManagedNotificationChildEventResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Associate`, `Disassociate`, `Create` operation families, including `ListChannels`, `ListEventRules`, `ListManagedNotificationChannelAssociations`, `ListManagedNotificationChildEvents`, `GetEventRule`, `GetManagedNotificationChildEvent`.

## Service Identity and Protocol

- AWS model slug: `notifications`
- AWS SDK for Rust slug: `notifications`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/notifications/service/2018-05-10/notifications-2018-05-10.json`
- SDK ID: `Notifications`
- Endpoint prefix: `notifications`
- ARN namespace: `notifications`
- CloudFormation name: `-`
- CloudTrail event source: `notifications.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Get` (7), `Associate` (4), `Disassociate` (4), `Create` (2), `Delete` (2), `Update` (2), `Deregister` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateChannel`, `AssociateManagedNotificationAccountContact`, `AssociateManagedNotificationAdditionalChannel`, `AssociateOrganizationalUnit`, `CreateEventRule`, `CreateNotificationConfiguration`, `DeleteEventRule`, `DeleteNotificationConfiguration`, `DeregisterNotificationHub`, `DisableNotificationsAccessForOrganization`, `DisassociateChannel`, `DisassociateManagedNotificationAccountContact`, `DisassociateManagedNotificationAdditionalChannel`, `DisassociateOrganizationalUnit`, `EnableNotificationsAccessForOrganization`, `RegisterNotificationHub`, `TagResource`, `UntagResource`, `UpdateEventRule`, `UpdateNotificationConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEventRule`, `GetManagedNotificationChildEvent`, `GetManagedNotificationConfiguration`, `GetManagedNotificationEvent`, `GetNotificationConfiguration`, `GetNotificationEvent`, `GetNotificationsAccessForOrganization`, `ListChannels`, `ListEventRules`, `ListManagedNotificationChannelAssociations`, `ListManagedNotificationChildEvents`, `ListManagedNotificationConfigurations`, `ListManagedNotificationEvents`, `ListMemberAccounts`, `ListNotificationConfigurations`, `ListNotificationEvents`, `ListNotificationHubs`, `ListOrganizationalUnits`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 20 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EventBridge`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Channel` | `arn`, `notificationConfigurationArn` | put: `AssociateChannel`; delete: `DisassociateChannel`; list: `ListChannels` | - | - |
| `EventRule` | `arn` | create: `CreateEventRule`; put: `UpdateEventRule`; read: `GetEventRule`; delete: `DeleteEventRule`; list: `ListEventRules` | - | - |
| `ManagedNotificationAccountContactAssociation` | `contactIdentifier`, `managedNotificationConfigurationArn` | put: `AssociateManagedNotificationAccountContact`; delete: `DisassociateManagedNotificationAccountContact` | - | - |
| `ManagedNotificationAdditionalChannelAssociation` | `channelArn`, `managedNotificationConfigurationArn` | put: `AssociateManagedNotificationAdditionalChannel`; delete: `DisassociateManagedNotificationAdditionalChannel` | - | - |
| `ManagedNotificationChildEventResource` | `arn` | read: `GetManagedNotificationChildEvent`; list: `ListManagedNotificationChildEvents` | - | - |
| `ManagedNotificationConfiguration` | `arn` | read: `GetManagedNotificationConfiguration`; list: `ListManagedNotificationConfigurations` | - | - |
| `ManagedNotificationEventResource` | `arn` | read: `GetManagedNotificationEvent`; list: `ListManagedNotificationEvents` | - | - |
| `NotificationConfiguration` | `arn` | create: `CreateNotificationConfiguration`; put: `UpdateNotificationConfiguration`; read: `GetNotificationConfiguration`; delete: `DeleteNotificationConfiguration`; list: `ListNotificationConfigurations` | - | - |
| `NotificationEventResource` | `arn` | read: `GetNotificationEvent`; list: `ListNotificationEvents` | - | - |
| `NotificationHub` | `notificationHubRegion` | put: `RegisterNotificationHub`; delete: `DeregisterNotificationHub`; list: `ListNotificationHubs` | - | - |
| `OrganizationAccess` | - | put: `EnableNotificationsAccessForOrganization`; read: `GetNotificationsAccessForOrganization`; delete: `DisableNotificationsAccessForOrganization` | - | - |
| `OrganizationalUnit` | `notificationConfigurationArn`, `organizationalUnitId` | put: `AssociateOrganizationalUnit`; delete: `DisassociateOrganizationalUnit`; list: `ListOrganizationalUnits` | - | - |
## Operation Groups

### List

- Operations: `ListManagedNotificationChannelAssociations`, `ListMemberAccounts`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListManagedNotificationChannelAssociations` | `GET /channels/list-managed-notification-channel-associations` | `readonly`, `paginated` | `managedNotificationConfigurationArn` | - | `ListManagedNotificationChannelAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Account contacts and Channels associated with a ManagedNotificationConfiguration , in paginated format. |
| `ListMemberAccounts` | `GET /list-member-accounts` | `readonly`, `paginated` | `notificationConfigurationArn` | - | `ListMemberAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of member accounts associated with a notification configuration. |
| `ListTagsForResource` | `GET /tags/{arn}` | `readonly` | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a specified Amazon Resource Name (ARN). For more information, see Tagging your Amazon Web Services resources in the Tagging Amazon Web Services Resources User Guide . This is only supported ... |
| `TagResource` | `POST /tags/{arn}` | `idempotent` | `arn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags the resource with a tag key and value. For more information, see Tagging your Amazon Web Services resources in the Tagging Amazon Web Services Resources User Guide . This is only supported for NotificationConfig ... |
| `UntagResource` | `DELETE /tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untags a resource with a specified Amazon Resource Name (ARN). For more information, see Tagging your Amazon Web Services resources in the Tagging Amazon Web Services Resources User Guide . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListManagedNotificationChannelAssociations` | - | `managedNotificationConfigurationArn -> managedNotificationConfigurationArn`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListMemberAccounts` | - | `notificationConfigurationArn -> notificationConfigurationArn`, `maxResults -> maxResults`, `nextToken -> nextToken`, `memberAccount -> memberAccount`, `status -> status`, `organizationalUnitId -> organizationalUnitId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | User does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId | Updating or deleting a resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message | Unexpected error during processing of request. |
| `ResourceNotFoundException` | `structure` | message, resourceId | Request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceType, resourceId, serviceCode, quotaCode | Request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | Request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | This exception is thrown when the notification event fails validation. |
| `ListManagedNotificationChannelAssociationsRequest` | `structure` | managedNotificationConfigurationArn, maxResults, nextToken | - |
| `ListManagedNotificationChannelAssociationsResponse` | `structure` | nextToken, channelAssociations | - |
| `ListMemberAccountsRequest` | `structure` | notificationConfigurationArn, maxResults, nextToken, memberAccount, status, organizationalUnitId | - |
| `ListMemberAccountsResponse` | `structure` | memberAccounts, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | arn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | arn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | arn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AccessStatus` | `enum` | ENABLED, DISABLED, PENDING, FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

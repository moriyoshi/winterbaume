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

- Operations: `ListChannels`, `ListEventRules`, `ListManagedNotificationChannelAssociations`, `ListManagedNotificationChildEvents`, `ListManagedNotificationConfigurations`, `ListManagedNotificationEvents`, `ListMemberAccounts`, `ListNotificationConfigurations`, `ListNotificationEvents`, `ListNotificationHubs`, `ListOrganizationalUnits`, `ListTagsForResource`
- Traits: `paginated` (11), `readonly` (12)
- Common required input members in this group: `aggregateManagedNotificationEventArn`, `arn`, `managedNotificationConfigurationArn`, `notificationConfigurationArn`

### Get

- Operations: `GetEventRule`, `GetManagedNotificationChildEvent`, `GetManagedNotificationConfiguration`, `GetManagedNotificationEvent`, `GetNotificationConfiguration`, `GetNotificationEvent`, `GetNotificationsAccessForOrganization`
- Traits: `readonly` (7)
- Common required input members in this group: `arn`

### Associate

- Operations: `AssociateChannel`, `AssociateManagedNotificationAccountContact`, `AssociateManagedNotificationAdditionalChannel`, `AssociateOrganizationalUnit`
- Traits: `idempotent` (4)
- Common required input members in this group: `arn`, `channelArn`, `contactIdentifier`, `managedNotificationConfigurationArn`, `notificationConfigurationArn`, `organizationalUnitId`

### Disassociate

- Operations: `DisassociateChannel`, `DisassociateManagedNotificationAccountContact`, `DisassociateManagedNotificationAdditionalChannel`, `DisassociateOrganizationalUnit`
- Traits: `idempotent` (4)
- Common required input members in this group: `arn`, `channelArn`, `contactIdentifier`, `managedNotificationConfigurationArn`, `notificationConfigurationArn`, `organizationalUnitId`

### Create

- Operations: `CreateEventRule`, `CreateNotificationConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `description`, `eventType`, `name`, `notificationConfigurationArn`, `regions`, `source`

### Delete

- Operations: `DeleteEventRule`, `DeleteNotificationConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `arn`

### Update

- Operations: `UpdateEventRule`, `UpdateNotificationConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `arn`

### Deregister

- Operations: `DeregisterNotificationHub`
- Traits: `idempotent` (1)
- Common required input members in this group: `notificationHubRegion`

### Disable

- Operations: `DisableNotificationsAccessForOrganization`
- Traits: `idempotent` (1)

### Enable

- Operations: `EnableNotificationsAccessForOrganization`
- Traits: `idempotent` (1)

### Register

- Operations: `RegisterNotificationHub`
- Traits: `idempotent` (1)
- Common required input members in this group: `notificationHubRegion`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateChannel` | `POST /channels/associate/{arn}` | `idempotent` | `arn`, `notificationConfigurationArn` | - | `AssociateChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a delivery Channel with a particular `NotificationConfiguration`. Supported Channels include Amazon Q Developer in chat applications, the Console Mobile Application, and emails (notifications-contacts). |
| `AssociateManagedNotificationAccountContact` | `PUT /contacts/associate-managed-notification/{contactIdentifier}` | `idempotent` | `contactIdentifier`, `managedNotificationConfigurationArn` | - | `AssociateManagedNotificationAccountContactResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates an Account Contact with a particular `ManagedNotificationConfiguration`. |
| `AssociateManagedNotificationAdditionalChannel` | `PUT /channels/associate-managed-notification/{channelArn}` | `idempotent` | `channelArn`, `managedNotificationConfigurationArn` | - | `AssociateManagedNotificationAdditionalChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates an additional Channel with a particular `ManagedNotificationConfiguration`. Supported Channels include Amazon Q Developer in chat applications, the Console Mobile Application, and emails (notifications-contacts). |
| `AssociateOrganizationalUnit` | `POST /organizational-units/associate/{organizationalUnitId}` | `idempotent` | `notificationConfigurationArn`, `organizationalUnitId` | - | `AssociateOrganizationalUnitResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates an organizational unit with a notification configuration. |
| `CreateEventRule` | `POST /event-rules` | `idempotent` | `eventType`, `notificationConfigurationArn`, `regions`, `source` | - | `CreateEventRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an `EventRule` that is associated with a specified `NotificationConfiguration`. |
| `CreateNotificationConfiguration` | `POST /notification-configurations` | `idempotent` | `description`, `name` | - | `CreateNotificationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new `NotificationConfiguration`. |
| `DeleteEventRule` | `DELETE /event-rules/{arn}` | `idempotent` | `arn` | - | `DeleteEventRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an `EventRule`. |
| `DeleteNotificationConfiguration` | `DELETE /notification-configurations/{arn}` | `idempotent` | `arn` | - | `DeleteNotificationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a `NotificationConfiguration`. |
| `DeregisterNotificationHub` | `DELETE /notification-hubs/{notificationHubRegion}` | `idempotent` | `notificationHubRegion` | - | `DeregisterNotificationHubResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregisters a `NotificationConfiguration` in the specified Region. You can't deregister the last `NotificationHub` in the account. |
| `DisableNotificationsAccessForOrganization` | `DELETE /organization/access` | `idempotent` | - | - | `DisableNotificationsAccessForOrganizationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disables service trust between User Notifications and Amazon Web Services Organizations. |
| `DisassociateChannel` | `POST /channels/disassociate/{arn}` | `idempotent` | `arn`, `notificationConfigurationArn` | - | `DisassociateChannelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a Channel from a specified `NotificationConfiguration`. Supported Channels include Amazon Q Developer in chat applications, the Console Mobile Application, and emails (notifications-contacts). |
| `DisassociateManagedNotificationAccountContact` | `PUT /contacts/disassociate-managed-notification/{contactIdentifier}` | `idempotent` | `contactIdentifier`, `managedNotificationConfigurationArn` | - | `DisassociateManagedNotificationAccountContactResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an Account Contact with a particular `ManagedNotificationConfiguration`. |
| `DisassociateManagedNotificationAdditionalChannel` | `PUT /channels/disassociate-managed-notification/{channelArn}` | `idempotent` | `channelArn`, `managedNotificationConfigurationArn` | - | `DisassociateManagedNotificationAdditionalChannelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an additional Channel from a particular `ManagedNotificationConfiguration`. Supported Channels include Amazon Q Developer in chat applications, the Console Mobile Application, and emails (notifications-contacts). |
| `DisassociateOrganizationalUnit` | `POST /organizational-units/disassociate/{organizationalUnitId}` | `idempotent` | `notificationConfigurationArn`, `organizationalUnitId` | - | `DisassociateOrganizationalUnitResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between an organizational unit and a notification configuration. |
| `EnableNotificationsAccessForOrganization` | `POST /organization/access` | `idempotent` | - | - | `EnableNotificationsAccessForOrganizationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables service trust between User Notifications and Amazon Web Services Organizations. |
| `GetEventRule` | `GET /event-rules/{arn}` | `readonly` | `arn` | - | `GetEventRuleResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specified `EventRule`. |
| `GetManagedNotificationChildEvent` | `GET /managed-notification-child-events/{arn}` | `readonly` | `arn` | - | `GetManagedNotificationChildEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the child event of a specific given `ManagedNotificationEvent`. |
| `GetManagedNotificationConfiguration` | `GET /managed-notification-configurations/{arn}` | `readonly` | `arn` | - | `GetManagedNotificationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specified `ManagedNotificationConfiguration`. |
| `GetManagedNotificationEvent` | `GET /managed-notification-events/{arn}` | `readonly` | `arn` | - | `GetManagedNotificationEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specified `ManagedNotificationEvent`. |
| `GetNotificationConfiguration` | `GET /notification-configurations/{arn}` | `readonly` | `arn` | - | `GetNotificationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specified `NotificationConfiguration`. |
| `GetNotificationEvent` | `GET /notification-events/{arn}` | `readonly` | `arn` | - | `GetNotificationEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specified `NotificationEvent`. User Notifications stores notifications in the individual Regions you register as notification hubs and the Region of the source event rule. |
| `GetNotificationsAccessForOrganization` | `GET /organization/access` | `readonly` | - | - | `GetNotificationsAccessForOrganizationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the AccessStatus of Service Trust Enablement for User Notifications and Amazon Web Services Organizations. |
| `ListChannels` | `GET /channels` | `readonly`, `paginated` | `notificationConfigurationArn` | - | `ListChannelsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Channels for a `NotificationConfiguration`. |
| `ListEventRules` | `GET /event-rules` | `readonly`, `paginated` | `notificationConfigurationArn` | - | `ListEventRulesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of `EventRules` according to specified filters, in reverse chronological order (newest first). |
| `ListManagedNotificationChannelAssociations` | `GET /channels/list-managed-notification-channel-associations` | `readonly`, `paginated` | `managedNotificationConfigurationArn` | - | `ListManagedNotificationChannelAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Account contacts and Channels associated with a `ManagedNotificationConfiguration`, in paginated format. |
| `ListManagedNotificationChildEvents` | `GET /list-managed-notification-child-events/{aggregateManagedNotificationEventArn}` | `readonly`, `paginated` | `aggregateManagedNotificationEventArn` | - | `ListManagedNotificationChildEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of `ManagedNotificationChildEvents` for a specified aggregate `ManagedNotificationEvent`, ordered by creation time in reverse chronological order (newest first). |
| `ListManagedNotificationConfigurations` | `GET /managed-notification-configurations` | `readonly`, `paginated` | - | - | `ListManagedNotificationConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of Managed Notification Configurations according to specified filters, ordered by creation time in reverse chronological order (newest first). |
| `ListManagedNotificationEvents` | `GET /managed-notification-events` | `readonly`, `paginated` | - | - | `ListManagedNotificationEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of Managed Notification Events according to specified filters, ordered by creation time in reverse chronological order (newest first). |
| `ListMemberAccounts` | `GET /list-member-accounts` | `readonly`, `paginated` | `notificationConfigurationArn` | - | `ListMemberAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of member accounts associated with a notification configuration. |
| `ListNotificationConfigurations` | `GET /notification-configurations` | `readonly`, `paginated` | - | - | `ListNotificationConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of abbreviated `NotificationConfigurations` according to specified filters, in reverse chronological order (newest first). |
| `ListNotificationEvents` | `GET /notification-events` | `readonly`, `paginated` | - | - | `ListNotificationEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of `NotificationEvents` according to specified filters, in reverse chronological order (newest first). User Notifications stores notifications in the individual Regions you register as notification hubs and the Region of the source event rule. |
| `ListNotificationHubs` | `GET /notification-hubs` | `readonly`, `paginated` | - | - | `ListNotificationHubsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of `NotificationHubs`. |
| `ListOrganizationalUnits` | `GET /organizational-units` | `readonly`, `paginated` | `notificationConfigurationArn` | - | `ListOrganizationalUnitsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of organizational units associated with a notification configuration. |
| `ListTagsForResource` | `GET /tags/{arn}` | `readonly` | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a specified Amazon Resource Name (ARN). For more information, see Tagging your Amazon Web Services resources in the Tagging Amazon Web Services Resources User Guide . |
| `RegisterNotificationHub` | `POST /notification-hubs` | `idempotent` | `notificationHubRegion` | - | `RegisterNotificationHubResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Registers a `NotificationConfiguration` in the specified Region. There is a maximum of one `NotificationConfiguration` per Region. |
| `TagResource` | `POST /tags/{arn}` | `idempotent` | `arn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags the resource with a tag key and value. For more information, see Tagging your Amazon Web Services resources in the Tagging Amazon Web Services Resources User Guide . |
| `UntagResource` | `DELETE /tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untags a resource with a specified Amazon Resource Name (ARN). For more information, see Tagging your Amazon Web Services resources in the Tagging Amazon Web Services Resources User Guide . |
| `UpdateEventRule` | `PUT /event-rules/{arn}` | `idempotent` | `arn` | - | `UpdateEventRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing `EventRule`. |
| `UpdateNotificationConfiguration` | `PUT /notification-configurations/{arn}` | `idempotent` | `arn` | - | `UpdateNotificationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a `NotificationConfiguration`. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | User does not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | Unexpected error during processing of request. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | Request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | This exception is thrown when the notification event fails validation. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId` | Request references a resource which does not exist. |
| `ConflictException` | `structure` | `message`, `resourceId` | Updating or deleting a resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | Request would cause a service quota to be exceeded. |
| `AssociateChannelRequest` | `structure` | `arn`, `notificationConfigurationArn` | - |
| `AssociateChannelResponse` | `structure` | - | - |
| `AssociateManagedNotificationAccountContactRequest` | `structure` | `contactIdentifier`, `managedNotificationConfigurationArn` | - |
| `AssociateManagedNotificationAccountContactResponse` | `structure` | - | - |
| `AssociateManagedNotificationAdditionalChannelRequest` | `structure` | `channelArn`, `managedNotificationConfigurationArn` | - |
| `AssociateManagedNotificationAdditionalChannelResponse` | `structure` | - | - |
| `AssociateOrganizationalUnitRequest` | `structure` | `notificationConfigurationArn`, `organizationalUnitId` | - |
| `AssociateOrganizationalUnitResponse` | `structure` | - | - |
| `CreateEventRuleRequest` | `structure` | `eventPattern`, `eventType`, `notificationConfigurationArn`, `regions`, `source` | - |
| `CreateEventRuleResponse` | `structure` | `arn`, `notificationConfigurationArn`, `statusSummaryByRegion` | - |
| `CreateNotificationConfigurationRequest` | `structure` | `aggregationDuration`, `description`, `name`, `tags` | - |
| `CreateNotificationConfigurationResponse` | `structure` | `arn`, `status` | - |
| `DeleteEventRuleRequest` | `structure` | `arn` | - |
| `DeleteEventRuleResponse` | `structure` | - | - |
| `DeleteNotificationConfigurationRequest` | `structure` | `arn` | - |
| `DeleteNotificationConfigurationResponse` | `structure` | - | - |
| `DeregisterNotificationHubRequest` | `structure` | `notificationHubRegion` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

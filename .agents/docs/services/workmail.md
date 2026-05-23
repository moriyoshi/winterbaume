# Amazon WorkMail

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

WorkMail is a secure, managed business email and calendaring service with support for existing desktop and mobile email clients. You can access your email, contacts, and calendars using Microsoft Outlook, your browser, or other native iOS and Android email applications. You can integrate WorkMail with your existing corporate directory and control both the keys that encrypt your data and the location in which your data is stored. The WorkMail API is designed for the following scenarios: Listing and describing organizations Managing users Managing groups Managing resources All WorkMail API operations are Amazon-authenticated and certificate-signed. They not only require the use of the AWS SDK, but also allow for the exclusive use of AWS Identity and Access Management users and roles to help facilitate access, trust, and permission policies.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon WorkMail where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon WorkMail by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon WorkMail by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon WorkMail workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Create`, `Describe`, `Get` operation families, including `ListAccessControlRules`, `ListAliases`, `ListAvailabilityConfigurations`, `ListGroupMembers`, `DeleteAccessControlRule`, `DeleteAlias`.

## Service Identity and Protocol

- AWS model slug: `workmail`
- AWS SDK for Rust slug: `workmail`
- Model version: `2017-10-01`
- Model file: `vendor/api-models-aws/models/workmail/service/2017-10-01/workmail-2017-10-01.json`
- SDK ID: `WorkMail`
- Endpoint prefix: `workmail`
- ARN namespace: `workmail`
- CloudFormation name: `WorkMail`
- CloudTrail event source: `workmail.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (18), `Delete` (16), `Create` (9), `Describe` (9), `Get` (9), `Update` (9), `Put` (7), `Associate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateDelegateToResource`, `AssociateMemberToGroup`, `CancelMailboxExportJob`, `CreateAlias`, `CreateAvailabilityConfiguration`, `CreateGroup`, `CreateIdentityCenterApplication`, `CreateImpersonationRole`, `CreateMobileDeviceAccessRule`, `CreateOrganization`, `CreateResource`, `CreateUser`, `DeleteAccessControlRule`, `DeleteAlias`, `DeleteAvailabilityConfiguration`, `DeleteEmailMonitoringConfiguration`, `DeleteGroup`, `DeleteIdentityCenterApplication`, `DeleteIdentityProviderConfiguration`, `DeleteImpersonationRole`, ... (+33).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeEmailMonitoringConfiguration`, `DescribeEntity`, `DescribeGroup`, `DescribeIdentityProviderConfiguration`, `DescribeInboundDmarcSettings`, `DescribeMailboxExportJob`, `DescribeOrganization`, `DescribeResource`, `DescribeUser`, `GetAccessControlEffect`, `GetDefaultRetentionPolicy`, `GetImpersonationRole`, `GetImpersonationRoleEffect`, `GetMailDomain`, `GetMailboxDetails`, `GetMobileDeviceAccessEffect`, `GetMobileDeviceAccessOverride`, `GetPersonalAccessTokenMetadata`, `ListAccessControlRules`, `ListAliases`, ... (+16).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 69 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelMailboxExportJob`, `DescribeMailboxExportJob`, `ListMailboxExportJobs`, `StartMailboxExportJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 92 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Lambda`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAccessControlRules`, `ListAliases`, `ListAvailabilityConfigurations`, `ListGroupMembers`, `ListGroups`, `ListGroupsForEntity`, `ListImpersonationRoles`, `ListMailboxExportJobs`, `ListMailboxPermissions`, `ListMailDomains`, `ListMobileDeviceAccessOverrides`, `ListMobileDeviceAccessRules`, `ListOrganizations`, `ListPersonalAccessTokens`, `ListResourceDelegates`, `ListResources`, `ListTagsForResource`, `ListUsers`
- Traits: `idempotent` (12), `paginated` (15)
- Common required input members in this group: `OrganizationId`, `EntityId`

### Delete

- Operations: `DeleteAccessControlRule`, `DeleteAlias`, `DeleteAvailabilityConfiguration`, `DeleteEmailMonitoringConfiguration`, `DeleteGroup`, `DeleteIdentityCenterApplication`, `DeleteIdentityProviderConfiguration`, `DeleteImpersonationRole`, `DeleteMailboxPermissions`, `DeleteMobileDeviceAccessOverride`, `DeleteMobileDeviceAccessRule`, `DeleteOrganization`, `DeletePersonalAccessToken`, `DeleteResource`, `DeleteRetentionPolicy`, `DeleteUser`
- Traits: `idempotent` (12), `idempotency-token` (1)
- Common required input members in this group: `OrganizationId`, `EntityId`, `UserId`

### Create

- Operations: `CreateAlias`, `CreateAvailabilityConfiguration`, `CreateGroup`, `CreateIdentityCenterApplication`, `CreateImpersonationRole`, `CreateMobileDeviceAccessRule`, `CreateOrganization`, `CreateResource`, `CreateUser`
- Traits: `idempotent` (7), `idempotency-token` (5)
- Common required input members in this group: `OrganizationId`, `Alias`, `Name`, `Type`

### Describe

- Operations: `DescribeEmailMonitoringConfiguration`, `DescribeEntity`, `DescribeGroup`, `DescribeIdentityProviderConfiguration`, `DescribeInboundDmarcSettings`, `DescribeMailboxExportJob`, `DescribeOrganization`, `DescribeResource`, `DescribeUser`
- Traits: `idempotent` (9)
- Common required input members in this group: `OrganizationId`

### Get

- Operations: `GetAccessControlEffect`, `GetDefaultRetentionPolicy`, `GetImpersonationRole`, `GetImpersonationRoleEffect`, `GetMailboxDetails`, `GetMailDomain`, `GetMobileDeviceAccessEffect`, `GetMobileDeviceAccessOverride`, `GetPersonalAccessTokenMetadata`
- Traits: `idempotent` (4)
- Common required input members in this group: `OrganizationId`, `ImpersonationRoleId`, `UserId`

### Update

- Operations: `UpdateAvailabilityConfiguration`, `UpdateDefaultMailDomain`, `UpdateGroup`, `UpdateImpersonationRole`, `UpdateMailboxQuota`, `UpdateMobileDeviceAccessRule`, `UpdatePrimaryEmailAddress`, `UpdateResource`, `UpdateUser`
- Traits: `idempotent` (7)
- Common required input members in this group: `OrganizationId`, `DomainName`, `Name`, `UserId`

### Put

- Operations: `PutAccessControlRule`, `PutEmailMonitoringConfiguration`, `PutIdentityProviderConfiguration`, `PutInboundDmarcSettings`, `PutMailboxPermissions`, `PutMobileDeviceAccessOverride`, `PutRetentionPolicy`
- Traits: `idempotent` (5)
- Common required input members in this group: `Name`, `Effect`, `OrganizationId`

### Associate

- Operations: `AssociateDelegateToResource`, `AssociateMemberToGroup`
- Traits: `idempotent` (2)
- Common required input members in this group: `OrganizationId`

### Deregister

- Operations: `DeregisterFromWorkMail`, `DeregisterMailDomain`
- Traits: `idempotent` (2)
- Common required input members in this group: `OrganizationId`

### Disassociate

- Operations: `DisassociateDelegateFromResource`, `DisassociateMemberFromGroup`
- Traits: `idempotent` (2)
- Common required input members in this group: `OrganizationId`

### Register

- Operations: `RegisterMailDomain`, `RegisterToWorkMail`
- Traits: `idempotent` (2), `idempotency-token` (1)
- Common required input members in this group: `OrganizationId`

### Assume

- Operations: `AssumeImpersonationRole`
- Common required input members in this group: -

### Cancel

- Operations: `CancelMailboxExportJob`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Reset

- Operations: `ResetPassword`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Start

- Operations: `StartMailboxExportJob`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Test

- Operations: `TestAvailabilityConfiguration`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateDelegateToResource` | `-` | `idempotent` | `OrganizationId`, `ResourceId`, `EntityId` | - | `AssociateDelegateToResourceResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Adds a member (user or group) to the resource's set of delegates. |
| `AssociateMemberToGroup` | `-` | `idempotent` | `OrganizationId`, `GroupId`, `MemberId` | - | `AssociateMemberToGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Adds a member (user or group) to the group's set. |
| `AssumeImpersonationRole` | `-` | - | `OrganizationId`, `ImpersonationRoleId` | - | `AssumeImpersonationRoleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Assumes an impersonation role for the given WorkMail organization. This method returns an authentication token you can use to make impersonated calls. |
| `CancelMailboxExportJob` | `-` | `idempotent`, `idempotency-token` | `ClientToken`, `JobId`, `OrganizationId` | `ClientToken` | `CancelMailboxExportJobResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Cancels a mailbox export job. If the mailbox export job is near completion, it might not be possible to cancel it. |
| `CreateAlias` | `-` | `idempotent` | `OrganizationId`, `EntityId`, `Alias` | - | `CreateAliasResponse` | `EmailAddressInUseException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `LimitExceededException`, `MailDomainNotFoundException`, `MailDomainStateException`, `OrganizationNotFoundException`, `OrganizationStateException` | Adds an alias to the set of a given member (user or group) of WorkMail. |
| `CreateAvailabilityConfiguration` | `-` | `idempotent`, `idempotency-token` | `OrganizationId`, `DomainName` | `ClientToken` | `CreateAvailabilityConfigurationResponse` | `InvalidParameterException`, `LimitExceededException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates an AvailabilityConfiguration for the given WorkMail organization and domain. |
| `CreateGroup` | `-` | `idempotent` | `OrganizationId`, `Name` | - | `CreateGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `InvalidParameterException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ReservedNameException`, `UnsupportedOperationException` | Creates a group that can be used in WorkMail by calling the RegisterToWorkMail operation. |
| `CreateIdentityCenterApplication` | `-` | `idempotent`, `idempotency-token` | `Name`, `InstanceArn` | `ClientToken` | `CreateIdentityCenterApplicationResponse` | `InvalidParameterException` | Creates the WorkMail application in IAM Identity Center that can be used later in the WorkMail - IdC integration. For more information, see PutIdentityProviderConfiguration. This action does not affect the authentica ... |
| `CreateImpersonationRole` | `-` | `idempotency-token` | `OrganizationId`, `Name`, `Type`, `Rules` | `ClientToken` | `CreateImpersonationRoleResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates an impersonation role for the given WorkMail organization. Idempotency ensures that an API request completes no more than one time. With an idempotent request, if the original request completes successfully, ... |
| `CreateMobileDeviceAccessRule` | `-` | `idempotency-token` | `OrganizationId`, `Name`, `Effect` | `ClientToken` | `CreateMobileDeviceAccessRuleResponse` | `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates a new mobile device access rule for the specified WorkMail organization. |
| `CreateOrganization` | `-` | `idempotent`, `idempotency-token` | `Alias` | `ClientToken` | `CreateOrganizationResponse` | `DirectoryInUseException`, `DirectoryUnavailableException`, `InvalidParameterException`, `LimitExceededException`, `NameAvailabilityException` | Creates a new WorkMail organization. Optionally, you can choose to associate an existing AWS Directory Service directory with your organization. If an AWS Directory Service directory ID is specified, the organization ... |
| `CreateResource` | `-` | `idempotent` | `OrganizationId`, `Name`, `Type` | - | `CreateResourceResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `InvalidParameterException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ReservedNameException`, `UnsupportedOperationException` | Creates a new WorkMail resource. |
| `CreateUser` | `-` | `idempotent` | `OrganizationId`, `Name`, `DisplayName` | - | `CreateUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `InvalidParameterException`, `InvalidPasswordException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ReservedNameException`, `UnsupportedOperationException` | Creates a user who can be used in WorkMail by calling the RegisterToWorkMail operation. |
| `DeleteAccessControlRule` | `-` | - | `OrganizationId`, `Name` | - | `DeleteAccessControlRuleResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Deletes an access control rule for the specified WorkMail organization. Deleting already deleted and non-existing rules does not produce an error. In those cases, the service sends back an HTTP 200 response with an e ... |
| `DeleteAlias` | `-` | `idempotent` | `OrganizationId`, `EntityId`, `Alias` | - | `DeleteAliasResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Remove one or more specified aliases from a set of aliases for a given user. |
| `DeleteAvailabilityConfiguration` | `-` | `idempotent` | `OrganizationId`, `DomainName` | - | `DeleteAvailabilityConfigurationResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the AvailabilityConfiguration for the given WorkMail organization and domain. |
| `DeleteEmailMonitoringConfiguration` | `-` | `idempotent` | `OrganizationId` | - | `DeleteEmailMonitoringConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the email monitoring configuration for a specified organization. |
| `DeleteGroup` | `-` | `idempotent` | `OrganizationId`, `GroupId` | - | `DeleteGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Deletes a group from WorkMail. |
| `DeleteIdentityCenterApplication` | `-` | `idempotent` | `ApplicationArn` | - | `DeleteIdentityCenterApplicationResponse` | `InvalidParameterException`, `OrganizationStateException` | Deletes the IAM Identity Center application from WorkMail. This action does not affect the authentication settings for any WorkMail organizations. |
| `DeleteIdentityProviderConfiguration` | `-` | `idempotent` | `OrganizationId` | - | `DeleteIdentityProviderConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Disables the integration between IdC and WorkMail. Authentication will continue with the directory as it was before the IdC integration. You might have to reset your directory passwords and reconfigure your desktop a ... |
| `DeleteImpersonationRole` | `-` | - | `OrganizationId`, `ImpersonationRoleId` | - | `DeleteImpersonationRoleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes an impersonation role for the given WorkMail organization. |
| `DeleteMailboxPermissions` | `-` | `idempotent` | `OrganizationId`, `EntityId`, `GranteeId` | - | `DeleteMailboxPermissionsResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes permissions granted to a member (user or group). |
| `DeleteMobileDeviceAccessOverride` | `-` | - | `OrganizationId`, `UserId`, `DeviceId` | - | `DeleteMobileDeviceAccessOverrideResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the mobile device access override for the given WorkMail organization, user, and device. Deleting already deleted and non-existing overrides does not produce an error. In those cases, the service sends back a ... |
| `DeleteMobileDeviceAccessRule` | `-` | - | `OrganizationId`, `MobileDeviceAccessRuleId` | - | `DeleteMobileDeviceAccessRuleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes a mobile device access rule for the specified WorkMail organization. Deleting already deleted and non-existing rules does not produce an error. In those cases, the service sends back an HTTP 200 response with ... |
| `DeleteOrganization` | `-` | `idempotent`, `idempotency-token` | `OrganizationId`, `DeleteDirectory` | `ClientToken` | `DeleteOrganizationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes an WorkMail organization and all underlying AWS resources managed by WorkMail as part of the organization. You can choose whether to delete the associated directory. For more information, see Removing an orga ... |
| `DeletePersonalAccessToken` | `-` | `idempotent` | `OrganizationId`, `PersonalAccessTokenId` | - | `DeletePersonalAccessTokenResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the Personal Access Token from the provided WorkMail Organization. |
| `DeleteResource` | `-` | `idempotent` | `OrganizationId`, `ResourceId` | - | `DeleteResourceResponse` | `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Deletes the specified resource. |
| `DeleteRetentionPolicy` | `-` | `idempotent` | `OrganizationId`, `Id` | - | `DeleteRetentionPolicyResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the specified retention policy from the specified organization. |
| `DeleteUser` | `-` | `idempotent` | `OrganizationId`, `UserId` | - | `DeleteUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Deletes a user from WorkMail and all subsequent systems. Before you can delete a user, the user state must be DISABLED . Use the DescribeUser action to confirm the user state. Deleting a user is permanent and cannot ... |
| `DeregisterFromWorkMail` | `-` | `idempotent` | `OrganizationId`, `EntityId` | - | `DeregisterFromWorkMailResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Mark a user, group, or resource as no longer used in WorkMail. This action disassociates the mailbox and schedules it for clean-up. WorkMail keeps mailboxes for 30 days before they are permanently removed. The functi ... |
| `DeregisterMailDomain` | `-` | `idempotent` | `OrganizationId`, `DomainName` | - | `DeregisterMailDomainResponse` | `InvalidCustomSesConfigurationException`, `InvalidParameterException`, `MailDomainInUseException`, `OrganizationNotFoundException`, `OrganizationStateException` | Removes a domain from WorkMail, stops email routing to WorkMail, and removes the authorization allowing WorkMail use. SES keeps the domain because other applications may use it. You must first remove any email addres ... |
| `DescribeEmailMonitoringConfiguration` | `-` | `idempotent` | `OrganizationId` | - | `DescribeEmailMonitoringConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Describes the current email monitoring configuration for a specified organization. |
| `DescribeEntity` | `-` | `idempotent` | `OrganizationId`, `Email` | - | `DescribeEntityResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns basic details about an entity in WorkMail. |
| `DescribeGroup` | `-` | `idempotent` | `OrganizationId`, `GroupId` | - | `DescribeGroupResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns the data available for the group. |
| `DescribeIdentityProviderConfiguration` | `-` | `idempotent` | `OrganizationId` | - | `DescribeIdentityProviderConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Returns detailed information on the current IdC setup for the WorkMail organization. |
| `DescribeInboundDmarcSettings` | `-` | `idempotent` | `OrganizationId` | - | `DescribeInboundDmarcSettingsResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Lists the settings in a DMARC policy for a specified organization. |
| `DescribeMailboxExportJob` | `-` | `idempotent` | `JobId`, `OrganizationId` | - | `DescribeMailboxExportJobResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Describes the current status of a mailbox export job. |
| `DescribeOrganization` | `-` | `idempotent` | `OrganizationId` | - | `DescribeOrganizationResponse` | `InvalidParameterException`, `OrganizationNotFoundException` | Provides more information regarding a given organization based on its identifier. |
| `DescribeResource` | `-` | `idempotent` | `OrganizationId`, `ResourceId` | - | `DescribeResourceResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Returns the data available for the resource. |
| `DescribeUser` | `-` | `idempotent` | `OrganizationId`, `UserId` | - | `DescribeUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Provides information regarding the user. |
| `DisassociateDelegateFromResource` | `-` | `idempotent` | `OrganizationId`, `ResourceId`, `EntityId` | - | `DisassociateDelegateFromResourceResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Removes a member from the resource's set of delegates. |
| `DisassociateMemberFromGroup` | `-` | `idempotent` | `OrganizationId`, `GroupId`, `MemberId` | - | `DisassociateMemberFromGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Removes a member from a group. |
| `GetAccessControlEffect` | `-` | - | `OrganizationId`, `IpAddress`, `Action` | - | `GetAccessControlEffectResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Gets the effects of an organization's access control rules as they apply to a specified IPv4 address, access protocol action, and user ID or impersonation role ID. You must provide either the user ID or impersonation ... |
| `GetDefaultRetentionPolicy` | `-` | `idempotent` | `OrganizationId` | - | `GetDefaultRetentionPolicyResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Gets the default retention policy details for the specified organization. |
| `GetImpersonationRole` | `-` | - | `OrganizationId`, `ImpersonationRoleId` | - | `GetImpersonationRoleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Gets the impersonation role details for the given WorkMail organization. |
| `GetImpersonationRoleEffect` | `-` | - | `OrganizationId`, `ImpersonationRoleId`, `TargetUser` | - | `GetImpersonationRoleEffectResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Tests whether the given impersonation role can impersonate a target user. |
| `GetMailboxDetails` | `-` | `idempotent` | `OrganizationId`, `UserId` | - | `GetMailboxDetailsResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Requests a user's mailbox details for a specified organization and user. |
| `GetMailDomain` | `-` | `idempotent` | `OrganizationId`, `DomainName` | - | `GetMailDomainResponse` | `InvalidParameterException`, `MailDomainNotFoundException`, `OrganizationNotFoundException`, `OrganizationStateException` | Gets details for a mail domain, including domain records required to configure your domain with recommended security. |
| `GetMobileDeviceAccessEffect` | `-` | - | `OrganizationId` | - | `GetMobileDeviceAccessEffectResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Simulates the effect of the mobile device access rules for the given attributes of a sample access event. Use this method to test the effects of the current set of mobile device access rules for the WorkMail organiza ... |
| `GetMobileDeviceAccessOverride` | `-` | - | `OrganizationId`, `UserId`, `DeviceId` | - | `GetMobileDeviceAccessOverrideResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Gets the mobile device access override for the given WorkMail organization, user, and device. |
| `GetPersonalAccessTokenMetadata` | `-` | `idempotent` | `OrganizationId`, `PersonalAccessTokenId` | - | `GetPersonalAccessTokenMetadataResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Requests details of a specific Personal Access Token within the WorkMail organization. |
| `ListAccessControlRules` | `-` | - | `OrganizationId` | - | `ListAccessControlRulesResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Lists the access control rules for the specified organization. |
| `ListAliases` | `-` | `idempotent`, `paginated` | `OrganizationId`, `EntityId` | - | `ListAliasesResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates a paginated call to list the aliases associated with a given entity. |
| `ListAvailabilityConfigurations` | `-` | `paginated` | `OrganizationId` | - | `ListAvailabilityConfigurationsResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | List all the AvailabilityConfiguration 's for the given WorkMail organization. |
| `ListGroupMembers` | `-` | `idempotent`, `paginated` | `OrganizationId`, `GroupId` | - | `ListGroupMembersResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns an overview of the members of a group. Users and groups can be members of a group. |
| `ListGroups` | `-` | `idempotent`, `paginated` | `OrganizationId` | - | `ListGroupsResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns summaries of the organization's groups. |
| `ListGroupsForEntity` | `-` | `idempotent`, `paginated` | `OrganizationId`, `EntityId` | - | `ListGroupsForEntityResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns all the groups to which an entity belongs. |
| `ListImpersonationRoles` | `-` | `paginated` | `OrganizationId` | - | `ListImpersonationRolesResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists all the impersonation roles for the given WorkMail organization. |
| `ListMailboxExportJobs` | `-` | `idempotent`, `paginated` | `OrganizationId` | - | `ListMailboxExportJobsResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mailbox export jobs started for the specified organization within the last seven days. |
| `ListMailboxPermissions` | `-` | `idempotent`, `paginated` | `OrganizationId`, `EntityId` | - | `ListMailboxPermissionsResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mailbox permissions associated with a user, group, or resource mailbox. |
| `ListMailDomains` | `-` | `idempotent`, `paginated` | `OrganizationId` | - | `ListMailDomainsResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mail domains in a given WorkMail organization. |
| `ListMobileDeviceAccessOverrides` | `-` | `paginated` | `OrganizationId` | - | `ListMobileDeviceAccessOverridesResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists all the mobile device access overrides for any given combination of WorkMail organization, user, or device. |
| `ListMobileDeviceAccessRules` | `-` | - | `OrganizationId` | - | `ListMobileDeviceAccessRulesResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mobile device access rules for the specified WorkMail organization. |
| `ListOrganizations` | `-` | `idempotent`, `paginated` | - | - | `ListOrganizationsResponse` | `InvalidParameterException` | Returns summaries of the customer's organizations. |
| `ListPersonalAccessTokens` | `-` | `idempotent`, `paginated` | `OrganizationId` | - | `ListPersonalAccessTokensResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns a summary of your Personal Access Tokens. |
| `ListResourceDelegates` | `-` | `idempotent`, `paginated` | `OrganizationId`, `ResourceId` | - | `ListResourceDelegatesResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource. |
| `ListResources` | `-` | `idempotent`, `paginated` | `OrganizationId` | - | `ListResourcesResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Returns summaries of the organization's resources. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags applied to an WorkMail organization resource. |
| `ListUsers` | `-` | `idempotent`, `paginated` | `OrganizationId` | - | `ListUsersResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns summaries of the organization's users. |
| `PutAccessControlRule` | `-` | - | `Name`, `Effect`, `Description`, `OrganizationId` | - | `PutAccessControlRuleResponse` | `EntityNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Adds a new access control rule for the specified organization. The rule allows or denies access to the organization for the specified IPv4 addresses, access protocol actions, user IDs and impersonation IDs. Adding a ... |
| `PutEmailMonitoringConfiguration` | `-` | `idempotent` | `OrganizationId`, `LogGroupArn` | - | `PutEmailMonitoringConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Creates or updates the email monitoring configuration for a specified organization. |
| `PutIdentityProviderConfiguration` | `-` | `idempotent` | `OrganizationId`, `AuthenticationMode`, `IdentityCenterConfiguration`, `PersonalAccessTokenConfiguration` | - | `PutIdentityProviderConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Enables integration between IAM Identity Center (IdC) and WorkMail to proxy authentication requests for mailbox users. You can connect your IdC directory or your external directory to WorkMail through IdC and manage ... |
| `PutInboundDmarcSettings` | `-` | `idempotent` | `OrganizationId`, `Enforced` | - | `PutInboundDmarcSettingsResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Enables or disables a DMARC policy for a given organization. |
| `PutMailboxPermissions` | `-` | `idempotent` | `OrganizationId`, `EntityId`, `GranteeId`, `PermissionValues` | - | `PutMailboxPermissionsResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Sets permissions for a user, group, or resource. This replaces any pre-existing permissions. |
| `PutMobileDeviceAccessOverride` | `-` | - | `OrganizationId`, `UserId`, `DeviceId`, `Effect` | - | `PutMobileDeviceAccessOverrideResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates or updates a mobile device access override for the given WorkMail organization, user, and device. |
| `PutRetentionPolicy` | `-` | `idempotent` | `OrganizationId`, `Name`, `FolderConfigurations` | - | `PutRetentionPolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Puts a retention policy to the specified organization. |
| `RegisterMailDomain` | `-` | `idempotent`, `idempotency-token` | `OrganizationId`, `DomainName` | `ClientToken` | `RegisterMailDomainResponse` | `InvalidParameterException`, `LimitExceededException`, `MailDomainInUseException`, `OrganizationNotFoundException`, `OrganizationStateException` | Registers a new domain in WorkMail and SES, and configures it for use by WorkMail. Emails received by SES for this domain are routed to the specified WorkMail organization, and WorkMail has permanent permission to us ... |
| `RegisterToWorkMail` | `-` | `idempotent` | `OrganizationId`, `EntityId`, `Email` | - | `RegisterToWorkMailResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EmailAddressInUseException`, `EntityAlreadyRegisteredException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Registers an existing and disabled user, group, or resource for WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the user, group, or resource is enabled and fails if the us ... |
| `ResetPassword` | `-` | `idempotent` | `OrganizationId`, `UserId`, `Password` | - | `ResetPasswordResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `InvalidPasswordException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Allows the administrator to reset the password for a user. |
| `StartMailboxExportJob` | `-` | `idempotent`, `idempotency-token` | `ClientToken`, `OrganizationId`, `EntityId`, `RoleArn`, `KmsKeyArn`, `S3BucketName`, `S3Prefix` | `ClientToken` | `StartMailboxExportJobResponse` | `EntityNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Starts a mailbox export job to export MIME-format email messages and calendar items from the specified mailbox to the specified Amazon Simple Storage Service (Amazon S3) bucket. For more information, see Exporting ma ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidParameterException`, `OrganizationStateException`, `ResourceNotFoundException`, `TooManyTagsException` | Applies the specified tags to the specified WorkMailorganization resource. |
| `TestAvailabilityConfiguration` | `-` | - | `OrganizationId` | - | `TestAvailabilityConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Performs a test on an availability provider to ensure that access is allowed. For EWS, it verifies the provided credentials can be used to successfully log in. For Lambda, it verifies that the Lambda function can be ... |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Untags the specified tags from the specified WorkMail organization resource. |
| `UpdateAvailabilityConfiguration` | `-` | `idempotent` | `OrganizationId`, `DomainName` | - | `UpdateAvailabilityConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Updates an existing AvailabilityConfiguration for the given WorkMail organization and domain. |
| `UpdateDefaultMailDomain` | `-` | `idempotent` | `OrganizationId`, `DomainName` | - | `UpdateDefaultMailDomainResponse` | `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, `OrganizationNotFoundException`, `OrganizationStateException` | Updates the default mail domain for an organization. The default mail domain is used by the WorkMail AWS Console to suggest an email address when enabling a mail user. You can only have one default domain. |
| `UpdateGroup` | `-` | `idempotent` | `OrganizationId`, `GroupId` | - | `UpdateGroupResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Updates attributes in a group. |
| `UpdateImpersonationRole` | `-` | - | `OrganizationId`, `ImpersonationRoleId`, `Name`, `Type`, `Rules` | - | `UpdateImpersonationRoleResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Updates an impersonation role for the given WorkMail organization. |
| `UpdateMailboxQuota` | `-` | `idempotent` | `OrganizationId`, `UserId`, `MailboxQuota` | - | `UpdateMailboxQuotaResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Updates a user's current mailbox quota for a specified organization and user. |
| `UpdateMobileDeviceAccessRule` | `-` | - | `OrganizationId`, `MobileDeviceAccessRuleId`, `Name`, `Effect` | - | `UpdateMobileDeviceAccessRuleResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Updates a mobile device access rule for the specified WorkMail organization. |
| `UpdatePrimaryEmailAddress` | `-` | `idempotent` | `OrganizationId`, `EntityId`, `Email` | - | `UpdatePrimaryEmailAddressResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EmailAddressInUseException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Updates the primary email for a user, group, or resource. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email), and the email provided in the input ... |
| `UpdateResource` | `-` | `idempotent` | `OrganizationId`, `ResourceId` | - | `UpdateResourceResponse` | `DirectoryUnavailableException`, `EmailAddressInUseException`, `EntityNotFoundException`, `EntityStateException`, `InvalidConfigurationException`, `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Updates data for the resource. To have the latest information, it must be preceded by a DescribeResource call. The dataset in the request should be the one expected when performing another DescribeResource call. |
| `UpdateUser` | `-` | `idempotent` | `OrganizationId`, `UserId` | - | `UpdateUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Updates data for the user. To have the latest information, it must be preceded by a DescribeUser call. The dataset in the request should be the one expected when performing another DescribeUser call. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DirectoryInUseException` | `structure` | Message | The directory is already in use by another WorkMail organization in the same account and Region. |
| `DirectoryServiceAuthenticationFailedException` | `structure` | Message | The directory service doesn't recognize the credentials supplied by WorkMail. |
| `DirectoryUnavailableException` | `structure` | Message | The directory is unavailable. It might be located in another Region or deleted. |
| `EmailAddressInUseException` | `structure` | Message | The email address that you're trying to assign is already created for a different user, group, or resource. |
| `EntityAlreadyRegisteredException` | `structure` | Message | The user, group, or resource that you're trying to register is already registered. |
| `EntityNotFoundException` | `structure` | Message | The identifier supplied for the user, group, or resource does not exist in your organization. |
| `EntityStateException` | `structure` | Message | You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user. |
| `InvalidConfigurationException` | `structure` | Message | The configuration for a resource isn't valid. A resource must either be able to auto-respond to requests or have at least one delegate associated that can d ... |
| `InvalidCustomSesConfigurationException` | `structure` | Message | You SES configuration has customizations that WorkMail cannot save. The error message lists the invalid setting. For examples of invalid settings, refer to ... |
| `InvalidParameterException` | `structure` | Message | One or more of the input parameters don't match the service's restrictions. |
| `InvalidPasswordException` | `structure` | Message | The supplied password doesn't match the minimum security constraints, such as length or use of special characters. |
| `LimitExceededException` | `structure` | Message | The request exceeds the limit of the resource. |
| `MailDomainInUseException` | `structure` | Message | The domain you're trying to change is in use by another user or organization in your account. See the error message for details. |
| `MailDomainNotFoundException` | `structure` | Message | The domain specified is not found in your organization. |
| `MailDomainStateException` | `structure` | Message | After a domain has been added to the organization, it must be verified. The domain is not yet verified. |
| `NameAvailabilityException` | `structure` | Message | The user, group, or resource name isn't unique in WorkMail. |
| `OrganizationNotFoundException` | `structure` | Message | An operation received a valid organization identifier that either doesn't belong or exist in the system. |
| `OrganizationStateException` | `structure` | Message | The organization must have a valid state to perform certain operations on the organization or its members. |
| `ReservedNameException` | `structure` | Message | This user, group, or resource name is not allowed in WorkMail. |
| `ResourceNotFoundException` | `structure` | Message | The resource cannot be found. |
| `TooManyTagsException` | `structure` | Message | The resource can have up to 50 user-applied tags. |
| `UnsupportedOperationException` | `structure` | Message | You can't perform a write operation against a read-only directory. |
| `AssociateDelegateToResourceRequest` | `structure` | OrganizationId, ResourceId, EntityId | - |
| `AssociateDelegateToResourceResponse` | `structure` | **empty (no members)** | - |
| `AssociateMemberToGroupRequest` | `structure` | OrganizationId, GroupId, MemberId | - |
| `AssociateMemberToGroupResponse` | `structure` | **empty (no members)** | - |
| `AssumeImpersonationRoleRequest` | `structure` | OrganizationId, ImpersonationRoleId | - |
| `AssumeImpersonationRoleResponse` | `structure` | Token, ExpiresIn | - |
| `CancelMailboxExportJobRequest` | `structure` | ClientToken, JobId, OrganizationId | - |
| `CancelMailboxExportJobResponse` | `structure` | **empty (no members)** | - |
| `CreateAliasRequest` | `structure` | OrganizationId, EntityId, Alias | - |
| `CreateAliasResponse` | `structure` | **empty (no members)** | - |
| `CreateAvailabilityConfigurationRequest` | `structure` | ClientToken, OrganizationId, DomainName, EwsProvider, LambdaProvider | - |
| `CreateAvailabilityConfigurationResponse` | `structure` | **empty (no members)** | - |
| `CreateGroupRequest` | `structure` | OrganizationId, Name, HiddenFromGlobalAddressList | - |
| `CreateGroupResponse` | `structure` | GroupId | - |
| `CreateIdentityCenterApplicationRequest` | `structure` | Name, InstanceArn, ClientToken | - |
| `CreateIdentityCenterApplicationResponse` | `structure` | ApplicationArn | - |
| `CreateImpersonationRoleRequest` | `structure` | ClientToken, OrganizationId, Name, Type, Description, Rules | - |
| `CreateImpersonationRoleResponse` | `structure` | ImpersonationRoleId | - |
| `AccessControlRuleEffect` | `enum` | ALLOW, DENY | - |
| `AccessEffect` | `enum` | ALLOW, DENY | - |
| `AvailabilityProviderType` | `enum` | EWS, LAMBDA | - |
| `DnsRecordVerificationStatus` | `enum` | PENDING, VERIFIED, FAILED | - |
| `EntityState` | `enum` | ENABLED, DISABLED, DELETED | - |
| `EntityType` | `enum` | GROUP, USER, RESOURCE | - |
| `FolderName` | `enum` | INBOX, DELETED_ITEMS, SENT_ITEMS, DRAFTS, JUNK_EMAIL | - |
| `IdentityProviderAuthenticationMode` | `enum` | IDENTITY_PROVIDER_ONLY, IDENTITY_PROVIDER_AND_DIRECTORY | - |
| `ImpersonationRoleType` | `enum` | FULL_ACCESS, READ_ONLY | - |
| `MailboxExportJobState` | `enum` | RUNNING, COMPLETED, FAILED, CANCELLED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

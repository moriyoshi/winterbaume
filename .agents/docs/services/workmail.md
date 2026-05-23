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

- Operations: `ListAccessControlRules`, `ListAliases`, `ListAvailabilityConfigurations`, `ListGroupMembers`, `ListGroups`, `ListGroupsForEntity`, `ListImpersonationRoles`, `ListMailDomains`, `ListMailboxExportJobs`, `ListMailboxPermissions`, `ListMobileDeviceAccessOverrides`, `ListMobileDeviceAccessRules`, `ListOrganizations`, `ListPersonalAccessTokens`, `ListResourceDelegates`, `ListResources`, `ListTagsForResource`, `ListUsers`
- Traits: `idempotent` (12), `paginated` (15)
- Common required input members in this group: `EntityId`, `GroupId`, `OrganizationId`, `ResourceARN`, `ResourceId`

### Delete

- Operations: `DeleteAccessControlRule`, `DeleteAlias`, `DeleteAvailabilityConfiguration`, `DeleteEmailMonitoringConfiguration`, `DeleteGroup`, `DeleteIdentityCenterApplication`, `DeleteIdentityProviderConfiguration`, `DeleteImpersonationRole`, `DeleteMailboxPermissions`, `DeleteMobileDeviceAccessOverride`, `DeleteMobileDeviceAccessRule`, `DeleteOrganization`, `DeletePersonalAccessToken`, `DeleteResource`, `DeleteRetentionPolicy`, `DeleteUser`
- Traits: `idempotency-token` (1), `idempotent` (12)
- Common required input members in this group: `Alias`, `ApplicationArn`, `DeleteDirectory`, `DeviceId`, `DomainName`, `EntityId`, `GranteeId`, `GroupId`, `Id`, `ImpersonationRoleId`, `MobileDeviceAccessRuleId`, `Name`, `OrganizationId`, `PersonalAccessTokenId`, `ResourceId`, `UserId`

### Create

- Operations: `CreateAlias`, `CreateAvailabilityConfiguration`, `CreateGroup`, `CreateIdentityCenterApplication`, `CreateImpersonationRole`, `CreateMobileDeviceAccessRule`, `CreateOrganization`, `CreateResource`, `CreateUser`
- Traits: `idempotency-token` (5), `idempotent` (7)
- Common required input members in this group: `Alias`, `DisplayName`, `DomainName`, `Effect`, `EntityId`, `InstanceArn`, `Name`, `OrganizationId`, `Rules`, `Type`

### Describe

- Operations: `DescribeEmailMonitoringConfiguration`, `DescribeEntity`, `DescribeGroup`, `DescribeIdentityProviderConfiguration`, `DescribeInboundDmarcSettings`, `DescribeMailboxExportJob`, `DescribeOrganization`, `DescribeResource`, `DescribeUser`
- Traits: `idempotent` (9)
- Common required input members in this group: `Email`, `GroupId`, `JobId`, `OrganizationId`, `ResourceId`, `UserId`

### Get

- Operations: `GetAccessControlEffect`, `GetDefaultRetentionPolicy`, `GetImpersonationRole`, `GetImpersonationRoleEffect`, `GetMailDomain`, `GetMailboxDetails`, `GetMobileDeviceAccessEffect`, `GetMobileDeviceAccessOverride`, `GetPersonalAccessTokenMetadata`
- Traits: `idempotent` (4)
- Common required input members in this group: `Action`, `DeviceId`, `DomainName`, `ImpersonationRoleId`, `IpAddress`, `OrganizationId`, `PersonalAccessTokenId`, `TargetUser`, `UserId`

### Update

- Operations: `UpdateAvailabilityConfiguration`, `UpdateDefaultMailDomain`, `UpdateGroup`, `UpdateImpersonationRole`, `UpdateMailboxQuota`, `UpdateMobileDeviceAccessRule`, `UpdatePrimaryEmailAddress`, `UpdateResource`, `UpdateUser`
- Traits: `idempotent` (7)
- Common required input members in this group: `DomainName`, `Effect`, `Email`, `EntityId`, `GroupId`, `ImpersonationRoleId`, `MailboxQuota`, `MobileDeviceAccessRuleId`, `Name`, `OrganizationId`, `ResourceId`, `Rules`, `Type`, `UserId`

### Put

- Operations: `PutAccessControlRule`, `PutEmailMonitoringConfiguration`, `PutIdentityProviderConfiguration`, `PutInboundDmarcSettings`, `PutMailboxPermissions`, `PutMobileDeviceAccessOverride`, `PutRetentionPolicy`
- Traits: `idempotent` (5)
- Common required input members in this group: `AuthenticationMode`, `Description`, `DeviceId`, `Effect`, `Enforced`, `EntityId`, `FolderConfigurations`, `GranteeId`, `IdentityCenterConfiguration`, `LogGroupArn`, `Name`, `OrganizationId`, `PermissionValues`, `PersonalAccessTokenConfiguration`, `UserId`

### Associate

- Operations: `AssociateDelegateToResource`, `AssociateMemberToGroup`
- Traits: `idempotent` (2)
- Common required input members in this group: `EntityId`, `GroupId`, `MemberId`, `OrganizationId`, `ResourceId`

### Deregister

- Operations: `DeregisterFromWorkMail`, `DeregisterMailDomain`
- Traits: `idempotent` (2)
- Common required input members in this group: `DomainName`, `EntityId`, `OrganizationId`

### Disassociate

- Operations: `DisassociateDelegateFromResource`, `DisassociateMemberFromGroup`
- Traits: `idempotent` (2)
- Common required input members in this group: `EntityId`, `GroupId`, `MemberId`, `OrganizationId`, `ResourceId`

### Register

- Operations: `RegisterMailDomain`, `RegisterToWorkMail`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `DomainName`, `Email`, `EntityId`, `OrganizationId`

### Assume

- Operations: `AssumeImpersonationRole`
- Common required input members in this group: `ImpersonationRoleId`, `OrganizationId`

### Cancel

- Operations: `CancelMailboxExportJob`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `ClientToken`, `JobId`, `OrganizationId`

### Reset

- Operations: `ResetPassword`
- Traits: `idempotent` (1)
- Common required input members in this group: `OrganizationId`, `Password`, `UserId`

### Start

- Operations: `StartMailboxExportJob`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `ClientToken`, `EntityId`, `KmsKeyArn`, `OrganizationId`, `RoleArn`, `S3BucketName`, `S3Prefix`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Test

- Operations: `TestAvailabilityConfiguration`
- Common required input members in this group: `OrganizationId`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateDelegateToResource` | - | `idempotent` | `EntityId`, `OrganizationId`, `ResourceId` | - | `AssociateDelegateToResourceResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Adds a member (user or group) to the resource's set of delegates. |
| `AssociateMemberToGroup` | - | `idempotent` | `GroupId`, `MemberId`, `OrganizationId` | - | `AssociateMemberToGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Adds a member (user or group) to the group's set. |
| `AssumeImpersonationRole` | - | - | `ImpersonationRoleId`, `OrganizationId` | - | `AssumeImpersonationRoleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Assumes an impersonation role for the given WorkMail organization. This method returns an authentication token you can use to make impersonated calls. |
| `CancelMailboxExportJob` | - | `idempotent`, `idempotency-token` | `ClientToken`, `JobId`, `OrganizationId` | `ClientToken` | `CancelMailboxExportJobResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Cancels a mailbox export job. If the mailbox export job is near completion, it might not be possible to cancel it. |
| `CreateAlias` | - | `idempotent` | `Alias`, `EntityId`, `OrganizationId` | - | `CreateAliasResponse` | `EmailAddressInUseException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `LimitExceededException`, `MailDomainNotFoundException`, `MailDomainStateException`, `OrganizationNotFoundException`, ... (+1) | Adds an alias to the set of a given member (user or group) of WorkMail. |
| `CreateAvailabilityConfiguration` | - | `idempotent`, `idempotency-token` | `DomainName`, `OrganizationId` | `ClientToken` | `CreateAvailabilityConfigurationResponse` | `InvalidParameterException`, `LimitExceededException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates an `AvailabilityConfiguration` for the given WorkMail organization and domain. |
| `CreateGroup` | - | `idempotent` | `Name`, `OrganizationId` | - | `CreateGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `InvalidParameterException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ReservedNameException`, `UnsupportedOperationException` | Creates a group that can be used in WorkMail by calling the RegisterToWorkMail operation. |
| `CreateIdentityCenterApplication` | - | `idempotent`, `idempotency-token` | `InstanceArn`, `Name` | `ClientToken` | `CreateIdentityCenterApplicationResponse` | `InvalidParameterException` | Creates the WorkMail application in IAM Identity Center that can be used later in the WorkMail - IdC integration. For more information, see PutIdentityProviderConfiguration. |
| `CreateImpersonationRole` | - | `idempotency-token` | `Name`, `OrganizationId`, `Rules`, `Type` | `ClientToken` | `CreateImpersonationRoleResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates an impersonation role for the given WorkMail organization. Idempotency ensures that an API request completes no more than one time. |
| `CreateMobileDeviceAccessRule` | - | `idempotency-token` | `Effect`, `Name`, `OrganizationId` | `ClientToken` | `CreateMobileDeviceAccessRuleResponse` | `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates a new mobile device access rule for the specified WorkMail organization. |
| `CreateOrganization` | - | `idempotent`, `idempotency-token` | `Alias` | `ClientToken` | `CreateOrganizationResponse` | `DirectoryInUseException`, `DirectoryUnavailableException`, `InvalidParameterException`, `LimitExceededException`, `NameAvailabilityException` | Creates a new WorkMail organization. Optionally, you can choose to associate an existing AWS Directory Service directory with your organization. |
| `CreateResource` | - | `idempotent` | `Name`, `OrganizationId`, `Type` | - | `CreateResourceResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `InvalidParameterException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ReservedNameException`, `UnsupportedOperationException` | Creates a new WorkMail resource. |
| `CreateUser` | - | `idempotent` | `DisplayName`, `Name`, `OrganizationId` | - | `CreateUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `InvalidParameterException`, `InvalidPasswordException`, `NameAvailabilityException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ReservedNameException`, ... (+1) | Creates a user who can be used in WorkMail by calling the RegisterToWorkMail operation. |
| `DeleteAccessControlRule` | - | - | `Name`, `OrganizationId` | - | `DeleteAccessControlRuleResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Deletes an access control rule for the specified WorkMail organization. Deleting already deleted and non-existing rules does not produce an error. |
| `DeleteAlias` | - | `idempotent` | `Alias`, `EntityId`, `OrganizationId` | - | `DeleteAliasResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Remove one or more specified aliases from a set of aliases for a given user. |
| `DeleteAvailabilityConfiguration` | - | `idempotent` | `DomainName`, `OrganizationId` | - | `DeleteAvailabilityConfigurationResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the `AvailabilityConfiguration` for the given WorkMail organization and domain. |
| `DeleteEmailMonitoringConfiguration` | - | `idempotent` | `OrganizationId` | - | `DeleteEmailMonitoringConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the email monitoring configuration for a specified organization. |
| `DeleteGroup` | - | `idempotent` | `GroupId`, `OrganizationId` | - | `DeleteGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Deletes a group from WorkMail. |
| `DeleteIdentityCenterApplication` | - | `idempotent` | `ApplicationArn` | - | `DeleteIdentityCenterApplicationResponse` | `InvalidParameterException`, `OrganizationStateException` | Deletes the IAM Identity Center application from WorkMail. This action does not affect the authentication settings for any WorkMail organizations. |
| `DeleteIdentityProviderConfiguration` | - | `idempotent` | `OrganizationId` | - | `DeleteIdentityProviderConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Disables the integration between IdC and WorkMail. Authentication will continue with the directory as it was before the IdC integration. |
| `DeleteImpersonationRole` | - | - | `ImpersonationRoleId`, `OrganizationId` | - | `DeleteImpersonationRoleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes an impersonation role for the given WorkMail organization. |
| `DeleteMailboxPermissions` | - | `idempotent` | `EntityId`, `GranteeId`, `OrganizationId` | - | `DeleteMailboxPermissionsResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes permissions granted to a member (user or group). |
| `DeleteMobileDeviceAccessOverride` | - | - | `DeviceId`, `OrganizationId`, `UserId` | - | `DeleteMobileDeviceAccessOverrideResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the mobile device access override for the given WorkMail organization, user, and device. Deleting already deleted and non-existing overrides does not produce an error. |
| `DeleteMobileDeviceAccessRule` | - | - | `MobileDeviceAccessRuleId`, `OrganizationId` | - | `DeleteMobileDeviceAccessRuleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes a mobile device access rule for the specified WorkMail organization. Deleting already deleted and non-existing rules does not produce an error. |
| `DeleteOrganization` | - | `idempotent`, `idempotency-token` | `DeleteDirectory`, `OrganizationId` | `ClientToken` | `DeleteOrganizationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes an WorkMail organization and all underlying AWS resources managed by WorkMail as part of the organization. You can choose whether to delete the associated directory. |
| `DeletePersonalAccessToken` | - | `idempotent` | `OrganizationId`, `PersonalAccessTokenId` | - | `DeletePersonalAccessTokenResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the Personal Access Token from the provided WorkMail Organization. |
| `DeleteResource` | - | `idempotent` | `OrganizationId`, `ResourceId` | - | `DeleteResourceResponse` | `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Deletes the specified resource. |
| `DeleteRetentionPolicy` | - | `idempotent` | `Id`, `OrganizationId` | - | `DeleteRetentionPolicyResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Deletes the specified retention policy from the specified organization. |
| `DeleteUser` | - | `idempotent` | `OrganizationId`, `UserId` | - | `DeleteUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Deletes a user from WorkMail and all subsequent systems. Before you can delete a user, the user state must be `DISABLED`. |
| `DeregisterFromWorkMail` | - | `idempotent` | `EntityId`, `OrganizationId` | - | `DeregisterFromWorkMailResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Mark a user, group, or resource as no longer used in WorkMail. This action disassociates the mailbox and schedules it for clean-up. |
| `DeregisterMailDomain` | - | `idempotent` | `DomainName`, `OrganizationId` | - | `DeregisterMailDomainResponse` | `InvalidCustomSesConfigurationException`, `InvalidParameterException`, `MailDomainInUseException`, `OrganizationNotFoundException`, `OrganizationStateException` | Removes a domain from WorkMail, stops email routing to WorkMail, and removes the authorization allowing WorkMail use. SES keeps the domain because other applications may use it. |
| `DescribeEmailMonitoringConfiguration` | - | `idempotent` | `OrganizationId` | - | `DescribeEmailMonitoringConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Describes the current email monitoring configuration for a specified organization. |
| `DescribeEntity` | - | `idempotent` | `Email`, `OrganizationId` | - | `DescribeEntityResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns basic details about an entity in WorkMail. |
| `DescribeGroup` | - | `idempotent` | `GroupId`, `OrganizationId` | - | `DescribeGroupResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns the data available for the group. |
| `DescribeIdentityProviderConfiguration` | - | `idempotent` | `OrganizationId` | - | `DescribeIdentityProviderConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Returns detailed information on the current IdC setup for the WorkMail organization. |
| `DescribeInboundDmarcSettings` | - | `idempotent` | `OrganizationId` | - | `DescribeInboundDmarcSettingsResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Lists the settings in a DMARC policy for a specified organization. |
| `DescribeMailboxExportJob` | - | `idempotent` | `JobId`, `OrganizationId` | - | `DescribeMailboxExportJobResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Describes the current status of a mailbox export job. |
| `DescribeOrganization` | - | `idempotent` | `OrganizationId` | - | `DescribeOrganizationResponse` | `InvalidParameterException`, `OrganizationNotFoundException` | Provides more information regarding a given organization based on its identifier. |
| `DescribeResource` | - | `idempotent` | `OrganizationId`, `ResourceId` | - | `DescribeResourceResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Returns the data available for the resource. |
| `DescribeUser` | - | `idempotent` | `OrganizationId`, `UserId` | - | `DescribeUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Provides information regarding the user. |
| `DisassociateDelegateFromResource` | - | `idempotent` | `EntityId`, `OrganizationId`, `ResourceId` | - | `DisassociateDelegateFromResourceResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Removes a member from the resource's set of delegates. |
| `DisassociateMemberFromGroup` | - | `idempotent` | `GroupId`, `MemberId`, `OrganizationId` | - | `DisassociateMemberFromGroupResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Removes a member from a group. |
| `GetAccessControlEffect` | - | - | `Action`, `IpAddress`, `OrganizationId` | - | `GetAccessControlEffectResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Gets the effects of an organization's access control rules as they apply to a specified IPv4 address, access protocol action, and user ID or impersonation role ID. You must provide either the user ID or impersonation role ID. |
| `GetDefaultRetentionPolicy` | - | `idempotent` | `OrganizationId` | - | `GetDefaultRetentionPolicyResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Gets the default retention policy details for the specified organization. |
| `GetImpersonationRole` | - | - | `ImpersonationRoleId`, `OrganizationId` | - | `GetImpersonationRoleResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Gets the impersonation role details for the given WorkMail organization. |
| `GetImpersonationRoleEffect` | - | - | `ImpersonationRoleId`, `OrganizationId`, `TargetUser` | - | `GetImpersonationRoleEffectResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Tests whether the given impersonation role can impersonate a target user. |
| `GetMailDomain` | - | `idempotent` | `DomainName`, `OrganizationId` | - | `GetMailDomainResponse` | `InvalidParameterException`, `MailDomainNotFoundException`, `OrganizationNotFoundException`, `OrganizationStateException` | Gets details for a mail domain, including domain records required to configure your domain with recommended security. |
| `GetMailboxDetails` | - | `idempotent` | `OrganizationId`, `UserId` | - | `GetMailboxDetailsResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Requests a user's mailbox details for a specified organization and user. |
| `GetMobileDeviceAccessEffect` | - | - | `OrganizationId` | - | `GetMobileDeviceAccessEffectResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Simulates the effect of the mobile device access rules for the given attributes of a sample access event. Use this method to test the effects of the current set of mobile device access rules for the WorkMail organization for a particular user's attributes. |
| `GetMobileDeviceAccessOverride` | - | - | `DeviceId`, `OrganizationId`, `UserId` | - | `GetMobileDeviceAccessOverrideResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Gets the mobile device access override for the given WorkMail organization, user, and device. |
| `GetPersonalAccessTokenMetadata` | - | `idempotent` | `OrganizationId`, `PersonalAccessTokenId` | - | `GetPersonalAccessTokenMetadataResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Requests details of a specific Personal Access Token within the WorkMail organization. |
| `ListAccessControlRules` | - | - | `OrganizationId` | - | `ListAccessControlRulesResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Lists the access control rules for the specified organization. |
| `ListAliases` | - | `idempotent`, `paginated` | `EntityId`, `OrganizationId` | - | `ListAliasesResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates a paginated call to list the aliases associated with a given entity. |
| `ListAvailabilityConfigurations` | - | `paginated` | `OrganizationId` | - | `ListAvailabilityConfigurationsResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | List all the `AvailabilityConfiguration`'s for the given WorkMail organization. |
| `ListGroupMembers` | - | `idempotent`, `paginated` | `GroupId`, `OrganizationId` | - | `ListGroupMembersResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns an overview of the members of a group. Users and groups can be members of a group. |
| `ListGroups` | - | `idempotent`, `paginated` | `OrganizationId` | - | `ListGroupsResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns summaries of the organization's groups. |
| `ListGroupsForEntity` | - | `idempotent`, `paginated` | `EntityId`, `OrganizationId` | - | `ListGroupsForEntityResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns all the groups to which an entity belongs. |
| `ListImpersonationRoles` | - | `paginated` | `OrganizationId` | - | `ListImpersonationRolesResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists all the impersonation roles for the given WorkMail organization. |
| `ListMailDomains` | - | `idempotent`, `paginated` | `OrganizationId` | - | `ListMailDomainsResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mail domains in a given WorkMail organization. |
| `ListMailboxExportJobs` | - | `idempotent`, `paginated` | `OrganizationId` | - | `ListMailboxExportJobsResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mailbox export jobs started for the specified organization within the last seven days. |
| `ListMailboxPermissions` | - | `idempotent`, `paginated` | `EntityId`, `OrganizationId` | - | `ListMailboxPermissionsResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mailbox permissions associated with a user, group, or resource mailbox. |
| `ListMobileDeviceAccessOverrides` | - | `paginated` | `OrganizationId` | - | `ListMobileDeviceAccessOverridesResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists all the mobile device access overrides for any given combination of WorkMail organization, user, or device. |
| `ListMobileDeviceAccessRules` | - | - | `OrganizationId` | - | `ListMobileDeviceAccessRulesResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Lists the mobile device access rules for the specified WorkMail organization. |
| `ListOrganizations` | - | `idempotent`, `paginated` | - | - | `ListOrganizationsResponse` | `InvalidParameterException` | Returns summaries of the customer's organizations. |
| `ListPersonalAccessTokens` | - | `idempotent`, `paginated` | `OrganizationId` | - | `ListPersonalAccessTokensResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns a summary of your Personal Access Tokens. |
| `ListResourceDelegates` | - | `idempotent`, `paginated` | `OrganizationId`, `ResourceId` | - | `ListResourceDelegatesResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Lists the delegates associated with a resource. Users and groups can be resource delegates and answer requests on behalf of the resource. |
| `ListResources` | - | `idempotent`, `paginated` | `OrganizationId` | - | `ListResourcesResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Returns summaries of the organization's resources. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags applied to an WorkMail organization resource. |
| `ListUsers` | - | `idempotent`, `paginated` | `OrganizationId` | - | `ListUsersResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Returns summaries of the organization's users. |
| `PutAccessControlRule` | - | - | `Description`, `Effect`, `Name`, `OrganizationId` | - | `PutAccessControlRuleResponse` | `EntityNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Adds a new access control rule for the specified organization. The rule allows or denies access to the organization for the specified IPv4 addresses, access protocol actions, user IDs and impersonation IDs. |
| `PutEmailMonitoringConfiguration` | - | `idempotent` | `LogGroupArn`, `OrganizationId` | - | `PutEmailMonitoringConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Creates or updates the email monitoring configuration for a specified organization. |
| `PutIdentityProviderConfiguration` | - | `idempotent` | `AuthenticationMode`, `IdentityCenterConfiguration`, `OrganizationId`, `PersonalAccessTokenConfiguration` | - | `PutIdentityProviderConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Enables integration between IAM Identity Center (IdC) and WorkMail to proxy authentication requests for mailbox users. You can connect your IdC directory or your external directory to WorkMail through IdC and manage access to WorkMail mailboxes in a single... |
| `PutInboundDmarcSettings` | - | `idempotent` | `Enforced`, `OrganizationId` | - | `PutInboundDmarcSettingsResponse` | `OrganizationNotFoundException`, `OrganizationStateException` | Enables or disables a DMARC policy for a given organization. |
| `PutMailboxPermissions` | - | `idempotent` | `EntityId`, `GranteeId`, `OrganizationId`, `PermissionValues` | - | `PutMailboxPermissionsResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Sets permissions for a user, group, or resource. This replaces any pre-existing permissions. |
| `PutMobileDeviceAccessOverride` | - | - | `DeviceId`, `Effect`, `OrganizationId`, `UserId` | - | `PutMobileDeviceAccessOverrideResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Creates or updates a mobile device access override for the given WorkMail organization, user, and device. |
| `PutRetentionPolicy` | - | `idempotent` | `FolderConfigurations`, `Name`, `OrganizationId` | - | `PutRetentionPolicyResponse` | `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Puts a retention policy to the specified organization. |
| `RegisterMailDomain` | - | `idempotent`, `idempotency-token` | `DomainName`, `OrganizationId` | `ClientToken` | `RegisterMailDomainResponse` | `InvalidParameterException`, `LimitExceededException`, `MailDomainInUseException`, `OrganizationNotFoundException`, `OrganizationStateException` | Registers a new domain in WorkMail and SES, and configures it for use by WorkMail. Emails received by SES for this domain are routed to the specified WorkMail organization, and WorkMail has permanent permission to use the specified domain for sending your... |
| `RegisterToWorkMail` | - | `idempotent` | `Email`, `EntityId`, `OrganizationId` | - | `RegisterToWorkMailResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EmailAddressInUseException`, `EntityAlreadyRegisteredException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `MailDomainNotFoundException`, ... (+4) | Registers an existing and disabled user, group, or resource for WorkMail use by associating a mailbox and calendaring capabilities. It performs no change if the user, group, or resource is enabled and fails if the user, group, or resource is deleted. |
| `ResetPassword` | - | `idempotent` | `OrganizationId`, `Password`, `UserId` | - | `ResetPasswordResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `InvalidPasswordException`, `OrganizationNotFoundException`, `OrganizationStateException`, ... (+1) | Allows the administrator to reset the password for a user. |
| `StartMailboxExportJob` | - | `idempotent`, `idempotency-token` | `ClientToken`, `EntityId`, `KmsKeyArn`, `OrganizationId`, `RoleArn`, `S3BucketName`, `S3Prefix` | `ClientToken` | `StartMailboxExportJobResponse` | `EntityNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException` | Starts a mailbox export job to export MIME-format email messages and calendar items from the specified mailbox to the specified Amazon Simple Storage Service (Amazon S3) bucket. For more information, see Exporting mailbox content in the WorkMail Administrator... |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidParameterException`, `OrganizationStateException`, `ResourceNotFoundException`, `TooManyTagsException` | Applies the specified tags to the specified WorkMailorganization resource. |
| `TestAvailabilityConfiguration` | - | - | `OrganizationId` | - | `TestAvailabilityConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Performs a test on an availability provider to ensure that access is allowed. For EWS, it verifies the provided credentials can be used to successfully log in. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Untags the specified tags from the specified WorkMail organization resource. |
| `UpdateAvailabilityConfiguration` | - | `idempotent` | `DomainName`, `OrganizationId` | - | `UpdateAvailabilityConfigurationResponse` | `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Updates an existing `AvailabilityConfiguration` for the given WorkMail organization and domain. |
| `UpdateDefaultMailDomain` | - | `idempotent` | `DomainName`, `OrganizationId` | - | `UpdateDefaultMailDomainResponse` | `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, `OrganizationNotFoundException`, `OrganizationStateException` | Updates the default mail domain for an organization. The default mail domain is used by the WorkMail AWS Console to suggest an email address when enabling a mail user. |
| `UpdateGroup` | - | `idempotent` | `GroupId`, `OrganizationId` | - | `UpdateGroupResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Updates attributes in a group. |
| `UpdateImpersonationRole` | - | - | `ImpersonationRoleId`, `Name`, `OrganizationId`, `Rules`, `Type` | - | `UpdateImpersonationRoleResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `LimitExceededException`, `OrganizationNotFoundException`, `OrganizationStateException`, `ResourceNotFoundException` | Updates an impersonation role for the given WorkMail organization. |
| `UpdateMailboxQuota` | - | `idempotent` | `MailboxQuota`, `OrganizationId`, `UserId` | - | `UpdateMailboxQuotaResponse` | `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Updates a user's current mailbox quota for a specified organization and user. |
| `UpdateMobileDeviceAccessRule` | - | - | `Effect`, `MobileDeviceAccessRuleId`, `Name`, `OrganizationId` | - | `UpdateMobileDeviceAccessRuleResponse` | `EntityNotFoundException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException` | Updates a mobile device access rule for the specified WorkMail organization. |
| `UpdatePrimaryEmailAddress` | - | `idempotent` | `Email`, `EntityId`, `OrganizationId` | - | `UpdatePrimaryEmailAddressResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EmailAddressInUseException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, ... (+3) | Updates the primary email for a user, group, or resource. The current email is moved into the list of aliases (or swapped between an existing alias and the current primary email), and the email provided in the input is promoted as the primary. |
| `UpdateResource` | - | `idempotent` | `OrganizationId`, `ResourceId` | - | `UpdateResourceResponse` | `DirectoryUnavailableException`, `EmailAddressInUseException`, `EntityNotFoundException`, `EntityStateException`, `InvalidConfigurationException`, `InvalidParameterException`, `MailDomainNotFoundException`, `MailDomainStateException`, ... (+4) | Updates data for the resource. To have the latest information, it must be preceded by a DescribeResource call. |
| `UpdateUser` | - | `idempotent` | `OrganizationId`, `UserId` | - | `UpdateUserResponse` | `DirectoryServiceAuthenticationFailedException`, `DirectoryUnavailableException`, `EntityNotFoundException`, `EntityStateException`, `InvalidParameterException`, `OrganizationNotFoundException`, `OrganizationStateException`, `UnsupportedOperationException` | Updates data for the user. To have the latest information, it must be preceded by a DescribeUser call. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `OrganizationStateException` | `structure` | `Message` | The organization must have a valid state to perform certain operations on the organization or its members. |
| `InvalidParameterException` | `structure` | `Message` | One or more of the input parameters don't match the service's restrictions. |
| `OrganizationNotFoundException` | `structure` | `Message` | An operation received a valid organization identifier that either doesn't belong or exist in the system. |
| `EntityNotFoundException` | `structure` | `Message` | The identifier supplied for the user, group, or resource does not exist in your organization. |
| `EntityStateException` | `structure` | `Message` | You are performing an operation on a user, group, or resource that isn't in the expected state, such as trying to delete an active user. |
| `UnsupportedOperationException` | `structure` | `Message` | You can't perform a write operation against a read-only directory. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource cannot be found. |
| `DirectoryUnavailableException` | `structure` | `Message` | The directory is unavailable. |
| `DirectoryServiceAuthenticationFailedException` | `structure` | `Message` | The directory service doesn't recognize the credentials supplied by WorkMail. |
| `LimitExceededException` | `structure` | `Message` | The request exceeds the limit of the resource. |
| `MailDomainNotFoundException` | `structure` | `Message` | The domain specified is not found in your organization. |
| `NameAvailabilityException` | `structure` | `Message` | The user, group, or resource name isn't unique in WorkMail. |
| `MailDomainStateException` | `structure` | `Message` | After a domain has been added to the organization, it must be verified. |
| `EmailAddressInUseException` | `structure` | `Message` | The email address that you're trying to assign is already created for a different user, group, or resource. |
| `ReservedNameException` | `structure` | `Message` | This user, group, or resource name is not allowed in WorkMail. |
| `InvalidPasswordException` | `structure` | `Message` | The supplied password doesn't match the minimum security constraints, such as length or use of special characters. |
| `MailDomainInUseException` | `structure` | `Message` | The domain you're trying to change is in use by another user or organization in your account. |
| `AssociateDelegateToResourceRequest` | `structure` | `EntityId`, `OrganizationId`, `ResourceId` | - |
| `AssociateDelegateToResourceResponse` | `structure` | - | - |
| `AssociateMemberToGroupRequest` | `structure` | `GroupId`, `MemberId`, `OrganizationId` | - |
| `AssociateMemberToGroupResponse` | `structure` | - | - |
| `AssumeImpersonationRoleRequest` | `structure` | `ImpersonationRoleId`, `OrganizationId` | - |
| `AssumeImpersonationRoleResponse` | `structure` | `ExpiresIn`, `Token` | - |
| `CancelMailboxExportJobRequest` | `structure` | `ClientToken`, `JobId`, `OrganizationId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

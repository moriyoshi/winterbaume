# AWS Organizations

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Organizations is a web service that enables you to consolidate your multiple Amazon Web Services accounts into an organization and centrally manage your accounts and their resources. This guide provides descriptions of the Organizations operations. For more information about using this service, see the Organizations User Guide. Support and feedback for Organizations We welcome your feedback. You can post your feedback and questions in the Organizations support forum.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Organizations where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Organizations by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS Organizations workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Enable` operation families, including `ListAWSServiceAccessForOrganization`, `ListAccounts`, `ListAccountsForParent`, `ListAccountsWithInvalidEffectivePolicy`, `DescribeAccount`, `DescribeCreateAccountStatus`.

## Service Identity and Protocol

- AWS model slug: `organizations`
- AWS SDK for Rust slug: `organizations`
- Model version: `2016-11-28`
- Model file: `vendor/api-models-aws/models/organizations/service/2016-11-28/organizations-2016-11-28.json`
- SDK ID: `Organizations`
- Endpoint prefix: `organizations`
- ARN namespace: `organizations`
- CloudFormation name: `Organizations`
- CloudTrail event source: `organizations.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (20), `Describe` (9), `Create` (5), `Delete` (4), `Enable` (3), `Update` (3), `Disable` (2), `Invite` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptHandshake`, `AttachPolicy`, `CancelHandshake`, `CreateAccount`, `CreateGovCloudAccount`, `CreateOrganization`, `CreateOrganizationalUnit`, `CreatePolicy`, `DeleteOrganization`, `DeleteOrganizationalUnit`, `DeletePolicy`, `DeleteResourcePolicy`, `DeregisterDelegatedAdministrator`, `DetachPolicy`, `DisableAWSServiceAccess`, `DisablePolicyType`, `EnableAWSServiceAccess`, `EnableAllFeatures`, `EnablePolicyType`, `PutResourcePolicy`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccount`, `DescribeCreateAccountStatus`, `DescribeEffectivePolicy`, `DescribeHandshake`, `DescribeOrganization`, `DescribeOrganizationalUnit`, `DescribePolicy`, `DescribeResourcePolicy`, `DescribeResponsibilityTransfer`, `ListAWSServiceAccessForOrganization`, `ListAccounts`, `ListAccountsForParent`, `ListAccountsWithInvalidEffectivePolicy`, `ListChildren`, `ListCreateAccountStatus`, `ListDelegatedAdministrators`, `ListDelegatedServicesForAccount`, `ListEffectivePolicyValidationErrors`, `ListHandshakesForAccount`, `ListHandshakesForOrganization`, ... (+9).
- Pagination is modelled for 18 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelHandshake`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 63 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scps.html
- https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_delegated_admin.html
- https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html

Research outcomes:
- Service control policies (SCPs) do not grant permissions. They define the maximum permissions that can be used by IAM users and roles in affected member accounts.
- Effective account permissions require the account and every parent above it to allow an action. A deny or missing allow at any parent prevents access even when an identity policy allows it.
- SCPs apply to member accounts, including root users and delegated administrators, but do not apply to management account users or roles.
- SCPs do not affect service-linked roles, principals from outside the organisation that are authorised by resource policies, or selected root/support tasks documented by AWS.
- If the SCP policy type is disabled for a root, all SCPs detach. Re-enabling SCPs restores only the default `FullAWSAccess` policy and previous attachments must be recreated.
- Delegated administrator accounts receive service-specific delegated permissions plus read-only Organizations permissions for describing and listing accounts, organisational units, policies, handshakes, roots, and related metadata.
- CreateAccount creates the member account in the root and returns a `CreateAccountStatus` in `IN_PROGRESS`; callers must use `DescribeCreateAccountStatus` to observe completion or failure.
- CreateAccount automatically creates the `OrganizationAccountAccessRole` in the member account and the Organizations service-linked role.
- Newly created member accounts inherit selected primary contact, communication language, and Marketplace information from the management account, and policies attached to the root apply immediately.

Parity implications:
- Model organisations, roots, accounts, organisational units, policy types, policy attachments, invitations/handshakes, create-account statuses, trusted access, and delegated administrators separately.
- Permission simulation should treat SCPs as account-bound maximum permission filters rather than identity policies.
- CreateAccount, invite flows, policy type enablement, and delegated-admin registration should expose asynchronous or eventually consistent status where AWS does.

## Operation Groups

### List

- Operations: `ListAWSServiceAccessForOrganization`, `ListAccounts`, `ListAccountsForParent`, `ListAccountsWithInvalidEffectivePolicy`, `ListChildren`, `ListCreateAccountStatus`, `ListDelegatedAdministrators`, `ListDelegatedServicesForAccount`, `ListEffectivePolicyValidationErrors`, `ListHandshakesForAccount`, `ListHandshakesForOrganization`, `ListInboundResponsibilityTransfers`, `ListOrganizationalUnitsForParent`, `ListOutboundResponsibilityTransfers`, `ListParents`, `ListPolicies`, `ListPoliciesForTarget`, `ListRoots`, `ListTagsForResource`, `ListTargetsForPolicy`
- Traits: `paginated` (18)
- Common required input members in this group: `AccountId`, `ChildId`, `ChildType`, `Filter`, `ParentId`, `PolicyId`, `PolicyType`, `ResourceId`, `TargetId`, `Type`

### Describe

- Operations: `DescribeAccount`, `DescribeCreateAccountStatus`, `DescribeEffectivePolicy`, `DescribeHandshake`, `DescribeOrganization`, `DescribeOrganizationalUnit`, `DescribePolicy`, `DescribeResourcePolicy`, `DescribeResponsibilityTransfer`
- Common required input members in this group: `AccountId`, `CreateAccountRequestId`, `HandshakeId`, `Id`, `OrganizationalUnitId`, `PolicyId`, `PolicyType`

### Create

- Operations: `CreateAccount`, `CreateGovCloudAccount`, `CreateOrganization`, `CreateOrganizationalUnit`, `CreatePolicy`
- Common required input members in this group: `AccountName`, `Content`, `Description`, `Email`, `Name`, `ParentId`, `Type`

### Delete

- Operations: `DeleteOrganization`, `DeleteOrganizationalUnit`, `DeletePolicy`, `DeleteResourcePolicy`
- Common required input members in this group: `OrganizationalUnitId`, `PolicyId`

### Enable

- Operations: `EnableAWSServiceAccess`, `EnableAllFeatures`, `EnablePolicyType`
- Common required input members in this group: `PolicyType`, `RootId`, `ServicePrincipal`

### Update

- Operations: `UpdateOrganizationalUnit`, `UpdatePolicy`, `UpdateResponsibilityTransfer`
- Common required input members in this group: `Id`, `Name`, `OrganizationalUnitId`, `PolicyId`

### Disable

- Operations: `DisableAWSServiceAccess`, `DisablePolicyType`
- Common required input members in this group: `PolicyType`, `RootId`, `ServicePrincipal`

### Invite

- Operations: `InviteAccountToOrganization`, `InviteOrganizationToTransferResponsibility`
- Common required input members in this group: `SourceName`, `StartTimestamp`, `Target`, `Type`

### Accept

- Operations: `AcceptHandshake`
- Common required input members in this group: `HandshakeId`

### Attach

- Operations: `AttachPolicy`
- Common required input members in this group: `PolicyId`, `TargetId`

### Cancel

- Operations: `CancelHandshake`
- Common required input members in this group: `HandshakeId`

### Close

- Operations: `CloseAccount`
- Common required input members in this group: `AccountId`

### Decline

- Operations: `DeclineHandshake`
- Common required input members in this group: `HandshakeId`

### Deregister

- Operations: `DeregisterDelegatedAdministrator`
- Common required input members in this group: `AccountId`, `ServicePrincipal`

### Detach

- Operations: `DetachPolicy`
- Common required input members in this group: `PolicyId`, `TargetId`

### Leave

- Operations: `LeaveOrganization`

### Move

- Operations: `MoveAccount`
- Common required input members in this group: `AccountId`, `DestinationParentId`, `SourceParentId`

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: `Content`

### Register

- Operations: `RegisterDelegatedAdministrator`
- Common required input members in this group: `AccountId`, `ServicePrincipal`

### Remove

- Operations: `RemoveAccountFromOrganization`
- Common required input members in this group: `AccountId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceId`, `Tags`

### Terminate

- Operations: `TerminateResponsibilityTransfer`
- Common required input members in this group: `Id`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceId`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptHandshake` | - | - | `HandshakeId` | - | `AcceptHandshakeResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccessDeniedForDependencyException`, `ConcurrentModificationException`, `ConstraintViolationException`, `HandshakeAlreadyInStateException`, `HandshakeConstraintViolationException`, `HandshakeNotFoundException`, ... (+5) | Accepts a handshake by sending an `ACCEPTED` response to the sender. You can view accepted handshakes in API responses for 30 days before they are deleted. |
| `AttachPolicy` | - | - | `PolicyId`, `TargetId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicatePolicyAttachmentException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyNotFoundException`, ... (+5) | Attaches a policy to a root, an organizational unit (OU), or an individual account. How the policy affects accounts depends on the type of policy. |
| `CancelHandshake` | - | - | `HandshakeId` | - | `CancelHandshakeResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `HandshakeAlreadyInStateException`, `HandshakeNotFoundException`, `InvalidHandshakeTransitionException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Cancels a Handshake. Only the account that sent a handshake can call this operation. |
| `CloseAccount` | - | - | `AccountId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountAlreadyClosedException`, `AccountNotFoundException`, `ConcurrentModificationException`, `ConflictException`, `ConstraintViolationException`, `InvalidInputException`, ... (+3) | Closes an Amazon Web Services member account within an organization. You can close an account when all features are enabled . |
| `CreateAccount` | - | - | `AccountName`, `Email` | - | `CreateAccountResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FinalizingOrganizationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, ... (+1) | Creates an Amazon Web Services account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that Amazon Web Services performs in the background. |
| `CreateGovCloudAccount` | - | - | `AccountName`, `Email` | - | `CreateGovCloudAccountResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FinalizingOrganizationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, ... (+1) | This action is available if all of the following are true: You're authorized to create accounts in the Amazon Web Services GovCloud (US) Region. For more information on the Amazon Web Services GovCloud (US) Region, see the Amazon Web Services GovCloud User... |
| `CreateOrganization` | - | - | - | - | `CreateOrganizationResponse` | `AccessDeniedException`, `AccessDeniedForDependencyException`, `AlreadyInOrganizationException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Creates an Amazon Web Services organization. The account whose user is calling the `CreateOrganization` operation automatically becomes the management account of the new organization. |
| `CreateOrganizationalUnit` | - | - | `Name`, `ParentId` | - | `CreateOrganizationalUnitResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicateOrganizationalUnitException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, ... (+1) | Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. |
| `CreatePolicy` | - | - | `Content`, `Description`, `Name`, `Type` | - | `CreatePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicatePolicyException`, `InvalidInputException`, `MalformedPolicyDocumentException`, `PolicyTypeNotAvailableForOrganizationException`, ... (+3) | Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual Amazon Web Services account. For more information about policies and their use, see Managing Organizations policies. |
| `DeclineHandshake` | - | - | `HandshakeId` | - | `DeclineHandshakeResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `HandshakeAlreadyInStateException`, `HandshakeNotFoundException`, `InvalidHandshakeTransitionException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Declines a Handshake. Only the account that receives a handshake can call this operation. |
| `DeleteOrganization` | - | - | - | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `OrganizationNotEmptyException`, `ServiceException`, `TooManyRequestsException` | Deletes the organization. You can delete an organization only by using credentials from the management account. |
| `DeleteOrganizationalUnit` | - | - | `OrganizationalUnitId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `InvalidInputException`, `OrganizationalUnitNotEmptyException`, `OrganizationalUnitNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes an organizational unit (OU) from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete. |
| `DeletePolicy` | - | - | `PolicyId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `InvalidInputException`, `PolicyInUseException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, ... (+1) | Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all organizational units (OUs), roots, and accounts. |
| `DeleteResourcePolicy` | - | - | - | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `ResourcePolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Deletes the resource policy from your organization. You can only call this operation from the management account. |
| `DeregisterDelegatedAdministrator` | - | - | `AccountId`, `ServicePrincipal` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `AccountNotRegisteredException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, ... (+2) | Removes the specified member Amazon Web Services account as a delegated administrator for the specified Amazon Web Services service. Deregistering a delegated administrator can have unintended impacts on the functionality of the enabled Amazon Web Services... |
| `DescribeAccount` | - | - | `AccountId` | - | `DescribeAccountResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Retrieves Organizations-related information about the specified account. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeCreateAccountStatus` | - | - | `CreateAccountRequestId` | - | `DescribeCreateAccountStatusResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `CreateAccountStatusNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves the current status of an asynchronous request to create an account. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeEffectivePolicy` | - | - | `PolicyType` | - | `DescribeEffectivePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `EffectivePolicyNotFoundException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException`, ... (+1) | Returns the contents of the effective policy for specified policy type and account. The effective policy is the aggregation of any policies of the specified type that the account inherits, plus any policy of that type that is directly attached to the account. |
| `DescribeHandshake` | - | - | `HandshakeId` | - | `DescribeHandshakeResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `HandshakeNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Returns details for a handshake. A handshake is the secure exchange of information between two Amazon Web Services accounts: a sender and a recipient. |
| `DescribeOrganization` | - | - | - | - | `DescribeOrganizationResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about the organization that the user's account belongs to. You can call this operation from any account in a organization. |
| `DescribeOrganizationalUnit` | - | - | `OrganizationalUnitId` | - | `DescribeOrganizationalUnitResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `OrganizationalUnitNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about an organizational unit (OU). You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribePolicy` | - | - | `PolicyId` | - | `DescribePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves information about a policy. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeResourcePolicy` | - | - | - | - | `DescribeResourcePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `ResourcePolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves information about a resource policy. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeResponsibilityTransfer` | - | - | `Id` | - | `DescribeResponsibilityTransferResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Returns details for a transfer. A transfer is an arrangement between two management accounts where one account designates the other with specified responsibilities for their organization. |
| `DetachPolicy` | - | - | `PolicyId`, `TargetId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyNotAttachedException`, `PolicyNotFoundException`, ... (+4) | Detaches a policy from a target root, organizational unit (OU), or account. If the policy being detached is a service control policy (SCP), the changes to permissions for Identity and Access Management (IAM) users and roles in affected accounts are immediate. |
| `DisableAWSServiceAccess` | - | - | `ServicePrincipal` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Disables the integration of an Amazon Web Services service (the service that is specified by `ServicePrincipal`) with Organizations. When you disable integration, the specified service no longer can create a service-linked role in new accounts in your... |
| `DisablePolicyType` | - | - | `PolicyType`, `RootId` | - | `DisablePolicyTypeResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyTypeNotEnabledException`, `RootNotFoundException`, ... (+3) | Disables an organizational policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. |
| `EnableAWSServiceAccess` | - | - | `ServicePrincipal` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Provides an Amazon Web Services service (the service that is specified by `ServicePrincipal`) with permissions to view the structure of an organization, create a service-linked role in all the accounts in the organization, and allow the service to perform... |
| `EnableAllFeatures` | - | - | - | - | `EnableAllFeaturesResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `HandshakeConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. |
| `EnablePolicyType` | - | - | `PolicyType`, `RootId` | - | `EnablePolicyTypeResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyTypeAlreadyEnabledException`, `PolicyTypeNotAvailableForOrganizationException`, ... (+4) | Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any organizational unit (OU), or account in that root. |
| `InviteAccountToOrganization` | - | - | `Target` | - | `InviteAccountToOrganizationResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountOwnerNotVerifiedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicateHandshakeException`, `FinalizingOrganizationException`, `HandshakeConstraintViolationException`, ... (+3) | Sends an invitation to another account to join your organization as a member account. Organizations sends email on your behalf to the email address that is associated with the other account's owner. |
| `InviteOrganizationToTransferResponsibility` | - | - | `SourceName`, `StartTimestamp`, `Target`, `Type` | - | `InviteOrganizationToTransferResponsibilityResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicateHandshakeException`, `HandshakeConstraintViolationException`, `InvalidInputException`, `ServiceException`, ... (+2) | Sends an invitation to another organization's management account to designate your account with the specified responsibilities for their organization. The invitation is implemented as a Handshake whose details are in the response. |
| `LeaveOrganization` | - | - | - | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `MasterCannotLeaveOrganizationException`, `ServiceException`, ... (+1) | Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. |
| `ListAWSServiceAccessForOrganization` | - | `paginated` | - | - | `ListAWSServiceAccessForOrganizationResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Returns a list of the Amazon Web Services services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts. |
| `ListAccounts` | - | `paginated` | - | - | `ListAccountsResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists all the accounts in the organization. To request only the accounts in a specified root or organizational unit (OU), use the ListAccountsForParent operation instead. |
| `ListAccountsForParent` | - | `paginated` | `ParentId` | - | `ListAccountsForParentResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that aren't in any OU. |
| `ListAccountsWithInvalidEffectivePolicy` | - | `paginated` | `PolicyType` | - | `ListAccountsWithInvalidEffectivePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `EffectivePolicyNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists all the accounts in an organization that have invalid effective policies. An invalid effective policy is an effective policy that fails validation checks, resulting in the effective policy not being fully enforced on all the intended accounts within an... |
| `ListChildren` | - | `paginated` | `ChildType`, `ParentId` | - | `ListChildrenResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists all of the organizational units (OUs) or accounts that are contained in the specified parent OU or root. This operation, along with ListParents enables you to traverse the tree structure that makes up this root. |
| `ListCreateAccountStatus` | - | `paginated` | - | - | `ListCreateAccountStatusResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists the account creation requests that match the specified status that is currently being tracked for the organization. When calling List* operations, always check the `NextToken` response parameter value, even if you receive an empty result set. |
| `ListDelegatedAdministrators` | - | `paginated` | - | - | `ListDelegatedAdministratorsResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists the Amazon Web Services accounts that are designated as delegated administrators in this organization. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `ListDelegatedServicesForAccount` | - | `paginated` | `AccountId` | - | `ListDelegatedServicesForAccountResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `AccountNotRegisteredException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, ... (+1) | List the Amazon Web Services services for which the specified account is a delegated administrator. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `ListEffectivePolicyValidationErrors` | - | `paginated` | `AccountId`, `PolicyType` | - | `ListEffectivePolicyValidationErrorsResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `ConstraintViolationException`, `EffectivePolicyNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, ... (+1) | Lists all the validation errors on an effective policy for a specified account and policy type. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `ListHandshakesForAccount` | - | `paginated` | - | - | `ListHandshakesForAccountResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the recent handshakes that you have received. You can view `CANCELED`, `ACCEPTED`, `DECLINED`, or `EXPIRED` handshakes in API responses for 30 days before they are deleted. |
| `ListHandshakesForOrganization` | - | `paginated` | - | - | `ListHandshakesForOrganizationResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the recent handshakes that you have sent. You can view `CANCELED`, `ACCEPTED`, `DECLINED`, or `EXPIRED` handshakes in API responses for 30 days before they are deleted. |
| `ListInboundResponsibilityTransfers` | - | - | `Type` | - | `ListInboundResponsibilityTransfersResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `InvalidInputException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists transfers that allow you to manage the specified responsibilities for another organization. This operation returns both transfer invitations and transfers. |
| `ListOrganizationalUnitsForParent` | - | `paginated` | `ParentId` | - | `ListOrganizationalUnitsForParentResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists the organizational units (OUs) in a parent organizational unit or root. When calling List* operations, always check the `NextToken` response parameter value, even if you receive an empty result set. |
| `ListOutboundResponsibilityTransfers` | - | - | `Type` | - | `ListOutboundResponsibilityTransfersResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists transfers that allow an account outside your organization to manage the specified responsibilities for your organization. This operation returns both transfer invitations and transfers. |
| `ListParents` | - | `paginated` | `ChildId` | - | `ListParentsResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ChildNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with ListChildren enables you to traverse the tree structure that makes up this root. |
| `ListPolicies` | - | `paginated` | `Filter` | - | `ListPoliciesResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves the list of all policies in an organization of a specified type. When calling List* operations, always check the `NextToken` response parameter value, even if you receive an empty result set. |
| `ListPoliciesForTarget` | - | `paginated` | `Filter`, `TargetId` | - | `ListPoliciesForTargetResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list. |
| `ListRoots` | - | `paginated` | - | - | `ListRootsResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the roots that are defined in the current organization. When calling List* operations, always check the `NextToken` response parameter value, even if you receive an empty result set. |
| `ListTagsForResource` | - | `paginated` | `ResourceId` | - | `ListTagsForResourceResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException` | Lists tags that are attached to the specified resource. You can attach tags to the following resources in Organizations. |
| `ListTargetsForPolicy` | - | `paginated` | `PolicyId` | - | `ListTargetsForPolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `InvalidInputException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists all the roots, organizational units (OUs), and accounts that the specified policy is attached to. When calling List* operations, always check the `NextToken` response parameter value, even if you receive an empty result set. |
| `MoveAccount` | - | - | `AccountId`, `DestinationParentId`, `SourceParentId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `ConcurrentModificationException`, `DestinationParentNotFoundException`, `DuplicateAccountException`, `InvalidInputException`, `ServiceException`, ... (+2) | Moves an account from its current source parent root or organizational unit (OU) to the specified destination parent root or OU. You can only call this operation from the management account. |
| `PutResourcePolicy` | - | - | `Content` | - | `PutResourcePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Creates or updates a resource policy. You can only call this operation from the management account.. |
| `RegisterDelegatedAdministrator` | - | - | `AccountId`, `ServicePrincipal` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountAlreadyRegisteredException`, `AccountNotFoundException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, ... (+2) | Enables the specified member account to administer the Organizations features of the specified Amazon Web Services service. It grants read-only access to Organizations service data. |
| `RemoveAccountFromOrganization` | - | - | `AccountId` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `AccountNotFoundException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `MasterCannotLeaveOrganizationException`, `ServiceException`, ... (+1) | Removes the specified account from the organization. The removed account becomes a standalone account that isn't a member of any organization. |
| `TagResource` | - | - | `ResourceId`, `Tags` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException` | Adds one or more tags to the specified resource. Currently, you can attach tags to the following resources in Organizations. |
| `TerminateResponsibilityTransfer` | - | - | `Id` | - | `TerminateResponsibilityTransferResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `InvalidResponsibilityTransferTransitionException`, `ResponsibilityTransferAlreadyInStatusException`, `ResponsibilityTransferNotFoundException`, ... (+3) | Ends a transfer. A transfer is an arrangement between two management accounts where one account designates the other with specified responsibilities for their organization. |
| `UntagResource` | - | - | `ResourceId`, `TagKeys` | - | `Unit` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException` | Removes any tags with the specified keys from the specified resource. You can attach tags to the following resources in Organizations. |
| `UpdateOrganizationalUnit` | - | - | `OrganizationalUnitId` | - | `UpdateOrganizationalUnitResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `DuplicateOrganizationalUnitException`, `InvalidInputException`, `OrganizationalUnitNotFoundException`, `ServiceException`, `TooManyRequestsException` | Renames the specified organizational unit (OU). The ID and ARN don't change. |
| `UpdatePolicy` | - | - | `PolicyId` | - | `UpdatePolicyResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicatePolicyException`, `InvalidInputException`, `MalformedPolicyDocumentException`, `PolicyChangesInProgressException`, ... (+4) | Updates an existing policy with a new name, description, or content. If you don't supply any parameter, that value remains unchanged. |
| `UpdateResponsibilityTransfer` | - | - | `Id`, `Name` | - | `UpdateResponsibilityTransferResponse` | `AWSOrganizationsNotInUseException`, `AccessDeniedException`, `ConstraintViolationException`, `InvalidInputException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Updates a transfer. A transfer is the arrangement between two management accounts where one account designates the other with specified responsibilities for their organization. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don't have permissions to perform the requested operation. |
| `ServiceException` | `structure` | `Message` | Organizations can't complete your request because of an internal service error. |
| `TooManyRequestsException` | `structure` | `Message`, `Type` | You have sent too many requests in too short a period of time. |
| `InvalidInputException` | `structure` | `Message`, `Reason` | The requested operation failed because you provided invalid values for one or more of the request parameters. |
| `AWSOrganizationsNotInUseException` | `structure` | `Message` | Your account isn't a member of an organization. |
| `ConcurrentModificationException` | `structure` | `Message` | The target of the operation is currently being modified by a different request. |
| `ConstraintViolationException` | `structure` | `Message`, `Reason` | Performing this operation violates a minimum or maximum value limit. |
| `UnsupportedAPIEndpointException` | `structure` | `Message` | This action isn't available in the current Amazon Web Services Region. |
| `AccountNotFoundException` | `structure` | `Message` | We can't find an Amazon Web Services account with the `AccountId` that you specified, or the account whose credentials you used to make this request isn't a member of an... |
| `TargetNotFoundException` | `structure` | `Message` | We can't find a root, OU, account, or policy with the `TargetId` that you specified. |
| `PolicyNotFoundException` | `structure` | `Message` | We can't find a policy with the `PolicyId` that you specified. |
| `PolicyChangesInProgressException` | `structure` | `Message` | Changes to the effective policy are in progress, and its contents can't be returned. |
| `HandshakeConstraintViolationException` | `structure` | `Message`, `Reason` | The requested operation would violate the constraint identified in the reason code. |
| `HandshakeNotFoundException` | `structure` | `Message` | We can't find a handshake with the `HandshakeId` that you specified. |
| `ParentNotFoundException` | `structure` | `Message` | We can't find a root or OU with the `ParentId` that you specified. |
| `ResponsibilityTransferNotFoundException` | `structure` | `Message` | We can't find a transfer that you specified. |
| `HandshakeAlreadyInStateException` | `structure` | `Message` | The specified handshake is already in the requested state. |
| `InvalidHandshakeTransitionException` | `structure` | `Message` | You can't perform the operation on the handshake in its current state. |
| `MasterCannotLeaveOrganizationException` | `structure` | `Message` | You can't remove a management account from an organization. |
| `FinalizingOrganizationException` | `structure` | `Message` | Organizations couldn't perform the operation because your organization hasn't finished initializing. |
| `OrganizationalUnitNotFoundException` | `structure` | `Message` | We can't find an OU with the `OrganizationalUnitId` that you specified. |
| `EffectivePolicyNotFoundException` | `structure` | `Message` | If you ran this action on the management account, this policy type is not enabled. |
| `AccessDeniedForDependencyException` | `structure` | `Message`, `Reason` | The operation that you attempted requires you to have the `iam:CreateServiceLinkedRole` for `organizations.amazonaws.com` permission so that Organizations can create the required... |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

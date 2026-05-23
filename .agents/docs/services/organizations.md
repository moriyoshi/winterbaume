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

- Operations: `ListAccounts`, `ListAccountsForParent`, `ListAccountsWithInvalidEffectivePolicy`, `ListAWSServiceAccessForOrganization`, `ListChildren`, `ListCreateAccountStatus`, `ListDelegatedAdministrators`, `ListDelegatedServicesForAccount`, `ListEffectivePolicyValidationErrors`, `ListHandshakesForAccount`, `ListHandshakesForOrganization`, `ListInboundResponsibilityTransfers`, `ListOrganizationalUnitsForParent`, `ListOutboundResponsibilityTransfers`, `ListParents`, `ListPolicies`, `ListPoliciesForTarget`, `ListRoots`, `ListTagsForResource`, `ListTargetsForPolicy`
- Traits: `paginated` (18)
- Common required input members in this group: `ParentId`, `PolicyType`, `AccountId`, `Type`, `Filter`

### Describe

- Operations: `DescribeAccount`, `DescribeCreateAccountStatus`, `DescribeEffectivePolicy`, `DescribeHandshake`, `DescribeOrganization`, `DescribeOrganizationalUnit`, `DescribePolicy`, `DescribeResourcePolicy`, `DescribeResponsibilityTransfer`
- Common required input members in this group: -

### Create

- Operations: `CreateAccount`, `CreateGovCloudAccount`, `CreateOrganization`, `CreateOrganizationalUnit`, `CreatePolicy`
- Common required input members in this group: `Email`, `AccountName`, `Name`

### Delete

- Operations: `DeleteOrganization`, `DeleteOrganizationalUnit`, `DeletePolicy`, `DeleteResourcePolicy`
- Common required input members in this group: -

### Enable

- Operations: `EnableAllFeatures`, `EnableAWSServiceAccess`, `EnablePolicyType`
- Common required input members in this group: -

### Update

- Operations: `UpdateOrganizationalUnit`, `UpdatePolicy`, `UpdateResponsibilityTransfer`
- Common required input members in this group: -

### Disable

- Operations: `DisableAWSServiceAccess`, `DisablePolicyType`
- Common required input members in this group: -

### Invite

- Operations: `InviteAccountToOrganization`, `InviteOrganizationToTransferResponsibility`
- Common required input members in this group: `Target`

### Accept

- Operations: `AcceptHandshake`
- Common required input members in this group: -

### Attach

- Operations: `AttachPolicy`
- Common required input members in this group: -

### Cancel

- Operations: `CancelHandshake`
- Common required input members in this group: -

### Close

- Operations: `CloseAccount`
- Common required input members in this group: -

### Decline

- Operations: `DeclineHandshake`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterDelegatedAdministrator`
- Common required input members in this group: -

### Detach

- Operations: `DetachPolicy`
- Common required input members in this group: -

### Leave

- Operations: `LeaveOrganization`
- Common required input members in this group: -

### Move

- Operations: `MoveAccount`
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: -

### Register

- Operations: `RegisterDelegatedAdministrator`
- Common required input members in this group: -

### Remove

- Operations: `RemoveAccountFromOrganization`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Terminate

- Operations: `TerminateResponsibilityTransfer`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptHandshake` | `-` | - | `HandshakeId` | - | `AcceptHandshakeResponse` | `AccessDeniedException`, `AccessDeniedForDependencyException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `HandshakeAlreadyInStateException`, `HandshakeConstraintViolationException`, `HandshakeNotFoundException`, `InvalidHandshakeTransitionException`, `InvalidInputException`, `MasterCannotLeaveOrganizationException`, `ServiceException`, `TooManyRequestsException` | Accepts a handshake by sending an ACCEPTED response to the sender. You can view accepted handshakes in API responses for 30 days before they are deleted. Only the management account can accept the following handshake ... |
| `AttachPolicy` | `-` | - | `PolicyId`, `TargetId` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicatePolicyAttachmentException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyNotFoundException`, `PolicyTypeNotEnabledException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Attaches a policy to a root, an organizational unit (OU), or an individual account. How the policy affects accounts depends on the type of policy. Refer to the Organizations User Guide for information about each poli ... |
| `CancelHandshake` | `-` | - | `HandshakeId` | - | `CancelHandshakeResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `HandshakeAlreadyInStateException`, `HandshakeNotFoundException`, `InvalidHandshakeTransitionException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Cancels a Handshake . Only the account that sent a handshake can call this operation. The recipient of the handshake can't cancel it, but can use DeclineHandshake to decline. After a handshake is canceled, the recipi ... |
| `CloseAccount` | `-` | - | `AccountId` | - | `Unit` | `AccessDeniedException`, `AccountAlreadyClosedException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConflictException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Closes an Amazon Web Services member account within an organization. You can close an account when all features are enabled . You can't close the management account with this API. This is an asynchronous request that ... |
| `CreateAccount` | `-` | - | `Email`, `AccountName` | - | `CreateAccountResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FinalizingOrganizationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Creates an Amazon Web Services account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that Amazon Web Services performs in the background. Becau ... |
| `CreateGovCloudAccount` | `-` | - | `Email`, `AccountName` | - | `CreateGovCloudAccountResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `FinalizingOrganizationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | This action is available if all of the following are true: You're authorized to create accounts in the Amazon Web Services GovCloud (US) Region. For more information on the Amazon Web Services GovCloud (US) Region, s ... |
| `CreateOrganization` | `-` | - | - | - | `CreateOrganizationResponse` | `AccessDeniedException`, `AccessDeniedForDependencyException`, `AlreadyInOrganizationException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Creates an Amazon Web Services organization. The account whose user is calling the CreateOrganization operation automatically becomes the management account of the new organization. This operation must be called usin ... |
| `CreateOrganizationalUnit` | `-` | - | `ParentId`, `Name` | - | `CreateOrganizationalUnitResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicateOrganizationalUnitException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of le ... |
| `CreatePolicy` | `-` | - | `Content`, `Description`, `Name`, `Type` | - | `CreatePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicatePolicyException`, `InvalidInputException`, `MalformedPolicyDocumentException`, `PolicyTypeNotAvailableForOrganizationException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual Amazon Web Services account. For more information about policies and their use, see Managing Organizat ... |
| `DeclineHandshake` | `-` | - | `HandshakeId` | - | `DeclineHandshakeResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `HandshakeAlreadyInStateException`, `HandshakeNotFoundException`, `InvalidHandshakeTransitionException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Declines a Handshake . Only the account that receives a handshake can call this operation. The sender of the handshake can use CancelHandshake to cancel if the handshake hasn't yet been responded to. You can view can ... |
| `DeleteOrganization` | `-` | - | - | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `OrganizationNotEmptyException`, `ServiceException`, `TooManyRequestsException` | Deletes the organization. You can delete an organization only by using credentials from the management account. The organization must be empty of member accounts. |
| `DeleteOrganizationalUnit` | `-` | - | `OrganizationalUnitId` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `InvalidInputException`, `OrganizationalUnitNotEmptyException`, `OrganizationalUnitNotFoundException`, `ServiceException`, `TooManyRequestsException` | Deletes an organizational unit (OU) from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete. You can only call this operation from the management account. |
| `DeletePolicy` | `-` | - | `PolicyId` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `InvalidInputException`, `PolicyInUseException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all organizational units (OUs), roots, and accounts. You can only call this operation from ... |
| `DeleteResourcePolicy` | `-` | - | - | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `ResourcePolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Deletes the resource policy from your organization. You can only call this operation from the management account. |
| `DeregisterDelegatedAdministrator` | `-` | - | `AccountId`, `ServicePrincipal` | - | `Unit` | `AccessDeniedException`, `AccountNotFoundException`, `AccountNotRegisteredException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Removes the specified member Amazon Web Services account as a delegated administrator for the specified Amazon Web Services service. Deregistering a delegated administrator can have unintended impacts on the function ... |
| `DescribeAccount` | `-` | - | `AccountId` | - | `DescribeAccountResponse` | `AccessDeniedException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Retrieves Organizations-related information about the specified account. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeCreateAccountStatus` | `-` | - | `CreateAccountRequestId` | - | `DescribeCreateAccountStatusResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `CreateAccountStatusNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves the current status of an asynchronous request to create an account. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeEffectivePolicy` | `-` | - | `PolicyType` | - | `DescribeEffectivePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `EffectivePolicyNotFoundException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Returns the contents of the effective policy for specified policy type and account. The effective policy is the aggregation of any policies of the specified type that the account inherits, plus any policy of that typ ... |
| `DescribeHandshake` | `-` | - | `HandshakeId` | - | `DescribeHandshakeResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `HandshakeNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Returns details for a handshake. A handshake is the secure exchange of information between two Amazon Web Services accounts: a sender and a recipient. You can view ACCEPTED , DECLINED , or CANCELED handshakes in API ... |
| `DescribeOrganization` | `-` | - | - | - | `DescribeOrganizationResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about the organization that the user's account belongs to. You can call this operation from any account in a organization. Even if a policy type is shown as available in the organization, you ca ... |
| `DescribeOrganizationalUnit` | `-` | - | `OrganizationalUnitId` | - | `DescribeOrganizationalUnitResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `OrganizationalUnitNotFoundException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about an organizational unit (OU). You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribePolicy` | `-` | - | `PolicyId` | - | `DescribePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves information about a policy. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeResourcePolicy` | `-` | - | - | - | `DescribeResourcePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `ResourcePolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves information about a resource policy. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `DescribeResponsibilityTransfer` | `-` | - | `Id` | - | `DescribeResponsibilityTransferResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Returns details for a transfer. A transfer is an arrangement between two management accounts where one account designates the other with specified responsibilities for their organization. |
| `DetachPolicy` | `-` | - | `PolicyId`, `TargetId` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyNotAttachedException`, `PolicyNotFoundException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Detaches a policy from a target root, organizational unit (OU), or account. If the policy being detached is a service control policy (SCP), the changes to permissions for Identity and Access Management (IAM) users an ... |
| `DisableAWSServiceAccess` | `-` | - | `ServicePrincipal` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Disables the integration of an Amazon Web Services service (the service that is specified by ServicePrincipal ) with Organizations. When you disable integration, the specified service no longer can create a service-l ... |
| `DisablePolicyType` | `-` | - | `RootId`, `PolicyType` | - | `DisablePolicyTypeResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyTypeNotEnabledException`, `RootNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Disables an organizational policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach ... |
| `EnableAllFeatures` | `-` | - | - | - | `EnableAllFeaturesResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `HandshakeConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access onl ... |
| `EnableAWSServiceAccess` | `-` | - | `ServicePrincipal` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Provides an Amazon Web Services service (the service that is specified by ServicePrincipal ) with permissions to view the structure of an organization, create a service-linked role in all the accounts in the organiza ... |
| `EnablePolicyType` | `-` | - | `RootId`, `PolicyType` | - | `EnablePolicyTypeResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `PolicyChangesInProgressException`, `PolicyTypeAlreadyEnabledException`, `PolicyTypeNotAvailableForOrganizationException`, `RootNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any organizational unit (OU), or account in that root. You can undo this by using the Disab ... |
| `InviteAccountToOrganization` | `-` | - | `Target` | - | `InviteAccountToOrganizationResponse` | `AccessDeniedException`, `AccountOwnerNotVerifiedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicateHandshakeException`, `FinalizingOrganizationException`, `HandshakeConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Sends an invitation to another account to join your organization as a member account. Organizations sends email on your behalf to the email address that is associated with the other account's owner. The invitation is ... |
| `InviteOrganizationToTransferResponsibility` | `-` | - | `Type`, `Target`, `StartTimestamp`, `SourceName` | - | `InviteOrganizationToTransferResponsibilityResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicateHandshakeException`, `HandshakeConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Sends an invitation to another organization's management account to designate your account with the specified responsibilities for their organization. The invitation is implemented as a Handshake whose details are in ... |
| `LeaveOrganization` | `-` | - | - | - | `Unit` | `AccessDeniedException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `MasterCannotLeaveOrganizationException`, `ServiceException`, `TooManyRequestsException` | Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the management account, use RemoveAccountF ... |
| `ListAccounts` | `-` | `paginated` | - | - | `ListAccountsResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists all the accounts in the organization. To request only the accounts in a specified root or organizational unit (OU), use the ListAccountsForParent operation instead. When calling List* operations, always check t ... |
| `ListAccountsForParent` | `-` | `paginated` | `ParentId` | - | `ListAccountsForParentResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that aren't in any OU. If you specify an ... |
| `ListAccountsWithInvalidEffectivePolicy` | `-` | `paginated` | `PolicyType` | - | `ListAccountsWithInvalidEffectivePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `EffectivePolicyNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists all the accounts in an organization that have invalid effective policies. An invalid effective policy is an effective policy that fails validation checks, resulting in the effective policy not being fully enfor ... |
| `ListAWSServiceAccessForOrganization` | `-` | `paginated` | - | - | `ListAWSServiceAccessForOrganizationResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Returns a list of the Amazon Web Services services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operati ... |
| `ListChildren` | `-` | `paginated` | `ParentId`, `ChildType` | - | `ListChildrenResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists all of the organizational units (OUs) or accounts that are contained in the specified parent OU or root. This operation, along with ListParents enables you to traverse the tree structure that makes up this root ... |
| `ListCreateAccountStatus` | `-` | `paginated` | - | - | `ListCreateAccountStatusResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists the account creation requests that match the specified status that is currently being tracked for the organization. When calling List* operations, always check the NextToken response parameter value, even if yo ... |
| `ListDelegatedAdministrators` | `-` | `paginated` | - | - | `ListDelegatedAdministratorsResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists the Amazon Web Services accounts that are designated as delegated administrators in this organization. You can only call this operation from the management account or a member account that is a delegated admini ... |
| `ListDelegatedServicesForAccount` | `-` | `paginated` | `AccountId` | - | `ListDelegatedServicesForAccountResponse` | `AccessDeniedException`, `AccountNotFoundException`, `AccountNotRegisteredException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | List the Amazon Web Services services for which the specified account is a delegated administrator. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `ListEffectivePolicyValidationErrors` | `-` | `paginated` | `AccountId`, `PolicyType` | - | `ListEffectivePolicyValidationErrorsResponse` | `AccessDeniedException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `EffectivePolicyNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists all the validation errors on an effective policy for a specified account and policy type. You can only call this operation from the management account or a member account that is a delegated administrator. |
| `ListHandshakesForAccount` | `-` | `paginated` | - | - | `ListHandshakesForAccountResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the recent handshakes that you have received. You can view CANCELED , ACCEPTED , DECLINED , or EXPIRED handshakes in API responses for 30 days before they are deleted. You can call this operation from any accou ... |
| `ListHandshakesForOrganization` | `-` | `paginated` | - | - | `ListHandshakesForOrganizationResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the recent handshakes that you have sent. You can view CANCELED , ACCEPTED , DECLINED , or EXPIRED handshakes in API responses for 30 days before they are deleted. You can only call this operation from the mana ... |
| `ListInboundResponsibilityTransfers` | `-` | - | `Type` | - | `ListInboundResponsibilityTransfersResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `InvalidInputException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists transfers that allow you to manage the specified responsibilities for another organization. This operation returns both transfer invitations and transfers. When calling List* operations, always check the NextTo ... |
| `ListOrganizationalUnitsForParent` | `-` | `paginated` | `ParentId` | - | `ListOrganizationalUnitsForParentResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ParentNotFoundException`, `ServiceException`, `TooManyRequestsException` | Lists the organizational units (OUs) in a parent organizational unit or root. When calling List* operations, always check the NextToken response parameter value, even if you receive an empty result set. These operati ... |
| `ListOutboundResponsibilityTransfers` | `-` | - | `Type` | - | `ListOutboundResponsibilityTransfersResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists transfers that allow an account outside your organization to manage the specified responsibilities for your organization. This operation returns both transfer invitations and transfers. When calling List* opera ... |
| `ListParents` | `-` | `paginated` | `ChildId` | - | `ListParentsResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ChildNotFoundException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with ListChildren enables you to traverse the tree structure that makes up t ... |
| `ListPolicies` | `-` | `paginated` | `Filter` | - | `ListPoliciesResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Retrieves the list of all policies in an organization of a specified type. When calling List* operations, always check the NextToken response parameter value, even if you receive an empty result set. These operations ... |
| `ListPoliciesForTarget` | `-` | `paginated` | `TargetId`, `Filter` | - | `ListPoliciesForTargetResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list. When calling List* opera ... |
| `ListRoots` | `-` | `paginated` | - | - | `ListRootsResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException` | Lists the roots that are defined in the current organization. When calling List* operations, always check the NextToken response parameter value, even if you receive an empty result set. These operations can occasion ... |
| `ListTagsForResource` | `-` | `paginated` | `ResourceId` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException` | Lists tags that are attached to the specified resource. You can attach tags to the following resources in Organizations. Amazon Web Services account Organization root Organizational unit (OU) Policy (any type) You ca ... |
| `ListTargetsForPolicy` | `-` | `paginated` | `PolicyId` | - | `ListTargetsForPolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `InvalidInputException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Lists all the roots, organizational units (OUs), and accounts that the specified policy is attached to. When calling List* operations, always check the NextToken response parameter value, even if you receive an empty ... |
| `MoveAccount` | `-` | - | `AccountId`, `SourceParentId`, `DestinationParentId` | - | `Unit` | `AccessDeniedException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `DestinationParentNotFoundException`, `DuplicateAccountException`, `InvalidInputException`, `ServiceException`, `SourceParentNotFoundException`, `TooManyRequestsException` | Moves an account from its current source parent root or organizational unit (OU) to the specified destination parent root or OU. You can only call this operation from the management account. |
| `PutResourcePolicy` | `-` | - | `Content` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Creates or updates a resource policy. You can only call this operation from the management account.. |
| `RegisterDelegatedAdministrator` | `-` | - | `AccountId`, `ServicePrincipal` | - | `Unit` | `AccessDeniedException`, `AccountAlreadyRegisteredException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Enables the specified member account to administer the Organizations features of the specified Amazon Web Services service. It grants read-only access to Organizations service data. The account still requires IAM per ... |
| `RemoveAccountFromOrganization` | `-` | - | `AccountId` | - | `Unit` | `AccessDeniedException`, `AccountNotFoundException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `MasterCannotLeaveOrganizationException`, `ServiceException`, `TooManyRequestsException` | Removes the specified account from the organization. The removed account becomes a standalone account that isn't a member of any organization. It's no longer subject to any policies and is responsible for its own bil ... |
| `TagResource` | `-` | - | `ResourceId`, `Tags` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException` | Adds one or more tags to the specified resource. Currently, you can attach tags to the following resources in Organizations. Amazon Web Services account Organization root Organizational unit (OU) Policy (any type) Yo ... |
| `TerminateResponsibilityTransfer` | `-` | - | `Id` | - | `TerminateResponsibilityTransferResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `InvalidResponsibilityTransferTransitionException`, `ResponsibilityTransferAlreadyInStatusException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Ends a transfer. A transfer is an arrangement between two management accounts where one account designates the other with specified responsibilities for their organization. |
| `UntagResource` | `-` | - | `ResourceId`, `TagKeys` | - | `Unit` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `InvalidInputException`, `ServiceException`, `TargetNotFoundException`, `TooManyRequestsException` | Removes any tags with the specified keys from the specified resource. You can attach tags to the following resources in Organizations. Amazon Web Services account Organization root Organizational unit (OU) Policy (an ... |
| `UpdateOrganizationalUnit` | `-` | - | `OrganizationalUnitId` | - | `UpdateOrganizationalUnitResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `DuplicateOrganizationalUnitException`, `InvalidInputException`, `OrganizationalUnitNotFoundException`, `ServiceException`, `TooManyRequestsException` | Renames the specified organizational unit (OU). The ID and ARN don't change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached. You can only call this operation from the ... |
| `UpdatePolicy` | `-` | - | `PolicyId` | - | `UpdatePolicyResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConcurrentModificationException`, `ConstraintViolationException`, `DuplicatePolicyException`, `InvalidInputException`, `MalformedPolicyDocumentException`, `PolicyChangesInProgressException`, `PolicyNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Updates an existing policy with a new name, description, or content. If you don't supply any parameter, that value remains unchanged. You can't change a policy's type. You can only call this operation from the manage ... |
| `UpdateResponsibilityTransfer` | `-` | - | `Id`, `Name` | - | `UpdateResponsibilityTransferResponse` | `AccessDeniedException`, `AWSOrganizationsNotInUseException`, `ConstraintViolationException`, `InvalidInputException`, `ResponsibilityTransferNotFoundException`, `ServiceException`, `TooManyRequestsException`, `UnsupportedAPIEndpointException` | Updates a transfer. A transfer is the arrangement between two management accounts where one account designates the other with specified responsibilities for their organization. You can update the name assigned to a t ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AWSOrganizationsNotInUseException` | `structure` | Message | Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization. |
| `AccessDeniedException` | `structure` | Message | You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy att ... |
| `AccessDeniedForDependencyException` | `structure` | Message, Reason | The operation that you attempted requires you to have the iam:CreateServiceLinkedRole for organizations.amazonaws.com permission so that Organizations can c ... |
| `AccountAlreadyClosedException` | `structure` | Message | You attempted to close an account that is already closed. |
| `AccountAlreadyRegisteredException` | `structure` | Message | The specified account is already a delegated administrator for this Amazon Web Services service. |
| `AccountNotFoundException` | `structure` | Message | We can't find an Amazon Web Services account with the AccountId that you specified, or the account whose credentials you used to make this request isn't a m ... |
| `AccountNotRegisteredException` | `structure` | Message | The specified account is not a delegated administrator for this Amazon Web Services service. |
| `AccountOwnerNotVerifiedException` | `structure` | Message | You can't invite an existing account to your organization until you verify that you own the email address associated with the management account. For more i ... |
| `AlreadyInOrganizationException` | `structure` | Message | This account is already a member of an organization. An account can belong to only one organization at a time. |
| `ChildNotFoundException` | `structure` | Message | We can't find an organizational unit (OU) or Amazon Web Services account with the ChildId that you specified. |
| `ConcurrentModificationException` | `structure` | Message | The target of the operation is currently being modified by a different request. Try again later. |
| `ConflictException` | `structure` | Message | The request failed because it conflicts with the current state of the specified resource. |
| `ConstraintViolationException` | `structure` | Message, Reason | Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or r ... |
| `CreateAccountStatusNotFoundException` | `structure` | Message | We can't find an create account request with the CreateAccountRequestId that you specified. |
| `DestinationParentNotFoundException` | `structure` | Message | We can't find the destination container (a root or OU) with the ParentId that you specified. |
| `DuplicateAccountException` | `structure` | Message | That account is already present in the specified destination. |
| `DuplicateHandshakeException` | `structure` | Message | A handshake with the same action and target already exists. For example, if you invited an account to join your organization, the invited account might alre ... |
| `DuplicateOrganizationalUnitException` | `structure` | Message | An OU with the same name already exists. |
| `DuplicatePolicyAttachmentException` | `structure` | Message | The selected policy is already attached to the specified target. |
| `DuplicatePolicyException` | `structure` | Message | A policy with the same name already exists. |
| `EffectivePolicyNotFoundException` | `structure` | Message | If you ran this action on the management account, this policy type is not enabled. If you ran the action on a member account, the account doesn't have an ef ... |
| `FinalizingOrganizationException` | `structure` | Message | Organizations couldn't perform the operation because your organization hasn't finished initializing. This can take up to an hour. Try again later. If after ... |
| `HandshakeAlreadyInStateException` | `structure` | Message | The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted. |
| `HandshakeConstraintViolationException` | `structure` | Message, Reason | The requested operation would violate the constraint identified in the reason code. Some of the reasons in the following list might not be applicable to thi ... |
| `HandshakeNotFoundException` | `structure` | Message | We can't find a handshake with the HandshakeId that you specified. |
| `InvalidHandshakeTransitionException` | `structure` | Message | You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a hands ... |
| `InvalidInputException` | `structure` | Message, Reason | The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains ... |
| `InvalidResponsibilityTransferTransitionException` | `structure` | Message | The responsibility transfer can't transition to the requested state because it's not in a valid state for this operation. |
| `MalformedPolicyDocumentException` | `structure` | Message | The provided policy document doesn't meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about servi ... |
| `MasterCannotLeaveOrganizationException` | `structure` | Message | You can't remove a management account from an organization. If you want the management account to become a member account in another organization, you must ... |
| `OrganizationNotEmptyException` | `structure` | Message | The organization isn't empty. To delete an organization, you must first remove all accounts except the management account. |
| `OrganizationalUnitNotEmptyException` | `structure` | Message | The specified OU is not empty. Move all accounts to another root or to other OUs, remove all child OUs, and try the operation again. |
| `OrganizationalUnitNotFoundException` | `structure` | Message | We can't find an OU with the OrganizationalUnitId that you specified. |
| `ParentNotFoundException` | `structure` | Message | We can't find a root or OU with the ParentId that you specified. |
| `PolicyChangesInProgressException` | `structure` | Message | Changes to the effective policy are in progress, and its contents can't be returned. Try the operation again later. |
| `PolicyInUseException` | `structure` | Message | The policy is attached to one or more entities. You must detach it from all roots, OUs, and accounts before performing this operation. |
| `PolicyNotAttachedException` | `structure` | Message | The policy isn't attached to the specified target in the specified root. |
| `PolicyNotFoundException` | `structure` | Message | We can't find a policy with the PolicyId that you specified. |
| `PolicyTypeAlreadyEnabledException` | `structure` | Message | The specified policy type is already enabled in the specified root. |
| `PolicyTypeNotAvailableForOrganizationException` | `structure` | Message | You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable SCPs only after you enable ... |
| `PolicyTypeNotEnabledException` | `structure` | Message | The specified policy type isn't currently enabled in this root. You can't attach policies of the specified type to entities in a root until you enable that ... |
| `ResourcePolicyNotFoundException` | `structure` | Message | We can't find a resource policy request with the parameter that you specified. |
| `ResponsibilityTransferAlreadyInStatusException` | `structure` | Message | The responsibility transfer is already in the status that you specified. |
| `ResponsibilityTransferNotFoundException` | `structure` | Message | We can't find a transfer that you specified. |
| `RootNotFoundException` | `structure` | Message | We can't find a root with the RootId that you specified. |
| `ServiceException` | `structure` | Message | Organizations can't complete your request because of an internal service error. Try again later. |
| `SourceParentNotFoundException` | `structure` | Message | We can't find a source root or OU with the ParentId that you specified. |
| `TargetNotFoundException` | `structure` | Message | We can't find a root, OU, account, or policy with the TargetId that you specified. |
| `TooManyRequestsException` | `structure` | Type, Message | You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later. For information a ... |
| `UnsupportedAPIEndpointException` | `structure` | Message | This action isn't available in the current Amazon Web Services Region. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

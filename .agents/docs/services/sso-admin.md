# AWS Single Sign-On Admin

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IAM Identity Center is the Amazon Web Services solution for connecting your workforce users to Amazon Web Services managed applications and other Amazon Web Services resources. You can connect your existing identity provider and synchronize users and groups from your directory, or create and manage your users directly in IAM Identity Center. You can then use IAM Identity Center for either or both of the following: User access to applications User access to Amazon Web Services accounts This guide provides information about single sign-on operations that you can use for access to applications and Amazon Web Services accounts. For information about IAM Identity Center features, see the IAM Identity Center User Guide. IAM Identity Center uses the `sso` and `identitystore` API namespaces.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Single Sign-On Admin where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented AWS Single Sign-On Admin workflows in the local mock. Key resources include `ApplicationAccessScopeResource`, `ApplicationAuthenticationMethodResource`, `ApplicationGrantResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Describe`, `Create`, `Get` operation families, including `ListAccountAssignmentCreationStatus`, `ListAccountAssignmentDeletionStatus`, `ListAccountAssignments`, `ListAccountAssignmentsForPrincipal`, `DeleteAccountAssignment`, `DeleteApplication`.

## Service Identity and Protocol

- AWS model slug: `sso-admin`
- AWS SDK for Rust slug: `ssoadmin`
- Model version: `2020-07-20`
- Model file: `vendor/api-models-aws/models/sso-admin/service/2020-07-20/sso-admin-2020-07-20.json`
- SDK ID: `SSO Admin`
- Endpoint prefix: `sso`
- ARN namespace: `sso`
- CloudFormation name: `SSO`
- CloudTrail event source: `sso.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (21), `Delete` (12), `Describe` (11), `Create` (7), `Get` (7), `Put` (7), `Update` (5), `Attach` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddRegion`, `AttachCustomerManagedPolicyReferenceToPermissionSet`, `AttachManagedPolicyToPermissionSet`, `CreateAccountAssignment`, `CreateApplication`, `CreateApplicationAssignment`, `CreateInstance`, `CreateInstanceAccessControlAttributeConfiguration`, `CreatePermissionSet`, `CreateTrustedTokenIssuer`, `DeleteAccountAssignment`, `DeleteApplication`, `DeleteApplicationAccessScope`, `DeleteApplicationAssignment`, `DeleteApplicationAuthenticationMethod`, `DeleteApplicationGrant`, `DeleteInlinePolicyFromPermissionSet`, `DeleteInstance`, `DeleteInstanceAccessControlAttributeConfiguration`, `DeletePermissionSet`, ... (+19).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAssignmentCreationStatus`, `DescribeAccountAssignmentDeletionStatus`, `DescribeApplication`, `DescribeApplicationAssignment`, `DescribeApplicationProvider`, `DescribeInstance`, `DescribeInstanceAccessControlAttributeConfiguration`, `DescribePermissionSet`, `DescribePermissionSetProvisioningStatus`, `DescribeRegion`, `DescribeTrustedTokenIssuer`, `GetApplicationAccessScope`, `GetApplicationAssignmentConfiguration`, `GetApplicationAuthenticationMethod`, `GetApplicationGrant`, `GetApplicationSessionConfiguration`, `GetInlinePolicyForPermissionSet`, `GetPermissionsBoundaryForPermissionSet`, `ListAccountAssignmentCreationStatus`, `ListAccountAssignmentDeletionStatus`, ... (+19).
- Pagination is modelled for 21 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 12 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 79 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApplicationAccessScopeResource` | `ApplicationArn`, `Scope` | put: `PutApplicationAccessScope`; read: `GetApplicationAccessScope`; delete: `DeleteApplicationAccessScope`; list: `ListApplicationAccessScopes` | - | - |
| `ApplicationAuthenticationMethodResource` | `ApplicationArn`, `AuthenticationMethodType` | put: `PutApplicationAuthenticationMethod`; read: `GetApplicationAuthenticationMethod`; delete: `DeleteApplicationAuthenticationMethod`; list: `ListApplicationAuthenticationMethods` | - | - |
| `ApplicationGrantResource` | `ApplicationArn`, `GrantType` | put: `PutApplicationGrant`; read: `GetApplicationGrant`; delete: `DeleteApplicationGrant`; list: `ListApplicationGrants` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/singlesignon/latest/userguide/permissionsetsconcept.html
- https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_CreateAccountAssignment.html
- https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html
- https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_PermissionSetProvisioningStatus.html

Research outcomes:
- A permission set is a template that defines one or more IAM policies for IAM Identity Center access.
- Assigning a permission set to a user or group for an AWS account creates Identity Center-controlled IAM roles in that target account and attaches policies to those roles.
- Permission sets can include AWS managed policies, customer managed policies, inline policies, job-function policies, and permissions boundaries.
- The same user can receive multiple permission sets for the same or different accounts.
- `CreateAccountAssignment` assigns a user or group principal to an account using a permission set and automatically provisions the permission set into the account as an IAM policy on the Identity Center role.
- `CreateAccountAssignment` is asynchronous; callers use `DescribeAccountAssignmentCreationStatus` to track the request.
- Updating a permission set does not automatically update already provisioned IAM policies and roles. `ProvisionPermissionSet` must be called to push changes.
- `ProvisionPermissionSet` can target one `AWS_ACCOUNT` or `ALL_PROVISIONED_ACCOUNTS`.
- `PermissionSetProvisioningStatus` exposes account id, created date, failure reason, permission set ARN, request id, and a status of `IN_PROGRESS`, `FAILED`, or `SUCCEEDED`.

Parity implications:
- Model instances, permission sets, attached/inline policies, permissions boundaries, account assignments, provisioning requests, and generated account-role state separately.
- Assignment creation and permission-set provisioning should be asynchronous and observable through status operations.
- Permission set updates should not mutate existing account-role policy state until explicit provisioning occurs.

## Operation Groups

### List

- Operations: `ListAccountAssignmentCreationStatus`, `ListAccountAssignmentDeletionStatus`, `ListAccountAssignments`, `ListAccountAssignmentsForPrincipal`, `ListAccountsForProvisionedPermissionSet`, `ListApplicationAssignments`, `ListApplicationAssignmentsForPrincipal`, `ListApplicationProviders`, `ListApplications`, `ListCustomerManagedPolicyReferencesInPermissionSet`, `ListInstances`, `ListManagedPoliciesInPermissionSet`, `ListPermissionSetProvisioningStatus`, `ListPermissionSets`, `ListPermissionSetsProvisionedToAccount`, `ListRegions`, `ListTagsForResource`, `ListTrustedTokenIssuers`
- Traits: `paginated` (18), `readonly` (2)
- Common required input members in this group: `InstanceArn`, `AccountId`, `PermissionSetArn`, `PrincipalId`, `PrincipalType`

### Describe

- Operations: `DescribeAccountAssignmentCreationStatus`, `DescribeAccountAssignmentDeletionStatus`, `DescribeApplication`, `DescribeApplicationAssignment`, `DescribeApplicationProvider`, `DescribeInstance`, `DescribeInstanceAccessControlAttributeConfiguration`, `DescribePermissionSet`, `DescribePermissionSetProvisioningStatus`, `DescribeRegion`, `DescribeTrustedTokenIssuer`
- Common required input members in this group: `InstanceArn`, `ApplicationArn`

### Delete

- Operations: `DeleteAccountAssignment`, `DeleteApplication`, `DeleteApplicationAssignment`, `DeleteInlinePolicyFromPermissionSet`, `DeleteInstance`, `DeleteInstanceAccessControlAttributeConfiguration`, `DeletePermissionsBoundaryFromPermissionSet`, `DeletePermissionSet`, `DeleteTrustedTokenIssuer`
- Traits: `idempotent` (1)
- Common required input members in this group: `InstanceArn`, `PermissionSetArn`, `PrincipalType`, `PrincipalId`, `ApplicationArn`

### Create

- Operations: `CreateAccountAssignment`, `CreateApplication`, `CreateApplicationAssignment`, `CreateInstance`, `CreateInstanceAccessControlAttributeConfiguration`, `CreatePermissionSet`, `CreateTrustedTokenIssuer`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `InstanceArn`, `PrincipalType`, `PrincipalId`, `Name`

### Update

- Operations: `UpdateApplication`, `UpdateInstance`, `UpdateInstanceAccessControlAttributeConfiguration`, `UpdatePermissionSet`, `UpdateTrustedTokenIssuer`
- Common required input members in this group: `InstanceArn`

### Get

- Operations: `GetApplicationAssignmentConfiguration`, `GetApplicationSessionConfiguration`, `GetInlinePolicyForPermissionSet`, `GetPermissionsBoundaryForPermissionSet`
- Traits: `readonly` (2)
- Common required input members in this group: `ApplicationArn`, `InstanceArn`, `PermissionSetArn`

### Put

- Operations: `PutApplicationAssignmentConfiguration`, `PutApplicationSessionConfiguration`, `PutInlinePolicyToPermissionSet`, `PutPermissionsBoundaryToPermissionSet`
- Traits: `idempotent` (2)
- Common required input members in this group: `ApplicationArn`, `InstanceArn`, `PermissionSetArn`

### Attach

- Operations: `AttachCustomerManagedPolicyReferenceToPermissionSet`, `AttachManagedPolicyToPermissionSet`
- Common required input members in this group: `InstanceArn`, `PermissionSetArn`

### Detach

- Operations: `DetachCustomerManagedPolicyReferenceFromPermissionSet`, `DetachManagedPolicyFromPermissionSet`
- Common required input members in this group: `InstanceArn`, `PermissionSetArn`

### Add

- Operations: `AddRegion`
- Common required input members in this group: -

### Provision

- Operations: `ProvisionPermissionSet`
- Common required input members in this group: -

### Remove

- Operations: `RemoveRegion`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddRegion` | `-` | - | `InstanceArn`, `RegionName` | - | `AddRegionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a Region to an IAM Identity Center instance. This operation initiates an asynchronous workflow to replicate the IAM Identity Center instance to the target Region. The Region status is set to ADDING at first and ... |
| `AttachCustomerManagedPolicyReferenceToPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `CustomerManagedPolicyReference` | - | `AttachCustomerManagedPolicyReferenceToPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Attaches the specified customer managed policy to the specified PermissionSet . |
| `AttachManagedPolicyToPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `ManagedPolicyArn` | - | `AttachManagedPolicyToPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Attaches an Amazon Web Services managed policy ARN to a permission set. If the permission set is already referenced by one or more account assignments, you will need to call ProvisionPermissionSet after this operatio ... |
| `CreateAccountAssignment` | `-` | - | `InstanceArn`, `TargetId`, `TargetType`, `PermissionSetArn`, `PrincipalType`, `PrincipalId` | - | `CreateAccountAssignmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Assigns access to a principal for a specified Amazon Web Services account using a specified permission set. The term principal here refers to a user or group that is defined in IAM Identity Center. As part of a succe ... |
| `CreateApplication` | `-` | `idempotency-token` | `InstanceArn`, `ApplicationProviderArn`, `Name` | `ClientToken` | `CreateApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an OAuth 2.0 customer managed application in IAM Identity Center for the given application provider. This API does not support creating SAML 2.0 customer managed applications or Amazon Web Services managed ap ... |
| `CreateApplicationAssignment` | `-` | - | `ApplicationArn`, `PrincipalId`, `PrincipalType` | - | `CreateApplicationAssignmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Grant application access to a user or group. |
| `CreateInstance` | `-` | `idempotency-token` | - | `ClientToken` | `CreateInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an instance of IAM Identity Center for a standalone Amazon Web Services account that is not managed by Organizations or a member Amazon Web Services account in an organization. You can create only one instanc ... |
| `CreateInstanceAccessControlAttributeConfiguration` | `-` | - | `InstanceArn`, `InstanceAccessControlAttributeConfiguration` | - | `CreateInstanceAccessControlAttributeConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables the attributes-based access control (ABAC) feature for the specified IAM Identity Center instance. You can also specify new attributes to add to your ABAC configuration during the enabling process. For more i ... |
| `CreatePermissionSet` | `-` | - | `Name`, `InstanceArn` | - | `CreatePermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a permission set within a specified IAM Identity Center instance. To grant users and groups access to Amazon Web Services account resources, use CreateAccountAssignment . |
| `CreateTrustedTokenIssuer` | `-` | `idempotency-token` | `InstanceArn`, `Name`, `TrustedTokenIssuerType`, `TrustedTokenIssuerConfiguration` | `ClientToken` | `CreateTrustedTokenIssuerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a connection to a trusted token issuer in an instance of IAM Identity Center. A trusted token issuer enables trusted identity propagation to be used with applications that authenticate outside of Amazon Web S ... |
| `DeleteAccountAssignment` | `-` | - | `InstanceArn`, `TargetId`, `TargetType`, `PermissionSetArn`, `PrincipalType`, `PrincipalId` | - | `DeleteAccountAssignmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a principal's access from a specified Amazon Web Services account using a specified permission set. After a successful response, call DescribeAccountAssignmentDeletionStatus to describe the status of an assig ... |
| `DeleteApplication` | `-` | - | `ApplicationArn` | - | `DeleteApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the association with the application. The connected service resource still exists. |
| `DeleteApplicationAssignment` | `-` | `idempotent` | `ApplicationArn`, `PrincipalId`, `PrincipalType` | - | `DeleteApplicationAssignmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Revoke application access to an application by deleting application assignments for a user or group. |
| `DeleteInlinePolicyFromPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `DeleteInlinePolicyFromPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the inline policy from a specified permission set. |
| `DeleteInstance` | `-` | - | `InstanceArn` | - | `DeleteInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the instance of IAM Identity Center. Only the account that owns the instance can call this API. Neither the delegated administrator nor member account can delete the organization instance, but those roles can ... |
| `DeleteInstanceAccessControlAttributeConfiguration` | `-` | - | `InstanceArn` | - | `DeleteInstanceAccessControlAttributeConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disables the attributes-based access control (ABAC) feature for the specified IAM Identity Center instance and deletes all of the attribute mappings that have been configured. Once deleted, any attributes that are re ... |
| `DeletePermissionsBoundaryFromPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `DeletePermissionsBoundaryFromPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the permissions boundary from a specified PermissionSet . |
| `DeletePermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `DeletePermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified permission set. |
| `DeleteTrustedTokenIssuer` | `-` | - | `TrustedTokenIssuerArn` | - | `DeleteTrustedTokenIssuerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a trusted token issuer configuration from an instance of IAM Identity Center. Deleting this trusted token issuer configuration will cause users to lose access to any applications that are configured to use th ... |
| `DescribeAccountAssignmentCreationStatus` | `-` | - | `InstanceArn`, `AccountAssignmentCreationRequestId` | - | `DescribeAccountAssignmentCreationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the status of the assignment creation request. |
| `DescribeAccountAssignmentDeletionStatus` | `-` | - | `InstanceArn`, `AccountAssignmentDeletionRequestId` | - | `DescribeAccountAssignmentDeletionStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the status of the assignment deletion request. |
| `DescribeApplication` | `-` | - | `ApplicationArn` | - | `DescribeApplicationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of an application associated with an instance of IAM Identity Center. |
| `DescribeApplicationAssignment` | `-` | - | `ApplicationArn`, `PrincipalId`, `PrincipalType` | - | `DescribeApplicationAssignmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a direct assignment of a user or group to an application. If the user doesn’t have a direct assignment to the application, the user may still have access to the application through a group. Therefore, don’t ... |
| `DescribeApplicationProvider` | `-` | - | `ApplicationProviderArn` | - | `DescribeApplicationProviderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a provider that can be used to connect an Amazon Web Services managed application or customer managed application to IAM Identity Center. |
| `DescribeInstance` | `-` | - | `InstanceArn` | - | `DescribeInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the details of an instance of IAM Identity Center. The status can be one of the following: CREATE_IN_PROGRESS - The instance is in the process of being created. When the instance is ready for use, DescribeIns ... |
| `DescribeInstanceAccessControlAttributeConfiguration` | `-` | - | `InstanceArn` | - | `DescribeInstanceAccessControlAttributeConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the list of IAM Identity Center identity store attributes that have been configured to work with attributes-based access control (ABAC) for the specified IAM Identity Center instance. This will not return att ... |
| `DescribePermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `DescribePermissionSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of the permission set. |
| `DescribePermissionSetProvisioningStatus` | `-` | - | `InstanceArn`, `ProvisionPermissionSetRequestId` | - | `DescribePermissionSetProvisioningStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the status for the given permission set provisioning request. |
| `DescribeRegion` | `-` | - | `InstanceArn`, `RegionName` | - | `DescribeRegionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a specific Region enabled in an IAM Identity Center instance. Details include the Region name, current status (ACTIVE, ADDING, or REMOVING), the date when the Region was added, and whether it ... |
| `DescribeTrustedTokenIssuer` | `-` | - | `TrustedTokenIssuerArn` | - | `DescribeTrustedTokenIssuerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a trusted token issuer configuration stored in an instance of IAM Identity Center. Details include the name of the trusted token issuer, the issuer URL, and the path of the source attribute an ... |
| `DetachCustomerManagedPolicyReferenceFromPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `CustomerManagedPolicyReference` | - | `DetachCustomerManagedPolicyReferenceFromPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Detaches the specified customer managed policy from the specified PermissionSet . |
| `DetachManagedPolicyFromPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `ManagedPolicyArn` | - | `DetachManagedPolicyFromPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Detaches the attached Amazon Web Services managed policy ARN from the specified permission set. |
| `GetApplicationAssignmentConfiguration` | `-` | `readonly` | `ApplicationArn` | - | `GetApplicationAssignmentConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration of PutApplicationAssignmentConfiguration . |
| `GetApplicationSessionConfiguration` | `-` | `readonly` | `ApplicationArn` | - | `GetApplicationSessionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the session configuration for an application in IAM Identity Center. The session configuration determines how users can access an application. This includes whether user background sessions are enabled. Use ... |
| `GetInlinePolicyForPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `GetInlinePolicyForPermissionSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Obtains the inline policy assigned to the permission set. |
| `GetPermissionsBoundaryForPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `GetPermissionsBoundaryForPermissionSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Obtains the permissions boundary for a specified PermissionSet . |
| `ListAccountAssignmentCreationStatus` | `-` | `paginated` | `InstanceArn` | - | `ListAccountAssignmentCreationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the status of the Amazon Web Services account assignment creation requests for a specified IAM Identity Center instance. |
| `ListAccountAssignmentDeletionStatus` | `-` | `paginated` | `InstanceArn` | - | `ListAccountAssignmentDeletionStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the status of the Amazon Web Services account assignment deletion requests for a specified IAM Identity Center instance. |
| `ListAccountAssignments` | `-` | `paginated` | `InstanceArn`, `AccountId`, `PermissionSetArn` | - | `ListAccountAssignmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the assignee of the specified Amazon Web Services account with the specified permission set. |
| `ListAccountAssignmentsForPrincipal` | `-` | `paginated` | `InstanceArn`, `PrincipalId`, `PrincipalType` | - | `ListAccountAssignmentsForPrincipalResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of the IAM Identity Center associated Amazon Web Services accounts that the principal has access to. This action must be called from the management account containing your organization instance of IA ... |
| `ListAccountsForProvisionedPermissionSet` | `-` | `paginated` | `InstanceArn`, `PermissionSetArn` | - | `ListAccountsForProvisionedPermissionSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the Amazon Web Services accounts where the specified permission set is provisioned. |
| `ListApplicationAssignments` | `-` | `readonly`, `paginated` | `ApplicationArn` | - | `ListApplicationAssignmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists Amazon Web Services account users that are assigned to an application. |
| `ListApplicationAssignmentsForPrincipal` | `-` | `readonly`, `paginated` | `InstanceArn`, `PrincipalId`, `PrincipalType` | - | `ListApplicationAssignmentsForPrincipalResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the applications to which a specified principal is assigned. You must provide a filter when calling this action from a member account against your organization instance of IAM Identity Center. A filter is not r ... |
| `ListApplicationProviders` | `-` | `paginated` | - | - | `ListApplicationProvidersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the application providers configured in the IAM Identity Center identity store. |
| `ListApplications` | `-` | `paginated` | `InstanceArn` | - | `ListApplicationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all applications associated with the instance of IAM Identity Center. When listing applications for an organization instance in the management account, member accounts must use the applicationAccount parameter ... |
| `ListCustomerManagedPolicyReferencesInPermissionSet` | `-` | `paginated` | `InstanceArn`, `PermissionSetArn` | - | `ListCustomerManagedPolicyReferencesInPermissionSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all customer managed policies attached to a specified PermissionSet . |
| `ListInstances` | `-` | `paginated` | - | - | `ListInstancesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the details of the organization and account instances of IAM Identity Center that were created in or visible to the account calling this API. |
| `ListManagedPoliciesInPermissionSet` | `-` | `paginated` | `InstanceArn`, `PermissionSetArn` | - | `ListManagedPoliciesInPermissionSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the Amazon Web Services managed policy that is attached to a specified permission set. |
| `ListPermissionSetProvisioningStatus` | `-` | `paginated` | `InstanceArn` | - | `ListPermissionSetProvisioningStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the status of the permission set provisioning requests for a specified IAM Identity Center instance. |
| `ListPermissionSets` | `-` | `paginated` | `InstanceArn` | - | `ListPermissionSetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the PermissionSet s in an IAM Identity Center instance. |
| `ListPermissionSetsProvisionedToAccount` | `-` | `paginated` | `InstanceArn`, `AccountId` | - | `ListPermissionSetsProvisionedToAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the permission sets that are provisioned to a specified Amazon Web Services account. |
| `ListRegions` | `-` | `paginated` | `InstanceArn` | - | `ListRegionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all enabled Regions of an IAM Identity Center instance, including those that are being added or removed. This operation returns Regions with ACTIVE, ADDING, or REMOVING status. The following actions are related ... |
| `ListTagsForResource` | `-` | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags that are attached to a specified resource. |
| `ListTrustedTokenIssuers` | `-` | `paginated` | `InstanceArn` | - | `ListTrustedTokenIssuersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all the trusted token issuers configured in an instance of IAM Identity Center. |
| `ProvisionPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `TargetType` | - | `ProvisionPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The process by which a specified permission set is provisioned to the specified target. |
| `PutApplicationAssignmentConfiguration` | `-` | `idempotent` | `ApplicationArn`, `AssignmentRequired` | - | `PutApplicationAssignmentConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Configure how users gain access to an application. If AssignmentsRequired is true (default value), users don’t have access to the application unless an assignment is created using the CreateApplicationAssignment API ... |
| `PutApplicationSessionConfiguration` | `-` | `idempotent` | `ApplicationArn` | - | `PutApplicationSessionConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the session configuration for an application in IAM Identity Center. The session configuration determines how users can access an application. This includes whether user background sessions are enabled. User ... |
| `PutInlinePolicyToPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `InlinePolicy` | - | `PutInlinePolicyToPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Attaches an inline policy to a permission set. If the permission set is already referenced by one or more account assignments, you will need to call ProvisionPermissionSet after this action to apply the corresponding ... |
| `PutPermissionsBoundaryToPermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn`, `PermissionsBoundary` | - | `PutPermissionsBoundaryToPermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches an Amazon Web Services managed or customer managed policy to the specified PermissionSet as a permissions boundary. |
| `RemoveRegion` | `-` | - | `InstanceArn`, `RegionName` | - | `RemoveRegionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes an additional Region from an IAM Identity Center instance. This operation initiates an asynchronous workflow to clean up IAM Identity Center resources in the specified additional Region. The Region status is ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a set of tags with a specified resource. |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a set of tags from a specified resource. |
| `UpdateApplication` | `-` | - | `ApplicationArn` | - | `UpdateApplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates application properties. |
| `UpdateInstance` | `-` | - | `InstanceArn` | - | `UpdateInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the details for the instance of IAM Identity Center that is owned by the Amazon Web Services account. |
| `UpdateInstanceAccessControlAttributeConfiguration` | `-` | - | `InstanceArn`, `InstanceAccessControlAttributeConfiguration` | - | `UpdateInstanceAccessControlAttributeConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the IAM Identity Center identity store attributes that you can use with the IAM Identity Center instance for attributes-based access control (ABAC). When using an external identity provider as an identity sou ... |
| `UpdatePermissionSet` | `-` | - | `InstanceArn`, `PermissionSetArn` | - | `UpdatePermissionSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing permission set. |
| `UpdateTrustedTokenIssuer` | `-` | - | `TrustedTokenIssuerArn` | - | `UpdateTrustedTokenIssuerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the name of the trusted token issuer, or the path of a source attribute or destination attribute for a trusted token issuer configuration. Updating this trusted token issuer configuration might cause users to ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, Reason | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message | Occurs when a conflict with a previous successful write is detected. This generally occurs when the previous write did not have time to propagate to the hos ... |
| `InternalServerException` | `structure` | Message | The request processing has failed because of an unknown error, exception, or failure with an internal server. |
| `ResourceNotFoundException` | `structure` | Message, Reason | Indicates that a requested resource is not found. |
| `ServiceQuotaExceededException` | `structure` | Message | Indicates that the principal has crossed the permitted number of resources that can be created. |
| `ThrottlingException` | `structure` | Message, Reason | Indicates that the principal has crossed the throttling limits of the API operations. |
| `ValidationException` | `structure` | Message, Reason | The request failed because it contains a syntax error. |
| `AddRegionRequest` | `structure` | InstanceArn, RegionName | - |
| `AddRegionResponse` | `structure` | Status | - |
| `AttachCustomerManagedPolicyReferenceToPermissionSetRequest` | `structure` | InstanceArn, PermissionSetArn, CustomerManagedPolicyReference | - |
| `AttachCustomerManagedPolicyReferenceToPermissionSetResponse` | `structure` | **empty (no members)** | - |
| `AttachManagedPolicyToPermissionSetRequest` | `structure` | InstanceArn, PermissionSetArn, ManagedPolicyArn | - |
| `AttachManagedPolicyToPermissionSetResponse` | `structure` | **empty (no members)** | - |
| `CreateAccountAssignmentRequest` | `structure` | InstanceArn, TargetId, TargetType, PermissionSetArn, PrincipalType, PrincipalId | - |
| `CreateAccountAssignmentResponse` | `structure` | AccountAssignmentCreationStatus | - |
| `CreateApplicationRequest` | `structure` | InstanceArn, ApplicationProviderArn, Name, Description, PortalOptions, Tags, Status, ClientToken | - |
| `CreateApplicationResponse` | `structure` | ApplicationArn, InstanceArn, IdentityStoreArn | - |
| `CreateApplicationAssignmentRequest` | `structure` | ApplicationArn, PrincipalId, PrincipalType | - |
| `CreateApplicationAssignmentResponse` | `structure` | **empty (no members)** | - |
| `CreateInstanceRequest` | `structure` | Name, ClientToken, Tags | - |
| `CreateInstanceResponse` | `structure` | InstanceArn | - |
| `CreateInstanceAccessControlAttributeConfigurationRequest` | `structure` | InstanceArn, InstanceAccessControlAttributeConfiguration | - |
| `CreateInstanceAccessControlAttributeConfigurationResponse` | `structure` | **empty (no members)** | - |
| `CreatePermissionSetRequest` | `structure` | Name, Description, InstanceArn, SessionDuration, RelayState, Tags | - |
| `CreatePermissionSetResponse` | `structure` | PermissionSet | - |
| `CreateTrustedTokenIssuerRequest` | `structure` | InstanceArn, Name, TrustedTokenIssuerType, TrustedTokenIssuerConfiguration, ClientToken, Tags | - |
| `CreateTrustedTokenIssuerResponse` | `structure` | TrustedTokenIssuerArn | - |
| `DeleteAccountAssignmentRequest` | `structure` | InstanceArn, TargetId, TargetType, PermissionSetArn, PrincipalType, PrincipalId | - |
| `DeleteAccountAssignmentResponse` | `structure` | AccountAssignmentDeletionStatus | - |
| `DeleteApplicationRequest` | `structure` | ApplicationArn | - |
| `DeleteApplicationResponse` | `structure` | **empty (no members)** | - |
| `DeleteApplicationAssignmentRequest` | `structure` | ApplicationArn, PrincipalId, PrincipalType | - |
| `DeleteApplicationAssignmentResponse` | `structure` | **empty (no members)** | - |
| `DeleteInlinePolicyFromPermissionSetRequest` | `structure` | InstanceArn, PermissionSetArn | - |
| `DeleteInlinePolicyFromPermissionSetResponse` | `structure` | **empty (no members)** | - |
| `DeleteInstanceRequest` | `structure` | InstanceArn | - |
| `DeleteInstanceResponse` | `structure` | **empty (no members)** | - |
| `DeleteInstanceAccessControlAttributeConfigurationRequest` | `structure` | InstanceArn | - |
| `DeleteInstanceAccessControlAttributeConfigurationResponse` | `structure` | **empty (no members)** | - |
| `DeletePermissionsBoundaryFromPermissionSetRequest` | `structure` | InstanceArn, PermissionSetArn | - |
| `AccessDeniedExceptionReason` | `enum` | KMS_ACCESS_DENIED_EXCEPTION | - |
| `ApplicationStatus` | `enum` | ENABLED, DISABLED | - |
| `ApplicationVisibility` | `enum` | ENABLED, DISABLED | - |
| `AuthenticationMethodType` | `enum` | IAM | - |
| `FederationProtocol` | `enum` | SAML, OAUTH | - |
| `GrantType` | `enum` | AUTHORIZATION_CODE, REFRESH_TOKEN, JWT_BEARER, TOKEN_EXCHANGE | - |
| `InstanceAccessControlAttributeConfigurationStatus` | `enum` | ENABLED, CREATION_IN_PROGRESS, CREATION_FAILED | - |
| `InstanceStatus` | `enum` | CREATE_IN_PROGRESS, CREATE_FAILED, DELETE_IN_PROGRESS, ACTIVE | - |
| `JwksRetrievalOption` | `enum` | OPEN_ID_DISCOVERY | - |
| `KmsKeyStatus` | `enum` | UPDATING, ENABLED, UPDATE_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

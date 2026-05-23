# AWS Resource Access Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Resource Access Manager API Reference . This documentation provides descriptions and syntax for each of the actions and data types in RAM. RAM is a service that helps you securely share your Amazon Web Services resources to other Amazon Web Services accounts. If you use Organizations to manage your accounts, then you can share your resources with your entire organization or to organizational units (OUs). For supported resource types, you can also share resources with individual Identity and Access Management (IAM) roles and users.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Resource Access Manager where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Resource Access Manager by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Resource Access Manager by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Resource Access Manager workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Associate` operation families, including `ListPendingInvitationResources`, `ListPermissionAssociations`, `ListPermissionVersions`, `ListPermissions`, `GetPermission`, `GetResourcePolicies`.

## Service Identity and Protocol

- AWS model slug: `ram`
- AWS SDK for Rust slug: `ram`
- Model version: `2018-01-04`
- Model file: `vendor/api-models-aws/models/ram/service/2018-01-04/ram-2018-01-04.json`
- SDK ID: `RAM`
- Endpoint prefix: `ram`
- ARN namespace: `ram`
- CloudFormation name: `RAM`
- CloudTrail event source: `ram.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (5), `Create` (3), `Delete` (3), `Associate` (2), `Disassociate` (2), `Promote` (2), `Accept` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptResourceShareInvitation`, `AssociateResourceShare`, `AssociateResourceSharePermission`, `CreatePermission`, `CreatePermissionVersion`, `CreateResourceShare`, `DeletePermission`, `DeletePermissionVersion`, `DeleteResourceShare`, `DisassociateResourceShare`, `DisassociateResourceSharePermission`, `EnableSharingWithAwsOrganization`, `RejectResourceShareInvitation`, `SetDefaultPermissionVersion`, `TagResource`, `UntagResource`, `UpdateResourceShare`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetPermission`, `GetResourcePolicies`, `GetResourceShareAssociations`, `GetResourceShareInvitations`, `GetResourceShares`, `ListPendingInvitationResources`, `ListPermissionAssociations`, `ListPermissionVersions`, `ListPermissions`, `ListPrincipals`, `ListReplacePermissionAssociationsWork`, `ListResourceSharePermissions`, `ListResourceTypes`, `ListResources`, `ListSourceAssociations`.
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 35 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/ram/latest/userguide/what-is.html
- https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html
- https://docs.aws.amazon.com/ram/latest/userguide/shareable.html

Research outcomes:
- AWS Resource Access Manager shares supported AWS resources across accounts, organisational units, organisations, IAM roles, and IAM users where supported.
- Resource shares contain resources, principals, managed permissions, and tags.
- Sharing within AWS Organizations can be enabled so supported principals can access shares without separate invitations.
- Sharing outside an organisation can require invitations that principals must accept before access is effective.
- Managed permissions define the allowed actions principals receive for shared resources.
- Not every AWS resource type is shareable; supported types have service-specific sharing semantics.
- Resource shares can be updated to add or remove principals, resources, tags, and managed permissions.

Parity implications:
- Model resource shares, associated resources, principals, managed permissions, invitations, organisation-sharing setting, and share status separately.
- Effective access should depend on principal type, organisation sharing, invitation acceptance, and resource type support.
- Removing a resource or principal should revoke share-derived access without deleting the underlying resource.

## Current Network Resource Stub Semantics

RAM currently recognises EC2 subnet sharing as permission metadata only.

- The default permission catalogue includes an `ec2:Subnet` resource type and subnet permission names.
- Resource shares store resource ARNs as opaque strings, so a shared subnet ARN is not resolved against EC2 subnet state.
- RAM association, invitation, and permission logic does not alter EC2 resource visibility in other service crates.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListPendingInvitationResources`, `ListPermissionAssociations`, `ListPermissions`, `ListPermissionVersions`, `ListPrincipals`, `ListReplacePermissionAssociationsWork`, `ListResources`, `ListResourceSharePermissions`, `ListResourceTypes`, `ListSourceAssociations`
- Traits: `paginated` (10)
- Common required input members in this group: `resourceOwner`

### Get

- Operations: `GetPermission`, `GetResourcePolicies`, `GetResourceShareAssociations`, `GetResourceShareInvitations`, `GetResourceShares`
- Traits: `paginated` (4)
- Common required input members in this group: -

### Create

- Operations: `CreatePermission`, `CreatePermissionVersion`, `CreateResourceShare`
- Common required input members in this group: `name`, `policyTemplate`

### Delete

- Operations: `DeletePermission`, `DeletePermissionVersion`, `DeleteResourceShare`
- Common required input members in this group: `permissionArn`

### Associate

- Operations: `AssociateResourceShare`, `AssociateResourceSharePermission`
- Common required input members in this group: `resourceShareArn`

### Disassociate

- Operations: `DisassociateResourceShare`, `DisassociateResourceSharePermission`
- Common required input members in this group: `resourceShareArn`

### Promote

- Operations: `PromotePermissionCreatedFromPolicy`, `PromoteResourceShareCreatedFromPolicy`
- Common required input members in this group: -

### Accept

- Operations: `AcceptResourceShareInvitation`
- Common required input members in this group: -

### Enable

- Operations: `EnableSharingWithAwsOrganization`
- Common required input members in this group: -

### Reject

- Operations: `RejectResourceShareInvitation`
- Common required input members in this group: -

### Replace

- Operations: `ReplacePermissionAssociations`
- Common required input members in this group: -

### Set

- Operations: `SetDefaultPermissionVersion`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateResourceShare`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptResourceShareInvitation` | `POST /acceptresourceshareinvitation` | - | `resourceShareInvitationArn` | - | `AcceptResourceShareInvitationResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `MalformedArnException`, `OperationNotPermittedException`, `ResourceShareInvitationAlreadyAcceptedException`, `ResourceShareInvitationAlreadyRejectedException`, `ResourceShareInvitationArnNotFoundException`, `ResourceShareInvitationExpiredException`, `ServerInternalException`, `ServiceUnavailableException` | Accepts an invitation to a resource share from another Amazon Web Services account. After you accept the invitation, the resources included in the resource share are available to interact with in the relevant Amazon ... |
| `AssociateResourceShare` | `POST /associateresourceshare` | - | `resourceShareArn` | - | `AssociateResourceShareResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `InvalidStateTransitionException`, `MalformedArnException`, `OperationNotPermittedException`, `ResourceShareLimitExceededException`, `ServerInternalException`, `ServiceUnavailableException`, `ThrottlingException`, `UnknownResourceException` | Adds the specified list of principals, resources, and source constraints to a resource share. Principals that already have access to this resource share immediately receive access to the added resources. Newly added ... |
| `AssociateResourceSharePermission` | `POST /associateresourcesharepermission` | - | `resourceShareArn`, `permissionArn` | - | `AssociateResourceSharePermissionResponse` | `InvalidClientTokenException`, `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Adds or replaces the RAM permission for a resource type included in a resource share. You can have exactly one permission associated with each resource type in the resource share. You can add a new RAM permission onl ... |
| `CreatePermission` | `POST /createpermission` | - | `name`, `resourceType`, `policyTemplate` | - | `CreatePermissionResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `InvalidPolicyException`, `MalformedPolicyTemplateException`, `OperationNotPermittedException`, `PermissionAlreadyExistsException`, `PermissionLimitExceededException`, `ServerInternalException`, `ServiceUnavailableException` | Creates a customer managed permission for a specified resource type that you can attach to resource shares. It is created in the Amazon Web Services Region in which you call the operation. |
| `CreatePermissionVersion` | `POST /createpermissionversion` | - | `permissionArn`, `policyTemplate` | - | `CreatePermissionVersionResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `InvalidPolicyException`, `MalformedArnException`, `MalformedPolicyTemplateException`, `PermissionVersionsLimitExceededException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Creates a new version of the specified customer managed permission. The new version is automatically set as the default version of the customer managed permission. New resource shares automatically use the default pe ... |
| `CreateResourceShare` | `POST /createresourceshare` | - | `name` | - | `CreateResourceShareResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `InvalidStateTransitionException`, `MalformedArnException`, `OperationNotPermittedException`, `ResourceShareLimitExceededException`, `ServerInternalException`, `ServiceUnavailableException`, `TagLimitExceededException`, `TagPolicyViolationException`, `ThrottlingException`, `UnknownResourceException` | Creates a resource share. You can provide a list of the Amazon Resource Names (ARNs) for the resources that you want to share, a list of principals you want to share the resources with, the permissions to grant those ... |
| `DeletePermission` | `DELETE /deletepermission` | - | `permissionArn` | - | `DeletePermissionResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Deletes the specified customer managed permission in the Amazon Web Services Region in which you call this operation. You can delete a customer managed permission only if it isn't attached to any resource share. The ... |
| `DeletePermissionVersion` | `DELETE /deletepermissionversion` | - | `permissionArn`, `permissionVersion` | - | `DeletePermissionVersionResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Deletes one version of a customer managed permission. The version you specify must not be attached to any resource share and must not be the default version for the permission. If a customer managed permission has th ... |
| `DeleteResourceShare` | `DELETE /deleteresourceshare` | - | `resourceShareArn` | - | `DeleteResourceShareResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `InvalidStateTransitionException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `ThrottlingException`, `UnknownResourceException` | Deletes the specified resource share. This doesn't delete any of the resources that were associated with the resource share; it only stops the sharing of those resources through this resource share. |
| `DisassociateResourceShare` | `POST /disassociateresourceshare` | - | `resourceShareArn` | - | `DisassociateResourceShareResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `InvalidStateTransitionException`, `MalformedArnException`, `OperationNotPermittedException`, `ResourceShareLimitExceededException`, `ServerInternalException`, `ServiceUnavailableException`, `ThrottlingException`, `UnknownResourceException` | Removes the specified principals, resources, or source constraints from participating in the specified resource share. |
| `DisassociateResourceSharePermission` | `POST /disassociateresourcesharepermission` | - | `resourceShareArn`, `permissionArn` | - | `DisassociateResourceSharePermissionResponse` | `InvalidClientTokenException`, `InvalidParameterException`, `InvalidStateTransitionException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Removes a managed permission from a resource share. Permission changes take effect immediately. You can remove a managed permission from a resource share only if there are currently no resources of the relevant resou ... |
| `EnableSharingWithAwsOrganization` | `POST /enablesharingwithawsorganization` | - | - | - | `EnableSharingWithAwsOrganizationResponse` | `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException` | Enables resource sharing within your organization in Organizations. This operation creates a service-linked role called AWSServiceRoleForResourceAccessManager that has the IAM managed policy named AWSResourceAccessMa ... |
| `GetPermission` | `POST /getpermission` | - | `permissionArn` | - | `GetPermissionResponse` | `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Retrieves the contents of a managed permission in JSON format. |
| `GetResourcePolicies` | `POST /getresourcepolicies` | `paginated` | `resourceArns` | - | `GetResourcePoliciesResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `ResourceArnNotFoundException`, `ServerInternalException`, `ServiceUnavailableException` | Retrieves the resource policies for the specified resources that you own and have shared. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can occasi ... |
| `GetResourceShareAssociations` | `POST /getresourceshareassociations` | `paginated` | `associationType` | - | `GetResourceShareAssociationsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Retrieves the lists of resources and principals that associated for resource shares that you own. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations ca ... |
| `GetResourceShareInvitations` | `POST /getresourceshareinvitations` | `paginated` | - | - | `GetResourceShareInvitationsResponse` | `InvalidMaxResultsException`, `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `ResourceShareInvitationArnNotFoundException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Retrieves details about invitations that you have received for resource shares. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can occasionally ret ... |
| `GetResourceShares` | `POST /getresourceshares` | `paginated` | `resourceOwner` | - | `GetResourceSharesResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Retrieves details about the resource shares that you own or that are shared with you. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can occasional ... |
| `ListPendingInvitationResources` | `POST /listpendinginvitationresources` | `paginated` | `resourceShareInvitationArn` | - | `ListPendingInvitationResourcesResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `MissingRequiredParameterException`, `ResourceShareInvitationAlreadyRejectedException`, `ResourceShareInvitationArnNotFoundException`, `ResourceShareInvitationExpiredException`, `ServerInternalException`, `ServiceUnavailableException` | Lists the resources in a resource share that is shared with you but for which the invitation is still PENDING . That means that you haven't accepted or rejected the invitation and the invitation hasn't expired. Alway ... |
| `ListPermissionAssociations` | `POST /listpermissionassociations` | `paginated` | - | - | `ListPermissionAssociationsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException` | Lists information about the managed permission and its associations to any resource shares that use this managed permission. This lets you see which resource shares use which versions of the specified managed permiss ... |
| `ListPermissions` | `POST /listpermissions` | `paginated` | - | - | `ListPermissionsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException` | Retrieves a list of available RAM permissions that you can use for the supported resource types. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can ... |
| `ListPermissionVersions` | `POST /listpermissionversions` | `paginated` | `permissionArn` | - | `ListPermissionVersionsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Lists the available versions of the specified RAM permission. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can occasionally return an empty set o ... |
| `ListPrincipals` | `POST /listprincipals` | `paginated` | `resourceOwner` | - | `ListPrincipalsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Lists the principals that you are sharing resources with or that are sharing resources with you. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can ... |
| `ListReplacePermissionAssociationsWork` | `POST /listreplacepermissionassociationswork` | `paginated` | - | - | `ListReplacePermissionAssociationsWorkResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `ServerInternalException`, `ServiceUnavailableException` | Retrieves the current status of the asynchronous tasks performed by RAM when you perform the ReplacePermissionAssociationsWork operation. Always check the NextToken response parameter for a null value when calling a ... |
| `ListResources` | `POST /listresources` | `paginated` | `resourceOwner` | - | `ListResourcesResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `InvalidResourceTypeException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Lists the resources that you added to a resource share or the resources that are shared with you. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations ca ... |
| `ListResourceSharePermissions` | `POST /listresourcesharepermissions` | `paginated` | `resourceShareArn` | - | `ListResourceSharePermissionsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Lists the RAM permissions that are associated with a resource share. Always check the NextToken response parameter for a null value when calling a paginated operation. These operations can occasionally return an empt ... |
| `ListResourceTypes` | `POST /listresourcetypes` | `paginated` | - | - | `ListResourceTypesResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `ServerInternalException`, `ServiceUnavailableException` | Lists the resource types that can be shared by RAM. |
| `ListSourceAssociations` | `POST /listsourceassociations` | `paginated` | - | - | `ListSourceAssociationsResponse` | `InvalidNextTokenException`, `InvalidParameterException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Lists source associations for resource shares. Source associations control which sources can be used with service principals in resource shares. This operation provides visibility into source associations for resourc ... |
| `PromotePermissionCreatedFromPolicy` | `POST /promotepermissioncreatedfrompolicy` | - | `permissionArn`, `name` | - | `PromotePermissionCreatedFromPolicyResponse` | `InvalidParameterException`, `InvalidPolicyException`, `MalformedArnException`, `MissingRequiredParameterException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | When you attach a resource-based policy to a resource, RAM automatically creates a resource share of featureSet = CREATED_FROM_POLICY with a managed permission that has the same IAM permissions as the original resour ... |
| `PromoteResourceShareCreatedFromPolicy` | `POST /promoteresourcesharecreatedfrompolicy` | - | `resourceShareArn` | - | `PromoteResourceShareCreatedFromPolicyResponse` | `InvalidParameterException`, `InvalidStateTransitionException`, `MalformedArnException`, `MissingRequiredParameterException`, `OperationNotPermittedException`, `ResourceShareLimitExceededException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException`, `UnmatchedPolicyPermissionException` | When you attach a resource-based policy to a resource, RAM automatically creates a resource share of featureSet = CREATED_FROM_POLICY with a managed permission that has the same IAM permissions as the original resour ... |
| `RejectResourceShareInvitation` | `POST /rejectresourceshareinvitation` | - | `resourceShareInvitationArn` | - | `RejectResourceShareInvitationResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `MalformedArnException`, `OperationNotPermittedException`, `ResourceShareInvitationAlreadyAcceptedException`, `ResourceShareInvitationAlreadyRejectedException`, `ResourceShareInvitationArnNotFoundException`, `ResourceShareInvitationExpiredException`, `ServerInternalException`, `ServiceUnavailableException` | Rejects an invitation to a resource share from another Amazon Web Services account. |
| `ReplacePermissionAssociations` | `POST /replacepermissionassociations` | - | `fromPermissionArn`, `toPermissionArn` | - | `ReplacePermissionAssociationsResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `MalformedArnException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Updates all resource shares that use a managed permission to a different managed permission. This operation always applies the default version of the target managed permission. You can optionally specify that the upd ... |
| `SetDefaultPermissionVersion` | `POST /setdefaultpermissionversion` | - | `permissionArn`, `permissionVersion` | - | `SetDefaultPermissionVersionResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Designates the specified version number as the default version for the specified customer managed permission. New resource shares automatically use this new default permission. Existing resource shares continue to us ... |
| `TagResource` | `POST /tagresource` | - | `tags` | - | `TagResourceResponse` | `InvalidParameterException`, `MalformedArnException`, `ResourceArnNotFoundException`, `ServerInternalException`, `ServiceUnavailableException`, `TagLimitExceededException`, `TagPolicyViolationException`, `UnknownResourceException` | Adds the specified tag keys and values to a resource share or managed permission. If you choose a resource share, the tags are attached to only the resource share, not to the resources that are in the resource share. ... |
| `UntagResource` | `POST /untagresource` | - | `tagKeys` | - | `UntagResourceResponse` | `InvalidParameterException`, `MalformedArnException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Removes the specified tag key and value pairs from the specified resource share or managed permission. |
| `UpdateResourceShare` | `POST /updateresourceshare` | - | `resourceShareArn` | - | `UpdateResourceShareResponse` | `IdempotentParameterMismatchException`, `InvalidClientTokenException`, `InvalidParameterException`, `MalformedArnException`, `MissingRequiredParameterException`, `OperationNotPermittedException`, `ServerInternalException`, `ServiceUnavailableException`, `UnknownResourceException` | Modifies some of the properties of the specified resource share. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeletePermission` | - | `permissionArn -> permissionArn`, `clientToken -> clientToken` | - | - |
| `DeletePermissionVersion` | - | `permissionArn -> permissionArn`, `permissionVersion -> permissionVersion`, `clientToken -> clientToken` | - | - |
| `DeleteResourceShare` | - | `resourceShareArn -> resourceShareArn`, `clientToken -> clientToken` | - | - |
| `PromoteResourceShareCreatedFromPolicy` | - | `resourceShareArn -> resourceShareArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `IdempotentParameterMismatchException` | `structure` | message | The operation failed because the client token input parameter matched one that was used with a previous call to the operation, but at least one of the other ... |
| `InvalidClientTokenException` | `structure` | message | The operation failed because the specified client token isn't valid. |
| `InvalidMaxResultsException` | `structure` | message | The operation failed because the specified value for MaxResults isn't valid. |
| `InvalidNextTokenException` | `structure` | message | The operation failed because the specified value for NextToken isn't valid. You must specify a value you received in the NextToken response of a previous ca ... |
| `InvalidParameterException` | `structure` | message | The operation failed because a parameter you specified isn't valid. |
| `InvalidPolicyException` | `structure` | message | The operation failed because a policy you specified isn't valid. |
| `InvalidResourceTypeException` | `structure` | message | The operation failed because the specified resource type isn't valid. |
| `InvalidStateTransitionException` | `structure` | message | The operation failed because the requested operation isn't valid for the resource share in its current state. |
| `MalformedArnException` | `structure` | message | The operation failed because the specified Amazon Resource Name (ARN) has a format that isn't valid. |
| `MalformedPolicyTemplateException` | `structure` | message | The operation failed because the policy template that you provided isn't valid. |
| `MissingRequiredParameterException` | `structure` | message | The operation failed because a required input parameter is missing. |
| `OperationNotPermittedException` | `structure` | message | The operation failed because the requested operation isn't permitted. |
| `PermissionAlreadyExistsException` | `structure` | message | The operation failed because a permission with the specified name already exists in the requested Amazon Web Services Region. Choose a different name. |
| `PermissionLimitExceededException` | `structure` | message | The operation failed because it would exceed the maximum number of permissions you can create in each Amazon Web Services Region. To view the limits for you ... |
| `PermissionVersionsLimitExceededException` | `structure` | message | The operation failed because it would exceed the limit for the number of versions you can have for a permission. To view the limits for your Amazon Web Serv ... |
| `ResourceArnNotFoundException` | `structure` | message | The operation failed because the specified Amazon Resource Name (ARN) was not found. |
| `ResourceShareInvitationAlreadyAcceptedException` | `structure` | message | The operation failed because the specified invitation was already accepted. |
| `ResourceShareInvitationAlreadyRejectedException` | `structure` | message | The operation failed because the specified invitation was already rejected. |
| `ResourceShareInvitationArnNotFoundException` | `structure` | message | The operation failed because the specified Amazon Resource Name (ARN) for an invitation was not found. |
| `ResourceShareInvitationExpiredException` | `structure` | message | The operation failed because the specified invitation is past its expiration date and time. |
| `ResourceShareLimitExceededException` | `structure` | message | The operation failed because it would exceed the limit for resource shares for your account. You can associate up to 100 resources per call. To view the lim ... |
| `ServerInternalException` | `structure` | message | The operation failed because the service could not respond to the request due to an internal problem. Try again later. |
| `ServiceUnavailableException` | `structure` | message | The operation failed because the service isn't available. Try again later. |
| `TagLimitExceededException` | `structure` | message | The operation failed because it would exceed the limit for tags for your Amazon Web Services account. |
| `TagPolicyViolationException` | `structure` | message | The operation failed because the specified tag key is a reserved word and can't be used. |
| `ThrottlingException` | `structure` | message | The operation failed because it exceeded the rate at which you are allowed to perform this operation. Please try again later. |
| `UnknownResourceException` | `structure` | message | The operation failed because a specified resource couldn't be found. |
| `UnmatchedPolicyPermissionException` | `structure` | message | There isn't an existing managed permission defined in RAM that has the same IAM permissions as the resource-based policy attached to the resource. You shoul ... |
| `AcceptResourceShareInvitationRequest` | `structure` | resourceShareInvitationArn, clientToken | - |
| `AcceptResourceShareInvitationResponse` | `structure` | resourceShareInvitation, clientToken | - |
| `AssociateResourceShareRequest` | `structure` | resourceShareArn, resourceArns, principals, clientToken, sources | - |
| `AssociateResourceShareResponse` | `structure` | resourceShareAssociations, clientToken | - |
| `AssociateResourceSharePermissionRequest` | `structure` | resourceShareArn, permissionArn, replace, clientToken, permissionVersion | - |
| `AssociateResourceSharePermissionResponse` | `structure` | returnValue, clientToken | - |
| `CreatePermissionRequest` | `structure` | name, resourceType, policyTemplate, clientToken, tags | - |
| `CreatePermissionResponse` | `structure` | permission, clientToken | - |
| `CreatePermissionVersionRequest` | `structure` | permissionArn, policyTemplate, clientToken | - |
| `CreatePermissionVersionResponse` | `structure` | permission, clientToken | - |
| `CreateResourceShareRequest` | `structure` | name, resourceArns, principals, tags, allowExternalPrincipals, clientToken, permissionArns, sources, resourceShareConfiguration | - |
| `CreateResourceShareResponse` | `structure` | resourceShare, clientToken | - |
| `PermissionFeatureSet` | `enum` | CREATED_FROM_POLICY, PROMOTING_TO_STANDARD, STANDARD | - |
| `PermissionStatus` | `enum` | ATTACHABLE, UNATTACHABLE, DELETING, DELETED | - |
| `PermissionType` | `enum` | CUSTOMER_MANAGED, AWS_MANAGED | - |
| `PermissionTypeFilter` | `enum` | ALL, AWS_MANAGED, CUSTOMER_MANAGED | - |
| `ReplacePermissionAssociationsWorkStatus` | `enum` | IN_PROGRESS, COMPLETED, FAILED | - |
| `ResourceOwner` | `enum` | SELF, OTHER_ACCOUNTS | - |
| `ResourceRegionScope` | `enum` | REGIONAL, GLOBAL | - |
| `ResourceRegionScopeFilter` | `enum` | ALL, REGIONAL, GLOBAL | - |
| `ResourceShareAssociationStatus` | `enum` | ASSOCIATING, ASSOCIATED, FAILED, DISASSOCIATING, DISASSOCIATED, SUSPENDED, SUSPENDING, RESTORING | - |
| `ResourceShareAssociationType` | `enum` | PRINCIPAL, RESOURCE, SOURCE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

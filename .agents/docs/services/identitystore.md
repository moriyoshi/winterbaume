# AWS SSO Identity Store

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Identity Store service used by IAM Identity Center provides a single place to retrieve all of your identities (users and groups). For more information, see the IAM Identity Center User Guide. This reference guide describes the identity store operations that you can call programmatically and includes detailed information about data types and errors. IAM Identity Center uses the `sso`, `sso-directory`, and `identitystore` API namespaces. The `sso-directory` and `identitystore` namespaces authorize access to data in the Identity Store.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS SSO Identity Store where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented AWS SSO Identity Store workflows in the local mock. Key resources include `GroupMembershipResource`, `GroupResource`, `UserResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Get` operation families, including `ListGroupMemberships`, `ListGroupMembershipsForMember`, `ListGroups`, `ListUsers`, `CreateGroup`, `CreateGroupMembership`.

## Service Identity and Protocol

- AWS model slug: `identitystore`
- AWS SDK for Rust slug: `identitystore`
- Model version: `2020-06-15`
- Model file: `vendor/api-models-aws/models/identitystore/service/2020-06-15/identitystore-2020-06-15.json`
- SDK ID: `identitystore`
- Endpoint prefix: `identitystore`
- ARN namespace: `identitystore`
- CloudFormation name: `IdentityStore`
- CloudTrail event source: `identitystore.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (3), `Delete` (3), `Describe` (3), `Get` (3), `Update` (2), `Is` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateGroup`, `CreateGroupMembership`, `CreateUser`, `DeleteGroup`, `DeleteGroupMembership`, `DeleteUser`, `UpdateGroup`, `UpdateUser`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeGroup`, `DescribeGroupMembership`, `DescribeUser`, `GetGroupId`, `GetGroupMembershipId`, `GetUserId`, `IsMemberInGroups`, `ListGroupMemberships`, `ListGroupMembershipsForMember`, `ListGroups`, `ListUsers`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `GroupMembershipResource` | `MembershipId` | create: `CreateGroupMembership`; read: `DescribeGroupMembership`; delete: `DeleteGroupMembership`; list: `ListGroupMemberships` | - | Resource Type definition for AWS:IdentityStore::GroupMembership |
| `GroupResource` | `GroupId` | create: `CreateGroup`; read: `DescribeGroup`; update: `UpdateGroup`; delete: `DeleteGroup`; list: `ListGroups` | - | Resource Type definition for AWS::IdentityStore::Group |
| `UserResource` | `UserId` | create: `CreateUser`; read: `DescribeUser`; update: `UpdateUser`; delete: `DeleteUser`; list: `ListUsers` | - | Resource Type definition for AWS:IdentityStore::User |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/singlesignon/latest/userguide/what-is.html
- https://docs.aws.amazon.com/singlesignon/latest/userguide/reconcile-auto-provisioning.html
- https://docs.aws.amazon.com/singlesignon/latest/APIReference/welcome.html

Research outcomes:
- Identity Store backs IAM Identity Center workforce identities and stores users, groups, and group memberships.
- Each identity store has an identity store id used by API operations.
- Users and groups can be provisioned from external identity providers through SCIM or managed directly depending on identity source configuration.
- Auto-provisioned users, groups, and memberships can be audited and reconciled with Identity Store CLI/API commands.
- Identity Store resources are used by IAM Identity Center account assignments and application assignments.
- Deleting or changing users and groups affects access through assignments that reference those principals.

Parity implications:
- Model identity stores, users, groups, memberships, external ids, SCIM-provisioned metadata, and principal references separately.
- User/group deletion should interact with assignment visibility and orphaned principal references.
- List and lookup operations should preserve identity store scoping and pagination semantics.

## Operation Groups

### List

- Operations: `ListGroupMemberships`, `ListGroupMembershipsForMember`, `ListGroups`, `ListUsers`
- Traits: `paginated` (4), `readonly` (4)
- Common required input members in this group: `GroupId`, `IdentityStoreId`, `MemberId`

### Create

- Operations: `CreateGroup`, `CreateGroupMembership`, `CreateUser`
- Common required input members in this group: `GroupId`, `IdentityStoreId`, `MemberId`

### Delete

- Operations: `DeleteGroup`, `DeleteGroupMembership`, `DeleteUser`
- Traits: `idempotent` (3)
- Common required input members in this group: `GroupId`, `IdentityStoreId`, `MembershipId`, `UserId`

### Describe

- Operations: `DescribeGroup`, `DescribeGroupMembership`, `DescribeUser`
- Traits: `readonly` (3)
- Common required input members in this group: `GroupId`, `IdentityStoreId`, `MembershipId`, `UserId`

### Get

- Operations: `GetGroupId`, `GetGroupMembershipId`, `GetUserId`
- Traits: `readonly` (3)
- Common required input members in this group: `AlternateIdentifier`, `GroupId`, `IdentityStoreId`, `MemberId`

### Update

- Operations: `UpdateGroup`, `UpdateUser`
- Common required input members in this group: `GroupId`, `IdentityStoreId`, `Operations`, `UserId`

### Is

- Operations: `IsMemberInGroups`
- Traits: `readonly` (1)
- Common required input members in this group: `GroupIds`, `IdentityStoreId`, `MemberId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateGroup` | - | - | `IdentityStoreId` | - | `CreateGroupResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a group within the specified identity store. |
| `CreateGroupMembership` | - | - | `GroupId`, `IdentityStoreId`, `MemberId` | - | `CreateGroupMembershipResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a relationship between a member and a group. The following identifiers must be specified: `GroupId`, `IdentityStoreId`, and `MemberId`. |
| `CreateUser` | - | - | `IdentityStoreId` | - | `CreateUserResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a user within the specified identity store. |
| `DeleteGroup` | - | `idempotent` | `GroupId`, `IdentityStoreId` | - | `DeleteGroupResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Delete a group within an identity store given `GroupId`. |
| `DeleteGroupMembership` | - | `idempotent` | `IdentityStoreId`, `MembershipId` | - | `DeleteGroupMembershipResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Delete a membership within a group given `MembershipId`. |
| `DeleteUser` | - | `idempotent` | `IdentityStoreId`, `UserId` | - | `DeleteUserResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes a user within an identity store given `UserId`. |
| `DescribeGroup` | - | `readonly` | `GroupId`, `IdentityStoreId` | - | `DescribeGroupResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the group metadata and attributes from `GroupId` in an identity store. If you have access to a member account, you can use this API operation from the member account. |
| `DescribeGroupMembership` | - | `readonly` | `IdentityStoreId`, `MembershipId` | - | `DescribeGroupMembershipResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves membership metadata and attributes from `MembershipId` in an identity store. If you have access to a member account, you can use this API operation from the member account. |
| `DescribeUser` | - | `readonly` | `IdentityStoreId`, `UserId` | - | `DescribeUserResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the user metadata and attributes from the `UserId` in an identity store. If you have access to a member account, you can use this API operation from the member account. |
| `GetGroupId` | - | `readonly` | `AlternateIdentifier`, `IdentityStoreId` | - | `GetGroupIdResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves `GroupId` in an identity store. If you have access to a member account, you can use this API operation from the member account. |
| `GetGroupMembershipId` | - | `readonly` | `GroupId`, `IdentityStoreId`, `MemberId` | - | `GetGroupMembershipIdResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the `MembershipId` in an identity store. If you have access to a member account, you can use this API operation from the member account. |
| `GetUserId` | - | `readonly` | `AlternateIdentifier`, `IdentityStoreId` | - | `GetUserIdResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the `UserId` in an identity store. If you have access to a member account, you can use this API operation from the member account. |
| `IsMemberInGroups` | - | `readonly` | `GroupIds`, `IdentityStoreId`, `MemberId` | - | `IsMemberInGroupsResponse` | `ResourceNotFoundException`, `ValidationException` | Checks the user's membership in all requested groups and returns if the member exists in all queried groups. If you have access to a member account, you can use this API operation from the member account. |
| `ListGroupMemberships` | - | `readonly`, `paginated` | `GroupId`, `IdentityStoreId` | - | `ListGroupMembershipsResponse` | `ResourceNotFoundException`, `ValidationException` | For the specified group in the specified identity store, returns the list of all ` GroupMembership` objects and returns results in paginated form. If you have access to a member account, you can use this API operation from the member account. |
| `ListGroupMembershipsForMember` | - | `readonly`, `paginated` | `IdentityStoreId`, `MemberId` | - | `ListGroupMembershipsForMemberResponse` | `ResourceNotFoundException`, `ValidationException` | For the specified member in the specified identity store, returns the list of all ` GroupMembership` objects and returns results in paginated form. If you have access to a member account, you can use this API operation from the member account. |
| `ListGroups` | - | `readonly`, `paginated` | `IdentityStoreId` | - | `ListGroupsResponse` | `ResourceNotFoundException`, `ValidationException` | Lists all groups in the identity store. Returns a paginated list of complete `Group` objects. |
| `ListUsers` | - | `readonly`, `paginated` | `IdentityStoreId` | - | `ListUsersResponse` | `ResourceNotFoundException`, `ValidationException` | Lists all users in the identity store. Returns a paginated list of complete `User` objects. |
| `UpdateGroup` | - | - | `GroupId`, `IdentityStoreId`, `Operations` | - | `UpdateGroupResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates the specified group metadata and attributes in the specified identity store. |
| `UpdateUser` | - | - | `IdentityStoreId`, `Operations`, `UserId` | - | `UpdateUserResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates the specified user metadata and attributes in the specified identity store. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `Message`, `Reason`, `RequestId`, `ResourceId`, `ResourceType` | Indicates that a requested resource is not found. |
| `ValidationException` | `structure` | `Message`, `Reason`, `RequestId` | The request failed because it contains a syntax error. |
| `ConflictException` | `structure` | `Message`, `Reason`, `RequestId` | This request cannot be completed for one of the following reasons: Performing the requested operation would violate an existing uniqueness claim in the identity store. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `RequestId` | The request would cause the number of users or groups in the identity store to exceed the maximum allowed. |
| `CreateGroupRequest` | `structure` | `Description`, `DisplayName`, `IdentityStoreId` | - |
| `CreateGroupResponse` | `structure` | `GroupId`, `IdentityStoreId` | - |
| `CreateGroupMembershipRequest` | `structure` | `GroupId`, `IdentityStoreId`, `MemberId` | - |
| `CreateGroupMembershipResponse` | `structure` | `IdentityStoreId`, `MembershipId` | - |
| `CreateUserRequest` | `structure` | `Addresses`, `Birthdate`, `DisplayName`, `Emails`, `Extensions`, `IdentityStoreId`, `Locale`, `Name`, `NickName`, `PhoneNumbers`, `Photos`, `PreferredLanguage`, ... (+7) | - |
| `CreateUserResponse` | `structure` | `IdentityStoreId`, `UserId` | - |
| `DeleteGroupRequest` | `structure` | `GroupId`, `IdentityStoreId` | - |
| `DeleteGroupResponse` | `structure` | - | - |
| `DeleteGroupMembershipRequest` | `structure` | `IdentityStoreId`, `MembershipId` | - |
| `DeleteGroupMembershipResponse` | `structure` | - | - |
| `DeleteUserRequest` | `structure` | `IdentityStoreId`, `UserId` | - |
| `DeleteUserResponse` | `structure` | - | - |
| `DescribeGroupRequest` | `structure` | `GroupId`, `IdentityStoreId` | - |
| `DescribeGroupResponse` | `structure` | `CreatedAt`, `CreatedBy`, `Description`, `DisplayName`, `ExternalIds`, `GroupId`, `IdentityStoreId`, `UpdatedAt`, `UpdatedBy` | - |
| `DescribeGroupMembershipRequest` | `structure` | `IdentityStoreId`, `MembershipId` | - |
| `DescribeGroupMembershipResponse` | `structure` | `CreatedAt`, `CreatedBy`, `GroupId`, `IdentityStoreId`, `MemberId`, `MembershipId`, `UpdatedAt`, `UpdatedBy` | - |
| `DescribeUserRequest` | `structure` | `Extensions`, `IdentityStoreId`, `UserId` | - |
| `DescribeUserResponse` | `structure` | `Addresses`, `Birthdate`, `CreatedAt`, `CreatedBy`, `DisplayName`, `Emails`, `Extensions`, `ExternalIds`, `IdentityStoreId`, `Locale`, `Name`, `NickName`, ... (+14) | - |
| `GetGroupIdRequest` | `structure` | `AlternateIdentifier`, `IdentityStoreId` | - |
| `GetGroupIdResponse` | `structure` | `GroupId`, `IdentityStoreId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

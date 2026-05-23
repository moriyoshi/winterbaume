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

### Get

- Operations: `GetGroupId`, `GetGroupMembershipId`, `GetUserId`
- Traits: `readonly` (3)
- Common required input members in this group: `IdentityStoreId`, `AlternateIdentifier`

### Is

- Operations: `IsMemberInGroups`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListGroupMembershipsForMember`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetGroupId` | `-` | `readonly` | `IdentityStoreId`, `AlternateIdentifier` | - | `GetGroupIdResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves GroupId in an identity store. If you have access to a member account, you can use this API operation from the member account. For more information, see Limiting access to the identity store from member acco ... |
| `GetGroupMembershipId` | `-` | `readonly` | `IdentityStoreId`, `GroupId`, `MemberId` | - | `GetGroupMembershipIdResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the MembershipId in an identity store. If you have access to a member account, you can use this API operation from the member account. For more information, see Limiting access to the identity store from me ... |
| `GetUserId` | `-` | `readonly` | `IdentityStoreId`, `AlternateIdentifier` | - | `GetUserIdResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieves the UserId in an identity store. If you have access to a member account, you can use this API operation from the member account. For more information, see Limiting access to the identity store from member a ... |
| `IsMemberInGroups` | `-` | `readonly` | `IdentityStoreId`, `MemberId`, `GroupIds` | - | `IsMemberInGroupsResponse` | `ResourceNotFoundException`, `ValidationException` | Checks the user's membership in all requested groups and returns if the member exists in all queried groups. If you have access to a member account, you can use this API operation from the member account. For more in ... |
| `ListGroupMembershipsForMember` | `-` | `readonly`, `paginated` | `IdentityStoreId`, `MemberId` | - | `ListGroupMembershipsForMemberResponse` | `ResourceNotFoundException`, `ValidationException` | For the specified member in the specified identity store, returns the list of all GroupMembership objects and returns results in paginated form. If you have access to a member account, you can use this API operation ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, RequestId, Reason | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message, RequestId, Reason | This request cannot be completed for one of the following reasons: Performing the requested operation would violate an existing uniqueness claim in the iden ... |
| `InternalServerException` | `structure` | Message, RequestId, RetryAfterSeconds | The request processing has failed because of an unknown error, exception or failure with an internal server. |
| `ResourceNotFoundException` | `structure` | ResourceType, ResourceId, Reason, Message, RequestId | Indicates that a requested resource is not found. |
| `ServiceQuotaExceededException` | `structure` | Message, RequestId | The request would cause the number of users or groups in the identity store to exceed the maximum allowed. |
| `ThrottlingException` | `structure` | Message, RequestId, RetryAfterSeconds, Reason | Indicates that the principal has crossed the throttling limits of the API operations. |
| `ValidationException` | `structure` | Message, RequestId, Reason | The request failed because it contains a syntax error. |
| `GetGroupIdRequest` | `structure` | IdentityStoreId, AlternateIdentifier | - |
| `GetGroupIdResponse` | `structure` | GroupId, IdentityStoreId | - |
| `GetGroupMembershipIdRequest` | `structure` | IdentityStoreId, GroupId, MemberId | - |
| `GetGroupMembershipIdResponse` | `structure` | MembershipId, IdentityStoreId | - |
| `GetUserIdRequest` | `structure` | IdentityStoreId, AlternateIdentifier | - |
| `GetUserIdResponse` | `structure` | IdentityStoreId, UserId | - |
| `IsMemberInGroupsRequest` | `structure` | IdentityStoreId, MemberId, GroupIds | - |
| `IsMemberInGroupsResponse` | `structure` | Results | - |
| `ListGroupMembershipsForMemberRequest` | `structure` | IdentityStoreId, MemberId, MaxResults, NextToken | - |
| `ListGroupMembershipsForMemberResponse` | `structure` | GroupMemberships, NextToken | - |
| `AccessDeniedExceptionReason` | `enum` | KMS_ACCESS_DENIED | - |
| `ConflictExceptionReason` | `enum` | UNIQUENESS_CONSTRAINT_VIOLATION, CONCURRENT_MODIFICATION | - |
| `ResourceNotFoundExceptionReason` | `enum` | KMS_KEY_NOT_FOUND | - |
| `ResourceType` | `enum` | GROUP, USER, IDENTITY_STORE, GROUP_MEMBERSHIP, RESOURCE_POLICY | - |
| `ThrottlingExceptionReason` | `enum` | KMS_THROTTLING | - |
| `UserStatus` | `enum` | ENABLED, DISABLED | - |
| `ValidationExceptionReason` | `enum` | KMS_INVALID_ARN, KMS_INVALID_KEY_USAGE, KMS_INVALID_STATE, KMS_DISABLED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

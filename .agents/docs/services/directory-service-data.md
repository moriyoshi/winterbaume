# AWS Directory Service Data

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Directory Service Data is an extension of Directory Service. This API reference provides detailed information about Directory Service Data operations and object types. With Directory Service Data, you can create, read, update, and delete users, groups, and memberships from your Managed Microsoft AD without additional costs and without deploying dedicated management instances. You can also perform built-in object management tasks across directories without direct network connectivity, which simplifies provisioning and access management to achieve fully automated deployments. Directory Service Data supports user and group write operations, such as `CreateUser` and `CreateGroup`, within the organizational unit (OU) of your Managed Microsoft AD.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Directory Service Data by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Directory Service Data by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Directory Service Data workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Search` operation families, including `ListGroupMembers`, `ListGroups`, `ListGroupsForMember`, `ListUsers`, `CreateGroup`, `CreateUser`.

## Service Identity and Protocol

- AWS model slug: `directory-service-data`
- AWS SDK for Rust slug: `directoryservicedata`
- Model version: `2023-05-31`
- Model file: `vendor/api-models-aws/models/directory-service-data/service/2023-05-31/directory-service-data-2023-05-31.json`
- SDK ID: `Directory Service Data`
- Endpoint prefix: `ds-data`
- ARN namespace: `ds`
- CloudFormation name: `DirectoryServiceData`
- CloudTrail event source: `ds.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (2), `Delete` (2), `Describe` (2), `Search` (2), `Update` (2), `Add` (1), `Disable` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddGroupMember`, `CreateGroup`, `CreateUser`, `DeleteGroup`, `DeleteUser`, `DisableUser`, `RemoveGroupMember`, `UpdateGroup`, `UpdateUser`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeGroup`, `DescribeUser`, `ListGroupMembers`, `ListGroups`, `ListGroupsForMember`, `ListUsers`, `SearchGroups`, `SearchUsers`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListGroupMembers`, `ListGroups`, `ListGroupsForMember`, `ListUsers`
- Traits: `paginated` (4), `readonly` (4)
- Common required input members in this group: `DirectoryId`, `SAMAccountName`

### Create

- Operations: `CreateGroup`, `CreateUser`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `DirectoryId`, `SAMAccountName`

### Delete

- Operations: `DeleteGroup`, `DeleteUser`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `DirectoryId`, `SAMAccountName`

### Describe

- Operations: `DescribeGroup`, `DescribeUser`
- Traits: `readonly` (2)
- Common required input members in this group: `DirectoryId`, `SAMAccountName`

### Search

- Operations: `SearchGroups`, `SearchUsers`
- Traits: `paginated` (2), `readonly` (2)
- Common required input members in this group: `DirectoryId`, `SearchAttributes`, `SearchString`

### Update

- Operations: `UpdateGroup`, `UpdateUser`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `DirectoryId`, `SAMAccountName`

### Add

- Operations: `AddGroupMember`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DirectoryId`, `GroupName`, `MemberName`

### Disable

- Operations: `DisableUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DirectoryId`, `SAMAccountName`

### Remove

- Operations: `RemoveGroupMember`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DirectoryId`, `GroupName`, `MemberName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddGroupMember` | `POST /GroupMemberships/AddGroupMember` | `idempotency-token` | `DirectoryId`, `GroupName`, `MemberName` | `ClientToken` | `AddGroupMemberResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds an existing user, group, or computer as a group member. |
| `CreateGroup` | `POST /Groups/CreateGroup` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `CreateGroupResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a new group. |
| `CreateUser` | `POST /Users/CreateUser` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `CreateUserResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a new user. |
| `DeleteGroup` | `POST /Groups/DeleteGroup` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `DeleteGroupResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a group. |
| `DeleteUser` | `POST /Users/DeleteUser` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `DeleteUserResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a user. |
| `DescribeGroup` | `POST /Groups/DescribeGroup` | `readonly` | `DirectoryId`, `SAMAccountName` | - | `DescribeGroupResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific group. |
| `DescribeUser` | `POST /Users/DescribeUser` | `readonly` | `DirectoryId`, `SAMAccountName` | - | `DescribeUserResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific user. |
| `DisableUser` | `POST /Users/DisableUser` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `DisableUserResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deactivates an active user account. For information about how to enable an inactive user account, see ResetUserPassword in the Directory Service API Reference . |
| `ListGroupMembers` | `POST /GroupMemberships/ListGroupMembers` | `readonly`, `paginated` | `DirectoryId`, `SAMAccountName` | - | `ListGroupMembersResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns member information for the specified group. This operation supports pagination with the use of the `NextToken` request and response parameters. |
| `ListGroups` | `POST /Groups/ListGroups` | `readonly`, `paginated` | `DirectoryId` | - | `ListGroupsResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns group information for the specified directory. This operation supports pagination with the use of the `NextToken` request and response parameters. |
| `ListGroupsForMember` | `POST /GroupMemberships/ListGroupsForMember` | `readonly`, `paginated` | `DirectoryId`, `SAMAccountName` | - | `ListGroupsForMemberResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns group information for the specified member. This operation supports pagination with the use of the `NextToken` request and response parameters. |
| `ListUsers` | `POST /Users/ListUsers` | `readonly`, `paginated` | `DirectoryId` | - | `ListUsersResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns user information for the specified directory. This operation supports pagination with the use of the `NextToken` request and response parameters. |
| `RemoveGroupMember` | `POST /GroupMemberships/RemoveGroupMember` | `idempotency-token` | `DirectoryId`, `GroupName`, `MemberName` | `ClientToken` | `RemoveGroupMemberResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a member from a group. |
| `SearchGroups` | `POST /Groups/SearchGroups` | `readonly`, `paginated` | `DirectoryId`, `SearchAttributes`, `SearchString` | - | `SearchGroupsResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches the specified directory for a group. You can find groups that match the `SearchString` parameter with the value of their attributes included in the `SearchString` parameter. |
| `SearchUsers` | `POST /Users/SearchUsers` | `readonly`, `paginated` | `DirectoryId`, `SearchAttributes`, `SearchString` | - | `SearchUsersResult` | `AccessDeniedException`, `DirectoryUnavailableException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches the specified directory for a user. You can find users that match the `SearchString` parameter with the value of their attributes included in the `SearchString` parameter. |
| `UpdateGroup` | `POST /Groups/UpdateGroup` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `UpdateGroupResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates group information. |
| `UpdateUser` | `POST /Users/UpdateUser` | `idempotency-token` | `DirectoryId`, `SAMAccountName` | `ClientToken` | `UpdateUserResult` | `AccessDeniedException`, `ConflictException`, `DirectoryUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates user information. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AddGroupMember` | - | `DirectoryId -> DirectoryId` | - | - |
| `CreateGroup` | - | `DirectoryId -> DirectoryId` | - | - |
| `CreateUser` | - | `DirectoryId -> DirectoryId` | - | - |
| `DeleteGroup` | - | `DirectoryId -> DirectoryId` | - | - |
| `DeleteUser` | - | `DirectoryId -> DirectoryId` | - | - |
| `DescribeGroup` | - | `DirectoryId -> DirectoryId` | - | - |
| `DescribeUser` | - | `DirectoryId -> DirectoryId` | - | - |
| `DisableUser` | - | `DirectoryId -> DirectoryId` | - | - |
| `ListGroupMembers` | - | `DirectoryId -> DirectoryId` | - | - |
| `ListGroups` | - | `DirectoryId -> DirectoryId` | - | - |
| `ListGroupsForMember` | - | `DirectoryId -> DirectoryId` | - | - |
| `ListUsers` | - | `DirectoryId -> DirectoryId` | - | - |
| `RemoveGroupMember` | - | `DirectoryId -> DirectoryId` | - | - |
| `SearchGroups` | - | `DirectoryId -> DirectoryId` | - | - |
| `SearchUsers` | - | `DirectoryId -> DirectoryId` | - | - |
| `UpdateGroup` | - | `DirectoryId -> DirectoryId` | - | - |
| `UpdateUser` | - | `DirectoryId -> DirectoryId` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message`, `Reason` | You don't have permission to perform the request or access the directory. |
| `DirectoryUnavailableException` | `structure` | `Message`, `Reason` | The request could not be completed due to a problem in the configuration or current state of the specified directory. |
| `InternalServerException` | `structure` | `Message` | The operation didn't succeed because an internal error occurred. |
| `ThrottlingException` | `structure` | `Message`, `RetryAfterSeconds` | The limit on the number of requests per second has been exceeded. |
| `ValidationException` | `structure` | `Message`, `Reason` | The request isn't valid. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource couldn't be found. |
| `ConflictException` | `structure` | `Message` | This error will occur when you try to create a resource that conflicts with an existing object. |
| `AddGroupMemberRequest` | `structure` | `ClientToken`, `DirectoryId`, `GroupName`, `MemberName`, `MemberRealm` | - |
| `AddGroupMemberResult` | `structure` | - | - |
| `CreateGroupRequest` | `structure` | `ClientToken`, `DirectoryId`, `GroupScope`, `GroupType`, `OtherAttributes`, `SAMAccountName` | - |
| `CreateGroupResult` | `structure` | `DirectoryId`, `SAMAccountName`, `SID` | - |
| `CreateUserRequest` | `structure` | `ClientToken`, `DirectoryId`, `EmailAddress`, `GivenName`, `OtherAttributes`, `SAMAccountName`, `Surname` | - |
| `CreateUserResult` | `structure` | `DirectoryId`, `SAMAccountName`, `SID` | - |
| `DeleteGroupRequest` | `structure` | `ClientToken`, `DirectoryId`, `SAMAccountName` | - |
| `DeleteGroupResult` | `structure` | - | - |
| `DeleteUserRequest` | `structure` | `ClientToken`, `DirectoryId`, `SAMAccountName` | - |
| `DeleteUserResult` | `structure` | - | - |
| `DescribeGroupRequest` | `structure` | `DirectoryId`, `OtherAttributes`, `Realm`, `SAMAccountName` | - |
| `DescribeGroupResult` | `structure` | `DirectoryId`, `DistinguishedName`, `GroupScope`, `GroupType`, `OtherAttributes`, `Realm`, `SAMAccountName`, `SID` | - |
| `DescribeUserRequest` | `structure` | `DirectoryId`, `OtherAttributes`, `Realm`, `SAMAccountName` | - |
| `DescribeUserResult` | `structure` | `DirectoryId`, `DistinguishedName`, `EmailAddress`, `Enabled`, `GivenName`, `OtherAttributes`, `Realm`, `SAMAccountName`, `SID`, `Surname`, `UserPrincipalName` | - |
| `DisableUserRequest` | `structure` | `ClientToken`, `DirectoryId`, `SAMAccountName` | - |
| `DisableUserResult` | `structure` | - | - |
| `ListGroupMembersRequest` | `structure` | `DirectoryId`, `MaxResults`, `MemberRealm`, `NextToken`, `Realm`, `SAMAccountName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

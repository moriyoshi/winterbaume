# FinSpace Public API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The FinSpace APIs let you take actions inside the FinSpace.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for FinSpace Public API where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for FinSpace Public API by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for FinSpace Public API by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented FinSpace Public API workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Update`, `Delete` operation families, including `GetChangeset`, `GetDataView`, `GetDataset`, `GetExternalDataViewAccessDetails`, `ListChangesets`, `ListDataViews`.

## Service Identity and Protocol

- AWS model slug: `finspace-data`
- AWS SDK for Rust slug: `finspacedata`
- Model version: `2020-07-13`
- Model file: `vendor/api-models-aws/models/finspace-data/service/2020-07-13/finspace-data-2020-07-13.json`
- SDK ID: `finspace data`
- Endpoint prefix: `finspace-api`
- ARN namespace: `finspace-api`
- CloudFormation name: `Finspacedata`
- CloudTrail event source: `finspacedata.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (8), `List` (7), `Create` (5), `Update` (4), `Delete` (2), `Associate` (1), `Disable` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateUserToPermissionGroup`, `CreateChangeset`, `CreateDataView`, `CreateDataset`, `CreatePermissionGroup`, `CreateUser`, `DeleteDataset`, `DeletePermissionGroup`, `DisableUser`, `DisassociateUserFromPermissionGroup`, `EnableUser`, `UpdateChangeset`, `UpdateDataset`, `UpdatePermissionGroup`, `UpdateUser`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetChangeset`, `GetDataView`, `GetDataset`, `GetExternalDataViewAccessDetails`, `GetPermissionGroup`, `GetProgrammaticAccessCredentials`, `GetUser`, `GetWorkingLocation`, `ListChangesets`, `ListDataViews`, `ListDatasets`, `ListPermissionGroups`, `ListPermissionGroupsByUser`, `ListUsers`, `ListUsersByPermissionGroup`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 16 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Glue`.

## Operation Groups

### Get

- Operations: `GetChangeset`, `GetDataView`, `GetDataset`, `GetExternalDataViewAccessDetails`, `GetPermissionGroup`, `GetProgrammaticAccessCredentials`, `GetUser`, `GetWorkingLocation`
- Common required input members in this group: `changesetId`, `dataViewId`, `datasetId`, `environmentId`, `permissionGroupId`, `userId`

### List

- Operations: `ListChangesets`, `ListDataViews`, `ListDatasets`, `ListPermissionGroups`, `ListPermissionGroupsByUser`, `ListUsers`, `ListUsersByPermissionGroup`
- Traits: `paginated` (5)
- Common required input members in this group: `datasetId`, `maxResults`, `permissionGroupId`, `userId`

### Create

- Operations: `CreateChangeset`, `CreateDataView`, `CreateDataset`, `CreatePermissionGroup`, `CreateUser`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `applicationPermissions`, `changeType`, `datasetId`, `datasetTitle`, `destinationTypeParams`, `emailAddress`, `formatParams`, `kind`, `name`, `permissionGroupParams`, `sourceParams`, `type`

### Update

- Operations: `UpdateChangeset`, `UpdateDataset`, `UpdatePermissionGroup`, `UpdateUser`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `changesetId`, `datasetId`, `datasetTitle`, `formatParams`, `kind`, `permissionGroupId`, `sourceParams`, `userId`

### Delete

- Operations: `DeleteDataset`, `DeletePermissionGroup`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `datasetId`, `permissionGroupId`

### Associate

- Operations: `AssociateUserToPermissionGroup`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `permissionGroupId`, `userId`

### Disable

- Operations: `DisableUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `userId`

### Disassociate

- Operations: `DisassociateUserFromPermissionGroup`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `permissionGroupId`, `userId`

### Enable

- Operations: `EnableUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `userId`

### Reset

- Operations: `ResetUserPassword`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `userId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateUserToPermissionGroup` | `POST /permission-group/{permissionGroupId}/users/{userId}` | `idempotency-token` | `permissionGroupId`, `userId` | `clientToken` | `AssociateUserToPermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a user to a permission group to grant permissions for actions a user can perform in FinSpace. |
| `CreateChangeset` | `POST /datasets/{datasetId}/changesetsv2` | `idempotency-token` | `changeType`, `datasetId`, `formatParams`, `sourceParams` | `clientToken` | `CreateChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new Changeset in a FinSpace Dataset. |
| `CreateDataView` | `POST /datasets/{datasetId}/dataviewsv2` | `idempotency-token` | `datasetId`, `destinationTypeParams` | `clientToken` | `CreateDataViewResponse` | `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a Dataview for a Dataset. |
| `CreateDataset` | `POST /datasetsv2` | `idempotency-token` | `datasetTitle`, `kind`, `permissionGroupParams` | `clientToken` | `CreateDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new FinSpace Dataset. |
| `CreatePermissionGroup` | `POST /permission-group` | `idempotency-token` | `applicationPermissions`, `name` | `clientToken` | `CreatePermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a group of permissions for various actions that a user can perform in FinSpace. |
| `CreateUser` | `POST /user` | `idempotency-token` | `emailAddress`, `type` | `clientToken` | `CreateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a new user in FinSpace. |
| `DeleteDataset` | `DELETE /datasetsv2/{datasetId}` | `idempotency-token` | `datasetId` | `clientToken` | `DeleteDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a FinSpace Dataset. |
| `DeletePermissionGroup` | `DELETE /permission-group/{permissionGroupId}` | `idempotency-token` | `permissionGroupId` | `clientToken` | `DeletePermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a permission group. This action is irreversible. |
| `DisableUser` | `POST /user/{userId}/disable` | `idempotency-token` | `userId` | `clientToken` | `DisableUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Denies access to the FinSpace web application and API for the specified user. |
| `DisassociateUserFromPermissionGroup` | `DELETE /permission-group/{permissionGroupId}/users/{userId}` | `idempotency-token` | `permissionGroupId`, `userId` | `clientToken` | `DisassociateUserFromPermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a user from a permission group. |
| `EnableUser` | `POST /user/{userId}/enable` | `idempotency-token` | `userId` | `clientToken` | `EnableUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows the specified user to access the FinSpace web application and API. |
| `GetChangeset` | `GET /datasets/{datasetId}/changesetsv2/{changesetId}` | - | `changesetId`, `datasetId` | - | `GetChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get information about a Changeset. |
| `GetDataView` | `GET /datasets/{datasetId}/dataviewsv2/{dataViewId}` | - | `dataViewId`, `datasetId` | - | `GetDataViewResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a Dataview. |
| `GetDataset` | `GET /datasetsv2/{datasetId}` | - | `datasetId` | - | `GetDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a Dataset. |
| `GetExternalDataViewAccessDetails` | `POST /datasets/{datasetId}/dataviewsv2/{dataViewId}/external-access-details` | - | `dataViewId`, `datasetId` | - | `GetExternalDataViewAccessDetailsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the credentials to access the external Dataview from an S3 location. To call this API: You must retrieve the programmatic credentials. |
| `GetPermissionGroup` | `GET /permission-group/{permissionGroupId}` | - | `permissionGroupId` | - | `GetPermissionGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of a specific permission group. |
| `GetProgrammaticAccessCredentials` | `GET /credentials/programmatic` | - | `environmentId` | - | `GetProgrammaticAccessCredentialsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Request programmatic credentials to use with FinSpace SDK. For more information, see Step 2. |
| `GetUser` | `GET /user/{userId}` | - | `userId` | - | `GetUserResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details for a specific user. |
| `GetWorkingLocation` | `POST /workingLocationV1` | - | - | - | `GetWorkingLocationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | A temporary Amazon S3 location, where you can copy your files from a source location to stage or use as a scratch space in FinSpace notebook. |
| `ListChangesets` | `GET /datasets/{datasetId}/changesetsv2` | `paginated` | `datasetId` | - | `ListChangesetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the FinSpace Changesets for a Dataset. |
| `ListDataViews` | `GET /datasets/{datasetId}/dataviewsv2` | `paginated` | `datasetId` | - | `ListDataViewsResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all available Dataviews for a Dataset. |
| `ListDatasets` | `GET /datasetsv2` | `paginated` | - | - | `ListDatasetsResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all of the active Datasets that a user has access to. |
| `ListPermissionGroups` | `GET /permission-group` | `paginated` | `maxResults` | - | `ListPermissionGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available permission groups in FinSpace. |
| `ListPermissionGroupsByUser` | `GET /user/{userId}/permission-groups` | - | `maxResults`, `userId` | - | `ListPermissionGroupsByUserResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the permission groups that are associated with a specific user. |
| `ListUsers` | `GET /user` | `paginated` | `maxResults` | - | `ListUsersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available users in FinSpace. |
| `ListUsersByPermissionGroup` | `GET /permission-group/{permissionGroupId}/users` | - | `maxResults`, `permissionGroupId` | - | `ListUsersByPermissionGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists details of all the users in a specific permission group. |
| `ResetUserPassword` | `POST /user/{userId}/password` | `idempotency-token` | `userId` | `clientToken` | `ResetUserPasswordResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Resets the password for a specified user ID and generates a temporary one. Only a superuser can reset password for other users. |
| `UpdateChangeset` | `PUT /datasets/{datasetId}/changesetsv2/{changesetId}` | `idempotency-token` | `changesetId`, `datasetId`, `formatParams`, `sourceParams` | `clientToken` | `UpdateChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a FinSpace Changeset. |
| `UpdateDataset` | `PUT /datasetsv2/{datasetId}` | `idempotency-token` | `datasetId`, `datasetTitle`, `kind` | `clientToken` | `UpdateDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a FinSpace Dataset. |
| `UpdatePermissionGroup` | `PUT /permission-group/{permissionGroupId}` | `idempotency-token` | `permissionGroupId` | `clientToken` | `UpdatePermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the details of a permission group. You cannot modify a `permissionGroupID`. |
| `UpdateUser` | `PUT /user/{userId}` | `idempotency-token` | `userId` | `clientToken` | `UpdateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the details of the specified user. You cannot update the `userId` for a user. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteDataset` | - | `clientToken -> clientToken` | - | - |
| `DeletePermissionGroup` | - | `clientToken -> clientToken` | - | - |
| `DisassociateUserFromPermissionGroup` | - | `clientToken -> clientToken` | - | - |
| `GetProgrammaticAccessCredentials` | - | `durationInMinutes -> durationInMinutes`, `environmentId -> environmentId` | - | - |
| `ListChangesets` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListDatasets` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDataViews` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPermissionGroups` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPermissionGroupsByUser` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListUsers` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListUsersByPermissionGroup` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | - | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message`, `reason` | The input fails to satisfy the constraints specified by an AWS service. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `ResourceNotFoundException` | `structure` | `message`, `reason` | One or more resources can't be found. |
| `ConflictException` | `structure` | `message`, `reason` | The request conflicts with an existing resource. |
| `LimitExceededException` | `structure` | `message` | A limit has exceeded. |
| `AssociateUserToPermissionGroupRequest` | `structure` | `clientToken`, `permissionGroupId`, `userId` | - |
| `AssociateUserToPermissionGroupResponse` | `structure` | `statusCode` | - |
| `CreateChangesetRequest` | `structure` | `changeType`, `clientToken`, `datasetId`, `formatParams`, `sourceParams` | The request for a CreateChangeset operation. |
| `CreateChangesetResponse` | `structure` | `changesetId`, `datasetId` | The response from a CreateChangeset operation. |
| `CreateDataViewRequest` | `structure` | `asOfTimestamp`, `autoUpdate`, `clientToken`, `datasetId`, `destinationTypeParams`, `partitionColumns`, `sortColumns` | Request for creating a data view. |
| `CreateDataViewResponse` | `structure` | `dataViewId`, `datasetId` | Response for creating a data view. |
| `CreateDatasetRequest` | `structure` | `alias`, `clientToken`, `datasetDescription`, `datasetTitle`, `kind`, `ownerInfo`, `permissionGroupParams`, `schemaDefinition` | The request for a CreateDataset operation |
| `CreateDatasetResponse` | `structure` | `datasetId` | The response from a CreateDataset operation |
| `CreatePermissionGroupRequest` | `structure` | `applicationPermissions`, `clientToken`, `description`, `name` | - |
| `CreatePermissionGroupResponse` | `structure` | `permissionGroupId` | - |
| `CreateUserRequest` | `structure` | `apiAccess`, `apiAccessPrincipalArn`, `clientToken`, `emailAddress`, `firstName`, `lastName`, `type` | - |
| `CreateUserResponse` | `structure` | `userId` | - |
| `DeleteDatasetRequest` | `structure` | `clientToken`, `datasetId` | The request for a DeleteDataset operation. |
| `DeleteDatasetResponse` | `structure` | `datasetId` | The response from an DeleteDataset operation |
| `DeletePermissionGroupRequest` | `structure` | `clientToken`, `permissionGroupId` | - |
| `DeletePermissionGroupResponse` | `structure` | `permissionGroupId` | - |
| `DisableUserRequest` | `structure` | `clientToken`, `userId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

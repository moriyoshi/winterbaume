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

- Operations: `GetChangeset`, `GetDataset`, `GetDataView`, `GetExternalDataViewAccessDetails`, `GetPermissionGroup`, `GetProgrammaticAccessCredentials`, `GetUser`, `GetWorkingLocation`
- Common required input members in this group: `datasetId`, `dataViewId`

### List

- Operations: `ListChangesets`, `ListDatasets`, `ListDataViews`, `ListPermissionGroups`, `ListPermissionGroupsByUser`, `ListUsers`, `ListUsersByPermissionGroup`
- Traits: `paginated` (5)
- Common required input members in this group: `datasetId`, `maxResults`

### Create

- Operations: `CreateChangeset`, `CreateDataset`, `CreateDataView`, `CreatePermissionGroup`, `CreateUser`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `datasetId`

### Update

- Operations: `UpdateChangeset`, `UpdateDataset`, `UpdatePermissionGroup`, `UpdateUser`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `datasetId`

### Delete

- Operations: `DeleteDataset`, `DeletePermissionGroup`
- Traits: `idempotency-token` (2)
- Common required input members in this group: -

### Associate

- Operations: `AssociateUserToPermissionGroup`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Disable

- Operations: `DisableUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateUserFromPermissionGroup`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Enable

- Operations: `EnableUser`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Reset

- Operations: `ResetUserPassword`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateUserToPermissionGroup` | `POST /permission-group/{permissionGroupId}/users/{userId}` | `idempotency-token` | `permissionGroupId`, `userId` | `clientToken` | `AssociateUserToPermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a user to a permission group to grant permissions for actions a user can perform in FinSpace. |
| `CreateChangeset` | `POST /datasets/{datasetId}/changesetsv2` | `idempotency-token` | `datasetId`, `changeType`, `sourceParams`, `formatParams` | `clientToken` | `CreateChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new Changeset in a FinSpace Dataset. |
| `CreateDataset` | `POST /datasetsv2` | `idempotency-token` | `datasetTitle`, `kind`, `permissionGroupParams` | `clientToken` | `CreateDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new FinSpace Dataset. |
| `CreateDataView` | `POST /datasets/{datasetId}/dataviewsv2` | `idempotency-token` | `datasetId`, `destinationTypeParams` | `clientToken` | `CreateDataViewResponse` | `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a Dataview for a Dataset. |
| `CreatePermissionGroup` | `POST /permission-group` | `idempotency-token` | `name`, `applicationPermissions` | `clientToken` | `CreatePermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a group of permissions for various actions that a user can perform in FinSpace. |
| `CreateUser` | `POST /user` | `idempotency-token` | `emailAddress`, `type` | `clientToken` | `CreateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a new user in FinSpace. |
| `DeleteDataset` | `DELETE /datasetsv2/{datasetId}` | `idempotency-token` | `datasetId` | `clientToken` | `DeleteDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a FinSpace Dataset. |
| `DeletePermissionGroup` | `DELETE /permission-group/{permissionGroupId}` | `idempotency-token` | `permissionGroupId` | `clientToken` | `DeletePermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a permission group. This action is irreversible. |
| `DisableUser` | `POST /user/{userId}/disable` | `idempotency-token` | `userId` | `clientToken` | `DisableUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Denies access to the FinSpace web application and API for the specified user. |
| `DisassociateUserFromPermissionGroup` | `DELETE /permission-group/{permissionGroupId}/users/{userId}` | `idempotency-token` | `permissionGroupId`, `userId` | `clientToken` | `DisassociateUserFromPermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a user from a permission group. |
| `EnableUser` | `POST /user/{userId}/enable` | `idempotency-token` | `userId` | `clientToken` | `EnableUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows the specified user to access the FinSpace web application and API. |
| `GetChangeset` | `GET /datasets/{datasetId}/changesetsv2/{changesetId}` | - | `datasetId`, `changesetId` | - | `GetChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get information about a Changeset. |
| `GetDataset` | `GET /datasetsv2/{datasetId}` | - | `datasetId` | - | `GetDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a Dataset. |
| `GetDataView` | `GET /datasets/{datasetId}/dataviewsv2/{dataViewId}` | - | `dataViewId`, `datasetId` | - | `GetDataViewResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a Dataview. |
| `GetExternalDataViewAccessDetails` | `POST /datasets/{datasetId}/dataviewsv2/{dataViewId}/external-access-details` | - | `dataViewId`, `datasetId` | - | `GetExternalDataViewAccessDetailsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the credentials to access the external Dataview from an S3 location. To call this API: You must retrieve the programmatic credentials. You must be a member of a FinSpace user group, where the dataset that you ... |
| `GetPermissionGroup` | `GET /permission-group/{permissionGroupId}` | - | `permissionGroupId` | - | `GetPermissionGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of a specific permission group. |
| `GetProgrammaticAccessCredentials` | `GET /credentials/programmatic` | - | `environmentId` | - | `GetProgrammaticAccessCredentialsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Request programmatic credentials to use with FinSpace SDK. For more information, see Step 2. Access credentials programmatically using IAM access key id and secret access key . |
| `GetUser` | `GET /user/{userId}` | - | `userId` | - | `GetUserResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details for a specific user. |
| `GetWorkingLocation` | `POST /workingLocationV1` | - | - | - | `GetWorkingLocationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | A temporary Amazon S3 location, where you can copy your files from a source location to stage or use as a scratch space in FinSpace notebook. |
| `ListChangesets` | `GET /datasets/{datasetId}/changesetsv2` | `paginated` | `datasetId` | - | `ListChangesetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the FinSpace Changesets for a Dataset. |
| `ListDatasets` | `GET /datasetsv2` | `paginated` | - | - | `ListDatasetsResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all of the active Datasets that a user has access to. |
| `ListDataViews` | `GET /datasets/{datasetId}/dataviewsv2` | `paginated` | `datasetId` | - | `ListDataViewsResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all available Dataviews for a Dataset. |
| `ListPermissionGroups` | `GET /permission-group` | `paginated` | `maxResults` | - | `ListPermissionGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available permission groups in FinSpace. |
| `ListPermissionGroupsByUser` | `GET /user/{userId}/permission-groups` | - | `userId`, `maxResults` | - | `ListPermissionGroupsByUserResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the permission groups that are associated with a specific user. |
| `ListUsers` | `GET /user` | `paginated` | `maxResults` | - | `ListUsersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available users in FinSpace. |
| `ListUsersByPermissionGroup` | `GET /permission-group/{permissionGroupId}/users` | - | `permissionGroupId`, `maxResults` | - | `ListUsersByPermissionGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists details of all the users in a specific permission group. |
| `ResetUserPassword` | `POST /user/{userId}/password` | `idempotency-token` | `userId` | `clientToken` | `ResetUserPasswordResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Resets the password for a specified user ID and generates a temporary one. Only a superuser can reset password for other users. Resetting the password immediately invalidates the previous password associated with the ... |
| `UpdateChangeset` | `PUT /datasets/{datasetId}/changesetsv2/{changesetId}` | `idempotency-token` | `datasetId`, `changesetId`, `sourceParams`, `formatParams` | `clientToken` | `UpdateChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a FinSpace Changeset. |
| `UpdateDataset` | `PUT /datasetsv2/{datasetId}` | `idempotency-token` | `datasetId`, `datasetTitle`, `kind` | `clientToken` | `UpdateDatasetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a FinSpace Dataset. |
| `UpdatePermissionGroup` | `PUT /permission-group/{permissionGroupId}` | `idempotency-token` | `permissionGroupId` | `clientToken` | `UpdatePermissionGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the details of a permission group. You cannot modify a permissionGroupID . |
| `UpdateUser` | `PUT /user/{userId}` | `idempotency-token` | `userId` | `clientToken` | `UpdateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the details of the specified user. You cannot update the userId for a user. |

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
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, reason | The request conflicts with an existing resource. |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure. |
| `LimitExceededException` | `structure` | message | A limit has exceeded. |
| `ResourceNotFoundException` | `structure` | message, reason | One or more resources can't be found. |
| `ThrottlingException` | `structure` | **empty (no members)** | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason | The input fails to satisfy the constraints specified by an AWS service. |
| `AssociateUserToPermissionGroupRequest` | `structure` | permissionGroupId, userId, clientToken | - |
| `AssociateUserToPermissionGroupResponse` | `structure` | statusCode | - |
| `CreateChangesetRequest` | `structure` | clientToken, datasetId, changeType, sourceParams, formatParams | The request for a CreateChangeset operation. |
| `CreateChangesetResponse` | `structure` | datasetId, changesetId | The response from a CreateChangeset operation. |
| `CreateDatasetRequest` | `structure` | clientToken, datasetTitle, kind, datasetDescription, ownerInfo, permissionGroupParams, alias, schemaDefinition | The request for a CreateDataset operation |
| `CreateDatasetResponse` | `structure` | datasetId | The response from a CreateDataset operation |
| `CreateDataViewRequest` | `structure` | clientToken, datasetId, autoUpdate, sortColumns, partitionColumns, asOfTimestamp, destinationTypeParams | Request for creating a data view. |
| `CreateDataViewResponse` | `structure` | datasetId, dataViewId | Response for creating a data view. |
| `CreatePermissionGroupRequest` | `structure` | name, description, applicationPermissions, clientToken | - |
| `CreatePermissionGroupResponse` | `structure` | permissionGroupId | - |
| `CreateUserRequest` | `structure` | emailAddress, type, firstName, lastName, apiAccess, apiAccessPrincipalArn, clientToken | - |
| `CreateUserResponse` | `structure` | userId | - |
| `DeleteDatasetRequest` | `structure` | clientToken, datasetId | The request for a DeleteDataset operation. |
| `DeleteDatasetResponse` | `structure` | datasetId | The response from an DeleteDataset operation |
| `DeletePermissionGroupRequest` | `structure` | permissionGroupId, clientToken | - |
| `DeletePermissionGroupResponse` | `structure` | permissionGroupId | - |
| `DisableUserRequest` | `structure` | userId, clientToken | - |
| `DisableUserResponse` | `structure` | userId | - |
| `DisassociateUserFromPermissionGroupRequest` | `structure` | permissionGroupId, userId, clientToken | - |
| `DisassociateUserFromPermissionGroupResponse` | `structure` | statusCode | - |
| `EnableUserRequest` | `structure` | userId, clientToken | - |
| `EnableUserResponse` | `structure` | userId | - |
| `GetChangesetRequest` | `structure` | datasetId, changesetId | Request to describe a changeset. |
| `GetChangesetResponse` | `structure` | changesetId, changesetArn, datasetId, changeType, sourceParams, formatParams, createTime, status, errorInfo, activeUntilTimestamp, activeFromTimestamp, updatesChangesetId, ... (+1) | The response from a describe changeset operation |
| `GetDatasetRequest` | `structure` | datasetId | Request for the GetDataset operation. |
| `GetDatasetResponse` | `structure` | datasetId, datasetArn, datasetTitle, kind, datasetDescription, createTime, lastModifiedTime, schemaDefinition, alias, status | Response for the GetDataset operation |
| `GetDataViewRequest` | `structure` | dataViewId, datasetId | Request for retrieving a data view detail. Grouped / accessible within a dataset by its dataset id. |
| `GetDataViewResponse` | `structure` | autoUpdate, partitionColumns, datasetId, asOfTimestamp, errorInfo, lastModifiedTime, createTime, sortColumns, dataViewId, dataViewArn, destinationTypeParams, status | Response from retrieving a dataview, which includes details on the target database and table name |
| `GetExternalDataViewAccessDetailsRequest` | `structure` | dataViewId, datasetId | - |
| `GetExternalDataViewAccessDetailsResponse` | `structure` | credentials, s3Location | - |
| `GetPermissionGroupRequest` | `structure` | permissionGroupId | - |
| `GetPermissionGroupResponse` | `structure` | permissionGroup | - |
| `GetProgrammaticAccessCredentialsRequest` | `structure` | durationInMinutes, environmentId | Request for GetProgrammaticAccessCredentials operation |
| `ApiAccess` | `enum` | ENABLED, DISABLED | - |
| `ApplicationPermission` | `enum` | CreateDataset, ManageClusters, ManageUsersAndGroups, ManageAttributeSets, ViewAuditData, AccessNotebooks, GetTemporaryCredentials | - |
| `ChangeType` | `enum` | REPLACE, APPEND, MODIFY | Indicates how the given change will be applied to the dataset. |
| `ColumnDataType` | `enum` | STRING, CHAR, INTEGER, TINYINT, SMALLINT, BIGINT, FLOAT, DOUBLE, DATE, DATETIME, BOOLEAN, BINARY | Data type of a column. |
| `DataViewStatus` | `enum` | RUNNING, STARTING, FAILED, CANCELLED, TIMEOUT, SUCCESS, PENDING, FAILED_CLEANUP_FAILED | Status of a DataView |
| `DatasetKind` | `enum` | TABULAR, NON_TABULAR | Dataset Kind |
| `DatasetStatus` | `enum` | PENDING, FAILED, SUCCESS, RUNNING | Status of the dataset process returned from scheduler service. |
| `ErrorCategory` | `enum` | VALIDATION, SERVICE_QUOTA_EXCEEDED, ACCESS_DENIED, RESOURCE_NOT_FOUND, THROTTLING, INTERNAL_SERVICE_EXCEPTION, CANCELLED, USER_RECOVERABLE | Changeset Error Category |
| `ExportFileFormat` | `enum` | PARQUET, DELIMITED_TEXT | Data View Export File Format |
| `IngestionStatus` | `enum` | PENDING, FAILED, SUCCESS, RUNNING, STOP_REQUESTED | Status of the ingestion process returned from scheduler service. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

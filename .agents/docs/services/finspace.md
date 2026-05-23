# FinSpace User Environment Management service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The FinSpace management service provides the APIs for managing FinSpace environments.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented FinSpace User Environment Management service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListEnvironments`, `ListKxChangesets`, `ListKxClusterNodes`, `ListKxClusters`, `GetEnvironment`, `GetKxChangeset`.

## Service Identity and Protocol

- AWS model slug: `finspace`
- AWS SDK for Rust slug: `finspace`
- Model version: `2021-03-12`
- Model file: `vendor/api-models-aws/models/finspace/service/2021-03-12/finspace-2021-03-12.json`
- SDK ID: `finspace`
- Endpoint prefix: `finspace`
- ARN namespace: `finspace`
- CloudFormation name: `Finspace`
- CloudTrail event source: `finspace.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Get` (10), `Create` (9), `Delete` (9), `Update` (9), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateEnvironment`, `CreateKxChangeset`, `CreateKxCluster`, `CreateKxDatabase`, `CreateKxDataview`, `CreateKxEnvironment`, `CreateKxScalingGroup`, `CreateKxUser`, `CreateKxVolume`, `DeleteEnvironment`, `DeleteKxCluster`, `DeleteKxClusterNode`, `DeleteKxDatabase`, `DeleteKxDataview`, `DeleteKxEnvironment`, `DeleteKxScalingGroup`, `DeleteKxUser`, `DeleteKxVolume`, `TagResource`, `UntagResource`, ... (+9).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEnvironment`, `GetKxChangeset`, `GetKxCluster`, `GetKxConnectionString`, `GetKxDatabase`, `GetKxDataview`, `GetKxEnvironment`, `GetKxScalingGroup`, `GetKxUser`, `GetKxVolume`, `ListEnvironments`, `ListKxChangesets`, `ListKxClusterNodes`, `ListKxClusters`, `ListKxDatabases`, `ListKxDataviews`, `ListKxEnvironments`, `ListKxScalingGroups`, `ListKxUsers`, `ListKxVolumes`, ... (+1).
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 23 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 50 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Glue`, `EC2/VPC`, `ECS`.

## Operation Groups

### List

- Operations: `ListEnvironments`, `ListKxChangesets`, `ListKxClusterNodes`, `ListKxClusters`, `ListKxDatabases`, `ListKxDataviews`, `ListKxEnvironments`, `ListKxScalingGroups`, `ListKxUsers`, `ListKxVolumes`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `environmentId`, `databaseName`

### Get

- Operations: `GetEnvironment`, `GetKxChangeset`, `GetKxCluster`, `GetKxConnectionString`, `GetKxDatabase`, `GetKxDataview`, `GetKxEnvironment`, `GetKxScalingGroup`, `GetKxUser`, `GetKxVolume`
- Common required input members in this group: `environmentId`, `databaseName`, `clusterName`

### Create

- Operations: `CreateEnvironment`, `CreateKxChangeset`, `CreateKxCluster`, `CreateKxDatabase`, `CreateKxDataview`, `CreateKxEnvironment`, `CreateKxScalingGroup`, `CreateKxUser`, `CreateKxVolume`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `name`, `environmentId`, `databaseName`, `clientToken`, `azMode`

### Delete

- Operations: `DeleteEnvironment`, `DeleteKxCluster`, `DeleteKxClusterNode`, `DeleteKxDatabase`, `DeleteKxDataview`, `DeleteKxEnvironment`, `DeleteKxScalingGroup`, `DeleteKxUser`, `DeleteKxVolume`
- Traits: `idempotency-token` (7)
- Common required input members in this group: `environmentId`, `clusterName`, `databaseName`, `clientToken`

### Update

- Operations: `UpdateEnvironment`, `UpdateKxClusterCodeConfiguration`, `UpdateKxClusterDatabases`, `UpdateKxDatabase`, `UpdateKxDataview`, `UpdateKxEnvironment`, `UpdateKxEnvironmentNetwork`, `UpdateKxUser`, `UpdateKxVolume`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `environmentId`, `clusterName`, `databaseName`, `clientToken`

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEnvironment` | `POST /environment` | - | `name` | - | `CreateEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new FinSpace environment. |
| `CreateKxChangeset` | `POST /kx/environments/{environmentId}/databases/{databaseName}/changesets` | `idempotency-token` | `environmentId`, `databaseName`, `changeRequests`, `clientToken` | `clientToken` | `CreateKxChangesetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a changeset for a kdb database. A changeset allows you to add and delete existing files by using an ordered list of change requests. |
| `CreateKxCluster` | `POST /kx/environments/{environmentId}/clusters` | `idempotency-token` | `environmentId`, `clusterName`, `clusterType`, `releaseLabel`, `vpcConfiguration`, `azMode` | `clientToken` | `CreateKxClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new kdb cluster. |
| `CreateKxDatabase` | `POST /kx/environments/{environmentId}/databases` | `idempotency-token` | `environmentId`, `databaseName`, `clientToken` | `clientToken` | `CreateKxDatabaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new kdb database in the environment. |
| `CreateKxDataview` | `POST /kx/environments/{environmentId}/databases/{databaseName}/dataviews` | `idempotency-token` | `environmentId`, `databaseName`, `dataviewName`, `azMode`, `clientToken` | `clientToken` | `CreateKxDataviewResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a snapshot of kdb database with tiered storage capabilities and a pre-warmed cache, ready for mounting on kdb clusters. Dataviews are only available for clusters running on a scaling group. They are not suppo ... |
| `CreateKxEnvironment` | `POST /kx/environments` | `idempotency-token` | `name`, `kmsKeyId` | `clientToken` | `CreateKxEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a managed kdb environment for the account. |
| `CreateKxScalingGroup` | `POST /kx/environments/{environmentId}/scalingGroups` | `idempotency-token` | `clientToken`, `environmentId`, `scalingGroupName`, `hostType`, `availabilityZoneId` | `clientToken` | `CreateKxScalingGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new scaling group. |
| `CreateKxUser` | `POST /kx/environments/{environmentId}/users` | `idempotency-token` | `environmentId`, `userName`, `iamRole` | `clientToken` | `CreateKxUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a user in FinSpace kdb environment with an associated IAM role. |
| `CreateKxVolume` | `POST /kx/environments/{environmentId}/kxvolumes` | `idempotency-token` | `environmentId`, `volumeType`, `volumeName`, `azMode`, `availabilityZoneIds` | `clientToken` | `CreateKxVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new volume with a specific amount of throughput and storage capacity. |
| `DeleteEnvironment` | `DELETE /environment/{environmentId}` | - | `environmentId` | - | `DeleteEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an FinSpace environment. |
| `DeleteKxCluster` | `DELETE /kx/environments/{environmentId}/clusters/{clusterName}` | `idempotency-token` | `environmentId`, `clusterName` | `clientToken` | `DeleteKxClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a kdb cluster. |
| `DeleteKxClusterNode` | `DELETE /kx/environments/{environmentId}/clusters/{clusterName}/nodes/{nodeId}` | - | `environmentId`, `clusterName`, `nodeId` | - | `DeleteKxClusterNodeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified nodes from a cluster. |
| `DeleteKxDatabase` | `DELETE /kx/environments/{environmentId}/databases/{databaseName}` | `idempotency-token` | `environmentId`, `databaseName`, `clientToken` | `clientToken` | `DeleteKxDatabaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified database and all of its associated data. This action is irreversible. You must copy any data out of the database before deleting it if the data is to be retained. |
| `DeleteKxDataview` | `DELETE /kx/environments/{environmentId}/databases/{databaseName}/dataviews/{dataviewName}` | `idempotency-token` | `environmentId`, `databaseName`, `dataviewName`, `clientToken` | `clientToken` | `DeleteKxDataviewResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified dataview. Before deleting a dataview, make sure that it is not in use by any cluster. |
| `DeleteKxEnvironment` | `DELETE /kx/environments/{environmentId}` | `idempotency-token` | `environmentId` | `clientToken` | `DeleteKxEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the kdb environment. This action is irreversible. Deleting a kdb environment will remove all the associated data and any services running in it. |
| `DeleteKxScalingGroup` | `DELETE /kx/environments/{environmentId}/scalingGroups/{scalingGroupName}` | `idempotency-token` | `environmentId`, `scalingGroupName` | `clientToken` | `DeleteKxScalingGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified scaling group. This action is irreversible. You cannot delete a scaling group until all the clusters running on it have been deleted. |
| `DeleteKxUser` | `DELETE /kx/environments/{environmentId}/users/{userName}` | `idempotency-token` | `userName`, `environmentId` | `clientToken` | `DeleteKxUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a user in the specified kdb environment. |
| `DeleteKxVolume` | `DELETE /kx/environments/{environmentId}/kxvolumes/{volumeName}` | `idempotency-token` | `environmentId`, `volumeName` | `clientToken` | `DeleteKxVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a volume. You can only delete a volume if it's not attached to a cluster or a dataview. When a volume is deleted, any data on the volume is lost. This action is irreversible. |
| `GetEnvironment` | `GET /environment/{environmentId}` | - | `environmentId` | - | `GetEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the FinSpace environment object. |
| `GetKxChangeset` | `GET /kx/environments/{environmentId}/databases/{databaseName}/changesets/{changesetId}` | - | `environmentId`, `databaseName`, `changesetId` | - | `GetKxChangesetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a kdb changeset. |
| `GetKxCluster` | `GET /kx/environments/{environmentId}/clusters/{clusterName}` | - | `environmentId`, `clusterName` | - | `GetKxClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a kdb cluster. |
| `GetKxConnectionString` | `GET /kx/environments/{environmentId}/connectionString` | - | `userArn`, `environmentId`, `clusterName` | - | `GetKxConnectionStringResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a connection string for a user to connect to a kdb cluster. You must call this API using the same role that you have defined while creating a user. |
| `GetKxDatabase` | `GET /kx/environments/{environmentId}/databases/{databaseName}` | - | `environmentId`, `databaseName` | - | `GetKxDatabaseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns database information for the specified environment ID. |
| `GetKxDataview` | `GET /kx/environments/{environmentId}/databases/{databaseName}/dataviews/{dataviewName}` | - | `environmentId`, `databaseName`, `dataviewName` | - | `GetKxDataviewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details of the dataview. |
| `GetKxEnvironment` | `GET /kx/environments/{environmentId}` | - | `environmentId` | - | `GetKxEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves all the information for the specified kdb environment. |
| `GetKxScalingGroup` | `GET /kx/environments/{environmentId}/scalingGroups/{scalingGroupName}` | - | `environmentId`, `scalingGroupName` | - | `GetKxScalingGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details of a scaling group. |
| `GetKxUser` | `GET /kx/environments/{environmentId}/users/{userName}` | - | `userName`, `environmentId` | - | `GetKxUserResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified kdb user. |
| `GetKxVolume` | `GET /kx/environments/{environmentId}/kxvolumes/{volumeName}` | - | `environmentId`, `volumeName` | - | `GetKxVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the information about the volume. |
| `ListEnvironments` | `GET /environment` | - | - | - | `ListEnvironmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | A list of all of your FinSpace environments. |
| `ListKxChangesets` | `GET /kx/environments/{environmentId}/databases/{databaseName}/changesets` | `paginated` | `environmentId`, `databaseName` | - | `ListKxChangesetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all the changesets for a database. |
| `ListKxClusterNodes` | `GET /kx/environments/{environmentId}/clusters/{clusterName}/nodes` | `paginated` | `environmentId`, `clusterName` | - | `ListKxClusterNodesResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the nodes in a kdb cluster. |
| `ListKxClusters` | `GET /kx/environments/{environmentId}/clusters` | - | `environmentId` | - | `ListKxClustersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of clusters. |
| `ListKxDatabases` | `GET /kx/environments/{environmentId}/databases` | `paginated` | `environmentId` | - | `ListKxDatabasesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all the databases in the kdb environment. |
| `ListKxDataviews` | `GET /kx/environments/{environmentId}/databases/{databaseName}/dataviews` | `paginated` | `environmentId`, `databaseName` | - | `ListKxDataviewsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all the dataviews in the database. |
| `ListKxEnvironments` | `GET /kx/environments` | `paginated` | - | - | `ListKxEnvironmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of kdb environments created in an account. |
| `ListKxScalingGroups` | `GET /kx/environments/{environmentId}/scalingGroups` | `paginated` | `environmentId` | - | `ListKxScalingGroupsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of scaling groups in a kdb environment. |
| `ListKxUsers` | `GET /kx/environments/{environmentId}/users` | - | `environmentId` | - | `ListKxUsersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the users in a kdb environment. |
| `ListKxVolumes` | `GET /kx/environments/{environmentId}/kxvolumes` | - | `environmentId` | - | `ListKxVolumesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the volumes in a kdb environment. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | A list of all tags for a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Adds metadata tags to a FinSpace resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Removes metadata tags from a FinSpace resource. |
| `UpdateEnvironment` | `PUT /environment/{environmentId}` | - | `environmentId` | - | `UpdateEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update your FinSpace environment. |
| `UpdateKxClusterCodeConfiguration` | `PUT /kx/environments/{environmentId}/clusters/{clusterName}/configuration/code` | `idempotency-token` | `environmentId`, `clusterName`, `code` | `clientToken` | `UpdateKxClusterCodeConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows you to update code configuration on a running cluster. By using this API you can update the code, the initialization script path, and the command line arguments for a specific cluster. The configuration that y ... |
| `UpdateKxClusterDatabases` | `PUT /kx/environments/{environmentId}/clusters/{clusterName}/configuration/databases` | `idempotency-token` | `environmentId`, `clusterName`, `databases` | `clientToken` | `UpdateKxClusterDatabasesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the databases mounted on a kdb cluster, which includes the changesetId and all the dbPaths to be cached. This API does not allow you to change a database name or add a database if you created a cluster withou ... |
| `UpdateKxDatabase` | `PUT /kx/environments/{environmentId}/databases/{databaseName}` | `idempotency-token` | `environmentId`, `databaseName`, `clientToken` | `clientToken` | `UpdateKxDatabaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates information for the given kdb database. |
| `UpdateKxDataview` | `PUT /kx/environments/{environmentId}/databases/{databaseName}/dataviews/{dataviewName}` | `idempotency-token` | `environmentId`, `databaseName`, `dataviewName`, `clientToken` | `clientToken` | `UpdateKxDataviewResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified dataview. The dataviews get automatically updated when any new changesets are ingested. Each update of the dataview creates a new version, including changeset details and cache configurations |
| `UpdateKxEnvironment` | `PUT /kx/environments/{environmentId}` | `idempotency-token` | `environmentId` | `clientToken` | `UpdateKxEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates information for the given kdb environment. |
| `UpdateKxEnvironmentNetwork` | `PUT /kx/environments/{environmentId}/network` | `idempotency-token` | `environmentId` | `clientToken` | `UpdateKxEnvironmentNetworkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates environment network to connect to your internal network by using a transit gateway. This API supports request to create a transit gateway attachment from FinSpace VPC to your transit gateway ID and create a c ... |
| `UpdateKxUser` | `PUT /kx/environments/{environmentId}/users/{userName}` | `idempotency-token` | `environmentId`, `userName`, `iamRole` | `clientToken` | `UpdateKxUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the user details. You can only update the IAM role associated with a user. |
| `UpdateKxVolume` | `PATCH /kx/environments/{environmentId}/kxvolumes/{volumeName}` | `idempotency-token` | `environmentId`, `volumeName` | `clientToken` | `UpdateKxVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the throughput or capacity of a volume. During the update process, the filesystem might be unavailable for a few minutes. You can retry any operations after the update is complete. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteKxCluster` | - | `clientToken -> clientToken` | - | - |
| `DeleteKxDatabase` | - | `clientToken -> clientToken` | - | - |
| `DeleteKxDataview` | - | `clientToken -> clientToken` | - | - |
| `DeleteKxEnvironment` | - | `clientToken -> clientToken` | - | - |
| `DeleteKxScalingGroup` | - | `clientToken -> clientToken` | - | - |
| `DeleteKxUser` | - | `clientToken -> clientToken` | - | - |
| `DeleteKxVolume` | - | `clientToken -> clientToken` | - | - |
| `GetKxConnectionString` | - | `userArn -> userArn`, `clusterName -> clusterName` | - | - |
| `ListEnvironments` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxChangesets` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxClusterNodes` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxClusters` | - | `clusterType -> clusterType`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListKxDatabases` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxDataviews` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxEnvironments` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxScalingGroups` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListKxUsers` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListKxVolumes` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `volumeType -> volumeType` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, reason | There was a conflict with this action, and it could not be completed. |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure. |
| `InvalidRequestException` | `structure` | message | The request is invalid. Something is wrong with the input to the request. |
| `LimitExceededException` | `structure` | message | A service limit or quota is exceeded. |
| `ResourceAlreadyExistsException` | `structure` | message | The specified resource group already exists. |
| `ResourceNotFoundException` | `structure` | message | One or more resources can't be found. |
| `ServiceQuotaExceededException` | `structure` | message | You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quo ... |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an AWS service. |
| `CreateEnvironmentRequest` | `structure` | name, description, kmsKeyId, tags, federationMode, federationParameters, superuserParameters, dataBundles | - |
| `CreateEnvironmentResponse` | `structure` | environmentId, environmentArn, environmentUrl | - |
| `CreateKxChangesetRequest` | `structure` | environmentId, databaseName, changeRequests, clientToken | - |
| `CreateKxChangesetResponse` | `structure` | changesetId, databaseName, environmentId, changeRequests, createdTimestamp, lastModifiedTimestamp, status, errorInfo | - |
| `CreateKxClusterRequest` | `structure` | clientToken, environmentId, clusterName, clusterType, tickerplantLogConfiguration, databases, cacheStorageConfigurations, autoScalingConfiguration, clusterDescription, capacityConfiguration, releaseLabel, vpcConfiguration, ... (+9) | - |
| `CreateKxClusterResponse` | `structure` | environmentId, status, statusReason, clusterName, clusterType, tickerplantLogConfiguration, volumes, databases, cacheStorageConfigurations, autoScalingConfiguration, clusterDescription, capacityConfiguration, ... (+12) | - |
| `CreateKxDatabaseRequest` | `structure` | environmentId, databaseName, description, tags, clientToken | - |
| `CreateKxDatabaseResponse` | `structure` | databaseName, databaseArn, environmentId, description, createdTimestamp, lastModifiedTimestamp | - |
| `CreateKxDataviewRequest` | `structure` | environmentId, databaseName, dataviewName, azMode, availabilityZoneId, changesetId, segmentConfigurations, autoUpdate, readWrite, description, tags, clientToken | - |
| `CreateKxDataviewResponse` | `structure` | dataviewName, databaseName, environmentId, azMode, availabilityZoneId, changesetId, segmentConfigurations, description, autoUpdate, readWrite, createdTimestamp, lastModifiedTimestamp, ... (+1) | - |
| `CreateKxEnvironmentRequest` | `structure` | name, description, kmsKeyId, tags, clientToken | - |
| `CreateKxEnvironmentResponse` | `structure` | name, status, environmentId, description, environmentArn, kmsKeyId, creationTimestamp | - |
| `CreateKxScalingGroupRequest` | `structure` | clientToken, environmentId, scalingGroupName, hostType, availabilityZoneId, tags | - |
| `CreateKxScalingGroupResponse` | `structure` | environmentId, scalingGroupName, hostType, availabilityZoneId, status, lastModifiedTimestamp, createdTimestamp | - |
| `CreateKxUserRequest` | `structure` | environmentId, userName, iamRole, tags, clientToken | - |
| `CreateKxUserResponse` | `structure` | userName, userArn, environmentId, iamRole | - |
| `CreateKxVolumeRequest` | `structure` | clientToken, environmentId, volumeType, volumeName, description, nas1Configuration, azMode, availabilityZoneIds, tags | - |
| `CreateKxVolumeResponse` | `structure` | environmentId, volumeName, volumeType, volumeArn, nas1Configuration, status, statusReason, azMode, description, availabilityZoneIds, createdTimestamp | - |
| `DeleteEnvironmentRequest` | `structure` | environmentId | - |
| `DeleteEnvironmentResponse` | `structure` | **empty (no members)** | - |
| `DeleteKxClusterRequest` | `structure` | environmentId, clusterName, clientToken | - |
| `DeleteKxClusterResponse` | `structure` | **empty (no members)** | - |
| `DeleteKxClusterNodeRequest` | `structure` | environmentId, clusterName, nodeId | - |
| `DeleteKxClusterNodeResponse` | `structure` | **empty (no members)** | - |
| `DeleteKxDatabaseRequest` | `structure` | environmentId, databaseName, clientToken | - |
| `DeleteKxDatabaseResponse` | `structure` | **empty (no members)** | - |
| `DeleteKxDataviewRequest` | `structure` | environmentId, databaseName, dataviewName, clientToken | - |
| `DeleteKxDataviewResponse` | `structure` | **empty (no members)** | - |
| `DeleteKxEnvironmentRequest` | `structure` | environmentId, clientToken | - |
| `DeleteKxEnvironmentResponse` | `structure` | **empty (no members)** | - |
| `AutoScalingMetric` | `enum` | CPU_UTILIZATION_PERCENTAGE | - |
| `ChangeType` | `enum` | PUT, DELETE | - |
| `ChangesetStatus` | `enum` | PENDING, PROCESSING, FAILED, COMPLETED | - |
| `EnvironmentStatus` | `enum` | CREATE_REQUESTED, CREATING, CREATED, DELETE_REQUESTED, DELETING, DELETED, FAILED_CREATION, RETRY_DELETION, FAILED_DELETION, UPDATE_NETWORK_REQUESTED, UPDATING_NETWORK, FAILED_UPDATING_NETWORK, ... (+1) | - |
| `ErrorDetails` | `enum` | VALIDATION, SERVICE_QUOTA_EXCEEDED, ACCESS_DENIED, RESOURCE_NOT_FOUND, THROTTLING, INTERNAL_SERVICE_EXCEPTION, CANCELLED, USER_RECOVERABLE | - |
| `FederationMode` | `enum` | FEDERATED, LOCAL | - |
| `IPAddressType` | `enum` | IP_V4 | - |
| `KxAzMode` | `enum` | SINGLE, MULTI | - |
| `KxClusterCodeDeploymentStrategy` | `enum` | NO_RESTART, ROLLING, FORCE | - |
| `KxClusterStatus` | `enum` | PENDING, CREATING, CREATE_FAILED, RUNNING, UPDATING, DELETING, DELETED, DELETE_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

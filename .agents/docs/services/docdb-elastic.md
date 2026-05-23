# Amazon DocumentDB Elastic Clusters

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon DocumentDB elastic clusters Amazon DocumentDB elastic-clusters support workloads with millions of reads/writes per second and petabytes of storage capacity. Amazon DocumentDB elastic clusters also simplify how developers interact with Amazon DocumentDB elastic-clusters by eliminating the need to choose, manage or upgrade instances. Amazon DocumentDB elastic-clusters were created to: provide a solution for customers looking for a database that provides virtually limitless scale with rich query capabilities and MongoDB API compatibility. give customers higher connection limits, and to reduce downtime from patching. continue investing in a cloud-native, elastic, and class leading architecture for JSON workloads.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon DocumentDB Elastic Clusters resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon DocumentDB Elastic Clusters workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Apply` operation families, including `ListClusterSnapshots`, `ListClusters`, `ListPendingMaintenanceActions`, `ListTagsForResource`, `GetCluster`, `GetClusterSnapshot`.

## Service Identity and Protocol

- AWS model slug: `docdb-elastic`
- AWS SDK for Rust slug: `docdbelastic`
- Model version: `2022-11-28`
- Model file: `vendor/api-models-aws/models/docdb-elastic/service/2022-11-28/docdb-elastic-2022-11-28.json`
- SDK ID: `DocDB Elastic`
- Endpoint prefix: `-`
- ARN namespace: `docdb-elastic`
- CloudFormation name: `-`
- CloudTrail event source: `CASCADES_EVENT_SOURCE`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Create` (2), `Delete` (2), `Apply` (1), `Copy` (1), `Restore` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCluster`, `CreateClusterSnapshot`, `DeleteCluster`, `DeleteClusterSnapshot`, `RestoreClusterFromSnapshot`, `StartCluster`, `StopCluster`, `TagResource`, `UntagResource`, `UpdateCluster`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCluster`, `GetClusterSnapshot`, `GetPendingMaintenanceAction`, `ListClusterSnapshots`, `ListClusters`, `ListPendingMaintenanceActions`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartCluster`, `StopCluster`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `ECS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/documentdb/latest/developerguide/limits.html
- https://docs.aws.amazon.com/documentdb/latest/developerguide/db-cluster-manage-performance.html
- https://docs.aws.amazon.com/documentdb/latest/developerguide/get-started-guide.html

Research outcomes:
- DocumentDB Elastic clusters are a sharded, elastic form of Amazon DocumentDB with separate elastic-cluster limits and shard limits.
- Elastic cluster limits cover shard count, CPU, memory, connections, and cursor limits per shard.
- Scaling behaviour differs from instance-based DocumentDB clusters and is governed by shard and vCPU configuration.
- Elastic cluster snapshots and restore workflows are separate from instance-based cluster operations.
- Naming, regional, and quota constraints are documented separately for elastic clusters.

Parity implications:
- Model elastic clusters, shards, vCPU/shard settings, subnet/security configuration, snapshots, restore state, and elastic-specific quotas separately from classic DocumentDB clusters.
- Scaling should change shard/capacity state rather than creating read replicas.
- API validation should use elastic cluster limits, not only classic DocumentDB limits.

## Operation Groups

### List

- Operations: `ListClusterSnapshots`, `ListClusters`, `ListPendingMaintenanceActions`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `resourceArn`

### Get

- Operations: `GetCluster`, `GetClusterSnapshot`, `GetPendingMaintenanceAction`
- Traits: `readonly` (3)
- Common required input members in this group: `clusterArn`, `resourceArn`, `snapshotArn`

### Create

- Operations: `CreateCluster`, `CreateClusterSnapshot`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `adminUserName`, `adminUserPassword`, `authType`, `clusterArn`, `clusterName`, `shardCapacity`, `shardCount`, `snapshotName`

### Delete

- Operations: `DeleteCluster`, `DeleteClusterSnapshot`
- Traits: `idempotent` (2)
- Common required input members in this group: `clusterArn`, `snapshotArn`

### Apply

- Operations: `ApplyPendingMaintenanceAction`
- Traits: `idempotent` (1)
- Common required input members in this group: `applyAction`, `optInType`, `resourceArn`

### Copy

- Operations: `CopyClusterSnapshot`
- Traits: `idempotent` (1)
- Common required input members in this group: `snapshotArn`, `targetSnapshotName`

### Restore

- Operations: `RestoreClusterFromSnapshot`
- Traits: `idempotent` (1)
- Common required input members in this group: `clusterName`, `snapshotArn`

### Start

- Operations: `StartCluster`
- Traits: `idempotent` (1)
- Common required input members in this group: `clusterArn`

### Stop

- Operations: `StopCluster`
- Traits: `idempotent` (1)
- Common required input members in this group: `clusterArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateCluster`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `clusterArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ApplyPendingMaintenanceAction` | `POST /pending-action` | `idempotent` | `applyAction`, `optInType`, `resourceArn` | - | `ApplyPendingMaintenanceActionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The type of pending maintenance action to be applied to the resource. |
| `CopyClusterSnapshot` | `POST /cluster-snapshot/{snapshotArn}/copy` | `idempotent` | `snapshotArn`, `targetSnapshotName` | - | `CopyClusterSnapshotOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Copies a snapshot of an elastic cluster. |
| `CreateCluster` | `POST /cluster` | `idempotent`, `idempotency-token` | `adminUserName`, `adminUserPassword`, `authType`, `clusterName`, `shardCapacity`, `shardCount` | `clientToken` | `CreateClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Amazon DocumentDB elastic cluster and returns its cluster structure. |
| `CreateClusterSnapshot` | `POST /cluster-snapshot` | `idempotent` | `clusterArn`, `snapshotName` | - | `CreateClusterSnapshotOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a snapshot of an elastic cluster. |
| `DeleteCluster` | `DELETE /cluster/{clusterArn}` | `idempotent` | `clusterArn` | - | `DeleteClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an elastic cluster. |
| `DeleteClusterSnapshot` | `DELETE /cluster-snapshot/{snapshotArn}` | `idempotent` | `snapshotArn` | - | `DeleteClusterSnapshotOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an elastic cluster snapshot. |
| `GetCluster` | `GET /cluster/{clusterArn}` | `readonly` | `clusterArn` | - | `GetClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific elastic cluster. |
| `GetClusterSnapshot` | `GET /cluster-snapshot/{snapshotArn}` | `readonly` | `snapshotArn` | - | `GetClusterSnapshotOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific elastic cluster snapshot |
| `GetPendingMaintenanceAction` | `GET /pending-action/{resourceArn}` | `readonly` | `resourceArn` | - | `GetPendingMaintenanceActionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves all maintenance actions that are pending. |
| `ListClusterSnapshots` | `GET /cluster-snapshots` | `readonly`, `paginated` | - | - | `ListClusterSnapshotsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about snapshots for a specified elastic cluster. |
| `ListClusters` | `GET /clusters` | `readonly`, `paginated` | - | - | `ListClustersOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about provisioned Amazon DocumentDB elastic clusters. |
| `ListPendingMaintenanceActions` | `GET /pending-actions` | `readonly`, `paginated` | - | - | `ListPendingMaintenanceActionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all maintenance actions that are pending. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags on a elastic cluster resource |
| `RestoreClusterFromSnapshot` | `POST /cluster-snapshot/{snapshotArn}/restore` | `idempotent` | `clusterName`, `snapshotArn` | - | `RestoreClusterFromSnapshotOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Restores an elastic cluster from a snapshot. |
| `StartCluster` | `POST /cluster/{clusterArn}/start` | `idempotent` | `clusterArn` | - | `StartClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Restarts the stopped elastic cluster that is specified by `clusterARN`. |
| `StopCluster` | `POST /cluster/{clusterArn}/stop` | `idempotent` | `clusterArn` | - | `StopClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops the running elastic cluster that is specified by `clusterArn`. The elastic cluster must be in the available state. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds metadata tags to an elastic cluster resource |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes metadata tags from an elastic cluster resource |
| `UpdateCluster` | `PUT /cluster/{clusterArn}` | `idempotent`, `idempotency-token` | `clusterArn` | `clientToken` | `UpdateClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an elastic cluster. This includes updating admin-username/password, upgrading the API version, and setting up a backup window and maintenance window |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListClusters` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListClusterSnapshots` | - | `clusterArn -> clusterArn`, `nextToken -> nextToken`, `maxResults -> maxResults`, `snapshotType -> snapshotType` | - | - |
| `ListPendingMaintenanceActions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | There was an internal server error. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | ThrottlingException will be thrown when request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | A structure defining a validation exception. |
| `AccessDeniedException` | `structure` | `message` | An exception that occurs when there are not sufficient permissions to perform an action. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The specified resource could not be located. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | There was an access conflict. |
| `ServiceQuotaExceededException` | `structure` | `message` | The service quota for the action was exceeded. |
| `ApplyPendingMaintenanceActionInput` | `structure` | `applyAction`, `applyOn`, `optInType`, `resourceArn` | - |
| `ApplyPendingMaintenanceActionOutput` | `structure` | `resourcePendingMaintenanceAction` | - |
| `CopyClusterSnapshotInput` | `structure` | `copyTags`, `kmsKeyId`, `snapshotArn`, `tags`, `targetSnapshotName` | - |
| `CopyClusterSnapshotOutput` | `structure` | `snapshot` | - |
| `CreateClusterInput` | `structure` | `adminUserName`, `adminUserPassword`, `authType`, `backupRetentionPeriod`, `clientToken`, `clusterName`, `kmsKeyId`, `preferredBackupWindow`, `preferredMaintenanceWindow`, `shardCapacity`, `shardCount`, `shardInstanceCount`, ... (+3) | - |
| `CreateClusterOutput` | `structure` | `cluster` | - |
| `CreateClusterSnapshotInput` | `structure` | `clusterArn`, `snapshotName`, `tags` | - |
| `CreateClusterSnapshotOutput` | `structure` | `snapshot` | - |
| `DeleteClusterInput` | `structure` | `clusterArn` | - |
| `DeleteClusterOutput` | `structure` | `cluster` | - |
| `DeleteClusterSnapshotInput` | `structure` | `snapshotArn` | - |
| `DeleteClusterSnapshotOutput` | `structure` | `snapshot` | - |
| `GetClusterInput` | `structure` | `clusterArn` | - |
| `GetClusterOutput` | `structure` | `cluster` | - |
| `GetClusterSnapshotInput` | `structure` | `snapshotArn` | - |
| `GetClusterSnapshotOutput` | `structure` | `snapshot` | - |
| `GetPendingMaintenanceActionInput` | `structure` | `resourceArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

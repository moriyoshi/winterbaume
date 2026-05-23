# Amazon ElastiCache

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon ElastiCache Amazon ElastiCache is a web service that makes it easier to set up, operate, and scale a distributed cache in the cloud. With ElastiCache, customers get all of the benefits of a high-performance, in-memory cache with less of the administrative burden involved in launching and managing a distributed cache. The service makes setup, scaling, and cluster failure handling much simpler than in a self-managed cache deployment. In addition, through integration with Amazon CloudWatch, customers get enhanced visibility into the key performance statistics associated with their cache and can receive alarms if a part of their cache runs hot.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon ElastiCache where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon ElastiCache by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon ElastiCache by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon ElastiCache workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Delete`, `Modify`, `Batch` operation families, including `DescribeCacheClusters`, `DescribeCacheEngineVersions`, `DescribeCacheParameterGroups`, `DescribeCacheParameters`, `CreateCacheCluster`, `CreateCacheParameterGroup`.

## Service Identity and Protocol

- AWS model slug: `elasticache`
- AWS SDK for Rust slug: `elasticache`
- Model version: `2015-02-02`
- Model file: `vendor/api-models-aws/models/elasticache/service/2015-02-02/elasticache-2015-02-02.json`
- SDK ID: `ElastiCache`
- Endpoint prefix: `elasticache`
- ARN namespace: `elasticache`
- CloudFormation name: `ElastiCache`
- CloudTrail event source: `elasticache.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (19), `Create` (11), `Delete` (11), `Modify` (9), `Batch` (2), `Copy` (2), `Decrease` (2), `Increase` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToResource`, `BatchApplyUpdateAction`, `BatchStopUpdateAction`, `CreateCacheCluster`, `CreateCacheParameterGroup`, `CreateCacheSecurityGroup`, `CreateCacheSubnetGroup`, `CreateGlobalReplicationGroup`, `CreateReplicationGroup`, `CreateServerlessCache`, `CreateServerlessCacheSnapshot`, `CreateSnapshot`, `CreateUser`, `CreateUserGroup`, `DeleteCacheCluster`, `DeleteCacheParameterGroup`, `DeleteCacheSecurityGroup`, `DeleteCacheSubnetGroup`, `DeleteGlobalReplicationGroup`, `DeleteReplicationGroup`, ... (+18).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCacheClusters`, `DescribeCacheEngineVersions`, `DescribeCacheParameterGroups`, `DescribeCacheParameters`, `DescribeCacheSecurityGroups`, `DescribeCacheSubnetGroups`, `DescribeEngineDefaultParameters`, `DescribeEvents`, `DescribeGlobalReplicationGroups`, `DescribeReplicationGroups`, `DescribeReservedCacheNodes`, `DescribeReservedCacheNodesOfferings`, `DescribeServerlessCacheSnapshots`, `DescribeServerlessCaches`, `DescribeServiceUpdates`, `DescribeSnapshots`, `DescribeUpdateActions`, `DescribeUserGroups`, `DescribeUsers`, `ListAllowedNodeTypeModifications`, ... (+1).
- Pagination is modelled for 19 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportServerlessCacheSnapshot`, `StartMigration`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 74 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `EC2/VPC`, `ECS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/Replication.html
- https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/AutoFailover.html
- https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/backups.html

Research outcomes:
- Valkey and Redis OSS replication groups organise one primary and zero to five read replicas inside each shard. Cluster mode disabled has one shard; cluster mode enabled partitions data across multiple shards.
- Replica data is updated asynchronously, so a primary failure can lose recent writes that have not reached a replica.
- Multi-AZ automatic failover promotes a read replica to primary and propagates the primary endpoint DNS name so writers can resume without changing the primary endpoint.
- Multi-AZ requires at least one available read replica for cluster mode disabled replication groups. Cluster mode enabled groups have Multi-AZ enabled by default when replica placement requirements are met.
- On primary-only failure, ElastiCache promotes the replica with least replication lag, creates a replacement replica in the failed primary's Availability Zone, and resynchronises replicas.
- Replication groups expose primary endpoints, reader or node endpoints, member clusters, node groups, role state, snapshot settings, and pending modified values.
- Backups are supported for Valkey, Redis OSS, and serverless Memcached, with additional node-type and cluster-mode constraints.
- Cluster mode enabled Valkey or Redis OSS backups are taken at the cluster or replication-group level, not the shard or node-group level.
- During backup, serverless caches cannot run other API or CLI operations, while node-based clusters can. For node-based Redis or Valkey, AWS recommends backing up from a read replica to reduce primary impact.
- A final backup on replication-group deletion is always taken from the primary node to capture the latest data.

Parity implications:
- Model cache clusters, replication groups, node groups, cache nodes, endpoints, subnet groups, parameter groups, snapshots, pending modifications, and failover state separately.
- Failover should update node roles, replacement replica provisioning, endpoint routing, and asynchronous-loss caveats.
- Snapshot creation should enforce engine, cluster-mode, serverless, and per-node or per-cache limits, and deletion final-snapshot behaviour should choose the primary.

## Current Network Resource Stub Semantics

ElastiCache currently keeps subnet groups and security group references inside ElastiCache state.

- Cache subnet groups store supplied subnet IDs and a local VPC ID field; describe operations return that stored metadata.
- Cache clusters and replication groups store subnet group names and security group references without resolving them through EC2.
- There is no cross-service check that subnet group subnets share a VPC, that security groups exist, or that a subnet group is free of dependent clusters before every mutation path.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeCacheClusters`, `DescribeCacheEngineVersions`, `DescribeCacheParameterGroups`, `DescribeCacheParameters`, `DescribeCacheSecurityGroups`, `DescribeCacheSubnetGroups`, `DescribeEngineDefaultParameters`, `DescribeEvents`, `DescribeGlobalReplicationGroups`, `DescribeReplicationGroups`, `DescribeReservedCacheNodes`, `DescribeReservedCacheNodesOfferings`, `DescribeServerlessCaches`, `DescribeServerlessCacheSnapshots`, `DescribeServiceUpdates`, `DescribeSnapshots`, `DescribeUpdateActions`, `DescribeUserGroups`, `DescribeUsers`
- Traits: `paginated` (19)
- Common required input members in this group: -

### Create

- Operations: `CreateCacheCluster`, `CreateCacheParameterGroup`, `CreateCacheSecurityGroup`, `CreateCacheSubnetGroup`, `CreateGlobalReplicationGroup`, `CreateReplicationGroup`, `CreateServerlessCache`, `CreateServerlessCacheSnapshot`, `CreateSnapshot`, `CreateUser`, `CreateUserGroup`
- Common required input members in this group: `Description`, `ServerlessCacheName`, `Engine`

### Delete

- Operations: `DeleteCacheCluster`, `DeleteCacheParameterGroup`, `DeleteCacheSecurityGroup`, `DeleteCacheSubnetGroup`, `DeleteGlobalReplicationGroup`, `DeleteReplicationGroup`, `DeleteServerlessCache`, `DeleteServerlessCacheSnapshot`, `DeleteSnapshot`, `DeleteUser`, `DeleteUserGroup`
- Common required input members in this group: -

### Modify

- Operations: `ModifyCacheCluster`, `ModifyCacheParameterGroup`, `ModifyCacheSubnetGroup`, `ModifyGlobalReplicationGroup`, `ModifyReplicationGroup`, `ModifyReplicationGroupShardConfiguration`, `ModifyServerlessCache`, `ModifyUser`, `ModifyUserGroup`
- Common required input members in this group: `ApplyImmediately`, `ReplicationGroupId`

### Batch

- Operations: `BatchApplyUpdateAction`, `BatchStopUpdateAction`
- Common required input members in this group: `ServiceUpdateName`

### Copy

- Operations: `CopyServerlessCacheSnapshot`, `CopySnapshot`
- Common required input members in this group: -

### Decrease

- Operations: `DecreaseNodeGroupsInGlobalReplicationGroup`, `DecreaseReplicaCount`
- Common required input members in this group: `ApplyImmediately`

### Increase

- Operations: `IncreaseNodeGroupsInGlobalReplicationGroup`, `IncreaseReplicaCount`
- Common required input members in this group: `ApplyImmediately`

### List

- Operations: `ListAllowedNodeTypeModifications`, `ListTagsForResource`
- Common required input members in this group: -

### Test

- Operations: `TestFailover`, `TestMigration`
- Common required input members in this group: `ReplicationGroupId`

### Add

- Operations: `AddTagsToResource`
- Common required input members in this group: -

### Authorize

- Operations: `AuthorizeCacheSecurityGroupIngress`
- Common required input members in this group: -

### Complete

- Operations: `CompleteMigration`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateGlobalReplicationGroup`
- Common required input members in this group: -

### Export

- Operations: `ExportServerlessCacheSnapshot`
- Common required input members in this group: -

### Failover

- Operations: `FailoverGlobalReplicationGroup`
- Common required input members in this group: -

### Purchase

- Operations: `PurchaseReservedCacheNodesOffering`
- Common required input members in this group: -

### Rebalance

- Operations: `RebalanceSlotsInGlobalReplicationGroup`
- Common required input members in this group: -

### Reboot

- Operations: `RebootCacheCluster`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: -

### Reset

- Operations: `ResetCacheParameterGroup`
- Common required input members in this group: -

### Revoke

- Operations: `RevokeCacheSecurityGroupIngress`
- Common required input members in this group: -

### Start

- Operations: `StartMigration`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToResource` | `-` | - | `ResourceName`, `Tags` | - | `TagListMessage` | `CacheClusterNotFoundFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `CacheSubnetGroupNotFoundFault`, `InvalidARNFault`, `InvalidReplicationGroupStateFault`, `InvalidServerlessCacheSnapshotStateFault`, `InvalidServerlessCacheStateFault`, `ReplicationGroupNotFoundFault`, `ReservedCacheNodeNotFoundFault`, `ServerlessCacheNotFoundFault`, `ServerlessCacheSnapshotNotFoundFault`, `SnapshotNotFoundFault`, `TagQuotaPerResourceExceeded`, `UserGroupNotFoundFault`, `UserNotFoundFault` | A tag is a key-value pair where the key and value are case-sensitive. You can use tags to categorize and track all your ElastiCache resources, with the exception of global replication group. When you add or remove ta ... |
| `AuthorizeCacheSecurityGroupIngress` | `-` | - | `CacheSecurityGroupName`, `EC2SecurityGroupName`, `EC2SecurityGroupOwnerId` | - | `AuthorizeCacheSecurityGroupIngressResult` | `AuthorizationAlreadyExistsFault`, `CacheSecurityGroupNotFoundFault`, `InvalidCacheSecurityGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Allows network ingress to a cache security group. Applications using ElastiCache must be running on Amazon EC2, and Amazon EC2 security groups are used as the authorization mechanism. You cannot authorize ingress fro ... |
| `BatchApplyUpdateAction` | `-` | - | `ServiceUpdateName` | - | `UpdateActionResultsMessage` | `InvalidParameterValueException`, `ServiceUpdateNotFoundFault` | Apply the service update. For more information on service updates and applying them, see Applying Service Updates . |
| `BatchStopUpdateAction` | `-` | - | `ServiceUpdateName` | - | `UpdateActionResultsMessage` | `InvalidParameterValueException`, `ServiceUpdateNotFoundFault` | Stop the service update. For more information on service updates and stopping them, see Stopping Service Updates . |
| `CompleteMigration` | `-` | - | `ReplicationGroupId` | - | `CompleteMigrationResponse` | `InvalidReplicationGroupStateFault`, `ReplicationGroupNotFoundFault`, `ReplicationGroupNotUnderMigrationFault` | Complete the migration of data. |
| `CopyServerlessCacheSnapshot` | `-` | - | `SourceServerlessCacheSnapshotName`, `TargetServerlessCacheSnapshotName` | - | `CopyServerlessCacheSnapshotResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidServerlessCacheSnapshotStateFault`, `ServerlessCacheSnapshotAlreadyExistsFault`, `ServerlessCacheSnapshotNotFoundFault`, `ServerlessCacheSnapshotQuotaExceededFault`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded` | Creates a copy of an existing serverless cache’s snapshot. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `CopySnapshot` | `-` | - | `SourceSnapshotName`, `TargetSnapshotName` | - | `CopySnapshotResult` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidSnapshotStateFault`, `SnapshotAlreadyExistsFault`, `SnapshotNotFoundFault`, `SnapshotQuotaExceededFault`, `TagQuotaPerResourceExceeded` | Makes a copy of an existing snapshot. This operation is valid for Valkey or Redis OSS only. Users or groups that have permissions to use the CopySnapshot operation can create their own Amazon S3 buckets and copy snap ... |
| `CreateCacheCluster` | `-` | - | `CacheClusterId` | - | `CreateCacheClusterResult` | `CacheClusterAlreadyExistsFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `CacheSubnetGroupNotFoundFault`, `ClusterQuotaForCustomerExceededFault`, `InsufficientCacheClusterCapacityFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `InvalidVPCNetworkStateFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `ReplicationGroupNotFoundFault`, `TagQuotaPerResourceExceeded` | Creates a cluster. All nodes in the cluster run the same protocol-compliant cache engine software, either Memcached, Valkey or Redis OSS. This operation is not supported for Valkey or Redis OSS (cluster mode enabled) ... |
| `CreateCacheParameterGroup` | `-` | - | `CacheParameterGroupName`, `CacheParameterGroupFamily`, `Description` | - | `CreateCacheParameterGroupResult` | `CacheParameterGroupAlreadyExistsFault`, `CacheParameterGroupQuotaExceededFault`, `InvalidCacheParameterGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `TagQuotaPerResourceExceeded` | Creates a new Amazon ElastiCache cache parameter group. An ElastiCache cache parameter group is a collection of parameters and their values that are applied to all of the nodes in any cluster or replication group usi ... |
| `CreateCacheSecurityGroup` | `-` | - | `CacheSecurityGroupName`, `Description` | - | `CreateCacheSecurityGroupResult` | `CacheSecurityGroupAlreadyExistsFault`, `CacheSecurityGroupQuotaExceededFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `TagQuotaPerResourceExceeded` | Creates a new cache security group. Use a cache security group to control access to one or more clusters. Cache security groups are only used when you are creating a cluster outside of an Amazon Virtual Private Cloud ... |
| `CreateCacheSubnetGroup` | `-` | - | `CacheSubnetGroupName`, `CacheSubnetGroupDescription`, `SubnetIds` | - | `CreateCacheSubnetGroupResult` | `CacheSubnetGroupAlreadyExistsFault`, `CacheSubnetGroupQuotaExceededFault`, `CacheSubnetQuotaExceededFault`, `InvalidSubnet`, `SubnetNotAllowedFault`, `TagQuotaPerResourceExceeded` | Creates a new cache subnet group. Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC). |
| `CreateGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupIdSuffix`, `PrimaryReplicationGroupId` | - | `CreateGlobalReplicationGroupResult` | `GlobalReplicationGroupAlreadyExistsFault`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `ReplicationGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Global Datastore offers fully managed, fast, reliable and secure cross-region replication. Using Global Datastore with Valkey or Redis OSS, you can create cross-region read replica clusters for ElastiCache to enable ... |
| `CreateReplicationGroup` | `-` | - | `ReplicationGroupId`, `ReplicationGroupDescription` | - | `CreateReplicationGroupResult` | `CacheClusterNotFoundFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `CacheSubnetGroupNotFoundFault`, `ClusterQuotaForCustomerExceededFault`, `GlobalReplicationGroupNotFoundFault`, `InsufficientCacheClusterCapacityFault`, `InvalidCacheClusterStateFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidUserGroupStateFault`, `InvalidVPCNetworkStateFault`, `NodeGroupsPerReplicationGroupQuotaExceededFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `ReplicationGroupAlreadyExistsFault`, `TagQuotaPerResourceExceeded`, `UserGroupNotFoundFault` | Creates a Valkey or Redis OSS (cluster mode disabled) or a Valkey or Redis OSS (cluster mode enabled) replication group. This API can be used to create a standalone regional replication group or a secondary replicati ... |
| `CreateServerlessCache` | `-` | - | `ServerlessCacheName`, `Engine` | - | `CreateServerlessCacheResponse` | `InvalidCredentialsException`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidServerlessCacheStateFault`, `InvalidUserGroupStateFault`, `ServerlessCacheAlreadyExistsFault`, `ServerlessCacheNotFoundFault`, `ServerlessCacheQuotaForCustomerExceededFault`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded`, `UserGroupNotFoundFault` | Creates a serverless cache. |
| `CreateServerlessCacheSnapshot` | `-` | - | `ServerlessCacheSnapshotName`, `ServerlessCacheName` | - | `CreateServerlessCacheSnapshotResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidServerlessCacheStateFault`, `ServerlessCacheNotFoundFault`, `ServerlessCacheSnapshotAlreadyExistsFault`, `ServerlessCacheSnapshotQuotaExceededFault`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded` | This API creates a copy of an entire ServerlessCache at a specific moment in time. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `CreateSnapshot` | `-` | - | `SnapshotName` | - | `CreateSnapshotResult` | `CacheClusterNotFoundFault`, `InvalidCacheClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `ReplicationGroupNotFoundFault`, `SnapshotAlreadyExistsFault`, `SnapshotFeatureNotSupportedFault`, `SnapshotQuotaExceededFault`, `TagQuotaPerResourceExceeded` | Creates a copy of an entire cluster or replication group at a specific moment in time. This operation is valid for Valkey or Redis OSS only. |
| `CreateUser` | `-` | - | `UserId`, `UserName`, `Engine`, `AccessString` | - | `User` | `DuplicateUserNameFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded`, `UserAlreadyExistsFault`, `UserQuotaExceededFault` | For Valkey engine version 7.2 onwards and Redis OSS 6.0 to 7.1: Creates a user. For more information, see Using Role Based Access Control (RBAC) . |
| `CreateUserGroup` | `-` | - | `UserGroupId`, `Engine` | - | `UserGroup` | `DefaultUserRequired`, `DuplicateUserNameFault`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded`, `UserGroupAlreadyExistsFault`, `UserGroupQuotaExceededFault`, `UserNotFoundFault` | For Valkey engine version 7.2 onwards and Redis OSS 6.0 to 7.1: Creates a user group. For more information, see Using Role Based Access Control (RBAC) |
| `DecreaseNodeGroupsInGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `NodeGroupCount`, `ApplyImmediately` | - | `DecreaseNodeGroupsInGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Decreases the number of node groups in a Global datastore |
| `DecreaseReplicaCount` | `-` | - | `ReplicationGroupId`, `ApplyImmediately` | - | `DecreaseReplicaCountResult` | `ClusterQuotaForCustomerExceededFault`, `InsufficientCacheClusterCapacityFault`, `InvalidCacheClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `InvalidVPCNetworkStateFault`, `NodeGroupsPerReplicationGroupQuotaExceededFault`, `NodeQuotaForCustomerExceededFault`, `NoOperationFault`, `ReplicationGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Dynamically decreases the number of replicas in a Valkey or Redis OSS (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Valkey or Redis OSS (cluster mod ... |
| `DeleteCacheCluster` | `-` | - | `CacheClusterId` | - | `DeleteCacheClusterResult` | `CacheClusterNotFoundFault`, `InvalidCacheClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `SnapshotAlreadyExistsFault`, `SnapshotFeatureNotSupportedFault`, `SnapshotQuotaExceededFault` | Deletes a previously provisioned cluster. DeleteCacheCluster deletes all associated cache nodes, node endpoints and the cluster itself. When you receive a successful response from this operation, Amazon ElastiCache i ... |
| `DeleteCacheParameterGroup` | `-` | - | `CacheParameterGroupName` | - | `Unit` | `CacheParameterGroupNotFoundFault`, `InvalidCacheParameterGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Deletes the specified cache parameter group. You cannot delete a cache parameter group if it is associated with any cache clusters. You cannot delete the default cache parameter groups in your account. |
| `DeleteCacheSecurityGroup` | `-` | - | `CacheSecurityGroupName` | - | `Unit` | `CacheSecurityGroupNotFoundFault`, `InvalidCacheSecurityGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Deletes a cache security group. You cannot delete a cache security group if it is associated with any clusters. |
| `DeleteCacheSubnetGroup` | `-` | - | `CacheSubnetGroupName` | - | `Unit` | `CacheSubnetGroupInUse`, `CacheSubnetGroupNotFoundFault` | Deletes a cache subnet group. You cannot delete a default cache subnet group or one that is associated with any clusters. |
| `DeleteGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `RetainPrimaryReplicationGroup` | - | `DeleteGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterValueException` | Deleting a Global datastore is a two-step process: First, you must DisassociateGlobalReplicationGroup to remove the secondary clusters in the Global datastore. Once the Global datastore contains only the primary clus ... |
| `DeleteReplicationGroup` | `-` | - | `ReplicationGroupId` | - | `DeleteReplicationGroupResult` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `ReplicationGroupNotFoundFault`, `SnapshotAlreadyExistsFault`, `SnapshotFeatureNotSupportedFault`, `SnapshotQuotaExceededFault` | Deletes an existing replication group. By default, this operation deletes the entire replication group, including the primary/primaries and all of the read replicas. If the replication group has only one primary, you ... |
| `DeleteServerlessCache` | `-` | - | `ServerlessCacheName` | - | `DeleteServerlessCacheResponse` | `InvalidCredentialsException`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidServerlessCacheStateFault`, `ServerlessCacheNotFoundFault`, `ServerlessCacheSnapshotAlreadyExistsFault`, `ServiceLinkedRoleNotFoundFault` | Deletes a specified existing serverless cache. CreateServerlessCacheSnapshot permission is required to create a final snapshot. Without this permission, the API call will fail with an Access Denied exception. |
| `DeleteServerlessCacheSnapshot` | `-` | - | `ServerlessCacheSnapshotName` | - | `DeleteServerlessCacheSnapshotResponse` | `InvalidParameterValueException`, `InvalidServerlessCacheSnapshotStateFault`, `ServerlessCacheSnapshotNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Deletes an existing serverless cache snapshot. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `DeleteSnapshot` | `-` | - | `SnapshotName` | - | `DeleteSnapshotResult` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidSnapshotStateFault`, `SnapshotNotFoundFault` | Deletes an existing snapshot. When you receive a successful response from this operation, ElastiCache immediately begins deleting the snapshot; you cannot cancel or revert this operation. This operation is valid for ... |
| `DeleteUser` | `-` | - | `UserId` | - | `User` | `DefaultUserAssociatedToUserGroupFault`, `InvalidParameterValueException`, `InvalidUserStateFault`, `ServiceLinkedRoleNotFoundFault`, `UserNotFoundFault` | For Valkey engine version 7.2 onwards and Redis OSS 6.0 onwards: Deletes a user. The user will be removed from all user groups and in turn removed from all replication groups. For more information, see Using Role Bas ... |
| `DeleteUserGroup` | `-` | - | `UserGroupId` | - | `UserGroup` | `InvalidParameterValueException`, `InvalidUserGroupStateFault`, `ServiceLinkedRoleNotFoundFault`, `UserGroupNotFoundFault` | For Valkey engine version 7.2 onwards and Redis OSS 6.0 onwards: Deletes a user group. The user group must first be disassociated from the replication group before it can be deleted. For more information, see Using R ... |
| `DescribeCacheClusters` | `-` | `paginated` | - | - | `CacheClusterMessage` | `CacheClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns information about all provisioned clusters if no cluster identifier is specified, or about a specific cache cluster if a cluster identifier is supplied. By default, abbreviated information about the clusters ... |
| `DescribeCacheEngineVersions` | `-` | `paginated` | - | - | `CacheEngineVersionMessage` | - | Returns a list of the available cache engines and their versions. |
| `DescribeCacheParameterGroups` | `-` | `paginated` | - | - | `CacheParameterGroupsMessage` | `CacheParameterGroupNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns a list of cache parameter group descriptions. If a cache parameter group name is specified, the list contains only the descriptions for that group. |
| `DescribeCacheParameters` | `-` | `paginated` | `CacheParameterGroupName` | - | `CacheParameterGroupDetails` | `CacheParameterGroupNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns the detailed parameter list for a particular cache parameter group. |
| `DescribeCacheSecurityGroups` | `-` | `paginated` | - | - | `CacheSecurityGroupMessage` | `CacheSecurityGroupNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns a list of cache security group descriptions. If a cache security group name is specified, the list contains only the description of that group. This applicable only when you have ElastiCache in Classic setup |
| `DescribeCacheSubnetGroups` | `-` | `paginated` | - | - | `CacheSubnetGroupMessage` | `CacheSubnetGroupNotFoundFault` | Returns a list of cache subnet group descriptions. If a subnet group name is specified, the list contains only the description of that group. This is applicable only when you have ElastiCache in VPC setup. All Elasti ... |
| `DescribeEngineDefaultParameters` | `-` | `paginated` | `CacheParameterGroupFamily` | - | `DescribeEngineDefaultParametersResult` | `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns the default engine and system parameter information for the specified cache engine. |
| `DescribeEvents` | `-` | `paginated` | - | - | `EventsMessage` | `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns events related to clusters, cache security groups, and cache parameter groups. You can obtain events specific to a particular cluster, cache security group, or cache parameter group by providing the name as a ... |
| `DescribeGlobalReplicationGroups` | `-` | `paginated` | - | - | `DescribeGlobalReplicationGroupsResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns information about a particular global replication group. If no identifier is specified, returns information about all Global datastores. |
| `DescribeReplicationGroups` | `-` | `paginated` | - | - | `ReplicationGroupMessage` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReplicationGroupNotFoundFault` | Returns information about a particular replication group. If no identifier is specified, DescribeReplicationGroups returns information about all replication groups. This operation is valid for Valkey or Redis OSS only. |
| `DescribeReservedCacheNodes` | `-` | `paginated` | - | - | `ReservedCacheNodeMessage` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReservedCacheNodeNotFoundFault` | Returns information about reserved cache nodes for this account, or about a specified reserved cache node. |
| `DescribeReservedCacheNodesOfferings` | `-` | `paginated` | - | - | `ReservedCacheNodesOfferingMessage` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReservedCacheNodesOfferingNotFoundFault` | Lists available reserved cache node offerings. |
| `DescribeServerlessCaches` | `-` | `paginated` | - | - | `DescribeServerlessCachesResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServerlessCacheNotFoundFault` | Returns information about a specific serverless cache. If no identifier is specified, then the API returns information on all the serverless caches belonging to this Amazon Web Services account. |
| `DescribeServerlessCacheSnapshots` | `-` | `paginated` | - | - | `DescribeServerlessCacheSnapshotsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServerlessCacheNotFoundFault`, `ServerlessCacheSnapshotNotFoundFault` | Returns information about serverless cache snapshots. By default, this API lists all of the customer’s serverless cache snapshots. It can also describe a single serverless cache snapshot, or the snapshots associated ... |
| `DescribeServiceUpdates` | `-` | `paginated` | - | - | `ServiceUpdatesMessage` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceUpdateNotFoundFault` | Returns details of the service updates |
| `DescribeSnapshots` | `-` | `paginated` | - | - | `DescribeSnapshotsListMessage` | `CacheClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `SnapshotNotFoundFault` | Returns information about cluster or replication group snapshots. By default, DescribeSnapshots lists all of your snapshots; it can optionally describe a single snapshot, or just the snapshots associated with a parti ... |
| `DescribeUpdateActions` | `-` | `paginated` | - | - | `UpdateActionsMessage` | `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns details of the update actions |
| `DescribeUserGroups` | `-` | `paginated` | - | - | `DescribeUserGroupsResult` | `InvalidParameterCombinationException`, `ServiceLinkedRoleNotFoundFault`, `UserGroupNotFoundFault` | Returns a list of user groups. |
| `DescribeUsers` | `-` | `paginated` | - | - | `DescribeUsersResult` | `InvalidParameterCombinationException`, `ServiceLinkedRoleNotFoundFault`, `UserNotFoundFault` | Returns a list of users. |
| `DisassociateGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `ReplicationGroupId`, `ReplicationGroupRegion` | - | `DisassociateGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Remove a secondary cluster from the Global datastore using the Global datastore name. The secondary cluster will no longer receive updates from the primary cluster, but will remain as a standalone cluster in that Ama ... |
| `ExportServerlessCacheSnapshot` | `-` | - | `ServerlessCacheSnapshotName`, `S3BucketName` | - | `ExportServerlessCacheSnapshotResponse` | `InvalidParameterValueException`, `InvalidServerlessCacheSnapshotStateFault`, `ServerlessCacheSnapshotNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Provides the functionality to export the serverless cache snapshot data to Amazon S3. Available for Valkey and Redis OSS only. |
| `FailoverGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `PrimaryRegion`, `PrimaryReplicationGroupId` | - | `FailoverGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Used to failover the primary region to a secondary region. The secondary region will become primary, and all other clusters will become secondary. |
| `IncreaseNodeGroupsInGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `NodeGroupCount`, `ApplyImmediately` | - | `IncreaseNodeGroupsInGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterValueException` | Increase the number of node groups in the Global datastore |
| `IncreaseReplicaCount` | `-` | - | `ReplicationGroupId`, `ApplyImmediately` | - | `IncreaseReplicaCountResult` | `ClusterQuotaForCustomerExceededFault`, `InsufficientCacheClusterCapacityFault`, `InvalidCacheClusterStateFault`, `InvalidKMSKeyFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `InvalidVPCNetworkStateFault`, `NodeGroupsPerReplicationGroupQuotaExceededFault`, `NodeQuotaForCustomerExceededFault`, `NoOperationFault`, `ReplicationGroupNotFoundFault` | Dynamically increases the number of replicas in a Valkey or Redis OSS (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Valkey or Redis OSS (cluster mod ... |
| `ListAllowedNodeTypeModifications` | `-` | - | - | - | `AllowedNodeTypeModificationsMessage` | `CacheClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReplicationGroupNotFoundFault` | Lists all available node types that you can scale with your cluster's replication group's current node type. When you use the ModifyCacheCluster or ModifyReplicationGroup operations to scale your cluster or replicati ... |
| `ListTagsForResource` | `-` | - | `ResourceName` | - | `TagListMessage` | `CacheClusterNotFoundFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `CacheSubnetGroupNotFoundFault`, `InvalidARNFault`, `InvalidReplicationGroupStateFault`, `InvalidServerlessCacheSnapshotStateFault`, `InvalidServerlessCacheStateFault`, `ReplicationGroupNotFoundFault`, `ReservedCacheNodeNotFoundFault`, `ServerlessCacheNotFoundFault`, `ServerlessCacheSnapshotNotFoundFault`, `SnapshotNotFoundFault`, `UserGroupNotFoundFault`, `UserNotFoundFault` | Lists all tags currently on a named resource. A tag is a key-value pair where the key and value are case-sensitive. You can use tags to categorize and track all your ElastiCache resources, with the exception of globa ... |
| `ModifyCacheCluster` | `-` | - | `CacheClusterId` | - | `ModifyCacheClusterResult` | `CacheClusterNotFoundFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `InsufficientCacheClusterCapacityFault`, `InvalidCacheClusterStateFault`, `InvalidCacheSecurityGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidVPCNetworkStateFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault` | Modifies the settings for a cluster. You can use this operation to change one or more cluster configuration parameters by specifying the parameters and the new values. |
| `ModifyCacheParameterGroup` | `-` | - | `CacheParameterGroupName`, `ParameterNameValues` | - | `CacheParameterGroupNameMessage` | `CacheParameterGroupNotFoundFault`, `InvalidCacheParameterGroupStateFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Modifies the parameters of a cache parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs. |
| `ModifyCacheSubnetGroup` | `-` | - | `CacheSubnetGroupName` | - | `ModifyCacheSubnetGroupResult` | `CacheSubnetGroupNotFoundFault`, `CacheSubnetQuotaExceededFault`, `InvalidSubnet`, `SubnetInUse`, `SubnetNotAllowedFault` | Modifies an existing cache subnet group. |
| `ModifyGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `ApplyImmediately` | - | `ModifyGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterValueException` | Modifies the settings for a Global datastore. |
| `ModifyReplicationGroup` | `-` | - | `ReplicationGroupId` | - | `ModifyReplicationGroupResult` | `CacheClusterNotFoundFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `InsufficientCacheClusterCapacityFault`, `InvalidCacheClusterStateFault`, `InvalidCacheSecurityGroupStateFault`, `InvalidKMSKeyFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `InvalidUserGroupStateFault`, `InvalidVPCNetworkStateFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `ReplicationGroupNotFoundFault`, `UserGroupNotFoundFault` | Modifies the settings for a replication group. This is limited to Valkey and Redis OSS 7 and above. Scaling for Valkey or Redis OSS (cluster mode enabled) in the ElastiCache User Guide ModifyReplicationGroupShardConf ... |
| `ModifyReplicationGroupShardConfiguration` | `-` | - | `ReplicationGroupId`, `NodeGroupCount`, `ApplyImmediately` | - | `ModifyReplicationGroupShardConfigurationResult` | `InsufficientCacheClusterCapacityFault`, `InvalidCacheClusterStateFault`, `InvalidKMSKeyFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `InvalidVPCNetworkStateFault`, `NodeGroupsPerReplicationGroupQuotaExceededFault`, `NodeQuotaForCustomerExceededFault`, `ReplicationGroupNotFoundFault` | Modifies a replication group's shards (node groups) by allowing you to add shards, remove shards, or rebalance the keyspaces among existing shards. |
| `ModifyServerlessCache` | `-` | - | `ServerlessCacheName` | - | `ModifyServerlessCacheResponse` | `InvalidCredentialsException`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidServerlessCacheStateFault`, `InvalidUserGroupStateFault`, `ServerlessCacheNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `UserGroupNotFoundFault` | This API modifies the attributes of a serverless cache. |
| `ModifyUser` | `-` | - | `UserId` | - | `User` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidUserStateFault`, `ServiceLinkedRoleNotFoundFault`, `UserNotFoundFault` | Changes user password(s) and/or access string. |
| `ModifyUserGroup` | `-` | - | `UserGroupId` | - | `UserGroup` | `DefaultUserRequired`, `DuplicateUserNameFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidUserGroupStateFault`, `ServiceLinkedRoleNotFoundFault`, `UserGroupNotFoundFault`, `UserNotFoundFault` | Changes the list of users that belong to the user group. |
| `PurchaseReservedCacheNodesOffering` | `-` | - | `ReservedCacheNodesOfferingId` | - | `PurchaseReservedCacheNodesOfferingResult` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReservedCacheNodeAlreadyExistsFault`, `ReservedCacheNodeQuotaExceededFault`, `ReservedCacheNodesOfferingNotFoundFault`, `TagQuotaPerResourceExceeded` | Allows you to purchase a reserved cache node offering. Reserved nodes are not eligible for cancellation and are non-refundable. For more information, see Managing Costs with Reserved Nodes . |
| `RebalanceSlotsInGlobalReplicationGroup` | `-` | - | `GlobalReplicationGroupId`, `ApplyImmediately` | - | `RebalanceSlotsInGlobalReplicationGroupResult` | `GlobalReplicationGroupNotFoundFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterValueException` | Redistribute slots to ensure uniform distribution across existing shards in the cluster. |
| `RebootCacheCluster` | `-` | - | `CacheClusterId`, `CacheNodeIdsToReboot` | - | `RebootCacheClusterResult` | `CacheClusterNotFoundFault`, `InvalidCacheClusterStateFault` | Reboots some, or all, of the cache nodes within a provisioned cluster. This operation applies any modified cache parameter groups to the cluster. The reboot operation takes place as soon as possible, and results in a ... |
| `RemoveTagsFromResource` | `-` | - | `ResourceName`, `TagKeys` | - | `TagListMessage` | `CacheClusterNotFoundFault`, `CacheParameterGroupNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `CacheSubnetGroupNotFoundFault`, `InvalidARNFault`, `InvalidReplicationGroupStateFault`, `InvalidServerlessCacheSnapshotStateFault`, `InvalidServerlessCacheStateFault`, `ReplicationGroupNotFoundFault`, `ReservedCacheNodeNotFoundFault`, `ServerlessCacheNotFoundFault`, `ServerlessCacheSnapshotNotFoundFault`, `SnapshotNotFoundFault`, `TagNotFoundFault`, `UserGroupNotFoundFault`, `UserNotFoundFault` | Removes the tags identified by the TagKeys list from the named resource. A tag is a key-value pair where the key and value are case-sensitive. You can use tags to categorize and track all your ElastiCache resources, ... |
| `ResetCacheParameterGroup` | `-` | - | `CacheParameterGroupName` | - | `CacheParameterGroupNameMessage` | `CacheParameterGroupNotFoundFault`, `InvalidCacheParameterGroupStateFault`, `InvalidGlobalReplicationGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Modifies the parameters of a cache parameter group to the engine or system default value. You can reset specific parameters by submitting a list of parameter names. To reset the entire cache parameter group, specify ... |
| `RevokeCacheSecurityGroupIngress` | `-` | - | `CacheSecurityGroupName`, `EC2SecurityGroupName`, `EC2SecurityGroupOwnerId` | - | `RevokeCacheSecurityGroupIngressResult` | `AuthorizationNotFoundFault`, `CacheSecurityGroupNotFoundFault`, `InvalidCacheSecurityGroupStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException` | Revokes ingress from a cache security group. Use this operation to disallow access from an Amazon EC2 security group that had been previously authorized. |
| `StartMigration` | `-` | - | `ReplicationGroupId`, `CustomerNodeEndpointList` | - | `StartMigrationResponse` | `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `ReplicationGroupAlreadyUnderMigrationFault`, `ReplicationGroupNotFoundFault` | Start the migration of data. |
| `TestFailover` | `-` | - | `ReplicationGroupId`, `NodeGroupId` | - | `TestFailoverResult` | `APICallRateForCustomerExceededFault`, `InvalidCacheClusterStateFault`, `InvalidKMSKeyFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `NodeGroupNotFoundFault`, `ReplicationGroupNotFoundFault`, `TestFailoverNotAvailableFault` | Represents the input of a TestFailover operation which tests automatic failover on a specified node group (called shard in the console) in a replication group (called cluster in the console). This API is designed for ... |
| `TestMigration` | `-` | - | `ReplicationGroupId`, `CustomerNodeEndpointList` | - | `TestMigrationResponse` | `InvalidParameterValueException`, `InvalidReplicationGroupStateFault`, `ReplicationGroupAlreadyUnderMigrationFault`, `ReplicationGroupNotFoundFault` | Async API to test connection between source and target replication group. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `APICallRateForCustomerExceededFault` | `structure` | message | The customer has exceeded the allowed rate of API calls. |
| `AuthorizationAlreadyExistsFault` | `structure` | message | The specified Amazon EC2 security group is already authorized for the specified cache security group. |
| `AuthorizationNotFoundFault` | `structure` | message | The specified Amazon EC2 security group is not authorized for the specified cache security group. |
| `CacheClusterAlreadyExistsFault` | `structure` | message | You already have a cluster with the given identifier. |
| `CacheClusterNotFoundFault` | `structure` | message | The requested cluster ID does not refer to an existing cluster. |
| `CacheParameterGroupAlreadyExistsFault` | `structure` | message | A cache parameter group with the requested name already exists. |
| `CacheParameterGroupNotFoundFault` | `structure` | message | The requested cache parameter group name does not refer to an existing cache parameter group. |
| `CacheParameterGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of cache security groups. |
| `CacheSecurityGroupAlreadyExistsFault` | `structure` | message | A cache security group with the specified name already exists. |
| `CacheSecurityGroupNotFoundFault` | `structure` | message | The requested cache security group name does not refer to an existing cache security group. |
| `CacheSecurityGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of cache security groups. |
| `CacheSubnetGroupAlreadyExistsFault` | `structure` | message | The requested cache subnet group name is already in use by an existing cache subnet group. |
| `CacheSubnetGroupInUse` | `structure` | message | The requested cache subnet group is currently in use. |
| `CacheSubnetGroupNotFoundFault` | `structure` | message | The requested cache subnet group name does not refer to an existing cache subnet group. |
| `CacheSubnetGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of cache subnet groups. |
| `CacheSubnetQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of subnets in a cache subnet group. |
| `ClusterQuotaForCustomerExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of clusters per customer. |
| `DefaultUserAssociatedToUserGroupFault` | `structure` | message | The default user assigned to the user group. |
| `DefaultUserRequired` | `structure` | message | You must add default user to a user group. |
| `DuplicateUserNameFault` | `structure` | message | A user with this username already exists. |
| `GlobalReplicationGroupAlreadyExistsFault` | `structure` | message | The Global datastore name already exists. |
| `GlobalReplicationGroupNotFoundFault` | `structure` | message | The Global datastore does not exist |
| `InsufficientCacheClusterCapacityFault` | `structure` | message | The requested cache node type is not available in the specified Availability Zone. For more information, see InsufficientCacheClusterCapacity in the ElastiC ... |
| `InvalidARNFault` | `structure` | message | The requested Amazon Resource Name (ARN) does not refer to an existing resource. |
| `InvalidCacheClusterStateFault` | `structure` | message | The requested cluster is not in the available state. |
| `InvalidCacheParameterGroupStateFault` | `structure` | message | The current state of the cache parameter group does not allow the requested operation to occur. |
| `InvalidCacheSecurityGroupStateFault` | `structure` | message | The current state of the cache security group does not allow deletion. |
| `InvalidCredentialsException` | `structure` | message | You must enter valid credentials. |
| `InvalidGlobalReplicationGroupStateFault` | `structure` | message | The Global datastore is not available or in primary-only state. |
| `InvalidKMSKeyFault` | `structure` | message | The KMS key supplied is not valid. |
| `InvalidParameterCombinationException` | `structure` | message | Two or more incompatible parameters were specified. |
| `InvalidParameterValueException` | `structure` | message | The value for a parameter is invalid. |
| `InvalidReplicationGroupStateFault` | `structure` | message | The requested replication group is not in the available state. |
| `InvalidServerlessCacheSnapshotStateFault` | `structure` | message | The state of the serverless cache snapshot was not received. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `InvalidServerlessCacheStateFault` | `structure` | message | The account for these credentials is not currently active. |
| `InvalidSnapshotStateFault` | `structure` | message | The current state of the snapshot does not allow the requested operation to occur. |
| `InvalidSubnet` | `structure` | message | An invalid subnet identifier was specified. |
| `InvalidUserGroupStateFault` | `structure` | message | The user group is not in an active state. |
| `InvalidUserStateFault` | `structure` | message | The user is not in active state. |
| `InvalidVPCNetworkStateFault` | `structure` | message | The VPC network is in an invalid state. |
| `NoOperationFault` | `structure` | message | The operation was not performed because no changes were required. |
| `NodeGroupNotFoundFault` | `structure` | message | The node group specified by the NodeGroupId parameter could not be found. Please verify that the node group exists and that you spelled the NodeGroupId valu ... |
| `NodeGroupsPerReplicationGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum allowed number of node groups (shards) in a single replication group. The default maximu ... |
| `NodeQuotaForClusterExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of cache nodes in a single cluster. |
| `NodeQuotaForCustomerExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of cache nodes per customer. |
| `ReplicationGroupAlreadyExistsFault` | `structure` | message | The specified replication group already exists. |
| `ReplicationGroupAlreadyUnderMigrationFault` | `structure` | message | The targeted replication group is not available. |
| `ReplicationGroupNotFoundFault` | `structure` | message | The specified replication group does not exist. |
| `ReplicationGroupNotUnderMigrationFault` | `structure` | message | The designated replication group is not available for data migration. |
| `ReservedCacheNodeAlreadyExistsFault` | `structure` | message | You already have a reservation with the given identifier. |
| `ReservedCacheNodeNotFoundFault` | `structure` | message | The requested reserved cache node was not found. |
| `ReservedCacheNodeQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the user's cache node quota. |
| `ReservedCacheNodesOfferingNotFoundFault` | `structure` | message | The requested cache node offering does not exist. |
| `ServerlessCacheAlreadyExistsFault` | `structure` | message | A serverless cache with this name already exists. |
| `ServerlessCacheNotFoundFault` | `structure` | message | The serverless cache was not found or does not exist. |
| `ServerlessCacheQuotaForCustomerExceededFault` | `structure` | message | The number of serverless caches exceeds the customer quota. |
| `ServerlessCacheSnapshotAlreadyExistsFault` | `structure` | message | A serverless cache snapshot with this name already exists. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `ServerlessCacheSnapshotNotFoundFault` | `structure` | message | This serverless cache snapshot could not be found or does not exist. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `ServerlessCacheSnapshotQuotaExceededFault` | `structure` | message | The number of serverless cache snapshots exceeds the customer snapshot quota. Available for Valkey, Redis OSS and Serverless Memcached only. |
| `ServiceLinkedRoleNotFoundFault` | `structure` | message | The specified service linked role (SLR) was not found. |
| `ServiceUpdateNotFoundFault` | `structure` | message | The service update doesn't exist |
| `SnapshotAlreadyExistsFault` | `structure` | message | You already have a snapshot with the given name. |
| `SnapshotFeatureNotSupportedFault` | `structure` | message | You attempted one of the following operations: Creating a snapshot of a Valkey or Redis OSS cluster running on a cache.t1.micro cache node. Creating a snaps ... |
| `SnapshotNotFoundFault` | `structure` | message | The requested snapshot name does not refer to an existing snapshot. |
| `SnapshotQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of snapshots. |
| `SubnetInUse` | `structure` | message | The requested subnet is being used by another cache subnet group. |
| `SubnetNotAllowedFault` | `structure` | message | At least one subnet ID does not match the other subnet IDs. This mismatch typically occurs when a user sets one subnet ID to a regional Availability Zone an ... |
| `TagNotFoundFault` | `structure` | message | The requested tag was not found on this resource. |
| `TagQuotaPerResourceExceeded` | `structure` | message | The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a ... |
| `TestFailoverNotAvailableFault` | `structure` | message | The TestFailover action is not available. |
| `UserAlreadyExistsFault` | `structure` | message | A user with this ID already exists. |
| `UserGroupAlreadyExistsFault` | `structure` | message | The user group with this ID already exists. |
| `UserGroupNotFoundFault` | `structure` | message | The user group was not found or does not exist |
| `UserGroupQuotaExceededFault` | `structure` | message | The number of users exceeds the user group limit. |
| `UserNotFoundFault` | `structure` | message | The user does not exist or could not be found. |
| `UserQuotaExceededFault` | `structure` | message | The quota of users has been exceeded. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

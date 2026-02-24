# Amazon Neptune

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Neptune Amazon Neptune is a fast, reliable, fully-managed graph database service that makes it easy to build and run applications that work with highly connected datasets. The core of Amazon Neptune is a purpose-built, high-performance graph database engine optimized for storing billions of relationships and querying the graph with milliseconds latency. Amazon Neptune supports popular graph models Property Graph and W3C's RDF, and their respective query languages Apache TinkerPop Gremlin and SPARQL, allowing you to easily build queries that efficiently navigate highly connected datasets. Neptune powers graph use cases such as recommendation engines, fraud detection, knowledge graphs, drug discovery, and network security. This interface reference for Amazon Neptune contains documentation for a programming or command line interface you can use to manage Amazon Neptune.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Neptune where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Neptune by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon Neptune resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Neptune workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Delete`, `Modify`, `Remove` operation families, including `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `CreateDBCluster`, `CreateDBClusterEndpoint`.

## Service Identity and Protocol

- AWS model slug: `neptune`
- AWS SDK for Rust slug: `neptune`
- Model version: `2014-10-31`
- Model file: `vendor/api-models-aws/models/neptune/service/2014-10-31/neptune-2014-10-31.json`
- SDK ID: `Neptune`
- Endpoint prefix: `rds`
- ARN namespace: `rds`
- CloudFormation name: `Neptune`
- CloudTrail event source: `neptune.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (20), `Create` (9), `Delete` (9), `Modify` (9), `Remove` (4), `Add` (3), `Copy` (3), `Failover` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddRoleToDBCluster`, `AddSourceIdentifierToSubscription`, `AddTagsToResource`, `CreateDBCluster`, `CreateDBClusterEndpoint`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBParameterGroup`, `CreateDBSubnetGroup`, `CreateEventSubscription`, `CreateGlobalCluster`, `DeleteDBCluster`, `DeleteDBClusterEndpoint`, `DeleteDBClusterParameterGroup`, `DeleteDBClusterSnapshot`, `DeleteDBInstance`, `DeleteDBParameterGroup`, `DeleteDBSubnetGroup`, `DeleteEventSubscription`, ... (+18).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstances`, `DescribeDBParameterGroups`, `DescribeDBParameters`, `DescribeDBSubnetGroups`, `DescribeEngineDefaultClusterParameters`, `DescribeEngineDefaultParameters`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeGlobalClusters`, `DescribeOrderableDBInstanceOptions`, `DescribePendingMaintenanceActions`, `DescribeValidDBInstanceModifications`, ... (+1).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDBCluster`, `StopDBCluster`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 64 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `EC2/VPC`, `ECS`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/neptune/latest/userguide/feature-overview-db-clusters.html
- https://docs.aws.amazon.com/neptune/latest/userguide/backup-restore.html

Research outcomes:
- A Neptune DB cluster has one primary DB instance and up to 15 read-replica DB instances that share one managed storage volume.
- The primary coordinates all writes and also supports reads.
- Read replicas are dedicated to read-only queries and each has its own endpoint.
- Neptune automatically fails over from an unavailable primary to a read replica, using a failover priority that can be specified.
- Replica lag is usually much less than 100 ms after primary writes, but can increase under large write volumes.
- If a primary fails and no read replica exists, the cluster remains unavailable until the primary is recreated, which is slower than replica promotion.
- Failover promotion briefly interrupts reads and writes to the primary endpoint while the promoted replica reboots.
- Neptune uses quorum writes across six copies in three Availability Zones, requiring four of six storage nodes to acknowledge a durable write.
- Each instance has query threads equal to two times its vCPU count. Additional requests wait in a FIFO server-side queue of roughly 8000 pending requests, after which Neptune returns `ThrottlingException`.
- Automated backups are continuous and incremental, with a retention period from 1 to 35 days. Neptune cannot disable automated backups.
- Deleting a DB cluster deletes automated backups, but manual snapshots are retained. Manual snapshots can preserve data beyond the retention period.

Parity implications:
- Model clusters, instances, reader/primary roles, endpoints, shared storage, replica lag, failover priority, snapshots, continuous backups, restore windows, and start/stop state separately.
- Query execution simulation should expose per-instance read/write role constraints and throttling/queueing limits where relevant.
- Delete and restore operations should distinguish automated backup retention from manual snapshot persistence.

## Current Network Resource Stub Semantics

Neptune currently stores DB subnet groups and VPC security group references locally.

- DB subnet groups keep raw subnet IDs and an optional or unset VPC ID field in Neptune state.
- DB instances and clusters store DB subnet group names and VPC security group IDs without EC2 resolution.
- Deletion and describe paths check Neptune-local resource maps, not EC2 dependencies.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstances`, `DescribeDBParameterGroups`, `DescribeDBParameters`, `DescribeDBSubnetGroups`, `DescribeEngineDefaultClusterParameters`, `DescribeEngineDefaultParameters`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeGlobalClusters`, `DescribeOrderableDBInstanceOptions`, `DescribePendingMaintenanceActions`, `DescribeValidDBInstanceModifications`
- Traits: `paginated` (16)
- Common required input members in this group: `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBParameterGroupFamily`, `DBParameterGroupName`, `Engine`

### Create

- Operations: `CreateDBCluster`, `CreateDBClusterEndpoint`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBParameterGroup`, `CreateDBSubnetGroup`, `CreateEventSubscription`, `CreateGlobalCluster`
- Common required input members in this group: `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceClass`, `DBInstanceIdentifier`, `DBParameterGroupFamily`, `DBParameterGroupName`, `DBSubnetGroupDescription`, `DBSubnetGroupName`, `Description`, `EndpointType`, `Engine`, `GlobalClusterIdentifier`, `SnsTopicArn`, `SubnetIds`, `SubscriptionName`

### Delete

- Operations: `DeleteDBCluster`, `DeleteDBClusterEndpoint`, `DeleteDBClusterParameterGroup`, `DeleteDBClusterSnapshot`, `DeleteDBInstance`, `DeleteDBParameterGroup`, `DeleteDBSubnetGroup`, `DeleteEventSubscription`, `DeleteGlobalCluster`
- Common required input members in this group: `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBParameterGroupName`, `DBSubnetGroupName`, `GlobalClusterIdentifier`, `SubscriptionName`

### Modify

- Operations: `ModifyDBCluster`, `ModifyDBClusterEndpoint`, `ModifyDBClusterParameterGroup`, `ModifyDBClusterSnapshotAttribute`, `ModifyDBInstance`, `ModifyDBParameterGroup`, `ModifyDBSubnetGroup`, `ModifyEventSubscription`, `ModifyGlobalCluster`
- Common required input members in this group: `AttributeName`, `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBParameterGroupName`, `DBSubnetGroupName`, `GlobalClusterIdentifier`, `Parameters`, `SubnetIds`, `SubscriptionName`

### Remove

- Operations: `RemoveFromGlobalCluster`, `RemoveRoleFromDBCluster`, `RemoveSourceIdentifierFromSubscription`, `RemoveTagsFromResource`
- Common required input members in this group: `DBClusterIdentifier`, `DbClusterIdentifier`, `GlobalClusterIdentifier`, `ResourceName`, `RoleArn`, `SourceIdentifier`, `SubscriptionName`, `TagKeys`

### Add

- Operations: `AddRoleToDBCluster`, `AddSourceIdentifierToSubscription`, `AddTagsToResource`
- Common required input members in this group: `DBClusterIdentifier`, `ResourceName`, `RoleArn`, `SourceIdentifier`, `SubscriptionName`, `Tags`

### Copy

- Operations: `CopyDBClusterParameterGroup`, `CopyDBClusterSnapshot`, `CopyDBParameterGroup`
- Common required input members in this group: `SourceDBClusterParameterGroupIdentifier`, `SourceDBClusterSnapshotIdentifier`, `SourceDBParameterGroupIdentifier`, `TargetDBClusterParameterGroupDescription`, `TargetDBClusterParameterGroupIdentifier`, `TargetDBClusterSnapshotIdentifier`, `TargetDBParameterGroupDescription`, `TargetDBParameterGroupIdentifier`

### Failover

- Operations: `FailoverDBCluster`, `FailoverGlobalCluster`
- Common required input members in this group: `GlobalClusterIdentifier`, `TargetDbClusterIdentifier`

### Reset

- Operations: `ResetDBClusterParameterGroup`, `ResetDBParameterGroup`
- Common required input members in this group: `DBClusterParameterGroupName`, `DBParameterGroupName`

### Restore

- Operations: `RestoreDBClusterFromSnapshot`, `RestoreDBClusterToPointInTime`
- Common required input members in this group: `DBClusterIdentifier`, `Engine`, `SnapshotIdentifier`, `SourceDBClusterIdentifier`

### Apply

- Operations: `ApplyPendingMaintenanceAction`
- Common required input members in this group: `ApplyAction`, `OptInType`, `ResourceIdentifier`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `ResourceName`

### Promote

- Operations: `PromoteReadReplicaDBCluster`
- Common required input members in this group: `DBClusterIdentifier`

### Reboot

- Operations: `RebootDBInstance`
- Common required input members in this group: `DBInstanceIdentifier`

### Start

- Operations: `StartDBCluster`
- Common required input members in this group: `DBClusterIdentifier`

### Stop

- Operations: `StopDBCluster`
- Common required input members in this group: `DBClusterIdentifier`

### Switchover

- Operations: `SwitchoverGlobalCluster`
- Common required input members in this group: `GlobalClusterIdentifier`, `TargetDbClusterIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddRoleToDBCluster` | - | - | `DBClusterIdentifier`, `RoleArn` | - | `Unit` | `DBClusterNotFoundFault`, `DBClusterRoleAlreadyExistsFault`, `DBClusterRoleQuotaExceededFault`, `InvalidDBClusterStateFault` | Associates an Identity and Access Management (IAM) role with an Neptune DB cluster. |
| `AddSourceIdentifierToSubscription` | - | - | `SourceIdentifier`, `SubscriptionName` | - | `AddSourceIdentifierToSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Adds a source identifier to an existing event notification subscription. |
| `AddTagsToResource` | - | - | `ResourceName`, `Tags` | - | `Unit` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSnapshotNotFoundFault` | Adds metadata tags to an Amazon Neptune resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon Neptune resources, or used in a Condition statement in an IAM policy for Amazon Neptune. |
| `ApplyPendingMaintenanceAction` | - | - | `ApplyAction`, `OptInType`, `ResourceIdentifier` | - | `ApplyPendingMaintenanceActionResult` | `ResourceNotFoundFault` | Applies a pending maintenance action to a resource (for example, to a DB instance). |
| `CopyDBClusterParameterGroup` | - | - | `SourceDBClusterParameterGroupIdentifier`, `TargetDBClusterParameterGroupDescription`, `TargetDBClusterParameterGroupIdentifier` | - | `CopyDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified DB cluster parameter group. |
| `CopyDBClusterSnapshot` | - | - | `SourceDBClusterSnapshotIdentifier`, `TargetDBClusterSnapshotIdentifier` | - | `CopyDBClusterSnapshotResult` | `DBClusterSnapshotAlreadyExistsFault`, `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Copies a snapshot of a DB cluster. To copy a DB cluster snapshot from a shared manual DB cluster snapshot, `SourceDBClusterSnapshotIdentifier` must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot. |
| `CopyDBParameterGroup` | - | - | `SourceDBParameterGroupIdentifier`, `TargetDBParameterGroupDescription`, `TargetDBParameterGroupIdentifier` | - | `CopyDBParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified DB parameter group. |
| `CreateDBCluster` | - | - | `DBClusterIdentifier`, `Engine` | - | `CreateDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `GlobalClusterNotFoundFault`, ... (+9) | Creates a new Amazon Neptune DB cluster. You can use the `ReplicationSourceIdentifier` parameter to create the DB cluster as a Read Replica of another DB cluster or Amazon Neptune DB instance. |
| `CreateDBClusterEndpoint` | - | - | `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `EndpointType` | - | `CreateDBClusterEndpointOutput` | `DBClusterEndpointAlreadyExistsFault`, `DBClusterEndpointQuotaExceededFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Creates a new custom endpoint and associates it with an Amazon Neptune DB cluster. |
| `CreateDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName`, `DBParameterGroupFamily`, `Description` | - | `CreateDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new DB cluster parameter group. Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster. |
| `CreateDBClusterSnapshot` | - | - | `DBClusterIdentifier`, `DBClusterSnapshotIdentifier` | - | `CreateDBClusterSnapshotResult` | `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `SnapshotQuotaExceededFault` | Creates a snapshot of a DB cluster. |
| `CreateDBInstance` | - | - | `DBClusterIdentifier`, `DBInstanceClass`, `DBInstanceIdentifier`, `Engine` | - | `CreateDBInstanceResult` | `AuthorizationNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, ... (+10) | Creates a new DB instance. |
| `CreateDBParameterGroup` | - | - | `DBParameterGroupFamily`, `DBParameterGroupName`, `Description` | - | `CreateDBParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new DB parameter group. A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. |
| `CreateDBSubnetGroup` | - | - | `DBSubnetGroupDescription`, `DBSubnetGroupName`, `SubnetIds` | - | `CreateDBSubnetGroupResult` | `DBSubnetGroupAlreadyExistsFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupQuotaExceededFault`, `DBSubnetQuotaExceededFault`, `InvalidSubnet` | Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the Amazon Region. |
| `CreateEventSubscription` | - | - | `SnsTopicArn`, `SubscriptionName` | - | `CreateEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SourceNotFoundFault`, `SubscriptionAlreadyExistFault`, `SubscriptionCategoryNotFoundFault` | Creates an event notification subscription. This action requires a topic ARN (Amazon Resource Name) created by either the Neptune console, the SNS console, or the SNS API. |
| `CreateGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `CreateGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterAlreadyExistsFault`, `GlobalClusterQuotaExceededFault`, `InvalidDBClusterStateFault` | Creates a Neptune global database spread across multiple Amazon Regions. The global database contains a single primary cluster with read-write capability, and read-only secondary clusters that receive data from the primary cluster through high-speed... |
| `DeleteDBCluster` | - | - | `DBClusterIdentifier` | - | `DeleteDBClusterResult` | `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `SnapshotQuotaExceededFault` | The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered. |
| `DeleteDBClusterEndpoint` | - | - | `DBClusterEndpointIdentifier` | - | `DeleteDBClusterEndpointOutput` | `DBClusterEndpointNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault` | Deletes a custom endpoint and removes it from an Amazon Neptune DB cluster. |
| `DeleteDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters. |
| `DeleteDBClusterSnapshot` | - | - | `DBClusterSnapshotIdentifier` | - | `DeleteDBClusterSnapshotResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault` | Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated. |
| `DeleteDBInstance` | - | - | `DBInstanceIdentifier` | - | `DeleteDBInstanceResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `SnapshotQuotaExceededFault` | The DeleteDBInstance action deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. |
| `DeleteDBParameterGroup` | - | - | `DBParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified DBParameterGroup. The DBParameterGroup to be deleted can't be associated with any DB instances. |
| `DeleteDBSubnetGroup` | - | - | `DBSubnetGroupName` | - | `Unit` | `DBSubnetGroupNotFoundFault`, `InvalidDBSubnetGroupStateFault`, `InvalidDBSubnetStateFault` | Deletes a DB subnet group. The specified database subnet group must not be associated with any DB instances. |
| `DeleteEventSubscription` | - | - | `SubscriptionName` | - | `DeleteEventSubscriptionResult` | `InvalidEventSubscriptionStateFault`, `SubscriptionNotFoundFault` | Deletes an event notification subscription. |
| `DeleteGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `DeleteGlobalClusterResult` | `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Deletes a global database. The primary and all secondary clusters must already be detached or deleted first. |
| `DescribeDBClusterEndpoints` | - | `paginated` | - | - | `DBClusterEndpointMessage` | `DBClusterNotFoundFault` | Returns information about endpoints for an Amazon Neptune DB cluster. This operation can also return information for Amazon RDS clusters and Amazon DocDB clusters. |
| `DescribeDBClusterParameterGroups` | - | `paginated` | - | - | `DBClusterParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of `DBClusterParameterGroup` descriptions. If a `DBClusterParameterGroupName` parameter is specified, the list will contain only the description of the specified DB cluster parameter group. |
| `DescribeDBClusterParameters` | - | `paginated` | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular DB cluster parameter group. |
| `DescribeDBClusterSnapshotAttributes` | - | - | `DBClusterSnapshotIdentifier` | - | `DescribeDBClusterSnapshotAttributesResult` | `DBClusterSnapshotNotFoundFault` | Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot. When sharing snapshots with other Amazon accounts, `DescribeDBClusterSnapshotAttributes` returns the `restore` attribute and a list of IDs for the Amazon... |
| `DescribeDBClusterSnapshots` | - | `paginated` | - | - | `DBClusterSnapshotMessage` | `DBClusterSnapshotNotFoundFault` | Returns information about DB cluster snapshots. This API action supports pagination. |
| `DescribeDBClusters` | - | `paginated` | - | - | `DBClusterMessage` | `DBClusterNotFoundFault` | Returns information about provisioned DB clusters, and supports pagination. This operation can also return information for Amazon RDS clusters and Amazon DocDB clusters. |
| `DescribeDBEngineVersions` | - | `paginated` | - | - | `DBEngineVersionMessage` | - | Returns a list of the available DB engines. |
| `DescribeDBInstances` | - | `paginated` | - | - | `DBInstanceMessage` | `DBInstanceNotFoundFault` | Returns information about provisioned instances, and supports pagination. This operation can also return information for Amazon RDS instances and Amazon DocDB instances. |
| `DescribeDBParameterGroups` | - | `paginated` | - | - | `DBParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of `DBParameterGroup` descriptions. If a `DBParameterGroupName` is specified, the list will contain only the description of the specified DB parameter group. |
| `DescribeDBParameters` | - | `paginated` | `DBParameterGroupName` | - | `DBParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular DB parameter group. |
| `DescribeDBSubnetGroups` | - | `paginated` | - | - | `DBSubnetGroupMessage` | `DBSubnetGroupNotFoundFault` | Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup. |
| `DescribeEngineDefaultClusterParameters` | - | - | `DBParameterGroupFamily` | - | `DescribeEngineDefaultClusterParametersResult` | - | Returns the default engine and system parameter information for the cluster database engine. |
| `DescribeEngineDefaultParameters` | - | `paginated` | `DBParameterGroupFamily` | - | `DescribeEngineDefaultParametersResult` | - | Returns the default engine and system parameter information for the specified database engine. |
| `DescribeEventCategories` | - | - | - | - | `EventCategoriesMessage` | - | Displays a list of categories for all event source types, or, if specified, for a specified source type. |
| `DescribeEventSubscriptions` | - | `paginated` | - | - | `EventSubscriptionsMessage` | `SubscriptionNotFoundFault` | Lists all the subscription descriptions for a customer account. The description for a subscription includes SubscriptionName, SNSTopicARN, CustomerID, SourceType, SourceID, CreationTime, and Status. |
| `DescribeEvents` | - | `paginated` | - | - | `EventsMessage` | - | Returns events related to DB instances, DB security groups, DB snapshots, and DB parameter groups for the past 14 days. Events specific to a particular DB instance, DB security group, database snapshot, or DB parameter group can be obtained by providing the... |
| `DescribeGlobalClusters` | - | `paginated` | - | - | `GlobalClustersMessage` | `GlobalClusterNotFoundFault` | Returns information about Neptune global database clusters. This API supports pagination. |
| `DescribeOrderableDBInstanceOptions` | - | `paginated` | `Engine` | - | `OrderableDBInstanceOptionsMessage` | - | Returns a list of orderable DB instance options for the specified engine. |
| `DescribePendingMaintenanceActions` | - | `paginated` | - | - | `PendingMaintenanceActionsMessage` | `ResourceNotFoundFault` | Returns a list of resources (for example, DB instances) that have at least one pending maintenance action. |
| `DescribeValidDBInstanceModifications` | - | - | `DBInstanceIdentifier` | - | `DescribeValidDBInstanceModificationsResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | You can call DescribeValidDBInstanceModifications to learn what modifications you can make to your DB instance. You can use this information when you call ModifyDBInstance. |
| `FailoverDBCluster` | - | - | - | - | `FailoverDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Forces a failover for a DB cluster. A failover for a DB cluster promotes one of the Read Replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer). |
| `FailoverGlobalCluster` | - | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `FailoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Initiates the failover process for a Neptune global database. A failover for a Neptune global database promotes one of secondary read-only DB clusters to be the primary DB cluster and demotes the primary DB cluster to being a secondary (read-only) DB cluster. |
| `ListTagsForResource` | - | - | `ResourceName` | - | `TagListMessage` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSnapshotNotFoundFault` | Lists all tags on an Amazon Neptune resource. |
| `ModifyDBCluster` | - | - | `DBClusterIdentifier` | - | `ModifyDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBSubnetGroupNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSecurityGroupStateFault`, `InvalidDBSubnetGroupStateFault`, ... (+4) | Modify a setting for a DB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. |
| `ModifyDBClusterEndpoint` | - | - | `DBClusterEndpointIdentifier` | - | `ModifyDBClusterEndpointOutput` | `DBClusterEndpointNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Modifies the properties of an endpoint in an Amazon Neptune DB cluster. |
| `ModifyDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName`, `Parameters` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: `ParameterName`, `ParameterValue`, and `ApplyMethod`. |
| `ModifyDBClusterSnapshotAttribute` | - | - | `AttributeName`, `DBClusterSnapshotIdentifier` | - | `ModifyDBClusterSnapshotAttributeResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `SharedSnapshotQuotaExceededFault` | Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot. To share a manual DB cluster snapshot with other Amazon accounts, specify `restore` as the `AttributeName` and use the `ValuesToAdd` parameter to add a... |
| `ModifyDBInstance` | - | - | `DBInstanceIdentifier` | - | `ModifyDBInstanceResult` | `AuthorizationNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBUpgradeDependencyFailureFault`, `DomainNotFoundFault`, ... (+8) | Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. |
| `ModifyDBParameterGroup` | - | - | `DBParameterGroupName`, `Parameters` | - | `DBParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: `ParameterName`, `ParameterValue`, and `ApplyMethod`. |
| `ModifyDBSubnetGroup` | - | - | `DBSubnetGroupName`, `SubnetIds` | - | `ModifyDBSubnetGroupResult` | `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DBSubnetQuotaExceededFault`, `InvalidSubnet`, `SubnetAlreadyInUse` | Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the Amazon Region. |
| `ModifyEventSubscription` | - | - | `SubscriptionName` | - | `ModifyEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SubscriptionCategoryNotFoundFault`, `SubscriptionNotFoundFault` | Modifies an existing event notification subscription. Note that you can't modify the source identifiers using this call; to change source identifiers for a subscription, use the AddSourceIdentifierToSubscription and RemoveSourceIdentifierFromSubscription... |
| `ModifyGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `ModifyGlobalClusterResult` | `GlobalClusterAlreadyExistsFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidGlobalClusterStateFault` | Modify a setting for an Amazon Neptune global cluster. You can change one or more database configuration parameters by specifying these parameters and their new values in the request. |
| `PromoteReadReplicaDBCluster` | - | - | `DBClusterIdentifier` | - | `PromoteReadReplicaDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault` | Not supported. |
| `RebootDBInstance` | - | - | `DBInstanceIdentifier` | - | `RebootDBInstanceResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the instance for the changes to take effect. |
| `RemoveFromGlobalCluster` | - | - | `DbClusterIdentifier`, `GlobalClusterIdentifier` | - | `RemoveFromGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Detaches a Neptune DB cluster from a Neptune global database. A secondary cluster becomes a normal standalone cluster with read-write capability instead of being read-only, and no longer receives data from a the primary cluster. |
| `RemoveRoleFromDBCluster` | - | - | `DBClusterIdentifier`, `RoleArn` | - | `Unit` | `DBClusterNotFoundFault`, `DBClusterRoleNotFoundFault`, `InvalidDBClusterStateFault` | Disassociates an Identity and Access Management (IAM) role from a DB cluster. |
| `RemoveSourceIdentifierFromSubscription` | - | - | `SourceIdentifier`, `SubscriptionName` | - | `RemoveSourceIdentifierFromSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Removes a source identifier from an existing event notification subscription. |
| `RemoveTagsFromResource` | - | - | `ResourceName`, `TagKeys` | - | `Unit` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSnapshotNotFoundFault` | Removes metadata tags from an Amazon Neptune resource. |
| `ResetDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: `ParameterName` and `ApplyMethod`. |
| `ResetDBParameterGroup` | - | - | `DBParameterGroupName` | - | `DBParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: `ParameterName` and `ApplyMethod`. |
| `RestoreDBClusterFromSnapshot` | - | - | `DBClusterIdentifier`, `Engine`, `SnapshotIdentifier` | - | `RestoreDBClusterFromSnapshotResult` | `DBClusterAlreadyExistsFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSubnetGroupNotFoundFault`, `InsufficientDBClusterCapacityFault`, `InsufficientStorageClusterCapacityFault`, ... (+8) | Creates a new DB cluster from a DB snapshot or DB cluster snapshot. If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group. |
| `RestoreDBClusterToPointInTime` | - | - | `DBClusterIdentifier`, `SourceDBClusterIdentifier` | - | `RestoreDBClusterToPointInTimeResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSubnetGroupNotFoundFault`, `InsufficientDBClusterCapacityFault`, `InsufficientStorageClusterCapacityFault`, ... (+9) | Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before `LatestRestorableTime` for up to `BackupRetentionPeriod` days. |
| `StartDBCluster` | - | - | `DBClusterIdentifier` | - | `StartDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Starts an Amazon Neptune DB cluster that was stopped using the Amazon console, the Amazon CLI stop-db-cluster command, or the StopDBCluster API. |
| `StopDBCluster` | - | - | `DBClusterIdentifier` | - | `StopDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Stops an Amazon Neptune DB cluster. When you stop a DB cluster, Neptune retains the DB cluster's metadata, including its endpoints and DB parameter groups. |
| `SwitchoverGlobalCluster` | - | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `SwitchoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Switches over the specified secondary DB cluster to be the new primary DB cluster in the global database cluster. Switchover operations were previously called "managed planned failovers." Promotes the specified secondary cluster to assume full read/write... |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DBClusterNotFoundFault` | `structure` | `message` | DBClusterIdentifier does not refer to an existing DB cluster. |
| `InvalidDBClusterStateFault` | `structure` | `message` | The DB cluster is not in a valid state. |
| `DBParameterGroupNotFoundFault` | `structure` | `message` | DBParameterGroupName does not refer to an existing DB parameter group. |
| `InvalidDBInstanceStateFault` | `structure` | `message` | The specified DB instance is not in the available state. |
| `DBInstanceNotFoundFault` | `structure` | `message` | DBInstanceIdentifier does not refer to an existing DB instance. |
| `DBSubnetGroupNotFoundFault` | `structure` | `message` | DBSubnetGroupName does not refer to an existing DB subnet group. |
| `DBClusterSnapshotNotFoundFault` | `structure` | `message` | DBClusterSnapshotIdentifier does not refer to an existing DB cluster snapshot. |
| `InvalidDBClusterSnapshotStateFault` | `structure` | `message` | The supplied value is not a valid DB cluster snapshot state. |
| `GlobalClusterNotFoundFault` | `structure` | `message` | The `GlobalClusterIdentifier` doesn't refer to an existing global database cluster. |
| `InvalidSubnet` | `structure` | `message` | The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC. |
| `InvalidGlobalClusterStateFault` | `structure` | `message` | The global cluster is in an invalid state and can't perform the requested operation. |
| `InvalidVPCNetworkStateFault` | `structure` | `message` | DB subnet group does not cover all Availability Zones after it is created because users' change. |
| `StorageQuotaExceededFault` | `structure` | `message` | Request would result in user exceeding the allowed amount of storage available across all DB instances. |
| `InvalidDBParameterGroupStateFault` | `structure` | `message` | The DB parameter group is in use or is in an invalid state. |
| `SubscriptionNotFoundFault` | `structure` | `message` | The designated subscription could not be found. |
| `KMSKeyNotAccessibleFault` | `structure` | `message` | Error accessing KMS key. |
| `DBSnapshotNotFoundFault` | `structure` | `message` | DBSnapshotIdentifier does not refer to an existing DB snapshot. |
| `DBParameterGroupAlreadyExistsFault` | `structure` | `message` | A DB parameter group with the same name exists. |
| `DBParameterGroupQuotaExceededFault` | `structure` | `message` | Request would result in user exceeding the allowed number of DB parameter groups. |
| `SnapshotQuotaExceededFault` | `structure` | `message` | Request would result in user exceeding the allowed number of DB snapshots. |
| `DBClusterAlreadyExistsFault` | `structure` | `message` | User already has a DB cluster with the given identifier. |
| `DBClusterParameterGroupNotFoundFault` | `structure` | `message` | DBClusterParameterGroupName does not refer to an existing DB Cluster parameter group. |
| `DBSubnetGroupDoesNotCoverEnoughAZs` | `structure` | `message` | Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon Relational Database Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Relational Database Service Amazon Relational Database Service (Amazon RDS) is a web service that makes it easier to set up, operate, and scale a relational database in the cloud. It provides cost-efficient, resizeable capacity for an industry-standard relational database and manages common database administration tasks, freeing up developers to focus on what makes their applications and businesses unique. Amazon RDS gives you access to the capabilities of a MySQL, MariaDB, PostgreSQL, Microsoft SQL Server, Oracle, Db2, or Amazon Aurora database server. These capabilities mean that the code, applications, and tools you already use today with your existing databases work with Amazon RDS without modification. Amazon RDS automatically backs up your database and maintains the database software that powers your DB instance.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Relational Database Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Relational Database Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Relational Database Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Relational Database Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Modify`, `Delete`, `Create`, `Restore` operation families, including `DescribeAccountAttributes`, `DescribeBlueGreenDeployments`, `DescribeCertificates`, `DescribeDBClusterAutomatedBackups`, `ModifyActivityStream`, `ModifyCertificates`.

## Service Identity and Protocol

- AWS model slug: `rds`
- AWS SDK for Rust slug: `rds`
- Model version: `2014-10-31`
- Model file: `vendor/api-models-aws/models/rds/service/2014-10-31/rds-2014-10-31.json`
- SDK ID: `RDS`
- Endpoint prefix: `rds`
- ARN namespace: `rds`
- CloudFormation name: `RDS`
- CloudTrail event source: `rds.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (46), `Modify` (23), `Delete` (21), `Create` (20), `Restore` (6), `Copy` (5), `Remove` (5), `Start` (5).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddRoleToDBCluster`, `AddRoleToDBInstance`, `AddSourceIdentifierToSubscription`, `AddTagsToResource`, `CancelExportTask`, `CreateBlueGreenDeployment`, `CreateCustomDBEngineVersion`, `CreateDBCluster`, `CreateDBClusterEndpoint`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBInstanceReadReplica`, `CreateDBParameterGroup`, `CreateDBProxy`, `CreateDBProxyEndpoint`, `CreateDBSecurityGroup`, `CreateDBShardGroup`, `CreateDBSnapshot`, `CreateDBSubnetGroup`, ... (+74).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAttributes`, `DescribeBlueGreenDeployments`, `DescribeCertificates`, `DescribeDBClusterAutomatedBackups`, `DescribeDBClusterBacktracks`, `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstanceAutomatedBackups`, `DescribeDBInstances`, `DescribeDBLogFiles`, `DescribeDBMajorEngineVersions`, `DescribeDBParameterGroups`, `DescribeDBParameters`, `DescribeDBProxies`, `DescribeDBProxyEndpoints`, ... (+27).
- Pagination is modelled for 41 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelExportTask`, `DescribeExportTasks`, `StartActivityStream`, `StartDBCluster`, `StartDBInstance`, `StartDBInstanceAutomatedBackupsReplication`, `StartExportTask`, `StopActivityStream`, `StopDBCluster`, `StopDBInstance`, `StopDBInstanceAutomatedBackupsReplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 151 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `Glue`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.DBInstance.Modifying.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_DeleteInstance.html

Research outcomes:
- Many DB instance modifications can be applied immediately or deferred to the next maintenance window. Some changes require a manual reboot to take effect.
- ApplyImmediately can cause downtime depending on the modification. Deferred changes are visible as pending modifications until applied.
- Every DB instance has a weekly maintenance window. If none is specified, RDS assigns a random 30-minute window from a Region-specific block.
- The backup window and maintenance window cannot overlap.
- Multi-AZ operating system maintenance updates the standby, promotes it, then updates the old primary. Engine upgrades for many Multi-AZ instances can make both primary and standby unavailable.
- Backup storage consists of automated backups and manual snapshots per Region. Manual snapshots are independent of automated backups and survive DB instance deletion.
- Deletion protection must be disabled before deleting a DB instance.
- Deleting an instance can create a final snapshot, retain automated backups for the retention period, and automatically promote same-Region read replicas to standalone instances.

Parity implications:
- Model DB instances, pending modifications, maintenance windows, backup windows, snapshots, automated backups, read replicas, and deletion state separately.
- Modify APIs should distinguish immediate effects, deferred pending values, and reboot-required changes.
- Delete workflows need deletion-protection checks, final snapshot constraints, retained-backup choices, read-replica promotion, and long-running deleting status.

## Control-Plane / Data-Plane Coherence

- **Paired with `rdsdata` ( RDS Data API ).** RDS Data ( `winterbaume-rdsdata` ) executes SQL against Aurora Serverless v1/v2 and Aurora MySQL/PostgreSQL clusters that this control plane creates and manages the lifecycle of. `ExecuteStatement` and `BeginTransaction` carry a `resourceArn` that must point to an existing cluster with the Data API enabled; in real AWS the call fails with `BadRequestException` ( "Cluster not enabled for Data API" ) if it is not.
- **Current Winterbaume status: deliberately separate.** `winterbaume-rdsdata` is a "bring-your-own-result" mock — callers `enqueue_result` ahead of time and `ExecuteStatement` dequeues. It does not validate `resourceArn` against this crate's clusters and never executes real SQL. This is acceptable for unit testing where the test author seeds expected results, but it is not parity behaviour.
- **What needs to change ( low priority ):** if a future workflow needs the data plane to validate cluster existence and "Data API enabled" state, `winterbaume-rdsdata` should observe this crate's `db_clusters` and reject unknown ARNs. Real SQL execution is not in scope for the mock.

## Current Network Resource Stub Semantics

RDS currently stores database subnet groups and security group references inside RDS state.

- DB subnet group operations store subnet IDs and return RDS-local subnet group data.
- DB instances and clusters store DB subnet group names, classic DB security group names, and VPC security group IDs as request metadata.
- Security group ingress operations record RDS-local authorisations and do not mutate EC2 security groups.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeBlueGreenDeployments`, `DescribeCertificates`, `DescribeDBClusterAutomatedBackups`, `DescribeDBClusterBacktracks`, `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstanceAutomatedBackups`, `DescribeDBInstances`, `DescribeDBLogFiles`, `DescribeDBMajorEngineVersions`, `DescribeDBParameterGroups`, `DescribeDBParameters`, `DescribeDBProxies`, `DescribeDBProxyEndpoints`, `DescribeDBProxyTargetGroups`, `DescribeDBProxyTargets`, `DescribeDBRecommendations`, `DescribeDBSecurityGroups`, `DescribeDBShardGroups`, `DescribeDBSnapshotAttributes`, `DescribeDBSnapshotTenantDatabases`, `DescribeDBSnapshots`, `DescribeDBSubnetGroups`, `DescribeEngineDefaultClusterParameters`, `DescribeEngineDefaultParameters`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeExportTasks`, `DescribeGlobalClusters`, `DescribeIntegrations`, `DescribeOptionGroupOptions`, `DescribeOptionGroups`, `DescribeOrderableDBInstanceOptions`, ... (+6)
- Traits: `paginated` (40)
- Common required input members in this group: `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBParameterGroupFamily`, `DBParameterGroupName`, `DBProxyName`, `DBSnapshotIdentifier`, `Engine`, `EngineName`

### Modify

- Operations: `ModifyActivityStream`, `ModifyCertificates`, `ModifyCurrentDBClusterCapacity`, `ModifyCustomDBEngineVersion`, `ModifyDBCluster`, `ModifyDBClusterEndpoint`, `ModifyDBClusterParameterGroup`, `ModifyDBClusterSnapshotAttribute`, `ModifyDBInstance`, `ModifyDBParameterGroup`, `ModifyDBProxy`, `ModifyDBProxyEndpoint`, `ModifyDBProxyTargetGroup`, `ModifyDBRecommendation`, `ModifyDBShardGroup`, `ModifyDBSnapshot`, `ModifyDBSnapshotAttribute`, `ModifyDBSubnetGroup`, `ModifyEventSubscription`, `ModifyGlobalCluster`, `ModifyIntegration`, `ModifyOptionGroup`, `ModifyTenantDatabase`
- Common required input members in this group: `AttributeName`, `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBParameterGroupName`, `DBProxyEndpointName`, `DBProxyName`, `DBShardGroupIdentifier`, `DBSnapshotIdentifier`, `DBSubnetGroupName`, `Engine`, `EngineVersion`, `GlobalClusterIdentifier`, `IntegrationIdentifier`, `OptionGroupName`, `Parameters`, `RecommendationId`, `SubnetIds`, `SubscriptionName`, `TargetGroupName`, `TenantDBName`

### Delete

- Operations: `DeleteBlueGreenDeployment`, `DeleteCustomDBEngineVersion`, `DeleteDBCluster`, `DeleteDBClusterAutomatedBackup`, `DeleteDBClusterEndpoint`, `DeleteDBClusterParameterGroup`, `DeleteDBClusterSnapshot`, `DeleteDBInstance`, `DeleteDBInstanceAutomatedBackup`, `DeleteDBParameterGroup`, `DeleteDBProxy`, `DeleteDBProxyEndpoint`, `DeleteDBSecurityGroup`, `DeleteDBShardGroup`, `DeleteDBSnapshot`, `DeleteDBSubnetGroup`, `DeleteEventSubscription`, `DeleteGlobalCluster`, `DeleteIntegration`, `DeleteOptionGroup`, `DeleteTenantDatabase`
- Common required input members in this group: `BlueGreenDeploymentIdentifier`, `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBParameterGroupName`, `DBProxyEndpointName`, `DBProxyName`, `DBSecurityGroupName`, `DBShardGroupIdentifier`, `DBSnapshotIdentifier`, `DBSubnetGroupName`, `DbClusterResourceId`, `Engine`, `EngineVersion`, `GlobalClusterIdentifier`, `IntegrationIdentifier`, `OptionGroupName`, `SubscriptionName`, `TenantDBName`

### Create

- Operations: `CreateBlueGreenDeployment`, `CreateCustomDBEngineVersion`, `CreateDBCluster`, `CreateDBClusterEndpoint`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBInstanceReadReplica`, `CreateDBParameterGroup`, `CreateDBProxy`, `CreateDBProxyEndpoint`, `CreateDBSecurityGroup`, `CreateDBShardGroup`, `CreateDBSnapshot`, `CreateDBSubnetGroup`, `CreateEventSubscription`, `CreateGlobalCluster`, `CreateIntegration`, `CreateOptionGroup`, `CreateTenantDatabase`
- Common required input members in this group: `BlueGreenDeploymentName`, `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceClass`, `DBInstanceIdentifier`, `DBParameterGroupFamily`, `DBParameterGroupName`, `DBProxyEndpointName`, `DBProxyName`, `DBSecurityGroupDescription`, `DBSecurityGroupName`, `DBShardGroupIdentifier`, `DBSnapshotIdentifier`, `DBSubnetGroupDescription`, `DBSubnetGroupName`, `Description`, `EndpointType`, `Engine`, `EngineFamily`, `EngineName`, `EngineVersion`, `GlobalClusterIdentifier`, ... (+15)

### Restore

- Operations: `RestoreDBClusterFromS3`, `RestoreDBClusterFromSnapshot`, `RestoreDBClusterToPointInTime`, `RestoreDBInstanceFromDBSnapshot`, `RestoreDBInstanceFromS3`, `RestoreDBInstanceToPointInTime`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceClass`, `DBInstanceIdentifier`, `Engine`, `MasterUsername`, `S3BucketName`, `S3IngestionRoleArn`, `SnapshotIdentifier`, `SourceEngine`, `SourceEngineVersion`, `TargetDBInstanceIdentifier`

### Copy

- Operations: `CopyDBClusterParameterGroup`, `CopyDBClusterSnapshot`, `CopyDBParameterGroup`, `CopyDBSnapshot`, `CopyOptionGroup`
- Common required input members in this group: `SourceDBClusterParameterGroupIdentifier`, `SourceDBClusterSnapshotIdentifier`, `SourceDBParameterGroupIdentifier`, `SourceDBSnapshotIdentifier`, `SourceOptionGroupIdentifier`, `TargetDBClusterParameterGroupDescription`, `TargetDBClusterParameterGroupIdentifier`, `TargetDBClusterSnapshotIdentifier`, `TargetDBParameterGroupDescription`, `TargetDBParameterGroupIdentifier`, `TargetDBSnapshotIdentifier`, `TargetOptionGroupDescription`, `TargetOptionGroupIdentifier`

### Remove

- Operations: `RemoveFromGlobalCluster`, `RemoveRoleFromDBCluster`, `RemoveRoleFromDBInstance`, `RemoveSourceIdentifierFromSubscription`, `RemoveTagsFromResource`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceIdentifier`, `DbClusterIdentifier`, `FeatureName`, `GlobalClusterIdentifier`, `ResourceName`, `RoleArn`, `SourceIdentifier`, `SubscriptionName`, `TagKeys`

### Start

- Operations: `StartActivityStream`, `StartDBCluster`, `StartDBInstance`, `StartDBInstanceAutomatedBackupsReplication`, `StartExportTask`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceIdentifier`, `ExportTaskIdentifier`, `IamRoleArn`, `KmsKeyId`, `Mode`, `ResourceArn`, `S3BucketName`, `SourceArn`, `SourceDBInstanceArn`

### Add

- Operations: `AddRoleToDBCluster`, `AddRoleToDBInstance`, `AddSourceIdentifierToSubscription`, `AddTagsToResource`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceIdentifier`, `FeatureName`, `ResourceName`, `RoleArn`, `SourceIdentifier`, `SubscriptionName`, `Tags`

### Stop

- Operations: `StopActivityStream`, `StopDBCluster`, `StopDBInstance`, `StopDBInstanceAutomatedBackupsReplication`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceIdentifier`, `ResourceArn`, `SourceDBInstanceArn`

### Reboot

- Operations: `RebootDBCluster`, `RebootDBInstance`, `RebootDBShardGroup`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceIdentifier`, `DBShardGroupIdentifier`

### Switchover

- Operations: `SwitchoverBlueGreenDeployment`, `SwitchoverGlobalCluster`, `SwitchoverReadReplica`
- Common required input members in this group: `BlueGreenDeploymentIdentifier`, `DBInstanceIdentifier`, `GlobalClusterIdentifier`, `TargetDbClusterIdentifier`

### Failover

- Operations: `FailoverDBCluster`, `FailoverGlobalCluster`
- Common required input members in this group: `DBClusterIdentifier`, `GlobalClusterIdentifier`, `TargetDbClusterIdentifier`

### Promote

- Operations: `PromoteReadReplica`, `PromoteReadReplicaDBCluster`
- Common required input members in this group: `DBClusterIdentifier`, `DBInstanceIdentifier`

### Reset

- Operations: `ResetDBClusterParameterGroup`, `ResetDBParameterGroup`
- Common required input members in this group: `DBClusterParameterGroupName`, `DBParameterGroupName`

### Apply

- Operations: `ApplyPendingMaintenanceAction`
- Common required input members in this group: `ApplyAction`, `OptInType`, `ResourceIdentifier`

### Authorize

- Operations: `AuthorizeDBSecurityGroupIngress`
- Common required input members in this group: `DBSecurityGroupName`

### Backtrack

- Operations: `BacktrackDBCluster`
- Common required input members in this group: `BacktrackTo`, `DBClusterIdentifier`

### Cancel

- Operations: `CancelExportTask`
- Common required input members in this group: `ExportTaskIdentifier`

### Deregister

- Operations: `DeregisterDBProxyTargets`
- Common required input members in this group: `DBProxyName`

### Disable

- Operations: `DisableHttpEndpoint`
- Common required input members in this group: `ResourceArn`

### Download

- Operations: `DownloadDBLogFilePortion`
- Traits: `paginated` (1)
- Common required input members in this group: `DBInstanceIdentifier`, `LogFileName`

### Enable

- Operations: `EnableHttpEndpoint`
- Common required input members in this group: `ResourceArn`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `ResourceName`

### Purchase

- Operations: `PurchaseReservedDBInstancesOffering`
- Common required input members in this group: `ReservedDBInstancesOfferingId`

### Register

- Operations: `RegisterDBProxyTargets`
- Common required input members in this group: `DBProxyName`

### Revoke

- Operations: `RevokeDBSecurityGroupIngress`
- Common required input members in this group: `DBSecurityGroupName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddRoleToDBCluster` | - | - | `DBClusterIdentifier`, `RoleArn` | - | `Unit` | `DBClusterNotFoundFault`, `DBClusterRoleAlreadyExistsFault`, `DBClusterRoleQuotaExceededFault`, `InvalidDBClusterStateFault` | Associates an Identity and Access Management (IAM) role with a DB cluster. |
| `AddRoleToDBInstance` | - | - | `DBInstanceIdentifier`, `FeatureName`, `RoleArn` | - | `Unit` | `DBInstanceNotFoundFault`, `DBInstanceRoleAlreadyExistsFault`, `DBInstanceRoleQuotaExceededFault`, `InvalidDBInstanceStateFault` | Associates an Amazon Web Services Identity and Access Management (IAM) role with a DB instance. To add a role to a DB instance, the status of the DB instance must be `available`. |
| `AddSourceIdentifierToSubscription` | - | - | `SourceIdentifier`, `SubscriptionName` | - | `AddSourceIdentifierToSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Adds a source identifier to an existing RDS event notification subscription. |
| `AddTagsToResource` | - | - | `ResourceName`, `Tags` | - | `Unit` | `BlueGreenDeploymentNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBShardGroupNotFoundFault`, `DBSnapshotNotFoundFault`, ... (+6) | Adds metadata tags to an Amazon RDS resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon RDS resources, or used in a Condition statement in an IAM policy for Amazon RDS. |
| `ApplyPendingMaintenanceAction` | - | - | `ApplyAction`, `OptInType`, `ResourceIdentifier` | - | `ApplyPendingMaintenanceActionResult` | `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Applies a pending maintenance action to a resource (for example, to a DB instance). |
| `AuthorizeDBSecurityGroupIngress` | - | - | `DBSecurityGroupName` | - | `AuthorizeDBSecurityGroupIngressResult` | `AuthorizationAlreadyExistsFault`, `AuthorizationQuotaExceededFault`, `DBSecurityGroupNotFoundFault`, `InvalidDBSecurityGroupStateFault` | Enables ingress to a DBSecurityGroup using one of two forms of authorization. First, EC2 or VPC security groups can be added to the DBSecurityGroup if the application using the database is running on EC2 or VPC instances. |
| `BacktrackDBCluster` | - | - | `BacktrackTo`, `DBClusterIdentifier` | - | `DBClusterBacktrack` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault` | Backtracks a DB cluster to a specific time, without creating a new DB cluster. For more information on backtracking, see Backtracking an Aurora DB Cluster in the Amazon Aurora User Guide . |
| `CancelExportTask` | - | - | `ExportTaskIdentifier` | - | `ExportTask` | `ExportTaskNotFoundFault`, `InvalidExportTaskStateFault` | Cancels an export task in progress that is exporting a snapshot or cluster to Amazon S3. Any data that has already been written to the S3 bucket isn't removed. |
| `CopyDBClusterParameterGroup` | - | - | `SourceDBClusterParameterGroupIdentifier`, `TargetDBClusterParameterGroupDescription`, `TargetDBClusterParameterGroupIdentifier` | - | `CopyDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified DB cluster parameter group. You can't copy a default DB cluster parameter group. |
| `CopyDBClusterSnapshot` | - | - | `SourceDBClusterSnapshotIdentifier`, `TargetDBClusterSnapshotIdentifier` | - | `CopyDBClusterSnapshotResult` | `DBClusterSnapshotAlreadyExistsFault`, `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Copies a snapshot of a DB cluster. To copy a DB cluster snapshot from a shared manual DB cluster snapshot, `SourceDBClusterSnapshotIdentifier` must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot. |
| `CopyDBParameterGroup` | - | - | `SourceDBParameterGroupIdentifier`, `TargetDBParameterGroupDescription`, `TargetDBParameterGroupIdentifier` | - | `CopyDBParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified DB parameter group. You can't copy a default DB parameter group. |
| `CopyDBSnapshot` | - | - | `SourceDBSnapshotIdentifier`, `TargetDBSnapshotIdentifier` | - | `CopyDBSnapshotResult` | `CustomAvailabilityZoneNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Copies the specified DB snapshot. The source DB snapshot must be in the `available` state. |
| `CopyOptionGroup` | - | - | `SourceOptionGroupIdentifier`, `TargetOptionGroupDescription`, `TargetOptionGroupIdentifier` | - | `CopyOptionGroupResult` | `OptionGroupAlreadyExistsFault`, `OptionGroupNotFoundFault`, `OptionGroupQuotaExceededFault` | Copies the specified option group. |
| `CreateBlueGreenDeployment` | - | - | `BlueGreenDeploymentName`, `Source` | - | `CreateBlueGreenDeploymentResponse` | `BlueGreenDeploymentAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `InstanceQuotaExceededFault`, `InvalidDBClusterStateFault`, ... (+4) | Creates a blue/green deployment. A blue/green deployment creates a staging environment that copies the production environment. |
| `CreateCustomDBEngineVersion` | - | - | `Engine`, `EngineVersion` | - | `DBEngineVersion` | `CreateCustomDBEngineVersionFault`, `CustomDBEngineVersionAlreadyExistsFault`, `CustomDBEngineVersionNotFoundFault`, `CustomDBEngineVersionQuotaExceededFault`, `Ec2ImagePropertiesNotSupportedFault`, `InvalidCustomDBEngineVersionStateFault`, `KMSKeyNotAccessibleFault` | Creates a custom DB engine version (CEV). |
| `CreateDBCluster` | - | - | `DBClusterIdentifier`, `Engine` | - | `CreateDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, ... (+16) | Creates a new Amazon Aurora DB cluster or Multi-AZ DB cluster. If you create an Aurora DB cluster, the request creates an empty cluster. |
| `CreateDBClusterEndpoint` | - | - | `DBClusterEndpointIdentifier`, `DBClusterIdentifier`, `EndpointType` | - | `DBClusterEndpoint` | `DBClusterEndpointAlreadyExistsFault`, `DBClusterEndpointQuotaExceededFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Creates a new custom endpoint and associates it with an Amazon Aurora DB cluster. This action applies only to Aurora DB clusters. |
| `CreateDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName`, `DBParameterGroupFamily`, `Description` | - | `CreateDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new DB cluster parameter group. Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster. |
| `CreateDBClusterSnapshot` | - | - | `DBClusterIdentifier`, `DBClusterSnapshotIdentifier` | - | `CreateDBClusterSnapshotResult` | `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `SnapshotQuotaExceededFault` | Creates a snapshot of a DB cluster. For more information on Amazon Aurora, see What is Amazon Aurora? |
| `CreateDBInstance` | - | - | `DBInstanceClass`, `DBInstanceIdentifier`, `Engine` | - | `CreateDBInstanceResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, ... (+15) | Creates a new DB instance. The new DB instance can be an RDS DB instance, or it can be a DB instance in an Aurora DB cluster. |
| `CreateDBInstanceReadReplica` | - | - | `DBInstanceIdentifier` | - | `CreateDBInstanceReadReplicaResult` | `CertificateNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotAllowedFault`, ... (+17) | Creates a new DB instance that acts as a read replica for an existing source DB instance or Multi-AZ DB cluster. You can create a read replica for a DB instance running Db2, MariaDB, MySQL, Oracle, PostgreSQL, or SQL Server. |
| `CreateDBParameterGroup` | - | - | `DBParameterGroupFamily`, `DBParameterGroupName`, `Description` | - | `CreateDBParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new DB parameter group. A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. |
| `CreateDBProxy` | - | - | `DBProxyName`, `EngineFamily`, `RoleArn`, `VpcSubnetIds` | - | `CreateDBProxyResponse` | `DBProxyAlreadyExistsFault`, `DBProxyQuotaExceededFault`, `InvalidSubnet` | Creates a new DB proxy. |
| `CreateDBProxyEndpoint` | - | - | `DBProxyEndpointName`, `DBProxyName`, `VpcSubnetIds` | - | `CreateDBProxyEndpointResponse` | `DBProxyEndpointAlreadyExistsFault`, `DBProxyEndpointQuotaExceededFault`, `DBProxyNotFoundFault`, `InvalidDBProxyStateFault`, `InvalidSubnet` | Creates a `DBProxyEndpoint`. Only applies to proxies that are associated with Aurora DB clusters. |
| `CreateDBSecurityGroup` | - | - | `DBSecurityGroupDescription`, `DBSecurityGroupName` | - | `CreateDBSecurityGroupResult` | `DBSecurityGroupAlreadyExistsFault`, `DBSecurityGroupNotSupportedFault`, `DBSecurityGroupQuotaExceededFault` | Creates a new DB security group. DB security groups control access to a DB instance. |
| `CreateDBShardGroup` | - | - | `DBClusterIdentifier`, `DBShardGroupIdentifier`, `MaxACU` | - | `DBShardGroup` | `DBClusterNotFoundFault`, `DBShardGroupAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidVPCNetworkStateFault`, `MaxDBShardGroupLimitReached`, `NetworkTypeNotSupported`, `UnsupportedDBEngineVersionFault` | Creates a new DB shard group for Aurora Limitless Database. You must enable Aurora Limitless Database to create a DB shard group. |
| `CreateDBSnapshot` | - | - | `DBInstanceIdentifier`, `DBSnapshotIdentifier` | - | `CreateDBSnapshotResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBInstanceStateFault`, `SnapshotQuotaExceededFault` | Creates a snapshot of a DB instance. The source DB instance must be in the `available` or `storage-optimization` state. |
| `CreateDBSubnetGroup` | - | - | `DBSubnetGroupDescription`, `DBSubnetGroupName`, `SubnetIds` | - | `CreateDBSubnetGroupResult` | `DBSubnetGroupAlreadyExistsFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupQuotaExceededFault`, `DBSubnetQuotaExceededFault`, `InvalidSubnet` | Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the Amazon Web Services Region. |
| `CreateEventSubscription` | - | - | `SnsTopicArn`, `SubscriptionName` | - | `CreateEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SourceNotFoundFault`, `SubscriptionAlreadyExistFault`, `SubscriptionCategoryNotFoundFault` | Creates an RDS event notification subscription. This operation requires a topic Amazon Resource Name (ARN) created by either the RDS console, the SNS console, or the SNS API. |
| `CreateGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `CreateGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterAlreadyExistsFault`, `GlobalClusterQuotaExceededFault`, `InvalidDBClusterStateFault`, `InvalidDBShardGroupStateFault`, `ResourceNotFoundFault` | Creates an Aurora global database spread across multiple Amazon Web Services Regions. The global database contains a single primary cluster with read-write capability, and a read-only secondary cluster that receives data from the primary cluster through... |
| `CreateIntegration` | - | - | `IntegrationName`, `SourceArn`, `TargetArn` | - | `Integration` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `IntegrationAlreadyExistsFault`, `IntegrationConflictOperationFault`, `IntegrationQuotaExceededFault`, `KMSKeyNotAccessibleFault` | Creates a zero-ETL integration with Amazon Redshift. |
| `CreateOptionGroup` | - | - | `EngineName`, `MajorEngineVersion`, `OptionGroupDescription`, `OptionGroupName` | - | `CreateOptionGroupResult` | `OptionGroupAlreadyExistsFault`, `OptionGroupQuotaExceededFault` | Creates a new option group. You can create up to 20 option groups. |
| `CreateTenantDatabase` | - | - | `DBInstanceIdentifier`, `MasterUsername`, `TenantDBName` | - | `CreateTenantDatabaseResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `TenantDatabaseAlreadyExistsFault`, `TenantDatabaseQuotaExceededFault` | Creates a tenant database in a DB instance that uses the multi-tenant configuration. Only RDS for Oracle container database (CDB) instances are supported. |
| `DeleteBlueGreenDeployment` | - | - | `BlueGreenDeploymentIdentifier` | - | `DeleteBlueGreenDeploymentResponse` | `BlueGreenDeploymentNotFoundFault`, `InvalidBlueGreenDeploymentStateFault` | Deletes a blue/green deployment. For more information, see Using Amazon RDS Blue/Green Deployments for database updates in the Amazon RDS User Guide and Using Amazon RDS Blue/Green Deployments for database updates in the Amazon Aurora User Guide . |
| `DeleteCustomDBEngineVersion` | - | - | `Engine`, `EngineVersion` | - | `DBEngineVersion` | `CustomDBEngineVersionNotFoundFault`, `InvalidCustomDBEngineVersionStateFault` | Deletes a custom engine version. To run this command, make sure you meet the following prerequisites: The CEV must not be the default for RDS Custom. |
| `DeleteDBCluster` | - | - | `DBClusterIdentifier` | - | `DeleteDBClusterResult` | `DBClusterAutomatedBackupQuotaExceededFault`, `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered. |
| `DeleteDBClusterAutomatedBackup` | - | - | `DbClusterResourceId` | - | `DeleteDBClusterAutomatedBackupResult` | `DBClusterAutomatedBackupNotFoundFault`, `InvalidDBClusterAutomatedBackupStateFault` | Deletes automated backups using the `DbClusterResourceId` value of the source DB cluster or the Amazon Resource Name (ARN) of the automated backups. |
| `DeleteDBClusterEndpoint` | - | - | `DBClusterEndpointIdentifier` | - | `DBClusterEndpoint` | `DBClusterEndpointNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault` | Deletes a custom endpoint and removes it from an Amazon Aurora DB cluster. This action only applies to Aurora DB clusters. |
| `DeleteDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters. |
| `DeleteDBClusterSnapshot` | - | - | `DBClusterSnapshotIdentifier` | - | `DeleteDBClusterSnapshotResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault` | Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated. |
| `DeleteDBInstance` | - | - | `DBInstanceIdentifier` | - | `DeleteDBInstanceResult` | `DBInstanceAutomatedBackupQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. |
| `DeleteDBInstanceAutomatedBackup` | - | - | - | - | `DeleteDBInstanceAutomatedBackupResult` | `DBInstanceAutomatedBackupNotFoundFault`, `InvalidDBInstanceAutomatedBackupStateFault` | Deletes automated backups using the `DbiResourceId` value of the source DB instance or the Amazon Resource Name (ARN) of the automated backups. |
| `DeleteDBParameterGroup` | - | - | `DBParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified DB parameter group. The DB parameter group to be deleted can't be associated with any DB instances. |
| `DeleteDBProxy` | - | - | `DBProxyName` | - | `DeleteDBProxyResponse` | `DBProxyNotFoundFault`, `InvalidDBProxyStateFault` | Deletes an existing DB proxy. |
| `DeleteDBProxyEndpoint` | - | - | `DBProxyEndpointName` | - | `DeleteDBProxyEndpointResponse` | `DBProxyEndpointNotFoundFault`, `InvalidDBProxyEndpointStateFault` | Deletes a `DBProxyEndpoint`. Doing so removes the ability to access the DB proxy using the endpoint that you defined. |
| `DeleteDBSecurityGroup` | - | - | `DBSecurityGroupName` | - | `Unit` | `DBSecurityGroupNotFoundFault`, `InvalidDBSecurityGroupStateFault` | Deletes a DB security group. The specified DB security group must not be associated with any DB instances. |
| `DeleteDBShardGroup` | - | - | `DBShardGroupIdentifier` | - | `DBShardGroup` | `DBShardGroupNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBShardGroupStateFault` | Deletes an Aurora Limitless Database DB shard group. |
| `DeleteDBSnapshot` | - | - | `DBSnapshotIdentifier` | - | `DeleteDBSnapshotResult` | `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault` | Deletes a DB snapshot. If the snapshot is being copied, the copy operation is terminated. |
| `DeleteDBSubnetGroup` | - | - | `DBSubnetGroupName` | - | `Unit` | `DBSubnetGroupNotFoundFault`, `InvalidDBSubnetGroupStateFault`, `InvalidDBSubnetStateFault` | Deletes a DB subnet group. The specified database subnet group must not be associated with any DB instances. |
| `DeleteEventSubscription` | - | - | `SubscriptionName` | - | `DeleteEventSubscriptionResult` | `InvalidEventSubscriptionStateFault`, `SubscriptionNotFoundFault` | Deletes an RDS event notification subscription. |
| `DeleteGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `DeleteGlobalClusterResult` | `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Deletes a global database cluster. The primary and secondary clusters must already be detached or destroyed first. |
| `DeleteIntegration` | - | - | `IntegrationIdentifier` | - | `Integration` | `IntegrationConflictOperationFault`, `IntegrationNotFoundFault`, `InvalidIntegrationStateFault` | Deletes a zero-ETL integration with Amazon Redshift. |
| `DeleteOptionGroup` | - | - | `OptionGroupName` | - | `Unit` | `InvalidOptionGroupStateFault`, `OptionGroupNotFoundFault` | Deletes an existing option group. |
| `DeleteTenantDatabase` | - | - | `DBInstanceIdentifier`, `TenantDBName` | - | `DeleteTenantDatabaseResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBInstanceStateFault`, `TenantDatabaseNotFoundFault` | Deletes a tenant database from your DB instance. This command only applies to RDS for Oracle container database (CDB) instances. |
| `DeregisterDBProxyTargets` | - | - | `DBProxyName` | - | `DeregisterDBProxyTargetsResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBProxyTargetNotFoundFault`, `InvalidDBProxyStateFault` | Remove the association between one or more `DBProxyTarget` data structures and a `DBProxyTargetGroup`. |
| `DescribeAccountAttributes` | - | - | - | - | `AccountAttributesMessage` | - | Lists all of the attributes for a customer account. The attributes include Amazon RDS quotas for the account, such as the number of DB instances allowed. |
| `DescribeBlueGreenDeployments` | - | `paginated` | - | - | `DescribeBlueGreenDeploymentsResponse` | `BlueGreenDeploymentNotFoundFault` | Describes one or more blue/green deployments. For more information, see Using Amazon RDS Blue/Green Deployments for database updates in the Amazon RDS User Guide and Using Amazon RDS Blue/Green Deployments for database updates in the Amazon Aurora User Guide . |
| `DescribeCertificates` | - | `paginated` | - | - | `CertificateMessage` | `CertificateNotFoundFault` | Lists the set of certificate authority (CA) certificates provided by Amazon RDS for this Amazon Web Services account. For more information, see Using SSL/TLS to encrypt a connection to a DB instance in the Amazon RDS User Guide and Using SSL/TLS to encrypt a... |
| `DescribeDBClusterAutomatedBackups` | - | `paginated` | - | - | `DBClusterAutomatedBackupMessage` | `DBClusterAutomatedBackupNotFoundFault` | Displays backups for both current and deleted DB clusters. For example, use this operation to find details about automated backups for previously deleted clusters. |
| `DescribeDBClusterBacktracks` | - | `paginated` | `DBClusterIdentifier` | - | `DBClusterBacktrackMessage` | `DBClusterBacktrackNotFoundFault`, `DBClusterNotFoundFault` | Returns information about backtracks for a DB cluster. For more information on Amazon Aurora, see What is Amazon Aurora? |
| `DescribeDBClusterEndpoints` | - | `paginated` | - | - | `DBClusterEndpointMessage` | `DBClusterNotFoundFault` | Returns information about endpoints for an Amazon Aurora DB cluster. This action only applies to Aurora DB clusters. |
| `DescribeDBClusterParameterGroups` | - | `paginated` | - | - | `DBClusterParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of `DBClusterParameterGroup` descriptions. If a `DBClusterParameterGroupName` parameter is specified, the list will contain only the description of the specified DB cluster parameter group. |
| `DescribeDBClusterParameters` | - | `paginated` | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular DB cluster parameter group. For more information on Amazon Aurora, see What is Amazon Aurora? |
| `DescribeDBClusterSnapshotAttributes` | - | - | `DBClusterSnapshotIdentifier` | - | `DescribeDBClusterSnapshotAttributesResult` | `DBClusterSnapshotNotFoundFault` | Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot. When sharing snapshots with other Amazon Web Services accounts, `DescribeDBClusterSnapshotAttributes` returns the `restore` attribute and a list of IDs for the... |
| `DescribeDBClusterSnapshots` | - | `paginated` | - | - | `DBClusterSnapshotMessage` | `DBClusterSnapshotNotFoundFault` | Returns information about DB cluster snapshots. This API action supports pagination. |
| `DescribeDBClusters` | - | `paginated` | - | - | `DBClusterMessage` | `DBClusterNotFoundFault` | Describes existing Amazon Aurora DB clusters and Multi-AZ DB clusters. This API supports pagination. |
| `DescribeDBEngineVersions` | - | `paginated` | - | - | `DBEngineVersionMessage` | - | Describes the properties of specific versions of DB engines. |
| `DescribeDBInstanceAutomatedBackups` | - | `paginated` | - | - | `DBInstanceAutomatedBackupMessage` | `DBInstanceAutomatedBackupNotFoundFault` | Displays backups for both current and deleted instances. For example, use this operation to find details about automated backups for previously deleted instances. |
| `DescribeDBInstances` | - | `paginated` | - | - | `DBInstanceMessage` | `DBInstanceNotFoundFault` | Describes provisioned RDS instances. This API supports pagination. |
| `DescribeDBLogFiles` | - | `paginated` | `DBInstanceIdentifier` | - | `DescribeDBLogFilesResponse` | `DBInstanceNotFoundFault`, `DBInstanceNotReadyFault` | Returns a list of DB log files for the DB instance. This command doesn't apply to RDS Custom. |
| `DescribeDBMajorEngineVersions` | - | `paginated` | - | - | `DescribeDBMajorEngineVersionsResponse` | - | Describes the properties of specific major versions of DB engines. |
| `DescribeDBParameterGroups` | - | `paginated` | - | - | `DBParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of `DBParameterGroup` descriptions. If a `DBParameterGroupName` is specified, the list will contain only the description of the specified DB parameter group. |
| `DescribeDBParameters` | - | `paginated` | `DBParameterGroupName` | - | `DBParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular DB parameter group. |
| `DescribeDBProxies` | - | `paginated` | - | - | `DescribeDBProxiesResponse` | `DBProxyNotFoundFault` | Returns information about DB proxies. |
| `DescribeDBProxyEndpoints` | - | `paginated` | - | - | `DescribeDBProxyEndpointsResponse` | `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault` | Returns information about DB proxy endpoints. |
| `DescribeDBProxyTargetGroups` | - | `paginated` | `DBProxyName` | - | `DescribeDBProxyTargetGroupsResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `InvalidDBProxyStateFault` | Returns information about DB proxy target groups, represented by `DBProxyTargetGroup` data structures. |
| `DescribeDBProxyTargets` | - | `paginated` | `DBProxyName` | - | `DescribeDBProxyTargetsResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBProxyTargetNotFoundFault`, `InvalidDBProxyStateFault` | Returns information about `DBProxyTarget` objects. This API supports pagination. |
| `DescribeDBRecommendations` | - | `paginated` | - | - | `DBRecommendationsMessage` | - | Describes the recommendations to resolve the issues for your DB instances, DB clusters, and DB parameter groups. |
| `DescribeDBSecurityGroups` | - | `paginated` | - | - | `DBSecurityGroupMessage` | `DBSecurityGroupNotFoundFault` | Returns a list of `DBSecurityGroup` descriptions. If a `DBSecurityGroupName` is specified, the list will contain only the descriptions of the specified DB security group. |
| `DescribeDBShardGroups` | - | - | - | - | `DescribeDBShardGroupsResponse` | `DBClusterNotFoundFault`, `DBShardGroupNotFoundFault` | Describes existing Aurora Limitless Database DB shard groups. |
| `DescribeDBSnapshotAttributes` | - | - | `DBSnapshotIdentifier` | - | `DescribeDBSnapshotAttributesResult` | `DBSnapshotNotFoundFault` | Returns a list of DB snapshot attribute names and values for a manual DB snapshot. When sharing snapshots with other Amazon Web Services accounts, `DescribeDBSnapshotAttributes` returns the `restore` attribute and a list of IDs for the Amazon Web Services... |
| `DescribeDBSnapshotTenantDatabases` | - | `paginated` | - | - | `DBSnapshotTenantDatabasesMessage` | `DBSnapshotNotFoundFault` | Describes the tenant databases that exist in a DB snapshot. This command only applies to RDS for Oracle DB instances in the multi-tenant configuration. |
| `DescribeDBSnapshots` | - | `paginated` | - | - | `DBSnapshotMessage` | `DBSnapshotNotFoundFault` | Returns information about DB snapshots. This API action supports pagination. |
| `DescribeDBSubnetGroups` | - | `paginated` | - | - | `DBSubnetGroupMessage` | `DBSubnetGroupNotFoundFault` | Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup. |
| `DescribeEngineDefaultClusterParameters` | - | `paginated` | `DBParameterGroupFamily` | - | `DescribeEngineDefaultClusterParametersResult` | - | Returns the default engine and system parameter information for the cluster database engine. For more information on Amazon Aurora, see What is Amazon Aurora? |
| `DescribeEngineDefaultParameters` | - | `paginated` | `DBParameterGroupFamily` | - | `DescribeEngineDefaultParametersResult` | - | Returns the default engine and system parameter information for the specified database engine. |
| `DescribeEventCategories` | - | - | - | - | `EventCategoriesMessage` | - | Displays a list of categories for all event source types, or, if specified, for a specified source type. You can also see this list in the "Amazon RDS event categories and event messages" section of the Amazon RDS User Guide or the Amazon Aurora User Guide . |
| `DescribeEventSubscriptions` | - | `paginated` | - | - | `EventSubscriptionsMessage` | `SubscriptionNotFoundFault` | Lists all the subscription descriptions for a customer account. The description for a subscription includes `SubscriptionName`, `SNSTopicARN`, `CustomerID`, `SourceType`, `SourceID`, `CreationTime`, and `Status`. |
| `DescribeEvents` | - | `paginated` | - | - | `EventsMessage` | - | Returns events related to DB instances, DB clusters, DB parameter groups, DB security groups, DB snapshots, DB cluster snapshots, and RDS Proxies for the past 14 days. Events specific to a particular DB instance, DB cluster, DB parameter group, DB security... |
| `DescribeExportTasks` | - | `paginated` | - | - | `ExportTasksMessage` | `ExportTaskNotFoundFault` | Returns information about a snapshot or cluster export to Amazon S3. This API operation supports pagination. |
| `DescribeGlobalClusters` | - | `paginated` | - | - | `GlobalClustersMessage` | `GlobalClusterNotFoundFault` | Returns information about Aurora global database clusters. This API supports pagination. |
| `DescribeIntegrations` | - | `paginated` | - | - | `DescribeIntegrationsResponse` | `IntegrationNotFoundFault` | Describe one or more zero-ETL integrations with Amazon Redshift. |
| `DescribeOptionGroupOptions` | - | `paginated` | `EngineName` | - | `OptionGroupOptionsMessage` | - | Describes all available options for the specified engine. |
| `DescribeOptionGroups` | - | `paginated` | - | - | `OptionGroups` | `OptionGroupNotFoundFault` | Describes the available option groups. |
| `DescribeOrderableDBInstanceOptions` | - | `paginated` | `Engine` | - | `OrderableDBInstanceOptionsMessage` | - | Describes the orderable DB instance options for a specified DB engine. |
| `DescribePendingMaintenanceActions` | - | `paginated` | - | - | `PendingMaintenanceActionsMessage` | `ResourceNotFoundFault` | Returns a list of resources (for example, DB instances) that have at least one pending maintenance action. This API follows an eventual consistency model. |
| `DescribeReservedDBInstances` | - | `paginated` | - | - | `ReservedDBInstanceMessage` | `ReservedDBInstanceNotFoundFault` | Returns information about reserved DB instances for this account, or about a specified reserved DB instance. |
| `DescribeReservedDBInstancesOfferings` | - | `paginated` | - | - | `ReservedDBInstancesOfferingMessage` | `ReservedDBInstancesOfferingNotFoundFault` | Lists available reserved DB instance offerings. |
| `DescribeSourceRegions` | - | `paginated` | - | - | `SourceRegionMessage` | - | Returns a list of the source Amazon Web Services Regions where the current Amazon Web Services Region can create a read replica, copy a DB snapshot from, or replicate automated backups from. Use this operation to determine whether cross-Region features are... |
| `DescribeTenantDatabases` | - | `paginated` | - | - | `TenantDatabasesMessage` | `DBInstanceNotFoundFault` | Describes the tenant databases in a DB instance that uses the multi-tenant configuration. Only RDS for Oracle CDB instances are supported. |
| `DescribeValidDBInstanceModifications` | - | - | `DBInstanceIdentifier` | - | `DescribeValidDBInstanceModificationsResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | You can call `DescribeValidDBInstanceModifications` to learn what modifications you can make to your DB instance. You can use this information when you call `ModifyDBInstance`. |
| `DisableHttpEndpoint` | - | - | `ResourceArn` | - | `DisableHttpEndpointResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Disables the HTTP endpoint for the specified DB cluster. Disabling this endpoint disables RDS Data API. |
| `DownloadDBLogFilePortion` | - | `paginated` | `DBInstanceIdentifier`, `LogFileName` | - | `DownloadDBLogFilePortionDetails` | `DBInstanceNotFoundFault`, `DBInstanceNotReadyFault`, `DBLogFileNotFoundFault` | Downloads all or a portion of the specified log file, up to 1 MB in size. This command doesn't apply to RDS Custom. |
| `EnableHttpEndpoint` | - | - | `ResourceArn` | - | `EnableHttpEndpointResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Enables the HTTP endpoint for the DB cluster. By default, the HTTP endpoint isn't enabled. |
| `FailoverDBCluster` | - | - | `DBClusterIdentifier` | - | `FailoverDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Forces a failover for a DB cluster. For an Aurora DB cluster, failover for a DB cluster promotes one of the Aurora Replicas (read-only instances) in the DB cluster to be the primary DB instance (the cluster writer). |
| `FailoverGlobalCluster` | - | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `FailoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Promotes the specified secondary DB cluster to be the primary DB cluster in the global database cluster to fail over or switch over a global database. Switchover operations were previously called "managed planned failovers." Although this operation can be... |
| `ListTagsForResource` | - | - | `ResourceName` | - | `TagListMessage` | `BlueGreenDeploymentNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBShardGroupNotFoundFault`, `DBSnapshotNotFoundFault`, ... (+3) | Lists all tags on an Amazon RDS resource. For an overview on tagging an Amazon RDS resource, see Tagging Amazon RDS Resources in the Amazon RDS User Guide or Tagging Amazon Aurora and Amazon RDS Resources in the Amazon Aurora User Guide . |
| `ModifyActivityStream` | - | - | - | - | `ModifyActivityStreamResponse` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Changes the audit policy state of a database activity stream to either locked (default) or unlocked. A locked policy is read-only, whereas an unlocked policy is read/write. |
| `ModifyCertificates` | - | - | - | - | `ModifyCertificatesResult` | `CertificateNotFoundFault` | Override the system-default Secure Sockets Layer/Transport Layer Security (SSL/TLS) certificate for Amazon RDS for new DB instances, or remove the override. By using this operation, you can specify an RDS-approved SSL/TLS certificate for new DB instances that... |
| `ModifyCurrentDBClusterCapacity` | - | - | `DBClusterIdentifier` | - | `DBClusterCapacityInfo` | `DBClusterNotFoundFault`, `InvalidDBClusterCapacityFault`, `InvalidDBClusterStateFault` | Set the capacity of an Aurora Serverless v1 DB cluster to a specific value. Aurora Serverless v1 scales seamlessly based on the workload on the DB cluster. |
| `ModifyCustomDBEngineVersion` | - | - | `Engine`, `EngineVersion` | - | `DBEngineVersion` | `CustomDBEngineVersionNotFoundFault`, `InvalidCustomDBEngineVersionStateFault` | Modifies the status of a custom engine version (CEV). You can find CEVs to modify by calling `DescribeDBEngineVersions`. |
| `ModifyDBCluster` | - | - | `DBClusterIdentifier` | - | `ModifyDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InvalidDBClusterStateFault`, ... (+13) | Modifies the settings of an Amazon Aurora DB cluster or a Multi-AZ DB cluster. You can change one or more settings by specifying these parameters and the new values in the request. |
| `ModifyDBClusterEndpoint` | - | - | `DBClusterEndpointIdentifier` | - | `DBClusterEndpoint` | `DBClusterEndpointNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Modifies the properties of an endpoint in an Amazon Aurora DB cluster. This operation only applies to Aurora DB clusters. |
| `ModifyDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName`, `Parameters` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: `ParameterName`, `ParameterValue`, and `ApplyMethod`. |
| `ModifyDBClusterSnapshotAttribute` | - | - | `AttributeName`, `DBClusterSnapshotIdentifier` | - | `ModifyDBClusterSnapshotAttributeResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `SharedSnapshotQuotaExceededFault` | Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot. To share a manual DB cluster snapshot with other Amazon Web Services accounts, specify `restore` as the `AttributeName` and use the `ValuesToAdd` parameter... |
| `ModifyDBInstance` | - | - | `DBInstanceIdentifier` | - | `ModifyDBInstanceResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBUpgradeDependencyFailureFault`, ... (+14) | Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. |
| `ModifyDBParameterGroup` | - | - | `DBParameterGroupName`, `Parameters` | - | `DBParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: `ParameterName`, `ParameterValue`, and `ApplyMethod`. |
| `ModifyDBProxy` | - | - | `DBProxyName` | - | `ModifyDBProxyResponse` | `DBProxyAlreadyExistsFault`, `DBProxyNotFoundFault`, `InvalidDBProxyStateFault` | Changes the settings for an existing DB proxy. |
| `ModifyDBProxyEndpoint` | - | - | `DBProxyEndpointName` | - | `ModifyDBProxyEndpointResponse` | `DBProxyEndpointAlreadyExistsFault`, `DBProxyEndpointNotFoundFault`, `InvalidDBProxyEndpointStateFault`, `InvalidDBProxyStateFault` | Changes the settings for an existing DB proxy endpoint. |
| `ModifyDBProxyTargetGroup` | - | - | `DBProxyName`, `TargetGroupName` | - | `ModifyDBProxyTargetGroupResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `InvalidDBProxyStateFault` | Modifies the properties of a `DBProxyTargetGroup`. |
| `ModifyDBRecommendation` | - | - | `RecommendationId` | - | `DBRecommendationMessage` | - | Updates the recommendation status and recommended action status for the specified recommendation. |
| `ModifyDBShardGroup` | - | - | `DBShardGroupIdentifier` | - | `DBShardGroup` | `DBShardGroupAlreadyExistsFault`, `DBShardGroupNotFoundFault`, `InvalidDBClusterStateFault` | Modifies the settings of an Aurora Limitless Database DB shard group. You can change one or more settings by specifying these parameters and the new values in the request. |
| `ModifyDBSnapshot` | - | - | `DBSnapshotIdentifier` | - | `ModifyDBSnapshotResult` | `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault`, `KMSKeyNotAccessibleFault` | Updates a manual DB snapshot with a new engine version. The snapshot can be encrypted or unencrypted, but not shared or public. |
| `ModifyDBSnapshotAttribute` | - | - | `AttributeName`, `DBSnapshotIdentifier` | - | `ModifyDBSnapshotAttributeResult` | `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault`, `SharedSnapshotQuotaExceededFault` | Adds an attribute and values to, or removes an attribute and values from, a manual DB snapshot. To share a manual DB snapshot with other Amazon Web Services accounts, specify `restore` as the `AttributeName` and use the `ValuesToAdd` parameter to add a list... |
| `ModifyDBSubnetGroup` | - | - | `DBSubnetGroupName`, `SubnetIds` | - | `ModifyDBSubnetGroupResult` | `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DBSubnetQuotaExceededFault`, `InvalidDBSubnetGroupStateFault`, `InvalidSubnet`, `SubnetAlreadyInUse` | Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the Amazon Web Services Region. |
| `ModifyEventSubscription` | - | - | `SubscriptionName` | - | `ModifyEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SubscriptionCategoryNotFoundFault`, `SubscriptionNotFoundFault` | Modifies an existing RDS event notification subscription. You can't modify the source identifiers using this call. |
| `ModifyGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `ModifyGlobalClusterResult` | `GlobalClusterAlreadyExistsFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidGlobalClusterStateFault` | Modifies a setting for an Amazon Aurora global database cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. |
| `ModifyIntegration` | - | - | `IntegrationIdentifier` | - | `Integration` | `IntegrationConflictOperationFault`, `IntegrationNotFoundFault`, `InvalidIntegrationStateFault` | Modifies a zero-ETL integration with Amazon Redshift. |
| `ModifyOptionGroup` | - | - | `OptionGroupName` | - | `ModifyOptionGroupResult` | `InvalidOptionGroupStateFault`, `OptionGroupNotFoundFault` | Modifies an existing option group. |
| `ModifyTenantDatabase` | - | - | `DBInstanceIdentifier`, `TenantDBName` | - | `ModifyTenantDatabaseResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `TenantDatabaseAlreadyExistsFault`, `TenantDatabaseNotFoundFault` | Modifies an existing tenant database in a DB instance. You can change the tenant database name or the master user password. |
| `PromoteReadReplica` | - | - | `DBInstanceIdentifier` | - | `PromoteReadReplicaResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | Promotes a read replica DB instance to a standalone DB instance. Backup duration is a function of the amount of changes to the database since the previous backup. |
| `PromoteReadReplicaDBCluster` | - | - | `DBClusterIdentifier` | - | `PromoteReadReplicaDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault` | Promotes a read replica DB cluster to a standalone DB cluster. |
| `PurchaseReservedDBInstancesOffering` | - | - | `ReservedDBInstancesOfferingId` | - | `PurchaseReservedDBInstancesOfferingResult` | `ReservedDBInstanceAlreadyExistsFault`, `ReservedDBInstanceQuotaExceededFault`, `ReservedDBInstancesOfferingNotFoundFault` | Purchases a reserved DB instance offering. |
| `RebootDBCluster` | - | - | `DBClusterIdentifier` | - | `RebootDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | You might need to reboot your DB cluster, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB cluster parameter group associated with the DB cluster, reboot the DB cluster for the changes to take effect. |
| `RebootDBInstance` | - | - | `DBInstanceIdentifier` | - | `RebootDBInstanceResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault` | You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the instance for the changes to take effect. |
| `RebootDBShardGroup` | - | - | `DBShardGroupIdentifier` | - | `DBShardGroup` | `DBShardGroupNotFoundFault`, `InvalidDBShardGroupStateFault` | You might need to reboot your DB shard group, usually for maintenance reasons. For example, if you make certain modifications, reboot the DB shard group for the changes to take effect. |
| `RegisterDBProxyTargets` | - | - | `DBProxyName` | - | `RegisterDBProxyTargetsResponse` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetAlreadyRegisteredFault`, `DBProxyTargetGroupNotFoundFault`, `InsufficientAvailableIPsInSubnetFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, ... (+1) | Associate one or more `DBProxyTarget` data structures with a `DBProxyTargetGroup`. |
| `RemoveFromGlobalCluster` | - | - | `DbClusterIdentifier`, `GlobalClusterIdentifier` | - | `RemoveFromGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Detaches an Aurora secondary cluster from an Aurora global database cluster. The cluster becomes a standalone cluster with read-write capability instead of being read-only and receiving data from a primary cluster in a different Region. |
| `RemoveRoleFromDBCluster` | - | - | `DBClusterIdentifier`, `RoleArn` | - | `Unit` | `DBClusterNotFoundFault`, `DBClusterRoleNotFoundFault`, `InvalidDBClusterStateFault` | Removes the asssociation of an Amazon Web Services Identity and Access Management (IAM) role from a DB cluster. For more information on Amazon Aurora DB clusters, see What is Amazon Aurora? |
| `RemoveRoleFromDBInstance` | - | - | `DBInstanceIdentifier`, `FeatureName`, `RoleArn` | - | `Unit` | `DBInstanceNotFoundFault`, `DBInstanceRoleNotFoundFault`, `InvalidDBInstanceStateFault` | Disassociates an Amazon Web Services Identity and Access Management (IAM) role from a DB instance. |
| `RemoveSourceIdentifierFromSubscription` | - | - | `SourceIdentifier`, `SubscriptionName` | - | `RemoveSourceIdentifierFromSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Removes a source identifier from an existing RDS event notification subscription. |
| `RemoveTagsFromResource` | - | - | `ResourceName`, `TagKeys` | - | `Unit` | `BlueGreenDeploymentNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBShardGroupNotFoundFault`, `DBSnapshotNotFoundFault`, ... (+6) | Removes metadata tags from an Amazon RDS resource. For an overview on tagging an Amazon RDS resource, see Tagging Amazon RDS Resources in the Amazon RDS User Guide or Tagging Amazon Aurora and Amazon RDS Resources in the Amazon Aurora User Guide . |
| `ResetDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: `ParameterName` and `ApplyMethod`. |
| `ResetDBParameterGroup` | - | - | `DBParameterGroupName` | - | `DBParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: `ParameterName` and `ApplyMethod`. |
| `RestoreDBClusterFromS3` | - | - | `DBClusterIdentifier`, `Engine`, `MasterUsername`, `S3BucketName`, `S3IngestionRoleArn`, `SourceEngine`, `SourceEngineVersion` | - | `RestoreDBClusterFromS3Result` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterStateFault`, ... (+8) | Creates an Amazon Aurora DB cluster from MySQL data stored in an Amazon S3 bucket. Amazon RDS must be authorized to access the Amazon S3 bucket and the data must be created using the Percona XtraBackup utility as described in Migrating Data from MySQL by... |
| `RestoreDBClusterFromSnapshot` | - | - | `DBClusterIdentifier`, `Engine`, `SnapshotIdentifier` | - | `RestoreDBClusterFromSnapshotResult` | `DBClusterAlreadyExistsFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, ... (+15) | Creates a new DB cluster from a DB snapshot or DB cluster snapshot. The target DB cluster is created from the source snapshot with a default configuration. |
| `RestoreDBClusterToPointInTime` | - | - | `DBClusterIdentifier` | - | `RestoreDBClusterToPointInTimeResult` | `DBClusterAlreadyExistsFault`, `DBClusterAutomatedBackupNotFoundFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, ... (+15) | Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before `LatestRestorableTime` for up to `BackupRetentionPeriod` days. |
| `RestoreDBInstanceFromDBSnapshot` | - | - | `DBInstanceIdentifier` | - | `RestoreDBInstanceFromDBSnapshotResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBClusterSnapshotNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSnapshotNotFoundFault`, ... (+17) | Creates a new DB instance from a DB snapshot. The target database is created from the source database restore point with most of the source's original configuration, including the default security group and DB parameter group. |
| `RestoreDBInstanceFromS3` | - | - | `DBInstanceClass`, `DBInstanceIdentifier`, `Engine`, `S3BucketName`, `S3IngestionRoleArn`, `SourceEngine`, `SourceEngineVersion` | - | `RestoreDBInstanceFromS3Result` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, ... (+12) | Amazon Relational Database Service (Amazon RDS) supports importing MySQL databases by using backup files. You can create a backup of your on-premises database, store it on Amazon Simple Storage Service (Amazon S3), and then restore the backup file onto a new... |
| `RestoreDBInstanceToPointInTime` | - | - | `TargetDBInstanceIdentifier` | - | `RestoreDBInstanceToPointInTimeResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceAutomatedBackupNotFoundFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, ... (+18) | Restores a DB instance to an arbitrary point in time. You can restore to any point in time before the time identified by the `LatestRestorableTime` property. |
| `RevokeDBSecurityGroupIngress` | - | - | `DBSecurityGroupName` | - | `RevokeDBSecurityGroupIngressResult` | `AuthorizationNotFoundFault`, `DBSecurityGroupNotFoundFault`, `InvalidDBSecurityGroupStateFault` | Revokes ingress from a DBSecurityGroup for previously authorized IP ranges or EC2 or VPC security groups. Required parameters for this API are one of CIDRIP, EC2SecurityGroupId for VPC, or (EC2SecurityGroupOwnerId and either EC2SecurityGroupName or... |
| `StartActivityStream` | - | - | `KmsKeyId`, `Mode`, `ResourceArn` | - | `StartActivityStreamResponse` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceNotFoundFault` | Starts a database activity stream to monitor activity on the database. For more information, see Monitoring Amazon Aurora with Database Activity Streams in the Amazon Aurora User Guide or Monitoring Amazon RDS with Database Activity Streams in the Amazon RDS... |
| `StartDBCluster` | - | - | `DBClusterIdentifier` | - | `StartDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBShardGroupStateFault`, `KMSKeyNotAccessibleFault`, `VpcEncryptionControlViolationException` | Starts an Amazon Aurora DB cluster that was stopped using the Amazon Web Services console, the stop-db-cluster CLI command, or the `StopDBCluster` operation. For more information, see Stopping and Starting an Aurora Cluster in the Amazon Aurora User Guide . |
| `StartDBInstance` | - | - | `DBInstanceIdentifier` | - | `StartDBInstanceResult` | `AuthorizationNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, ... (+4) | Starts an Amazon RDS DB instance that was stopped using the Amazon Web Services console, the stop-db-instance CLI command, or the `StopDBInstance` operation. For more information, see Starting an Amazon RDS DB instance That Was Previously Stopped in the... |
| `StartDBInstanceAutomatedBackupsReplication` | - | - | `SourceDBInstanceArn` | - | `StartDBInstanceAutomatedBackupsReplicationResult` | `DBInstanceAutomatedBackupQuotaExceededFault`, `DBInstanceNotFoundFault`, `InvalidDBInstanceAutomatedBackupStateFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `StorageTypeNotSupportedFault` | Enables replication of automated backups to a different Amazon Web Services Region. This command doesn't apply to RDS Custom. |
| `StartExportTask` | - | - | `ExportTaskIdentifier`, `IamRoleArn`, `KmsKeyId`, `S3BucketName`, `SourceArn` | - | `ExportTask` | `DBClusterNotFoundFault`, `DBClusterSnapshotNotFoundFault`, `DBSnapshotNotFoundFault`, `ExportTaskAlreadyExistsFault`, `IamRoleMissingPermissionsFault`, `IamRoleNotFoundFault`, `InvalidExportOnlyFault`, `InvalidExportSourceStateFault`, ... (+2) | Starts an export of DB snapshot or DB cluster data to Amazon S3. The provided IAM role must have access to the S3 bucket. |
| `StopActivityStream` | - | - | `ResourceArn` | - | `StopActivityStreamResponse` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Stops a database activity stream that was started using the Amazon Web Services console, the `start-activity-stream` CLI command, or the `StartActivityStream` operation. For more information, see Monitoring Amazon Aurora with Database Activity Streams in the... |
| `StopDBCluster` | - | - | `DBClusterIdentifier` | - | `StopDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBShardGroupStateFault` | Stops an Amazon Aurora DB cluster. When you stop a DB cluster, Aurora retains the DB cluster's metadata, including its endpoints and DB parameter groups. |
| `StopDBInstance` | - | - | `DBInstanceIdentifier` | - | `StopDBInstanceResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `SnapshotQuotaExceededFault` | Stops an Amazon RDS DB instance temporarily. When you stop a DB instance, Amazon RDS retains the DB instance's metadata, including its endpoint, DB parameter group, and option group membership. |
| `StopDBInstanceAutomatedBackupsReplication` | - | - | `SourceDBInstanceArn` | - | `StopDBInstanceAutomatedBackupsReplicationResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | Stops automated backup replication for a DB instance. This command doesn't apply to RDS Custom, Aurora MySQL, and Aurora PostgreSQL. |
| `SwitchoverBlueGreenDeployment` | - | - | `BlueGreenDeploymentIdentifier` | - | `SwitchoverBlueGreenDeploymentResponse` | `BlueGreenDeploymentNotFoundFault`, `InvalidBlueGreenDeploymentStateFault` | Switches over a blue/green deployment. Before you switch over, production traffic is routed to the databases in the blue environment. |
| `SwitchoverGlobalCluster` | - | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `SwitchoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Switches over the specified secondary DB cluster to be the new primary DB cluster in the global database cluster. Switchover operations were previously called "managed planned failovers." Aurora promotes the specified secondary cluster to assume full... |
| `SwitchoverReadReplica` | - | - | `DBInstanceIdentifier` | - | `SwitchoverReadReplicaResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | Switches over an Oracle standby database in an Oracle Data Guard environment, making it the new primary database. Issue this command in the Region that hosts the current standby database. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidDBClusterStateFault` | `structure` | `message` | The requested operation can't be performed while the cluster is in this state. |
| `DBClusterNotFoundFault` | `structure` | `message` | `DBClusterIdentifier` doesn't refer to an existing DB cluster. |
| `InvalidDBInstanceStateFault` | `structure` | `message` | The DB instance isn't in a valid state. |
| `DBInstanceNotFoundFault` | `structure` | `message` | `DBInstanceIdentifier` doesn't refer to an existing DB instance. |
| `KMSKeyNotAccessibleFault` | `structure` | `message` | An error occurred accessing an Amazon Web Services KMS key. |
| `DBParameterGroupNotFoundFault` | `structure` | `message` | `DBParameterGroupName` doesn't refer to an existing DB parameter group. |
| `InvalidSubnet` | `structure` | `message` | The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC. |
| `OptionGroupNotFoundFault` | `structure` | `message` | The specified option group could not be found. |
| `DBSubnetGroupNotFoundFault` | `structure` | `message` | `DBSubnetGroupName` doesn't refer to an existing DB subnet group. |
| `DBProxyNotFoundFault` | `structure` | `message` | The specified proxy name doesn't correspond to a proxy owned by your Amazon Web Services account in the specified Amazon Web Services Region. |
| `DBSnapshotNotFoundFault` | `structure` | `message` | `DBSnapshotIdentifier` doesn't refer to an existing DB snapshot. |
| `InvalidVPCNetworkStateFault` | `structure` | `message` | The DB subnet group doesn't cover all Availability Zones after it's created because of users' change. |
| `StorageQuotaExceededFault` | `structure` | `message` | The request would result in the user exceeding the allowed amount of storage available across all DB instances. |
| `NetworkTypeNotSupported` | `structure` | `message` | The network type is invalid for the DB instance. |
| `StorageTypeNotSupportedFault` | `structure` | `message` | The specified `StorageType` can't be associated with the DB instance. |
| `VpcEncryptionControlViolationException` | `structure` | `message` | The operation violates VPC encryption control settings. |
| `DBSecurityGroupNotFoundFault` | `structure` | `message` | `DBSecurityGroupName` doesn't refer to an existing DB security group. |
| `DBSubnetGroupDoesNotCoverEnoughAZs` | `structure` | `message` | Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone. |
| `DomainNotFoundFault` | `structure` | `message` | `Domain` doesn't refer to an existing Active Directory domain. |
| `InsufficientDBInstanceCapacityFault` | `structure` | `message` | The specified DB instance class isn't available in the specified Availability Zone. |
| `DBClusterSnapshotNotFoundFault` | `structure` | `message` | `DBClusterSnapshotIdentifier` doesn't refer to an existing DB cluster snapshot. |
| `InvalidDBProxyStateFault` | `structure` | `message` | The requested operation can't be performed while the proxy is in this state. |
| `DBProxyTargetGroupNotFoundFault` | `structure` | `message` | The specified target group isn't available for a proxy owned by your Amazon Web Services account in the specified Amazon Web Services Region. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

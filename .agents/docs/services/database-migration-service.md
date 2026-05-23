# AWS Database Migration Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Database Migration Service Database Migration Service (DMS) can migrate your data to and from the most widely used commercial and open-source databases such as Oracle, PostgreSQL, Microsoft SQL Server, Amazon Redshift, MariaDB, Amazon Aurora, MySQL, and SAP Adaptive Server Enterprise (ASE). The service supports homogeneous migrations such as Oracle to Oracle, as well as heterogeneous migrations between different database platforms, such as Oracle to MySQL or SQL Server to PostgreSQL. For more information about DMS, see What Is Database Migration Service? in the Database Migration Service User Guide.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-databasemigration/tests/scenario_test.rs`: build a migration pipeline with replication instances, endpoints, replication tasks, task start/stop state, and cleanup.
- Backported from `scenario_test.rs`: set up secure migration infrastructure with subnet groups and imported certificates.
- Backported from `scenario_test.rs`: snapshot and restore service state while preserving a running migration pipeline.
- Scenario insight from EC2: include mutable binding failover for AWS Database Migration Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: add full state-machine walks for AWS Database Migration Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent source/target endpoint configuration, replication instances, migration tasks, certificates, subnet groups, event subscriptions, assessment/reporting resources, and lifecycle state transitions.

## Service Identity and Protocol

- AWS model slug: `database-migration-service`
- AWS SDK for Rust slug: `databasemigration`
- Model version: `2016-01-01`
- Model file: `vendor/api-models-aws/models/database-migration-service/service/2016-01-01/database-migration-service-2016-01-01.json`
- SDK ID: `Database Migration Service`
- Endpoint prefix: `dms`
- ARN namespace: `dms`
- CloudFormation name: `DMS`
- CloudTrail event source: `dms.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (47), `Delete` (15), `Start` (13), `Create` (11), `Modify` (11), `Cancel` (3), `Stop` (3), `Reload` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToResource`, `BatchStartRecommendations`, `CancelMetadataModelConversion`, `CancelMetadataModelCreation`, `CancelReplicationTaskAssessmentRun`, `CreateDataMigration`, `CreateDataProvider`, `CreateEndpoint`, `CreateEventSubscription`, `CreateFleetAdvisorCollector`, `CreateInstanceProfile`, `CreateMigrationProject`, `CreateReplicationConfig`, `CreateReplicationInstance`, `CreateReplicationSubnetGroup`, `CreateReplicationTask`, `DeleteCertificate`, `DeleteConnection`, `DeleteDataMigration`, `DeleteDataProvider`, ... (+41).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAttributes`, `DescribeApplicableIndividualAssessments`, `DescribeCertificates`, `DescribeConnections`, `DescribeConversionConfiguration`, `DescribeDataMigrations`, `DescribeDataProviders`, `DescribeEndpointSettings`, `DescribeEndpointTypes`, `DescribeEndpoints`, `DescribeEngineVersions`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeExtensionPackAssociations`, `DescribeFleetAdvisorCollectors`, `DescribeFleetAdvisorDatabases`, `DescribeFleetAdvisorLsaAnalysis`, `DescribeFleetAdvisorSchemaObjectSummary`, `DescribeFleetAdvisorSchemas`, ... (+29).
- Pagination is modelled for 42 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelMetadataModelConversion`, `CancelMetadataModelCreation`, `CancelReplicationTaskAssessmentRun`, `CreateReplicationTask`, `DeleteReplicationTask`, `DeleteReplicationTaskAssessmentRun`, `DescribeFleetAdvisorLsaAnalysis`, `DescribeMetadataModelExportsAsScript`, `DescribeMetadataModelExportsToTarget`, `DescribeMetadataModelImports`, `DescribeReplicationInstanceTaskLogs`, `DescribeReplicationTaskAssessmentResults`, `DescribeReplicationTaskAssessmentRuns`, `DescribeReplicationTaskIndividualAssessments`, `DescribeReplicationTasks`, `ExportMetadataModelAssessment`, `ImportCertificate`, `ModifyReplicationTask`, `MoveReplicationTask`, `RunFleetAdvisorLsaAnalysis`, ... (+16).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 111 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SNS`, `Glue`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/dms/latest/userguide/Welcome.html
- https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.Creating.html
- https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Monitoring.html

Research outcomes:
- AWS DMS migrations require source endpoints, target endpoints, and a replication instance before a migration task is created.
- Migration types include full load, full load plus change data capture (CDC), and CDC-only replication.
- Full load creates or uses target tables and loads source data in parallel.
- Full load plus CDC captures source changes during the full load and then applies captured changes to the target as committed transactions after initial load completion.
- CDC-only reads the source DBMS recovery log, groups entries by transaction, and buffers changes on the replication server if the target cannot keep up or is inaccessible.
- Task settings control logging, control tables, error handling, LOB handling, validation, target table preparation, and stop-after-full-load behaviour.
- Target table preparation modes include do nothing, drop tables on target, and truncate, with target-specific caveats.
- Table mappings select, filter, and transform the source data included in a task.
- Task statuses include creating, ready, starting, running, stopped, stopping, modifying, moving, failed-move, deleting, testing, failed, error, and running with errors.
- Table states during migration include table does not exist, before load, full load, table completed, table cancelled, and table error.
- DMS provides automatic failover for the replication server, and data at rest is encrypted with KMS while in-flight data can use SSL.

Parity implications:
- Model replication instances, endpoints, subnet groups, certificates, tasks, task settings, table mappings, assessment runs, validation, table statistics, logs, and replication server state separately.
- Task execution should reflect migration type, target table preparation, full-load progress, CDC buffering/apply, stop reasons, and per-table states.
- Status operations should expose both task-level state and table-level progress/errors.

## Current Network Resource Stub Semantics

Database Migration Service currently keeps replication subnet groups as DMS-local records.

- `CreateReplicationSubnetGroup` stores the supplied subnet IDs and optional VPC ID on a `ReplicationSubnetGroup`.
- The subnet group status is returned as complete from local state; there is no asynchronous subnet validation or EC2 subnet-to-VPC derivation.
- Replication instances and tasks can refer to DMS resources independently of EC2 networking availability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeApplicableIndividualAssessments`, `DescribeCertificates`, `DescribeConnections`, `DescribeConversionConfiguration`, `DescribeDataMigrations`, `DescribeDataProviders`, `DescribeEndpoints`, `DescribeEndpointSettings`, `DescribeEndpointTypes`, `DescribeEngineVersions`, `DescribeEventCategories`, `DescribeEvents`, `DescribeEventSubscriptions`, `DescribeExtensionPackAssociations`, `DescribeFleetAdvisorCollectors`, `DescribeFleetAdvisorDatabases`, `DescribeFleetAdvisorLsaAnalysis`, `DescribeFleetAdvisorSchemaObjectSummary`, `DescribeFleetAdvisorSchemas`, `DescribeInstanceProfiles`, `DescribeMetadataModel`, `DescribeMetadataModelAssessments`, `DescribeMetadataModelChildren`, `DescribeMetadataModelConversions`, `DescribeMetadataModelCreations`, `DescribeMetadataModelExportsAsScript`, `DescribeMetadataModelExportsToTarget`, `DescribeMetadataModelImports`, `DescribeMigrationProjects`, `DescribeOrderableReplicationInstances`, `DescribePendingMaintenanceActions`, `DescribeRecommendationLimitations`, `DescribeRecommendations`, `DescribeRefreshSchemasStatus`, `DescribeReplicationConfigs`, `DescribeReplicationInstances`, `DescribeReplicationInstanceTaskLogs`, `DescribeReplications`, `DescribeReplicationSubnetGroups`, `DescribeReplicationTableStatistics`, `DescribeReplicationTaskAssessmentResults`, `DescribeReplicationTaskAssessmentRuns`, `DescribeReplicationTaskIndividualAssessments`, `DescribeReplicationTasks`, `DescribeSchemas`, `DescribeTableStatistics`
- Traits: `paginated` (42)
- Common required input members in this group: `MigrationProjectIdentifier`, `SelectionRules`, `Origin`, `EndpointArn`

### Delete

- Operations: `DeleteCertificate`, `DeleteConnection`, `DeleteDataMigration`, `DeleteDataProvider`, `DeleteEndpoint`, `DeleteEventSubscription`, `DeleteFleetAdvisorCollector`, `DeleteFleetAdvisorDatabases`, `DeleteInstanceProfile`, `DeleteMigrationProject`, `DeleteReplicationConfig`, `DeleteReplicationInstance`, `DeleteReplicationSubnetGroup`, `DeleteReplicationTask`, `DeleteReplicationTaskAssessmentRun`
- Common required input members in this group: `EndpointArn`, `ReplicationInstanceArn`

### Start

- Operations: `StartDataMigration`, `StartExtensionPackAssociation`, `StartMetadataModelAssessment`, `StartMetadataModelConversion`, `StartMetadataModelCreation`, `StartMetadataModelExportAsScript`, `StartMetadataModelExportToTarget`, `StartMetadataModelImport`, `StartRecommendations`, `StartReplication`, `StartReplicationTask`, `StartReplicationTaskAssessment`, `StartReplicationTaskAssessmentRun`
- Common required input members in this group: `MigrationProjectIdentifier`, `SelectionRules`, `Origin`, `ReplicationTaskArn`

### Create

- Operations: `CreateDataMigration`, `CreateDataProvider`, `CreateEndpoint`, `CreateEventSubscription`, `CreateFleetAdvisorCollector`, `CreateInstanceProfile`, `CreateMigrationProject`, `CreateReplicationConfig`, `CreateReplicationInstance`, `CreateReplicationSubnetGroup`, `CreateReplicationTask`
- Common required input members in this group: `ServiceAccessRoleArn`, `SourceEndpointArn`, `TargetEndpointArn`, `TableMappings`

### Modify

- Operations: `ModifyConversionConfiguration`, `ModifyDataMigration`, `ModifyDataProvider`, `ModifyEndpoint`, `ModifyEventSubscription`, `ModifyInstanceProfile`, `ModifyMigrationProject`, `ModifyReplicationConfig`, `ModifyReplicationInstance`, `ModifyReplicationSubnetGroup`, `ModifyReplicationTask`
- Common required input members in this group: `MigrationProjectIdentifier`

### Cancel

- Operations: `CancelMetadataModelConversion`, `CancelMetadataModelCreation`, `CancelReplicationTaskAssessmentRun`
- Common required input members in this group: `MigrationProjectIdentifier`, `RequestIdentifier`

### Stop

- Operations: `StopDataMigration`, `StopReplication`, `StopReplicationTask`
- Common required input members in this group: -

### Reload

- Operations: `ReloadReplicationTables`, `ReloadTables`
- Common required input members in this group: `TablesToReload`

### Add

- Operations: `AddTagsToResource`
- Common required input members in this group: -

### Apply

- Operations: `ApplyPendingMaintenanceAction`
- Common required input members in this group: -

### Batch

- Operations: `BatchStartRecommendations`
- Common required input members in this group: -

### Export

- Operations: `ExportMetadataModelAssessment`
- Common required input members in this group: -

### Get

- Operations: `GetTargetSelectionRules`
- Common required input members in this group: -

### Import

- Operations: `ImportCertificate`
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: -

### Move

- Operations: `MoveReplicationTask`
- Common required input members in this group: -

### Reboot

- Operations: `RebootReplicationInstance`
- Common required input members in this group: -

### Refresh

- Operations: `RefreshSchemas`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: -

### Run

- Operations: `RunFleetAdvisorLsaAnalysis`
- Common required input members in this group: -

### Test

- Operations: `TestConnection`
- Common required input members in this group: -

### Update

- Operations: `UpdateSubscriptionsToEventBridge`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToResource` | `-` | - | `ResourceArn`, `Tags` | - | `AddTagsToResourceResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Adds metadata tags to an DMS resource, including replication instance, endpoint, subnet group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resource ... |
| `ApplyPendingMaintenanceAction` | `-` | - | `ReplicationInstanceArn`, `ApplyAction`, `OptInType` | - | `ApplyPendingMaintenanceActionResponse` | `ResourceNotFoundFault` | Applies a pending maintenance action to a resource (for example, to a replication instance). |
| `BatchStartRecommendations` | `-` | - | - | - | `BatchStartRecommendationsResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `CancelMetadataModelConversion` | `-` | - | `MigrationProjectIdentifier`, `RequestIdentifier` | - | `CancelMetadataModelConversionResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Cancels a single metadata model conversion operation that was started with StartMetadataModelConversion . |
| `CancelMetadataModelCreation` | `-` | - | `MigrationProjectIdentifier`, `RequestIdentifier` | - | `CancelMetadataModelCreationResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Cancels a single metadata model creation operation that was started with StartMetadataModelCreation . |
| `CancelReplicationTaskAssessmentRun` | `-` | - | `ReplicationTaskAssessmentRunArn` | - | `CancelReplicationTaskAssessmentRunResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Cancels a single premigration assessment run. This operation prevents any individual assessments from running if they haven't started running. It also attempts to cancel any individual assessments that are currently ... |
| `CreateDataMigration` | `-` | - | `MigrationProjectIdentifier`, `DataMigrationType`, `ServiceAccessRoleArn` | - | `CreateDataMigrationResponse` | `FailedDependencyFault`, `InvalidOperationFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Creates a data migration using the provided settings. |
| `CreateDataProvider` | `-` | - | `Engine`, `Settings` | - | `CreateDataProviderResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `ResourceAlreadyExistsFault`, `ResourceQuotaExceededFault` | Creates a data provider using the provided settings. A data provider stores a data store type and location information about your database. |
| `CreateEndpoint` | `-` | - | `EndpointIdentifier`, `EndpointType`, `EngineName` | - | `CreateEndpointResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault` | Creates an endpoint using the provided settings. For a MySQL source or target endpoint, don't explicitly specify the database using the DatabaseName request parameter on the CreateEndpoint API call. Specifying Databa ... |
| `CreateEventSubscription` | `-` | - | `SubscriptionName`, `SnsTopicArn` | - | `CreateEventSubscriptionResponse` | `KMSAccessDeniedFault`, `KMSDisabledFault`, `KMSInvalidStateFault`, `KMSNotFoundFault`, `KMSThrottlingFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault` | Creates an DMS event notification subscription. You can specify the type of source ( SourceType ) you want to be notified of, provide a list of DMS source IDs ( SourceIds ) that triggers the events, and provide a lis ... |
| `CreateFleetAdvisorCollector` | `-` | - | `CollectorName`, `ServiceAccessRoleArn`, `S3BucketName` | - | `CreateFleetAdvisorCollectorResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `CreateInstanceProfile` | `-` | - | - | - | `CreateInstanceProfileResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Creates the instance profile using the specified parameters. |
| `CreateMigrationProject` | `-` | - | `SourceDataProviderDescriptors`, `TargetDataProviderDescriptors`, `InstanceProfileIdentifier` | - | `CreateMigrationProjectResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Creates the migration project using the specified parameters. You can run this action only after you create an instance profile and data providers using CreateInstanceProfile and CreateDataProvider . |
| `CreateReplicationConfig` | `-` | - | `ReplicationConfigIdentifier`, `SourceEndpointArn`, `TargetEndpointArn`, `ComputeConfig`, `ReplicationType`, `TableMappings` | - | `CreateReplicationConfigResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `InvalidSubnet`, `KMSKeyNotAccessibleFault`, `ReplicationSubnetGroupDoesNotCoverEnoughAZs`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Creates a configuration that you can later provide to configure and start an DMS Serverless replication. You can also provide options to validate the configuration inputs before you start the replication. |
| `CreateReplicationInstance` | `-` | - | `ReplicationInstanceIdentifier`, `ReplicationInstanceClass` | - | `CreateReplicationInstanceResponse` | `AccessDeniedFault`, `InsufficientResourceCapacityFault`, `InvalidResourceStateFault`, `InvalidSubnet`, `KMSKeyNotAccessibleFault`, `ReplicationSubnetGroupDoesNotCoverEnoughAZs`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `StorageQuotaExceededFault` | Creates the replication instance using the specified parameters. DMS requires that your account have certain roles with appropriate permissions before you can create a replication instance. For information on the req ... |
| `CreateReplicationSubnetGroup` | `-` | - | `ReplicationSubnetGroupIdentifier`, `ReplicationSubnetGroupDescription`, `SubnetIds` | - | `CreateReplicationSubnetGroupResponse` | `AccessDeniedFault`, `InvalidSubnet`, `ReplicationSubnetGroupDoesNotCoverEnoughAZs`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Creates a replication subnet group given a list of the subnet IDs in a VPC. The VPC needs to have at least one subnet in at least two availability zones in the Amazon Web Services Region, otherwise the service will t ... |
| `CreateReplicationTask` | `-` | - | `ReplicationTaskIdentifier`, `SourceEndpointArn`, `TargetEndpointArn`, `ReplicationInstanceArn`, `MigrationType`, `TableMappings` | - | `CreateReplicationTaskResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Creates a replication task using the specified parameters. |
| `DeleteCertificate` | `-` | - | `CertificateArn` | - | `DeleteCertificateResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified certificate. |
| `DeleteConnection` | `-` | - | `EndpointArn`, `ReplicationInstanceArn` | - | `DeleteConnectionResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the connection between a replication instance and an endpoint. |
| `DeleteDataMigration` | `-` | - | `DataMigrationIdentifier` | - | `DeleteDataMigrationResponse` | `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified data migration. |
| `DeleteDataProvider` | `-` | - | `DataProviderIdentifier` | - | `DeleteDataProviderResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified data provider. All migration projects associated with the data provider must be deleted or modified before you can delete the data provider. |
| `DeleteEndpoint` | `-` | - | `EndpointArn` | - | `DeleteEndpointResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified endpoint. All tasks associated with the endpoint must be deleted before you can delete the endpoint. |
| `DeleteEventSubscription` | `-` | - | `SubscriptionName` | - | `DeleteEventSubscriptionResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes an DMS event subscription. |
| `DeleteFleetAdvisorCollector` | `-` | - | `CollectorReferencedId` | - | `Unit` | `AccessDeniedFault`, `CollectorNotFoundFault`, `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DeleteFleetAdvisorDatabases` | `-` | - | `DatabaseIds` | - | `DeleteFleetAdvisorDatabasesResponse` | `AccessDeniedFault`, `InvalidOperationFault`, `ResourceNotFoundFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DeleteInstanceProfile` | `-` | - | `InstanceProfileIdentifier` | - | `DeleteInstanceProfileResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified instance profile. All migration projects associated with the instance profile must be deleted or modified before you can delete the instance profile. |
| `DeleteMigrationProject` | `-` | - | `MigrationProjectIdentifier` | - | `DeleteMigrationProjectResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified migration project. The migration project must be closed before you can delete it. |
| `DeleteReplicationConfig` | `-` | - | `ReplicationConfigArn` | - | `DeleteReplicationConfigResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes an DMS Serverless replication configuration. This effectively deprovisions any and all replications that use this configuration. You can't delete the configuration for an DMS Serverless replication that is on ... |
| `DeleteReplicationInstance` | `-` | - | `ReplicationInstanceArn` | - | `DeleteReplicationInstanceResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified replication instance. You must delete any migration tasks that are associated with the replication instance before you can delete it. |
| `DeleteReplicationSubnetGroup` | `-` | - | `ReplicationSubnetGroupIdentifier` | - | `DeleteReplicationSubnetGroupResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes a subnet group. |
| `DeleteReplicationTask` | `-` | - | `ReplicationTaskArn` | - | `DeleteReplicationTaskResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the specified replication task. |
| `DeleteReplicationTaskAssessmentRun` | `-` | - | `ReplicationTaskAssessmentRunArn` | - | `DeleteReplicationTaskAssessmentRunResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Deletes the record of a single premigration assessment run. This operation removes all metadata that DMS maintains about this assessment run. However, the operation leaves untouched all information about this assessm ... |
| `DescribeAccountAttributes` | `-` | - | - | - | `DescribeAccountAttributesResponse` | - | Lists all of the DMS attributes for a customer account. These attributes include DMS quotas for the account and a unique account identifier in a particular DMS region. DMS quotas include a list of resource quotas sup ... |
| `DescribeApplicableIndividualAssessments` | `-` | `paginated` | - | - | `DescribeApplicableIndividualAssessmentsResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Provides a list of individual assessments that you can specify for a new premigration assessment run, given one or more parameters. If you specify an existing migration task, this operation provides the default indiv ... |
| `DescribeCertificates` | `-` | `paginated` | - | - | `DescribeCertificatesResponse` | `ResourceNotFoundFault` | Provides a description of the certificate. |
| `DescribeConnections` | `-` | `paginated` | - | - | `DescribeConnectionsResponse` | `ResourceNotFoundFault` | Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint. |
| `DescribeConversionConfiguration` | `-` | - | `MigrationProjectIdentifier` | - | `DescribeConversionConfigurationResponse` | `ResourceNotFoundFault` | Returns configuration parameters for a schema conversion project. |
| `DescribeDataMigrations` | `-` | `paginated` | - | - | `DescribeDataMigrationsResponse` | `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Returns information about data migrations. |
| `DescribeDataProviders` | `-` | `paginated` | - | - | `DescribeDataProvidersResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `ResourceNotFoundFault` | Returns a paginated list of data providers for your account in the current region. |
| `DescribeEndpoints` | `-` | `paginated` | - | - | `DescribeEndpointsResponse` | `ResourceNotFoundFault` | Returns information about the endpoints for your account in the current region. |
| `DescribeEndpointSettings` | `-` | `paginated` | `EngineName` | - | `DescribeEndpointSettingsResponse` | - | Returns information about the possible endpoint settings available when you create an endpoint for a specific database engine. |
| `DescribeEndpointTypes` | `-` | `paginated` | - | - | `DescribeEndpointTypesResponse` | - | Returns information about the type of endpoints available. |
| `DescribeEngineVersions` | `-` | `paginated` | - | - | `DescribeEngineVersionsResponse` | - | Returns information about the replication instance versions used in the project. |
| `DescribeEventCategories` | `-` | - | - | - | `DescribeEventCategoriesResponse` | - | Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in Working with Events and Notifications in the Database Migrati ... |
| `DescribeEvents` | `-` | `paginated` | - | - | `DescribeEventsResponse` | - | Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on DMS events, see Working with Events and Notifications in the Database Migration Service U ... |
| `DescribeEventSubscriptions` | `-` | `paginated` | - | - | `DescribeEventSubscriptionsResponse` | `ResourceNotFoundFault` | Lists all the event subscriptions for a customer account. The description of a subscription includes SubscriptionName , SNSTopicARN , CustomerID , SourceType , SourceID , CreationTime , and Status . If you specify Su ... |
| `DescribeExtensionPackAssociations` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeExtensionPackAssociationsResponse` | - | Returns a paginated list of extension pack associations for the specified migration project. An extension pack is an add-on module that emulates functions present in a source database that are required when convertin ... |
| `DescribeFleetAdvisorCollectors` | `-` | `paginated` | - | - | `DescribeFleetAdvisorCollectorsResponse` | `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeFleetAdvisorDatabases` | `-` | `paginated` | - | - | `DescribeFleetAdvisorDatabasesResponse` | `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeFleetAdvisorLsaAnalysis` | `-` | `paginated` | - | - | `DescribeFleetAdvisorLsaAnalysisResponse` | `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeFleetAdvisorSchemaObjectSummary` | `-` | `paginated` | - | - | `DescribeFleetAdvisorSchemaObjectSummaryResponse` | `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeFleetAdvisorSchemas` | `-` | `paginated` | - | - | `DescribeFleetAdvisorSchemasResponse` | `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeInstanceProfiles` | `-` | `paginated` | - | - | `DescribeInstanceProfilesResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `ResourceNotFoundFault` | Returns a paginated list of instance profiles for your account in the current region. |
| `DescribeMetadataModel` | `-` | - | `SelectionRules`, `MigrationProjectIdentifier`, `Origin` | - | `DescribeMetadataModelResponse` | `AccessDeniedFault`, `ResourceNotFoundFault` | Gets detailed information about the specified metadata model, including its definition and corresponding converted objects in the target database if applicable. |
| `DescribeMetadataModelAssessments` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeMetadataModelAssessmentsResponse` | `ResourceNotFoundFault` | Returns a paginated list of metadata model assessments for your account in the current region. |
| `DescribeMetadataModelChildren` | `-` | `paginated` | `SelectionRules`, `MigrationProjectIdentifier`, `Origin` | - | `DescribeMetadataModelChildrenResponse` | `AccessDeniedFault`, `ResourceNotFoundFault` | Gets a list of child metadata models for the specified metadata model in the database hierarchy. |
| `DescribeMetadataModelConversions` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeMetadataModelConversionsResponse` | `ResourceNotFoundFault` | Returns a paginated list of metadata model conversions for a migration project. |
| `DescribeMetadataModelCreations` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeMetadataModelCreationsResponse` | `AccessDeniedFault`, `ResourceNotFoundFault` | Returns a paginated list of metadata model creation requests for a migration project. |
| `DescribeMetadataModelExportsAsScript` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeMetadataModelExportsAsScriptResponse` | `ResourceNotFoundFault` | Returns a paginated list of metadata model exports. |
| `DescribeMetadataModelExportsToTarget` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeMetadataModelExportsToTargetResponse` | `ResourceNotFoundFault` | Returns a paginated list of metadata model exports. |
| `DescribeMetadataModelImports` | `-` | `paginated` | `MigrationProjectIdentifier` | - | `DescribeMetadataModelImportsResponse` | `ResourceNotFoundFault` | Returns a paginated list of metadata model imports. |
| `DescribeMigrationProjects` | `-` | `paginated` | - | - | `DescribeMigrationProjectsResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `ResourceNotFoundFault` | Returns a paginated list of migration projects for your account in the current region. |
| `DescribeOrderableReplicationInstances` | `-` | `paginated` | - | - | `DescribeOrderableReplicationInstancesResponse` | - | Returns information about the replication instance types that can be created in the specified region. |
| `DescribePendingMaintenanceActions` | `-` | `paginated` | - | - | `DescribePendingMaintenanceActionsResponse` | `ResourceNotFoundFault` | Returns a list of upcoming maintenance events for replication instances in your account in the current Region. |
| `DescribeRecommendationLimitations` | `-` | `paginated` | - | - | `DescribeRecommendationLimitationsResponse` | `AccessDeniedFault`, `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeRecommendations` | `-` | `paginated` | - | - | `DescribeRecommendationsResponse` | `AccessDeniedFault`, `InvalidResourceStateFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `DescribeRefreshSchemasStatus` | `-` | - | `EndpointArn` | - | `DescribeRefreshSchemasStatusResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Returns the status of the RefreshSchemas operation. |
| `DescribeReplicationConfigs` | `-` | `paginated` | - | - | `DescribeReplicationConfigsResponse` | `ResourceNotFoundFault` | Returns one or more existing DMS Serverless replication configurations as a list of structures. |
| `DescribeReplicationInstances` | `-` | `paginated` | - | - | `DescribeReplicationInstancesResponse` | `ResourceNotFoundFault` | Returns information about replication instances for your account in the current region. |
| `DescribeReplicationInstanceTaskLogs` | `-` | `paginated` | `ReplicationInstanceArn` | - | `DescribeReplicationInstanceTaskLogsResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Returns information about the task logs for the specified task. |
| `DescribeReplications` | `-` | `paginated` | - | - | `DescribeReplicationsResponse` | `ResourceNotFoundFault` | Provides details on replication progress by returning status information for one or more provisioned DMS Serverless replications. |
| `DescribeReplicationSubnetGroups` | `-` | `paginated` | - | - | `DescribeReplicationSubnetGroupsResponse` | `ResourceNotFoundFault` | Returns information about the replication subnet groups. |
| `DescribeReplicationTableStatistics` | `-` | `paginated` | `ReplicationConfigArn` | - | `DescribeReplicationTableStatisticsResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Returns table and schema statistics for one or more provisioned replications that use a given DMS Serverless replication configuration. |
| `DescribeReplicationTaskAssessmentResults` | `-` | `paginated` | - | - | `DescribeReplicationTaskAssessmentResultsResponse` | `ResourceNotFoundFault` | Returns the task assessment results from the Amazon S3 bucket that DMS creates in your Amazon Web Services account. This action always returns the latest results. For more information about DMS task assessments, see ... |
| `DescribeReplicationTaskAssessmentRuns` | `-` | `paginated` | - | - | `DescribeReplicationTaskAssessmentRunsResponse` | `ResourceNotFoundFault` | Returns a paginated list of premigration assessment runs based on filter settings. These filter settings can specify a combination of premigration assessment runs, migration tasks, replication instances, and assessme ... |
| `DescribeReplicationTaskIndividualAssessments` | `-` | `paginated` | - | - | `DescribeReplicationTaskIndividualAssessmentsResponse` | `ResourceNotFoundFault` | Returns a paginated list of individual assessments based on filter settings. These filter settings can specify a combination of premigration assessment runs, migration tasks, and assessment status values. |
| `DescribeReplicationTasks` | `-` | `paginated` | - | - | `DescribeReplicationTasksResponse` | `ResourceNotFoundFault` | Returns information about replication tasks for your account in the current region. |
| `DescribeSchemas` | `-` | `paginated` | `EndpointArn` | - | `DescribeSchemasResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Returns information about the schema for the specified endpoint. |
| `DescribeTableStatistics` | `-` | `paginated` | `ReplicationTaskArn` | - | `DescribeTableStatisticsResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted. Note that the "last updated" column the DMS console only indicates the time that DMS last ... |
| `ExportMetadataModelAssessment` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules` | - | `ExportMetadataModelAssessmentResponse` | `ResourceNotFoundFault` | Saves a copy of a database migration assessment report to your Amazon S3 bucket. DMS can save your assessment report as a comma-separated value (CSV) or a PDF file. |
| `GetTargetSelectionRules` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules` | - | `GetTargetSelectionRulesResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Converts source selection rules into their target counterparts for schema conversion operations. |
| `ImportCertificate` | `-` | - | `CertificateIdentifier` | - | `ImportCertificateResponse` | `InvalidCertificateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceQuotaExceededFault` | Uploads the specified certificate. |
| `ListTagsForResource` | `-` | - | - | - | `ListTagsForResourceResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Lists all metadata tags attached to an DMS resource, including replication instance, endpoint, subnet group, and migration task. For more information, see Tag data type description. |
| `ModifyConversionConfiguration` | `-` | - | `MigrationProjectIdentifier`, `ConversionConfiguration` | - | `ModifyConversionConfigurationResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Modifies the specified schema conversion configuration using the provided parameters. |
| `ModifyDataMigration` | `-` | - | `DataMigrationIdentifier` | - | `ModifyDataMigrationResponse` | `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Modifies an existing DMS data migration. |
| `ModifyDataProvider` | `-` | - | `DataProviderIdentifier` | - | `ModifyDataProviderResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Modifies the specified data provider using the provided settings. You must remove the data provider from all migration projects before you can modify it. |
| `ModifyEndpoint` | `-` | - | `EndpointArn` | - | `ModifyEndpointResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault` | Modifies the specified endpoint. For a MySQL source or target endpoint, don't explicitly specify the database using the DatabaseName request parameter on the ModifyEndpoint API call. Specifying DatabaseName when you ... |
| `ModifyEventSubscription` | `-` | - | `SubscriptionName` | - | `ModifyEventSubscriptionResponse` | `AccessDeniedFault`, `KMSAccessDeniedFault`, `KMSDisabledFault`, `KMSInvalidStateFault`, `KMSNotFoundFault`, `KMSThrottlingFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault` | Modifies an existing DMS event notification subscription. |
| `ModifyInstanceProfile` | `-` | - | `InstanceProfileIdentifier` | - | `ModifyInstanceProfileResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceNotFoundFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Modifies the specified instance profile using the provided parameters. All migration projects associated with the instance profile must be deleted or modified before you can modify the instance profile. |
| `ModifyMigrationProject` | `-` | - | `MigrationProjectIdentifier` | - | `ModifyMigrationProjectResponse` | `AccessDeniedFault`, `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Modifies the specified migration project using the provided parameters. The migration project must be closed before you can modify it. |
| `ModifyReplicationConfig` | `-` | - | `ReplicationConfigArn` | - | `ModifyReplicationConfigResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `InvalidSubnet`, `KMSKeyNotAccessibleFault`, `ReplicationSubnetGroupDoesNotCoverEnoughAZs`, `ResourceNotFoundFault` | Modifies an existing DMS Serverless replication configuration that you can use to start a replication. This command includes input validation and logic to check the state of any replication that uses this configurati ... |
| `ModifyReplicationInstance` | `-` | - | `ReplicationInstanceArn` | - | `ModifyReplicationInstanceResponse` | `AccessDeniedFault`, `InsufficientResourceCapacityFault`, `InvalidResourceStateFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `StorageQuotaExceededFault`, `UpgradeDependencyFailureFault` | Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request. Some settings are applied during the maintenance window. |
| `ModifyReplicationSubnetGroup` | `-` | - | `ReplicationSubnetGroupIdentifier`, `SubnetIds` | - | `ModifyReplicationSubnetGroupResponse` | `AccessDeniedFault`, `InvalidSubnet`, `ReplicationSubnetGroupDoesNotCoverEnoughAZs`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `SubnetAlreadyInUse` | Modifies the settings for the specified replication subnet group. |
| `ModifyReplicationTask` | `-` | - | `ReplicationTaskArn` | - | `ModifyReplicationTaskResponse` | `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault` | Modifies the specified replication task. You can't modify the task endpoints. The task must be stopped before you can modify it. For more information about DMS tasks, see Working with Migration Tasks in the Database ... |
| `MoveReplicationTask` | `-` | - | `ReplicationTaskArn`, `TargetReplicationInstanceArn` | - | `MoveReplicationTaskResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Moves a replication task from its current replication instance to a different target replication instance using the specified parameters. The target replication instance must be created with the same or later DMS ver ... |
| `RebootReplicationInstance` | `-` | - | `ReplicationInstanceArn` | - | `RebootReplicationInstanceResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Reboots a replication instance. Rebooting results in a momentary outage, until the replication instance becomes available again. |
| `RefreshSchemas` | `-` | - | `EndpointArn`, `ReplicationInstanceArn` | - | `RefreshSchemasResponse` | `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation. |
| `ReloadReplicationTables` | `-` | - | `ReplicationConfigArn`, `TablesToReload` | - | `ReloadReplicationTablesResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Reloads the target database table with the source data for a given DMS Serverless replication configuration. You can only use this operation with a task in the RUNNING state, otherwise the service will throw an Inval ... |
| `ReloadTables` | `-` | - | `ReplicationTaskArn`, `TablesToReload` | - | `ReloadTablesResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Reloads the target database table with the source data. You can only use this operation with a task in the RUNNING state, otherwise the service will throw an InvalidResourceStateFault exception. |
| `RemoveTagsFromResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `RemoveTagsFromResourceResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Removes metadata tags from an DMS resource, including replication instance, endpoint, subnet group, and migration task. For more information, see Tag data type description. |
| `RunFleetAdvisorLsaAnalysis` | `-` | - | - | - | `RunFleetAdvisorLsaAnalysisResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `StartDataMigration` | `-` | - | `DataMigrationIdentifier`, `StartType` | - | `StartDataMigrationResponse` | `FailedDependencyFault`, `InvalidOperationFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Starts the specified data migration. |
| `StartExtensionPackAssociation` | `-` | - | `MigrationProjectIdentifier` | - | `StartExtensionPackAssociationResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Applies the extension pack to your target database. An extension pack is an add-on module that emulates functions present in a source database that are required when converting objects to the target database. |
| `StartMetadataModelAssessment` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules` | - | `StartMetadataModelAssessmentResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Creates a database migration assessment report by assessing the migration complexity for your source database. A database migration assessment report summarizes all of the schema conversion tasks. It also details the ... |
| `StartMetadataModelConversion` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules` | - | `StartMetadataModelConversionResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Converts your source database objects to a format compatible with the target database. |
| `StartMetadataModelCreation` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules`, `MetadataModelName`, `Properties` | - | `StartMetadataModelCreationResponse` | `AccessDeniedFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Creates source metadata model of the given type with the specified properties for schema conversion operations. This action supports only these directions: from SQL Server to Aurora PostgreSQL, or from SQL Server to ... |
| `StartMetadataModelExportAsScript` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules`, `Origin` | - | `StartMetadataModelExportAsScriptResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Saves your converted code to a file as a SQL script, and stores this file on your Amazon S3 bucket. |
| `StartMetadataModelExportToTarget` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules` | - | `StartMetadataModelExportToTargetResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Applies converted database objects to your target database. |
| `StartMetadataModelImport` | `-` | - | `MigrationProjectIdentifier`, `SelectionRules`, `Origin` | - | `StartMetadataModelImportResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Loads the metadata for all the dependent database objects of the parent object. This operation uses your project's Amazon S3 bucket as a metadata cache to improve performance. |
| `StartRecommendations` | `-` | - | `DatabaseId`, `Settings` | - | `Unit` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advi ... |
| `StartReplication` | `-` | - | `ReplicationConfigArn`, `StartReplicationType` | - | `StartReplicationResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | For a given DMS Serverless replication configuration, DMS connects to the source endpoint and collects the metadata to analyze the replication workload. Using this metadata, DMS then computes and provisions the requi ... |
| `StartReplicationTask` | `-` | - | `ReplicationTaskArn`, `StartReplicationTaskType` | - | `StartReplicationTaskResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Starts the replication task. For more information about DMS tasks, see Working with Migration Tasks in the Database Migration Service User Guide. |
| `StartReplicationTaskAssessment` | `-` | - | `ReplicationTaskArn` | - | `StartReplicationTaskAssessmentResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Starts the replication task assessment for unsupported data types in the source database. You can only use this operation for a task if the following conditions are true: The task must be in the stopped state. The ta ... |
| `StartReplicationTaskAssessmentRun` | `-` | - | `ReplicationTaskArn`, `ServiceAccessRoleArn`, `ResultLocationBucket`, `AssessmentRunName` | - | `StartReplicationTaskAssessmentRunResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSAccessDeniedFault`, `KMSDisabledFault`, `KMSFault`, `KMSInvalidStateFault`, `KMSKeyNotAccessibleFault`, `KMSNotFoundFault`, `ResourceAlreadyExistsFault`, `ResourceNotFoundFault`, `S3AccessDeniedFault`, `S3ResourceNotFoundFault` | Starts a new premigration assessment run for one or more individual assessments of a migration task. The assessments that you can specify depend on the source and target database engine and the migration type defined ... |
| `StopDataMigration` | `-` | - | `DataMigrationIdentifier` | - | `StopDataMigrationResponse` | `FailedDependencyFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | Stops the specified data migration. |
| `StopReplication` | `-` | - | `ReplicationConfigArn` | - | `StopReplicationResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `ResourceNotFoundFault` | For a given DMS Serverless replication configuration, DMS stops any and all ongoing DMS Serverless replications. This command doesn't deprovision the stopped replications. |
| `StopReplicationTask` | `-` | - | `ReplicationTaskArn` | - | `StopReplicationTaskResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Stops the replication task. |
| `TestConnection` | `-` | - | `ReplicationInstanceArn`, `EndpointArn` | - | `TestConnectionResponse` | `AccessDeniedFault`, `InvalidResourceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceNotFoundFault`, `ResourceQuotaExceededFault` | Tests the connection between the replication instance and the endpoint. |
| `UpdateSubscriptionsToEventBridge` | `-` | - | - | - | `UpdateSubscriptionsToEventBridgeResponse` | `AccessDeniedFault`, `InvalidResourceStateFault` | Migrates 10 active and enabled Amazon SNS subscriptions at a time and converts them to corresponding Amazon EventBridge rules. By default, this operation migrates subscriptions only when all your replication instance ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedFault` | `structure` | message | DMS was denied access to the endpoint. Check that the role is correctly configured. |
| `CollectorNotFoundFault` | `structure` | message | The specified collector doesn't exist. |
| `FailedDependencyFault` | `structure` | message | A dependency threw an exception. |
| `InsufficientResourceCapacityFault` | `structure` | message | There are not enough resources allocated to the database migration. |
| `InvalidCertificateFault` | `structure` | message | The certificate was not valid. |
| `InvalidOperationFault` | `structure` | message | The action or operation requested isn't valid. |
| `InvalidResourceStateFault` | `structure` | message | The resource is in a state that prevents it from being used for database migration. |
| `InvalidSubnet` | `structure` | message | The subnet provided isn't valid. |
| `KMSAccessDeniedFault` | `structure` | message | The ciphertext references a key that doesn't exist or that the DMS account doesn't have access to. |
| `KMSDisabledFault` | `structure` | message | The specified KMS key isn't enabled. |
| `KMSFault` | `structure` | message | An Key Management Service (KMS) error is preventing access to KMS. |
| `KMSInvalidStateFault` | `structure` | message | The state of the specified KMS resource isn't valid for this request. |
| `KMSKeyNotAccessibleFault` | `structure` | message | DMS cannot access the KMS key. |
| `KMSNotFoundFault` | `structure` | message | The specified KMS entity or resource can't be found. |
| `KMSThrottlingFault` | `structure` | message | This request triggered KMS request throttling. |
| `ReplicationSubnetGroupDoesNotCoverEnoughAZs` | `structure` | message | The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs. |
| `ResourceAlreadyExistsFault` | `structure` | message, resourceArn | The resource you are attempting to create already exists. |
| `ResourceNotFoundFault` | `structure` | message | The resource could not be found. |
| `ResourceQuotaExceededFault` | `structure` | message | The quota for this resource quota has been exceeded. |
| `S3AccessDeniedFault` | `structure` | message | Insufficient privileges are preventing access to an Amazon S3 object. |
| `S3ResourceNotFoundFault` | `structure` | message | A specified Amazon S3 bucket, bucket folder, or other object can't be found. |
| `SNSInvalidTopicFault` | `structure` | message | The SNS topic is invalid. |
| `SNSNoAuthorizationFault` | `structure` | message | You are not authorized for the SNS subscription. |
| `StorageQuotaExceededFault` | `structure` | message | The storage quota has been exceeded. |
| `SubnetAlreadyInUse` | `structure` | message | The specified subnet is already in use. |
| `UpgradeDependencyFailureFault` | `structure` | message | An upgrade dependency is preventing the database migration. |
| `AddTagsToResourceMessage` | `structure` | ResourceArn, Tags | Associates a set of tags with an DMS resource. |
| `AddTagsToResourceResponse` | `structure` | **empty (no members)** | - |
| `ApplyPendingMaintenanceActionMessage` | `structure` | ReplicationInstanceArn, ApplyAction, OptInType | - |
| `ApplyPendingMaintenanceActionResponse` | `structure` | ResourcePendingMaintenanceActions | - |
| `BatchStartRecommendationsRequest` | `structure` | Data | - |
| `BatchStartRecommendationsResponse` | `structure` | ErrorEntries | - |
| `CancelMetadataModelConversionMessage` | `structure` | MigrationProjectIdentifier, RequestIdentifier | - |
| `CancelMetadataModelConversionResponse` | `structure` | Request | - |
| `CancelMetadataModelCreationMessage` | `structure` | MigrationProjectIdentifier, RequestIdentifier | - |
| `CancelMetadataModelCreationResponse` | `structure` | Request | - |
| `CancelReplicationTaskAssessmentRunMessage` | `structure` | ReplicationTaskAssessmentRunArn | - |
| `CancelReplicationTaskAssessmentRunResponse` | `structure` | ReplicationTaskAssessmentRun | - |
| `CreateDataMigrationMessage` | `structure` | DataMigrationName, MigrationProjectIdentifier, DataMigrationType, ServiceAccessRoleArn, EnableCloudwatchLogs, SourceDataSettings, TargetDataSettings, NumberOfJobs, Tags, SelectionRules | - |
| `CreateDataMigrationResponse` | `structure` | DataMigration | - |
| `AssessmentReportType` | `enum` | PDF, CSV | - |
| `AuthMechanismValue` | `enum` | DEFAULT, MONGODB_CR, SCRAM_SHA_1 | - |
| `AuthTypeValue` | `enum` | NO, PASSWORD | - |
| `CannedAclForObjectsValue` | `enum` | NONE, PRIVATE, PUBLIC_READ, PUBLIC_READ_WRITE, AUTHENTICATED_READ, AWS_EXEC_READ, BUCKET_OWNER_READ, BUCKET_OWNER_FULL_CONTROL | - |
| `CharLengthSemantics` | `enum` | DEFAULT, CHAR, BYTE | - |
| `CollectorStatus` | `enum` | UNREGISTERED, ACTIVE | - |
| `CompressionTypeValue` | `enum` | NONE, GZIP | - |
| `DataFormatValue` | `enum` | CSV, PARQUET | - |
| `DatabaseMode` | `enum` | DEFAULT, BABELFISH | - |
| `DatePartitionDelimiterValue` | `enum` | SLASH, UNDERSCORE, DASH, NONE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

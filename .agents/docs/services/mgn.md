# Application Migration Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Application Migration Service service.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Application Migration Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Application Migration Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Application Migration Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Application Migration Service workflows in the local mock. Key resources include `AccountResource`, `ApplianceResource`, `ApplicationResource`, `ConnectorResource`, `ExportResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Start`, `Update`, `Delete`, `Create` operation families, including `ListApplications`, `ListConnectors`, `ListExportErrors`, `ListExports`, `StartCutover`, `StartExport`.

## Service Identity and Protocol

- AWS model slug: `mgn`
- AWS SDK for Rust slug: `mgn`
- Model version: `2020-02-26`
- Model file: `vendor/api-models-aws/models/mgn/service/2020-02-26/mgn-2020-02-26.json`
- SDK ID: `mgn`
- Endpoint prefix: `-`
- ARN namespace: `mgn`
- CloudFormation name: `-`
- CloudTrail event source: `mgn.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (24), `Start` (11), `Update` (11), `Delete` (9), `Create` (6), `Describe` (6), `Get` (4), `Archive` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateApplications`, `AssociateSourceServers`, `CreateApplication`, `CreateConnector`, `CreateLaunchConfigurationTemplate`, `CreateNetworkMigrationDefinition`, `CreateReplicationConfigurationTemplate`, `CreateWave`, `DeleteApplication`, `DeleteConnector`, `DeleteJob`, `DeleteLaunchConfigurationTemplate`, `DeleteNetworkMigrationDefinition`, `DeleteReplicationConfigurationTemplate`, `DeleteSourceServer`, `DeleteVcenterClient`, `DeleteWave`, `DisassociateApplications`, `DisassociateSourceServers`, `PutSourceServerAction`, ... (+29).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeJobLogItems`, `DescribeJobs`, `DescribeLaunchConfigurationTemplates`, `DescribeReplicationConfigurationTemplates`, `DescribeSourceServers`, `DescribeVcenterClients`, `GetLaunchConfiguration`, `GetNetworkMigrationDefinition`, `GetNetworkMigrationMapperSegmentConstruct`, `GetReplicationConfiguration`, `ListApplications`, `ListConnectors`, `ListExportErrors`, `ListExports`, `ListImportErrors`, `ListImportFileEnrichments`, `ListImports`, `ListManagedAccounts`, `ListNetworkMigrationAnalyses`, `ListNetworkMigrationAnalysisResults`, ... (+14).
- Pagination is modelled for 29 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 34 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteJob`, `DescribeJobLogItems`, `DescribeJobs`, `ListExportErrors`, `ListExports`, `ListImportErrors`, `ListImportFileEnrichments`, `ListImports`, `ListNetworkMigrationAnalysisResults`, `ListNetworkMigrationExecutions`, `StartCutover`, `StartExport`, `StartImport`, `StartImportFileEnrichment`, `StartNetworkMigrationAnalysis`, `StartNetworkMigrationCodeGeneration`, `StartNetworkMigrationDeployment`, `StartNetworkMigrationMapping`, `StartNetworkMigrationMappingUpdate`, `StartReplication`, ... (+2).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 95 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccountResource` | `accountID` | - | - | - |
| `ApplianceResource` | `applianceID` | - | - | - |
| `ApplicationResource` | `applicationID` | create: `CreateApplication`; delete: `DeleteApplication`; list: `ListApplications` | `ArchiveApplication`, `AssociateSourceServers`, `DisassociateSourceServers`, `UnarchiveApplication`, `UpdateApplication` | - |
| `ConnectorResource` | `connectorID` | create: `CreateConnector`; update: `UpdateConnector`; delete: `DeleteConnector`; list: `ListConnectors` | - | - |
| `ExportResource` | `exportID` | create: `StartExport`; list: `ListExports` | `ListExportErrors` | - |
| `ImportResource` | `importID` | create: `StartImport`; list: `ListImports` | `ListImportErrors` | - |
| `JobResource` | `jobID` | delete: `DeleteJob`; list: `DescribeJobs` | `DescribeJobLogItems` | - |
| `LaunchConfigurationTemplateResource` | `launchConfigurationTemplateID` | create: `CreateLaunchConfigurationTemplate`; update: `UpdateLaunchConfigurationTemplate`; delete: `DeleteLaunchConfigurationTemplate`; list: `DescribeLaunchConfigurationTemplates` | `ListTemplateActions`, `PutTemplateAction`, `RemoveTemplateAction` | - |
| `NetworkMigrationDefinitionResource` | `networkMigrationDefinitionID` | create: `CreateNetworkMigrationDefinition`; update: `UpdateNetworkMigrationDefinition`; delete: `DeleteNetworkMigrationDefinition`; list: `ListNetworkMigrationDefinitions` | `GetNetworkMigrationDefinition`, `GetNetworkMigrationMapperSegmentConstruct`, `ListNetworkMigrationAnalyses`, `ListNetworkMigrationAnalysisResults`, `ListNetworkMigrationCodeGenerationSegments`, `ListNetworkMigrationCodeGenerations`, `ListNetworkMigrationDeployedStacks`, `ListNetworkMigrationDeployments`, `ListNetworkMigrationExecutions`, `ListNetworkMigrationMapperSegmentConstructs`, ... (+9) | - |
| `ReplicationConfigurationTemplateResource` | `replicationConfigurationTemplateID` | create: `CreateReplicationConfigurationTemplate`; update: `UpdateReplicationConfigurationTemplate`; delete: `DeleteReplicationConfigurationTemplate`; list: `DescribeReplicationConfigurationTemplates` | - | - |
| `SourceServerResource` | `sourceServerID` | update: `UpdateSourceServer`; delete: `DeleteSourceServer`; list: `DescribeSourceServers` | `StartCutover`, `StartTest`, `TerminateTargetInstances`, `ChangeServerLifeCycleState`, `DisconnectFromService`, `FinalizeCutover`, `GetLaunchConfiguration`, `GetReplicationConfiguration`, `ListSourceServerActions`, `MarkAsArchived`, ... (+10) | - |
| `VcenterClientResource` | `vcenterClientID` | delete: `DeleteVcenterClient`; list: `DescribeVcenterClients` | - | - |
| `WaveResource` | `waveID` | create: `CreateWave`; delete: `DeleteWave`; list: `ListWaves` | `ArchiveWave`, `AssociateApplications`, `DisassociateApplications`, `UnarchiveWave`, `UpdateWave` | - |
## Operation Groups

### List

- Operations: `ListApplications`, `ListConnectors`, `ListExportErrors`, `ListExports`, `ListImportErrors`, `ListImportFileEnrichments`, `ListImports`, `ListManagedAccounts`, `ListNetworkMigrationAnalyses`, `ListNetworkMigrationAnalysisResults`, `ListNetworkMigrationCodeGenerationSegments`, `ListNetworkMigrationCodeGenerations`, `ListNetworkMigrationDefinitions`, `ListNetworkMigrationDeployedStacks`, `ListNetworkMigrationDeployments`, `ListNetworkMigrationExecutions`, `ListNetworkMigrationMapperSegmentConstructs`, `ListNetworkMigrationMapperSegments`, `ListNetworkMigrationMappingUpdates`, `ListNetworkMigrationMappings`, `ListSourceServerActions`, `ListTagsForResource`, `ListTemplateActions`, `ListWaves`
- Traits: `paginated` (23), `readonly` (24)
- Common required input members in this group: `exportID`, `importID`, `launchConfigurationTemplateID`, `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `resourceArn`, `segmentID`, `sourceServerID`

### Start

- Operations: `StartCutover`, `StartExport`, `StartImport`, `StartImportFileEnrichment`, `StartNetworkMigrationAnalysis`, `StartNetworkMigrationCodeGeneration`, `StartNetworkMigrationDeployment`, `StartNetworkMigrationMapping`, `StartNetworkMigrationMappingUpdate`, `StartReplication`, `StartTest`
- Traits: `idempotency-token` (2), `idempotent` (6)
- Common required input members in this group: `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `s3Bucket`, `s3BucketSource`, `s3BucketTarget`, `s3Key`, `sourceServerID`, `sourceServerIDs`

### Update

- Operations: `UpdateApplication`, `UpdateConnector`, `UpdateLaunchConfiguration`, `UpdateLaunchConfigurationTemplate`, `UpdateNetworkMigrationDefinition`, `UpdateNetworkMigrationMapperSegment`, `UpdateReplicationConfiguration`, `UpdateReplicationConfigurationTemplate`, `UpdateSourceServer`, `UpdateSourceServerReplicationType`, `UpdateWave`
- Traits: `idempotent` (8)
- Common required input members in this group: `applicationID`, `connectorID`, `launchConfigurationTemplateID`, `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `replicationConfigurationTemplateID`, `replicationType`, `segmentID`, `sourceServerID`, `waveID`

### Delete

- Operations: `DeleteApplication`, `DeleteConnector`, `DeleteJob`, `DeleteLaunchConfigurationTemplate`, `DeleteNetworkMigrationDefinition`, `DeleteReplicationConfigurationTemplate`, `DeleteSourceServer`, `DeleteVcenterClient`, `DeleteWave`
- Traits: `idempotent` (9)
- Common required input members in this group: `applicationID`, `connectorID`, `jobID`, `launchConfigurationTemplateID`, `networkMigrationDefinitionID`, `replicationConfigurationTemplateID`, `sourceServerID`, `vcenterClientID`, `waveID`

### Create

- Operations: `CreateApplication`, `CreateConnector`, `CreateLaunchConfigurationTemplate`, `CreateNetworkMigrationDefinition`, `CreateReplicationConfigurationTemplate`, `CreateWave`
- Traits: `idempotent` (4)
- Common required input members in this group: `associateDefaultSecurityGroup`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `name`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, `ssmInstanceID`, `stagingAreaSubnetId`, `stagingAreaTags`, `targetNetwork`, `targetS3Configuration`, `useDedicatedReplicationServer`

### Describe

- Operations: `DescribeJobLogItems`, `DescribeJobs`, `DescribeLaunchConfigurationTemplates`, `DescribeReplicationConfigurationTemplates`, `DescribeSourceServers`, `DescribeVcenterClients`
- Traits: `paginated` (6), `readonly` (6)
- Common required input members in this group: `jobID`

### Get

- Operations: `GetLaunchConfiguration`, `GetNetworkMigrationDefinition`, `GetNetworkMigrationMapperSegmentConstruct`, `GetReplicationConfiguration`
- Traits: `readonly` (4)
- Common required input members in this group: `constructID`, `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `segmentID`, `sourceServerID`

### Archive

- Operations: `ArchiveApplication`, `ArchiveWave`
- Common required input members in this group: `applicationID`, `waveID`

### Associate

- Operations: `AssociateApplications`, `AssociateSourceServers`
- Traits: `idempotent` (2)
- Common required input members in this group: `applicationID`, `applicationIDs`, `sourceServerIDs`, `waveID`

### Disassociate

- Operations: `DisassociateApplications`, `DisassociateSourceServers`
- Traits: `idempotent` (2)
- Common required input members in this group: `applicationID`, `applicationIDs`, `sourceServerIDs`, `waveID`

### Put

- Operations: `PutSourceServerAction`, `PutTemplateAction`
- Common required input members in this group: `actionID`, `actionName`, `documentIdentifier`, `launchConfigurationTemplateID`, `order`, `sourceServerID`

### Remove

- Operations: `RemoveSourceServerAction`, `RemoveTemplateAction`
- Common required input members in this group: `actionID`, `launchConfigurationTemplateID`, `sourceServerID`

### Unarchive

- Operations: `UnarchiveApplication`, `UnarchiveWave`
- Common required input members in this group: `applicationID`, `waveID`

### Change

- Operations: `ChangeServerLifeCycleState`
- Common required input members in this group: `lifeCycle`, `sourceServerID`

### Disconnect

- Operations: `DisconnectFromService`
- Common required input members in this group: `sourceServerID`

### Finalize

- Operations: `FinalizeCutover`
- Common required input members in this group: `sourceServerID`

### Initialize

- Operations: `InitializeService`

### Mark

- Operations: `MarkAsArchived`
- Common required input members in this group: `sourceServerID`

### Pause

- Operations: `PauseReplication`
- Common required input members in this group: `sourceServerID`

### Resume

- Operations: `ResumeReplication`
- Common required input members in this group: `sourceServerID`

### Retry

- Operations: `RetryDataReplication`
- Common required input members in this group: `sourceServerID`

### Stop

- Operations: `StopReplication`
- Common required input members in this group: `sourceServerID`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Terminate

- Operations: `TerminateTargetInstances`
- Common required input members in this group: `sourceServerIDs`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ArchiveApplication` | `POST /ArchiveApplication` | - | `applicationID` | - | `Application` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Archive application. |
| `ArchiveWave` | `POST /ArchiveWave` | - | `waveID` | - | `Wave` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Archive wave. |
| `AssociateApplications` | `POST /AssociateApplications` | `idempotent` | `applicationIDs`, `waveID` | - | `AssociateApplicationsResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Associate applications to wave. |
| `AssociateSourceServers` | `POST /AssociateSourceServers` | `idempotent` | `applicationID`, `sourceServerIDs` | - | `AssociateSourceServersResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Associate source servers to application. |
| `ChangeServerLifeCycleState` | `POST /ChangeServerLifeCycleState` | - | `lifeCycle`, `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Allows the user to set the SourceServer.LifeCycle.state property for specific Source Server IDs to one of the following: READY_FOR_TEST or READY_FOR_CUTOVER. This command only works if the Source Server is already launchable (dataReplicationInfo.lagDuration... |
| `CreateApplication` | `POST /CreateApplication` | `idempotent` | `name` | - | `Application` | `ConflictException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Create application. |
| `CreateConnector` | `POST /CreateConnector` | `idempotent` | `name`, `ssmInstanceID` | - | `Connector` | `UninitializedAccountException`, `ValidationException` | Create Connector. |
| `CreateLaunchConfigurationTemplate` | `POST /CreateLaunchConfigurationTemplate` | - | - | - | `LaunchConfigurationTemplate` | `AccessDeniedException`, `UninitializedAccountException`, `ValidationException` | Creates a new Launch Configuration Template. |
| `CreateNetworkMigrationDefinition` | `POST /network-migration/CreateNetworkMigrationDefinition` | `idempotent` | `name`, `targetNetwork`, `targetS3Configuration` | - | `NetworkMigrationDefinition` | `ServiceQuotaExceededException`, `ValidationException` | Creates a new network migration definition that specifies the source and target network configuration for a migration. |
| `CreateReplicationConfigurationTemplate` | `POST /CreateReplicationConfigurationTemplate` | - | `associateDefaultSecurityGroup`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, `stagingAreaSubnetId`, `stagingAreaTags`, ... (+1) | - | `ReplicationConfigurationTemplate` | `AccessDeniedException`, `UninitializedAccountException`, `ValidationException` | Creates a new ReplicationConfigurationTemplate. |
| `CreateWave` | `POST /CreateWave` | `idempotent` | `name` | - | `Wave` | `ConflictException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Create wave. |
| `DeleteApplication` | `POST /DeleteApplication` | `idempotent` | `applicationID` | - | `DeleteApplicationResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Delete application. |
| `DeleteConnector` | `POST /DeleteConnector` | `idempotent` | `connectorID` | - | `Unit` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Delete Connector. |
| `DeleteJob` | `POST /DeleteJob` | `idempotent` | `jobID` | - | `DeleteJobResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Deletes a single Job by ID. |
| `DeleteLaunchConfigurationTemplate` | `POST /DeleteLaunchConfigurationTemplate` | `idempotent` | `launchConfigurationTemplateID` | - | `DeleteLaunchConfigurationTemplateResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Deletes a single Launch Configuration Template by ID. |
| `DeleteNetworkMigrationDefinition` | `POST /network-migration/DeleteNetworkMigrationDefinition` | `idempotent` | `networkMigrationDefinitionID` | - | `DeleteNetworkMigrationDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException` | Deletes a network migration definition. This operation removes the migration definition and all associated metadata. |
| `DeleteReplicationConfigurationTemplate` | `POST /DeleteReplicationConfigurationTemplate` | `idempotent` | `replicationConfigurationTemplateID` | - | `DeleteReplicationConfigurationTemplateResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Deletes a single Replication Configuration Template by ID |
| `DeleteSourceServer` | `POST /DeleteSourceServer` | `idempotent` | `sourceServerID` | - | `DeleteSourceServerResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Deletes a single source server by ID. |
| `DeleteVcenterClient` | `POST /DeleteVcenterClient` | `idempotent` | `vcenterClientID` | - | `Unit` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Deletes a given vCenter client by ID. |
| `DeleteWave` | `POST /DeleteWave` | `idempotent` | `waveID` | - | `DeleteWaveResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Delete wave. |
| `DescribeJobLogItems` | `POST /DescribeJobLogItems` | `readonly`, `paginated` | `jobID` | - | `DescribeJobLogItemsResponse` | `UninitializedAccountException`, `ValidationException` | Retrieves detailed job log items with paging. |
| `DescribeJobs` | `POST /DescribeJobs` | `readonly`, `paginated` | - | - | `DescribeJobsResponse` | `UninitializedAccountException`, `ValidationException` | Returns a list of Jobs. Use the JobsID and fromDate and toData filters to limit which jobs are returned. |
| `DescribeLaunchConfigurationTemplates` | `POST /DescribeLaunchConfigurationTemplates` | `readonly`, `paginated` | - | - | `DescribeLaunchConfigurationTemplatesResponse` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Lists all Launch Configuration Templates, filtered by Launch Configuration Template IDs |
| `DescribeReplicationConfigurationTemplates` | `POST /DescribeReplicationConfigurationTemplates` | `readonly`, `paginated` | - | - | `DescribeReplicationConfigurationTemplatesResponse` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Lists all ReplicationConfigurationTemplates, filtered by Source Server IDs. |
| `DescribeSourceServers` | `POST /DescribeSourceServers` | `readonly`, `paginated` | - | - | `DescribeSourceServersResponse` | `UninitializedAccountException`, `ValidationException` | Retrieves all SourceServers or multiple SourceServers by ID. |
| `DescribeVcenterClients` | `GET /DescribeVcenterClients` | `readonly`, `paginated` | - | - | `DescribeVcenterClientsResponse` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Returns a list of the installed vCenter clients. |
| `DisassociateApplications` | `POST /DisassociateApplications` | `idempotent` | `applicationIDs`, `waveID` | - | `DisassociateApplicationsResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Disassociate applications from wave. |
| `DisassociateSourceServers` | `POST /DisassociateSourceServers` | `idempotent` | `applicationID`, `sourceServerIDs` | - | `DisassociateSourceServersResponse` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Disassociate source servers from application. |
| `DisconnectFromService` | `POST /DisconnectFromService` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Disconnects specific Source Servers from Application Migration Service. Data replication is stopped immediately. |
| `FinalizeCutover` | `POST /FinalizeCutover` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Finalizes the cutover immediately for specific Source Servers. All AWS resources created by Application Migration Service for enabling the replication of these source servers will be terminated / deleted within 90 minutes. |
| `GetLaunchConfiguration` | `POST /GetLaunchConfiguration` | `readonly` | `sourceServerID` | - | `LaunchConfiguration` | `ResourceNotFoundException`, `UninitializedAccountException` | Lists all LaunchConfigurations available, filtered by Source Server IDs. |
| `GetNetworkMigrationDefinition` | `POST /network-migration/GetNetworkMigrationDefinition` | `readonly` | `networkMigrationDefinitionID` | - | `NetworkMigrationDefinition` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves the details of a network migration definition including source and target configurations. |
| `GetNetworkMigrationMapperSegmentConstruct` | `POST /network-migration/GetNetworkMigrationMapperSegmentConstruct` | `readonly` | `constructID`, `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `segmentID` | - | `GetNetworkMigrationMapperSegmentConstructResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves detailed information about a specific construct within a mapper segment, including its properties and configuration data. |
| `GetReplicationConfiguration` | `POST /GetReplicationConfiguration` | `readonly` | `sourceServerID` | - | `ReplicationConfiguration` | `ResourceNotFoundException`, `UninitializedAccountException` | Lists all ReplicationConfigurations, filtered by Source Server ID. |
| `InitializeService` | `POST /InitializeService` | - | - | - | `InitializeServiceResponse` | `AccessDeniedException`, `ValidationException` | Initialize Application Migration Service. |
| `ListApplications` | `POST /ListApplications` | `readonly`, `paginated` | - | - | `ListApplicationsResponse` | `UninitializedAccountException` | Retrieves all applications or multiple applications by ID. |
| `ListConnectors` | `POST /ListConnectors` | `readonly`, `paginated` | - | - | `ListConnectorsResponse` | `UninitializedAccountException`, `ValidationException` | List Connectors. |
| `ListExportErrors` | `POST /ListExportErrors` | `readonly`, `paginated` | `exportID` | - | `ListExportErrorsResponse` | `UninitializedAccountException`, `ValidationException` | List export errors. |
| `ListExports` | `POST /ListExports` | `readonly`, `paginated` | - | - | `ListExportsResponse` | `UninitializedAccountException` | List exports. |
| `ListImportErrors` | `POST /ListImportErrors` | `readonly`, `paginated` | `importID` | - | `ListImportErrorsResponse` | `UninitializedAccountException`, `ValidationException` | List import errors. |
| `ListImportFileEnrichments` | `POST /network-migration/ListImportFileEnrichments` | `readonly`, `paginated` | - | - | `ListImportFileEnrichmentsResponse` | `ValidationException` | Lists import file enrichment jobs with optional filtering by job IDs. |
| `ListImports` | `POST /ListImports` | `readonly`, `paginated` | - | - | `ListImportsResponse` | `UninitializedAccountException`, `ValidationException` | List imports. |
| `ListManagedAccounts` | `POST /ListManagedAccounts` | `readonly`, `paginated` | - | - | `ListManagedAccountsResponse` | `UninitializedAccountException`, `ValidationException` | List Managed Accounts. |
| `ListNetworkMigrationAnalyses` | `POST /network-migration/ListNetworkMigrationAnalyses` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationAnalysesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists network migration analysis jobs for a specified execution. Returns information about analysis job status and results. |
| `ListNetworkMigrationAnalysisResults` | `POST /network-migration/ListNetworkMigrationAnalysisResults` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationAnalysisResultsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the results of network migration analyses, showing connectivity and compatibility findings for migrated resources. |
| `ListNetworkMigrationCodeGenerationSegments` | `POST /network-migration/ListNetworkMigrationCodeGenerationSegments` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationCodeGenerationSegmentsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists code generation segments, which represent individual infrastructure components generated as code templates. |
| `ListNetworkMigrationCodeGenerations` | `POST /network-migration/ListNetworkMigrationCodeGenerations` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationCodeGenerationsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists network migration code generation jobs, which convert network mappings into infrastructure-as-code templates. |
| `ListNetworkMigrationDefinitions` | `POST /network-migration/ListNetworkMigrationDefinitions` | `readonly`, `paginated` | - | - | `ListNetworkMigrationDefinitionsResponse` | `AccessDeniedException` | Lists all network migration definitions in the account, with optional filtering. |
| `ListNetworkMigrationDeployedStacks` | `POST /network-migration/ListNetworkMigrationDeployedStacks` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationDeployedStacksResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists CloudFormation stacks that have been deployed as part of the network migration. |
| `ListNetworkMigrationDeployments` | `POST /network-migration/ListNetworkMigrationDeployments` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationDeployerJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists network migration deployment jobs and their current status. |
| `ListNetworkMigrationExecutions` | `POST /network-migration/ListNetworkMigrationExecutions` | `readonly`, `paginated` | `networkMigrationDefinitionID` | - | `ListNetworkMigrationExecutionsResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Lists network migration execution instances for a given definition, showing the status and progress of each execution. |
| `ListNetworkMigrationMapperSegmentConstructs` | `POST /network-migration/ListNetworkMigrationMapperSegmentConstructs` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `segmentID` | - | `ListNetworkMigrationMapperSegmentConstructsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists constructs within a mapper segment, representing individual infrastructure components like VPCs, subnets, or security groups. |
| `ListNetworkMigrationMapperSegments` | `POST /network-migration/ListNetworkMigrationMapperSegments` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationMapperSegmentsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists mapper segments, which represent logical groupings of network resources to be migrated together. |
| `ListNetworkMigrationMappingUpdates` | `POST /network-migration/ListNetworkMigrationMappingUpdates` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationMappingUpdatesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists mapping update jobs, which apply customer modifications to the generated network mappings. |
| `ListNetworkMigrationMappings` | `POST /network-migration/ListNetworkMigrationMappings` | `readonly`, `paginated` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `ListNetworkMigrationMappingsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists network migration mapping jobs, which analyze and create relationships between source and target network resources. |
| `ListSourceServerActions` | `POST /ListSourceServerActions` | `readonly`, `paginated` | `sourceServerID` | - | `ListSourceServerActionsResponse` | `ResourceNotFoundException`, `UninitializedAccountException` | List source server post migration custom actions. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags for your Application Migration Service resources. |
| `ListTemplateActions` | `POST /ListTemplateActions` | `readonly`, `paginated` | `launchConfigurationTemplateID` | - | `ListTemplateActionsResponse` | `ResourceNotFoundException`, `UninitializedAccountException` | List template post migration custom actions. |
| `ListWaves` | `POST /ListWaves` | `readonly`, `paginated` | - | - | `ListWavesResponse` | `UninitializedAccountException` | Retrieves all waves or multiple waves by ID. |
| `MarkAsArchived` | `POST /MarkAsArchived` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Archives specific Source Servers by setting the SourceServer.isArchived property to true for specified SourceServers by ID. This command only works for SourceServers with a lifecycle. |
| `PauseReplication` | `POST /PauseReplication` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException`, `ValidationException` | Pause Replication. |
| `PutSourceServerAction` | `POST /PutSourceServerAction` | - | `actionID`, `actionName`, `documentIdentifier`, `order`, `sourceServerID` | - | `SourceServerActionDocument` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Put source server post migration custom action. |
| `PutTemplateAction` | `POST /PutTemplateAction` | - | `actionID`, `actionName`, `documentIdentifier`, `launchConfigurationTemplateID`, `order` | - | `TemplateActionDocument` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Put template post migration custom action. |
| `RemoveSourceServerAction` | `POST /RemoveSourceServerAction` | - | `actionID`, `sourceServerID` | - | `RemoveSourceServerActionResponse` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Remove source server post migration custom action. |
| `RemoveTemplateAction` | `POST /RemoveTemplateAction` | - | `actionID`, `launchConfigurationTemplateID` | - | `RemoveTemplateActionResponse` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Remove template post migration custom action. |
| `ResumeReplication` | `POST /ResumeReplication` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException`, `ValidationException` | Resume Replication. |
| `RetryDataReplication` | `POST /RetryDataReplication` | - | `sourceServerID` | - | `SourceServer` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Causes the data replication initiation sequence to begin immediately upon next Handshake for specified SourceServer IDs, regardless of when the previous initiation started. This command will not work if the SourceServer is not stalled or is in a DISCONNECTED... |
| `StartCutover` | `POST /StartCutover` | - | `sourceServerIDs` | - | `StartCutoverResponse` | `ConflictException`, `UninitializedAccountException`, `ValidationException` | Launches a Cutover Instance for specific Source Servers. This command starts a LAUNCH job whose initiatedBy property is StartCutover and changes the SourceServer.lifeCycle.state property to CUTTING_OVER. |
| `StartExport` | `POST /StartExport` | - | `s3Bucket`, `s3Key` | - | `StartExportResponse` | `ServiceQuotaExceededException`, `UninitializedAccountException`, `ValidationException` | Start export. |
| `StartImport` | `POST /StartImport` | `idempotency-token` | `s3BucketSource` | `clientToken` | `StartImportResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException`, `ValidationException` | Start import. |
| `StartImportFileEnrichment` | `POST /network-migration/StartImportFileEnrichment` | `idempotent`, `idempotency-token` | `s3BucketSource`, `s3BucketTarget` | `clientToken` | `StartImportFileEnrichmentResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts an import file enrichment job to process and enrich network migration import files with additional metadata and IP assignment strategies. |
| `StartNetworkMigrationAnalysis` | `POST /network-migration/StartNetworkMigrationAnalysis` | `idempotent` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `StartNetworkMigrationAnalysisResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a network migration analysis job to evaluate connectivity and compatibility of the migration mappings. |
| `StartNetworkMigrationCodeGeneration` | `POST /network-migration/StartNetworkMigrationCodeGeneration` | `idempotent` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `StartNetworkMigrationCodeGenerationResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a code generation job to convert network migration mappings into infrastructure-as-code templates. |
| `StartNetworkMigrationDeployment` | `POST /network-migration/StartNetworkMigrationDeployment` | `idempotent` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `StartNetworkMigrationDeployerJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a deployment job to create the target network infrastructure based on the generated code templates. |
| `StartNetworkMigrationMapping` | `POST /network-migration/StartNetworkMigrationMapping` | `idempotent` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `StartNetworkMigrationMappingResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts the network migration mapping process for a given network migration execution. |
| `StartNetworkMigrationMappingUpdate` | `POST /network-migration/StartNetworkMigrationMappingUpdate` | `idempotent` | `networkMigrationDefinitionID`, `networkMigrationExecutionID` | - | `StartNetworkMigrationMappingUpdateResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a job to apply customer modifications to network migration mappings, such as changing properties. |
| `StartReplication` | `POST /StartReplication` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException`, `ValidationException` | Start replication for source server irrespective of its replication type. |
| `StartTest` | `POST /StartTest` | - | `sourceServerIDs` | - | `StartTestResponse` | `ConflictException`, `UninitializedAccountException`, `ValidationException` | Launches a Test Instance for specific Source Servers. This command starts a LAUNCH job whose initiatedBy property is StartTest and changes the SourceServer.lifeCycle.state property to TESTING. |
| `StopReplication` | `POST /StopReplication` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException`, `ValidationException` | Stop Replication. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or overwrites only the specified tags for the specified Application Migration Service resource or resources. When you specify an existing tag key, the value is overwritten with the new value. |
| `TerminateTargetInstances` | `POST /TerminateTargetInstances` | - | `sourceServerIDs` | - | `TerminateTargetInstancesResponse` | `ConflictException`, `UninitializedAccountException`, `ValidationException` | Starts a job that terminates specific launched EC2 Test and Cutover instances. This command will not work for any Source Server with a lifecycle.state of TESTING, CUTTING_OVER, or CUTOVER. |
| `UnarchiveApplication` | `POST /UnarchiveApplication` | - | `applicationID` | - | `Application` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Unarchive application. |
| `UnarchiveWave` | `POST /UnarchiveWave` | - | `waveID` | - | `Wave` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UninitializedAccountException` | Unarchive wave. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified set of tags from the specified set of Application Migration Service resources. |
| `UpdateApplication` | `POST /UpdateApplication` | `idempotent` | `applicationID` | - | `Application` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Update application. |
| `UpdateConnector` | `POST /UpdateConnector` | `idempotent` | `connectorID` | - | `Connector` | `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Update Connector. |
| `UpdateLaunchConfiguration` | `POST /UpdateLaunchConfiguration` | `idempotent` | `sourceServerID` | - | `LaunchConfiguration` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Updates multiple LaunchConfigurations by Source Server ID. bootMode valid values are `LEGACY_BIOS \| UEFI` |
| `UpdateLaunchConfigurationTemplate` | `POST /UpdateLaunchConfigurationTemplate` | - | `launchConfigurationTemplateID` | - | `LaunchConfigurationTemplate` | `AccessDeniedException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Updates an existing Launch Configuration Template by ID. |
| `UpdateNetworkMigrationDefinition` | `POST /network-migration/UpdateNetworkMigrationDefinition` | `idempotent` | `networkMigrationDefinitionID` | - | `NetworkMigrationDefinition` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates an existing network migration definition with new source or target configurations. |
| `UpdateNetworkMigrationMapperSegment` | `POST /network-migration/UpdateNetworkMigrationMapperSegment` | `idempotent` | `networkMigrationDefinitionID`, `networkMigrationExecutionID`, `segmentID` | - | `NetworkMigrationMapperSegment` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates a mapper segment's configuration, such as changing its scope tags. |
| `UpdateReplicationConfiguration` | `POST /UpdateReplicationConfiguration` | `idempotent` | `sourceServerID` | - | `ReplicationConfiguration` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Allows you to update multiple ReplicationConfigurations by Source Server ID. |
| `UpdateReplicationConfigurationTemplate` | `POST /UpdateReplicationConfigurationTemplate` | - | `replicationConfigurationTemplateID` | - | `ReplicationConfigurationTemplate` | `AccessDeniedException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Updates multiple ReplicationConfigurationTemplates by ID. |
| `UpdateSourceServer` | `POST /UpdateSourceServer` | `idempotent` | `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Update Source Server. |
| `UpdateSourceServerReplicationType` | `POST /UpdateSourceServerReplicationType` | - | `replicationType`, `sourceServerID` | - | `SourceServer` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException`, `ValidationException` | Allows you to change between the AGENT_BASED replication type and the SNAPSHOT_SHIPPING replication type. SNAPSHOT_SHIPPING should be used for agentless replication. |
| `UpdateWave` | `POST /UpdateWave` | `idempotent` | `waveID` | - | `Wave` | `ConflictException`, `ResourceNotFoundException`, `UninitializedAccountException` | Update wave. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `code`, `message`, `resourceId`, `resourceType` | Resource not found exception. |
| `UninitializedAccountException` | `structure` | `code`, `message` | Uninitialized account exception. |
| `ValidationException` | `structure` | `code`, `fieldList`, `message`, `reason` | Validate exception. |
| `ConflictException` | `structure` | `code`, `errors`, `message`, `resourceId`, `resourceType` | The request could not be completed due to a conflict with the current state of the target resource. |
| `AccessDeniedException` | `structure` | `code`, `message` | Operating denied due to a file permission or access check error. |
| `ServiceQuotaExceededException` | `structure` | `code`, `message`, `quotaCode`, `quotaValue`, `resourceId`, `resourceType`, `serviceCode` | The request could not be completed because its exceeded the service quota. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | Reached throttling quota exception. |
| `SourceServer` | `structure` | `applicationID`, `arn`, `connectorAction`, `dataReplicationInfo`, `fqdnForActionFramework`, `isArchived`, `launchedInstance`, `lifeCycle`, `replicationType`, `sourceProperties`, `sourceServerID`, `tags`, ... (+2) | - |
| `Application` | `structure` | `applicationAggregatedStatus`, `applicationID`, `arn`, `creationDateTime`, `description`, `isArchived`, `lastModifiedDateTime`, `name`, `tags`, `waveID` | - |
| `Wave` | `structure` | `arn`, `creationDateTime`, `description`, `isArchived`, `lastModifiedDateTime`, `name`, `tags`, `waveAggregatedStatus`, `waveID` | - |
| `NetworkMigrationDefinition` | `structure` | `arn`, `createdAt`, `description`, `name`, `networkMigrationDefinitionID`, `scopeTags`, `sourceConfigurations`, `tags`, `targetDeployment`, `targetNetwork`, `targetS3Configuration`, `updatedAt` | - |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The server encountered an unexpected condition that prevented it from fulfilling the request. |
| `Connector` | `structure` | `arn`, `connectorID`, `name`, `ssmCommandConfig`, `ssmInstanceID`, `tags` | - |
| `LaunchConfigurationTemplate` | `structure` | `arn`, `associatePublicIpAddress`, `bootMode`, `copyPrivateIp`, `copyTags`, `ec2LaunchTemplateID`, `enableMapAutoTagging`, `enableParametersEncryption`, `largeVolumeConf`, `launchConfigurationTemplateID`, `launchDisposition`, `licensing`, ... (+7) | - |
| `ReplicationConfigurationTemplate` | `structure` | `arn`, `associateDefaultSecurityGroup`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `ebsEncryptionKeyArn`, `internetProtocol`, `replicationConfigurationTemplateID`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, ... (+6) | - |
| `LaunchConfiguration` | `structure` | `bootMode`, `copyPrivateIp`, `copyTags`, `ec2LaunchTemplateID`, `enableMapAutoTagging`, `launchDisposition`, `licensing`, `mapAutoTaggingMpeID`, `name`, `postLaunchActions`, `sourceServerID`, `targetInstanceTypeRightSizingMethod` | - |
| `ReplicationConfiguration` | `structure` | `associateDefaultSecurityGroup`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `ebsEncryptionKeyArn`, `internetProtocol`, `name`, `replicatedDisks`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, ... (+6) | - |
| `ArchiveApplicationRequest` | `structure` | `accountID`, `applicationID` | - |
| `ArchiveWaveRequest` | `structure` | `accountID`, `waveID` | - |
| `AssociateApplicationsRequest` | `structure` | `accountID`, `applicationIDs`, `waveID` | - |
| `AssociateApplicationsResponse` | `structure` | - | - |
| `AssociateSourceServersRequest` | `structure` | `accountID`, `applicationID`, `sourceServerIDs` | - |
| `AssociateSourceServersResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

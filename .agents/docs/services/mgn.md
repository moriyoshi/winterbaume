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

- Operations: `ListImportFileEnrichments`, `ListManagedAccounts`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Initialize

- Operations: `InitializeService`
- Common required input members in this group: -

### Start

- Operations: `StartImportFileEnrichment`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `InitializeService` | `POST /InitializeService` | - | - | - | `InitializeServiceResponse` | `AccessDeniedException`, `ValidationException` | Initialize Application Migration Service. |
| `ListImportFileEnrichments` | `POST /network-migration/ListImportFileEnrichments` | `readonly`, `paginated` | - | - | `ListImportFileEnrichmentsResponse` | `ValidationException` | Lists import file enrichment jobs with optional filtering by job IDs. |
| `ListManagedAccounts` | `POST /ListManagedAccounts` | `readonly`, `paginated` | - | - | `ListManagedAccountsResponse` | `UninitializedAccountException`, `ValidationException` | List Managed Accounts. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags for your Application Migration Service resources. |
| `StartImportFileEnrichment` | `POST /network-migration/StartImportFileEnrichment` | `idempotent`, `idempotency-token` | `s3BucketSource`, `s3BucketTarget` | `clientToken` | `StartImportFileEnrichmentResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts an import file enrichment job to process and enrich network migration import files with additional metadata and IP assignment strategies. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or overwrites only the specified tags for the specified Application Migration Service resource or resources. When you specify an existing tag key, the value is overwritten with the new value. Each resource can h ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified set of tags from the specified set of Application Migration Service resources. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, code | Operating denied due to a file permission or access check error. |
| `ConflictException` | `structure` | message, code, resourceId, resourceType, errors | The request could not be completed due to a conflict with the current state of the target resource. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | The server encountered an unexpected condition that prevented it from fulfilling the request. |
| `ResourceNotFoundException` | `structure` | message, code, resourceId, resourceType | Resource not found exception. |
| `ServiceQuotaExceededException` | `structure` | message, code, resourceId, resourceType, serviceCode, quotaCode, quotaValue | The request could not be completed because its exceeded the service quota. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | Reached throttling quota exception. |
| `UninitializedAccountException` | `structure` | message, code | Uninitialized account exception. |
| `ValidationException` | `structure` | message, code, reason, fieldList | Validate exception. |
| `InitializeServiceRequest` | `structure` | **empty (no members)** | - |
| `InitializeServiceResponse` | `structure` | **empty (no members)** | - |
| `ListImportFileEnrichmentsRequest` | `structure` | filters, maxResults, nextToken | - |
| `ListImportFileEnrichmentsResponse` | `structure` | items, nextToken | - |
| `ListManagedAccountsRequest` | `structure` | maxResults, nextToken | List managed accounts request. |
| `ListManagedAccountsResponse` | `structure` | items, nextToken | List managed accounts response. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `StartImportFileEnrichmentRequest` | `structure` | clientToken, s3BucketSource, s3BucketTarget, ipAssignmentStrategy | - |
| `StartImportFileEnrichmentResponse` | `structure` | jobID | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

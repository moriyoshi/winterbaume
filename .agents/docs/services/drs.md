# Elastic Disaster Recovery Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Elastic Disaster Recovery Service.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Elastic Disaster Recovery Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Elastic Disaster Recovery Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Elastic Disaster Recovery Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Elastic Disaster Recovery Service workflows in the local mock. Key resources include `AccountResource`, `JobResource`, `LaunchConfigurationTemplateResource`, `RecoveryInstanceResource`, `ReplicationConfigurationTemplateResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Delete`, `Start`, `Update`, `Create` operation families, including `DescribeJobLogItems`, `DescribeJobs`, `DescribeLaunchConfigurationTemplates`, `DescribeRecoveryInstances`, `DeleteJob`, `DeleteLaunchAction`.

## Service Identity and Protocol

- AWS model slug: `drs`
- AWS SDK for Rust slug: `drs`
- Model version: `2020-02-26`
- Model file: `vendor/api-models-aws/models/drs/service/2020-02-26/drs-2020-02-26.json`
- SDK ID: `drs`
- Endpoint prefix: `-`
- ARN namespace: `drs`
- CloudFormation name: `-`
- CloudTrail event source: `drs.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (8), `Delete` (7), `Start` (5), `Update` (5), `Create` (4), `List` (4), `Get` (3), `Stop` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateSourceNetworkStack`, `CreateExtendedSourceServer`, `CreateLaunchConfigurationTemplate`, `CreateReplicationConfigurationTemplate`, `CreateSourceNetwork`, `DeleteJob`, `DeleteLaunchAction`, `DeleteLaunchConfigurationTemplate`, `DeleteRecoveryInstance`, `DeleteReplicationConfigurationTemplate`, `DeleteSourceNetwork`, `DeleteSourceServer`, `PutLaunchAction`, `StartFailbackLaunch`, `StartRecovery`, `StartReplication`, `StartSourceNetworkRecovery`, `StartSourceNetworkReplication`, `StopFailback`, `StopReplication`, ... (+9).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeJobLogItems`, `DescribeJobs`, `DescribeLaunchConfigurationTemplates`, `DescribeRecoveryInstances`, `DescribeRecoverySnapshots`, `DescribeReplicationConfigurationTemplates`, `DescribeSourceNetworks`, `DescribeSourceServers`, `GetFailbackReplicationConfiguration`, `GetLaunchConfiguration`, `GetReplicationConfiguration`, `ListExtensibleSourceServers`, `ListLaunchActions`, `ListStagingAccounts`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteJob`, `DescribeJobLogItems`, `DescribeJobs`, `ExportSourceNetworkCfnTemplate`, `StartFailbackLaunch`, `StartRecovery`, `StartReplication`, `StartSourceNetworkRecovery`, `StartSourceNetworkReplication`, `StopFailback`, `StopReplication`, `StopSourceNetworkReplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 50 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccountResource` | `accountID` | - | - | - |
| `JobResource` | `jobID` | delete: `DeleteJob`; list: `DescribeJobs` | `DescribeJobLogItems` | - |
| `LaunchConfigurationTemplateResource` | `launchConfigurationTemplateID` | create: `CreateLaunchConfigurationTemplate`; update: `UpdateLaunchConfigurationTemplate`; delete: `DeleteLaunchConfigurationTemplate`; list: `DescribeLaunchConfigurationTemplates` | - | - |
| `RecoveryInstanceResource` | `recoveryInstanceID` | list: `DescribeRecoveryInstances` | `StartFailbackLaunch`, `TerminateRecoveryInstances`, `DeleteRecoveryInstance`, `DisconnectRecoveryInstance`, `GetFailbackReplicationConfiguration`, `ReverseReplication`, `StopFailback`, `UpdateFailbackReplicationConfiguration` | - |
| `ReplicationConfigurationTemplateResource` | `replicationConfigurationTemplateID` | create: `CreateReplicationConfigurationTemplate`; update: `UpdateReplicationConfigurationTemplate`; delete: `DeleteReplicationConfigurationTemplate`; list: `DescribeReplicationConfigurationTemplates` | - | - |
| `SourceNetworkResource` | `sourceNetworkID` | create: `CreateSourceNetwork`; delete: `DeleteSourceNetwork`; list: `DescribeSourceNetworks` | `StartSourceNetworkRecovery`, `AssociateSourceNetworkStack`, `ExportSourceNetworkCfnTemplate`, `StartSourceNetworkReplication`, `StopSourceNetworkReplication` | - |
| `SourceServerResource` | `sourceServerID` | delete: `DeleteSourceServer`; list: `DescribeSourceServers` | `StartRecovery`, `DescribeRecoverySnapshots`, `DisconnectSourceServer`, `GetLaunchConfiguration`, `GetReplicationConfiguration`, `RetryDataReplication`, `StartReplication`, `StopReplication`, `UpdateLaunchConfiguration`, `UpdateReplicationConfiguration` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/drs/latest/userguide/getting-started.html
- https://docs.aws.amazon.com/drs/latest/userguide/source-servers.html
- https://docs.aws.amazon.com/drs/latest/userguide/individual-replication-settings.html

Research outcomes:
- Elastic Disaster Recovery replicates source servers to AWS and launches recovery instances for drills or failover.
- Source servers are added by installing the AWS Replication Agent.
- Replication settings control staging area, replication servers, storage, network, and cost-related behaviour.
- Source servers expose replication status, recovery readiness, launch settings, and post-launch settings.
- Failover, recovery drills, and failback are separate operational phases.
- DRS can consolidate replication servers for source servers with identical replication settings.

Parity implications:
- Model source servers, replication settings, staging resources, launch settings, recovery instances, jobs, recovery snapshots, and failback state separately.
- Agent installation should create or update source-server records and begin replication state.
- Launch/recovery operations should be asynchronous and should not mutate source-server replication configuration directly.

## Operation Groups

### Describe

- Operations: `DescribeJobLogItems`, `DescribeJobs`, `DescribeLaunchConfigurationTemplates`, `DescribeRecoveryInstances`, `DescribeRecoverySnapshots`, `DescribeReplicationConfigurationTemplates`, `DescribeSourceNetworks`, `DescribeSourceServers`
- Traits: `paginated` (8), `readonly` (8)
- Common required input members in this group: `jobID`, `sourceServerID`

### Delete

- Operations: `DeleteJob`, `DeleteLaunchAction`, `DeleteLaunchConfigurationTemplate`, `DeleteRecoveryInstance`, `DeleteReplicationConfigurationTemplate`, `DeleteSourceNetwork`, `DeleteSourceServer`
- Traits: `idempotent` (5)
- Common required input members in this group: `actionId`, `jobID`, `launchConfigurationTemplateID`, `recoveryInstanceID`, `replicationConfigurationTemplateID`, `resourceId`, `sourceNetworkID`, `sourceServerID`

### Start

- Operations: `StartFailbackLaunch`, `StartRecovery`, `StartReplication`, `StartSourceNetworkRecovery`, `StartSourceNetworkReplication`
- Common required input members in this group: `recoveryInstanceIDs`, `sourceNetworkID`, `sourceNetworks`, `sourceServerID`, `sourceServers`

### Update

- Operations: `UpdateFailbackReplicationConfiguration`, `UpdateLaunchConfiguration`, `UpdateLaunchConfigurationTemplate`, `UpdateReplicationConfiguration`, `UpdateReplicationConfigurationTemplate`
- Traits: `idempotent` (2)
- Common required input members in this group: `launchConfigurationTemplateID`, `recoveryInstanceID`, `replicationConfigurationTemplateID`, `sourceServerID`

### Create

- Operations: `CreateExtendedSourceServer`, `CreateLaunchConfigurationTemplate`, `CreateReplicationConfigurationTemplate`, `CreateSourceNetwork`
- Common required input members in this group: `associateDefaultSecurityGroup`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `originAccountID`, `originRegion`, `pitPolicy`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, `sourceServerArn`, `stagingAreaSubnetId`, `stagingAreaTags`, `useDedicatedReplicationServer`, `vpcID`

### List

- Operations: `ListExtensibleSourceServers`, `ListLaunchActions`, `ListStagingAccounts`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `resourceArn`, `resourceId`, `stagingAccountID`

### Get

- Operations: `GetFailbackReplicationConfiguration`, `GetLaunchConfiguration`, `GetReplicationConfiguration`
- Traits: `readonly` (3)
- Common required input members in this group: `recoveryInstanceID`, `sourceServerID`

### Stop

- Operations: `StopFailback`, `StopReplication`, `StopSourceNetworkReplication`
- Common required input members in this group: `recoveryInstanceID`, `sourceNetworkID`, `sourceServerID`

### Disconnect

- Operations: `DisconnectRecoveryInstance`, `DisconnectSourceServer`
- Common required input members in this group: `recoveryInstanceID`, `sourceServerID`

### Associate

- Operations: `AssociateSourceNetworkStack`
- Common required input members in this group: `cfnStackName`, `sourceNetworkID`

### Export

- Operations: `ExportSourceNetworkCfnTemplate`
- Common required input members in this group: `sourceNetworkID`

### Initialize

- Operations: `InitializeService`

### Put

- Operations: `PutLaunchAction`
- Common required input members in this group: `actionCode`, `actionId`, `actionVersion`, `active`, `category`, `description`, `name`, `optional`, `order`, `resourceId`

### Retry

- Operations: `RetryDataReplication`
- Common required input members in this group: `sourceServerID`

### Reverse

- Operations: `ReverseReplication`
- Common required input members in this group: `recoveryInstanceID`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Terminate

- Operations: `TerminateRecoveryInstances`
- Common required input members in this group: `recoveryInstanceIDs`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateSourceNetworkStack` | `POST /AssociateSourceNetworkStack` | - | `cfnStackName`, `sourceNetworkID` | - | `AssociateSourceNetworkStackResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Associate a Source Network to an existing CloudFormation Stack and modify launch templates to use this network. Can be used for reverting to previously deployed CloudFormation stacks. |
| `CreateExtendedSourceServer` | `POST /CreateExtendedSourceServer` | - | `sourceServerArn` | - | `CreateExtendedSourceServerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Create an extended source server in the target Account based on the source server in staging account. |
| `CreateLaunchConfigurationTemplate` | `POST /CreateLaunchConfigurationTemplate` | - | - | - | `CreateLaunchConfigurationTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Creates a new Launch Configuration Template. |
| `CreateReplicationConfigurationTemplate` | `POST /CreateReplicationConfigurationTemplate` | - | `associateDefaultSecurityGroup`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `pitPolicy`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, `stagingAreaSubnetId`, ... (+2) | - | `ReplicationConfigurationTemplate` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Creates a new ReplicationConfigurationTemplate. |
| `CreateSourceNetwork` | `POST /CreateSourceNetwork` | - | `originAccountID`, `originRegion`, `vpcID` | - | `CreateSourceNetworkResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Create a new Source Network resource for a provided VPC ID. |
| `DeleteJob` | `POST /DeleteJob` | `idempotent` | `jobID` | - | `DeleteJobResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Deletes a single Job by ID. |
| `DeleteLaunchAction` | `POST /DeleteLaunchAction` | - | `actionId`, `resourceId` | - | `DeleteLaunchActionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Deletes a resource launch action. |
| `DeleteLaunchConfigurationTemplate` | `POST /DeleteLaunchConfigurationTemplate` | `idempotent` | `launchConfigurationTemplateID` | - | `DeleteLaunchConfigurationTemplateResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Deletes a single Launch Configuration Template by ID. |
| `DeleteRecoveryInstance` | `POST /DeleteRecoveryInstance` | - | `recoveryInstanceID` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException` | Deletes a single Recovery Instance by ID. This deletes the Recovery Instance resource from Elastic Disaster Recovery. |
| `DeleteReplicationConfigurationTemplate` | `POST /DeleteReplicationConfigurationTemplate` | `idempotent` | `replicationConfigurationTemplateID` | - | `DeleteReplicationConfigurationTemplateResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Deletes a single Replication Configuration Template by ID |
| `DeleteSourceNetwork` | `POST /DeleteSourceNetwork` | `idempotent` | `sourceNetworkID` | - | `DeleteSourceNetworkResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Delete Source Network resource. |
| `DeleteSourceServer` | `POST /DeleteSourceServer` | `idempotent` | `sourceServerID` | - | `DeleteSourceServerResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Deletes a single Source Server by ID. The Source Server must be disconnected first. |
| `DescribeJobLogItems` | `POST /DescribeJobLogItems` | `readonly`, `paginated` | `jobID` | - | `DescribeJobLogItemsResponse` | `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Retrieves a detailed Job log with pagination. |
| `DescribeJobs` | `POST /DescribeJobs` | `readonly`, `paginated` | - | - | `DescribeJobsResponse` | `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Returns a list of Jobs. Use the JobsID and fromDate and toDate filters to limit which jobs are returned. |
| `DescribeLaunchConfigurationTemplates` | `POST /DescribeLaunchConfigurationTemplates` | `readonly`, `paginated` | - | - | `DescribeLaunchConfigurationTemplatesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Lists all Launch Configuration Templates, filtered by Launch Configuration Template IDs |
| `DescribeRecoveryInstances` | `POST /DescribeRecoveryInstances` | `readonly`, `paginated` | - | - | `DescribeRecoveryInstancesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException` | Lists all Recovery Instances or multiple Recovery Instances by ID. |
| `DescribeRecoverySnapshots` | `POST /DescribeRecoverySnapshots` | `readonly`, `paginated` | `sourceServerID` | - | `DescribeRecoverySnapshotsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Lists all Recovery Snapshots for a single Source Server. |
| `DescribeReplicationConfigurationTemplates` | `POST /DescribeReplicationConfigurationTemplates` | `readonly`, `paginated` | - | - | `DescribeReplicationConfigurationTemplatesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Lists all ReplicationConfigurationTemplates, filtered by Source Server IDs. |
| `DescribeSourceNetworks` | `POST /DescribeSourceNetworks` | `readonly`, `paginated` | - | - | `DescribeSourceNetworksResponse` | `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Lists all Source Networks or multiple Source Networks filtered by ID. |
| `DescribeSourceServers` | `POST /DescribeSourceServers` | `readonly`, `paginated` | - | - | `DescribeSourceServersResponse` | `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Lists all Source Servers or multiple Source Servers filtered by ID. |
| `DisconnectRecoveryInstance` | `POST /DisconnectRecoveryInstance` | - | `recoveryInstanceID` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Disconnect a Recovery Instance from Elastic Disaster Recovery. Data replication is stopped immediately. |
| `DisconnectSourceServer` | `POST /DisconnectSourceServer` | - | `sourceServerID` | - | `SourceServer` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Disconnects a specific Source Server from Elastic Disaster Recovery. Data replication is stopped immediately. |
| `ExportSourceNetworkCfnTemplate` | `POST /ExportSourceNetworkCfnTemplate` | - | `sourceNetworkID` | - | `ExportSourceNetworkCfnTemplateResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Export the Source Network CloudFormation template to an S3 bucket. |
| `GetFailbackReplicationConfiguration` | `POST /GetFailbackReplicationConfiguration` | `readonly` | `recoveryInstanceID` | - | `GetFailbackReplicationConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Lists all Failback ReplicationConfigurations, filtered by Recovery Instance ID. |
| `GetLaunchConfiguration` | `POST /GetLaunchConfiguration` | `readonly` | `sourceServerID` | - | `LaunchConfiguration` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Gets a LaunchConfiguration, filtered by Source Server IDs. |
| `GetReplicationConfiguration` | `POST /GetReplicationConfiguration` | `readonly` | `sourceServerID` | - | `ReplicationConfiguration` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Gets a ReplicationConfiguration, filtered by Source Server ID. |
| `InitializeService` | `POST /InitializeService` | - | - | - | `InitializeServiceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Initialize Elastic Disaster Recovery. |
| `ListExtensibleSourceServers` | `POST /ListExtensibleSourceServers` | `readonly`, `paginated` | `stagingAccountID` | - | `ListExtensibleSourceServersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Returns a list of source servers on a staging account that are extensible, which means that: a. The source server is not already extended into this Account. |
| `ListLaunchActions` | `POST /ListLaunchActions` | `readonly`, `paginated` | `resourceId` | - | `ListLaunchActionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException` | Lists resource launch actions. |
| `ListStagingAccounts` | `GET /ListStagingAccounts` | `readonly`, `paginated` | - | - | `ListStagingAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Returns an array of staging accounts for existing extended source servers. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags for your Elastic Disaster Recovery resources. |
| `PutLaunchAction` | `POST /PutLaunchAction` | - | `actionCode`, `actionId`, `actionVersion`, `active`, `category`, `description`, `name`, `optional`, `order`, `resourceId` | - | `PutLaunchActionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Puts a resource launch action. |
| `RetryDataReplication` | `POST /RetryDataReplication` | - | `sourceServerID` | - | `SourceServer` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | WARNING: RetryDataReplication is deprecated. Causes the data replication initiation sequence to begin immediately upon next Handshake for the specified Source Server ID, regardless of when the previous initiation started. |
| `ReverseReplication` | `POST /ReverseReplication` | - | `recoveryInstanceID` | - | `ReverseReplicationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Start replication to origin / target region - applies only to protected instances that originated in EC2. For recovery instances on target region - starts replication back to origin region. |
| `StartFailbackLaunch` | `POST /StartFailbackLaunch` | - | `recoveryInstanceIDs` | - | `StartFailbackLaunchResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Initiates a Job for launching the machine that is being failed back to from the specified Recovery Instance. This will run conversion on the failback client and will reboot your machine, thus completing the failback process. |
| `StartRecovery` | `POST /StartRecovery` | - | `sourceServers` | - | `StartRecoveryResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException` | Launches Recovery Instances for the specified Source Servers. For each Source Server you may choose a point in time snapshot to launch from, or use an on demand snapshot. |
| `StartReplication` | `POST /StartReplication` | - | `sourceServerID` | - | `StartReplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Starts replication for a stopped Source Server. This action would make the Source Server protected again and restart billing for it. |
| `StartSourceNetworkRecovery` | `POST /StartSourceNetworkRecovery` | - | `sourceNetworks` | - | `StartSourceNetworkRecoveryResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Deploy VPC for the specified Source Network and modify launch templates to use this network. The VPC will be deployed using a dedicated CloudFormation stack. |
| `StartSourceNetworkReplication` | `POST /StartSourceNetworkReplication` | - | `sourceNetworkID` | - | `StartSourceNetworkReplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Starts replication for a Source Network. This action would make the Source Network protected. |
| `StopFailback` | `POST /StopFailback` | - | `recoveryInstanceID` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Stops the failback process for a specified Recovery Instance. This changes the Failback State of the Recovery Instance back to FAILBACK_NOT_STARTED. |
| `StopReplication` | `POST /StopReplication` | - | `sourceServerID` | - | `StopReplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Stops replication for a Source Server. This action would make the Source Server unprotected, delete its existing snapshots and stop billing for it. |
| `StopSourceNetworkReplication` | `POST /StopSourceNetworkReplication` | - | `sourceNetworkID` | - | `StopSourceNetworkReplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Stops replication for a Source Network. This action would make the Source Network unprotected. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or overwrites only the specified tags for the specified Elastic Disaster Recovery resource or resources. When you specify an existing tag key, the value is overwritten with the new value. |
| `TerminateRecoveryInstances` | `POST /TerminateRecoveryInstances` | - | `recoveryInstanceIDs` | - | `TerminateRecoveryInstancesResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException` | Initiates a Job for terminating the EC2 resources associated with the specified Recovery Instances, and then will delete the Recovery Instances from the Elastic Disaster Recovery service. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified set of tags from the specified set of Elastic Disaster Recovery resources. |
| `UpdateFailbackReplicationConfiguration` | `POST /UpdateFailbackReplicationConfiguration` | - | `recoveryInstanceID` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException` | Allows you to update the failback replication configuration of a Recovery Instance by ID. |
| `UpdateLaunchConfiguration` | `POST /UpdateLaunchConfiguration` | `idempotent` | `sourceServerID` | - | `LaunchConfiguration` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Updates a LaunchConfiguration by Source Server ID. |
| `UpdateLaunchConfigurationTemplate` | `POST /UpdateLaunchConfigurationTemplate` | - | `launchConfigurationTemplateID` | - | `UpdateLaunchConfigurationTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Updates an existing Launch Configuration Template by ID. |
| `UpdateReplicationConfiguration` | `POST /UpdateReplicationConfiguration` | `idempotent` | `sourceServerID` | - | `ReplicationConfiguration` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Allows you to update a ReplicationConfiguration by Source Server ID. |
| `UpdateReplicationConfigurationTemplate` | `POST /UpdateReplicationConfigurationTemplate` | - | `replicationConfigurationTemplateID` | - | `ReplicationConfigurationTemplate` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Updates a ReplicationConfigurationTemplate by ID. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListStagingAccounts` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied due to request throttling. |
| `UninitializedAccountException` | `structure` | `code`, `message` | The account performing the request has not been initialized. |
| `ResourceNotFoundException` | `structure` | `code`, `message`, `resourceId`, `resourceType` | The resource for this operation was not found. |
| `ValidationException` | `structure` | `code`, `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by the AWS service. |
| `ConflictException` | `structure` | `code`, `message`, `resourceId`, `resourceType` | The request could not be completed due to a conflict with the current state of the target resource. |
| `AccessDeniedException` | `structure` | `code`, `message` | You do not have sufficient access to perform this action. |
| `ServiceQuotaExceededException` | `structure` | `code`, `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request could not be completed because its exceeded the service quota. |
| `ReplicationConfigurationTemplate` | `structure` | `arn`, `associateDefaultSecurityGroup`, `autoReplicateNewDisks`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `ebsEncryptionKeyArn`, `pitPolicy`, `replicationConfigurationTemplateID`, `replicationServerInstanceType`, ... (+5) | - |
| `SourceServer` | `structure` | `agentVersion`, `arn`, `dataReplicationInfo`, `lastLaunchResult`, `lifeCycle`, `recoveryInstanceId`, `replicationDirection`, `reversedDirectionSourceServerArn`, `sourceCloudProperties`, `sourceNetworkID`, `sourceProperties`, `sourceServerID`, ... (+2) | - |
| `LaunchConfiguration` | `structure` | `copyPrivateIp`, `copyTags`, `ec2LaunchTemplateID`, `launchDisposition`, `launchIntoInstanceProperties`, `licensing`, `name`, `postLaunchEnabled`, `sourceServerID`, `targetInstanceTypeRightSizingMethod` | - |
| `ReplicationConfiguration` | `structure` | `associateDefaultSecurityGroup`, `autoReplicateNewDisks`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `ebsEncryptionKeyArn`, `name`, `pitPolicy`, `replicatedDisks`, `replicationServerInstanceType`, ... (+5) | - |
| `AssociateSourceNetworkStackRequest` | `structure` | `cfnStackName`, `sourceNetworkID` | - |
| `AssociateSourceNetworkStackResponse` | `structure` | `job` | - |
| `CreateExtendedSourceServerRequest` | `structure` | `sourceServerArn`, `tags` | - |
| `CreateExtendedSourceServerResponse` | `structure` | `sourceServer` | - |
| `CreateLaunchConfigurationTemplateRequest` | `structure` | `copyPrivateIp`, `copyTags`, `exportBucketArn`, `launchDisposition`, `launchIntoSourceInstance`, `licensing`, `postLaunchEnabled`, `tags`, `targetInstanceTypeRightSizingMethod` | - |
| `CreateLaunchConfigurationTemplateResponse` | `structure` | `launchConfigurationTemplate` | - |
| `CreateReplicationConfigurationTemplateRequest` | `structure` | `associateDefaultSecurityGroup`, `autoReplicateNewDisks`, `bandwidthThrottling`, `createPublicIP`, `dataPlaneRouting`, `defaultLargeStagingDiskType`, `ebsEncryption`, `ebsEncryptionKeyArn`, `pitPolicy`, `replicationServerInstanceType`, `replicationServersSecurityGroupsIDs`, `stagingAreaSubnetId`, ... (+3) | - |
| `CreateSourceNetworkRequest` | `structure` | `originAccountID`, `originRegion`, `tags`, `vpcID` | - |
| `CreateSourceNetworkResponse` | `structure` | `sourceNetworkID` | - |
| `DeleteJobRequest` | `structure` | `jobID` | - |
| `DeleteJobResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

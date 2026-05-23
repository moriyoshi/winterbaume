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

### List

- Operations: `ListExtensibleSourceServers`, `ListLaunchActions`, `ListStagingAccounts`, `ListTagsForResource`
- Traits: `readonly` (4), `paginated` (3)
- Common required input members in this group: -

### Create

- Operations: `CreateExtendedSourceServer`
- Common required input members in this group: -

### Delete

- Operations: `DeleteLaunchAction`
- Common required input members in this group: -

### Initialize

- Operations: `InitializeService`
- Common required input members in this group: -

### Put

- Operations: `PutLaunchAction`
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
| `CreateExtendedSourceServer` | `POST /CreateExtendedSourceServer` | - | `sourceServerArn` | - | `CreateExtendedSourceServerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Create an extended source server in the target Account based on the source server in staging account. |
| `DeleteLaunchAction` | `POST /DeleteLaunchAction` | - | `resourceId`, `actionId` | - | `DeleteLaunchActionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Deletes a resource launch action. |
| `InitializeService` | `POST /InitializeService` | - | - | - | `InitializeServiceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Initialize Elastic Disaster Recovery. |
| `ListExtensibleSourceServers` | `POST /ListExtensibleSourceServers` | `readonly`, `paginated` | `stagingAccountID` | - | `ListExtensibleSourceServersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Returns a list of source servers on a staging account that are extensible, which means that: a. The source server is not already extended into this Account. b. The source server on the Account we’re reading from is n ... |
| `ListLaunchActions` | `POST /ListLaunchActions` | `readonly`, `paginated` | `resourceId` | - | `ListLaunchActionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UninitializedAccountException` | Lists resource launch actions. |
| `ListStagingAccounts` | `GET /ListStagingAccounts` | `readonly`, `paginated` | - | - | `ListStagingAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Returns an array of staging accounts for existing extended source servers. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags for your Elastic Disaster Recovery resources. |
| `PutLaunchAction` | `POST /PutLaunchAction` | - | `resourceId`, `actionCode`, `order`, `actionId`, `optional`, `active`, `name`, `actionVersion`, `category`, `description` | - | `PutLaunchActionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UninitializedAccountException`, `ValidationException` | Puts a resource launch action. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or overwrites only the specified tags for the specified Elastic Disaster Recovery resource or resources. When you specify an existing tag key, the value is overwritten with the new value. Each resource can have ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified set of tags from the specified set of Elastic Disaster Recovery resources. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListStagingAccounts` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, code | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, code, resourceId, resourceType | The request could not be completed due to a conflict with the current state of the target resource. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | The request processing has failed because of an unknown error, exception or failure. |
| `ResourceNotFoundException` | `structure` | message, code, resourceId, resourceType | The resource for this operation was not found. |
| `ServiceQuotaExceededException` | `structure` | message, code, resourceId, resourceType, serviceCode, quotaCode | The request could not be completed because its exceeded the service quota. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied due to request throttling. |
| `UninitializedAccountException` | `structure` | message, code | The account performing the request has not been initialized. |
| `ValidationException` | `structure` | message, code, reason, fieldList | The input fails to satisfy the constraints specified by the AWS service. |
| `CreateExtendedSourceServerRequest` | `structure` | sourceServerArn, tags | - |
| `CreateExtendedSourceServerResponse` | `structure` | sourceServer | - |
| `DeleteLaunchActionRequest` | `structure` | resourceId, actionId | - |
| `DeleteLaunchActionResponse` | `structure` | **empty (no members)** | - |
| `InitializeServiceRequest` | `structure` | **empty (no members)** | - |
| `InitializeServiceResponse` | `structure` | **empty (no members)** | - |
| `ListExtensibleSourceServersRequest` | `structure` | stagingAccountID, maxResults, nextToken | - |
| `ListExtensibleSourceServersResponse` | `structure` | items, nextToken | - |
| `ListLaunchActionsRequest` | `structure` | resourceId, filters, maxResults, nextToken | - |
| `ListLaunchActionsResponse` | `structure` | items, nextToken | - |
| `ListStagingAccountsRequest` | `structure` | maxResults, nextToken | - |
| `ListStagingAccountsResponse` | `structure` | accounts, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `PutLaunchActionRequest` | `structure` | resourceId, actionCode, order, actionId, optional, active, name, actionVersion, category, parameters, description | - |
| `PutLaunchActionResponse` | `structure` | resourceId, actionId, actionCode, type, name, active, order, actionVersion, optional, parameters, description, category | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

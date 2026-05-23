# AWS Application Discovery Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Application Discovery Service Amazon Web Services Application Discovery Service (Application Discovery Service) helps you plan application migration projects. It automatically identifies servers, virtual machines (VMs), and network dependencies in your on-premises data centers. For more information, see the Amazon Web Services Application Discovery Service FAQ. Application Discovery Service offers three ways of performing discovery and collecting data about your on-premises servers: Agentless discovery using Amazon Web Services Application Discovery Service Agentless Collector (Agentless Collector), which doesn't require you to install an agent on each host. Agentless Collector gathers server information regardless of the operating systems, which minimizes the time required for initial on-premises infrastructure assessment.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Application Discovery Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Application Discovery Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Application Discovery Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: collect and query on-premises server, agent, application, and connection data for migration planning.
- From the operation surface: support import tasks, continuous export, tag/association management, and discovery summaries used before cloud migration.

## Service Identity and Protocol

- AWS model slug: `application-discovery-service`
- AWS SDK for Rust slug: `applicationdiscovery`
- Model version: `2015-11-01`
- Model file: `vendor/api-models-aws/models/application-discovery-service/service/2015-11-01/application-discovery-service-2015-11-01.json`
- SDK ID: `Application Discovery Service`
- Endpoint prefix: `discovery`
- ARN namespace: `discovery`
- CloudFormation name: `ApplicationDiscoveryService`
- CloudTrail event source: `applicationdiscoveryservice.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (8), `Start` (5), `Batch` (2), `Create` (2), `Delete` (2), `List` (2), `Stop` (2), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateConfigurationItemsToApplication`, `BatchDeleteAgents`, `BatchDeleteImportData`, `CreateApplication`, `CreateTags`, `DeleteApplications`, `DeleteTags`, `DisassociateConfigurationItemsFromApplication`, `StartBatchDeleteConfigurationTask`, `StartContinuousExport`, `StartDataCollectionByAgentIds`, `StartExportTask`, `StartImportTask`, `StopContinuousExport`, `StopDataCollectionByAgentIds`, `UpdateApplication`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAgents`, `DescribeBatchDeleteConfigurationTask`, `DescribeConfigurations`, `DescribeContinuousExports`, `DescribeExportConfigurations`, `DescribeExportTasks`, `DescribeImportTasks`, `DescribeTags`, `GetDiscoverySummary`, `ListConfigurations`, `ListServerNeighbors`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchDeleteImportData`, `DescribeBatchDeleteConfigurationTask`, `DescribeContinuousExports`, `DescribeExportConfigurations`, `DescribeExportTasks`, `DescribeImportTasks`, `ExportConfigurations`, `StartBatchDeleteConfigurationTask`, `StartContinuousExport`, `StartDataCollectionByAgentIds`, `StartExportTask`, `StartImportTask`, `StopContinuousExport`, `StopDataCollectionByAgentIds`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 28 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/application-discovery/latest/userguide/what-is-appdiscovery.html
- https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-agent.html
- https://docs.aws.amazon.com/application-discovery/latest/userguide/agentless-collector-deploying.html

Research outcomes:
- Application Discovery Service collects on-premises server inventory, performance, and dependency data for migration planning.
- Discovery data can come from agentless VMware collection, installed Discovery Agents, or imported data.
- Discovery Agents collect system configuration, time-series utilisation, running processes, and network connections from servers.
- Agentless Collector is deployed as a VMware virtual appliance and uses IAM permissions to send discovery data.
- Discovered servers can be grouped into applications and viewed in Migration Hub.
- Collection state, discovered server state, tags, and application grouping are separate concepts.

Parity implications:
- Model agents, agentless collectors, discovered servers, configurations, performance samples, network connections, imports, and application groups separately.
- Start/stop data collection should affect future samples without deleting already discovered data.
- Migration Hub application grouping should reference discovered resources rather than copying server records.

## Operation Groups

### Describe

- Operations: `DescribeAgents`, `DescribeBatchDeleteConfigurationTask`, `DescribeConfigurations`, `DescribeContinuousExports`, `DescribeExportConfigurations`, `DescribeExportTasks`, `DescribeImportTasks`, `DescribeTags`
- Traits: `paginated` (6)
- Common required input members in this group: `configurationIds`, `taskId`

### Start

- Operations: `StartBatchDeleteConfigurationTask`, `StartContinuousExport`, `StartDataCollectionByAgentIds`, `StartExportTask`, `StartImportTask`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `agentIds`, `configurationIds`, `configurationType`, `importUrl`, `name`

### Batch

- Operations: `BatchDeleteAgents`, `BatchDeleteImportData`
- Common required input members in this group: `deleteAgents`, `importTaskIds`

### Create

- Operations: `CreateApplication`, `CreateTags`
- Common required input members in this group: `configurationIds`, `name`, `tags`

### Delete

- Operations: `DeleteApplications`, `DeleteTags`
- Common required input members in this group: `configurationIds`

### List

- Operations: `ListConfigurations`, `ListServerNeighbors`
- Traits: `paginated` (1)
- Common required input members in this group: `configurationId`, `configurationType`

### Stop

- Operations: `StopContinuousExport`, `StopDataCollectionByAgentIds`
- Common required input members in this group: `agentIds`, `exportId`

### Associate

- Operations: `AssociateConfigurationItemsToApplication`
- Common required input members in this group: `applicationConfigurationId`, `configurationIds`

### Disassociate

- Operations: `DisassociateConfigurationItemsFromApplication`
- Common required input members in this group: `applicationConfigurationId`, `configurationIds`

### Export

- Operations: `ExportConfigurations`

### Get

- Operations: `GetDiscoverySummary`

### Update

- Operations: `UpdateApplication`
- Common required input members in this group: `configurationId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateConfigurationItemsToApplication` | - | - | `applicationConfigurationId`, `configurationIds` | - | `AssociateConfigurationItemsToApplicationResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Associates one or more configuration items with an application. |
| `BatchDeleteAgents` | - | - | `deleteAgents` | - | `BatchDeleteAgentsResponse` | `AuthorizationErrorException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Deletes one or more agents or collectors as specified by ID. Deleting an agent or collector does not delete the previously discovered data. |
| `BatchDeleteImportData` | - | - | `importTaskIds` | - | `BatchDeleteImportDataResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Deletes one or more import tasks, each identified by their import ID. Each import task has a number of records that can identify servers or applications. |
| `CreateApplication` | - | - | `name` | - | `CreateApplicationResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Creates an application with the given name and description. |
| `CreateTags` | - | - | `configurationIds`, `tags` | - | `CreateTagsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `ServerInternalErrorException` | Creates one or more tags for configuration items. Tags are metadata that help you categorize IT assets. |
| `DeleteApplications` | - | - | `configurationIds` | - | `DeleteApplicationsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Deletes a list of applications and their associations with configuration items. |
| `DeleteTags` | - | - | `configurationIds` | - | `DeleteTagsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `ServerInternalErrorException` | Deletes the association between configuration items and one or more tags. This API accepts a list of multiple configuration items. |
| `DescribeAgents` | - | `paginated` | - | - | `DescribeAgentsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Lists agents or collectors as specified by ID or other filters. All agents/collectors associated with your user can be listed if you call `DescribeAgents` as is without passing any parameters. |
| `DescribeBatchDeleteConfigurationTask` | - | - | `taskId` | - | `DescribeBatchDeleteConfigurationTaskResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Takes a unique deletion task identifier as input and returns metadata about a configuration deletion task. |
| `DescribeConfigurations` | - | - | `configurationIds` | - | `DescribeConfigurationsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Retrieves attributes for a list of configuration item IDs. All of the supplied IDs must be for the same asset type from one of the following: server application process connection Output fields are specific to the asset type specified. |
| `DescribeContinuousExports` | - | `paginated` | - | - | `DescribeContinuousExportsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `OperationNotPermittedException`, `ResourceNotFoundException`, `ServerInternalErrorException` | Lists exports as specified by ID. All continuous exports associated with your user can be listed if you call `DescribeContinuousExports` as is without passing any parameters. |
| `DescribeExportConfigurations` | - | `paginated` | - | - | `DescribeExportConfigurationsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `ServerInternalErrorException` | `DescribeExportConfigurations` is deprecated. Use DescribeExportTasks, instead. |
| `DescribeExportTasks` | - | `paginated` | - | - | `DescribeExportTasksResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Retrieve status of one or more export tasks. You can retrieve the status of up to 100 export tasks. |
| `DescribeImportTasks` | - | `paginated` | - | - | `DescribeImportTasksResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Returns an array of import tasks for your account, including status information, times, IDs, the Amazon S3 Object URL for the import file, and more. |
| `DescribeTags` | - | `paginated` | - | - | `DescribeTagsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `ServerInternalErrorException` | Retrieves a list of configuration items that have tags as specified by the key-value pairs, name and value, passed to the optional parameter `filters`. There are three valid tag filter names: tagKey tagValue configurationId Also, all configuration items... |
| `DisassociateConfigurationItemsFromApplication` | - | - | `applicationConfigurationId`, `configurationIds` | - | `DisassociateConfigurationItemsFromApplicationResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Disassociates one or more configuration items from an application. |
| `ExportConfigurations` | - | - | - | - | `ExportConfigurationsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `OperationNotPermittedException`, `ServerInternalErrorException` | Deprecated. Use `StartExportTask` instead. |
| `GetDiscoverySummary` | - | - | - | - | `GetDiscoverySummaryResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Retrieves a short summary of discovered assets. This API operation takes no request parameters and is called as is at the command prompt as shown in the example. |
| `ListConfigurations` | - | `paginated` | `configurationType` | - | `ListConfigurationsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ResourceNotFoundException`, `ServerInternalErrorException` | Retrieves a list of configuration items as specified by the value passed to the required parameter `configurationType`. Optional filtering may be applied to refine search results. |
| `ListServerNeighbors` | - | - | `configurationId` | - | `ListServerNeighborsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Retrieves a list of servers that are one network hop away from a specified server. |
| `StartBatchDeleteConfigurationTask` | - | - | `configurationIds`, `configurationType` | - | `StartBatchDeleteConfigurationTaskResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `LimitExceededException`, `OperationNotPermittedException`, `ServerInternalErrorException` | Takes a list of configurationId as input and starts an asynchronous deletion task to remove the configurationItems. Returns a unique deletion task identifier. |
| `StartContinuousExport` | - | - | - | - | `StartContinuousExportResponse` | `AuthorizationErrorException`, `ConflictErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `OperationNotPermittedException`, `ResourceInUseException`, `ServerInternalErrorException` | Start the continuous flow of agent's discovered data into Amazon Athena. |
| `StartDataCollectionByAgentIds` | - | - | `agentIds` | - | `StartDataCollectionByAgentIdsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Instructs the specified agents to start collecting data. |
| `StartExportTask` | - | - | - | - | `StartExportTaskResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `OperationNotPermittedException`, `ServerInternalErrorException` | Begins the export of a discovered data report to an Amazon S3 bucket managed by Amazon Web Services. Exports might provide an estimate of fees and savings based on certain information that you provide. |
| `StartImportTask` | - | `idempotency-token` | `importUrl`, `name` | `clientRequestToken` | `StartImportTaskResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ResourceInUseException`, `ServerInternalErrorException` | Starts an import task, which allows you to import details of your on-premises environment directly into Amazon Web Services Migration Hub without having to use the Amazon Web Services Application Discovery Service (Application Discovery Service) tools such as... |
| `StopContinuousExport` | - | - | `exportId` | - | `StopContinuousExportResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `OperationNotPermittedException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerInternalErrorException` | Stop the continuous flow of agent's discovered data into Amazon Athena. |
| `StopDataCollectionByAgentIds` | - | - | `agentIds` | - | `StopDataCollectionByAgentIdsResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Instructs the specified agents to stop collecting data. |
| `UpdateApplication` | - | - | `configurationId` | - | `UpdateApplicationResponse` | `AuthorizationErrorException`, `HomeRegionNotSetException`, `InvalidParameterException`, `InvalidParameterValueException`, `ServerInternalErrorException` | Updates metadata about an application. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AuthorizationErrorException` | `structure` | `message` | The user does not have permission to perform the action. |
| `InvalidParameterValueException` | `structure` | `message` | The value of one or more parameters are either invalid or out of range. |
| `ServerInternalErrorException` | `structure` | `message` | The server experienced an internal error. |
| `HomeRegionNotSetException` | `structure` | `message` | The home Region is not set. |
| `InvalidParameterException` | `structure` | `message` | One or more parameters are not valid. |
| `ResourceNotFoundException` | `structure` | `message` | The specified configuration ID was not located. |
| `OperationNotPermittedException` | `structure` | `message` | This operation is not permitted. |
| `ResourceInUseException` | `structure` | `message` | This issue occurs when the same `clientRequestToken` is used with the `StartImportTask` action, but with different parameters. |
| `AssociateConfigurationItemsToApplicationRequest` | `structure` | `applicationConfigurationId`, `configurationIds` | - |
| `AssociateConfigurationItemsToApplicationResponse` | `structure` | - | - |
| `BatchDeleteAgentsRequest` | `structure` | `deleteAgents` | - |
| `BatchDeleteAgentsResponse` | `structure` | `errors` | - |
| `BatchDeleteImportDataRequest` | `structure` | `deleteHistory`, `importTaskIds` | - |
| `BatchDeleteImportDataResponse` | `structure` | `errors` | - |
| `CreateApplicationRequest` | `structure` | `description`, `name`, `wave` | - |
| `CreateApplicationResponse` | `structure` | `configurationId` | - |
| `CreateTagsRequest` | `structure` | `configurationIds`, `tags` | - |
| `CreateTagsResponse` | `structure` | - | - |
| `DeleteApplicationsRequest` | `structure` | `configurationIds` | - |
| `DeleteApplicationsResponse` | `structure` | - | - |
| `DeleteTagsRequest` | `structure` | `configurationIds`, `tags` | - |
| `DeleteTagsResponse` | `structure` | - | - |
| `DescribeAgentsRequest` | `structure` | `agentIds`, `filters`, `maxResults`, `nextToken` | - |
| `DescribeAgentsResponse` | `structure` | `agentsInfo`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Network Flow Monitor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Network Flow Monitor is a feature of Amazon CloudWatch Network Monitoring that provides visibility into the performance of network flows for your Amazon Web Services workloads, between instances in subnets, as well as to and from Amazon Web Services. Lightweight agents that you install on the instances capture performance metrics for your network flows, such as packet loss and latency, and send them to the Network Flow Monitor backend. Then, you can view and analyze metrics from the top contributors for each metric type, to help troubleshoot issues. In addition, when you create a monitor, Network Flow Monitor provides a network health indicator (NHI) that informs you whether there were Amazon Web Services network issues for one or more of the network flows tracked by a monitor, during a time period that you choose. By using this value, you can independently determine if the Amazon Web Services network is impacting your workload during a specific time frame, to help you focus troubleshooting efforts.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Network Flow Monitor workflows in the local mock. Key resources include `MonitorResource`, `ScopeResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Start`, `Stop`, `Create` operation families, including `GetMonitor`, `GetQueryResultsMonitorTopContributors`, `GetQueryResultsWorkloadInsightsTopContributors`, `GetQueryResultsWorkloadInsightsTopContributorsData`, `ListMonitors`, `ListScopes`.

## Service Identity and Protocol

- AWS model slug: `networkflowmonitor`
- AWS SDK for Rust slug: `networkflowmonitor`
- Model version: `2023-04-19`
- Model file: `vendor/api-models-aws/models/networkflowmonitor/service/2023-04-19/networkflowmonitor-2023-04-19.json`
- SDK ID: `NetworkFlowMonitor`
- Endpoint prefix: `-`
- ARN namespace: `networkflowmonitor`
- CloudFormation name: `-`
- CloudTrail event source: `networkflowmonitor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (8), `List` (3), `Start` (3), `Stop` (3), `Create` (2), `Delete` (2), `Update` (2), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateMonitor`, `CreateScope`, `DeleteMonitor`, `DeleteScope`, `StartQueryMonitorTopContributors`, `StartQueryWorkloadInsightsTopContributors`, `StartQueryWorkloadInsightsTopContributorsData`, `StopQueryMonitorTopContributors`, `StopQueryWorkloadInsightsTopContributors`, `StopQueryWorkloadInsightsTopContributorsData`, `TagResource`, `UntagResource`, `UpdateMonitor`, `UpdateScope`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetMonitor`, `GetQueryResultsMonitorTopContributors`, `GetQueryResultsWorkloadInsightsTopContributors`, `GetQueryResultsWorkloadInsightsTopContributorsData`, `GetQueryStatusMonitorTopContributors`, `GetQueryStatusWorkloadInsightsTopContributors`, `GetQueryStatusWorkloadInsightsTopContributorsData`, `GetScope`, `ListMonitors`, `ListScopes`, `ListTagsForResource`, `StartQueryWorkloadInsightsTopContributors`, `StartQueryWorkloadInsightsTopContributorsData`, `StopQueryWorkloadInsightsTopContributors`, `StopQueryWorkloadInsightsTopContributorsData`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartQueryMonitorTopContributors`, `StartQueryWorkloadInsightsTopContributors`, `StartQueryWorkloadInsightsTopContributorsData`, `StopQueryMonitorTopContributors`, `StopQueryWorkloadInsightsTopContributors`, `StopQueryWorkloadInsightsTopContributorsData`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `EC2/VPC`, `ECS`, `EKS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `MonitorResource` | `monitorName` | put: `CreateMonitor`; read: `GetMonitor`; update: `UpdateMonitor`; delete: `DeleteMonitor`; list: `ListMonitors` | `GetQueryResultsMonitorTopContributors`, `GetQueryStatusMonitorTopContributors`, `StartQueryMonitorTopContributors`, `StopQueryMonitorTopContributors` | - |
| `ScopeResource` | `scopeId` | create: `CreateScope`; read: `GetScope`; update: `UpdateScope`; delete: `DeleteScope`; list: `ListScopes` | `GetQueryResultsWorkloadInsightsTopContributors`, `GetQueryResultsWorkloadInsightsTopContributorsData`, `GetQueryStatusWorkloadInsightsTopContributors`, `GetQueryStatusWorkloadInsightsTopContributorsData`, `StartQueryWorkloadInsightsTopContributors`, `StartQueryWorkloadInsightsTopContributorsData`, `StopQueryWorkloadInsightsTopContributors`, `StopQueryWorkloadInsightsTopContributorsData` | - |
## Operation Groups

### Get

- Operations: `GetMonitor`, `GetQueryResultsMonitorTopContributors`, `GetQueryResultsWorkloadInsightsTopContributors`, `GetQueryResultsWorkloadInsightsTopContributorsData`, `GetQueryStatusMonitorTopContributors`, `GetQueryStatusWorkloadInsightsTopContributors`, `GetQueryStatusWorkloadInsightsTopContributorsData`, `GetScope`
- Traits: `paginated` (3), `readonly` (8)
- Common required input members in this group: `monitorName`, `queryId`, `scopeId`

### List

- Operations: `ListMonitors`, `ListScopes`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `resourceArn`

### Start

- Operations: `StartQueryMonitorTopContributors`, `StartQueryWorkloadInsightsTopContributors`, `StartQueryWorkloadInsightsTopContributorsData`
- Traits: `readonly` (2)
- Common required input members in this group: `destinationCategory`, `endTime`, `metricName`, `monitorName`, `scopeId`, `startTime`

### Stop

- Operations: `StopQueryMonitorTopContributors`, `StopQueryWorkloadInsightsTopContributors`, `StopQueryWorkloadInsightsTopContributorsData`
- Traits: `idempotent` (1), `readonly` (2)
- Common required input members in this group: `monitorName`, `queryId`, `scopeId`

### Create

- Operations: `CreateMonitor`, `CreateScope`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `localResources`, `monitorName`, `scopeArn`, `targets`

### Delete

- Operations: `DeleteMonitor`, `DeleteScope`
- Traits: `idempotent` (2)
- Common required input members in this group: `monitorName`, `scopeId`

### Update

- Operations: `UpdateMonitor`, `UpdateScope`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `monitorName`, `scopeId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateMonitor` | `POST /monitors` | `idempotent`, `idempotency-token` | `localResources`, `monitorName`, `scopeArn` | `clientToken` | `CreateMonitorOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a monitor for specific network flows between local and remote resources, so that you can monitor network performance for one or several of your workloads. For each monitor, Network Flow Monitor publishes detailed end-to-end performance metrics and a... |
| `CreateScope` | `POST /scopes` | `idempotent`, `idempotency-token` | `targets` | `clientToken` | `CreateScopeOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | In Network Flow Monitor, you specify a scope for the service to generate metrics for. By using the scope, Network Flow Monitor can generate a topology of all the resources to measure performance metrics for. |
| `DeleteMonitor` | `DELETE /monitors/{monitorName}` | `idempotent` | `monitorName` | - | `DeleteMonitorOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a monitor in Network Flow Monitor. |
| `DeleteScope` | `DELETE /scopes/{scopeId}` | `idempotent` | `scopeId` | - | `DeleteScopeOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a scope that has been defined. |
| `GetMonitor` | `GET /monitors/{monitorName}` | `readonly` | `monitorName` | - | `GetMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a monitor in Network Flow Monitor based on a monitor name. The information returned includes the Amazon Resource Name (ARN), create time, modified time, resources included in the monitor, and status information. |
| `GetQueryResultsMonitorTopContributors` | `GET /monitors/{monitorName}/topContributorsQueries/{queryId}/results` | `readonly`, `paginated` | `monitorName`, `queryId` | - | `GetQueryResultsMonitorTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Return the data for a query with the Network Flow Monitor query interface. You specify the query that you want to return results for by providing a query ID and a monitor name. |
| `GetQueryResultsWorkloadInsightsTopContributors` | `GET /workloadInsights/{scopeId}/topContributorsQueries/{queryId}/results` | `readonly`, `paginated` | `queryId`, `scopeId` | - | `GetQueryResultsWorkloadInsightsTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Return the data for a query with the Network Flow Monitor query interface. You specify the query that you want to return results for by providing a query ID and a monitor name. |
| `GetQueryResultsWorkloadInsightsTopContributorsData` | `GET /workloadInsights/{scopeId}/topContributorsDataQueries/{queryId}/results` | `readonly`, `paginated` | `queryId`, `scopeId` | - | `GetQueryResultsWorkloadInsightsTopContributorsDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Return the data for a query with the Network Flow Monitor query interface. Specify the query that you want to return results for by providing a query ID and a scope ID. |
| `GetQueryStatusMonitorTopContributors` | `GET /monitors/{monitorName}/topContributorsQueries/{queryId}/status` | `readonly` | `monitorName`, `queryId` | - | `GetQueryStatusMonitorTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Returns the current status of a query for the Network Flow Monitor query interface, for a specified query ID and monitor. This call returns the query status for the top contributors for a monitor. |
| `GetQueryStatusWorkloadInsightsTopContributors` | `GET /workloadInsights/{scopeId}/topContributorsQueries/{queryId}/status` | `readonly` | `queryId`, `scopeId` | - | `GetQueryStatusWorkloadInsightsTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Return the data for a query with the Network Flow Monitor query interface. Specify the query that you want to return results for by providing a query ID and a monitor name. |
| `GetQueryStatusWorkloadInsightsTopContributorsData` | `GET /workloadInsights/{scopeId}/topContributorsDataQueries/{queryId}/status` | `readonly` | `queryId`, `scopeId` | - | `GetQueryStatusWorkloadInsightsTopContributorsDataOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Returns the current status of a query for the Network Flow Monitor query interface, for a specified query ID and monitor. This call returns the query status for the top contributors data for workload insights. |
| `GetScope` | `GET /scopes/{scopeId}` | `readonly` | `scopeId` | - | `GetScopeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets information about a scope, including the name, status, tags, and target details. The scope in Network Flow Monitor is an account. |
| `ListMonitors` | `GET /monitors` | `readonly`, `paginated` | - | - | `ListMonitorsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all monitors in an account. Optionally, you can list only monitors that have a specific status, by using the `STATUS` parameter. |
| `ListScopes` | `GET /scopes` | `readonly`, `paginated` | - | - | `ListScopesOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | List all the scopes for an account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns all the tags for a resource. |
| `StartQueryMonitorTopContributors` | `POST /monitors/{monitorName}/topContributorsQueries` | - | `destinationCategory`, `endTime`, `metricName`, `monitorName`, `startTime` | - | `StartQueryMonitorTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a query that you can use with the Network Flow Monitor query interface to return the top contributors for a monitor. Specify the monitor that you want to create the query for. |
| `StartQueryWorkloadInsightsTopContributors` | `POST /workloadInsights/{scopeId}/topContributorsQueries` | `readonly` | `destinationCategory`, `endTime`, `metricName`, `scopeId`, `startTime` | - | `StartQueryWorkloadInsightsTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a query with the Network Flow Monitor query interface that you can run to return workload insights top contributors. Specify the scope that you want to create a query for. |
| `StartQueryWorkloadInsightsTopContributorsData` | `POST /workloadInsights/{scopeId}/topContributorsDataQueries` | `readonly` | `destinationCategory`, `endTime`, `metricName`, `scopeId`, `startTime` | - | `StartQueryWorkloadInsightsTopContributorsDataOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a query with the Network Flow Monitor query interface that you can run to return data for workload insights top contributors. Specify the scope that you want to create a query for. |
| `StopQueryMonitorTopContributors` | `DELETE /monitors/{monitorName}/topContributorsQueries/{queryId}` | `idempotent` | `monitorName`, `queryId` | - | `StopQueryMonitorTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Stop a top contributors query for a monitor. Specify the query that you want to stop by providing a query ID and a monitor name. |
| `StopQueryWorkloadInsightsTopContributors` | `DELETE /workloadInsights/{scopeId}/topContributorsQueries/{queryId}` | `readonly` | `queryId`, `scopeId` | - | `StopQueryWorkloadInsightsTopContributorsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Stop a top contributors query for workload insights. Specify the query that you want to stop by providing a query ID and a scope ID. |
| `StopQueryWorkloadInsightsTopContributorsData` | `DELETE /workloadInsights/{scopeId}/topContributorsDataQueries/{queryId}` | `readonly` | `queryId`, `scopeId` | - | `StopQueryWorkloadInsightsTopContributorsDataOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Stop a top contributors data query for workload insights. Specify the query that you want to stop by providing a query ID and a scope ID. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a tag to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from a resource. |
| `UpdateMonitor` | `PATCH /monitors/{monitorName}` | `idempotent`, `idempotency-token` | `monitorName` | `clientToken` | `UpdateMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a monitor to add or remove local or remote resources. |
| `UpdateScope` | `PATCH /scopes/{scopeId}` | `idempotent` | `scopeId` | - | `UpdateScopeOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update a scope to add or remove resources that you want to be available for Network Flow Monitor to generate metrics for, when you have active agents on those resources sending metrics reports to the Network Flow Monitor backend. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permission to perform this action. |
| `InternalServerException` | `structure` | `message` | An internal error occurred. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | Invalid request. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request exceeded a service quota. |
| `ResourceNotFoundException` | `structure` | `message` | The request specifies a resource that doesn't exist. |
| `ConflictException` | `structure` | `message` | The requested resource is in use. |
| `CreateMonitorInput` | `structure` | `clientToken`, `localResources`, `monitorName`, `remoteResources`, `scopeArn`, `tags` | - |
| `CreateMonitorOutput` | `structure` | `createdAt`, `localResources`, `modifiedAt`, `monitorArn`, `monitorName`, `monitorStatus`, `remoteResources`, `tags` | - |
| `CreateScopeInput` | `structure` | `clientToken`, `tags`, `targets` | - |
| `CreateScopeOutput` | `structure` | `scopeArn`, `scopeId`, `status`, `tags` | - |
| `DeleteMonitorInput` | `structure` | `monitorName` | - |
| `DeleteMonitorOutput` | `structure` | - | - |
| `DeleteScopeInput` | `structure` | `scopeId` | - |
| `DeleteScopeOutput` | `structure` | - | - |
| `GetMonitorInput` | `structure` | `monitorName` | - |
| `GetMonitorOutput` | `structure` | `createdAt`, `localResources`, `modifiedAt`, `monitorArn`, `monitorName`, `monitorStatus`, `remoteResources`, `tags` | - |
| `GetQueryResultsMonitorTopContributorsInput` | `structure` | `maxResults`, `monitorName`, `nextToken`, `queryId` | - |
| `GetQueryResultsMonitorTopContributorsOutput` | `structure` | `nextToken`, `topContributors`, `unit` | - |
| `GetQueryResultsWorkloadInsightsTopContributorsInput` | `structure` | `maxResults`, `nextToken`, `queryId`, `scopeId` | - |
| `GetQueryResultsWorkloadInsightsTopContributorsOutput` | `structure` | `nextToken`, `topContributors` | - |
| `GetQueryResultsWorkloadInsightsTopContributorsDataInput` | `structure` | `maxResults`, `nextToken`, `queryId`, `scopeId` | - |
| `GetQueryResultsWorkloadInsightsTopContributorsDataOutput` | `structure` | `datapoints`, `nextToken`, `unit` | - |
| `GetQueryStatusMonitorTopContributorsInput` | `structure` | `monitorName`, `queryId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

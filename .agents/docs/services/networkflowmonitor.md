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

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns all the tags for a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a tag to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permission to perform this action. |
| `ConflictException` | `structure` | message | The requested resource is in use. |
| `InternalServerException` | `structure` | message | An internal error occurred. |
| `ResourceNotFoundException` | `structure` | message | The request specifies a resource that doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | message | The request exceeded a service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | Invalid request. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `DestinationCategory` | `enum` | INTRA_AZ, INTER_AZ, INTER_VPC, UNCLASSIFIED, AMAZON_S3, AMAZON_DYNAMODB, INTER_REGION | - |
| `MetricUnit` | `enum` | SECONDS, MICROSECONDS, MILLISECONDS, BYTES, KILOBYTES, MEGABYTES, GIGABYTES, TERABYTES, BITS, KILOBITS, MEGABITS, GIGABITS, ... (+15) | - |
| `MonitorLocalResourceType` | `enum` | AWS_VPC, AWS_AZ, AWS_SUBNET, AWS_REGION, EKS_CLUSTER | - |
| `MonitorMetric` | `enum` | ROUND_TRIP_TIME, TIMEOUTS, RETRANSMISSIONS, DATA_TRANSFERRED | - |
| `MonitorRemoteResourceType` | `enum` | AWS_VPC, AWS_AZ, AWS_SUBNET, AWS_SERVICE, AWS_REGION | - |
| `MonitorStatus` | `enum` | PENDING, ACTIVE, INACTIVE, ERROR, DELETING | - |
| `QueryStatus` | `enum` | QUEUED, RUNNING, SUCCEEDED, FAILED, CANCELED | - |
| `ScopeStatus` | `enum` | SUCCEEDED, IN_PROGRESS, FAILED, DEACTIVATING, DEACTIVATED | - |
| `TargetType` | `enum` | ACCOUNT | - |
| `WorkloadInsightsMetric` | `enum` | TIMEOUTS, RETRANSMISSIONS, DATA_TRANSFERRED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

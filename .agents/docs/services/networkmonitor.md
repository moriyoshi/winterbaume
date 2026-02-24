# Amazon CloudWatch Network Monitor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudWatch Network Monitor is an Amazon Web Services active network monitoring service that identifies if a network issues exists within the Amazon Web Services network or your own company network. Within Network Monitor you'll choose the source VPCs and subnets from the Amazon Web Services network in which you operate and then you'll choose the destination IP addresses from your on-premises network. From these sources and destinations, Network Monitor creates a monitor containing all the possible source and destination combinations, each of which is called a probe, within a single monitor. These probes then monitor network traffic to help you identify where network issues might be affecting your traffic. Before you begin, ensure the Amazon Web Services CLI is configured in the Amazon Web Services Account where you will create the Network Monitor resource.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon CloudWatch Network Monitor workflows in the local mock. Key resources include `MonitorResource`, `ProbeResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Delete`, `Get`, `List`, `Update` operation families, including `CreateMonitor`, `CreateProbe`, `DeleteMonitor`, `DeleteProbe`, `GetMonitor`, `GetProbe`.

## Service Identity and Protocol

- AWS model slug: `networkmonitor`
- AWS SDK for Rust slug: `networkmonitor`
- Model version: `2023-08-01`
- Model file: `vendor/api-models-aws/models/networkmonitor/service/2023-08-01/networkmonitor-2023-08-01.json`
- SDK ID: `NetworkMonitor`
- Endpoint prefix: `-`
- ARN namespace: `networkmonitor`
- CloudFormation name: `NetworkMonitor`
- CloudTrail event source: `networkmonitor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (2), `Delete` (2), `Get` (2), `List` (2), `Update` (2), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateMonitor`, `CreateProbe`, `DeleteMonitor`, `DeleteProbe`, `TagResource`, `UntagResource`, `UpdateMonitor`, `UpdateProbe`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetMonitor`, `GetProbe`, `ListMonitors`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `MonitorResource` | `monitorName` | put: `CreateMonitor`; read: `GetMonitor`; update: `UpdateMonitor`; delete: `DeleteMonitor`; list: `ListMonitors` | - | - |
| `ProbeResource` | `monitorName`, `probeId` | create: `CreateProbe`; read: `GetProbe`; update: `UpdateProbe`; delete: `DeleteProbe` | - | - |
## Operation Groups

### Create

- Operations: `CreateMonitor`, `CreateProbe`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `monitorName`, `probe`

### Delete

- Operations: `DeleteMonitor`, `DeleteProbe`
- Traits: `idempotent` (2)
- Common required input members in this group: `monitorName`, `probeId`

### Get

- Operations: `GetMonitor`, `GetProbe`
- Traits: `readonly` (2)
- Common required input members in this group: `monitorName`, `probeId`

### List

- Operations: `ListMonitors`, `ListTagsForResource`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `resourceArn`

### Update

- Operations: `UpdateMonitor`, `UpdateProbe`
- Traits: `idempotent` (2)
- Common required input members in this group: `aggregationPeriod`, `monitorName`, `probeId`

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
| `CreateMonitor` | `POST /monitors` | `idempotent`, `idempotency-token` | `monitorName` | `clientToken` | `CreateMonitorOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a monitor between a source subnet and destination IP address. Within a monitor you'll create one or more probes that monitor network traffic between your source Amazon Web Services VPC subnets and your destination IP addresses. |
| `CreateProbe` | `POST /monitors/{monitorName}/probes` | `idempotent`, `idempotency-token` | `monitorName`, `probe` | `clientToken` | `CreateProbeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a probe within a monitor. Once you create a probe, and it begins monitoring your network traffic, you'll incur billing charges for that probe. |
| `DeleteMonitor` | `DELETE /monitors/{monitorName}` | `idempotent` | `monitorName` | - | `DeleteMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a specified monitor. This action requires the `monitorName` parameter. |
| `DeleteProbe` | `DELETE /monitors/{monitorName}/probes/{probeId}` | `idempotent` | `monitorName`, `probeId` | - | `DeleteProbeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes the specified probe. Once a probe is deleted you'll no longer incur any billing fees for that probe. |
| `GetMonitor` | `GET /monitors/{monitorName}` | `readonly` | `monitorName` | - | `GetMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about a specific monitor. This action requires the `monitorName` parameter. |
| `GetProbe` | `GET /monitors/{monitorName}/probes/{probeId}` | `readonly` | `monitorName`, `probeId` | - | `GetProbeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the details about a probe. This action requires both the `monitorName` and `probeId` parameters. |
| `ListMonitors` | `GET /monitors` | `readonly`, `paginated` | - | - | `ListMonitorsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all of your monitors. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags assigned to this resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds key-value pairs to a monitor or probe. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a key-value pair from a monitor or probe. |
| `UpdateMonitor` | `PATCH /monitors/{monitorName}` | `idempotent` | `aggregationPeriod`, `monitorName` | - | `UpdateMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the `aggregationPeriod` for a monitor. Monitors support an `aggregationPeriod` of either `30` or `60` seconds. |
| `UpdateProbe` | `PATCH /monitors/{monitorName}/probes/{probeId}` | `idempotent` | `monitorName`, `probeId` | - | `UpdateProbeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a monitor probe. This action requires both the `monitorName` and `probeId` parameters. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling |
| `ValidationException` | `structure` | `message` | One of the parameters for the request is not valid. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | `message` | This request exceeds a service quota. |
| `ConflictException` | `structure` | `message` | This operation attempted to create a resource that already exists. |
| `CreateMonitorInput` | `structure` | `aggregationPeriod`, `clientToken`, `monitorName`, `probes`, `tags` | - |
| `CreateMonitorOutput` | `structure` | `aggregationPeriod`, `monitorArn`, `monitorName`, `state`, `tags` | - |
| `CreateProbeInput` | `structure` | `clientToken`, `monitorName`, `probe`, `tags` | - |
| `CreateProbeOutput` | `structure` | `addressFamily`, `createdAt`, `destination`, `destinationPort`, `modifiedAt`, `packetSize`, `probeArn`, `probeId`, `protocol`, `sourceArn`, `state`, `tags`, ... (+1) | - |
| `DeleteMonitorInput` | `structure` | `monitorName` | - |
| `DeleteMonitorOutput` | `structure` | - | - |
| `DeleteProbeInput` | `structure` | `monitorName`, `probeId` | - |
| `DeleteProbeOutput` | `structure` | - | - |
| `GetMonitorInput` | `structure` | `monitorName` | - |
| `GetMonitorOutput` | `structure` | `aggregationPeriod`, `createdAt`, `modifiedAt`, `monitorArn`, `monitorName`, `probes`, `state`, `tags` | - |
| `GetProbeInput` | `structure` | `monitorName`, `probeId` | - |
| `GetProbeOutput` | `structure` | `addressFamily`, `createdAt`, `destination`, `destinationPort`, `modifiedAt`, `packetSize`, `probeArn`, `probeId`, `protocol`, `sourceArn`, `state`, `tags`, ... (+1) | - |
| `ListMonitorsInput` | `structure` | `maxResults`, `nextToken`, `state` | - |
| `ListMonitorsOutput` | `structure` | `monitors`, `nextToken` | - |
| `ListTagsForResourceInput` | `structure` | `resourceArn` | - |
| `ListTagsForResourceOutput` | `structure` | `tags` | - |
| `TagResourceInput` | `structure` | `resourceArn`, `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags assigned to this resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds key-value pairs to a monitor or probe. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a key-value pair from a monitor or probe. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | This operation attempted to create a resource that already exists. |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure. |
| `ResourceNotFoundException` | `structure` | message | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | This request exceeds a service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling |
| `ValidationException` | `structure` | message | One of the parameters for the request is not valid. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `AddressFamily` | `enum` | IPV4, IPV6 | - |
| `MonitorState` | `enum` | PENDING, ACTIVE, INACTIVE, ERROR, DELETING | - |
| `ProbeState` | `enum` | PENDING, ACTIVE, INACTIVE, ERROR, DELETING, DELETED | - |
| `Protocol` | `enum` | TCP, ICMP | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

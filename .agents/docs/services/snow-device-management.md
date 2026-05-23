# AWS Snow Device Management

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Snow Device Management documentation.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Snow Device Management resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Snow Device Management workflows in the local mock. Key resources include `Execution`, `ManagedDevice`, `Task`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Cancel`, `Create`, `Tag` operation families, including `ListDeviceResources`, `ListDevices`, `ListExecutions`, `ListTagsForResource`, `DescribeDevice`, `DescribeDeviceEc2Instances`.

## Service Identity and Protocol

- AWS model slug: `snow-device-management`
- AWS SDK for Rust slug: `snowdevicemanagement`
- Model version: `2021-08-04`
- Model file: `vendor/api-models-aws/models/snow-device-management/service/2021-08-04/snow-device-management-2021-08-04.json`
- SDK ID: `Snow Device Management`
- Endpoint prefix: `-`
- ARN namespace: `snow-device-management`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Describe` (4), `Cancel` (1), `Create` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelTask`, `CreateTask`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDevice`, `DescribeDeviceEc2Instances`, `DescribeExecution`, `DescribeTask`, `ListDeviceResources`, `ListDevices`, `ListExecutions`, `ListTagsForResource`, `ListTasks`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelTask`, `CreateTask`, `DescribeExecution`, `DescribeTask`, `ListExecutions`, `ListTasks`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `SQS`, `EC2/VPC`, `ECS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Execution` | `managedDeviceId`, `taskId` | read: `DescribeExecution`; list: `ListExecutions` | - | - |
| `ManagedDevice` | `managedDeviceId` | read: `DescribeDevice`; list: `ListDevices` | `DescribeDeviceEc2Instances`, `ListDeviceResources` | - |
| `Task` | `taskId` | create: `CreateTask`; read: `DescribeTask`; list: `ListTasks` | `CancelTask` | - |

## Current Network Resource Stub Semantics

Snow Device Management currently returns physical network interface data as device-local response metadata.

- Device descriptions can include `physical_network_interfaces`, but current handler responses use an empty list.
- The service has no VPC, subnet, security group, or ENI state map.
- Snow device network data is not reconciled with EC2 or on-premises network resources.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags for a managed device or task. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds or replaces tags on a device or task. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from a device or task. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | message | An unexpected error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | message | The request references a resource that doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | message | The request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

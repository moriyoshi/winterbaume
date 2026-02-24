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

- Operations: `ListDeviceResources`, `ListDevices`, `ListExecutions`, `ListTagsForResource`, `ListTasks`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `managedDeviceId`, `resourceArn`, `taskId`

### Describe

- Operations: `DescribeDevice`, `DescribeDeviceEc2Instances`, `DescribeExecution`, `DescribeTask`
- Traits: `readonly` (4)
- Common required input members in this group: `instanceIds`, `managedDeviceId`, `taskId`

### Cancel

- Operations: `CancelTask`
- Common required input members in this group: `taskId`

### Create

- Operations: `CreateTask`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `command`, `targets`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelTask` | `POST /task/{taskId}/cancel` | - | `taskId` | - | `CancelTaskOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends a cancel request for a specified task. You can cancel a task only if it's still in a `QUEUED` state. |
| `CreateTask` | `POST /task` | `idempotency-token` | `command`, `targets` | `clientToken` | `CreateTaskOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Instructs one or more devices to start a task, such as unlocking or rebooting. |
| `DescribeDevice` | `POST /managed-device/{managedDeviceId}/describe` | `readonly` | `managedDeviceId` | - | `DescribeDeviceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Checks device-specific information, such as the device type, software version, IP addresses, and lock status. |
| `DescribeDeviceEc2Instances` | `POST /managed-device/{managedDeviceId}/resources/ec2/describe` | `readonly` | `instanceIds`, `managedDeviceId` | - | `DescribeDeviceEc2Output` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Checks the current state of the Amazon EC2 instances. The output is similar to `describeDevice`, but the results are sourced from the device cache in the Amazon Web Services Cloud and include a subset of the available fields. |
| `DescribeExecution` | `POST /task/{taskId}/execution/{managedDeviceId}` | `readonly` | `managedDeviceId`, `taskId` | - | `DescribeExecutionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Checks the status of a remote task running on one or more target devices. |
| `DescribeTask` | `POST /task/{taskId}` | `readonly` | `taskId` | - | `DescribeTaskOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Checks the metadata for a given task on a device. |
| `ListDeviceResources` | `GET /managed-device/{managedDeviceId}/resources` | `readonly`, `paginated` | `managedDeviceId` | - | `ListDeviceResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the Amazon Web Services resources available for a device. Currently, Amazon EC2 instances are the only supported resource type. |
| `ListDevices` | `GET /managed-devices` | `readonly`, `paginated` | - | - | `ListDevicesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all devices on your Amazon Web Services account that have Amazon Web Services Snow Device Management enabled in the Amazon Web Services Region where the command is run. |
| `ListExecutions` | `GET /executions` | `readonly`, `paginated` | `taskId` | - | `ListExecutionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the status of tasks for one or more target devices. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags for a managed device or task. |
| `ListTasks` | `GET /tasks` | `readonly`, `paginated` | - | - | `ListTasksOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of tasks that can be filtered by state. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds or replaces tags on a device or task. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from a device or task. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An unexpected error occurred while processing the request. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message` | The request references a resource that doesn't exist. |
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `CancelTaskInput` | `structure` | `taskId` | - |
| `CancelTaskOutput` | `structure` | `taskId` | - |
| `CreateTaskInput` | `structure` | `clientToken`, `command`, `description`, `tags`, `targets` | - |
| `CreateTaskOutput` | `structure` | `taskArn`, `taskId` | - |
| `ServiceQuotaExceededException` | `structure` | `message` | The request would cause a service quota to be exceeded. |
| `DescribeDeviceInput` | `structure` | `managedDeviceId` | - |
| `DescribeDeviceOutput` | `structure` | `associatedWithJob`, `deviceCapacities`, `deviceState`, `deviceType`, `lastReachedOutAt`, `lastUpdatedAt`, `managedDeviceArn`, `managedDeviceId`, `physicalNetworkInterfaces`, `software`, `tags` | - |
| `DescribeDeviceEc2Input` | `structure` | `instanceIds`, `managedDeviceId` | - |
| `DescribeDeviceEc2Output` | `structure` | `instances` | - |
| `DescribeExecutionInput` | `structure` | `managedDeviceId`, `taskId` | - |
| `DescribeExecutionOutput` | `structure` | `executionId`, `lastUpdatedAt`, `managedDeviceId`, `startedAt`, `state`, `taskId` | - |
| `DescribeTaskInput` | `structure` | `taskId` | - |
| `DescribeTaskOutput` | `structure` | `completedAt`, `createdAt`, `description`, `lastUpdatedAt`, `state`, `tags`, `targets`, `taskArn`, `taskId` | - |
| `ListDeviceResourcesInput` | `structure` | `managedDeviceId`, `maxResults`, `nextToken`, `type` | - |
| `ListDeviceResourcesOutput` | `structure` | `nextToken`, `resources` | - |
| `ListDevicesInput` | `structure` | `jobId`, `maxResults`, `nextToken` | - |
| `ListDevicesOutput` | `structure` | `devices`, `nextToken` | - |
| `ListExecutionsInput` | `structure` | `maxResults`, `nextToken`, `state`, `taskId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

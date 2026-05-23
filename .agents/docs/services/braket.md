# Braket

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Braket API Reference provides information about the operations and structures supported by Amazon Braket. To learn about the permissions required to call an Amazon Braket API action, see Actions, resources, and condition keys for Amazon Braket. Amazon Braket Python SDK and the AWS Command Line Interface can be used to make discovery and creation of API calls easier. For more information about Amazon Braket features, see What is Amazon Braket? and important terms and concepts in the Amazon Braket Developer Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Braket resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create and monitor quantum tasks and jobs against Braket devices.
- From the operation surface: support device discovery, task cancellation, job lifecycle management, result location references, and tag-based quantum workload tracking.

## Service Identity and Protocol

- AWS model slug: `braket`
- AWS SDK for Rust slug: `braket`
- Model version: `2019-09-01`
- Model file: `vendor/api-models-aws/models/braket/service/2019-09-01/braket-2019-09-01.json`
- SDK ID: `Braket`
- Endpoint prefix: `-`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `braket.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Search` (4), `Create` (3), `Get` (3), `Cancel` (2), `Delete` (1), `List` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelJob`, `CancelQuantumTask`, `CreateJob`, `CreateQuantumTask`, `CreateSpendingLimit`, `DeleteSpendingLimit`, `TagResource`, `UntagResource`, `UpdateSpendingLimit`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDevice`, `GetJob`, `GetQuantumTask`, `ListTagsForResource`, `SearchDevices`, `SearchJobs`, `SearchQuantumTasks`, `SearchSpendingLimits`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelJob`, `CancelQuantumTask`, `CreateJob`, `CreateQuantumTask`, `GetJob`, `GetQuantumTask`, `SearchJobs`, `SearchQuantumTasks`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`, `SQS`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DeviceResource` | `deviceArn` | read: `GetDevice`; list: `SearchDevices` | - | - |
| `JobResource` | `jobArn` | create: `CreateJob`; read: `GetJob`; update: `CancelJob` | `SearchJobs` | - |
| `QuantumTaskResource` | `quantumTaskArn` | create: `CreateQuantumTask`; read: `GetQuantumTask`; update: `CancelQuantumTask`; list: `SearchQuantumTasks` | - | - |
| `SpendingLimitResource` | `spendingLimitArn` | create: `CreateSpendingLimit`; update: `UpdateSpendingLimit`; delete: `DeleteSpendingLimit`; list: `SearchSpendingLimits` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/braket/latest/developerguide/braket-monitor-tasks-sdk.html
- https://docs.aws.amazon.com/braket/latest/developerguide/braket-submit-tasks-to-braket.html
- https://docs.aws.amazon.com/braket/latest/developerguide/braket-submit-tasks-simulators.html

Research outcomes:
- `CreateQuantumTask` creates an asynchronous quantum task with a unique ARN. The client can continue while the task is processed.
- Quantum tasks are queued until target device capacity is available, then run, then write results to the configured S3 location.
- Task states observed through the SDK include `QUEUED`, `RUNNING`, `COMPLETED`, and `CANCELLING`, with cancellation requested through the task cancel operation.
- The S3 output location is required for all devices except the local simulator. Task metadata includes output bucket and output directory.
- A task can be reconstructed later from its quantum task ARN and then used to fetch results from S3.
- Devices advertise supported action types, operations, result types, and shot limits through device properties.
- The quantum task action payload is limited to 5 MB.
- Shots control measurement sampling. For shots equal to zero, supported simulators perform exact simulation and return true values for result types. QPU devices require shots greater than zero.
- For non-zero shots, simulators sample from the output distribution to emulate QPU shot noise.
- SDK result polling uses a timeout and interval. The default polling timeout is five days, and QPU queues can require long polling windows when devices are unavailable.

Parity implications:
- Model devices, device properties, quantum tasks, task states, S3 output locations, result metadata, cancellation, and shot validation separately.
- Task execution should be asynchronous, capacity-queued, and device-capability aware.
- Result retrieval should depend on task completion and S3 output metadata rather than returning results synchronously from create.

## Operation Groups

### Search

- Operations: `SearchDevices`, `SearchJobs`, `SearchQuantumTasks`, `SearchSpendingLimits`
- Traits: `paginated` (4), `readonly` (3)
- Common required input members in this group: `filters`

### Create

- Operations: `CreateJob`, `CreateQuantumTask`, `CreateSpendingLimit`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `action`, `algorithmSpecification`, `clientToken`, `deviceArn`, `deviceConfig`, `instanceConfig`, `jobName`, `outputDataConfig`, `outputS3Bucket`, `outputS3KeyPrefix`, `roleArn`, `shots`, `spendingLimit`

### Get

- Operations: `GetDevice`, `GetJob`, `GetQuantumTask`
- Traits: `readonly` (3)
- Common required input members in this group: `deviceArn`, `jobArn`, `quantumTaskArn`

### Cancel

- Operations: `CancelJob`, `CancelQuantumTask`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `clientToken`, `jobArn`, `quantumTaskArn`

### Delete

- Operations: `DeleteSpendingLimit`
- Traits: `idempotent` (1)
- Common required input members in this group: `spendingLimitArn`

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: `resourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateSpendingLimit`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `clientToken`, `spendingLimitArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelJob` | `PUT /job/{jobArn}/cancel` | `idempotent` | `jobArn` | - | `CancelJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an Amazon Braket hybrid job. |
| `CancelQuantumTask` | `PUT /quantum-task/{quantumTaskArn}/cancel` | `idempotent`, `idempotency-token` | `clientToken`, `quantumTaskArn` | `clientToken` | `CancelQuantumTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels the specified task. |
| `CreateJob` | `POST /job` | `idempotency-token` | `algorithmSpecification`, `clientToken`, `deviceConfig`, `instanceConfig`, `jobName`, `outputDataConfig`, `roleArn` | `clientToken` | `CreateJobResponse` | `AccessDeniedException`, `ConflictException`, `DeviceOfflineException`, `DeviceRetiredException`, `InternalServiceException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Braket hybrid job. |
| `CreateQuantumTask` | `POST /quantum-task` | `idempotency-token` | `action`, `clientToken`, `deviceArn`, `outputS3Bucket`, `outputS3KeyPrefix`, `shots` | `clientToken` | `CreateQuantumTaskResponse` | `AccessDeniedException`, `DeviceOfflineException`, `DeviceRetiredException`, `InternalServiceException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a quantum task. |
| `CreateSpendingLimit` | `POST /spending-limit` | `idempotency-token` | `clientToken`, `deviceArn`, `spendingLimit` | `clientToken` | `CreateSpendingLimitResponse` | `AccessDeniedException`, `DeviceRetiredException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Creates a spending limit for a specified quantum device. Spending limits help you control costs by setting maximum amounts that can be spent on quantum computing tasks within a specified time period. |
| `DeleteSpendingLimit` | `DELETE /spending-limit/{spendingLimitArn}/delete` | `idempotent` | `spendingLimitArn` | - | `DeleteSpendingLimitResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing spending limit. This operation permanently removes the spending limit and cannot be undone. |
| `GetDevice` | `GET /device/{deviceArn}` | `readonly` | `deviceArn` | - | `GetDeviceResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the devices available in Amazon Braket. For backwards compatibility with older versions of BraketSchemas, OpenQASM information is omitted from GetDevice API calls. |
| `GetJob` | `GET /job/{jobArn}` | `readonly` | `jobArn` | - | `GetJobResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified Amazon Braket hybrid job. |
| `GetQuantumTask` | `GET /quantum-task/{quantumTaskArn}` | `readonly` | `quantumTaskArn` | - | `GetQuantumTaskResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified quantum task. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceException`, `ResourceNotFoundException`, `ValidationException` | Shows the tags associated with this resource. |
| `SearchDevices` | `POST /devices` | `readonly`, `paginated` | `filters` | - | `SearchDevicesResponse` | `AccessDeniedException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Searches for devices using the specified filters. |
| `SearchJobs` | `POST /jobs` | `paginated` | `filters` | - | `SearchJobsResponse` | `AccessDeniedException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Searches for Amazon Braket hybrid jobs that match the specified filter values. |
| `SearchQuantumTasks` | `POST /quantum-tasks` | `readonly`, `paginated` | `filters` | - | `SearchQuantumTasksResponse` | `AccessDeniedException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Searches for tasks that match the specified filter values. |
| `SearchSpendingLimits` | `POST /spending-limits` | `readonly`, `paginated` | - | - | `SearchSpendingLimitsResponse` | `AccessDeniedException`, `InternalServiceException`, `ThrottlingException`, `ValidationException` | Searches and lists spending limits based on specified filters. This operation supports pagination and allows filtering by various criteria to find specific spending limits. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServiceException`, `ResourceNotFoundException`, `ValidationException` | Add a tag to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServiceException`, `ResourceNotFoundException`, `ValidationException` | Remove tags from a resource. |
| `UpdateSpendingLimit` | `PATCH /spending-limit/{spendingLimitArn}/update` | `idempotent`, `idempotency-token` | `clientToken`, `spendingLimitArn` | `clientToken` | `UpdateSpendingLimitResponse` | `AccessDeniedException`, `InternalServiceException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing spending limit. You can modify the spending amount or time period. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceException` | `structure` | `message` | The request failed because of an unknown error. |
| `ValidationException` | `structure` | `message`, `programSetValidationFailures`, `reason` | The input request failed to satisfy constraints expected by Amazon Braket. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient permissions to perform this action. |
| `ThrottlingException` | `structure` | `message` | The API throttling rate limit is exceeded. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource was not found. |
| `ConflictException` | `structure` | `message` | An error occurred due to a conflict. |
| `DeviceRetiredException` | `structure` | `message` | The specified device has been retired. |
| `DeviceOfflineException` | `structure` | `message` | The specified device is currently offline. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request failed because a service quota is exceeded. |
| `CancelJobRequest` | `structure` | `jobArn` | - |
| `CancelJobResponse` | `structure` | `cancellationStatus`, `jobArn` | - |
| `CancelQuantumTaskRequest` | `structure` | `clientToken`, `quantumTaskArn` | - |
| `CancelQuantumTaskResponse` | `structure` | `cancellationStatus`, `quantumTaskArn` | - |
| `CreateJobRequest` | `structure` | `algorithmSpecification`, `associations`, `checkpointConfig`, `clientToken`, `deviceConfig`, `hyperParameters`, `inputDataConfig`, `instanceConfig`, `jobName`, `outputDataConfig`, `roleArn`, `stoppingCondition`, ... (+1) | - |
| `CreateJobResponse` | `structure` | `jobArn` | - |
| `CreateQuantumTaskRequest` | `structure` | `action`, `associations`, `clientToken`, `deviceArn`, `deviceParameters`, `experimentalCapabilities`, `jobToken`, `outputS3Bucket`, `outputS3KeyPrefix`, `shots`, `tags` | - |
| `CreateQuantumTaskResponse` | `structure` | `quantumTaskArn` | - |
| `CreateSpendingLimitRequest` | `structure` | `clientToken`, `deviceArn`, `spendingLimit`, `tags`, `timePeriod` | - |
| `CreateSpendingLimitResponse` | `structure` | `spendingLimitArn` | - |
| `DeleteSpendingLimitRequest` | `structure` | `spendingLimitArn` | - |
| `DeleteSpendingLimitResponse` | `structure` | - | - |
| `GetDeviceRequest` | `structure` | `deviceArn` | - |
| `GetDeviceResponse` | `structure` | `deviceArn`, `deviceCapabilities`, `deviceName`, `deviceQueueInfo`, `deviceStatus`, `deviceType`, `providerName` | - |
| `GetJobRequest` | `structure` | `additionalAttributeNames`, `jobArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceException`, `ResourceNotFoundException`, `ValidationException` | Shows the tags associated with this resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServiceException`, `ResourceNotFoundException`, `ValidationException` | Add a tag to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServiceException`, `ResourceNotFoundException`, `ValidationException` | Remove tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient permissions to perform this action. |
| `ConflictException` | `structure` | message | An error occurred due to a conflict. |
| `DeviceOfflineException` | `structure` | message | The specified device is currently offline. |
| `DeviceRetiredException` | `structure` | message | The specified device has been retired. |
| `InternalServiceException` | `structure` | message | The request failed because of an unknown error. |
| `ResourceNotFoundException` | `structure` | message | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message | The request failed because a service quota is exceeded. |
| `ThrottlingException` | `structure` | message | The API throttling rate limit is exceeded. |
| `ValidationException` | `structure` | message, reason, programSetValidationFailures | The input request failed to satisfy constraints expected by Amazon Braket. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS IoT Jobs Data Plane

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IoT Jobs is a service that allows you to define a set of jobs — remote operations that are sent to and executed on one or more devices connected to Amazon Web Services IoT Core. For example, you can define a job that instructs a set of devices to download and install application or firmware updates, reboot, rotate certificates, or perform remote troubleshooting operations. Find the endpoint address for actions in the IoT jobs data plane by running this CLI command: `aws iot describe-endpoint --endpoint-type iot:Jobs` The service name used by Amazon Web Services Signature Version 4 to sign requests is: iot-jobs-data . To create a job, you make a job document which is a description of the remote operations to be performed, and you specify a list of targets that should perform the operations. The targets can be individual things, thing groups or both.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS IoT Jobs Data Plane resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS IoT Jobs Data Plane workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Start`, `Describe`, `Get`, `Update` operation families, including `StartCommandExecution`, `StartNextPendingJobExecution`, `DescribeJobExecution`, `GetPendingJobExecutions`, `UpdateJobExecution`.

## Service Identity and Protocol

- AWS model slug: `iot-jobs-data-plane`
- AWS SDK for Rust slug: `iotjobsdataplane`
- Model version: `2017-09-29`
- Model file: `vendor/api-models-aws/models/iot-jobs-data-plane/service/2017-09-29/iot-jobs-data-plane-2017-09-29.json`
- SDK ID: `IoT Jobs Data Plane`
- Endpoint prefix: `data.jobs.iot`
- ARN namespace: `iot-jobs-data`
- CloudFormation name: `IoTJobsDataPlane`
- CloudTrail event source: `iotjobsdataplane.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Start` (2), `Describe` (1), `Get` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartCommandExecution`, `StartNextPendingJobExecution`, `UpdateJobExecution`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeJobExecution`, `GetPendingJobExecutions`.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeJobExecution`, `GetPendingJobExecutions`, `StartCommandExecution`, `StartNextPendingJobExecution`, `UpdateJobExecution`.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Start

- Operations: `StartCommandExecution`, `StartNextPendingJobExecution`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Describe

- Operations: `DescribeJobExecution`
- Common required input members in this group: -

### Get

- Operations: `GetPendingJobExecutions`
- Common required input members in this group: -

### Update

- Operations: `UpdateJobExecution`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeJobExecution` | `GET /things/{thingName}/jobs/{jobId}` | - | `jobId`, `thingName` | - | `DescribeJobExecutionResponse` | `CertificateValidationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `TerminalStateException`, `ThrottlingException` | Gets details of a job execution. Requires permission to access the DescribeJobExecution action. |
| `GetPendingJobExecutions` | `GET /things/{thingName}/jobs` | - | `thingName` | - | `GetPendingJobExecutionsResponse` | `CertificateValidationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Gets the list of all jobs for a thing that are not in a terminal status. Requires permission to access the GetPendingJobExecutions action. |
| `StartCommandExecution` | `POST /command-executions` | `idempotency-token` | `targetArn`, `commandArn` | `clientToken` | `StartCommandExecutionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Using the command created with the CreateCommand API, start a command execution on a specific device. |
| `StartNextPendingJobExecution` | `PUT /things/{thingName}/jobs/$next` | - | `thingName` | - | `StartNextPendingJobExecutionResponse` | `CertificateValidationException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Gets and starts the next pending (status IN_PROGRESS or QUEUED) job execution for a thing. Requires permission to access the StartNextPendingJobExecution action. |
| `UpdateJobExecution` | `POST /things/{thingName}/jobs/{jobId}` | - | `jobId`, `thingName`, `status` | - | `UpdateJobExecutionResponse` | `CertificateValidationException`, `InvalidRequestException`, `InvalidStateTransitionException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Updates the status of a job execution. Requires permission to access the UpdateJobExecution action. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeJobExecution` | - | `includeJobDocument -> includeJobDocument`, `executionNumber -> executionNumber` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CertificateValidationException` | `structure` | message | The certificate is invalid. |
| `ConflictException` | `structure` | message, resourceId | A conflict has occurred when performing the API request. |
| `InternalServerException` | `structure` | message | An internal server error occurred when performing the API request. |
| `InvalidRequestException` | `structure` | message | The contents of the request were invalid. |
| `InvalidStateTransitionException` | `structure` | message | An update attempted to change the job execution to a state that is invalid because of the job execution's current state (for example, an attempt to change a ... |
| `ResourceNotFoundException` | `structure` | message | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | The service quota has been exceeded for this request. |
| `ServiceUnavailableException` | `structure` | message | The service is temporarily unavailable. |
| `TerminalStateException` | `structure` | message | The job is in a terminal state. |
| `ThrottlingException` | `structure` | message, payload | The rate exceeds the limit. |
| `ValidationException` | `structure` | message | A validation error occurred when performing the API request. |
| `DescribeJobExecutionRequest` | `structure` | jobId, thingName, includeJobDocument, executionNumber | - |
| `DescribeJobExecutionResponse` | `structure` | execution | - |
| `GetPendingJobExecutionsRequest` | `structure` | thingName | - |
| `GetPendingJobExecutionsResponse` | `structure` | inProgressJobs, queuedJobs | - |
| `StartCommandExecutionRequest` | `structure` | targetArn, commandArn, parameters, executionTimeoutSeconds, clientToken | - |
| `StartCommandExecutionResponse` | `structure` | executionId | - |
| `StartNextPendingJobExecutionRequest` | `structure` | thingName, statusDetails, stepTimeoutInMinutes | - |
| `StartNextPendingJobExecutionResponse` | `structure` | execution | - |
| `UpdateJobExecutionRequest` | `structure` | jobId, thingName, status, statusDetails, stepTimeoutInMinutes, expectedVersion, includeJobExecutionState, includeJobDocument, executionNumber | - |
| `UpdateJobExecutionResponse` | `structure` | executionState, jobDocument | - |
| `JobExecutionStatus` | `enum` | QUEUED, IN_PROGRESS, SUCCEEDED, FAILED, TIMED_OUT, REJECTED, REMOVED, CANCELED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

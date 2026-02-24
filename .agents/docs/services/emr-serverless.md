# EMR Serverless

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EMR Serverless is a new deployment option for Amazon EMR. Amazon EMR Serverless provides a serverless runtime environment that simplifies running analytics applications using the latest open source frameworks such as Apache Spark and Apache Hive. With Amazon EMR Serverless, you don’t have to configure, optimize, secure, or operate clusters to run applications with these frameworks. The API reference to Amazon EMR Serverless is `emr-serverless`. The `emr-serverless` prefix is used in the following scenarios: It is the prefix in the CLI commands for Amazon EMR Serverless.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for EMR Serverless resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented EMR Serverless workflows in the local mock. Key resources include `ApplicationResource`, `JobRunResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Start`, `Cancel`, `Create` operation families, including `ListApplications`, `ListJobRunAttempts`, `ListJobRuns`, `ListTagsForResource`, `GetApplication`, `GetDashboardForJobRun`.

## Service Identity and Protocol

- AWS model slug: `emr-serverless`
- AWS SDK for Rust slug: `emrserverless`
- Model version: `2021-07-13`
- Model file: `vendor/api-models-aws/models/emr-serverless/service/2021-07-13/emr-serverless-2021-07-13.json`
- SDK ID: `EMR Serverless`
- Endpoint prefix: `-`
- ARN namespace: `emr-serverless`
- CloudFormation name: `EMRServerless`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Start` (2), `Cancel` (1), `Create` (1), `Delete` (1), `Stop` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelJobRun`, `CreateApplication`, `DeleteApplication`, `StartApplication`, `StartJobRun`, `StopApplication`, `TagResource`, `UntagResource`, `UpdateApplication`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetDashboardForJobRun`, `GetJobRun`, `ListApplications`, `ListJobRunAttempts`, `ListJobRuns`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelJobRun`, `GetDashboardForJobRun`, `GetJobRun`, `ListJobRunAttempts`, `ListJobRuns`, `StartApplication`, `StartJobRun`, `StopApplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApplicationResource` | `applicationId` | create: `CreateApplication`; read: `GetApplication`; update: `UpdateApplication`; delete: `DeleteApplication`; list: `ListApplications` | `StartApplication`, `StopApplication` | - |
| `JobRunResource` | `applicationId`, `jobRunId` | create: `StartJobRun`; read: `GetJobRun`; delete: `CancelJobRun`; list: `ListJobRuns` | `GetDashboardForJobRun`, `ListJobRunAttempts` | - |

## Current Network Resource Stub Semantics

EMR Serverless currently stores application network configuration locally.

- Application records can include a network configuration with subnet IDs and security group IDs.
- Start, stop, and job-run state transitions do not allocate ENIs or check subnet capacity.
- Updating or reading an application only observes the stored EMR Serverless metadata.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListApplications`, `ListJobRunAttempts`, `ListJobRuns`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `applicationId`, `jobRunId`, `resourceArn`

### Get

- Operations: `GetApplication`, `GetDashboardForJobRun`, `GetJobRun`
- Traits: `readonly` (2)
- Common required input members in this group: `applicationId`, `jobRunId`

### Start

- Operations: `StartApplication`, `StartJobRun`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `applicationId`, `clientToken`, `executionRoleArn`

### Cancel

- Operations: `CancelJobRun`
- Traits: `idempotent` (1)
- Common required input members in this group: `applicationId`, `jobRunId`

### Create

- Operations: `CreateApplication`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `clientToken`, `releaseLabel`, `type`

### Delete

- Operations: `DeleteApplication`
- Traits: `idempotent` (1)
- Common required input members in this group: `applicationId`

### Stop

- Operations: `StopApplication`
- Traits: `idempotent` (1)
- Common required input members in this group: `applicationId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateApplication`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `applicationId`, `clientToken`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelJobRun` | `DELETE /applications/{applicationId}/jobruns/{jobRunId}` | `idempotent` | `applicationId`, `jobRunId` | - | `CancelJobRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Cancels a job run. |
| `CreateApplication` | `POST /applications` | `idempotent`, `idempotency-token` | `clientToken`, `releaseLabel`, `type` | `clientToken` | `CreateApplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates an application. |
| `DeleteApplication` | `DELETE /applications/{applicationId}` | `idempotent` | `applicationId` | - | `DeleteApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an application. An application has to be in a stopped or created state in order to be deleted. |
| `GetApplication` | `GET /applications/{applicationId}` | `readonly` | `applicationId` | - | `GetApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a specified application. |
| `GetDashboardForJobRun` | `GET /applications/{applicationId}/jobruns/{jobRunId}/dashboard` | - | `applicationId`, `jobRunId` | - | `GetDashboardForJobRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates and returns a URL that you can use to access the application UIs for a job run. For jobs in a running state, the application UI is a live user interface such as the Spark or Tez web UI. |
| `GetJobRun` | `GET /applications/{applicationId}/jobruns/{jobRunId}` | `readonly` | `applicationId`, `jobRunId` | - | `GetJobRunResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays detailed information about a job run. |
| `ListApplications` | `GET /applications` | `readonly`, `paginated` | - | - | `ListApplicationsResponse` | `InternalServerException`, `ValidationException` | Lists applications based on a set of parameters. |
| `ListJobRunAttempts` | `GET /applications/{applicationId}/jobruns/{jobRunId}/attempts` | `readonly`, `paginated` | `applicationId`, `jobRunId` | - | `ListJobRunAttemptsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all attempt of a job run. |
| `ListJobRuns` | `GET /applications/{applicationId}/jobruns` | `readonly`, `paginated` | `applicationId` | - | `ListJobRunsResponse` | `InternalServerException`, `ValidationException` | Lists job runs based on a set of parameters. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags assigned to the resources. |
| `StartApplication` | `POST /applications/{applicationId}/start` | `idempotent` | `applicationId` | - | `StartApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Starts a specified application and initializes initial capacity if configured. |
| `StartJobRun` | `POST /applications/{applicationId}/jobruns` | `idempotent`, `idempotency-token` | `applicationId`, `clientToken`, `executionRoleArn` | `clientToken` | `StartJobRunResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Starts a job run. |
| `StopApplication` | `POST /applications/{applicationId}/stop` | `idempotent` | `applicationId` | - | `StopApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops a specified application and releases initial capacity if configured. All scheduled and running jobs must be completed or cancelled before stopping an application. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns tags to resources. A tag is a label that you assign to an Amazon Web Services resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from resources. |
| `UpdateApplication` | `PATCH /applications/{applicationId}` | `idempotency-token` | `applicationId`, `clientToken` | `clientToken` | `UpdateApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a specified application. An application has to be in a stopped or created state in order to be updated. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | Request processing failed because of an error or failure with the service. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource was not found. |
| `ConflictException` | `structure` | `message` | The request could not be processed because of conflict in the current state of the resource. |
| `CancelJobRunRequest` | `structure` | `applicationId`, `jobRunId`, `shutdownGracePeriodInSeconds` | - |
| `CancelJobRunResponse` | `structure` | `applicationId`, `jobRunId` | - |
| `CreateApplicationRequest` | `structure` | `architecture`, `autoStartConfiguration`, `autoStopConfiguration`, `clientToken`, `diskEncryptionConfiguration`, `identityCenterConfiguration`, `imageConfiguration`, `initialCapacity`, `interactiveConfiguration`, `jobLevelCostAllocationConfiguration`, `maximumCapacity`, `monitoringConfiguration`, ... (+8) | - |
| `CreateApplicationResponse` | `structure` | `applicationId`, `arn`, `name` | - |
| `DeleteApplicationRequest` | `structure` | `applicationId` | - |
| `DeleteApplicationResponse` | `structure` | - | - |
| `GetApplicationRequest` | `structure` | `applicationId` | - |
| `GetApplicationResponse` | `structure` | `application` | - |
| `GetDashboardForJobRunRequest` | `structure` | `accessSystemProfileLogs`, `applicationId`, `attempt`, `jobRunId` | - |
| `GetDashboardForJobRunResponse` | `structure` | `url` | - |
| `GetJobRunRequest` | `structure` | `applicationId`, `attempt`, `jobRunId` | - |
| `GetJobRunResponse` | `structure` | `jobRun` | - |
| `ListApplicationsRequest` | `structure` | `maxResults`, `nextToken`, `states` | - |
| `ListApplicationsResponse` | `structure` | `applications`, `nextToken` | - |
| `ListJobRunAttemptsRequest` | `structure` | `applicationId`, `jobRunId`, `maxResults`, `nextToken` | - |
| `ListJobRunAttemptsResponse` | `structure` | `jobRunAttempts`, `nextToken` | - |
| `ListJobRunsRequest` | `structure` | `applicationId`, `createdAtAfter`, `createdAtBefore`, `maxResults`, `mode`, `nextToken`, `states` | - |
| `ListJobRunsResponse` | `structure` | `jobRuns`, `nextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

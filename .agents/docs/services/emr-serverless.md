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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags assigned to the resources. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns tags to resources. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value, both of which you define. Tags enable you to categorize your Amazon We ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from resources. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | message | The request could not be processed because of conflict in the current state of the resource. |
| `InternalServerException` | `structure` | message | Request processing failed because of an error or failure with the service. |
| `ResourceNotFoundException` | `structure` | message | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message | The maximum number of resources per account has been reached. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
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

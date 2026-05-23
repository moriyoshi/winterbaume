# Amazon CodeGuru Profiler

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This section provides documentation for the Amazon CodeGuru Profiler API operations. Amazon CodeGuru Profiler collects runtime performance data from your live applications, and provides recommendations that can help you fine-tune your application performance. Using machine learning algorithms, CodeGuru Profiler can help you find your most expensive lines of code and suggest ways you can improve efficiency and remove CPU bottlenecks. Amazon CodeGuru Profiler provides different visualizations of profiling data to help you identify what code is running on the CPU, see how much time is consumed, and suggest ways to reduce CPU utilization. Amazon CodeGuru Profiler currently supports applications written in all Java virtual machine (JVM) languages and Python.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon CodeGuru Profiler resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon CodeGuru Profiler workflows in the local mock. Key resources include `ProfilingGroup`.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `List`, `Remove`, `Add`, `Batch` operation families, including `GetFindingsReportAccountSummary`, `GetNotificationConfiguration`, `GetPolicy`, `GetProfile`, `ListFindingsReports`, `ListProfileTimes`.

## Service Identity and Protocol

- AWS model slug: `codeguruprofiler`
- AWS SDK for Rust slug: `codeguruprofiler`
- Model version: `2019-07-18`
- Model file: `vendor/api-models-aws/models/codeguruprofiler/service/2019-07-18/codeguruprofiler-2019-07-18.json`
- SDK ID: `CodeGuruProfiler`
- Endpoint prefix: `codeguru-profiler`
- ARN namespace: `codeguru-profiler`
- CloudFormation name: `CodeGuruProfiler`
- CloudTrail event source: `codeguru-profiler.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (4), `Remove` (2), `Add` (1), `Batch` (1), `Configure` (1), `Create` (1), `Delete` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddNotificationChannels`, `BatchGetFrameMetricData`, `CreateProfilingGroup`, `DeleteProfilingGroup`, `PutPermission`, `RemoveNotificationChannel`, `RemovePermission`, `SubmitFeedback`, `TagResource`, `UntagResource`, `UpdateProfilingGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetFrameMetricData`, `DescribeProfilingGroup`, `GetFindingsReportAccountSummary`, `GetNotificationConfiguration`, `GetPolicy`, `GetProfile`, `GetRecommendations`, `ListFindingsReports`, `ListProfileTimes`, `ListProfilingGroups`, `ListTagsForResource`, `SubmitFeedback`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetFindingsReportAccountSummary`, `ListFindingsReports`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ProfilingGroup` | `profilingGroupName` | put: `CreateProfilingGroup`; read: `DescribeProfilingGroup`; update: `UpdateProfilingGroup`; delete: `DeleteProfilingGroup`; list: `ListProfilingGroups` | `AddNotificationChannels`, `BatchGetFrameMetricData`, `ConfigureAgent`, `GetNotificationConfiguration`, `GetPolicy`, `GetProfile`, `GetRecommendations`, `ListFindingsReports`, `ListProfileTimes`, `PostAgentProfile`, ... (+4) | - |
## Operation Groups

### Get

- Operations: `GetFindingsReportAccountSummary`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

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
| `GetFindingsReportAccountSummary` | `GET /internal/findingsReports` | `readonly`, `paginated` | - | - | `GetFindingsReportAccountSummaryResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of FindingsReportSummary objects that contain analysis results for all profiling groups in your AWS account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of the tags that are assigned to a specified resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Use to assign one or more tags to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Use to remove one or more tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetFindingsReportAccountSummary` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `dailyReportsOnly -> dailyReportsOnly` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | message | The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retryin ... |
| `InternalServerException` | `structure` | message | The server encountered an internal error and is unable to complete the request. |
| `ResourceNotFoundException` | `structure` | message | The resource specified in the request does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quo ... |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The parameter is not valid. |
| `GetFindingsReportAccountSummaryRequest` | `structure` | nextToken, maxResults, dailyReportsOnly | The structure representing the GetFindingsReportAccountSummaryRequest. |
| `GetFindingsReportAccountSummaryResponse` | `structure` | reportSummaries, nextToken | The structure representing the GetFindingsReportAccountSummaryResponse. |
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

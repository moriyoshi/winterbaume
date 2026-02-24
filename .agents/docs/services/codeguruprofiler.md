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

- Operations: `GetFindingsReportAccountSummary`, `GetNotificationConfiguration`, `GetPolicy`, `GetProfile`, `GetRecommendations`
- Traits: `paginated` (1), `readonly` (5)
- Common required input members in this group: `endTime`, `profilingGroupName`, `startTime`

### List

- Operations: `ListFindingsReports`, `ListProfileTimes`, `ListProfilingGroups`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `endTime`, `period`, `profilingGroupName`, `resourceArn`, `startTime`

### Remove

- Operations: `RemoveNotificationChannel`, `RemovePermission`
- Traits: `idempotent` (1)
- Common required input members in this group: `actionGroup`, `channelId`, `profilingGroupName`, `revisionId`

### Add

- Operations: `AddNotificationChannels`
- Common required input members in this group: `channels`, `profilingGroupName`

### Batch

- Operations: `BatchGetFrameMetricData`
- Traits: `readonly` (1)
- Common required input members in this group: `profilingGroupName`

### Configure

- Operations: `ConfigureAgent`
- Common required input members in this group: `profilingGroupName`

### Create

- Operations: `CreateProfilingGroup`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `clientToken`, `profilingGroupName`

### Delete

- Operations: `DeleteProfilingGroup`
- Traits: `idempotent` (1)
- Common required input members in this group: `profilingGroupName`

### Describe

- Operations: `DescribeProfilingGroup`
- Traits: `readonly` (1)
- Common required input members in this group: `profilingGroupName`

### Post

- Operations: `PostAgentProfile`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `agentProfile`, `contentType`, `profilingGroupName`

### Put

- Operations: `PutPermission`
- Traits: `idempotent` (1)
- Common required input members in this group: `actionGroup`, `principals`, `profilingGroupName`

### Submit

- Operations: `SubmitFeedback`
- Traits: `readonly` (1)
- Common required input members in this group: `anomalyInstanceId`, `profilingGroupName`, `type`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateProfilingGroup`
- Traits: `idempotent` (1)
- Common required input members in this group: `agentOrchestrationConfig`, `profilingGroupName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddNotificationChannels` | `POST /profilingGroups/{profilingGroupName}/notificationConfiguration` | - | `channels`, `profilingGroupName` | - | `AddNotificationChannelsResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Add up to 2 anomaly notifications channels for a profiling group. |
| `BatchGetFrameMetricData` | `POST /profilingGroups/{profilingGroupName}/frames/-/metrics` | `readonly` | `profilingGroupName` | - | `BatchGetFrameMetricDataResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the time series of values for a requested list of frame metrics from a time period. |
| `ConfigureAgent` | `POST /profilingGroups/{profilingGroupName}/configureAgent` | - | `profilingGroupName` | - | `ConfigureAgentResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Used by profiler agents to report their current state and to receive remote configuration updates. For example, `ConfigureAgent` can be used to tell an agent whether to profile or not and for how long to return profiling data. |
| `CreateProfilingGroup` | `POST /profilingGroups` | `idempotent`, `idempotency-token` | `clientToken`, `profilingGroupName` | `clientToken` | `CreateProfilingGroupResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a profiling group. |
| `DeleteProfilingGroup` | `DELETE /profilingGroups/{profilingGroupName}` | `idempotent` | `profilingGroupName` | - | `DeleteProfilingGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a profiling group. |
| `DescribeProfilingGroup` | `GET /profilingGroups/{profilingGroupName}` | `readonly` | `profilingGroupName` | - | `DescribeProfilingGroupResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a `ProfilingGroupDescription` object that contains information about the requested profiling group. |
| `GetFindingsReportAccountSummary` | `GET /internal/findingsReports` | `readonly`, `paginated` | - | - | `GetFindingsReportAccountSummaryResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of `FindingsReportSummary` objects that contain analysis results for all profiling groups in your AWS account. |
| `GetNotificationConfiguration` | `GET /profilingGroups/{profilingGroupName}/notificationConfiguration` | `readonly` | `profilingGroupName` | - | `GetNotificationConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the current configuration for anomaly notifications for a profiling group. |
| `GetPolicy` | `GET /profilingGroups/{profilingGroupName}/policy` | `readonly` | `profilingGroupName` | - | `GetPolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the JSON-formatted resource-based policy on a profiling group. |
| `GetProfile` | `GET /profilingGroups/{profilingGroupName}/profile` | `readonly` | `profilingGroupName` | - | `GetProfileResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the aggregated profile of a profiling group for a specified time range. Amazon CodeGuru Profiler collects posted agent profiles for a profiling group into aggregated profiles. |
| `GetRecommendations` | `GET /internal/profilingGroups/{profilingGroupName}/recommendations` | `readonly` | `endTime`, `profilingGroupName`, `startTime` | - | `GetRecommendationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of `Recommendation` objects that contain recommendations for a profiling group for a given time period. A list of `Anomaly` objects that contains details about anomalies detected in the profiling group for the same time period is also returned. |
| `ListFindingsReports` | `GET /internal/profilingGroups/{profilingGroupName}/findingsReports` | `readonly`, `paginated` | `endTime`, `profilingGroupName`, `startTime` | - | `ListFindingsReportsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the available reports for a given profiling group and time range. |
| `ListProfileTimes` | `GET /profilingGroups/{profilingGroupName}/profileTimes` | `readonly`, `paginated` | `endTime`, `period`, `profilingGroupName`, `startTime` | - | `ListProfileTimesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the start times of the available aggregated profiles of a profiling group for an aggregation period within the specified time range. |
| `ListProfilingGroups` | `GET /profilingGroups` | `readonly`, `paginated` | - | - | `ListProfilingGroupsResponse` | `InternalServerException`, `ThrottlingException` | Returns a list of profiling groups. The profiling groups are returned as `ProfilingGroupDescription` objects. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of the tags that are assigned to a specified resource. |
| `PostAgentProfile` | `POST /profilingGroups/{profilingGroupName}/agentProfile` | `idempotency-token` | `agentProfile`, `contentType`, `profilingGroupName` | `profileToken` | `PostAgentProfileResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Submits profiling data to an aggregated profile of a profiling group. To get an aggregated profile that is created with this profiling data, use `GetProfile` . |
| `PutPermission` | `PUT /profilingGroups/{profilingGroupName}/policy/{actionGroup}` | `idempotent` | `actionGroup`, `principals`, `profilingGroupName` | - | `PutPermissionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds permissions to a profiling group's resource-based policy that are provided using an action group. If a profiling group doesn't have a resource-based policy, one is created for it using the permissions in the action group and the roles and users in the... |
| `RemoveNotificationChannel` | `DELETE /profilingGroups/{profilingGroupName}/notificationConfiguration/{channelId}` | `idempotent` | `channelId`, `profilingGroupName` | - | `RemoveNotificationChannelResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove one anomaly notifications channel for a profiling group. |
| `RemovePermission` | `DELETE /profilingGroups/{profilingGroupName}/policy/{actionGroup}` | - | `actionGroup`, `profilingGroupName`, `revisionId` | - | `RemovePermissionResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes permissions from a profiling group's resource-based policy that are provided using an action group. The one supported action group that can be removed is `agentPermission` which grants `ConfigureAgent` and `PostAgent` permissions. |
| `SubmitFeedback` | `POST /internal/profilingGroups/{profilingGroupName}/anomalies/{anomalyInstanceId}/feedback` | `readonly` | `anomalyInstanceId`, `profilingGroupName`, `type` | - | `SubmitFeedbackResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends feedback to CodeGuru Profiler about whether the anomaly detected by the analysis is useful or not. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Use to assign one or more tags to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Use to remove one or more tags from a resource. |
| `UpdateProfilingGroup` | `PUT /profilingGroups/{profilingGroupName}` | `idempotent` | `agentOrchestrationConfig`, `profilingGroupName` | - | `UpdateProfilingGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a profiling group. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | The server encountered an internal error and is unable to complete the request. |
| `ValidationException` | `structure` | `message` | The parameter is not valid. |
| `ResourceNotFoundException` | `structure` | `message` | The resource specified in the request does not exist. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ConflictException` | `structure` | `message` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `ServiceQuotaExceededException` | `structure` | `message` | You have exceeded your service quota. |
| `AddNotificationChannelsRequest` | `structure` | `channels`, `profilingGroupName` | The structure representing the AddNotificationChannelsRequest. |
| `AddNotificationChannelsResponse` | `structure` | `notificationConfiguration` | The structure representing the AddNotificationChannelsResponse. |
| `BatchGetFrameMetricDataRequest` | `structure` | `endTime`, `frameMetrics`, `period`, `profilingGroupName`, `startTime`, `targetResolution` | The structure representing the BatchGetFrameMetricDataRequest. |
| `BatchGetFrameMetricDataResponse` | `structure` | `endTime`, `endTimes`, `frameMetricData`, `resolution`, `startTime`, `unprocessedEndTimes` | The structure representing the BatchGetFrameMetricDataResponse. |
| `ConfigureAgentRequest` | `structure` | `fleetInstanceId`, `metadata`, `profilingGroupName` | The structure representing the configureAgentRequest. |
| `ConfigureAgentResponse` | `structure` | `configuration` | The structure representing the configureAgentResponse. |
| `CreateProfilingGroupRequest` | `structure` | `agentOrchestrationConfig`, `clientToken`, `computePlatform`, `profilingGroupName`, `tags` | The structure representing the createProfiliingGroupRequest. |
| `CreateProfilingGroupResponse` | `structure` | `profilingGroup` | The structure representing the createProfilingGroupResponse. |
| `DeleteProfilingGroupRequest` | `structure` | `profilingGroupName` | The structure representing the deleteProfilingGroupRequest. |
| `DeleteProfilingGroupResponse` | `structure` | - | The structure representing the deleteProfilingGroupResponse. |
| `DescribeProfilingGroupRequest` | `structure` | `profilingGroupName` | The structure representing the describeProfilingGroupRequest. |
| `DescribeProfilingGroupResponse` | `structure` | `profilingGroup` | The structure representing the describeProfilingGroupResponse. |
| `GetFindingsReportAccountSummaryRequest` | `structure` | `dailyReportsOnly`, `maxResults`, `nextToken` | The structure representing the GetFindingsReportAccountSummaryRequest. |
| `GetFindingsReportAccountSummaryResponse` | `structure` | `nextToken`, `reportSummaries` | The structure representing the GetFindingsReportAccountSummaryResponse. |
| `GetNotificationConfigurationRequest` | `structure` | `profilingGroupName` | The structure representing the GetNotificationConfigurationRequest. |
| `GetNotificationConfigurationResponse` | `structure` | `notificationConfiguration` | The structure representing the GetNotificationConfigurationResponse. |
| `GetPolicyRequest` | `structure` | `profilingGroupName` | The structure representing the `getPolicyRequest`. |
| `GetPolicyResponse` | `structure` | `policy`, `revisionId` | The structure representing the `getPolicyResponse`. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

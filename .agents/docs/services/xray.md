# AWS X-Ray

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services X-Ray provides APIs for managing debug traces and retrieving service maps and other data created by processing those traces.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS X-Ray workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Put`, `Update`, `Delete`, `List` operation families, including `GetEncryptionConfig`, `GetGroup`, `GetGroups`, `GetIndexingRules`, `PutEncryptionConfig`, `PutResourcePolicy`.

## Service Identity and Protocol

- AWS model slug: `xray`
- AWS SDK for Rust slug: `xray`
- Model version: `2016-04-12`
- Model file: `vendor/api-models-aws/models/xray/service/2016-04-12/xray-2016-04-12.json`
- SDK ID: `XRay`
- Endpoint prefix: `xray`
- ARN namespace: `xray`
- CloudFormation name: `XRay`
- CloudTrail event source: `xray.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (17), `Put` (4), `Update` (4), `Delete` (3), `List` (3), `Create` (2), `Batch` (1), `Cancel` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetTraces`, `CancelTraceRetrieval`, `CreateGroup`, `CreateSamplingRule`, `DeleteGroup`, `DeleteResourcePolicy`, `DeleteSamplingRule`, `PutEncryptionConfig`, `PutResourcePolicy`, `PutTelemetryRecords`, `PutTraceSegments`, `StartTraceRetrieval`, `TagResource`, `UntagResource`, `UpdateGroup`, `UpdateIndexingRule`, `UpdateSamplingRule`, `UpdateTraceSegmentDestination`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEncryptionConfig`, `GetGroup`, `GetGroups`, `GetIndexingRules`, `GetInsight`, `GetInsightEvents`, `GetInsightImpactGraph`, `GetInsightSummaries`, `GetRetrievedTracesGraph`, `GetSamplingRules`, `GetSamplingStatisticSummaries`, `GetSamplingTargets`, `GetServiceGraph`, `GetTimeSeriesServiceStatistics`, `GetTraceGraph`, `GetTraceSegmentDestination`, `GetTraceSummaries`, `ListResourcePolicies`, `ListRetrievedTraces`, `ListTagsForResource`.
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelTraceRetrieval`, `StartTraceRetrieval`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 38 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `SQS`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/xray/latest/devguide/xray-console-sampling.html
- https://docs.aws.amazon.com/xray/latest/devguide/xray-api.html
- https://docs.aws.amazon.com/xray/latest/devguide/xray-daemon.html

Research outcomes:
- X-Ray ingests segment documents through PutTraceSegments and groups them into traces for APIs such as BatchGetTraces, GetTraceSummaries, GetTraceGraph, and GetServiceGraph.
- Sampling rules are evaluated in ascending priority order, and the first matching rule determines the sampling decision.
- Sampling rule string fields can use `?` for one character and `*` for zero or more characters.
- Reservoir is a fixed number of matching requests per second before fixed-rate sampling. The reservoir applies collectively to all services using the rule.
- Fixed rate is applied after reservoir capacity is exhausted. Console percentages are 0 to 100, while SDK JSON values are 0 to 1.
- Services can temporarily borrow from reservoir before X-Ray assigns quota, and sampling statistics distinguish fixed-rate, reservoir, and borrowed samples.
- Sampling rule matching can include service name, service type, host, HTTP method, URL path, resource ARN, and optional attributes.
- The X-Ray daemon listens locally, receives segment documents, and relays them to the X-Ray API with IAM permissions.

Parity implications:
- Model traces, segments, trace summaries, service graphs, groups, sampling rules, sampling targets, daemon submission, and encryption settings.
- Sampling needs priority ordering, wildcard matching, per-second reservoir accounting, fixed-rate decisions, and borrowed/quota statistics.
- Trace read APIs should reconstruct trace and service graph views from submitted segment documents rather than storing only raw blobs.

## Operation Groups

### Get

- Operations: `GetEncryptionConfig`, `GetGroup`, `GetGroups`, `GetIndexingRules`, `GetInsight`, `GetInsightEvents`, `GetInsightImpactGraph`, `GetInsightSummaries`, `GetRetrievedTracesGraph`, `GetSamplingRules`, `GetSamplingStatisticSummaries`, `GetSamplingTargets`, `GetServiceGraph`, `GetTimeSeriesServiceStatistics`, `GetTraceGraph`, `GetTraceSegmentDestination`, `GetTraceSummaries`
- Traits: `paginated` (9)
- Common required input members in this group: `EndTime`, `InsightId`, `RetrievalToken`, `SamplingStatisticsDocuments`, `StartTime`, `TraceIds`

### Put

- Operations: `PutEncryptionConfig`, `PutResourcePolicy`, `PutTelemetryRecords`, `PutTraceSegments`
- Common required input members in this group: `PolicyDocument`, `PolicyName`, `TelemetryRecords`, `TraceSegmentDocuments`, `Type`

### Update

- Operations: `UpdateGroup`, `UpdateIndexingRule`, `UpdateSamplingRule`, `UpdateTraceSegmentDestination`
- Common required input members in this group: `Name`, `Rule`, `SamplingRuleUpdate`

### Delete

- Operations: `DeleteGroup`, `DeleteResourcePolicy`, `DeleteSamplingRule`
- Common required input members in this group: `PolicyName`

### List

- Operations: `ListResourcePolicies`, `ListRetrievedTraces`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: `ResourceARN`, `RetrievalToken`

### Create

- Operations: `CreateGroup`, `CreateSamplingRule`
- Common required input members in this group: `GroupName`, `SamplingRule`

### Batch

- Operations: `BatchGetTraces`
- Traits: `paginated` (1)
- Common required input members in this group: `TraceIds`

### Cancel

- Operations: `CancelTraceRetrieval`
- Common required input members in this group: `RetrievalToken`

### Start

- Operations: `StartTraceRetrieval`
- Common required input members in this group: `EndTime`, `StartTime`, `TraceIds`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetTraces` | `POST /Traces` | `paginated` | `TraceIds` | - | `BatchGetTracesResult` | `InvalidRequestException`, `ThrottledException` | You cannot find traces through this API if Transaction Search is enabled since trace is not indexed in X-Ray. Retrieves a list of traces specified by ID. |
| `CancelTraceRetrieval` | `POST /CancelTraceRetrieval` | - | `RetrievalToken` | - | `CancelTraceRetrievalResult` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Cancels an ongoing trace retrieval job initiated by `StartTraceRetrieval` using the provided `RetrievalToken`. A successful cancellation will return an HTTP 200 response. |
| `CreateGroup` | `POST /CreateGroup` | - | `GroupName` | - | `CreateGroupResult` | `InvalidRequestException`, `ThrottledException` | Creates a group resource with a name and a filter expression. |
| `CreateSamplingRule` | `POST /CreateSamplingRule` | - | `SamplingRule` | - | `CreateSamplingRuleResult` | `InvalidRequestException`, `RuleLimitExceededException`, `ThrottledException` | Creates a rule to control sampling behavior for instrumented applications. Services retrieve rules with GetSamplingRules, and evaluate each rule in ascending order of priority for each request. |
| `DeleteGroup` | `POST /DeleteGroup` | - | - | - | `DeleteGroupResult` | `InvalidRequestException`, `ThrottledException` | Deletes a group resource. |
| `DeleteResourcePolicy` | `POST /DeleteResourcePolicy` | - | `PolicyName` | - | `DeleteResourcePolicyResult` | `InvalidPolicyRevisionIdException`, `InvalidRequestException`, `ThrottledException` | Deletes a resource policy from the target Amazon Web Services account. |
| `DeleteSamplingRule` | `POST /DeleteSamplingRule` | - | - | - | `DeleteSamplingRuleResult` | `InvalidRequestException`, `ThrottledException` | Deletes a sampling rule. |
| `GetEncryptionConfig` | `POST /EncryptionConfig` | - | - | - | `GetEncryptionConfigResult` | `InvalidRequestException`, `ThrottledException` | Retrieves the current encryption configuration for X-Ray data. |
| `GetGroup` | `POST /GetGroup` | - | - | - | `GetGroupResult` | `InvalidRequestException`, `ThrottledException` | Retrieves group resource details. |
| `GetGroups` | `POST /Groups` | `paginated` | - | - | `GetGroupsResult` | `InvalidRequestException`, `ThrottledException` | Retrieves all active group details. |
| `GetIndexingRules` | `POST /GetIndexingRules` | - | - | - | `GetIndexingRulesResult` | `InvalidRequestException`, `ThrottledException` | Retrieves all indexing rules. Indexing rules are used to determine the server-side sampling rate for spans ingested through the CloudWatchLogs destination and indexed by X-Ray. |
| `GetInsight` | `POST /Insight` | - | `InsightId` | - | `GetInsightResult` | `InvalidRequestException`, `ThrottledException` | Retrieves the summary information of an insight. This includes impact to clients and root cause services, the top anomalous services, the category, the state of the insight, and the start and end time of the insight. |
| `GetInsightEvents` | `POST /InsightEvents` | `paginated` | `InsightId` | - | `GetInsightEventsResult` | `InvalidRequestException`, `ThrottledException` | X-Ray reevaluates insights periodically until they're resolved, and records each intermediate state as an event. You can review an insight's events in the Impact Timeline on the Inspect page in the X-Ray console. |
| `GetInsightImpactGraph` | `POST /InsightImpactGraph` | - | `EndTime`, `InsightId`, `StartTime` | - | `GetInsightImpactGraphResult` | `InvalidRequestException`, `ThrottledException` | Retrieves a service graph structure filtered by the specified insight. The service graph is limited to only structural information. |
| `GetInsightSummaries` | `POST /InsightSummaries` | `paginated` | `EndTime`, `StartTime` | - | `GetInsightSummariesResult` | `InvalidRequestException`, `ThrottledException` | Retrieves the summaries of all insights in the specified group matching the provided filter values. |
| `GetRetrievedTracesGraph` | `POST /GetRetrievedTracesGraph` | - | `RetrievalToken` | - | `GetRetrievedTracesGraphResult` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Retrieves a service graph for traces based on the specified `RetrievalToken` from the CloudWatch log group generated by Transaction Search. This API does not initiate a retrieval job. |
| `GetSamplingRules` | `POST /GetSamplingRules` | `paginated` | - | - | `GetSamplingRulesResult` | `InvalidRequestException`, `ThrottledException` | Retrieves all sampling rules. |
| `GetSamplingStatisticSummaries` | `POST /SamplingStatisticSummaries` | `paginated` | - | - | `GetSamplingStatisticSummariesResult` | `InvalidRequestException`, `ThrottledException` | Retrieves information about recent sampling results for all sampling rules. |
| `GetSamplingTargets` | `POST /SamplingTargets` | - | `SamplingStatisticsDocuments` | - | `GetSamplingTargetsResult` | `InvalidRequestException`, `ThrottledException` | Requests a sampling quota for rules that the service is using to sample requests. |
| `GetServiceGraph` | `POST /ServiceGraph` | `paginated` | `EndTime`, `StartTime` | - | `GetServiceGraphResult` | `InvalidRequestException`, `ThrottledException` | Retrieves a document that describes services that process incoming requests, and downstream services that they call as a result. Root services process incoming requests and make calls to downstream services. |
| `GetTimeSeriesServiceStatistics` | `POST /TimeSeriesServiceStatistics` | `paginated` | `EndTime`, `StartTime` | - | `GetTimeSeriesServiceStatisticsResult` | `InvalidRequestException`, `ThrottledException` | Get an aggregation of service statistics defined by a specific time range. |
| `GetTraceGraph` | `POST /TraceGraph` | `paginated` | `TraceIds` | - | `GetTraceGraphResult` | `InvalidRequestException`, `ThrottledException` | Retrieves a service graph for one or more specific trace IDs. |
| `GetTraceSegmentDestination` | `POST /GetTraceSegmentDestination` | - | - | - | `GetTraceSegmentDestinationResult` | `InvalidRequestException`, `ThrottledException` | Retrieves the current destination of data sent to `PutTraceSegments` and OpenTelemetry protocol (OTLP) endpoint. The Transaction Search feature requires a CloudWatchLogs destination. |
| `GetTraceSummaries` | `POST /TraceSummaries` | `paginated` | `EndTime`, `StartTime` | - | `GetTraceSummariesResult` | `InvalidRequestException`, `ThrottledException` | Retrieves IDs and annotations for traces available for a specified time frame using an optional filter. To get the full traces, pass the trace IDs to `BatchGetTraces`. |
| `ListResourcePolicies` | `POST /ListResourcePolicies` | `paginated` | - | - | `ListResourcePoliciesResult` | `InvalidRequestException`, `ThrottledException` | Returns the list of resource policies in the target Amazon Web Services account. |
| `ListRetrievedTraces` | `POST /ListRetrievedTraces` | - | `RetrievalToken` | - | `ListRetrievedTracesResult` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Retrieves a list of traces for a given `RetrievalToken` from the CloudWatch log group generated by Transaction Search. For information on what each trace returns, see BatchGetTraces. |
| `ListTagsForResource` | `POST /ListTagsForResource` | `paginated` | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Returns a list of tags that are applied to the specified Amazon Web Services X-Ray group or sampling rule. |
| `PutEncryptionConfig` | `POST /PutEncryptionConfig` | - | `Type` | - | `PutEncryptionConfigResult` | `InvalidRequestException`, `ThrottledException` | Updates the encryption configuration for X-Ray data. |
| `PutResourcePolicy` | `POST /PutResourcePolicy` | - | `PolicyDocument`, `PolicyName` | - | `PutResourcePolicyResult` | `InvalidPolicyRevisionIdException`, `LockoutPreventionException`, `MalformedPolicyDocumentException`, `PolicyCountLimitExceededException`, `PolicySizeLimitExceededException`, `ThrottledException` | Sets the resource policy to grant one or more Amazon Web Services services and accounts permissions to access X-Ray. Each resource policy will be associated with a specific Amazon Web Services account. |
| `PutTelemetryRecords` | `POST /TelemetryRecords` | - | `TelemetryRecords` | - | `PutTelemetryRecordsResult` | `InvalidRequestException`, `ThrottledException` | Used by the Amazon Web Services X-Ray daemon to upload telemetry. |
| `PutTraceSegments` | `POST /TraceSegments` | - | `TraceSegmentDocuments` | - | `PutTraceSegmentsResult` | `InvalidRequestException`, `ThrottledException` | Uploads segment documents to Amazon Web Services X-Ray. A segment document can be a completed segment, an in-progress segment, or an array of subsegments. |
| `StartTraceRetrieval` | `POST /StartTraceRetrieval` | - | `EndTime`, `StartTime`, `TraceIds` | - | `StartTraceRetrievalResult` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Initiates a trace retrieval process using the specified time range and for the given trace IDs in the Transaction Search generated CloudWatch log group. For more information, see Transaction Search. |
| `TagResource` | `POST /TagResource` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException`, `TooManyTagsException` | Applies tags to an existing Amazon Web Services X-Ray group or sampling rule. |
| `UntagResource` | `POST /UntagResource` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Removes tags from an Amazon Web Services X-Ray group or sampling rule. You cannot edit or delete system tags (those with an `aws:` prefix). |
| `UpdateGroup` | `POST /UpdateGroup` | - | - | - | `UpdateGroupResult` | `InvalidRequestException`, `ThrottledException` | Updates a group resource. |
| `UpdateIndexingRule` | `POST /UpdateIndexingRule` | - | `Name`, `Rule` | - | `UpdateIndexingRuleResult` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottledException` | Modifies an indexing rule’s configuration. Indexing rules are used for determining the sampling rate for spans indexed from CloudWatch Logs. |
| `UpdateSamplingRule` | `POST /UpdateSamplingRule` | - | `SamplingRuleUpdate` | - | `UpdateSamplingRuleResult` | `InvalidRequestException`, `ThrottledException` | Modifies a sampling rule's configuration. |
| `UpdateTraceSegmentDestination` | `POST /UpdateTraceSegmentDestination` | - | - | - | `UpdateTraceSegmentDestinationResult` | `InvalidRequestException`, `ThrottledException` | Modifies the destination of data sent to `PutTraceSegments`. The Transaction Search feature requires the CloudWatchLogs destination. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ThrottledException` | `structure` | `Message` | The request exceeds the maximum number of requests per second. |
| `InvalidRequestException` | `structure` | `Message` | The request is missing required parameters or has invalid parameters. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceName` | The resource was not found. |
| `InvalidPolicyRevisionIdException` | `structure` | `Message` | A policy revision id was provided which does not match the latest policy revision. |
| `BatchGetTracesRequest` | `structure` | `NextToken`, `TraceIds` | - |
| `BatchGetTracesResult` | `structure` | `NextToken`, `Traces`, `UnprocessedTraceIds` | - |
| `CancelTraceRetrievalRequest` | `structure` | `RetrievalToken` | - |
| `CancelTraceRetrievalResult` | `structure` | - | - |
| `CreateGroupRequest` | `structure` | `FilterExpression`, `GroupName`, `InsightsConfiguration`, `Tags` | - |
| `CreateGroupResult` | `structure` | `Group` | - |
| `CreateSamplingRuleRequest` | `structure` | `SamplingRule`, `Tags` | - |
| `CreateSamplingRuleResult` | `structure` | `SamplingRuleRecord` | - |
| `RuleLimitExceededException` | `structure` | `Message` | You have reached the maximum number of sampling rules. |
| `DeleteGroupRequest` | `structure` | `GroupARN`, `GroupName` | - |
| `DeleteGroupResult` | `structure` | - | - |
| `DeleteResourcePolicyRequest` | `structure` | `PolicyName`, `PolicyRevisionId` | - |
| `DeleteResourcePolicyResult` | `structure` | - | - |
| `DeleteSamplingRuleRequest` | `structure` | `RuleARN`, `RuleName` | - |
| `DeleteSamplingRuleResult` | `structure` | `SamplingRuleRecord` | - |
| `GetEncryptionConfigRequest` | `structure` | - | - |
| `GetEncryptionConfigResult` | `structure` | `EncryptionConfig` | - |
| `GetGroupRequest` | `structure` | `GroupARN`, `GroupName` | - |
| `GetGroupResult` | `structure` | `Group` | - |
| `GetGroupsRequest` | `structure` | `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

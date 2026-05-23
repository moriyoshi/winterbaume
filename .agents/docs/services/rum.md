# CloudWatch RUM

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With Amazon CloudWatch RUM, you can perform real-user monitoring to collect client-side data about your web application performance from actual user sessions in real time. The data collected includes page load times, client-side errors, and user behavior. When you view this data, you can see it all aggregated together and also see breakdowns by the browsers and devices that your customers use. You can use the collected data to quickly identify and debug client-side performance issues. CloudWatch RUM helps you visualize anomalies in your application performance and find relevant debugging data such as error messages, stack traces, and user sessions.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented CloudWatch RUM workflows in the local mock. Key resources include `AppMonitorResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Batch`, `Delete`, `Get`, `List`, `Put` operation families, including `BatchCreateRumMetricDefinitions`, `BatchDeleteRumMetricDefinitions`, `BatchGetRumMetricDefinitions`, `DeleteAppMonitor`, `DeleteResourcePolicy`, `DeleteRumMetricsDestination`.

## Service Identity and Protocol

- AWS model slug: `rum`
- AWS SDK for Rust slug: `rum`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/rum/service/2018-05-10/rum-2018-05-10.json`
- SDK ID: `RUM`
- Endpoint prefix: `-`
- ARN namespace: `rum`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Batch` (3), `Delete` (3), `Get` (3), `List` (3), `Put` (3), `Update` (2), `Create` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateRumMetricDefinitions`, `BatchDeleteRumMetricDefinitions`, `BatchGetRumMetricDefinitions`, `CreateAppMonitor`, `DeleteAppMonitor`, `DeleteResourcePolicy`, `DeleteRumMetricsDestination`, `PutResourcePolicy`, `PutRumEvents`, `PutRumMetricsDestination`, `TagResource`, `UntagResource`, `UpdateAppMonitor`, `UpdateRumMetricDefinition`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetRumMetricDefinitions`, `GetAppMonitor`, `GetAppMonitorData`, `GetResourcePolicy`, `ListAppMonitors`, `ListRumMetricsDestinations`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 20 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`, `CloudWatch Logs`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AppMonitorResource` | `Name` | read: `GetAppMonitor`; update: `UpdateAppMonitor`; delete: `DeleteAppMonitor`; list: `ListAppMonitors` | `BatchCreateRumMetricDefinitions`, `BatchDeleteRumMetricDefinitions`, `BatchGetRumMetricDefinitions`, `CreateAppMonitor`, `DeleteResourcePolicy`, `DeleteRumMetricsDestination`, `GetAppMonitorData`, `GetResourcePolicy`, `ListRumMetricsDestinations`, `PutResourcePolicy`, ... (+2) | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutRumEvents`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays the tags associated with a CloudWatch RUM resource. |
| `PutRumEvents` | `POST /appmonitors/{Id}/` | - | `Id`, `BatchId`, `AppMonitorDetails`, `UserDetails`, `RumEvents` | - | `PutRumEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends telemetry events about your application performance and user behavior to CloudWatch RUM. The code snippet that RUM generates for you to add to your application includes PutRumEvents operations to send this data ... |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified CloudWatch RUM resource. Currently, the only resources that can be tagged app monitors. Tags can help you organize and categorize your resources. You can al ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes one or more tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permissions to perform this action. |
| `ConflictException` | `structure` | message, resourceName, resourceType | This operation attempted to create a resource that already exists. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | Internal service exception. |
| `InvalidPolicyRevisionIdException` | `structure` | message | The policy revision ID that you provided doeesn't match the latest policy revision ID. |
| `MalformedPolicyDocumentException` | `structure` | message | The policy document that you specified is not formatted correctly. |
| `PolicyNotFoundException` | `structure` | message | The resource-based policy doesn't exist on this app monitor. |
| `PolicySizeLimitExceededException` | `structure` | message | The policy document is too large. The limit is 4 KB. |
| `ResourceNotFoundException` | `structure` | message, resourceName, resourceType | Resource not found. |
| `ServiceQuotaExceededException` | `structure` | message | This request exceeds a service quota. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was throttled because of quota limits. |
| `ValidationException` | `structure` | message | One of the arguments for the request is not valid. |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | ResourceArn, Tags | - |
| `PutRumEventsRequest` | `structure` | Id, BatchId, AppMonitorDetails, UserDetails, RumEvents, Alias | - |
| `PutRumEventsResponse` | `structure` | **empty (no members)** | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

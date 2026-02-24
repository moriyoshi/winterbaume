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

### Batch

- Operations: `BatchCreateRumMetricDefinitions`, `BatchDeleteRumMetricDefinitions`, `BatchGetRumMetricDefinitions`
- Traits: `idempotent` (2), `paginated` (1), `readonly` (1)
- Common required input members in this group: `AppMonitorName`, `Destination`, `MetricDefinitionIds`, `MetricDefinitions`

### Delete

- Operations: `DeleteAppMonitor`, `DeleteResourcePolicy`, `DeleteRumMetricsDestination`
- Traits: `idempotent` (3)
- Common required input members in this group: `AppMonitorName`, `Destination`, `Name`

### Get

- Operations: `GetAppMonitor`, `GetAppMonitorData`, `GetResourcePolicy`
- Traits: `paginated` (1), `readonly` (3)
- Common required input members in this group: `Name`, `TimeRange`

### List

- Operations: `ListAppMonitors`, `ListRumMetricsDestinations`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `AppMonitorName`, `ResourceArn`

### Put

- Operations: `PutResourcePolicy`, `PutRumEvents`, `PutRumMetricsDestination`
- Traits: `endpoint-bound` (1), `idempotent` (2)
- Common required input members in this group: `AppMonitorDetails`, `AppMonitorName`, `BatchId`, `Destination`, `Id`, `Name`, `PolicyDocument`, `RumEvents`, `UserDetails`

### Update

- Operations: `UpdateAppMonitor`, `UpdateRumMetricDefinition`
- Traits: `idempotent` (1)
- Common required input members in this group: `AppMonitorName`, `Destination`, `MetricDefinition`, `MetricDefinitionId`, `Name`

### Create

- Operations: `CreateAppMonitor`
- Traits: `idempotent` (1)
- Common required input members in this group: `Name`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateRumMetricDefinitions` | `POST /rummetrics/{AppMonitorName}/metrics` | `idempotent` | `AppMonitorName`, `Destination`, `MetricDefinitions` | - | `BatchCreateRumMetricDefinitionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Specifies the extended metrics and custom metrics that you want a CloudWatch RUM app monitor to send to a destination. Valid destinations include CloudWatch and Evidently. |
| `BatchDeleteRumMetricDefinitions` | `DELETE /rummetrics/{AppMonitorName}/metrics` | `idempotent` | `AppMonitorName`, `Destination`, `MetricDefinitionIds` | - | `BatchDeleteRumMetricDefinitionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified metrics from being sent to an extended metrics destination. If some metric definition IDs specified in a `BatchDeleteRumMetricDefinitions` operations are not valid, those metric definitions fail and return errors, but all valid metric... |
| `BatchGetRumMetricDefinitions` | `GET /rummetrics/{AppMonitorName}/metrics` | `readonly`, `paginated` | `AppMonitorName`, `Destination` | - | `BatchGetRumMetricDefinitionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the list of metrics and dimensions that a RUM app monitor is sending to a single destination. |
| `CreateAppMonitor` | `POST /appmonitor` | `idempotent` | `Name` | - | `CreateAppMonitorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a Amazon CloudWatch RUM app monitor, which collects telemetry data from your application and sends that data to RUM. The data includes performance and reliability information such as page load time, client-side errors, and user behavior. |
| `DeleteAppMonitor` | `DELETE /appmonitor/{Name}` | `idempotent` | `Name` | - | `DeleteAppMonitorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing app monitor. This immediately stops the collection of data. |
| `DeleteResourcePolicy` | `DELETE /appmonitor/{Name}/policy` | `idempotent` | `Name` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidPolicyRevisionIdException`, `PolicyNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association of a resource-based policy from an app monitor. |
| `DeleteRumMetricsDestination` | `DELETE /rummetrics/{AppMonitorName}/metricsdestination` | `idempotent` | `AppMonitorName`, `Destination` | - | `DeleteRumMetricsDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a destination for CloudWatch RUM extended metrics, so that the specified app monitor stops sending extended metrics to that destination. |
| `GetAppMonitor` | `GET /appmonitor/{Name}` | `readonly` | `Name` | - | `GetAppMonitorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the complete configuration information for one app monitor. |
| `GetAppMonitorData` | `POST /appmonitor/{Name}/data` | `readonly`, `paginated` | `Name`, `TimeRange` | - | `GetAppMonitorDataResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the raw performance events that RUM has collected from your web application, so that you can do your own processing or analysis of this data. |
| `GetResourcePolicy` | `GET /appmonitor/{Name}/policy` | `readonly` | `Name` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `PolicyNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to retrieve information about a resource-based policy that is attached to an app monitor. |
| `ListAppMonitors` | `POST /appmonitors` | `readonly`, `paginated` | - | - | `ListAppMonitorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of the Amazon CloudWatch RUM app monitors in the account. |
| `ListRumMetricsDestinations` | `GET /rummetrics/{AppMonitorName}/metricsdestination` | `readonly`, `paginated` | `AppMonitorName` | - | `ListRumMetricsDestinationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of destinations that you have created to receive RUM extended metrics, for the specified app monitor. For more information about extended metrics, see AddRumMetrics. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Displays the tags associated with a CloudWatch RUM resource. |
| `PutResourcePolicy` | `PUT /appmonitor/{Name}/policy` | `idempotent` | `Name`, `PolicyDocument` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidPolicyRevisionIdException`, `MalformedPolicyDocumentException`, `PolicySizeLimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, ... (+1) | Use this operation to assign a resource-based policy to a CloudWatch RUM app monitor to control access to it. Each app monitor can have one resource-based policy. |
| `PutRumEvents` | `POST /appmonitors/{Id}/` | `endpoint-bound` | `AppMonitorDetails`, `BatchId`, `Id`, `RumEvents`, `UserDetails` | - | `PutRumEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends telemetry events about your application performance and user behavior to CloudWatch RUM. The code snippet that RUM generates for you to add to your application includes `PutRumEvents` operations to send this data to RUM. |
| `PutRumMetricsDestination` | `POST /rummetrics/{AppMonitorName}/metricsdestination` | `idempotent` | `AppMonitorName`, `Destination` | - | `PutRumMetricsDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates a destination to receive extended metrics from CloudWatch RUM. You can send extended metrics to CloudWatch or to a CloudWatch Evidently experiment. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified CloudWatch RUM resource. Currently, the only resources that can be tagged app monitors. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes one or more tags from the specified resource. |
| `UpdateAppMonitor` | `PATCH /appmonitor/{Name}` | - | `Name` | - | `UpdateAppMonitorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing app monitor. When you use this operation, only the parts of the app monitor configuration that you specify in this operation are changed. |
| `UpdateRumMetricDefinition` | `PATCH /rummetrics/{AppMonitorName}/metrics` | `idempotent` | `AppMonitorName`, `Destination`, `MetricDefinition`, `MetricDefinitionId` | - | `UpdateRumMetricDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Modifies one existing metric definition for CloudWatch RUM extended metrics. For more information about extended metrics, see BatchCreateRumMetricsDefinitions. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | Internal service exception. |
| `ValidationException` | `structure` | `message` | One of the arguments for the request is not valid. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceName`, `resourceType` | Resource not found. |
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permissions to perform this action. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was throttled because of quota limits. |
| `ConflictException` | `structure` | `message`, `resourceName`, `resourceType` | This operation attempted to create a resource that already exists. |
| `ServiceQuotaExceededException` | `structure` | `message` | This request exceeds a service quota. |
| `InvalidPolicyRevisionIdException` | `structure` | `message` | The policy revision ID that you provided doeesn't match the latest policy revision ID. |
| `PolicyNotFoundException` | `structure` | `message` | The resource-based policy doesn't exist on this app monitor. |
| `BatchCreateRumMetricDefinitionsRequest` | `structure` | `AppMonitorName`, `Destination`, `DestinationArn`, `MetricDefinitions` | - |
| `BatchCreateRumMetricDefinitionsResponse` | `structure` | `Errors`, `MetricDefinitions` | - |
| `BatchDeleteRumMetricDefinitionsRequest` | `structure` | `AppMonitorName`, `Destination`, `DestinationArn`, `MetricDefinitionIds` | - |
| `BatchDeleteRumMetricDefinitionsResponse` | `structure` | `Errors`, `MetricDefinitionIds` | - |
| `BatchGetRumMetricDefinitionsRequest` | `structure` | `AppMonitorName`, `Destination`, `DestinationArn`, `MaxResults`, `NextToken` | - |
| `BatchGetRumMetricDefinitionsResponse` | `structure` | `MetricDefinitions`, `NextToken` | - |
| `CreateAppMonitorRequest` | `structure` | `AppMonitorConfiguration`, `CustomEvents`, `CwLogEnabled`, `DeobfuscationConfiguration`, `Domain`, `DomainList`, `Name`, `Platform`, `Tags` | - |
| `CreateAppMonitorResponse` | `structure` | `Id` | - |
| `DeleteAppMonitorRequest` | `structure` | `Name` | - |
| `DeleteAppMonitorResponse` | `structure` | - | - |
| `DeleteResourcePolicyRequest` | `structure` | `Name`, `PolicyRevisionId` | - |
| `DeleteResourcePolicyResponse` | `structure` | `PolicyRevisionId` | - |
| `DeleteRumMetricsDestinationRequest` | `structure` | `AppMonitorName`, `Destination`, `DestinationArn` | - |
| `DeleteRumMetricsDestinationResponse` | `structure` | - | - |
| `GetAppMonitorRequest` | `structure` | `Name` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon CloudWatch Internet Monitor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudWatch Internet Monitor provides visibility into how internet issues impact the performance and availability between your applications hosted on Amazon Web Services and your end users. It can reduce the time it takes for you to diagnose internet issues from days to minutes. Internet Monitor uses the connectivity data that Amazon Web Services captures from its global networking footprint to calculate a baseline of performance and availability for internet traffic. This is the same data that Amazon Web Services uses to monitor internet uptime and availability. With those measurements as a baseline, Internet Monitor raises awareness for you when there are significant problems for your end users in the different geographic locations where your application runs.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon CloudWatch Internet Monitor workflows in the local mock. Key resources include `HealthEventResource`, `InternetEventResource`, `MonitorResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Start` operation families, including `GetHealthEvent`, `GetInternetEvent`, `GetMonitor`, `GetQueryResults`, `ListHealthEvents`, `ListInternetEvents`.

## Service Identity and Protocol

- AWS model slug: `internetmonitor`
- AWS SDK for Rust slug: `internetmonitor`
- Model version: `2021-06-03`
- Model file: `vendor/api-models-aws/models/internetmonitor/service/2021-06-03/internetmonitor-2021-06-03.json`
- SDK ID: `InternetMonitor`
- Endpoint prefix: `-`
- ARN namespace: `internetmonitor`
- CloudFormation name: `InternetMonitor`
- CloudTrail event source: `internetmonitor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (4), `Create` (1), `Delete` (1), `Start` (1), `Stop` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateMonitor`, `DeleteMonitor`, `StartQuery`, `StopQuery`, `TagResource`, `UntagResource`, `UpdateMonitor`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetHealthEvent`, `GetInternetEvent`, `GetMonitor`, `GetQueryResults`, `GetQueryStatus`, `ListHealthEvents`, `ListInternetEvents`, `ListMonitors`, `ListTagsForResource`, `StartQuery`, `StopQuery`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartQuery`, `StopQuery`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `HealthEventResource` | `EventId`, `MonitorName` | read: `GetHealthEvent`; list: `ListHealthEvents` | - | - |
| `InternetEventResource` | `EventId` | read: `GetInternetEvent`; list: `ListInternetEvents` | - | Represents an internet event for a specific city+AS location |
| `MonitorResource` | `MonitorName` | put: `CreateMonitor`; read: `GetMonitor`; update: `UpdateMonitor`; delete: `DeleteMonitor`; list: `ListMonitors` | `GetQueryResults`, `GetQueryStatus`, `StartQuery`, `StopQuery` | - |
## Operation Groups

### Get

- Operations: `GetHealthEvent`, `GetInternetEvent`, `GetMonitor`, `GetQueryResults`, `GetQueryStatus`
- Traits: `paginated` (1), `readonly` (5)
- Common required input members in this group: `EventId`, `MonitorName`, `QueryId`

### List

- Operations: `ListHealthEvents`, `ListInternetEvents`, `ListMonitors`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `MonitorName`, `ResourceArn`

### Create

- Operations: `CreateMonitor`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `MonitorName`

### Delete

- Operations: `DeleteMonitor`
- Traits: `idempotent` (1)
- Common required input members in this group: `MonitorName`

### Start

- Operations: `StartQuery`
- Traits: `readonly` (1)
- Common required input members in this group: `EndTime`, `MonitorName`, `QueryType`, `StartTime`

### Stop

- Operations: `StopQuery`
- Traits: `readonly` (1)
- Common required input members in this group: `MonitorName`, `QueryId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateMonitor`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `MonitorName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateMonitor` | `POST /v20210603/Monitors` | `idempotent`, `idempotency-token` | `MonitorName` | `ClientToken` | `CreateMonitorOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a monitor in Amazon CloudWatch Internet Monitor. A monitor is built based on information from the application resources that you add: VPCs, Network Load Balancers (NLBs), Amazon CloudFront distributions, and Amazon WorkSpaces directories. |
| `DeleteMonitor` | `DELETE /v20210603/Monitors/{MonitorName}` | `idempotent` | `MonitorName` | - | `DeleteMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a monitor in Amazon CloudWatch Internet Monitor. |
| `GetHealthEvent` | `GET /v20210603/Monitors/{MonitorName}/HealthEvents/{EventId}` | `readonly` | `EventId`, `MonitorName` | - | `GetHealthEventOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets information that Amazon CloudWatch Internet Monitor has created and stored about a health event for a specified monitor. This information includes the impacted locations, and all the information related to the event, by location. |
| `GetInternetEvent` | `GET /v20210603/InternetEvents/{EventId}` | `readonly` | `EventId` | - | `GetInternetEventOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets information that Amazon CloudWatch Internet Monitor has generated about an internet event. Internet Monitor displays information about recent global health events, called internet events, on a global outages map that is available to all Amazon Web... |
| `GetMonitor` | `GET /v20210603/Monitors/{MonitorName}` | `readonly` | `MonitorName` | - | `GetMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets information about a monitor in Amazon CloudWatch Internet Monitor based on a monitor name. The information returned includes the Amazon Resource Name (ARN), create time, modified time, resources included in the monitor, and status information. |
| `GetQueryResults` | `GET /v20210603/Monitors/{MonitorName}/Queries/{QueryId}/Results` | `readonly`, `paginated` | `MonitorName`, `QueryId` | - | `GetQueryResultsOutput` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Return the data for a query with the Amazon CloudWatch Internet Monitor query interface. Specify the query that you want to return results for by providing a `QueryId` and a monitor name. |
| `GetQueryStatus` | `GET /v20210603/Monitors/{MonitorName}/Queries/{QueryId}/Status` | `readonly` | `MonitorName`, `QueryId` | - | `GetQueryStatusOutput` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Returns the current status of a query for the Amazon CloudWatch Internet Monitor query interface, for a specified query ID and monitor. When you run a query, check the status to make sure that the query has `SUCCEEDED` before you review the results. |
| `ListHealthEvents` | `GET /v20210603/Monitors/{MonitorName}/HealthEvents` | `readonly`, `paginated` | `MonitorName` | - | `ListHealthEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all health events for a monitor in Amazon CloudWatch Internet Monitor. Returns information for health events including the event start and end times, and the status. |
| `ListInternetEvents` | `GET /v20210603/InternetEvents` | `readonly`, `paginated` | - | - | `ListInternetEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists internet events that cause performance or availability issues for client locations. Amazon CloudWatch Internet Monitor displays information about recent global health events, called internet events, on a global outages map that is available to all... |
| `ListMonitors` | `GET /v20210603/Monitors` | `readonly`, `paginated` | - | - | `ListMonitorsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all of your monitors for Amazon CloudWatch Internet Monitor and their statuses, along with the Amazon Resource Name (ARN) and name of each monitor. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists the tags for a resource. Tags are supported only for monitors in Amazon CloudWatch Internet Monitor. |
| `StartQuery` | `POST /v20210603/Monitors/{MonitorName}/Queries` | `readonly` | `EndTime`, `MonitorName`, `QueryType`, `StartTime` | - | `StartQueryOutput` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Start a query to return data for a specific query type for the Amazon CloudWatch Internet Monitor query interface. Specify a time period for the data that you want returned by using `StartTime` and `EndTime`. |
| `StopQuery` | `DELETE /v20210603/Monitors/{MonitorName}/Queries/{QueryId}` | `readonly` | `MonitorName`, `QueryId` | - | `StopQueryOutput` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Stop a query that is progress for a specific monitor. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Adds a tag to a resource. Tags are supported only for monitors in Amazon CloudWatch Internet Monitor. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Removes a tag from a resource. |
| `UpdateMonitor` | `PATCH /v20210603/Monitors/{MonitorName}` | `idempotent`, `idempotency-token` | `MonitorName` | `ClientToken` | `UpdateMonitorOutput` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a monitor. You can update a monitor to change the percentage of traffic to monitor or the maximum number of city-networks (locations and ASNs), to add or remove resources, or to change the status of the monitor. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permission to perform this action. |
| `InternalServerException` | `structure` | `message` | An internal error occurred. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | Invalid request. |
| `LimitExceededException` | `structure` | `message` | The request exceeded a service quota. |
| `BadRequestException` | `structure` | `message` | A bad request was received. |
| `InternalServerErrorException` | `structure` | `message` | There was an internal server error. |
| `NotFoundException` | `structure` | `message` | The request specifies something that doesn't exist. |
| `TooManyRequestsException` | `structure` | `message` | There were too many requests. |
| `CreateMonitorInput` | `structure` | `ClientToken`, `HealthEventsConfig`, `InternetMeasurementsLogDelivery`, `MaxCityNetworksToMonitor`, `MonitorName`, `Resources`, `Tags`, `TrafficPercentageToMonitor` | - |
| `CreateMonitorOutput` | `structure` | `Arn`, `Status` | - |
| `ConflictException` | `structure` | `message` | The requested resource is in use. |
| `DeleteMonitorInput` | `structure` | `MonitorName` | - |
| `DeleteMonitorOutput` | `structure` | - | - |
| `GetHealthEventInput` | `structure` | `EventId`, `LinkedAccountId`, `MonitorName` | - |
| `GetHealthEventOutput` | `structure` | `CreatedAt`, `EndedAt`, `EventArn`, `EventId`, `HealthScoreThreshold`, `ImpactType`, `ImpactedLocations`, `LastUpdatedAt`, `PercentOfTotalTrafficImpacted`, `StartedAt`, `Status` | - |
| `GetInternetEventInput` | `structure` | `EventId` | - |
| `GetInternetEventOutput` | `structure` | `ClientLocation`, `EndedAt`, `EventArn`, `EventId`, `EventStatus`, `EventType`, `StartedAt` | - |
| `GetMonitorInput` | `structure` | `LinkedAccountId`, `MonitorName` | - |
| `GetMonitorOutput` | `structure` | `CreatedAt`, `HealthEventsConfig`, `InternetMeasurementsLogDelivery`, `MaxCityNetworksToMonitor`, `ModifiedAt`, `MonitorArn`, `MonitorName`, `ProcessingStatus`, `ProcessingStatusInfo`, `Resources`, `Status`, `Tags`, ... (+1) | - |
| `GetQueryResultsInput` | `structure` | `MaxResults`, `MonitorName`, `NextToken`, `QueryId` | - |
| `GetQueryResultsOutput` | `structure` | `Data`, `Fields`, `NextToken` | - |
| `GetQueryStatusInput` | `structure` | `MonitorName`, `QueryId` | - |
| `GetQueryStatusOutput` | `structure` | `Status` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

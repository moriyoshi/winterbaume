# Amazon CloudWatch Application Signals

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use CloudWatch Application Signals for comprehensive observability of your cloud-based applications. It enables real-time service health dashboards and helps you track long-term performance trends against your business goals. The application-centric view provides you with unified visibility across your applications, services, and dependencies, so you can proactively monitor and efficiently triage any issues that may arise, ensuring optimal customer experience. Application Signals provides the following benefits: Automatically collect metrics and traces from your applications, and display key metrics such as call volume, availability, latency, faults, and errors. Create and monitor service level objectives (SLOs).

## Possible Usage Scenarios
- Backported from `crates/winterbaume-applicationsignals/tests/scenario_test.rs`: create and manage a service-level objective budget pipeline, including goal configuration, lookup, update, and deletion.
- Backported from `scenario_test.rs`: manage grouping configuration so related services can be organised for observability views.
- Scenario insight from EC2: add full state-machine walks for Amazon CloudWatch Application Signals resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support service, operation, and SLO observability workflows using CloudWatch telemetry, burn-rate or attainment tracking, and grouping behaviour for application topology.

## Service Identity and Protocol

- AWS model slug: `application-signals`
- AWS SDK for Rust slug: `applicationsignals`
- Model version: `2024-04-15`
- Model file: `vendor/api-models-aws/models/application-signals/service/2024-04-15/application-signals-2024-04-15.json`
- SDK ID: `Application Signals`
- Endpoint prefix: `application-signals`
- ARN namespace: `application-signals`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Batch` (2), `Delete` (2), `Get` (2), `Create` (1), `Put` (1), `Start` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetServiceLevelObjectiveBudgetReport`, `BatchUpdateExclusionWindows`, `CreateServiceLevelObjective`, `DeleteGroupingConfiguration`, `DeleteServiceLevelObjective`, `PutGroupingConfiguration`, `StartDiscovery`, `TagResource`, `UntagResource`, `UpdateServiceLevelObjective`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetService`, `GetServiceLevelObjective`, `ListAuditFindings`, `ListEntityEvents`, `ListGroupingAttributeDefinitions`, `ListServiceDependencies`, `ListServiceDependents`, `ListServiceLevelObjectiveExclusionWindows`, `ListServiceLevelObjectives`, `ListServiceOperations`, `ListServiceStates`, `ListServices`, `ListTagsForResource`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchGetServiceLevelObjectiveBudgetReport`, `StartDiscovery`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ServiceLevelObjectiveResource` | `Id` | create: `CreateServiceLevelObjective`; read: `GetServiceLevelObjective`; update: `UpdateServiceLevelObjective`; delete: `DeleteServiceLevelObjective`; list: `ListServiceLevelObjectives` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cli/latest/reference/application-signals/
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries.html

Research outcomes:
- CloudWatch Application Signals surfaces application services, service maps, client pages, Synthetics canaries, and service level objectives.
- Application Signals integrates with CloudWatch RUM, CloudWatch Synthetics, and Service Catalog AppRegistry for application names and dashboards.
- Service-level objectives define reliability goals over telemetry such as latency and availability.
- Synthetics canaries can monitor endpoints and APIs and contribute to service visibility.
- The service depends on telemetry emitted from instrumented applications and related CloudWatch resources.

Parity implications:
- Model services, service dependencies, service level objectives, burn-rate or attainment state, RUM pages, canary links, and AppRegistry application links separately.
- SLO state should be computed from metric windows rather than stored as static metadata.
- Application Signals resources should reference CloudWatch telemetry and not replace metrics/logs/traces storage.

## Operation Groups

### List

- Operations: `ListAuditFindings`, `ListEntityEvents`, `ListGroupingAttributeDefinitions`, `ListServiceDependencies`, `ListServiceDependents`, `ListServiceLevelObjectiveExclusionWindows`, `ListServiceOperations`, `ListServices`, `ListServiceStates`, `ListTagsForResource`
- Traits: `readonly` (9), `paginated` (7)
- Common required input members in this group: `StartTime`, `EndTime`, `KeyAttributes`

### Batch

- Operations: `BatchGetServiceLevelObjectiveBudgetReport`, `BatchUpdateExclusionWindows`
- Common required input members in this group: `SloIds`

### Delete

- Operations: `DeleteGroupingConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetService`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutGroupingConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Start

- Operations: `StartDiscovery`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetServiceLevelObjectiveBudgetReport` | `POST /budget-report` | - | `Timestamp`, `SloIds` | - | `BatchGetServiceLevelObjectiveBudgetReportOutput` | `ThrottlingException`, `ValidationException` | Use this operation to retrieve one or more service level objective (SLO) budget reports . An error budget is the amount of time or requests in an unhealthy state that your service can accumulate during an interval be ... |
| `BatchUpdateExclusionWindows` | `PATCH /exclusion-windows` | - | `SloIds` | - | `BatchUpdateExclusionWindowsOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add or remove time window exclusions for one or more Service Level Objectives (SLOs). |
| `DeleteGroupingConfiguration` | `DELETE /grouping-configuration` | `idempotent` | - | - | `DeleteGroupingConfigurationOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Deletes the grouping configuration for this account. This removes all custom grouping attribute definitions that were previously configured. |
| `GetService` | `POST /service` | `readonly` | `StartTime`, `EndTime`, `KeyAttributes` | - | `GetServiceOutput` | `ThrottlingException`, `ValidationException` | Returns information about a service discovered by Application Signals. |
| `ListAuditFindings` | `POST /auditFindings` | `readonly` | `StartTime`, `EndTime`, `AuditTargets` | - | `ListAuditFindingsOutput` | `ThrottlingException`, `ValidationException` | Returns a list of audit findings that provide automated analysis of service behavior and root cause analysis. These findings help identify the most significant observations about your services, including performance ... |
| `ListEntityEvents` | `POST /events` | `readonly`, `paginated` | `Entity`, `StartTime`, `EndTime` | - | `ListEntityEventsOutput` | `ThrottlingException`, `ValidationException` | Returns a list of change events for a specific entity, such as deployments, configuration changes, or other state-changing activities. This operation helps track the history of changes that may have affected service ... |
| `ListGroupingAttributeDefinitions` | `POST /grouping-attribute-definitions` | `readonly` | - | - | `ListGroupingAttributeDefinitionsOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns the current grouping configuration for this account, including all custom grouping attribute definitions that have been configured. These definitions determine how services are logically grouped based on tele ... |
| `ListServiceDependencies` | `POST /service-dependencies` | `readonly`, `paginated` | `StartTime`, `EndTime`, `KeyAttributes` | - | `ListServiceDependenciesOutput` | `ThrottlingException`, `ValidationException` | Returns a list of service dependencies of the service that you specify. A dependency is an infrastructure component that an operation of this service connects with. Dependencies can include Amazon Web Services servic ... |
| `ListServiceDependents` | `POST /service-dependents` | `readonly`, `paginated` | `StartTime`, `EndTime`, `KeyAttributes` | - | `ListServiceDependentsOutput` | `ThrottlingException`, `ValidationException` | Returns the list of dependents that invoked the specified service during the provided time range. Dependents include other services, CloudWatch Synthetics canaries, and clients that are instrumented with CloudWatch R ... |
| `ListServiceLevelObjectiveExclusionWindows` | `GET /slo/{Id}/exclusion-windows` | `readonly`, `paginated` | `Id` | - | `ListServiceLevelObjectiveExclusionWindowsOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves all exclusion windows configured for a specific SLO. |
| `ListServiceOperations` | `POST /service-operations` | `readonly`, `paginated` | `StartTime`, `EndTime`, `KeyAttributes` | - | `ListServiceOperationsOutput` | `ThrottlingException`, `ValidationException` | Returns a list of the operations of this service that have been discovered by Application Signals. Only the operations that were invoked during the specified time range are returned. |
| `ListServices` | `GET /services` | `readonly`, `paginated` | `StartTime`, `EndTime` | - | `ListServicesOutput` | `ThrottlingException`, `ValidationException` | Returns a list of services that have been discovered by Application Signals. A service represents a minimum logical and transactional unit that completes a business function. Services are discovered through Applicati ... |
| `ListServiceStates` | `POST /service/states` | `paginated` | `StartTime`, `EndTime` | - | `ListServiceStatesOutput` | `ThrottlingException`, `ValidationException` | Returns information about the last deployment and other change states of services. This API provides visibility into recent changes that may have affected service performance, helping with troubleshooting and change ... |
| `ListTagsForResource` | `GET /tags` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ThrottlingException` | Displays the tags associated with a CloudWatch resource. Tags can be assigned to service level objectives. |
| `PutGroupingConfiguration` | `PUT /grouping-configuration` | `idempotent` | `GroupingAttributeDefinitions` | - | `PutGroupingConfigurationOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Creates or updates the grouping configuration for this account. This operation allows you to define custom grouping attributes that determine how services are logically grouped based on telemetry attributes, Amazon W ... |
| `StartDiscovery` | `POST /start-discovery` | - | - | - | `StartDiscoveryOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Enables this Amazon Web Services account to be able to use CloudWatch Application Signals by creating the AWSServiceRoleForCloudWatchApplicationSignals service-linked role. This service- linked role has the following ... |
| `TagResource` | `POST /tag-resource` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Assigns one or more tags (key-value pairs) to the specified CloudWatch resource, such as a service level objective. Tags can help you organize and categorize your resources. You can also use them to scope user permis ... |
| `UntagResource` | `POST /untag-resource` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetService` | - | `StartTime -> StartTime`, `EndTime -> EndTime` | - | - |
| `ListAuditFindings` | - | `StartTime -> StartTime`, `EndTime -> EndTime` | - | - |
| `ListEntityEvents` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListGroupingAttributeDefinitions` | - | `NextToken -> NextToken`, `AwsAccountId -> AwsAccountId`, `IncludeLinkedAccounts -> IncludeLinkedAccounts` | - | - |
| `ListServiceDependencies` | - | `StartTime -> StartTime`, `EndTime -> EndTime`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListServiceDependents` | - | `StartTime -> StartTime`, `EndTime -> EndTime`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListServiceLevelObjectiveExclusionWindows` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListServiceOperations` | - | `StartTime -> StartTime`, `EndTime -> EndTime`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListServices` | - | `StartTime -> StartTime`, `EndTime -> EndTime`, `MaxResults -> MaxResults`, `NextToken -> NextToken`, `IncludeLinkedAccounts -> IncludeLinkedAccounts`, `AwsAccountId -> AwsAccountId` | - | - |
| `ListTagsForResource` | - | `ResourceArn -> ResourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You don't have sufficient permissions to perform this action. |
| `ConflictException` | `structure` | Message | This operation attempted to create a resource that already exists. |
| `ResourceNotFoundException` | `structure` | ResourceType, ResourceId, Message | Resource not found. |
| `ServiceQuotaExceededException` | `structure` | Message | This request exceeds a service quota. |
| `ThrottlingException` | `structure` | Message | The request was throttled because of quota limits. |
| `ValidationException` | `structure` | message | The resource is not valid. |
| `BatchGetServiceLevelObjectiveBudgetReportInput` | `structure` | Timestamp, SloIds | - |
| `BatchGetServiceLevelObjectiveBudgetReportOutput` | `structure` | Timestamp, Reports, Errors | - |
| `BatchUpdateExclusionWindowsInput` | `structure` | SloIds, AddExclusionWindows, RemoveExclusionWindows | - |
| `BatchUpdateExclusionWindowsOutput` | `structure` | SloIds, Errors | - |
| `DeleteGroupingConfigurationOutput` | `structure` | **empty (no members)** | - |
| `GetServiceInput` | `structure` | StartTime, EndTime, KeyAttributes | - |
| `GetServiceOutput` | `structure` | Service, StartTime, EndTime, LogGroupReferences | - |
| `ListAuditFindingsInput` | `structure` | StartTime, EndTime, Auditors, AuditTargets, DetailLevel, NextToken, MaxResults | - |
| `ListAuditFindingsOutput` | `structure` | StartTime, EndTime, AuditFindings, NextToken | - |
| `ListEntityEventsInput` | `structure` | Entity, StartTime, EndTime, MaxResults, NextToken | - |
| `ListEntityEventsOutput` | `structure` | StartTime, EndTime, ChangeEvents, NextToken | - |
| `ListGroupingAttributeDefinitionsInput` | `structure` | NextToken, AwsAccountId, IncludeLinkedAccounts | - |
| `ListGroupingAttributeDefinitionsOutput` | `structure` | GroupingAttributeDefinitions, UpdatedAt, NextToken | - |
| `ListServiceDependenciesInput` | `structure` | StartTime, EndTime, KeyAttributes, MaxResults, NextToken | - |
| `ListServiceDependenciesOutput` | `structure` | StartTime, EndTime, ServiceDependencies, NextToken | - |
| `ListServiceDependentsInput` | `structure` | StartTime, EndTime, KeyAttributes, MaxResults, NextToken | - |
| `ListServiceDependentsOutput` | `structure` | StartTime, EndTime, ServiceDependents, NextToken | - |
| `ListServiceLevelObjectiveExclusionWindowsInput` | `structure` | Id, MaxResults, NextToken | - |
| `ListServiceLevelObjectiveExclusionWindowsOutput` | `structure` | ExclusionWindows, NextToken | - |
| `ListServiceOperationsInput` | `structure` | StartTime, EndTime, KeyAttributes, MaxResults, NextToken | - |
| `ListServiceOperationsOutput` | `structure` | StartTime, EndTime, ServiceOperations, NextToken | - |
| `ListServicesInput` | `structure` | StartTime, EndTime, MaxResults, NextToken, IncludeLinkedAccounts, AwsAccountId | - |
| `ListServicesOutput` | `structure` | StartTime, EndTime, ServiceSummaries, NextToken | - |
| `ListServiceStatesInput` | `structure` | StartTime, EndTime, MaxResults, NextToken, IncludeLinkedAccounts, AwsAccountId, AttributeFilters | - |
| `ListServiceStatesOutput` | `structure` | StartTime, EndTime, ServiceStates, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `PutGroupingConfigurationInput` | `structure` | GroupingAttributeDefinitions | - |
| `PutGroupingConfigurationOutput` | `structure` | GroupingConfiguration | - |
| `StartDiscoveryInput` | `structure` | **empty (no members)** | - |
| `StartDiscoveryOutput` | `structure` | **empty (no members)** | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `ChangeEventType` | `enum` | DEPLOYMENT, CONFIGURATION | - |
| `ConnectionType` | `enum` | INDIRECT, DIRECT | - |
| `DetailLevel` | `enum` | BRIEF, DETAILED | - |
| `DurationUnit` | `enum` | MINUTE, HOUR, DAY, MONTH | - |
| `EvaluationType` | `enum` | PERIOD_BASED, REQUEST_BASED | - |
| `MetricSourceType` | `enum` | SERVICE_OPERATION, CLOUDWATCH_METRIC, SERVICE_DEPENDENCY, APPMONITOR, CANARY, SERVICE | - |
| `SelectionType` | `enum` | EXPLICIT, PREFIX, REGEX | The strategy for selecting operations to include in a service-level SLO. EXPLICIT — You provide a specific list of operations in the Components field of Com ... |
| `ServiceLevelIndicatorComparisonOperator` | `enum` | GREATER_THAN_OR_EQUAL_TO, GREATER_THAN, LESS_THAN, LESS_THAN_OR_EQUAL_TO | - |
| `ServiceLevelIndicatorMetricType` | `enum` | LATENCY, AVAILABILITY | - |
| `ServiceLevelObjectiveBudgetStatus` | `enum` | OK, WARNING, BREACHED, INSUFFICIENT_DATA | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

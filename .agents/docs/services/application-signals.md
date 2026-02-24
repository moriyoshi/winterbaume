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

- Operations: `ListAuditFindings`, `ListEntityEvents`, `ListGroupingAttributeDefinitions`, `ListServiceDependencies`, `ListServiceDependents`, `ListServiceLevelObjectiveExclusionWindows`, `ListServiceLevelObjectives`, `ListServiceOperations`, `ListServiceStates`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (8), `readonly` (10)
- Common required input members in this group: `AuditTargets`, `EndTime`, `Entity`, `Id`, `KeyAttributes`, `ResourceArn`, `StartTime`

### Batch

- Operations: `BatchGetServiceLevelObjectiveBudgetReport`, `BatchUpdateExclusionWindows`
- Common required input members in this group: `SloIds`, `Timestamp`

### Delete

- Operations: `DeleteGroupingConfiguration`, `DeleteServiceLevelObjective`
- Traits: `idempotent` (2)
- Common required input members in this group: `Id`

### Get

- Operations: `GetService`, `GetServiceLevelObjective`
- Traits: `readonly` (2)
- Common required input members in this group: `EndTime`, `Id`, `KeyAttributes`, `StartTime`

### Create

- Operations: `CreateServiceLevelObjective`
- Common required input members in this group: `Name`

### Put

- Operations: `PutGroupingConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `GroupingAttributeDefinitions`

### Start

- Operations: `StartDiscovery`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateServiceLevelObjective`
- Common required input members in this group: `Id`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetServiceLevelObjectiveBudgetReport` | `POST /budget-report` | - | `SloIds`, `Timestamp` | - | `BatchGetServiceLevelObjectiveBudgetReportOutput` | `ThrottlingException`, `ValidationException` | Use this operation to retrieve one or more service level objective (SLO) budget reports . An error budget is the amount of time or requests in an unhealthy state that your service can accumulate during an interval before your overall SLO budget health is... |
| `BatchUpdateExclusionWindows` | `PATCH /exclusion-windows` | - | `SloIds` | - | `BatchUpdateExclusionWindowsOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add or remove time window exclusions for one or more Service Level Objectives (SLOs). |
| `CreateServiceLevelObjective` | `POST /slo` | - | `Name` | - | `CreateServiceLevelObjectiveOutput` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a service level objective (SLO), which can help you ensure that your critical business operations are meeting customer expectations. Use SLOs to set and track specific target levels for the reliability and availability of your applications and... |
| `DeleteGroupingConfiguration` | `DELETE /grouping-configuration` | `idempotent` | - | - | `DeleteGroupingConfigurationOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Deletes the grouping configuration for this account. This removes all custom grouping attribute definitions that were previously configured. |
| `DeleteServiceLevelObjective` | `DELETE /slo/{Id}` | `idempotent` | `Id` | - | `DeleteServiceLevelObjectiveOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified service level objective. |
| `GetService` | `POST /service` | `readonly` | `EndTime`, `KeyAttributes`, `StartTime` | - | `GetServiceOutput` | `ThrottlingException`, `ValidationException` | Returns information about a service discovered by Application Signals. |
| `GetServiceLevelObjective` | `GET /slo/{Id}` | `readonly` | `Id` | - | `GetServiceLevelObjectiveOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about one SLO created in the account. |
| `ListAuditFindings` | `POST /auditFindings` | `readonly` | `AuditTargets`, `EndTime`, `StartTime` | - | `ListAuditFindingsOutput` | `ThrottlingException`, `ValidationException` | Returns a list of audit findings that provide automated analysis of service behavior and root cause analysis. These findings help identify the most significant observations about your services, including performance issues, anomalies, and potential problems. |
| `ListEntityEvents` | `POST /events` | `readonly`, `paginated` | `EndTime`, `Entity`, `StartTime` | - | `ListEntityEventsOutput` | `ThrottlingException`, `ValidationException` | Returns a list of change events for a specific entity, such as deployments, configuration changes, or other state-changing activities. This operation helps track the history of changes that may have affected service performance. |
| `ListGroupingAttributeDefinitions` | `POST /grouping-attribute-definitions` | `readonly` | - | - | `ListGroupingAttributeDefinitionsOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Returns the current grouping configuration for this account, including all custom grouping attribute definitions that have been configured. These definitions determine how services are logically grouped based on telemetry attributes, Amazon Web Services tags... |
| `ListServiceDependencies` | `POST /service-dependencies` | `readonly`, `paginated` | `EndTime`, `KeyAttributes`, `StartTime` | - | `ListServiceDependenciesOutput` | `ThrottlingException`, `ValidationException` | Returns a list of service dependencies of the service that you specify. A dependency is an infrastructure component that an operation of this service connects with. |
| `ListServiceDependents` | `POST /service-dependents` | `readonly`, `paginated` | `EndTime`, `KeyAttributes`, `StartTime` | - | `ListServiceDependentsOutput` | `ThrottlingException`, `ValidationException` | Returns the list of dependents that invoked the specified service during the provided time range. Dependents include other services, CloudWatch Synthetics canaries, and clients that are instrumented with CloudWatch RUM app monitors. |
| `ListServiceLevelObjectiveExclusionWindows` | `GET /slo/{Id}/exclusion-windows` | `readonly`, `paginated` | `Id` | - | `ListServiceLevelObjectiveExclusionWindowsOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves all exclusion windows configured for a specific SLO. |
| `ListServiceLevelObjectives` | `POST /slos` | `readonly`, `paginated` | - | - | `ListServiceLevelObjectivesOutput` | `ThrottlingException`, `ValidationException` | Returns a list of SLOs created in this account. |
| `ListServiceOperations` | `POST /service-operations` | `readonly`, `paginated` | `EndTime`, `KeyAttributes`, `StartTime` | - | `ListServiceOperationsOutput` | `ThrottlingException`, `ValidationException` | Returns a list of the operations of this service that have been discovered by Application Signals. Only the operations that were invoked during the specified time range are returned. |
| `ListServiceStates` | `POST /service/states` | `paginated` | `EndTime`, `StartTime` | - | `ListServiceStatesOutput` | `ThrottlingException`, `ValidationException` | Returns information about the last deployment and other change states of services. This API provides visibility into recent changes that may have affected service performance, helping with troubleshooting and change correlation. |
| `ListServices` | `GET /services` | `readonly`, `paginated` | `EndTime`, `StartTime` | - | `ListServicesOutput` | `ThrottlingException`, `ValidationException` | Returns a list of services that have been discovered by Application Signals. A service represents a minimum logical and transactional unit that completes a business function. |
| `ListTagsForResource` | `GET /tags` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ThrottlingException` | Displays the tags associated with a CloudWatch resource. Tags can be assigned to service level objectives. |
| `PutGroupingConfiguration` | `PUT /grouping-configuration` | `idempotent` | `GroupingAttributeDefinitions` | - | `PutGroupingConfigurationOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Creates or updates the grouping configuration for this account. This operation allows you to define custom grouping attributes that determine how services are logically grouped based on telemetry attributes, Amazon Web Services tags, or predefined mappings. |
| `StartDiscovery` | `POST /start-discovery` | - | - | - | `StartDiscoveryOutput` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Enables this Amazon Web Services account to be able to use CloudWatch Application Signals by creating the AWSServiceRoleForCloudWatchApplicationSignals service-linked role. This service- linked role has the following permissions: `xray:GetServiceGraph`... |
| `TagResource` | `POST /tag-resource` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Assigns one or more tags (key-value pairs) to the specified CloudWatch resource, such as a service level objective. Tags can help you organize and categorize your resources. |
| `UntagResource` | `POST /untag-resource` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags from the specified resource. |
| `UpdateServiceLevelObjective` | `PATCH /slo/{Id}` | - | `Id` | - | `UpdateServiceLevelObjectiveOutput` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing service level objective (SLO). If you omit parameters, the previous values of those parameters are retained. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ThrottlingException` | `structure` | `Message` | The request was throttled because of quota limits. |
| `ValidationException` | `structure` | `message` | The resource is not valid. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Resource not found. |
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient permissions to perform this action. |
| `ServiceQuotaExceededException` | `structure` | `Message` | This request exceeds a service quota. |
| `BatchGetServiceLevelObjectiveBudgetReportInput` | `structure` | `SloIds`, `Timestamp` | - |
| `BatchGetServiceLevelObjectiveBudgetReportOutput` | `structure` | `Errors`, `Reports`, `Timestamp` | - |
| `BatchUpdateExclusionWindowsInput` | `structure` | `AddExclusionWindows`, `RemoveExclusionWindows`, `SloIds` | - |
| `BatchUpdateExclusionWindowsOutput` | `structure` | `Errors`, `SloIds` | - |
| `CreateServiceLevelObjectiveInput` | `structure` | `BurnRateConfigurations`, `Description`, `Goal`, `Name`, `RequestBasedSliConfig`, `SliConfig`, `Tags` | - |
| `CreateServiceLevelObjectiveOutput` | `structure` | `Slo` | - |
| `ConflictException` | `structure` | `Message` | This operation attempted to create a resource that already exists. |
| `DeleteGroupingConfigurationOutput` | `structure` | - | - |
| `DeleteServiceLevelObjectiveInput` | `structure` | `Id` | - |
| `DeleteServiceLevelObjectiveOutput` | `structure` | - | - |
| `GetServiceInput` | `structure` | `EndTime`, `KeyAttributes`, `StartTime` | - |
| `GetServiceOutput` | `structure` | `EndTime`, `LogGroupReferences`, `Service`, `StartTime` | - |
| `GetServiceLevelObjectiveInput` | `structure` | `Id` | - |
| `GetServiceLevelObjectiveOutput` | `structure` | `Slo` | - |
| `ListAuditFindingsInput` | `structure` | `AuditTargets`, `Auditors`, `DetailLevel`, `EndTime`, `MaxResults`, `NextToken`, `StartTime` | - |
| `ListAuditFindingsOutput` | `structure` | `AuditFindings`, `EndTime`, `NextToken`, `StartTime` | - |
| `ListEntityEventsInput` | `structure` | `EndTime`, `Entity`, `MaxResults`, `NextToken`, `StartTime` | - |
| `ListEntityEventsOutput` | `structure` | `ChangeEvents`, `EndTime`, `NextToken`, `StartTime` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon Prometheus Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Service for Prometheus is a serverless, Prometheus-compatible monitoring service for container metrics that makes it easier to securely monitor container environments at scale. With Amazon Managed Service for Prometheus, you can use the same open-source Prometheus data model and query language that you use today to monitor the performance of your containerized workloads, and also enjoy improved scalability, availability, and security without having to manage the underlying infrastructure. For more information about Amazon Managed Service for Prometheus, see the Amazon Managed Service for Prometheus User Guide. Amazon Managed Service for Prometheus includes two APIs. Use the Amazon Web Services API described in this guide to manage Amazon Managed Service for Prometheus resources, such as workspaces, rule groups, and alert managers.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-amp/tests/scenario_test.rs`: provision a workspace, attach rule group namespaces, list and describe them, then clean up the workspace.
- Backported from `scenario_test.rs`: configure workspace logging, verify the logging configuration, update it, and delete it.
- Scenario insight from EC2: exercise account or service defaults for Amazon Prometheus Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: support managed Prometheus workspaces for metric ingestion, alert/rule management, logging, tagging, and workspace lifecycle automation.

## Service Identity and Protocol

- AWS model slug: `amp`
- AWS SDK for Rust slug: `amp`
- Model version: `2020-08-01`
- Model file: `vendor/api-models-aws/models/amp/service/2020-08-01/amp-2020-08-01.json`
- SDK ID: `amp`
- Endpoint prefix: `-`
- ARN namespace: `aps`
- CloudFormation name: `-`
- CloudTrail event source: `CLOUDTRAIL_PLACEHOLDER_REPLACED_BY_CDK`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (10), `Delete` (9), `Create` (7), `Update` (6), `List` (5), `Put` (4), `Get` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAlertManagerDefinition`, `CreateAnomalyDetector`, `CreateLoggingConfiguration`, `CreateQueryLoggingConfiguration`, `CreateRuleGroupsNamespace`, `CreateScraper`, `CreateWorkspace`, `DeleteAlertManagerDefinition`, `DeleteAnomalyDetector`, `DeleteLoggingConfiguration`, `DeleteQueryLoggingConfiguration`, `DeleteResourcePolicy`, `DeleteRuleGroupsNamespace`, `DeleteScraper`, `DeleteScraperLoggingConfiguration`, `DeleteWorkspace`, `PutAlertManagerDefinition`, `PutAnomalyDetector`, `PutResourcePolicy`, `PutRuleGroupsNamespace`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAlertManagerDefinition`, `DescribeAnomalyDetector`, `DescribeLoggingConfiguration`, `DescribeQueryLoggingConfiguration`, `DescribeResourcePolicy`, `DescribeRuleGroupsNamespace`, `DescribeScraper`, `DescribeScraperLoggingConfiguration`, `DescribeWorkspace`, `DescribeWorkspaceConfiguration`, `GetDefaultScraperConfiguration`, `ListAnomalyDetectors`, `ListRuleGroupsNamespaces`, `ListScrapers`, `ListTagsForResource`, `ListWorkspaces`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 27 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 44 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AlertManagerDefinition` | `workspaceId` | put: `CreateAlertManagerDefinition`; read: `DescribeAlertManagerDefinition`; update: `PutAlertManagerDefinition`; delete: `DeleteAlertManagerDefinition` | - | The alert manager definition, a YAML configuration for the alertmanager in your Amazon Managed Service for Prometheus workspace. |
| `AnomalyDetector` | `anomalyDetectorId`, `workspaceId` | create: `CreateAnomalyDetector`; put: `PutAnomalyDetector`; read: `DescribeAnomalyDetector`; delete: `DeleteAnomalyDetector`; list: `ListAnomalyDetectors` | - | - |
| `LoggingConfiguration` | `workspaceId` | put: `CreateLoggingConfiguration`; read: `DescribeLoggingConfiguration`; update: `UpdateLoggingConfiguration`; delete: `DeleteLoggingConfiguration` | - | The definition of logging configuration in an Amazon Managed Service for Prometheus workspace. |
| `QueryLoggingConfiguration` | `workspaceId` | put: `CreateQueryLoggingConfiguration`; read: `DescribeQueryLoggingConfiguration`; update: `UpdateQueryLoggingConfiguration`; delete: `DeleteQueryLoggingConfiguration` | - | - |
| `RuleGroupsNamespace` | `name`, `workspaceId` | put: `CreateRuleGroupsNamespace`; read: `DescribeRuleGroupsNamespace`; update: `PutRuleGroupsNamespace`; delete: `DeleteRuleGroupsNamespace`; list: `ListRuleGroupsNamespaces` | - | The definition of a rule groups namespace in an Amazon Managed Service for Prometheus workspace. |
| `Scraper` | `scraperId` | create: `CreateScraper`; read: `DescribeScraper`; update: `UpdateScraper`; delete: `DeleteScraper`; list: `ListScrapers` | - | A scraper is a fully-managed agentless collector that discovers and pulls metrics automatically. |
| `ScraperLoggingConfiguration` | `scraperId` | put: `UpdateScraperLoggingConfiguration`; read: `DescribeScraperLoggingConfiguration`; update: `UpdateScraperLoggingConfiguration`; delete: `DeleteScraperLoggingConfiguration` | - | - |
| `Workspace` | `workspaceId` | create: `CreateWorkspace`; read: `DescribeWorkspace`; update: `UpdateWorkspaceAlias`; delete: `DeleteWorkspace`; list: `ListWorkspaces` | - | An Amazon Managed Service for Prometheus workspace is a logical and isolated Prometheus server dedicated to ingesting, storing, and querying your Prometheus-compatible metrics. |
| `WorkspaceConfiguration` | `workspaceId` | read: `DescribeWorkspaceConfiguration`; update: `UpdateWorkspaceConfiguration` | - | - |
| `WorkspaceResourcePolicy` | `workspaceId` | put: `PutResourcePolicy`; read: `DescribeResourcePolicy`; delete: `DeleteResourcePolicy` | - | - |
## Operation Groups

### Describe

- Operations: `DescribeAlertManagerDefinition`, `DescribeAnomalyDetector`, `DescribeLoggingConfiguration`, `DescribeQueryLoggingConfiguration`, `DescribeResourcePolicy`, `DescribeRuleGroupsNamespace`, `DescribeScraper`, `DescribeScraperLoggingConfiguration`, `DescribeWorkspace`, `DescribeWorkspaceConfiguration`
- Traits: `readonly` (10)
- Common required input members in this group: `anomalyDetectorId`, `name`, `scraperId`, `workspaceId`

### Delete

- Operations: `DeleteAlertManagerDefinition`, `DeleteAnomalyDetector`, `DeleteLoggingConfiguration`, `DeleteQueryLoggingConfiguration`, `DeleteResourcePolicy`, `DeleteRuleGroupsNamespace`, `DeleteScraper`, `DeleteScraperLoggingConfiguration`, `DeleteWorkspace`
- Traits: `idempotency-token` (9), `idempotent` (9)
- Common required input members in this group: `anomalyDetectorId`, `name`, `scraperId`, `workspaceId`

### Create

- Operations: `CreateAlertManagerDefinition`, `CreateAnomalyDetector`, `CreateLoggingConfiguration`, `CreateQueryLoggingConfiguration`, `CreateRuleGroupsNamespace`, `CreateScraper`, `CreateWorkspace`
- Traits: `idempotency-token` (7), `idempotent` (7)
- Common required input members in this group: `alias`, `configuration`, `data`, `destination`, `destinations`, `logGroupArn`, `name`, `scrapeConfiguration`, `source`, `workspaceId`

### Update

- Operations: `UpdateLoggingConfiguration`, `UpdateQueryLoggingConfiguration`, `UpdateScraper`, `UpdateScraperLoggingConfiguration`, `UpdateWorkspaceAlias`, `UpdateWorkspaceConfiguration`
- Traits: `idempotency-token` (5), `idempotent` (6)
- Common required input members in this group: `destinations`, `logGroupArn`, `loggingDestination`, `scraperId`, `workspaceId`

### List

- Operations: `ListAnomalyDetectors`, `ListRuleGroupsNamespaces`, `ListScrapers`, `ListTagsForResource`, `ListWorkspaces`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `resourceArn`, `workspaceId`

### Put

- Operations: `PutAlertManagerDefinition`, `PutAnomalyDetector`, `PutResourcePolicy`, `PutRuleGroupsNamespace`
- Traits: `idempotency-token` (4), `idempotent` (4)
- Common required input members in this group: `anomalyDetectorId`, `configuration`, `data`, `name`, `policyDocument`, `workspaceId`

### Get

- Operations: `GetDefaultScraperConfiguration`
- Traits: `readonly` (1)

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAlertManagerDefinition` | `POST /workspaces/{workspaceId}/alertmanager/definition` | `idempotent`, `idempotency-token` | `data`, `workspaceId` | `clientToken` | `CreateAlertManagerDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The `CreateAlertManagerDefinition` operation creates the alert manager definition in a workspace. If a workspace already has an alert manager definition, don't use this operation to update it. |
| `CreateAnomalyDetector` | `POST /workspaces/{workspaceId}/anomalydetectors` | `idempotent`, `idempotency-token` | `alias`, `configuration`, `workspaceId` | `clientToken` | `CreateAnomalyDetectorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an anomaly detector within a workspace using the Random Cut Forest algorithm for time-series analysis. The anomaly detector analyzes Amazon Managed Service for Prometheus metrics to identify unusual patterns and behaviors. |
| `CreateLoggingConfiguration` | `POST /workspaces/{workspaceId}/logging` | `idempotent`, `idempotency-token` | `logGroupArn`, `workspaceId` | `clientToken` | `CreateLoggingConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | The `CreateLoggingConfiguration` operation creates rules and alerting logging configuration for the workspace. Use this operation to set the CloudWatch log group to which the logs will be published to. |
| `CreateQueryLoggingConfiguration` | `POST /workspaces/{workspaceId}/logging/query` | `idempotent`, `idempotency-token` | `destinations`, `workspaceId` | `clientToken` | `CreateQueryLoggingConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a query logging configuration for the specified workspace. This operation enables logging of queries that exceed the specified QSP threshold. |
| `CreateRuleGroupsNamespace` | `POST /workspaces/{workspaceId}/rulegroupsnamespaces` | `idempotent`, `idempotency-token` | `data`, `name`, `workspaceId` | `clientToken` | `CreateRuleGroupsNamespaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The `CreateRuleGroupsNamespace` operation creates a rule groups namespace within a workspace. A rule groups namespace is associated with exactly one rules file. |
| `CreateScraper` | `POST /scrapers` | `idempotent`, `idempotency-token` | `destination`, `scrapeConfiguration`, `source` | `clientToken` | `CreateScraperResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The `CreateScraper` operation creates a scraper to collect metrics. A scraper pulls metrics from Prometheus-compatible sources and sends them to your Amazon Managed Service for Prometheus workspace. |
| `CreateWorkspace` | `POST /workspaces` | `idempotent`, `idempotency-token` | - | `clientToken` | `CreateWorkspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a Prometheus workspace. A workspace is a logical space dedicated to the storage and querying of Prometheus metrics. |
| `DeleteAlertManagerDefinition` | `DELETE /workspaces/{workspaceId}/alertmanager/definition` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the alert manager definition from a workspace. |
| `DeleteAnomalyDetector` | `DELETE /workspaces/{workspaceId}/anomalydetectors/{anomalyDetectorId}` | `idempotent`, `idempotency-token` | `anomalyDetectorId`, `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes an anomaly detector from a workspace. This operation is idempotent. |
| `DeleteLoggingConfiguration` | `DELETE /workspaces/{workspaceId}/logging` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the rules and alerting logging configuration for a workspace. These logging configurations are only for rules and alerting logs. |
| `DeleteQueryLoggingConfiguration` | `DELETE /workspaces/{workspaceId}/logging/query` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the query logging configuration for the specified workspace. |
| `DeleteResourcePolicy` | `DELETE /workspaces/{workspaceId}/policy` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource-based policy attached to an Amazon Managed Service for Prometheus workspace. |
| `DeleteRuleGroupsNamespace` | `DELETE /workspaces/{workspaceId}/rulegroupsnamespaces/{name}` | `idempotent`, `idempotency-token` | `name`, `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes one rule groups namespace and its associated rule groups definition. |
| `DeleteScraper` | `DELETE /scrapers/{scraperId}` | `idempotent`, `idempotency-token` | `scraperId` | `clientToken` | `DeleteScraperResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `DeleteScraper` operation deletes one scraper, and stops any metrics collection that the scraper performs. |
| `DeleteScraperLoggingConfiguration` | `DELETE /scrapers/{scraperId}/logging-configuration` | `idempotent`, `idempotency-token` | `scraperId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the logging configuration for a Amazon Managed Service for Prometheus scraper. |
| `DeleteWorkspace` | `DELETE /workspaces/{workspaceId}` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing workspace. When you delete a workspace, the data that has been ingested into it is not immediately deleted. |
| `DescribeAlertManagerDefinition` | `GET /workspaces/{workspaceId}/alertmanager/definition` | `readonly` | `workspaceId` | - | `DescribeAlertManagerDefinitionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the full information about the alert manager definition for a workspace. |
| `DescribeAnomalyDetector` | `GET /workspaces/{workspaceId}/anomalydetectors/{anomalyDetectorId}` | `readonly` | `anomalyDetectorId`, `workspaceId` | - | `DescribeAnomalyDetectorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific anomaly detector, including its status and configuration. |
| `DescribeLoggingConfiguration` | `GET /workspaces/{workspaceId}/logging` | `readonly` | `workspaceId` | - | `DescribeLoggingConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns complete information about the current rules and alerting logging configuration of the workspace. These logging configurations are only for rules and alerting logs. |
| `DescribeQueryLoggingConfiguration` | `GET /workspaces/{workspaceId}/logging/query` | `readonly` | `workspaceId` | - | `DescribeQueryLoggingConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the details of the query logging configuration for the specified workspace. |
| `DescribeResourcePolicy` | `GET /workspaces/{workspaceId}/policy` | `readonly` | `workspaceId` | - | `DescribeResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the resource-based policy attached to an Amazon Managed Service for Prometheus workspace. |
| `DescribeRuleGroupsNamespace` | `GET /workspaces/{workspaceId}/rulegroupsnamespaces/{name}` | `readonly` | `name`, `workspaceId` | - | `DescribeRuleGroupsNamespaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns complete information about one rule groups namespace. To retrieve a list of rule groups namespaces, use `ListRuleGroupsNamespaces`. |
| `DescribeScraper` | `GET /scrapers/{scraperId}` | `readonly` | `scraperId` | - | `DescribeScraperResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `DescribeScraper` operation displays information about an existing scraper. |
| `DescribeScraperLoggingConfiguration` | `GET /scrapers/{scraperId}/logging-configuration` | `readonly` | `scraperId` | - | `DescribeScraperLoggingConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the logging configuration for a Amazon Managed Service for Prometheus scraper. |
| `DescribeWorkspace` | `GET /workspaces/{workspaceId}` | `readonly` | `workspaceId` | - | `DescribeWorkspaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about an existing workspace. |
| `DescribeWorkspaceConfiguration` | `GET /workspaces/{workspaceId}/configuration` | `readonly` | `workspaceId` | - | `DescribeWorkspaceConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to return information about the configuration of a workspace. The configuration details returned include workspace configuration status, label set limits, and retention period. |
| `GetDefaultScraperConfiguration` | `GET /scraperconfiguration` | `readonly` | - | - | `GetDefaultScraperConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | The `GetDefaultScraperConfiguration` operation returns the default scraper configuration used when Amazon EKS creates a scraper for you. |
| `ListAnomalyDetectors` | `GET /workspaces/{workspaceId}/anomalydetectors` | `readonly`, `paginated` | `workspaceId` | - | `ListAnomalyDetectorsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of anomaly detectors for a workspace with optional filtering by alias. |
| `ListRuleGroupsNamespaces` | `GET /workspaces/{workspaceId}/rulegroupsnamespaces` | `readonly`, `paginated` | `workspaceId` | - | `ListRuleGroupsNamespacesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of rule groups namespaces in a workspace. |
| `ListScrapers` | `GET /scrapers` | `readonly`, `paginated` | - | - | `ListScrapersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | The `ListScrapers` operation lists all of the scrapers in your account. This includes scrapers being created or deleted. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `ListTagsForResource` operation returns the tags that are associated with an Amazon Managed Service for Prometheus resource. Currently, the only resources that can be tagged are scrapers, workspaces, and rule groups namespaces. |
| `ListWorkspaces` | `GET /workspaces` | `readonly`, `paginated` | - | - | `ListWorkspacesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all of the Amazon Managed Service for Prometheus workspaces in your account. This includes workspaces being created or deleted. |
| `PutAlertManagerDefinition` | `PUT /workspaces/{workspaceId}/alertmanager/definition` | `idempotent`, `idempotency-token` | `data`, `workspaceId` | `clientToken` | `PutAlertManagerDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing alert manager definition in a workspace. If the workspace does not already have an alert manager definition, don't use this operation to create it. |
| `PutAnomalyDetector` | `PUT /workspaces/{workspaceId}/anomalydetectors/{anomalyDetectorId}` | `idempotent`, `idempotency-token` | `anomalyDetectorId`, `configuration`, `workspaceId` | `clientToken` | `PutAnomalyDetectorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | When you call `PutAnomalyDetector`, the operation creates a new anomaly detector if one doesn't exist, or updates an existing one. Each call to this operation triggers a complete retraining of the detector, which includes querying the minimum required samples... |
| `PutResourcePolicy` | `PUT /workspaces/{workspaceId}/policy` | `idempotent`, `idempotency-token` | `policyDocument`, `workspaceId` | `clientToken` | `PutResourcePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates a resource-based policy for an Amazon Managed Service for Prometheus workspace. Use resource-based policies to grant permissions to other AWS accounts or services to access your workspace. |
| `PutRuleGroupsNamespace` | `PUT /workspaces/{workspaceId}/rulegroupsnamespaces/{name}` | `idempotent`, `idempotency-token` | `data`, `name`, `workspaceId` | `clientToken` | `PutRuleGroupsNamespaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing rule groups namespace within a workspace. A rule groups namespace is associated with exactly one rules file. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `TagResource` operation associates tags with an Amazon Managed Service for Prometheus resource. The only resources that can be tagged are rule groups namespaces, scrapers, and workspaces. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified tags from an Amazon Managed Service for Prometheus resource. The only resources that can be tagged are rule groups namespaces, scrapers, and workspaces. |
| `UpdateLoggingConfiguration` | `PUT /workspaces/{workspaceId}/logging` | `idempotent`, `idempotency-token` | `logGroupArn`, `workspaceId` | `clientToken` | `UpdateLoggingConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the log group ARN or the workspace ID of the current rules and alerting logging configuration. These logging configurations are only for rules and alerting logs. |
| `UpdateQueryLoggingConfiguration` | `PUT /workspaces/{workspaceId}/logging/query` | `idempotent`, `idempotency-token` | `destinations`, `workspaceId` | `clientToken` | `UpdateQueryLoggingConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the query logging configuration for the specified workspace. |
| `UpdateScraper` | `PUT /scrapers/{scraperId}` | `idempotent`, `idempotency-token` | `scraperId` | `clientToken` | `UpdateScraperResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing scraper. You can't use this function to update the source from which the scraper is collecting metrics. |
| `UpdateScraperLoggingConfiguration` | `PUT /scrapers/{scraperId}/logging-configuration` | `idempotent` | `loggingDestination`, `scraperId` | - | `UpdateScraperLoggingConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the logging configuration for a Amazon Managed Service for Prometheus scraper. |
| `UpdateWorkspaceAlias` | `POST /workspaces/{workspaceId}/alias` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the alias of an existing workspace. |
| `UpdateWorkspaceConfiguration` | `PATCH /workspaces/{workspaceId}/configuration` | `idempotent`, `idempotency-token` | `workspaceId` | `clientToken` | `UpdateWorkspaceConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Use this operation to create or update the label sets, label set limits, and retention period of a workspace. You must specify at least one of `limitsPerLabelSet` or `retentionPeriodInDays` for the request to be valid. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | An unexpected error occurred during the processing of the request. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The request references a resources that doesn't exist. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied due to request throttling. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request would cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | Completing the request would cause a service quota to be exceeded. |
| `CreateAlertManagerDefinitionRequest` | `structure` | `clientToken`, `data`, `workspaceId` | Represents the input of a `CreateAlertManagerDefinition` operation. |
| `CreateAlertManagerDefinitionResponse` | `structure` | `status` | Represents the output of a `CreateAlertManagerDefinition` operation. |
| `CreateAnomalyDetectorRequest` | `structure` | `alias`, `clientToken`, `configuration`, `evaluationIntervalInSeconds`, `labels`, `missingDataAction`, `tags`, `workspaceId` | - |
| `CreateAnomalyDetectorResponse` | `structure` | `anomalyDetectorId`, `arn`, `status`, `tags` | - |
| `CreateLoggingConfigurationRequest` | `structure` | `clientToken`, `logGroupArn`, `workspaceId` | Represents the input of a `CreateLoggingConfiguration` operation. |
| `CreateLoggingConfigurationResponse` | `structure` | `status` | Represents the output of a `CreateLoggingConfiguration` operation. |
| `CreateQueryLoggingConfigurationRequest` | `structure` | `clientToken`, `destinations`, `workspaceId` | - |
| `CreateQueryLoggingConfigurationResponse` | `structure` | `status` | - |
| `CreateRuleGroupsNamespaceRequest` | `structure` | `clientToken`, `data`, `name`, `tags`, `workspaceId` | Represents the input of a `CreateRuleGroupsNamespace` operation. |
| `CreateRuleGroupsNamespaceResponse` | `structure` | `arn`, `name`, `status`, `tags` | Represents the output of a `CreateRuleGroupsNamespace` operation. |
| `CreateScraperRequest` | `structure` | `alias`, `clientToken`, `destination`, `roleConfiguration`, `scrapeConfiguration`, `source`, `tags` | Represents the input of a `CreateScraper` operation. |
| `CreateScraperResponse` | `structure` | `arn`, `scraperId`, `status`, `tags` | Represents the output of a `CreateScraper` operation. |
| `CreateWorkspaceRequest` | `structure` | `alias`, `clientToken`, `kmsKeyArn`, `tags` | Represents the input of a `CreateWorkspace` operation. |
| `CreateWorkspaceResponse` | `structure` | `arn`, `kmsKeyArn`, `status`, `tags`, `workspaceId` | Represents the output of a `CreateWorkspace` operation. |
| `DeleteAlertManagerDefinitionRequest` | `structure` | `clientToken`, `workspaceId` | Represents the input of a `DeleteAlertManagerDefinition` operation. |
| `DeleteAnomalyDetectorRequest` | `structure` | `anomalyDetectorId`, `clientToken`, `workspaceId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

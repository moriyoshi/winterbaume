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

### Get

- Operations: `GetDefaultScraperConfiguration`
- Traits: `readonly` (1)
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
| `GetDefaultScraperConfiguration` | `GET /scraperconfiguration` | `readonly` | - | - | `GetDefaultScraperConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | The GetDefaultScraperConfiguration operation returns the default scraper configuration used when Amazon EKS creates a scraper for you. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The ListTagsForResource operation returns the tags that are associated with an Amazon Managed Service for Prometheus resource. Currently, the only resources that can be tagged are scrapers, workspaces, and rule group ... |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The TagResource operation associates tags with an Amazon Managed Service for Prometheus resource. The only resources that can be tagged are rule groups namespaces, scrapers, and workspaces. If you specify a new tag k ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified tags from an Amazon Managed Service for Prometheus resource. The only resources that can be tagged are rule groups namespaces, scrapers, and workspaces. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request would cause an inconsistent state. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | An unexpected error occurred during the processing of the request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The request references a resources that doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | Completing the request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `GetDefaultScraperConfigurationRequest` | `structure` | **empty (no members)** | Represents the input of a GetDefaultScraperConfiguration operation. |
| `GetDefaultScraperConfigurationResponse` | `structure` | configuration | Represents the output of a GetDefaultScraperConfiguration operation. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AnomalyDetectorStatusCode` | `enum` | CREATING, ACTIVE, UPDATING, DELETING, CREATION_FAILED, UPDATE_FAILED, DELETION_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

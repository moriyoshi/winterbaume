# AWS Billing and Cost Management Dashboards

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Billing and Cost Management Dashboards is a service that enables you to create, manage, and share dashboards that combine multiple visualizations of your Amazon Web Services cost and usage data. You can combine multiple data sources including Cost Explorer, Savings Plans, and Reserved Instance metrics into unified dashboards, helping you analyze spending patterns and share cost insights across your organization. You can use the Amazon Web Services Billing and Cost Management Dashboards API to programmatically create, manage, and share dashboards. This includes creating custom dashboards, configuring widgets, managing dashboard permissions, and sharing dashboards across accounts in your organization.

## Possible Usage Scenarios
- From the AWS documentation and model: create, update, list, and delete Billing and Cost Management dashboards for cost visibility.
- From the operation surface: model dashboard definitions, widget layout, account-level billing views, and tag-based dashboard administration.

## Service Identity and Protocol

- AWS model slug: `bcm-dashboards`
- AWS SDK for Rust slug: `bcmdashboards`
- Model version: `2025-08-18`
- Model file: `vendor/api-models-aws/models/bcm-dashboards/service/2025-08-18/bcm-dashboards-2025-08-18.json`
- SDK ID: `BCM Dashboards`
- Endpoint prefix: `bcm-dashboards`
- ARN namespace: `bcm-dashboards`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `List` (2), `Create` (1), `Delete` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDashboard`, `DeleteDashboard`, `TagResource`, `UntagResource`, `UpdateDashboard`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDashboard`, `GetResourcePolicy`, `ListDashboards`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetDashboard`, `GetResourcePolicy`, `GetScheduledReport`
- Traits: `readonly` (3)
- Common required input members in this group: `arn`

### List

- Operations: `ListDashboards`, `ListScheduledReports`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Create

- Operations: `CreateDashboard`, `CreateScheduledReport`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteDashboard`, `DeleteScheduledReport`
- Common required input members in this group: `arn`

### Update

- Operations: `UpdateDashboard`, `UpdateScheduledReport`
- Common required input members in this group: `arn`

### Execute

- Operations: `ExecuteScheduledReport`
- Traits: `idempotency-token` (1)
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
| `CreateDashboard` | `-` | - | `name`, `widgets` | - | `CreateDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new dashboard that can contain multiple widgets displaying cost and usage data. You can add custom widgets or use predefined widgets, arranging them in your preferred layout. |
| `CreateScheduledReport` | `-` | `idempotency-token` | `scheduledReport` | `clientToken` | `CreateScheduledReportResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new scheduled report for a dashboard. A scheduled report automatically generates and delivers dashboard snapshots on a recurring schedule. Reports are delivered within 15 minutes of the scheduled delivery time. |
| `DeleteDashboard` | `-` | - | `arn` | - | `DeleteDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a specified dashboard. This action cannot be undone. |
| `DeleteScheduledReport` | `-` | - | `arn` | - | `DeleteScheduledReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a specified scheduled report. This is an irreversible operation. |
| `ExecuteScheduledReport` | `-` | `idempotency-token` | `arn` | `clientToken` | `ExecuteScheduledReportResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Triggers an immediate execution of a scheduled report, outside of its regular schedule. The scheduled report must be in ENABLED state. Calling this operation on a DISABLED scheduled report returns a ValidationExcepti ... |
| `GetDashboard` | `-` | `readonly` | `arn` | - | `GetDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration and metadata of a specified dashboard, including its widgets and layout settings. |
| `GetResourcePolicy` | `-` | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the resource-based policy attached to a dashboard, showing sharing configurations and permissions. |
| `GetScheduledReport` | `-` | `readonly` | `arn` | - | `GetScheduledReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration and metadata of a specified scheduled report. |
| `ListDashboards` | `-` | `readonly`, `paginated` | - | - | `ListDashboardsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all dashboards in your account. |
| `ListScheduledReports` | `-` | `readonly`, `paginated` | - | - | `ListScheduledReportsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of scheduled reports in your account. |
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all tags associated with a specified dashboard resource. |
| `TagResource` | `-` | - | `resourceArn`, `resourceTags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified dashboard resource. |
| `UntagResource` | `-` | - | `resourceArn`, `resourceTagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes specified tags from a dashboard resource. |
| `UpdateDashboard` | `-` | - | `arn`, `name` | - | `UpdateDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing dashboard's properties, including its name, description, and widget configurations. |
| `UpdateScheduledReport` | `-` | - | `arn` | - | `UpdateScheduledReportResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing scheduled report's properties, including its name, description, schedule configuration, and widget settings. Only the parameters included in the request are updated; all other properties remain un ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateScheduledReport` | `clientToken -> X-Amzn-Client-Token` | - | - | - |
| `ExecuteScheduledReport` | `clientToken -> X-Amzn-Client-Token` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient permissions to perform this action. Verify your IAM permissions and any resource policies. |
| `ConflictException` | `structure` | message | The request could not be completed due to a conflict with the current state of the resource. For example, attempting to create a resource that already exist ... |
| `InternalServerException` | `structure` | message | An internal error occurred while processing the request. Retry your request. If the problem persists, contact Amazon Web Services Support. |
| `ResourceNotFoundException` | `structure` | message | The specified resource (dashboard, policy, or widget) was not found. Verify the ARN and try again. |
| `ServiceQuotaExceededException` | `structure` | message | The request would exceed a service quota. Review the service quotas for Amazon Web Services Billing and Cost Management Dashboards and retry your request. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. Reduce the frequency of requests and use exponential backoff. |
| `ValidationException` | `structure` | message | The input parameters do not satisfy the requirements. Check the error message for specific validation details. |
| `CreateDashboardRequest` | `structure` | name, description, widgets, resourceTags | - |
| `CreateDashboardResponse` | `structure` | arn | - |
| `CreateScheduledReportRequest` | `structure` | scheduledReport, resourceTags, clientToken | - |
| `CreateScheduledReportResponse` | `structure` | arn | - |
| `DeleteDashboardRequest` | `structure` | arn | - |
| `DeleteDashboardResponse` | `structure` | arn | - |
| `DeleteScheduledReportRequest` | `structure` | arn | - |
| `DeleteScheduledReportResponse` | `structure` | arn | - |
| `ExecuteScheduledReportRequest` | `structure` | arn, clientToken, dryRun | - |
| `ExecuteScheduledReportResponse` | `structure` | healthStatus, executionTriggered | - |
| `GetDashboardRequest` | `structure` | arn | - |
| `GetDashboardResponse` | `structure` | arn, name, description, type, widgets, createdAt, updatedAt | - |
| `GetResourcePolicyRequest` | `structure` | resourceArn | - |
| `GetResourcePolicyResponse` | `structure` | resourceArn, policyDocument | - |
| `GetScheduledReportRequest` | `structure` | arn | - |
| `GetScheduledReportResponse` | `structure` | scheduledReport | - |
| `ListDashboardsRequest` | `structure` | maxResults, nextToken | - |
| `ListDashboardsResponse` | `structure` | dashboards, nextToken | - |
| `ListScheduledReportsRequest` | `structure` | nextToken, maxResults | - |
| `ListScheduledReportsResponse` | `structure` | scheduledReports, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | resourceTags | - |
| `TagResourceRequest` | `structure` | resourceArn, resourceTags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, resourceTagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateDashboardRequest` | `structure` | arn, name, description, widgets | - |
| `UpdateDashboardResponse` | `structure` | arn | - |
| `UpdateScheduledReportRequest` | `structure` | arn, name, description, dashboardArn, scheduledReportExecutionRoleArn, scheduleConfig, widgetIds, widgetDateRangeOverride, clearWidgetIds, clearWidgetDateRangeOverride | - |
| `UpdateScheduledReportResponse` | `structure` | arn | - |
| `DashboardType` | `enum` | CUSTOM | - |
| `DateTimeType` | `enum` | ABSOLUTE, RELATIVE | - |
| `Dimension` | `enum` | AZ, INSTANCE_TYPE, LINKED_ACCOUNT, OPERATION, PURCHASE_TYPE, REGION, SERVICE, USAGE_TYPE, USAGE_TYPE_GROUP, RECORD_TYPE, RESOURCE_ID, SUBSCRIPTION_ID, ... (+14) | - |
| `Granularity` | `enum` | HOURLY, DAILY, MONTHLY | - |
| `GroupDefinitionType` | `enum` | DIMENSION, TAG, COST_CATEGORY | - |
| `HealthStatusCode` | `enum` | HEALTHY, UNHEALTHY | - |
| `MatchOption` | `enum` | EQUALS, ABSENT, STARTS_WITH, ENDS_WITH, CONTAINS, GREATER_THAN_OR_EQUAL, CASE_SENSITIVE, CASE_INSENSITIVE | - |
| `MetricName` | `enum` | AmortizedCost, BlendedCost, NetAmortizedCost, NetUnblendedCost, NormalizedUsageAmount, UnblendedCost, UsageQuantity, SpendCoveredBySavingsPlans, Hour, Unit, Cost | - |
| `ScheduleState` | `enum` | ENABLED, DISABLED | - |
| `StatusReason` | `enum` | DATA_SOURCE_ACCESS_DENIED, EXECUTION_ROLE_ASSUME_FAILED, EXECUTION_ROLE_INSUFFICIENT_PERMISSIONS, DASHBOARD_NOT_FOUND, DASHBOARD_ACCESS_DENIED, INTERNAL_FAILURE, WIDGET_ID_NOT_FOUND | - |
| `VisualType` | `enum` | LINE, BAR, STACK | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

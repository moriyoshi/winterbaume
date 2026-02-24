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

- Operations: `GetDashboard`, `GetResourcePolicy`
- Traits: `readonly` (2)
- Common required input members in this group: `arn`, `resourceArn`

### List

- Operations: `ListDashboards`, `ListTagsForResource`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateDashboard`
- Common required input members in this group: `name`, `widgets`

### Delete

- Operations: `DeleteDashboard`
- Common required input members in this group: `arn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `resourceTags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `resourceTagKeys`

### Update

- Operations: `UpdateDashboard`
- Common required input members in this group: `arn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateDashboard` | - | - | `name`, `widgets` | - | `CreateDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new dashboard that can contain multiple widgets displaying cost and usage data. You can add custom widgets or use predefined widgets, arranging them in your preferred layout. |
| `DeleteDashboard` | - | - | `arn` | - | `DeleteDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a specified dashboard. This action cannot be undone. |
| `GetDashboard` | - | `readonly` | `arn` | - | `GetDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration and metadata of a specified dashboard, including its widgets and layout settings. |
| `GetResourcePolicy` | - | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the resource-based policy attached to a dashboard, showing sharing configurations and permissions. |
| `ListDashboards` | - | `readonly`, `paginated` | - | - | `ListDashboardsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all dashboards in your account. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all tags associated with a specified dashboard resource. |
| `TagResource` | - | - | `resourceArn`, `resourceTags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified dashboard resource. |
| `UntagResource` | - | - | `resourceArn`, `resourceTagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes specified tags from a dashboard resource. |
| `UpdateDashboard` | - | - | `arn` | - | `UpdateDashboardResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing dashboard's properties, including its name, description, and widget configurations. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An internal error occurred while processing the request. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | The input parameters do not satisfy the requirements. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient permissions to perform this action. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource (dashboard, policy, or widget) was not found. |
| `CreateDashboardRequest` | `structure` | `description`, `name`, `resourceTags`, `widgets` | - |
| `CreateDashboardResponse` | `structure` | `arn` | - |
| `ServiceQuotaExceededException` | `structure` | `message` | The request would exceed service quotas. |
| `DeleteDashboardRequest` | `structure` | `arn` | - |
| `DeleteDashboardResponse` | `structure` | `arn` | - |
| `GetDashboardRequest` | `structure` | `arn` | - |
| `GetDashboardResponse` | `structure` | `arn`, `createdAt`, `description`, `name`, `type`, `updatedAt`, `widgets` | - |
| `GetResourcePolicyRequest` | `structure` | `resourceArn` | - |
| `GetResourcePolicyResponse` | `structure` | `policyDocument`, `resourceArn` | - |
| `ListDashboardsRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListDashboardsResponse` | `structure` | `dashboards`, `nextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `resourceTags` | - |
| `TagResourceRequest` | `structure` | `resourceArn`, `resourceTags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `resourceArn`, `resourceTagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |
| `UpdateDashboardRequest` | `structure` | `arn`, `description`, `name`, `widgets` | - |
| `UpdateDashboardResponse` | `structure` | `arn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

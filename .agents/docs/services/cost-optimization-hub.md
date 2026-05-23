# Cost Optimization Hub

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Cost Optimization Hub API to programmatically identify, filter, aggregate, and quantify savings for your cost optimization recommendations across multiple Amazon Web Services Regions and Amazon Web Services accounts in your organization. The Cost Optimization Hub API provides the following endpoint: https://cost-optimization-hub.us-east-1.amazonaws.com

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Cost Optimization Hub by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Cost Optimization Hub workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get`, `Update` operation families, including `ListEfficiencyMetrics`, `ListEnrollmentStatuses`, `ListRecommendationSummaries`, `ListRecommendations`, `GetPreferences`, `GetRecommendation`.

## Service Identity and Protocol

- AWS model slug: `cost-optimization-hub`
- AWS SDK for Rust slug: `costoptimizationhub`
- Model version: `2022-07-26`
- Model file: `vendor/api-models-aws/models/cost-optimization-hub/service/2022-07-26/cost-optimization-hub-2022-07-26.json`
- SDK ID: `Cost Optimization Hub`
- Endpoint prefix: `cost-optimization-hub`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (2), `Update` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `UpdateEnrollmentStatus`, `UpdatePreferences`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetPreferences`, `GetRecommendation`, `ListEfficiencyMetrics`, `ListEnrollmentStatuses`, `ListRecommendationSummaries`, `ListRecommendations`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- 8 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`, `Lambda`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cost-management/latest/userguide/cost-optimization-hub.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/coh-estimated-monthly-savings.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/coh-preferences.html

Research outcomes:
- Cost Optimization Hub consolidates cost optimisation recommendations from multiple AWS services and categories.
- Recommendations include rightsizing, idle resource deletion, purchase options, and other savings opportunities across accounts and Regions.
- Estimated monthly savings can be aggregated, with logic to avoid double-counting overlapping savings recommendations.
- Preferences include savings estimation mode and commitment preferences for Reserved Instance and Savings Plans recommendations.
- The service supports AWS Organizations integration and service-linked roles for recommendation collection.
- Compute Optimizer savings estimation mode can depend on Cost Optimization Hub enrolment.

Parity implications:
- Model enrolment, service-linked role state, recommendations, account/Region/resource scope, savings estimates, preferences, and aggregation groups separately.
- Aggregated savings should deduplicate overlapping recommendations.
- Preference updates should influence generated or returned recommendations consistently with Cost Explorer and Compute Optimizer integrations.

## Operation Groups

### List

- Operations: `ListEfficiencyMetrics`, `ListEnrollmentStatuses`, `ListRecommendations`, `ListRecommendationSummaries`
- Traits: `readonly` (4), `paginated` (4)
- Common required input members in this group: -

### Get

- Operations: `GetPreferences`, `GetRecommendation`
- Traits: `readonly` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateEnrollmentStatus`, `UpdatePreferences`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetPreferences` | `-` | `readonly` | - | - | `GetPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a set of preferences for an account in order to add account-specific preferences into the service. These preferences impact how the savings associated with recommendations are presented—estimated savings afte ... |
| `GetRecommendation` | `-` | `readonly` | `recommendationId` | - | `GetRecommendationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns both the current and recommended resource configuration and the estimated cost impact for a recommendation. The recommendationId is only valid for up to a maximum of 24 hours as recommendations are refreshed ... |
| `ListEfficiencyMetrics` | `-` | `readonly`, `paginated` | `granularity`, `timePeriod` | - | `ListEfficiencyMetricsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns cost efficiency metrics aggregated over time and optionally grouped by a specified dimension. The metrics provide insights into your cost optimization progress by tracking estimated savings, spending, and mea ... |
| `ListEnrollmentStatuses` | `-` | `readonly`, `paginated` | - | - | `ListEnrollmentStatusesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the enrollment status for an account. It can also return the list of accounts that are enrolled under the organization. |
| `ListRecommendations` | `-` | `readonly`, `paginated` | - | - | `ListRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of recommendations. |
| `ListRecommendationSummaries` | `-` | `readonly`, `paginated` | `groupBy` | - | `ListRecommendationSummariesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a concise representation of savings estimates for resources. Also returns de-duped savings across different types of recommendations. The following filters are not supported for this API: recommendationIds , ... |
| `UpdateEnrollmentStatus` | `-` | - | `status` | - | `UpdateEnrollmentStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the enrollment (opt in and opt out) status of an account to the Cost Optimization Hub service. If the account is a management account of an organization, this action can also be used to enroll member accounts ... |
| `UpdatePreferences` | `-` | - | - | - | `UpdatePreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates a set of preferences for an account in order to add account-specific preferences into the service. These preferences impact how the savings associated with recommendations are presented. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You are not authorized to use this operation with the given parameters. |
| `InternalServerException` | `structure` | message | An error on the server occurred during the processing of your request. Try again later. |
| `ResourceNotFoundException` | `structure` | message, resourceId | The specified Amazon Resource Name (ARN) in the request doesn't exist. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fields | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `GetPreferencesRequest` | `structure` | **empty (no members)** | - |
| `GetPreferencesResponse` | `structure` | savingsEstimationMode, memberAccountDiscountVisibility, preferredCommitment | - |
| `GetRecommendationRequest` | `structure` | recommendationId | - |
| `GetRecommendationResponse` | `structure` | recommendationId, resourceId, resourceArn, accountId, currencyCode, recommendationLookbackPeriodInDays, costCalculationLookbackPeriodInDays, estimatedSavingsPercentage, estimatedSavingsOverCostCalculationLookbackPeriod, currentResourceType, recommendedResourceType, region, ... (+11) | - |
| `ListEfficiencyMetricsRequest` | `structure` | groupBy, granularity, timePeriod, maxResults, orderBy, nextToken | - |
| `ListEfficiencyMetricsResponse` | `structure` | efficiencyMetricsByGroup, nextToken | - |
| `ListEnrollmentStatusesRequest` | `structure` | includeOrganizationInfo, accountId, nextToken, maxResults | - |
| `ListEnrollmentStatusesResponse` | `structure` | items, includeMemberAccounts, nextToken | - |
| `ListRecommendationsRequest` | `structure` | filter, orderBy, includeAllRecommendations, maxResults, nextToken | - |
| `ListRecommendationsResponse` | `structure` | items, nextToken | - |
| `ListRecommendationSummariesRequest` | `structure` | filter, groupBy, maxResults, metrics, nextToken | - |
| `ListRecommendationSummariesResponse` | `structure` | estimatedTotalDedupedSavings, items, groupBy, currencyCode, metrics, nextToken | - |
| `UpdateEnrollmentStatusRequest` | `structure` | status, includeMemberAccounts | - |
| `UpdateEnrollmentStatusResponse` | `structure` | status | - |
| `UpdatePreferencesRequest` | `structure` | savingsEstimationMode, memberAccountDiscountVisibility, preferredCommitment | - |
| `UpdatePreferencesResponse` | `structure` | savingsEstimationMode, memberAccountDiscountVisibility, preferredCommitment | - |
| `ActionType` | `enum` | RIGHTSIZE, STOP, UPGRADE, PURCHASE_SAVINGS_PLANS, PURCHASE_RESERVED_INSTANCES, MIGRATE_TO_GRAVITON, DELETE, SCALE_IN | - |
| `AllocationStrategy` | `enum` | PRIORITIZED, LOWEST_PRICE | - |
| `Ec2AutoScalingGroupType` | `enum` | SINGLE_INSTANCE_TYPE, MIXED_INSTANCE_TYPES | - |
| `EnrollmentStatus` | `enum` | ACTIVE, INACTIVE | - |
| `GranularityType` | `enum` | DAILY, MONTHLY | The time granularity for aggregating the cost efficiency metrics. |
| `ImplementationEffort` | `enum` | VERY_LOW, LOW, MEDIUM, HIGH, VERY_HIGH | - |
| `MemberAccountDiscountVisibility` | `enum` | ALL, NONE | - |
| `Order` | `enum` | ASC, DESC | - |
| `PaymentOption` | `enum` | ALL_UPFRONT, PARTIAL_UPFRONT, NO_UPFRONT | - |
| `ResourceType` | `enum` | EC2_INSTANCE, LAMBDA_FUNCTION, EBS_VOLUME, ECS_SERVICE, EC2_AUTO_SCALING_GROUP, EC2_INSTANCE_SAVINGS_PLANS, COMPUTE_SAVINGS_PLANS, SAGE_MAKER_SAVINGS_PLANS, EC2_RESERVED_INSTANCES, RDS_RESERVED_INSTANCES, OPEN_SEARCH_RESERVED_INSTANCES, REDSHIFT_RESERVED_INSTANCES, ... (+7) | - |
| `SavingsEstimationMode` | `enum` | BEFORE_DISCOUNTS, AFTER_DISCOUNTS | - |
| `Source` | `enum` | COMPUTE_OPTIMIZER, COST_EXPLORER | - |
| `SummaryMetrics` | `enum` | SAVINGS_PERCENTAGE | - |
| `Term` | `enum` | ONE_YEAR, THREE_YEARS | - |
| `ValidationExceptionReason` | `enum` | FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

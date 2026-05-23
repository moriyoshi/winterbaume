# AWS Billing and Cost Management Recommended Actions

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Billing and Cost Management Recommended Actions API to programmatically query your best practices and recommendations to optimize your costs. The Billing and Cost Management Recommended Actions API provides the following endpoint: https://bcm-recommended-actions.us-east-1.api.aws

## Possible Usage Scenarios
- Backported from `crates/winterbaume-bcmrecommendedactions/tests/scenario_test.rs`: query the recommended-action catalogue exposed by the service and verify returned recommendation metadata.
- From the AWS documentation and model: support cost-optimisation recommendation discovery, filtering, and account-level billing action review.

## Service Identity and Protocol

- AWS model slug: `bcm-recommended-actions`
- AWS SDK for Rust slug: `bcmrecommendedactions`
- Model version: `2024-11-14`
- Model file: `vendor/api-models-aws/models/bcm-recommended-actions/service/2024-11-14/bcm-recommended-actions-2024-11-14.json`
- SDK ID: `BCM Recommended Actions`
- Endpoint prefix: `bcm-recommended-actions`
- ARN namespace: `bcm-recommended-actions`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ListRecommendedActions`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListRecommendedActions`
- Traits: `paginated` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListRecommendedActions` | `-` | `paginated` | - | - | `ListRecommendedActionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of recommended actions that match the filter criteria. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | message | An unexpected error occurred during the processing of your request. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ListRecommendedActionsRequest` | `structure` | filter, maxResults, nextToken | - |
| `ListRecommendedActionsResponse` | `structure` | recommendedActions, nextToken | - |
| `ActionType` | `enum` | ADD_ALTERNATE_BILLING_CONTACT, CREATE_ANOMALY_MONITOR, CREATE_BUDGET, ENABLE_COST_OPTIMIZATION_HUB, MIGRATE_TO_GRANULAR_PERMISSIONS, PAYMENTS_DUE, PAYMENTS_PAST_DUE, REVIEW_ANOMALIES, REVIEW_BUDGET_ALERTS, REVIEW_BUDGETS_EXCEEDED, REVIEW_EXPIRING_RI, REVIEW_EXPIRING_SP, ... (+8) | - |
| `Feature` | `enum` | ACCOUNT, BUDGETS, COST_ANOMALY_DETECTION, COST_OPTIMIZATION_HUB, FREE_TIER, IAM, PAYMENTS, RESERVATIONS, SAVINGS_PLANS, TAX_SETTINGS | - |
| `FilterName` | `enum` | FEATURE, SEVERITY, TYPE | - |
| `MatchOption` | `enum` | EQUALS, NOT_EQUALS | - |
| `Severity` | `enum` | INFO, WARNING, CRITICAL | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

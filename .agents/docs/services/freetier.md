# AWS Free Tier

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Amazon Web Services Free Tier API to query programmatically your Free Tier usage data. Free Tier tracks your monthly usage data for all free tier offers that are associated with your Amazon Web Services account. You can use the Free Tier API to filter and show only the data that you want. Service endpoint The Free Tier API provides the following endpoint: For more information, see Using the Amazon Web Services Free Tier in the Billing User Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Free Tier workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `List`, `Upgrade` operation families, including `GetAccountActivity`, `GetAccountPlanState`, `GetFreeTierUsage`, `ListAccountActivities`, `UpgradeAccountPlan`.

## Service Identity and Protocol

- AWS model slug: `freetier`
- AWS SDK for Rust slug: `freetier`
- Model version: `2023-09-07`
- Model file: `vendor/api-models-aws/models/freetier/service/2023-09-07/freetier-2023-09-07.json`
- SDK ID: `FreeTier`
- Endpoint prefix: `freetier`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `List` (1), `Upgrade` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountActivity`, `GetAccountPlanState`, `GetFreeTierUsage`, `ListAccountActivities`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAccountActivity`, `GetAccountPlanState`, `GetFreeTierUsage`
- Traits: `readonly` (2), `paginated` (1)
- Common required input members in this group: -

### List

- Operations: `ListAccountActivities`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Upgrade

- Operations: `UpgradeAccountPlan`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetAccountActivity` | `-` | `readonly` | `activityId` | - | `GetAccountActivityResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specific activity record that is available to the customer. |
| `GetAccountPlanState` | `-` | `readonly` | - | - | `GetAccountPlanStateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This returns all of the information related to the state of the account plan related to Free Tier. |
| `GetFreeTierUsage` | `-` | `paginated` | - | - | `GetFreeTierUsageResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all Free Tier usage objects that match your filters. |
| `ListAccountActivities` | `-` | `readonly`, `paginated` | - | - | `ListAccountActivitiesResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of activities that are available. This operation supports pagination and filtering by status. |
| `UpgradeAccountPlan` | `-` | - | `accountPlanType` | - | `UpgradeAccountPlanResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The account plan type for the Amazon Web Services account. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | message | An unexpected error occurred during the processing of your request. |
| `ResourceNotFoundException` | `structure` | message | This exception is thrown when the requested resource cannot be found. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `GetAccountActivityRequest` | `structure` | activityId, languageCode | - |
| `GetAccountActivityResponse` | `structure` | activityId, title, description, status, instructionsUrl, reward, estimatedTimeToCompleteInMinutes, expiresAt, startedAt, completedAt | - |
| `GetAccountPlanStateRequest` | `structure` | **empty (no members)** | - |
| `GetAccountPlanStateResponse` | `structure` | accountId, accountPlanType, accountPlanStatus, accountPlanRemainingCredits, accountPlanExpirationDate | - |
| `GetFreeTierUsageRequest` | `structure` | filter, maxResults, nextToken | - |
| `GetFreeTierUsageResponse` | `structure` | freeTierUsages, nextToken | - |
| `ListAccountActivitiesRequest` | `structure` | filterActivityStatuses, nextToken, maxResults, languageCode | - |
| `ListAccountActivitiesResponse` | `structure` | activities, nextToken | - |
| `UpgradeAccountPlanRequest` | `structure` | accountPlanType | - |
| `UpgradeAccountPlanResponse` | `structure` | accountId, accountPlanType, accountPlanStatus | - |
| `AccountPlanStatus` | `enum` | NOT_STARTED, ACTIVE, EXPIRED | - |
| `AccountPlanType` | `enum` | FREE, PAID | - |
| `ActivityStatus` | `enum` | NOT_STARTED, IN_PROGRESS, COMPLETED, EXPIRING | - |
| `CurrencyCode` | `enum` | USD | - |
| `Dimension` | `enum` | SERVICE, OPERATION, USAGE_TYPE, REGION, FREE_TIER_TYPE, DESCRIPTION, USAGE_PERCENTAGE | - |
| `LanguageCode` | `enum` | EN_US, EN_GB, ID_ID, DE_DE, ES_ES, FR_FR, JA_JP, IT_IT, PT_PT, KO_KR, ZH_CN, ZH_TW, ... (+1) | - |
| `MatchOption` | `enum` | EQUALS, STARTS_WITH, ENDS_WITH, CONTAINS, GREATER_THAN_OR_EQUAL | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

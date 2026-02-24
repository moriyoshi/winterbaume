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
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `activityId`

### List

- Operations: `ListAccountActivities`
- Traits: `paginated` (1), `readonly` (1)

### Upgrade

- Operations: `UpgradeAccountPlan`
- Common required input members in this group: `accountPlanType`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetAccountActivity` | - | `readonly` | `activityId` | - | `GetAccountActivityResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a specific activity record that is available to the customer. |
| `GetAccountPlanState` | - | `readonly` | - | - | `GetAccountPlanStateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This returns all of the information related to the state of the account plan related to Free Tier. |
| `GetFreeTierUsage` | - | `paginated` | - | - | `GetFreeTierUsageResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all Free Tier usage objects that match your filters. |
| `ListAccountActivities` | - | `readonly`, `paginated` | - | - | `ListAccountActivitiesResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of activities that are available. This operation supports pagination and filtering by status. |
| `UpgradeAccountPlan` | - | - | `accountPlanType` | - | `UpgradeAccountPlanResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The account plan type for the Amazon Web Services account. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An unexpected error occurred during the processing of your request. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message` | This exception is thrown when the requested resource cannot be found. |
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access to perform this action. |
| `GetAccountActivityRequest` | `structure` | `activityId`, `languageCode` | - |
| `GetAccountActivityResponse` | `structure` | `activityId`, `completedAt`, `description`, `estimatedTimeToCompleteInMinutes`, `expiresAt`, `instructionsUrl`, `reward`, `startedAt`, `status`, `title` | - |
| `GetAccountPlanStateRequest` | `structure` | - | - |
| `GetAccountPlanStateResponse` | `structure` | `accountId`, `accountPlanExpirationDate`, `accountPlanRemainingCredits`, `accountPlanStatus`, `accountPlanType` | - |
| `GetFreeTierUsageRequest` | `structure` | `filter`, `maxResults`, `nextToken` | - |
| `GetFreeTierUsageResponse` | `structure` | `freeTierUsages`, `nextToken` | - |
| `ListAccountActivitiesRequest` | `structure` | `filterActivityStatuses`, `languageCode`, `maxResults`, `nextToken` | - |
| `ListAccountActivitiesResponse` | `structure` | `activities`, `nextToken` | - |
| `UpgradeAccountPlanRequest` | `structure` | `accountPlanType` | - |
| `UpgradeAccountPlanResponse` | `structure` | `accountId`, `accountPlanStatus`, `accountPlanType` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Savings Plans

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Savings Plans are a pricing model that offer significant savings on Amazon Web Services usage (for example, on Amazon EC2 instances). You commit to a consistent amount of usage per hour, in the specified currency, for a term of one or three years, and receive a lower price for that usage. For more information, see the Amazon Web Services Savings Plans User Guide.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-savingsplans/tests/scenario_test.rs`: purchase a savings plan, tag it, verify tags on describe, and return it.
- Backported from `scenario_test.rs`: manage multiple plans, filter by state, and untag resources.
- From the AWS documentation and model: support commitment-offering discovery, plan purchase and return workflows, utilisation/coverage/rate descriptions, state filtering, and tag-based financial inventory.

## Service Identity and Protocol

- AWS model slug: `savingsplans`
- AWS SDK for Rust slug: `savingsplans`
- Model version: `2019-06-28`
- Model file: `vendor/api-models-aws/models/savingsplans/service/2019-06-28/savingsplans-2019-06-28.json`
- SDK ID: `savingsplans`
- Endpoint prefix: `savingsplans`
- ARN namespace: `savingsplans`
- CloudFormation name: `Savingsplans`
- CloudTrail event source: `savingsplans.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (4), `Create` (1), `Delete` (1), `List` (1), `Return` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSavingsPlan`, `DeleteQueuedSavingsPlan`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeSavingsPlanRates`, `DescribeSavingsPlans`, `DescribeSavingsPlansOfferingRates`, `DescribeSavingsPlansOfferings`, `ListTagsForResource`.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeSavingsPlanRates`, `DescribeSavingsPlans`, `DescribeSavingsPlansOfferingRates`, `DescribeSavingsPlansOfferings`
- Common required input members in this group: `savingsPlanId`

### Create

- Operations: `CreateSavingsPlan`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `commitment`, `savingsPlanOfferingId`

### Delete

- Operations: `DeleteQueuedSavingsPlan`
- Common required input members in this group: `savingsPlanId`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `resourceArn`

### Return

- Operations: `ReturnSavingsPlan`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `savingsPlanId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateSavingsPlan` | `POST /CreateSavingsPlan` | `idempotency-token` | `commitment`, `savingsPlanOfferingId` | `clientToken` | `CreateSavingsPlanResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a Savings Plan. |
| `DeleteQueuedSavingsPlan` | `POST /DeleteQueuedSavingsPlan` | - | `savingsPlanId` | - | `DeleteQueuedSavingsPlanResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Deletes the queued purchase for the specified Savings Plan. |
| `DescribeSavingsPlanRates` | `POST /DescribeSavingsPlanRates` | - | `savingsPlanId` | - | `DescribeSavingsPlanRatesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the rates for a specific, existing Savings Plan. |
| `DescribeSavingsPlans` | `POST /DescribeSavingsPlans` | - | - | - | `DescribeSavingsPlansResponse` | `InternalServerException`, `ValidationException` | Describes the specified Savings Plans. |
| `DescribeSavingsPlansOfferingRates` | `POST /DescribeSavingsPlansOfferingRates` | - | - | - | `DescribeSavingsPlansOfferingRatesResponse` | `InternalServerException`, `ValidationException` | Describes the offering rates for Savings Plans you might want to purchase. |
| `DescribeSavingsPlansOfferings` | `POST /DescribeSavingsPlansOfferings` | - | - | - | `DescribeSavingsPlansOfferingsResponse` | `InternalServerException`, `ValidationException` | Describes the offerings for the specified Savings Plans. |
| `ListTagsForResource` | `POST /ListTagsForResource` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags for the specified resource. |
| `ReturnSavingsPlan` | `POST /ReturnSavingsPlan` | `idempotency-token` | `savingsPlanId` | `clientToken` | `ReturnSavingsPlanResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns the specified Savings Plan. |
| `TagResource` | `POST /TagResource` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `POST /UntagResource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified tags from the specified resource. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An unexpected error occurred. |
| `ValidationException` | `structure` | `message` | One of the input parameters is not valid. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | `message` | A service quota has been exceeded. |
| `CreateSavingsPlanRequest` | `structure` | `clientToken`, `commitment`, `purchaseTime`, `savingsPlanOfferingId`, `tags`, `upfrontPaymentAmount` | - |
| `CreateSavingsPlanResponse` | `structure` | `savingsPlanId` | - |
| `DeleteQueuedSavingsPlanRequest` | `structure` | `savingsPlanId` | - |
| `DeleteQueuedSavingsPlanResponse` | `structure` | - | - |
| `DescribeSavingsPlanRatesRequest` | `structure` | `filters`, `maxResults`, `nextToken`, `savingsPlanId` | - |
| `DescribeSavingsPlanRatesResponse` | `structure` | `nextToken`, `savingsPlanId`, `searchResults` | - |
| `DescribeSavingsPlansRequest` | `structure` | `filters`, `maxResults`, `nextToken`, `savingsPlanArns`, `savingsPlanIds`, `states` | - |
| `DescribeSavingsPlansResponse` | `structure` | `nextToken`, `savingsPlans` | - |
| `DescribeSavingsPlansOfferingRatesRequest` | `structure` | `filters`, `maxResults`, `nextToken`, `operations`, `products`, `savingsPlanOfferingIds`, `savingsPlanPaymentOptions`, `savingsPlanTypes`, `serviceCodes`, `usageTypes` | - |
| `DescribeSavingsPlansOfferingRatesResponse` | `structure` | `nextToken`, `searchResults` | - |
| `DescribeSavingsPlansOfferingsRequest` | `structure` | `currencies`, `descriptions`, `durations`, `filters`, `maxResults`, `nextToken`, `offeringIds`, `operations`, `paymentOptions`, `planTypes`, `productType`, `serviceCodes`, ... (+1) | - |
| `DescribeSavingsPlansOfferingsResponse` | `structure` | `nextToken`, `searchResults` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `ReturnSavingsPlanRequest` | `structure` | `clientToken`, `savingsPlanId` | - |
| `ReturnSavingsPlanResponse` | `structure` | `savingsPlanId` | - |
| `TagResourceRequest` | `structure` | `resourceArn`, `tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `resourceArn`, `tagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

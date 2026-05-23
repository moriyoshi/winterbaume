# Amazon Personalize Runtime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

No high-level service documentation is embedded in the AWS API model.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Personalize Runtime workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get` operation families, including `GetActionRecommendations`, `GetPersonalizedRanking`, `GetRecommendations`.

## Service Identity and Protocol

- AWS model slug: `personalize-runtime`
- AWS SDK for Rust slug: `personalizeruntime`
- Model version: `2018-05-22`
- Model file: `vendor/api-models-aws/models/personalize-runtime/service/2018-05-22/personalize-runtime-2018-05-22.json`
- SDK ID: `Personalize Runtime`
- Endpoint prefix: `personalize-runtime`
- ARN namespace: `personalize`
- CloudFormation name: `PersonalizeRuntime`
- CloudTrail event source: `personalizeruntime.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetActionRecommendations`, `GetPersonalizedRanking`, `GetRecommendations`.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetActionRecommendations`, `GetPersonalizedRanking`, `GetRecommendations`
- Traits: `idempotent` (3)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetActionRecommendations` | `POST /action-recommendations` | `idempotent` | - | - | `GetActionRecommendationsResponse` | `InvalidInputException`, `ResourceNotFoundException` | Returns a list of recommended actions in sorted in descending order by prediction score. Use the GetActionRecommendations API if you have a custom campaign that deploys a solution version trained with a PERSONALIZED_ ... |
| `GetPersonalizedRanking` | `POST /personalize-ranking` | `idempotent` | `campaignArn`, `inputList`, `userId` | - | `GetPersonalizedRankingResponse` | `InvalidInputException`, `ResourceNotFoundException` | Re-ranks a list of recommended items for the given user. The first item in the list is deemed the most likely item to be of interest to the user. The solution backing the campaign must have been created using a recip ... |
| `GetRecommendations` | `POST /recommendations` | `idempotent` | - | - | `GetRecommendationsResponse` | `InvalidInputException`, `ResourceNotFoundException` | Returns a list of recommended items. For campaigns, the campaign's Amazon Resource Name (ARN) is required and the required user and item input depends on the recipe type used to create the solution backing the campai ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInputException` | `structure` | message | Provide a valid value for the field or parameter. |
| `ResourceNotFoundException` | `structure` | message | The specified resource does not exist. |
| `GetActionRecommendationsRequest` | `structure` | campaignArn, userId, numResults, filterArn, filterValues | - |
| `GetActionRecommendationsResponse` | `structure` | actionList, recommendationId | - |
| `GetPersonalizedRankingRequest` | `structure` | campaignArn, inputList, userId, context, filterArn, filterValues, metadataColumns | - |
| `GetPersonalizedRankingResponse` | `structure` | personalizedRanking, recommendationId | - |
| `GetRecommendationsRequest` | `structure` | campaignArn, itemId, userId, numResults, context, filterArn, filterValues, recommenderArn, promotions, metadataColumns | - |
| `GetRecommendationsResponse` | `structure` | itemList, recommendationId | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

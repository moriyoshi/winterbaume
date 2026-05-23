# Amazon Personalize Events

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Personalize can consume real-time user event data, such as stream or click data, and use it for model training either alone or combined with historical data. For more information see Recording item interaction events.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-personalizeevents/tests/scenario_test.rs`: ingest a batch of user-item interaction events.
- Backported from `scenario_test.rs`: build an action catalogue and record action-interaction events.
- From the AWS documentation and model: model real-time Personalize event ingestion for users, items, actions, sessions, event trackers, recommendation feedback, and batch-style application event pipelines.

## Service Identity and Protocol

- AWS model slug: `personalize-events`
- AWS SDK for Rust slug: `personalizeevents`
- Model version: `2018-03-22`
- Model file: `vendor/api-models-aws/models/personalize-events/service/2018-03-22/personalize-events-2018-03-22.json`
- SDK ID: `Personalize Events`
- Endpoint prefix: `personalize-events`
- ARN namespace: `personalize`
- CloudFormation name: `PersonalizeEvents`
- CloudTrail event source: `personalizeevents.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Put` (5).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutActionInteractions`, `PutActions`, `PutEvents`, `PutItems`, `PutUsers`.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Put

- Operations: `PutActionInteractions`, `PutActions`, `PutEvents`, `PutItems`, `PutUsers`
- Common required input members in this group: `trackingId`, `datasetArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `PutActionInteractions` | `POST /action-interactions` | - | `trackingId`, `actionInteractions` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Records action interaction event data. An action interaction event is an interaction between a user and an action . For example, a user taking an action, such a enrolling in a membership program or downloading your a ... |
| `PutActions` | `POST /actions` | - | `datasetArn`, `actions` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds one or more actions to an Actions dataset. For more information see Importing actions individually . |
| `PutEvents` | `POST /events` | - | `trackingId`, `sessionId`, `eventList` | - | `Unit` | `InvalidInputException` | Records item interaction event data. For more information see Recording item interaction events . |
| `PutItems` | `POST /items` | - | `datasetArn`, `items` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds one or more items to an Items dataset. For more information see Importing items individually . |
| `PutUsers` | `POST /users` | - | `datasetArn`, `users` | - | `Unit` | `InvalidInputException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds one or more users to a Users dataset. For more information see Importing users individually . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInputException` | `structure` | message | Provide a valid value for the field or parameter. |
| `ResourceInUseException` | `structure` | message | The specified resource is in use. |
| `ResourceNotFoundException` | `structure` | message | Could not find the specified resource. |
| `PutActionInteractionsRequest` | `structure` | trackingId, actionInteractions | - |
| `PutActionsRequest` | `structure` | datasetArn, actions | - |
| `PutEventsRequest` | `structure` | trackingId, userId, sessionId, eventList | - |
| `PutItemsRequest` | `structure` | datasetArn, items | - |
| `PutUsersRequest` | `structure` | datasetArn, users | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

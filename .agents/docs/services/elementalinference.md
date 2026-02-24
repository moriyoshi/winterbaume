# AWS Elemental Inference

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the AWS Elemental Inference REST API Reference. It provides information on the URL, request contents, and response contents of each AWS Elemental Inference REST operation. We assume that you have the IAM permissions that you need to use AWS Elemental Inference via the REST API. We also assume that you are familiar with the features and operations of AWS Elemental Inference as described in AWS Elemental Inference User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Elemental Inference where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Elemental Inference by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Elemental Inference workflows in the local mock. Key resources include `FeedResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Associate`, `Create`, `Delete`, `Disassociate` operation families, including `ListFeeds`, `ListTagsForResource`, `AssociateFeed`, `CreateFeed`, `DeleteFeed`, `DisassociateFeed`.

## Service Identity and Protocol

- AWS model slug: `elementalinference`
- AWS SDK for Rust slug: `elementalinference`
- Model version: `2018-11-14`
- Model file: `vendor/api-models-aws/models/elementalinference/service/2018-11-14/elementalinference-2018-11-14.json`
- SDK ID: `ElementalInference`
- Endpoint prefix: `-`
- ARN namespace: `elemental-inference`
- CloudFormation name: `ElementalInference`
- CloudTrail event source: `elemental-inference.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Associate` (1), `Create` (1), `Delete` (1), `Disassociate` (1), `Get` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateFeed`, `CreateFeed`, `DeleteFeed`, `DisassociateFeed`, `TagResource`, `UntagResource`, `UpdateFeed`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetFeed`, `ListFeeds`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `FeedResource` | `id` | create: `CreateFeed`; read: `GetFeed`; update: `UpdateFeed`; delete: `DeleteFeed`; list: `ListFeeds` | `AssociateFeed`, `DisassociateFeed` | - |
## Operation Groups

### List

- Operations: `ListFeeds`, `ListTagsForResource`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `resourceArn`

### Associate

- Operations: `AssociateFeed`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `associatedResourceName`, `id`, `outputs`

### Create

- Operations: `CreateFeed`
- Traits: `idempotent` (1)
- Common required input members in this group: `name`, `outputs`

### Delete

- Operations: `DeleteFeed`
- Traits: `idempotent` (1)
- Common required input members in this group: `id`

### Disassociate

- Operations: `DisassociateFeed`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `associatedResourceName`, `id`

### Get

- Operations: `GetFeed`
- Traits: `readonly` (1)
- Common required input members in this group: `id`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateFeed`
- Traits: `idempotent` (1)
- Common required input members in this group: `id`, `name`, `outputs`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateFeed` | `POST /v1/feed/{id}/associate` | `idempotent`, `idempotency-token` | `associatedResourceName`, `id`, `outputs` | `associatedResourceName` | `AssociateFeedResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestException`, `ValidationException` | Associates a resource with the feed. The resource provides the input that Elemental Inference needs needs in order to perform an Elemental Inference feature, such as cropping video. |
| `CreateFeed` | `POST /v1/feed` | `idempotent` | `name`, `outputs` | - | `CreateFeedResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ServiceQuotaExceededException`, `TooManyRequestException`, `ValidationException` | Creates a feed. The feed is the target for live streams being sent by the calling application. |
| `DeleteFeed` | `DELETE /v1/feed/{id}` | `idempotent` | `id` | - | `DeleteFeedResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException`, `ValidationException` | Deletes the specified feed. The feed can be deleted at any time. |
| `DisassociateFeed` | `POST /v1/feed/{id}/disassociate` | `idempotent`, `idempotency-token` | `associatedResourceName`, `id` | `associatedResourceName` | `DisassociateFeedResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException`, `ValidationException` | Releases the resource (for example, an MediaLive channel) that is associated with this feed. The outputs in the feed become disabled. |
| `GetFeed` | `GET /v1/feed/{id}` | `readonly` | `id` | - | `GetFeedResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException` | Retrieves information about the specified feed. |
| `ListFeeds` | `GET /v1/feeds` | `readonly`, `paginated` | - | - | `ListFeedsResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException`, `ValidationException` | Displays a list of feeds that belong to this AWS account. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException`, `ValidationException` | List all tags that are on an Elemental Inference resource in the current region. |
| `TagResource` | `POST /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException`, `ValidationException` | Associates the specified tags to the resource identified by the specified resourceArn in the current region. If existing tags on a resource are not specified in the request parameters, they are not changed. |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `TooManyRequestException`, `ValidationException` | Deletes specified tags from the specified resource in the current region. |
| `UpdateFeed` | `PUT /v1/feed/{id}` | `idempotent` | `id`, `name`, `outputs` | - | `UpdateFeedResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerErrorException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestException`, `ValidationException` | Updates the name and/or outputs in a feed. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerErrorException` | `structure` | `message` | An internal server error occurred. |
| `TooManyRequestException` | `structure` | `message` | The request was denied due to request throttling. |
| `ResourceNotFoundException` | `structure` | `message` | The resource specified in the action doesn't exist. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by the service. |
| `ConflictException` | `structure` | `message` | The request could not be completed due to a conflict. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request was rejected because it would exceed one or more service quotas for your account. |
| `AssociateFeedRequest` | `structure` | `associatedResourceName`, `dryRun`, `id`, `outputs` | - |
| `AssociateFeedResponse` | `structure` | `arn`, `id` | - |
| `CreateFeedRequest` | `structure` | `name`, `outputs`, `tags` | - |
| `CreateFeedResponse` | `structure` | `arn`, `association`, `dataEndpoints`, `id`, `name`, `outputs`, `status`, `tags` | - |
| `DeleteFeedRequest` | `structure` | `id` | - |
| `DeleteFeedResponse` | `structure` | `arn`, `id`, `status` | - |
| `DisassociateFeedRequest` | `structure` | `associatedResourceName`, `dryRun`, `id` | - |
| `DisassociateFeedResponse` | `structure` | `arn`, `id` | - |
| `GetFeedRequest` | `structure` | `id` | - |
| `GetFeedResponse` | `structure` | `arn`, `association`, `dataEndpoints`, `id`, `name`, `outputs`, `status`, `tags` | - |
| `ListFeedsRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListFeedsResponse` | `structure` | `feeds`, `nextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `TagResourceRequest` | `structure` | `resourceArn`, `tags` | - |
| `UntagResourceRequest` | `structure` | `resourceArn`, `tagKeys` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

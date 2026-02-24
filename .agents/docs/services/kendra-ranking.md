# Amazon Kendra Intelligent Ranking

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Kendra Intelligent Ranking uses Amazon Kendra semantic search capabilities to intelligently re-rank a search service's results.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Kendra Intelligent Ranking workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Rescore` operation families, including `ListRescoreExecutionPlans`, `ListTagsForResource`, `CreateRescoreExecutionPlan`, `DeleteRescoreExecutionPlan`, `DescribeRescoreExecutionPlan`, `Rescore`.

## Service Identity and Protocol

- AWS model slug: `kendra-ranking`
- AWS SDK for Rust slug: `kendraranking`
- Model version: `2022-10-19`
- Model file: `vendor/api-models-aws/models/kendra-ranking/service/2022-10-19/kendra-ranking-2022-10-19.json`
- SDK ID: `Kendra Ranking`
- Endpoint prefix: `kendra-ranking`
- ARN namespace: `kendra-ranking`
- CloudFormation name: `KendraRanking`
- CloudTrail event source: `kendraranking.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Create` (1), `Delete` (1), `Describe` (1), `Rescore` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateRescoreExecutionPlan`, `DeleteRescoreExecutionPlan`, `TagResource`, `UntagResource`, `UpdateRescoreExecutionPlan`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeRescoreExecutionPlan`, `ListRescoreExecutionPlans`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateRescoreExecutionPlan`, `DeleteRescoreExecutionPlan`, `DescribeRescoreExecutionPlan`, `ListRescoreExecutionPlans`, `UpdateRescoreExecutionPlan`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListRescoreExecutionPlans`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: `ResourceARN`

### Create

- Operations: `CreateRescoreExecutionPlan`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Name`

### Delete

- Operations: `DeleteRescoreExecutionPlan`
- Common required input members in this group: `Id`

### Describe

- Operations: `DescribeRescoreExecutionPlan`
- Common required input members in this group: `Id`

### Rescore

- Operations: `Rescore`
- Common required input members in this group: `Documents`, `RescoreExecutionPlanId`, `SearchQuery`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

### Update

- Operations: `UpdateRescoreExecutionPlan`
- Common required input members in this group: `Id`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateRescoreExecutionPlan` | `POST /rescore-execution-plans` | `idempotency-token` | `Name` | `ClientToken` | `CreateRescoreExecutionPlanResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` API. |
| `DeleteRescoreExecutionPlan` | `DELETE /rescore-execution-plans/{Id}` | - | `Id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` API. |
| `DescribeRescoreExecutionPlan` | `GET /rescore-execution-plans/{Id}` | - | `Id` | - | `DescribeRescoreExecutionPlanResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` API. |
| `ListRescoreExecutionPlans` | `GET /rescore-execution-plans` | `paginated` | - | - | `ListRescoreExecutionPlansResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists your rescore execution plans. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` API. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets a list of tags associated with a specified resource. A rescore execution plan is an example of a resource that can have tags associated with it. |
| `Rescore` | `POST /rescore-execution-plans/{RescoreExecutionPlanId}/rescore` | - | `Documents`, `RescoreExecutionPlanId`, `SearchQuery` | - | `RescoreResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rescores or re-ranks search results from a search service such as OpenSearch (self managed). You use the semantic search capabilities of Amazon Kendra Intelligent Ranking to improve the search service's results. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Adds a specified tag to a specified rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` API. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Removes a tag from a rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` operation. |
| `UpdateRescoreExecutionPlan` | `PUT /rescore-execution-plans/{Id}` | - | `Id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the `Rescore` API. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don’t have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | An issue occurred with the internal server used for your Amazon Kendra Intelligent Ranking service. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the constraints set by the Amazon Kendra Intelligent Ranking service. |
| `ConflictException` | `structure` | `Message` | A conflict occurred with the request. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource you want to use doesn't exist. |
| `ResourceUnavailableException` | `structure` | `Message` | The resource you want to use is unavailable. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You have exceeded the set limits for your Amazon Kendra Intelligent Ranking service. |
| `CreateRescoreExecutionPlanRequest` | `structure` | `CapacityUnits`, `ClientToken`, `Description`, `Name`, `Tags` | - |
| `CreateRescoreExecutionPlanResponse` | `structure` | `Arn`, `Id` | - |
| `DeleteRescoreExecutionPlanRequest` | `structure` | `Id` | - |
| `DescribeRescoreExecutionPlanRequest` | `structure` | `Id` | - |
| `DescribeRescoreExecutionPlanResponse` | `structure` | `Arn`, `CapacityUnits`, `CreatedAt`, `Description`, `ErrorMessage`, `Id`, `Name`, `Status`, `UpdatedAt` | - |
| `ListRescoreExecutionPlansRequest` | `structure` | `MaxResults`, `NextToken` | - |
| `ListRescoreExecutionPlansResponse` | `structure` | `NextToken`, `SummaryItems` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceARN` | The request information for listing tags associated with a rescore execution plan. |
| `ListTagsForResourceResponse` | `structure` | `Tags` | If the action is successful, the service sends back an HTTP 200 response. |
| `RescoreRequest` | `structure` | `Documents`, `RescoreExecutionPlanId`, `SearchQuery` | - |
| `RescoreResult` | `structure` | `RescoreId`, `ResultItems` | - |
| `TagResourceRequest` | `structure` | `ResourceARN`, `Tags` | The request information for tagging a rescore execution plan. |
| `TagResourceResponse` | `structure` | - | If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body. |
| `UntagResourceRequest` | `structure` | `ResourceARN`, `TagKeys` | The request information to remove a tag from a rescore execution plan. |
| `UntagResourceResponse` | `structure` | - | If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

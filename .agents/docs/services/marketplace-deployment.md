# AWS Marketplace Deployment Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The AWS Marketplace Deployment Service supports the Quick Launch experience, which is a deployment option for software as a service (SaaS) products. Quick Launch simplifies and reduces the time, resources, and steps required to configure, deploy, and launch a products. The AWS Marketplace Deployment Service provides sellers with a secure method for passing deployment parameters, such as API keys and external IDs, to buyers during the Quick Launch experience.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Marketplace Deployment Service workflows in the local mock. Key resources include `DeploymentParameter`.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `List`, `Put`, `Tag`, `Untag` operation families, including `ListTagsForResource`, `PutDeploymentParameter`, `TagResource`, `UntagResource`.

## Service Identity and Protocol

- AWS model slug: `marketplace-deployment`
- AWS SDK for Rust slug: `marketplacedeployment`
- Model version: `2023-01-25`
- Model file: `vendor/api-models-aws/models/marketplace-deployment/service/2023-01-25/marketplace-deployment-2023-01-25.json`
- SDK ID: `Marketplace Deployment`
- Endpoint prefix: `deployment-marketplace`
- ARN namespace: `aws-marketplace`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (1), `Put` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutDeploymentParameter`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ListTagsForResource`.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DeploymentParameter` | `catalog`, `deploymentParameterId`, `productId` | create: `PutDeploymentParameter` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: `resourceArn`

### Put

- Operations: `PutDeploymentParameter`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `agreementId`, `catalog`, `deploymentParameter`, `productId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags that have been added to a deployment parameter resource. |
| `PutDeploymentParameter` | `POST /catalogs/{catalog}/products/{productId}/deployment-parameters` | `idempotent`, `idempotency-token` | `agreementId`, `catalog`, `deploymentParameter`, `productId` | `clientToken` | `PutDeploymentParameterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates or updates a deployment parameter and is targeted by `catalog` and `agreementId`. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or list of tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | There was an internal service exception. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource wasn't found. |
| `ThrottlingException` | `structure` | `message` | Too many requests. |
| `ValidationException` | `structure` | `fieldName`, `message` | An error occurred during validation. |
| `ConflictException` | `structure` | `message`, `resourceId` | The request configuration has conflicts. |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `PutDeploymentParameterRequest` | `structure` | `agreementId`, `catalog`, `clientToken`, `deploymentParameter`, `expirationDate`, `productId`, `tags` | - |
| `PutDeploymentParameterResponse` | `structure` | `agreementId`, `deploymentParameterId`, `resourceArn`, `tags` | - |
| `ServiceQuotaExceededException` | `structure` | `message` | The maximum number of requests per account has been exceeded. |
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

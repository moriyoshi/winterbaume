# Amazon S3 Vectors

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon S3 vector buckets are a bucket type to store and search vectors with sub-second search times. They are designed to provide dedicated API operations for you to interact with vectors to do similarity search. Within a vector bucket, you use a vector index to organize and logically group your vector data. When you make a write or read request, you direct it to a single vector index. You store your vector data as vectors.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon S3 Vectors workflows in the local mock. Key resources include `IndexResource`, `VectorBucketResource`, `VectorResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Get`, `List`, `Create`, `Put` operation families, including `DeleteIndex`, `DeleteVectorBucket`, `DeleteVectorBucketPolicy`, `DeleteVectors`, `GetIndex`, `GetVectorBucket`.

## Service Identity and Protocol

- AWS model slug: `s3vectors`
- AWS SDK for Rust slug: `s3vectors`
- Model version: `2025-07-15`
- Model file: `vendor/api-models-aws/models/s3vectors/service/2025-07-15/s3vectors-2025-07-15.json`
- SDK ID: `S3Vectors`
- Endpoint prefix: `s3vectors`
- ARN namespace: `s3vectors`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (4), `Get` (4), `List` (4), `Create` (2), `Put` (2), `Query` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateIndex`, `CreateVectorBucket`, `DeleteIndex`, `DeleteVectorBucket`, `DeleteVectorBucketPolicy`, `DeleteVectors`, `PutVectorBucketPolicy`, `PutVectors`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetIndex`, `GetVectorBucket`, `GetVectorBucketPolicy`, `GetVectors`, `ListIndexes`, `ListTagsForResource`, `ListVectorBuckets`, `ListVectors`, `QueryVectors`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `IndexResource` | - | - | `CreateIndex`, `DeleteIndex`, `GetIndex`, `ListIndexes` | - |
| `VectorBucketResource` | - | - | `CreateVectorBucket`, `DeleteVectorBucket`, `DeleteVectorBucketPolicy`, `GetVectorBucket`, `GetVectorBucketPolicy`, `ListVectorBuckets`, `PutVectorBucketPolicy` | - |
| `VectorResource` | - | - | `DeleteVectors`, `GetVectors`, `ListVectors`, `PutVectors`, `QueryVectors` | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `NotFoundException`, `ServiceUnavailableException` | Lists all of the tags applied to a specified Amazon S3 Vectors resource. Each tag is a label consisting of a key and value pair. Tags can help you organize, track costs for, and control access to resources. For a lis ... |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `ConflictException`, `NotFoundException`, `ServiceUnavailableException` | Applies one or more user-defined tags to an Amazon S3 Vectors resource or updates existing tags. Each tag is a label consisting of a key and value pair. Tags can help you organize, track costs for, and control access ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `ConflictException`, `NotFoundException`, `ServiceUnavailableException` | Removes the specified user-defined tags from an Amazon S3 Vectors resource. You can pass one or more tag keys. For a list of S3 resources that support tagging, see Managing tags for Amazon S3 resources . Permissions ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | Access denied. |
| `ConflictException` | `structure` | message | The request failed because a vector bucket name or a vector index name already exists. Vector bucket names must be unique within your Amazon Web Services ac ... |
| `InternalServerException` | `structure` | message | The request failed due to an internal server error. |
| `KmsDisabledException` | `structure` | message | The specified Amazon Web Services KMS key isn't enabled. |
| `KmsInvalidKeyUsageException` | `structure` | message | The request was rejected for one of the following reasons: The KeyUsage value of the KMS key is incompatible with the API operation. The encryption algorith ... |
| `KmsInvalidStateException` | `structure` | message | The key state of the KMS key isn't compatible with the operation. For more information, see KMSInvalidStateException in the Amazon Web Services Key Manageme ... |
| `KmsNotFoundException` | `structure` | message | The KMS key can't be found. |
| `NotFoundException` | `structure` | message | The request was rejected because the specified resource can't be found. |
| `RequestTimeoutException` | `structure` | message | The request timed out. Retry your request. |
| `ServiceQuotaExceededException` | `structure` | message | Your request exceeds a service quota. |
| `ServiceUnavailableException` | `structure` | message | The service is unavailable. Wait briefly and retry your request. If it continues to fail, increase your waiting time between retries. |
| `TooManyRequestsException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, fieldList | The requested action isn't valid. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `DataType` | `enum` | FLOAT32 | - |
| `DistanceMetric` | `enum` | EUCLIDEAN, COSINE | - |
| `SseType` | `enum` | AES256, AWS_KMS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

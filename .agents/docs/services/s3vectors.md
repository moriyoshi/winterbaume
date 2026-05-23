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

### Delete

- Operations: `DeleteIndex`, `DeleteVectorBucket`, `DeleteVectorBucketPolicy`, `DeleteVectors`
- Traits: `idempotent` (2)
- Common required input members in this group: `keys`

### Get

- Operations: `GetIndex`, `GetVectorBucket`, `GetVectorBucketPolicy`, `GetVectors`
- Traits: `readonly` (4)
- Common required input members in this group: `keys`

### List

- Operations: `ListIndexes`, `ListTagsForResource`, `ListVectorBuckets`, `ListVectors`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateIndex`, `CreateVectorBucket`
- Common required input members in this group: `dataType`, `dimension`, `distanceMetric`, `indexName`, `vectorBucketName`

### Put

- Operations: `PutVectorBucketPolicy`, `PutVectors`
- Traits: `idempotent` (2)
- Common required input members in this group: `policy`, `vectors`

### Query

- Operations: `QueryVectors`
- Traits: `readonly` (1)
- Common required input members in this group: `queryVector`, `topK`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateIndex` | `POST /CreateIndex` | - | `dataType`, `dimension`, `distanceMetric`, `indexName` | - | `CreateIndexOutput` | `ConflictException`, `NotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a vector index within a vector bucket. To specify the vector bucket, you must use either the vector bucket name or the vector bucket Amazon Resource Name (ARN). |
| `CreateVectorBucket` | `POST /CreateVectorBucket` | - | `vectorBucketName` | - | `CreateVectorBucketOutput` | `ConflictException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Creates a vector bucket in the Amazon Web Services Region that you want your bucket to be in. Permissions You must have the `s3vectors:CreateVectorBucket` permission to use this operation. |
| `DeleteIndex` | `POST /DeleteIndex` | - | - | - | `DeleteIndexOutput` | `NotFoundException`, `ServiceUnavailableException` | Deletes a vector index. To specify the vector index, you can either use both the vector bucket name and vector index name, or use the vector index Amazon Resource Name (ARN). |
| `DeleteVectorBucket` | `POST /DeleteVectorBucket` | - | - | - | `DeleteVectorBucketOutput` | `ConflictException`, `NotFoundException`, `ServiceUnavailableException` | Deletes a vector bucket. All vector indexes in the vector bucket must be deleted before the vector bucket can be deleted. |
| `DeleteVectorBucketPolicy` | `POST /DeleteVectorBucketPolicy` | `idempotent` | - | - | `DeleteVectorBucketPolicyOutput` | `NotFoundException`, `ServiceUnavailableException` | Deletes a vector bucket policy. To specify the bucket, you must use either the vector bucket name or the vector bucket Amazon Resource Name (ARN). |
| `DeleteVectors` | `POST /DeleteVectors` | `idempotent` | `keys` | - | `DeleteVectorsOutput` | `AccessDeniedException`, `KmsDisabledException`, `KmsInvalidKeyUsageException`, `KmsInvalidStateException`, `KmsNotFoundException`, `NotFoundException`, `ServiceUnavailableException` | Deletes one or more vectors in a vector index. To specify the vector index, you can either use both the vector bucket name and vector index name, or use the vector index Amazon Resource Name (ARN). |
| `GetIndex` | `POST /GetIndex` | `readonly` | - | - | `GetIndexOutput` | `NotFoundException`, `ServiceUnavailableException` | Returns vector index attributes. To specify the vector index, you can either use both the vector bucket name and the vector index name, or use the vector index Amazon Resource Name (ARN). |
| `GetVectorBucket` | `POST /GetVectorBucket` | `readonly` | - | - | `GetVectorBucketOutput` | `NotFoundException`, `ServiceUnavailableException` | Returns vector bucket attributes. To specify the bucket, you must use either the vector bucket name or the vector bucket Amazon Resource Name (ARN). |
| `GetVectorBucketPolicy` | `POST /GetVectorBucketPolicy` | `readonly` | - | - | `GetVectorBucketPolicyOutput` | `NotFoundException`, `ServiceUnavailableException` | Gets details about a vector bucket policy. To specify the bucket, you must use either the vector bucket name or the vector bucket Amazon Resource Name (ARN). |
| `GetVectors` | `POST /GetVectors` | `readonly` | `keys` | - | `GetVectorsOutput` | `KmsDisabledException`, `KmsInvalidKeyUsageException`, `KmsInvalidStateException`, `KmsNotFoundException`, `NotFoundException`, `ServiceUnavailableException` | Returns vector attributes. To specify the vector index, you can either use both the vector bucket name and the vector index name, or use the vector index Amazon Resource Name (ARN). |
| `ListIndexes` | `POST /ListIndexes` | `readonly`, `paginated` | - | - | `ListIndexesOutput` | `NotFoundException`, `ServiceUnavailableException` | Returns a list of all the vector indexes within the specified vector bucket. To specify the bucket, you must use either the vector bucket name or the vector bucket Amazon Resource Name (ARN). |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `NotFoundException`, `ServiceUnavailableException` | Lists all of the tags applied to a specified Amazon S3 Vectors resource. Each tag is a label consisting of a key and value pair. |
| `ListVectorBuckets` | `POST /ListVectorBuckets` | `readonly`, `paginated` | - | - | `ListVectorBucketsOutput` | `ServiceUnavailableException` | Returns a list of all the vector buckets that are owned by the authenticated sender of the request. Permissions You must have the `s3vectors:ListVectorBuckets` permission to use this operation. |
| `ListVectors` | `POST /ListVectors` | `readonly`, `paginated` | - | - | `ListVectorsOutput` | `AccessDeniedException`, `NotFoundException`, `ServiceUnavailableException` | List vectors in the specified vector index. To specify the vector index, you can either use both the vector bucket name and the vector index name, or use the vector index Amazon Resource Name (ARN). |
| `PutVectorBucketPolicy` | `POST /PutVectorBucketPolicy` | `idempotent` | `policy` | - | `PutVectorBucketPolicyOutput` | `NotFoundException`, `ServiceUnavailableException` | Creates a bucket policy for a vector bucket. To specify the bucket, you must use either the vector bucket name or the vector bucket Amazon Resource Name (ARN). |
| `PutVectors` | `POST /PutVectors` | `idempotent` | `vectors` | - | `PutVectorsOutput` | `AccessDeniedException`, `KmsDisabledException`, `KmsInvalidKeyUsageException`, `KmsInvalidStateException`, `KmsNotFoundException`, `NotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException` | Adds one or more vectors to a vector index. To specify the vector index, you can either use both the vector bucket name and the vector index name, or use the vector index Amazon Resource Name (ARN). |
| `QueryVectors` | `POST /QueryVectors` | `readonly` | `queryVector`, `topK` | - | `QueryVectorsOutput` | `KmsDisabledException`, `KmsInvalidKeyUsageException`, `KmsInvalidStateException`, `KmsNotFoundException`, `NotFoundException`, `ServiceUnavailableException` | Performs an approximate nearest neighbor search query in a vector index using a query vector. By default, it returns the keys of approximate nearest neighbors. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `ConflictException`, `NotFoundException`, `ServiceUnavailableException` | Applies one or more user-defined tags to an Amazon S3 Vectors resource or updates existing tags. Each tag is a label consisting of a key and value pair. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `ConflictException`, `NotFoundException`, `ServiceUnavailableException` | Removes the specified user-defined tags from an Amazon S3 Vectors resource. You can pass one or more tag keys. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ServiceUnavailableException` | `structure` | `message` | The service is unavailable. |
| `NotFoundException` | `structure` | `message` | The request was rejected because the specified resource can't be found. |
| `ConflictException` | `structure` | `message` | The request failed because a vector bucket name or a vector index name already exists. |
| `KmsDisabledException` | `structure` | `message` | The specified Amazon Web Services KMS key isn't enabled. |
| `KmsInvalidKeyUsageException` | `structure` | `message` | The request was rejected for one of the following reasons: The `KeyUsage` value of the KMS key is incompatible with the API operation. |
| `KmsInvalidStateException` | `structure` | `message` | The key state of the KMS key isn't compatible with the operation. |
| `KmsNotFoundException` | `structure` | `message` | The KMS key can't be found. |
| `ServiceQuotaExceededException` | `structure` | `message` | Your request exceeds a service quota. |
| `AccessDeniedException` | `structure` | `message` | Access denied. |
| `CreateIndexInput` | `structure` | `dataType`, `dimension`, `distanceMetric`, `encryptionConfiguration`, `indexName`, `metadataConfiguration`, `tags`, `vectorBucketArn`, `vectorBucketName` | - |
| `CreateIndexOutput` | `structure` | `indexArn` | - |
| `CreateVectorBucketInput` | `structure` | `encryptionConfiguration`, `tags`, `vectorBucketName` | - |
| `CreateVectorBucketOutput` | `structure` | `vectorBucketArn` | - |
| `DeleteIndexInput` | `structure` | `indexArn`, `indexName`, `vectorBucketName` | - |
| `DeleteIndexOutput` | `structure` | - | - |
| `DeleteVectorBucketInput` | `structure` | `vectorBucketArn`, `vectorBucketName` | - |
| `DeleteVectorBucketOutput` | `structure` | - | - |
| `DeleteVectorBucketPolicyInput` | `structure` | `vectorBucketArn`, `vectorBucketName` | - |
| `DeleteVectorBucketPolicyOutput` | `structure` | - | - |
| `DeleteVectorsInput` | `structure` | `indexArn`, `indexName`, `keys`, `vectorBucketName` | - |
| `DeleteVectorsOutput` | `structure` | - | - |
| `GetIndexInput` | `structure` | `indexArn`, `indexName`, `vectorBucketName` | - |
| `GetIndexOutput` | `structure` | `index` | - |
| `GetVectorBucketInput` | `structure` | `vectorBucketArn`, `vectorBucketName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

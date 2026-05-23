# Amazon CloudFront KeyValueStore

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudFront KeyValueStore Service to View and Update Data in a KVS Resource

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon CloudFront KeyValueStore workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Describe`, `Get`, `List`, `Put` operation families, including `DeleteKey`, `DescribeKeyValueStore`, `GetKey`, `ListKeys`, `PutKey`, `UpdateKeys`.

## Service Identity and Protocol

- AWS model slug: `cloudfront-keyvaluestore`
- AWS SDK for Rust slug: `cloudfrontkeyvaluestore`
- Model version: `2022-07-26`
- Model file: `vendor/api-models-aws/models/cloudfront-keyvaluestore/service/2022-07-26/cloudfront-keyvaluestore-2022-07-26.json`
- SDK ID: `CloudFront KeyValueStore`
- Endpoint prefix: `cloudfront-keyvaluestore`
- ARN namespace: `key-value-store`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `KvsARN`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Describe` (1), `Get` (1), `List` (1), `Put` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteKey`, `PutKey`, `UpdateKeys`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeKeyValueStore`, `GetKey`, `ListKeys`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Delete

- Operations: `DeleteKey`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Describe

- Operations: `DescribeKeyValueStore`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Get

- Operations: `GetKey`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListKeys`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Put

- Operations: `PutKey`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateKeys`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteKey` | `DELETE /key-value-stores/{KvsARN}/keys/{Key}` | `idempotent` | `KvsARN`, `Key`, `IfMatch` | - | `DeleteKeyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Deletes the key value pair specified by the key. |
| `DescribeKeyValueStore` | `GET /key-value-stores/{KvsARN}` | `readonly` | `KvsARN` | - | `DescribeKeyValueStoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Returns metadata information about Key Value Store. |
| `GetKey` | `GET /key-value-stores/{KvsARN}/keys/{Key}` | `readonly` | `KvsARN`, `Key` | - | `GetKeyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Returns a key value pair. |
| `ListKeys` | `GET /key-value-stores/{KvsARN}/keys` | `readonly`, `paginated` | `KvsARN` | - | `ListKeysResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of key value pairs. |
| `PutKey` | `PUT /key-value-stores/{KvsARN}/keys/{Key}` | `idempotent` | `Key`, `Value`, `KvsARN`, `IfMatch` | - | `PutKeyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new key value pair or replaces the value of an existing key. |
| `UpdateKeys` | `POST /key-value-stores/{KvsARN}/keys` | `idempotent` | `KvsARN`, `IfMatch` | - | `UpdateKeysResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Puts or Deletes multiple key value pairs in a single, all-or-nothing operation. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteKey` | `IfMatch -> If-Match` | - | - | - |
| `ListKeys` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `PutKey` | `IfMatch -> If-Match` | - | - | - |
| `UpdateKeys` | `IfMatch -> If-Match` | - | - | - |

**Conditional-write/read coverage:** the following operations model RFC 7232 conditional headers and therefore must enforce 412 PreconditionFailed (and may emit 409 ConditionalRequestConflict on races) even though those error codes are typically not in the modelled `errors:` list: `DeleteKey`, `PutKey`, `UpdateKeys`.

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Access denied. |
| `ConflictException` | `structure` | Message | Resource is not in expected state. |
| `InternalServerException` | `structure` | Message | Internal server error. |
| `ResourceNotFoundException` | `structure` | Message | Resource was not found. |
| `ServiceQuotaExceededException` | `structure` | Message | Limit exceeded. |
| `ValidationException` | `structure` | Message | Validation failed. |
| `DeleteKeyRequest` | `structure` | KvsARN, Key, IfMatch | - |
| `DeleteKeyResponse` | `structure` | ItemCount, TotalSizeInBytes, ETag | Metadata information about a Key Value Store. |
| `DescribeKeyValueStoreRequest` | `structure` | KvsARN | - |
| `DescribeKeyValueStoreResponse` | `structure` | ItemCount, TotalSizeInBytes, KvsARN, Created, ETag, LastModified, Status, FailureReason | Metadata information about a Key Value Store. |
| `GetKeyRequest` | `structure` | KvsARN, Key | - |
| `GetKeyResponse` | `structure` | Key, Value, ItemCount, TotalSizeInBytes | A key value pair. |
| `ListKeysRequest` | `structure` | KvsARN, NextToken, MaxResults | - |
| `ListKeysResponse` | `structure` | NextToken, Items | - |
| `PutKeyRequest` | `structure` | Key, Value, KvsARN, IfMatch | A key value pair. |
| `PutKeyResponse` | `structure` | ItemCount, TotalSizeInBytes, ETag | Metadata information about a Key Value Store. |
| `UpdateKeysRequest` | `structure` | KvsARN, IfMatch, Puts, Deletes | - |
| `UpdateKeysResponse` | `structure` | ItemCount, TotalSizeInBytes, ETag | Metadata information about a Key Value Store. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

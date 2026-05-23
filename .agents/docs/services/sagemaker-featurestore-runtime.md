# Amazon SageMaker Feature Store Runtime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Contains all data plane API operations and data types for the Amazon SageMaker Feature Store. Use this API to put, delete, and retrieve (get) features from a feature store. Use the following operations to configure your `OnlineStore` and `OfflineStore` features, and to create and manage feature groups: CreateFeatureGroup DeleteFeatureGroup DescribeFeatureGroup ListFeatureGroups

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon SageMaker Feature Store Runtime workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Batch`, `Delete`, `Get`, `Put` operation families, including `BatchGetRecord`, `DeleteRecord`, `GetRecord`, `PutRecord`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-featurestore-runtime`
- AWS SDK for Rust slug: `sagemaker`
- Model version: `2020-07-01`
- Model file: `vendor/api-models-aws/models/sagemaker-featurestore-runtime/service/2020-07-01/sagemaker-featurestore-runtime-2020-07-01.json`
- SDK ID: `SageMaker FeatureStore Runtime`
- Endpoint prefix: `featurestore-runtime.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SageMakerFeatureStoreRuntime`
- CloudTrail event source: `sagemakerfeaturestoreruntime.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Batch` (1), `Delete` (1), `Get` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetRecord`, `DeleteRecord`, `PutRecord`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRecord`.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Batch

- Operations: `BatchGetRecord`
- Common required input members in this group: `Identifiers`

### Delete

- Operations: `DeleteRecord`
- Common required input members in this group: `EventTime`, `FeatureGroupName`, `RecordIdentifierValueAsString`

### Get

- Operations: `GetRecord`
- Common required input members in this group: `FeatureGroupName`, `RecordIdentifierValueAsString`

### Put

- Operations: `PutRecord`
- Common required input members in this group: `FeatureGroupName`, `Record`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetRecord` | `POST /BatchGetRecord` | - | `Identifiers` | - | `BatchGetRecordResponse` | `AccessForbidden`, `InternalFailure`, `ServiceUnavailable`, `ValidationError` | Retrieves a batch of `Records` from a `FeatureGroup`. |
| `DeleteRecord` | `DELETE /FeatureGroup/{FeatureGroupName}` | - | `EventTime`, `FeatureGroupName`, `RecordIdentifierValueAsString` | - | `Unit` | `AccessForbidden`, `InternalFailure`, `ServiceUnavailable`, `ValidationError` | Deletes a `Record` from a `FeatureGroup` in the `OnlineStore`. Feature Store supports both `SoftDelete` and `HardDelete`. |
| `GetRecord` | `GET /FeatureGroup/{FeatureGroupName}` | - | `FeatureGroupName`, `RecordIdentifierValueAsString` | - | `GetRecordResponse` | `AccessForbidden`, `InternalFailure`, `ResourceNotFound`, `ServiceUnavailable`, `ValidationError` | Use for `OnlineStore` serving from a `FeatureStore`. Only the latest records stored in the `OnlineStore` can be retrieved. |
| `PutRecord` | `PUT /FeatureGroup/{FeatureGroupName}` | - | `FeatureGroupName`, `Record` | - | `Unit` | `AccessForbidden`, `InternalFailure`, `ServiceUnavailable`, `ValidationError` | The `PutRecord` API is used to ingest a list of `Records` into your feature group. If a new record’s `EventTime` is greater, the new record is written to both the `OnlineStore` and `OfflineStore`. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteRecord` | - | `RecordIdentifierValueAsString -> RecordIdentifierValueAsString`, `EventTime -> EventTime`, `TargetStores -> TargetStores`, `DeletionMode -> DeletionMode` | - | - |
| `GetRecord` | - | `RecordIdentifierValueAsString -> RecordIdentifierValueAsString`, `FeatureNames -> FeatureName`, `ExpirationTimeResponse -> ExpirationTimeResponse` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessForbidden` | `structure` | `Message` | You do not have permission to perform an action. |
| `InternalFailure` | `structure` | `Message` | An internal failure occurred. |
| `ServiceUnavailable` | `structure` | `Message` | The service is currently unavailable. |
| `ValidationError` | `structure` | `Message` | There was an error validating your request. |
| `BatchGetRecordRequest` | `structure` | `ExpirationTimeResponse`, `Identifiers` | - |
| `BatchGetRecordResponse` | `structure` | `Errors`, `Records`, `UnprocessedIdentifiers` | - |
| `DeleteRecordRequest` | `structure` | `DeletionMode`, `EventTime`, `FeatureGroupName`, `RecordIdentifierValueAsString`, `TargetStores` | - |
| `GetRecordRequest` | `structure` | `ExpirationTimeResponse`, `FeatureGroupName`, `FeatureNames`, `RecordIdentifierValueAsString` | - |
| `GetRecordResponse` | `structure` | `ExpiresAt`, `Record` | - |
| `ResourceNotFound` | `structure` | `Message` | A resource that is required to perform an action was not found. |
| `PutRecordRequest` | `structure` | `FeatureGroupName`, `Record`, `TargetStores`, `TtlDuration` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

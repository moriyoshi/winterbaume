# Amazon S3 Tables

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

An Amazon S3 table represents a structured dataset consisting of tabular data in Apache Parquet format and related metadata. This data is stored inside an S3 table as a subresource. All tables in a table bucket are stored in the Apache Iceberg table format. Through integration with the Amazon Web Services Glue Data Catalog you can interact with your tables using Amazon Web Services analytics services, such as Amazon Athena and Amazon Redshift. Amazon S3 manages maintenance of your tables through automatic file compaction and snapshot management.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon S3 Tables by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon S3 Tables workflows in the local mock. Key resources include `NamespaceResource`, `TableBucketEncryptionResource`, `TableBucketPolicyResource`, `TableBucketReplicationResource`, `TableBucketResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Put`, `Delete`, `List`, `Create` operation families, including `GetNamespace`, `GetTable`, `GetTableBucket`, `GetTableBucketEncryption`, `PutTableBucketEncryption`, `PutTableBucketMaintenanceConfiguration`.

## Service Identity and Protocol

- AWS model slug: `s3tables`
- AWS SDK for Rust slug: `s3tables`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/s3tables/service/2018-05-10/s3tables-2018-05-10.json`
- SDK ID: `S3Tables`
- Endpoint prefix: `-`
- ARN namespace: `s3tables`
- CloudFormation name: `-`
- CloudTrail event source: `s3tables.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (19), `Put` (10), `Delete` (9), `List` (4), `Create` (3), `Rename` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateNamespace`, `CreateTable`, `CreateTableBucket`, `DeleteNamespace`, `DeleteTable`, `DeleteTableBucket`, `DeleteTableBucketEncryption`, `DeleteTableBucketMetricsConfiguration`, `DeleteTableBucketPolicy`, `DeleteTableBucketReplication`, `DeleteTablePolicy`, `DeleteTableReplication`, `PutTableBucketEncryption`, `PutTableBucketMaintenanceConfiguration`, `PutTableBucketMetricsConfiguration`, `PutTableBucketPolicy`, `PutTableBucketReplication`, `PutTableBucketStorageClass`, `PutTableMaintenanceConfiguration`, `PutTablePolicy`, ... (+5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetNamespace`, `GetTable`, `GetTableBucket`, `GetTableBucketEncryption`, `GetTableBucketMaintenanceConfiguration`, `GetTableBucketMetricsConfiguration`, `GetTableBucketPolicy`, `GetTableBucketReplication`, `GetTableBucketStorageClass`, `GetTableEncryption`, `GetTableMaintenanceConfiguration`, `GetTableMaintenanceJobStatus`, `GetTableMetadataLocation`, `GetTablePolicy`, `GetTableRecordExpirationConfiguration`, `GetTableRecordExpirationJobStatus`, `GetTableReplication`, `GetTableReplicationStatus`, `GetTableStorageClass`, `ListNamespaces`, ... (+3).
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 19 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetTableMaintenanceJobStatus`, `GetTableRecordExpirationJobStatus`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 49 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `Glue`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `NamespaceResource` | - | - | `CreateNamespace`, `DeleteNamespace`, `GetNamespace`, `ListNamespaces` | - |
| `TableBucketEncryptionResource` | - | - | `DeleteTableBucketEncryption`, `GetTableBucketEncryption`, `PutTableBucketEncryption` | - |
| `TableBucketPolicyResource` | - | - | `DeleteTableBucketPolicy`, `GetTableBucketPolicy`, `PutTableBucketPolicy` | - |
| `TableBucketReplicationResource` | - | - | `DeleteTableBucketReplication`, `GetTableBucketReplication`, `PutTableBucketReplication` | - |
| `TableBucketResource` | - | - | `CreateTableBucket`, `DeleteTableBucket`, `DeleteTableBucketMetricsConfiguration`, `GetTableBucket`, `GetTableBucketMaintenanceConfiguration`, `GetTableBucketMetricsConfiguration`, `GetTableBucketStorageClass`, `ListTableBuckets`, `PutTableBucketMaintenanceConfiguration`, `PutTableBucketMetricsConfiguration`, ... (+1) | - |
| `TableEncryptionResource` | - | - | `GetTableEncryption` | - |
| `TablePolicyResource` | - | - | `DeleteTablePolicy`, `GetTablePolicy`, `PutTablePolicy` | - |
| `TableReplicationResource` | - | - | `DeleteTableReplication`, `GetTableReplication`, `GetTableReplicationStatus`, `PutTableReplication` | - |
| `TableResource` | - | - | `CreateTable`, `DeleteTable`, `GetTable`, `GetTableMaintenanceConfiguration`, `GetTableMaintenanceJobStatus`, `GetTableMetadataLocation`, `GetTableRecordExpirationConfiguration`, `GetTableRecordExpirationJobStatus`, `GetTableStorageClass`, `ListTables`, ... (+4) | - |
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
| `ListTagsForResource` | `GET /tag/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists all of the tags applied to a specified Amazon S3 Tables resource. Each tag is a label consisting of a key and value pair. Tags can help you organize, track costs for, and control access to resources. For a list ... |
| `TagResource` | `POST /tag/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Applies one or more user-defined tags to an Amazon S3 Tables resource or updates existing tags. Each tag is a label consisting of a key and value pair. Tags can help you organize, track costs for, and control access ... |
| `UntagResource` | `DELETE /tag/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Removes the specified user-defined tags from an Amazon S3 Tables resource. You can pass one or more tag keys. For a list of S3 resources that support tagging, see Managing tags for Amazon S3 resources . Permissions F ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The action cannot be performed because you do not have the required permission. |
| `BadRequestException` | `structure` | message | The request is invalid or malformed. |
| `ConflictException` | `structure` | message | The request failed because there is a conflict with a previous write. You can retry the request. |
| `ForbiddenException` | `structure` | message | The caller isn't authorized to make the request. |
| `InternalServerErrorException` | `structure` | message | The request failed due to an internal server error. |
| `MethodNotAllowedException` | `structure` | message | The requested operation is not allowed on this resource. This may occur when attempting to modify a resource that is managed by a service or has restriction ... |
| `NotFoundException` | `structure` | message | The request was rejected because the specified resource could not be found. |
| `TooManyRequestsException` | `structure` | message | The limit on the number of requests per second was exceeded. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `IcebergCompactionStrategy` | `enum` | AUTO, BINPACK, SORT, ZORDER | - |
| `IcebergNullOrder` | `enum` | NULLS_FIRST, NULLS_LAST | - |
| `IcebergSortDirection` | `enum` | ASC, DESC | - |
| `JobStatus` | `enum` | NOT_YET_RUN, SUCCESSFUL, FAILED, DISABLED | - |
| `MaintenanceStatus` | `enum` | ENABLED, DISABLED | - |
| `OpenTableFormat` | `enum` | ICEBERG | - |
| `ReplicationStatus` | `enum` | PENDING, COMPLETED, FAILED | - |
| `SSEAlgorithm` | `enum` | AES256, AWS_KMS | - |
| `SchemaV2FieldType` | `enum` | STRUCT | - |
| `StorageClass` | `enum` | STANDARD, INTELLIGENT_TIERING | - |
| `TableBucketMaintenanceType` | `enum` | ICEBERG_UNREFERENCED_FILE_REMOVAL | - |
| `TableBucketType` | `enum` | CUSTOMER, AWS | - |
| `TableMaintenanceJobType` | `enum` | ICEBERG_COMPACTION, ICEBERG_SNAPSHOT_MANAGEMENT, ICEBERG_UNREFERENCED_FILE_REMOVAL | - |
| `TableMaintenanceType` | `enum` | ICEBERG_COMPACTION, ICEBERG_SNAPSHOT_MANAGEMENT | - |
| `TableRecordExpirationJobStatus` | `enum` | NOT_YET_RUN, SUCCESSFUL, FAILED, DISABLED | - |
| `TableRecordExpirationStatus` | `enum` | ENABLED, DISABLED | - |
| `TableType` | `enum` | CUSTOMER, AWS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

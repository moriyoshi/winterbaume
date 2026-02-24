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

### Get

- Operations: `GetNamespace`, `GetTable`, `GetTableBucket`, `GetTableBucketEncryption`, `GetTableBucketMaintenanceConfiguration`, `GetTableBucketMetricsConfiguration`, `GetTableBucketPolicy`, `GetTableBucketReplication`, `GetTableBucketStorageClass`, `GetTableEncryption`, `GetTableMaintenanceConfiguration`, `GetTableMaintenanceJobStatus`, `GetTableMetadataLocation`, `GetTablePolicy`, `GetTableRecordExpirationConfiguration`, `GetTableRecordExpirationJobStatus`, `GetTableReplication`, `GetTableReplicationStatus`, `GetTableStorageClass`
- Traits: `readonly` (19)
- Common required input members in this group: `name`, `namespace`, `tableArn`, `tableBucketARN`

### Put

- Operations: `PutTableBucketEncryption`, `PutTableBucketMaintenanceConfiguration`, `PutTableBucketMetricsConfiguration`, `PutTableBucketPolicy`, `PutTableBucketReplication`, `PutTableBucketStorageClass`, `PutTableMaintenanceConfiguration`, `PutTablePolicy`, `PutTableRecordExpirationConfiguration`, `PutTableReplication`
- Traits: `idempotent` (8)
- Common required input members in this group: `configuration`, `encryptionConfiguration`, `name`, `namespace`, `resourcePolicy`, `storageClassConfiguration`, `tableArn`, `tableBucketARN`, `type`, `value`

### Delete

- Operations: `DeleteNamespace`, `DeleteTable`, `DeleteTableBucket`, `DeleteTableBucketEncryption`, `DeleteTableBucketMetricsConfiguration`, `DeleteTableBucketPolicy`, `DeleteTableBucketReplication`, `DeleteTablePolicy`, `DeleteTableReplication`
- Traits: `idempotent` (9)
- Common required input members in this group: `name`, `namespace`, `tableArn`, `tableBucketARN`, `versionToken`

### List

- Operations: `ListNamespaces`, `ListTableBuckets`, `ListTables`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `resourceArn`, `tableBucketARN`

### Create

- Operations: `CreateNamespace`, `CreateTable`, `CreateTableBucket`
- Common required input members in this group: `format`, `name`, `namespace`, `tableBucketARN`

### Rename

- Operations: `RenameTable`
- Common required input members in this group: `name`, `namespace`, `tableBucketARN`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateTableMetadataLocation`
- Common required input members in this group: `metadataLocation`, `name`, `namespace`, `tableBucketARN`, `versionToken`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateNamespace` | `PUT /namespaces/{tableBucketARN}` | - | `namespace`, `tableBucketARN` | - | `CreateNamespaceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a namespace. A namespace is a logical grouping of tables within your table bucket, which you can use to organize tables. |
| `CreateTable` | `PUT /tables/{tableBucketARN}/{namespace}` | - | `format`, `name`, `namespace`, `tableBucketARN` | - | `CreateTableResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a new table associated with the given namespace in a table bucket. For more information, see Creating an Amazon S3 table in the Amazon Simple Storage Service User Guide . |
| `CreateTableBucket` | `PUT /buckets` | - | `name` | - | `CreateTableBucketResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a table bucket. For more information, see Creating a table bucket in the Amazon Simple Storage Service User Guide . |
| `DeleteNamespace` | `DELETE /namespaces/{tableBucketARN}/{namespace}` | `idempotent` | `namespace`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a namespace. For more information, see Delete a namespace in the Amazon Simple Storage Service User Guide . |
| `DeleteTable` | `DELETE /tables/{tableBucketARN}/{namespace}/{name}` | `idempotent` | `name`, `namespace`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a table. For more information, see Deleting an Amazon S3 table in the Amazon Simple Storage Service User Guide . |
| `DeleteTableBucket` | `DELETE /buckets/{tableBucketARN}` | `idempotent` | `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a table bucket. For more information, see Deleting a table bucket in the Amazon Simple Storage Service User Guide . |
| `DeleteTableBucketEncryption` | `DELETE /buckets/{tableBucketARN}/encryption` | `idempotent` | `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the encryption configuration for a table bucket. Permissions You must have the `s3tables:DeleteTableBucketEncryption` permission to use this operation. |
| `DeleteTableBucketMetricsConfiguration` | `DELETE /buckets/{tableBucketARN}/metrics` | `idempotent` | `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the metrics configuration for a table bucket. Permissions You must have the `s3tables:DeleteTableBucketMetricsConfiguration` permission to use this operation. |
| `DeleteTableBucketPolicy` | `DELETE /buckets/{tableBucketARN}/policy` | `idempotent` | `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a table bucket policy. For more information, see Deleting a table bucket policy in the Amazon Simple Storage Service User Guide . |
| `DeleteTableBucketReplication` | `DELETE /table-bucket-replication` | `idempotent` | `tableBucketARN` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the replication configuration for a table bucket. After deletion, new table updates will no longer be replicated to destination buckets, though existing replicated tables will remain in destination buckets. |
| `DeleteTablePolicy` | `DELETE /tables/{tableBucketARN}/{namespace}/{name}/policy` | `idempotent` | `name`, `namespace`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a table policy. For more information, see Deleting a table policy in the Amazon Simple Storage Service User Guide . |
| `DeleteTableReplication` | `DELETE /table-replication` | `idempotent` | `tableArn`, `versionToken` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the replication configuration for a specific table. After deletion, new updates to this table will no longer be replicated to destination tables, though existing replicated copies will remain in destination buckets. |
| `GetNamespace` | `GET /namespaces/{tableBucketARN}/{namespace}` | `readonly` | `namespace`, `tableBucketARN` | - | `GetNamespaceResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a namespace. For more information, see Table namespaces in the Amazon Simple Storage Service User Guide . |
| `GetTable` | `GET /get-table` | `readonly` | - | - | `GetTableResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a table. For more information, see S3 Tables in the Amazon Simple Storage Service User Guide . |
| `GetTableBucket` | `GET /buckets/{tableBucketARN}` | `readonly` | `tableBucketARN` | - | `GetTableBucketResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details on a table bucket. For more information, see Viewing details about an Amazon S3 table bucket in the Amazon Simple Storage Service User Guide . |
| `GetTableBucketEncryption` | `GET /buckets/{tableBucketARN}/encryption` | `readonly` | `tableBucketARN` | - | `GetTableBucketEncryptionResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the encryption configuration for a table bucket. Permissions You must have the `s3tables:GetTableBucketEncryption` permission to use this operation. |
| `GetTableBucketMaintenanceConfiguration` | `GET /buckets/{tableBucketARN}/maintenance` | `readonly` | `tableBucketARN` | - | `GetTableBucketMaintenanceConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a maintenance configuration for a given table bucket. For more information, see Amazon S3 table bucket maintenance in the Amazon Simple Storage Service User Guide . |
| `GetTableBucketMetricsConfiguration` | `GET /buckets/{tableBucketARN}/metrics` | `readonly` | `tableBucketARN` | - | `GetTableBucketMetricsConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the metrics configuration for a table bucket. Permissions You must have the `s3tables:GetTableBucketMetricsConfiguration` permission to use this operation. |
| `GetTableBucketPolicy` | `GET /buckets/{tableBucketARN}/policy` | `readonly` | `tableBucketARN` | - | `GetTableBucketPolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a table bucket policy. For more information, see Viewing a table bucket policy in the Amazon Simple Storage Service User Guide . |
| `GetTableBucketReplication` | `GET /table-bucket-replication` | `readonly` | `tableBucketARN` | - | `GetTableBucketReplicationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the replication configuration for a table bucket.This operation returns the IAM role, `versionToken`, and replication rules that define how tables in this bucket are replicated to other buckets. Permissions You must have the... |
| `GetTableBucketStorageClass` | `GET /buckets/{tableBucketARN}/storage-class` | `readonly` | `tableBucketARN` | - | `GetTableBucketStorageClassResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the storage class configuration for a specific table. This allows you to view the storage class settings that apply to an individual table, which may differ from the table bucket's default configuration. |
| `GetTableEncryption` | `GET /tables/{tableBucketARN}/{namespace}/{name}/encryption` | `readonly` | `name`, `namespace`, `tableBucketARN` | - | `GetTableEncryptionResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the encryption configuration for a table. Permissions You must have the `s3tables:GetTableEncryption` permission to use this operation. |
| `GetTableMaintenanceConfiguration` | `GET /tables/{tableBucketARN}/{namespace}/{name}/maintenance` | `readonly` | `name`, `namespace`, `tableBucketARN` | - | `GetTableMaintenanceConfigurationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about the maintenance configuration of a table. For more information, see S3 Tables maintenance in the Amazon Simple Storage Service User Guide . |
| `GetTableMaintenanceJobStatus` | `GET /tables/{tableBucketARN}/{namespace}/{name}/maintenance-job-status` | `readonly` | `name`, `namespace`, `tableBucketARN` | - | `GetTableMaintenanceJobStatusResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the status of a maintenance job for a table. For more information, see S3 Tables maintenance in the Amazon Simple Storage Service User Guide . |
| `GetTableMetadataLocation` | `GET /tables/{tableBucketARN}/{namespace}/{name}/metadata-location` | `readonly` | `name`, `namespace`, `tableBucketARN` | - | `GetTableMetadataLocationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the location of the table metadata. Permissions You must have the `s3tables:GetTableMetadataLocation` permission to use this operation. |
| `GetTablePolicy` | `GET /tables/{tableBucketARN}/{namespace}/{name}/policy` | `readonly` | `name`, `namespace`, `tableBucketARN` | - | `GetTablePolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a table policy. For more information, see Viewing a table policy in the Amazon Simple Storage Service User Guide . |
| `GetTableRecordExpirationConfiguration` | `GET /table-record-expiration` | `readonly` | `tableArn` | - | `GetTableRecordExpirationConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the expiration configuration settings for records in a table, and the status of the configuration. If the status of the configuration is `enabled`, records expire and are automatically removed from the table after the specified number of days. |
| `GetTableRecordExpirationJobStatus` | `GET /table-record-expiration-job-status` | `readonly` | `tableArn` | - | `GetTableRecordExpirationJobStatusResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the status, metrics, and details of the latest record expiration job for a table. This includes when the job ran, and whether it succeeded or failed. |
| `GetTableReplication` | `GET /table-replication` | `readonly` | `tableArn` | - | `GetTableReplicationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the replication configuration for a specific table. Permissions You must have the `s3tables:GetTableReplication` permission to use this operation. |
| `GetTableReplicationStatus` | `GET /replication-status` | `readonly` | `tableArn` | - | `GetTableReplicationStatusResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the replication status for a table, including the status of replication to each destination. This operation provides visibility into replication health and progress. |
| `GetTableStorageClass` | `GET /tables/{tableBucketARN}/{namespace}/{name}/storage-class` | `readonly` | `name`, `namespace`, `tableBucketARN` | - | `GetTableStorageClassResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the storage class configuration for a specific table. This allows you to view the storage class settings that apply to an individual table, which may differ from the table bucket's default configuration. |
| `ListNamespaces` | `GET /namespaces/{tableBucketARN}` | `readonly`, `paginated` | `tableBucketARN` | - | `ListNamespacesResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists the namespaces within a table bucket. For more information, see Table namespaces in the Amazon Simple Storage Service User Guide . |
| `ListTableBuckets` | `GET /buckets` | `readonly`, `paginated` | - | - | `ListTableBucketsResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists table buckets for your account. For more information, see S3 Table buckets in the Amazon Simple Storage Service User Guide . |
| `ListTables` | `GET /tables/{tableBucketARN}` | `readonly`, `paginated` | `tableBucketARN` | - | `ListTablesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | List tables in the given table bucket. For more information, see S3 Tables in the Amazon Simple Storage Service User Guide . |
| `ListTagsForResource` | `GET /tag/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists all of the tags applied to a specified Amazon S3 Tables resource. Each tag is a label consisting of a key and value pair. |
| `PutTableBucketEncryption` | `PUT /buckets/{tableBucketARN}/encryption` | `idempotent` | `encryptionConfiguration`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Sets the encryption configuration for a table bucket. Permissions You must have the `s3tables:PutTableBucketEncryption` permission to use this operation. |
| `PutTableBucketMaintenanceConfiguration` | `PUT /buckets/{tableBucketARN}/maintenance/{type}` | - | `tableBucketARN`, `type`, `value` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a new maintenance configuration or replaces an existing maintenance configuration for a table bucket. For more information, see Amazon S3 table bucket maintenance in the Amazon Simple Storage Service User Guide . |
| `PutTableBucketMetricsConfiguration` | `PUT /buckets/{tableBucketARN}/metrics` | `idempotent` | `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Sets the metrics configuration for a table bucket. Permissions You must have the `s3tables:PutTableBucketMetricsConfiguration` permission to use this operation. |
| `PutTableBucketPolicy` | `PUT /buckets/{tableBucketARN}/policy` | `idempotent` | `resourcePolicy`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a new table bucket policy or replaces an existing table bucket policy for a table bucket. For more information, see Adding a table bucket policy in the Amazon Simple Storage Service User Guide . |
| `PutTableBucketReplication` | `PUT /table-bucket-replication` | `idempotent` | `configuration`, `tableBucketARN` | - | `PutTableBucketReplicationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates or updates the replication configuration for a table bucket. This operation defines how tables in the source bucket are replicated to destination buckets. |
| `PutTableBucketStorageClass` | `PUT /buckets/{tableBucketARN}/storage-class` | `idempotent` | `storageClassConfiguration`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Sets or updates the storage class configuration for a table bucket. This configuration serves as the default storage class for all new tables created in the bucket, allowing you to optimize storage costs at the bucket level. |
| `PutTableMaintenanceConfiguration` | `PUT /tables/{tableBucketARN}/{namespace}/{name}/maintenance/{type}` | - | `name`, `namespace`, `tableBucketARN`, `type`, `value` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a new maintenance configuration or replaces an existing maintenance configuration for a table. For more information, see S3 Tables maintenance in the Amazon Simple Storage Service User Guide . |
| `PutTablePolicy` | `PUT /tables/{tableBucketARN}/{namespace}/{name}/policy` | `idempotent` | `name`, `namespace`, `resourcePolicy`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a new table policy or replaces an existing table policy for a table. For more information, see Adding a table policy in the Amazon Simple Storage Service User Guide . |
| `PutTableRecordExpirationConfiguration` | `PUT /table-record-expiration` | `idempotent` | `tableArn`, `value` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `TooManyRequestsException` | Creates or updates the expiration configuration settings for records in a table, including the status of the configuration. If you enable record expiration for a table, records expire and are automatically removed from the table after the number of days that... |
| `PutTableReplication` | `PUT /table-replication` | `idempotent` | `configuration`, `tableArn` | - | `PutTableReplicationResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates or updates the replication configuration for a specific table. This operation allows you to define table-level replication independently of bucket-level replication, providing granular control over which tables are replicated and where. |
| `RenameTable` | `PUT /tables/{tableBucketARN}/{namespace}/{name}/rename` | - | `name`, `namespace`, `tableBucketARN` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Renames a table or a namespace. For more information, see S3 Tables in the Amazon Simple Storage Service User Guide . |
| `TagResource` | `POST /tag/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Applies one or more user-defined tags to an Amazon S3 Tables resource or updates existing tags. Each tag is a label consisting of a key and value pair. |
| `UntagResource` | `DELETE /tag/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Removes the specified user-defined tags from an Amazon S3 Tables resource. You can pass one or more tag keys. |
| `UpdateTableMetadataLocation` | `PUT /tables/{tableBucketARN}/{namespace}/{name}/metadata-location` | - | `metadataLocation`, `name`, `namespace`, `tableBucketARN`, `versionToken` | - | `UpdateTableMetadataLocationResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Updates the metadata location for a table. The metadata location of a table must be an S3 URI that begins with the table's warehouse location. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `message` | The request is invalid or malformed. |
| `ForbiddenException` | `structure` | `message` | The caller isn't authorized to make the request. |
| `InternalServerErrorException` | `structure` | `message` | The request failed due to an internal server error. |
| `NotFoundException` | `structure` | `message` | The request was rejected because the specified resource could not be found. |
| `TooManyRequestsException` | `structure` | `message` | The limit on the number of requests per second was exceeded. |
| `ConflictException` | `structure` | `message` | The request failed because there is a conflict with a previous write. |
| `AccessDeniedException` | `structure` | `message` | The action cannot be performed because you do not have the required permission. |
| `MethodNotAllowedException` | `structure` | `message` | The requested operation is not allowed on this resource. |
| `CreateNamespaceRequest` | `structure` | `namespace`, `tableBucketARN` | - |
| `CreateNamespaceResponse` | `structure` | `namespace`, `tableBucketARN` | - |
| `CreateTableRequest` | `structure` | `encryptionConfiguration`, `format`, `metadata`, `name`, `namespace`, `storageClassConfiguration`, `tableBucketARN`, `tags` | - |
| `CreateTableResponse` | `structure` | `tableARN`, `versionToken` | - |
| `CreateTableBucketRequest` | `structure` | `encryptionConfiguration`, `name`, `storageClassConfiguration`, `tags` | - |
| `CreateTableBucketResponse` | `structure` | `arn` | - |
| `DeleteNamespaceRequest` | `structure` | `namespace`, `tableBucketARN` | - |
| `DeleteTableRequest` | `structure` | `name`, `namespace`, `tableBucketARN`, `versionToken` | - |
| `DeleteTableBucketRequest` | `structure` | `tableBucketARN` | - |
| `DeleteTableBucketEncryptionRequest` | `structure` | `tableBucketARN` | - |
| `DeleteTableBucketMetricsConfigurationRequest` | `structure` | `tableBucketARN` | - |
| `DeleteTableBucketPolicyRequest` | `structure` | `tableBucketARN` | - |
| `DeleteTableBucketReplicationRequest` | `structure` | `tableBucketARN`, `versionToken` | - |
| `DeleteTablePolicyRequest` | `structure` | `name`, `namespace`, `tableBucketARN` | - |
| `DeleteTableReplicationRequest` | `structure` | `tableArn`, `versionToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

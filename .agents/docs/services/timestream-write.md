# Amazon Timestream Write

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Timestream Write Amazon Timestream is a fast, scalable, fully managed time-series database service that makes it easy to store and analyze trillions of time-series data points per day. With Timestream, you can easily store and analyze IoT sensor data to derive insights from your IoT applications. You can analyze industrial telemetry to streamline equipment management and maintenance. You can also store and analyze log data and metrics to improve the performance and availability of your applications. Timestream is built from the ground up to effectively ingest, process, and store time-series data.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Timestream Write resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Timestream Write workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Create`, `Delete`, `Update` operation families, including `DescribeBatchLoadTask`, `DescribeDatabase`, `DescribeEndpoints`, `DescribeTable`, `ListBatchLoadTasks`, `ListDatabases`.

## Service Identity and Protocol

- AWS model slug: `timestream-write`
- AWS SDK for Rust slug: `timestreamwrite`
- Model version: `2018-11-01`
- Model file: `vendor/api-models-aws/models/timestream-write/service/2018-11-01/timestream-write-2018-11-01.json`
- SDK ID: `Timestream Write`
- Endpoint prefix: `ingest.timestream`
- ARN namespace: `timestream`
- CloudFormation name: `TimestreamWrite`
- CloudTrail event source: `timestreamwrite.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (4), `List` (4), `Create` (3), `Delete` (2), `Update` (2), `Resume` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBatchLoadTask`, `CreateDatabase`, `CreateTable`, `DeleteDatabase`, `DeleteTable`, `TagResource`, `UntagResource`, `UpdateDatabase`, `UpdateTable`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBatchLoadTask`, `DescribeDatabase`, `DescribeEndpoints`, `DescribeTable`, `ListBatchLoadTasks`, `ListDatabases`, `ListTables`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateBatchLoadTask`, `DescribeBatchLoadTask`, `ListBatchLoadTasks`, `ResumeBatchLoadTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`, `Glue`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeBatchLoadTask`, `DescribeDatabase`, `DescribeEndpoints`, `DescribeTable`
- Common required input members in this group: `DatabaseName`

### List

- Operations: `ListBatchLoadTasks`, `ListDatabases`, `ListTables`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Create

- Operations: `CreateBatchLoadTask`, `CreateDatabase`, `CreateTable`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DatabaseName`

### Delete

- Operations: `DeleteDatabase`, `DeleteTable`
- Common required input members in this group: `DatabaseName`

### Update

- Operations: `UpdateDatabase`, `UpdateTable`
- Common required input members in this group: `DatabaseName`

### Resume

- Operations: `ResumeBatchLoadTask`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Write

- Operations: `WriteRecords`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateBatchLoadTask` | `-` | `idempotency-token` | `DataSourceConfiguration`, `ReportConfiguration`, `TargetDatabaseName`, `TargetTableName` | `ClientToken` | `CreateBatchLoadTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream batch load task. A batch load task processes data from a CSV source in an S3 location and writes to a Timestream table. A mapping from source to target is defined in a batch load task. Errors ... |
| `CreateDatabase` | `-` | - | `DatabaseName` | - | `CreateDatabaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. For more information, see Amazon Web Services managed keys ... |
| `CreateTable` | `-` | - | `DatabaseName`, `TableName` | - | `CreateTableResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a new table to an existing database in your account. In an Amazon Web Services account, table names must be at least unique within each Region if they are in the same database. You might have identical table nam ... |
| `DeleteDatabase` | `-` | - | `DatabaseName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a given Timestream database. This is an irreversible operation. After a database is deleted, the time-series data from its tables cannot be recovered. All tables in the database must be deleted first, or a Va ... |
| `DeleteTable` | `-` | - | `DatabaseName`, `TableName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a given Timestream table. This is an irreversible operation. After a Timestream database table is deleted, the time-series data stored in the table cannot be recovered. Due to the nature of distributed retrie ... |
| `DescribeBatchLoadTask` | `-` | - | `TaskId` | - | `DescribeBatchLoadTaskResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about the batch load task, including configurations, mappings, progress, and other details. Service quotas apply . See code sample for details. |
| `DescribeDatabase` | `-` | - | `DatabaseName` | - | `DescribeDatabaseResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the database, including the database name, time that the database was created, and the total number of tables found within the database. Service quotas apply . See code sample for details. |
| `DescribeEndpoints` | `-` | - | - | - | `DescribeEndpointsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of available endpoints to make Timestream API calls against. This API operation is available through both the Write and Query APIs. Because the Timestream SDKs are designed to transparently work with t ... |
| `DescribeTable` | `-` | - | `DatabaseName`, `TableName` | - | `DescribeTableResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the table, including the table name, database name, retention duration of the memory store and the magnetic store. Service quotas apply . See code sample for details. |
| `ListBatchLoadTasks` | `-` | `paginated` | - | - | `ListBatchLoadTasksResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Provides a list of batch load tasks, along with the name, status, when the task is resumable until, and other details. See code sample for details. |
| `ListDatabases` | `-` | `paginated` | - | - | `ListDatabasesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Returns a list of your Timestream databases. Service quotas apply . See code sample for details. |
| `ListTables` | `-` | `paginated` | - | - | `ListTablesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a list of tables, along with the name, status, and retention properties of each table. See code sample for details. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags on a Timestream resource. |
| `ResumeBatchLoadTask` | `-` | - | `TaskId` | - | `ResumeBatchLoadTaskResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | - |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a set of tags with a Timestream resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes the association of tags from a Timestream resource. |
| `UpdateDatabase` | `-` | - | `DatabaseName`, `KmsKeyId` | - | `UpdateDatabaseResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Modifies the KMS key for an existing database. While updating the database, you must specify the database name and the identifier of the new KMS key to be used ( KmsKeyId ). If there are any concurrent UpdateDatabase ... |
| `UpdateTable` | `-` | - | `DatabaseName`, `TableName` | - | `UpdateTableResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the retention duration of the memory store and magnetic store for your Timestream table. Note that the change in retention duration takes effect immediately. For example, if the retention period of the memor ... |
| `WriteRecords` | `-` | - | `DatabaseName`, `TableName`, `Records` | - | `WriteRecordsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `RejectedRecordsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to write your time-series data into Timestream. You can specify a single data point or a batch of data points to be inserted into the system. Timestream offers you a flexible schema that auto detects the ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You are not authorized to perform this action. |
| `ConflictException` | `structure` | Message | Timestream was unable to process this request because it contains resource that already exists. |
| `InternalServerException` | `structure` | Message | Timestream was unable to fully process this request because of an internal server error. |
| `InvalidEndpointException` | `structure` | Message | The requested endpoint was not valid. |
| `RejectedRecordsException` | `structure` | Message, RejectedRecords | WriteRecords would throw this exception in the following cases: Records with duplicate data where there are multiple records with the same dimensions, times ... |
| `ResourceNotFoundException` | `structure` | Message | The operation tried to access a nonexistent resource. The resource might not be specified correctly, or its status might not be ACTIVE. |
| `ServiceQuotaExceededException` | `structure` | Message | The instance quota of resource exceeded for this account. |
| `ThrottlingException` | `structure` | Message | Too many requests were made by a user and they exceeded the service quotas. The request was throttled. |
| `ValidationException` | `structure` | Message | An invalid or malformed request. |
| `CreateBatchLoadTaskRequest` | `structure` | ClientToken, DataModelConfiguration, DataSourceConfiguration, ReportConfiguration, TargetDatabaseName, TargetTableName, RecordVersion | - |
| `CreateBatchLoadTaskResponse` | `structure` | TaskId | - |
| `CreateDatabaseRequest` | `structure` | DatabaseName, KmsKeyId, Tags | - |
| `CreateDatabaseResponse` | `structure` | Database | - |
| `CreateTableRequest` | `structure` | DatabaseName, TableName, RetentionProperties, Tags, MagneticStoreWriteProperties, Schema | - |
| `CreateTableResponse` | `structure` | Table | - |
| `DeleteDatabaseRequest` | `structure` | DatabaseName | - |
| `DeleteTableRequest` | `structure` | DatabaseName, TableName | - |
| `DescribeBatchLoadTaskRequest` | `structure` | TaskId | - |
| `DescribeBatchLoadTaskResponse` | `structure` | BatchLoadTaskDescription | - |
| `DescribeDatabaseRequest` | `structure` | DatabaseName | - |
| `DescribeDatabaseResponse` | `structure` | Database | - |
| `DescribeEndpointsRequest` | `structure` | **empty (no members)** | - |
| `DescribeEndpointsResponse` | `structure` | Endpoints | - |
| `DescribeTableRequest` | `structure` | DatabaseName, TableName | - |
| `DescribeTableResponse` | `structure` | Table | - |
| `ListBatchLoadTasksRequest` | `structure` | NextToken, MaxResults, TaskStatus | - |
| `ListBatchLoadTasksResponse` | `structure` | NextToken, BatchLoadTasks | - |
| `ListDatabasesRequest` | `structure` | NextToken, MaxResults | - |
| `ListDatabasesResponse` | `structure` | Databases, NextToken | - |
| `ListTablesRequest` | `structure` | DatabaseName, NextToken, MaxResults | - |
| `ListTablesResponse` | `structure` | Tables, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `ResumeBatchLoadTaskRequest` | `structure` | TaskId | - |
| `ResumeBatchLoadTaskResponse` | `structure` | **empty (no members)** | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateDatabaseRequest` | `structure` | DatabaseName, KmsKeyId | - |
| `BatchLoadDataFormat` | `enum` | CSV | - |
| `BatchLoadStatus` | `enum` | CREATED, IN_PROGRESS, FAILED, SUCCEEDED, PROGRESS_STOPPED, PENDING_RESUME | - |
| `DimensionValueType` | `enum` | VARCHAR | - |
| `MeasureValueType` | `enum` | DOUBLE, BIGINT, VARCHAR, BOOLEAN, TIMESTAMP, MULTI | - |
| `PartitionKeyEnforcementLevel` | `enum` | REQUIRED, OPTIONAL | - |
| `PartitionKeyType` | `enum` | DIMENSION, MEASURE | - |
| `S3EncryptionOption` | `enum` | SSE_S3, SSE_KMS | - |
| `ScalarMeasureValueType` | `enum` | DOUBLE, BIGINT, BOOLEAN, VARCHAR, TIMESTAMP | - |
| `TableStatus` | `enum` | ACTIVE, DELETING, RESTORING | - |
| `TimeUnit` | `enum` | MILLISECONDS, SECONDS, MICROSECONDS, NANOSECONDS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

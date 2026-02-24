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
- Common required input members in this group: `DatabaseName`, `TableName`, `TaskId`

### List

- Operations: `ListBatchLoadTasks`, `ListDatabases`, `ListTables`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `ResourceARN`

### Create

- Operations: `CreateBatchLoadTask`, `CreateDatabase`, `CreateTable`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DataSourceConfiguration`, `DatabaseName`, `ReportConfiguration`, `TableName`, `TargetDatabaseName`, `TargetTableName`

### Delete

- Operations: `DeleteDatabase`, `DeleteTable`
- Common required input members in this group: `DatabaseName`, `TableName`

### Update

- Operations: `UpdateDatabase`, `UpdateTable`
- Common required input members in this group: `DatabaseName`, `KmsKeyId`, `TableName`

### Resume

- Operations: `ResumeBatchLoadTask`
- Common required input members in this group: `TaskId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

### Write

- Operations: `WriteRecords`
- Common required input members in this group: `DatabaseName`, `Records`, `TableName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateBatchLoadTask` | - | `idempotency-token` | `DataSourceConfiguration`, `ReportConfiguration`, `TargetDatabaseName`, `TargetTableName` | `ClientToken` | `CreateBatchLoadTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream batch load task. A batch load task processes data from a CSV source in an S3 location and writes to a Timestream table. |
| `CreateDatabase` | - | - | `DatabaseName` | - | `CreateDatabaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. |
| `CreateTable` | - | - | `DatabaseName`, `TableName` | - | `CreateTableResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a new table to an existing database in your account. In an Amazon Web Services account, table names must be at least unique within each Region if they are in the same database. |
| `DeleteDatabase` | - | - | `DatabaseName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a given Timestream database. This is an irreversible operation. |
| `DeleteTable` | - | - | `DatabaseName`, `TableName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a given Timestream table. This is an irreversible operation. |
| `DescribeBatchLoadTask` | - | - | `TaskId` | - | `DescribeBatchLoadTaskResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException` | Returns information about the batch load task, including configurations, mappings, progress, and other details. Service quotas apply. |
| `DescribeDatabase` | - | - | `DatabaseName` | - | `DescribeDatabaseResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the database, including the database name, time that the database was created, and the total number of tables found within the database. Service quotas apply. |
| `DescribeEndpoints` | - | - | - | - | `DescribeEndpointsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of available endpoints to make Timestream API calls against. This API operation is available through both the Write and Query APIs. |
| `DescribeTable` | - | - | `DatabaseName`, `TableName` | - | `DescribeTableResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the table, including the table name, database name, retention duration of the memory store and the magnetic store. Service quotas apply. |
| `ListBatchLoadTasks` | - | `paginated` | - | - | `ListBatchLoadTasksResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Provides a list of batch load tasks, along with the name, status, when the task is resumable until, and other details. See code sample for details. |
| `ListDatabases` | - | `paginated` | - | - | `ListDatabasesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Returns a list of your Timestream databases. Service quotas apply. |
| `ListTables` | - | `paginated` | - | - | `ListTablesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a list of tables, along with the name, status, and retention properties of each table. See code sample for details. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags on a Timestream resource. |
| `ResumeBatchLoadTask` | - | - | `TaskId` | - | `ResumeBatchLoadTaskResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | - |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a set of tags with a Timestream resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Removes the association of tags from a Timestream resource. |
| `UpdateDatabase` | - | - | `DatabaseName`, `KmsKeyId` | - | `UpdateDatabaseResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Modifies the KMS key for an existing database. While updating the database, you must specify the database name and the identifier of the new KMS key to be used (`KmsKeyId`). |
| `UpdateTable` | - | - | `DatabaseName`, `TableName` | - | `UpdateTableResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the retention duration of the memory store and magnetic store for your Timestream table. Note that the change in retention duration takes effect immediately. |
| `WriteRecords` | - | - | `DatabaseName`, `Records`, `TableName` | - | `WriteRecordsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `RejectedRecordsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to write your time-series data into Timestream. You can specify a single data point or a batch of data points to be inserted into the system. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ThrottlingException` | `structure` | `Message` | Too many requests were made by a user and they exceeded the service quotas. |
| `InvalidEndpointException` | `structure` | `Message` | The requested endpoint was not valid. |
| `ValidationException` | `structure` | `Message` | An invalid or malformed request. |
| `InternalServerException` | `structure` | `Message` | Timestream was unable to fully process this request because of an internal server error. |
| `AccessDeniedException` | `structure` | `Message` | You are not authorized to perform this action. |
| `ResourceNotFoundException` | `structure` | `Message` | The operation tried to access a nonexistent resource. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The instance quota of resource exceeded for this account. |
| `ConflictException` | `structure` | `Message` | Timestream was unable to process this request because it contains resource that already exists. |
| `CreateBatchLoadTaskRequest` | `structure` | `ClientToken`, `DataModelConfiguration`, `DataSourceConfiguration`, `RecordVersion`, `ReportConfiguration`, `TargetDatabaseName`, `TargetTableName` | - |
| `CreateBatchLoadTaskResponse` | `structure` | `TaskId` | - |
| `CreateDatabaseRequest` | `structure` | `DatabaseName`, `KmsKeyId`, `Tags` | - |
| `CreateDatabaseResponse` | `structure` | `Database` | - |
| `CreateTableRequest` | `structure` | `DatabaseName`, `MagneticStoreWriteProperties`, `RetentionProperties`, `Schema`, `TableName`, `Tags` | - |
| `CreateTableResponse` | `structure` | `Table` | - |
| `DeleteDatabaseRequest` | `structure` | `DatabaseName` | - |
| `DeleteTableRequest` | `structure` | `DatabaseName`, `TableName` | - |
| `DescribeBatchLoadTaskRequest` | `structure` | `TaskId` | - |
| `DescribeBatchLoadTaskResponse` | `structure` | `BatchLoadTaskDescription` | - |
| `DescribeDatabaseRequest` | `structure` | `DatabaseName` | - |
| `DescribeDatabaseResponse` | `structure` | `Database` | - |
| `DescribeEndpointsRequest` | `structure` | - | - |
| `DescribeEndpointsResponse` | `structure` | `Endpoints` | - |
| `DescribeTableRequest` | `structure` | `DatabaseName`, `TableName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

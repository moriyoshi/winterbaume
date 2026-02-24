# Amazon DynamoDB

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon DynamoDB Amazon DynamoDB is a fully managed NoSQL database service that provides fast and predictable performance with seamless scalability. DynamoDB lets you offload the administrative burdens of operating and scaling a distributed database, so that you don't have to worry about hardware provisioning, setup and configuration, replication, software patching, or cluster scaling. With DynamoDB, you can create database tables that can store and retrieve any amount of data, and serve any level of request traffic. You can scale up or scale down your tables' throughput capacity without downtime or performance degradation, and use the Amazon Web Services Management Console to monitor resource utilization and performance metrics. DynamoDB automatically spreads the data and traffic for your tables over a sufficient number of servers to handle your throughput and storage requirements, while maintaining consistent and fast performance.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-dynamodb/tests/scenario_test.rs`: manage user profile CRUD with optimistic locking through conditional writes.
- Backported from `scenario_test.rs`: build leaderboard and reporting workflows using query, scan, filters, pagination, and a global secondary index.
- Backported from `scenario_test.rs`: process a shopping cart checkout with atomic multi-table writes and stock guards.
- Scenario insight from EC2: exercise account or service defaults for Amazon DynamoDB by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon DynamoDB resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support low-latency key-value and document workloads, conditional expressions, transactions, secondary indexes, paginated reads, item-level update semantics, and table lifecycle configuration.

## Service Identity and Protocol

- AWS model slug: `dynamodb`
- AWS SDK for Rust slug: `dynamodb`
- Model version: `2012-08-10`
- Model file: `vendor/api-models-aws/models/dynamodb/service/2012-08-10/dynamodb-2012-08-10.json`
- SDK ID: `DynamoDB`
- Endpoint prefix: `dynamodb`
- ARN namespace: `dynamodb`
- CloudFormation name: `DynamoDB`
- CloudTrail event source: `dynamodb.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `AccountId`, `AccountIdEndpointMode`, `Endpoint`, `Region`, `ResourceArn`, `ResourceArnList`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (13), `Update` (9), `List` (7), `Delete` (4), `Batch` (3), `Create` (3), `Execute` (2), `Get` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchExecuteStatement`, `BatchGetItem`, `BatchWriteItem`, `CreateBackup`, `CreateGlobalTable`, `CreateTable`, `DeleteBackup`, `DeleteItem`, `DeleteResourcePolicy`, `DeleteTable`, `DisableKinesisStreamingDestination`, `EnableKinesisStreamingDestination`, `ImportTable`, `PutItem`, `PutResourcePolicy`, `RestoreTableFromBackup`, `RestoreTableToPointInTime`, `TagResource`, `UntagResource`, `UpdateContinuousBackups`, ... (+8).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBackup`, `DescribeContinuousBackups`, `DescribeContributorInsights`, `DescribeEndpoints`, `DescribeExport`, `DescribeGlobalTable`, `DescribeGlobalTableSettings`, `DescribeImport`, `DescribeKinesisStreamingDestination`, `DescribeLimits`, `DescribeTable`, `DescribeTableReplicaAutoScaling`, `DescribeTimeToLive`, `GetItem`, `GetResourcePolicy`, `ListBackups`, `ListContributorInsights`, `ListExports`, `ListGlobalTables`, `ListImports`, ... (+2).
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeExport`, `DescribeImport`, `ExportTableToPointInTime`, `ImportTable`, `ListExports`, `ListImports`, `Scan`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 56 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SQS`, `Glue`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/transaction-apis.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadConsistency.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.html

Research outcomes:
- TransactWriteItems is synchronous, idempotent when a client token is supplied, and all-or-nothing across up to 100 actions, 100 distinct items, and 4 MB aggregate item size, within one account and Region.
- Transaction actions cannot target the same item more than once in the same request.
- TransactWriteItems differs from BatchWriteItem because a batch can partially succeed, whereas a transaction either commits all changes or none.
- Transaction client tokens are valid for 10 minutes after request completion. Reusing a token with different request parameters during that window returns IdempotentParameterMismatch.
- DynamoDB transaction isolation is serializable between transactional operations and single-item GetItem/PutItem/UpdateItem/DeleteItem operations, but BatchGetItem, BatchWriteItem, Query, and Scan have weaker operation-level isolation.
- Default reads are eventually consistent. Strongly consistent reads are available for tables and local secondary indexes through ConsistentRead, but not for global secondary indexes or streams.
- Transaction conflicts have distinct failure modes: single item write conflicts can fail with TransactionConflictException, while transaction-level conflicts fail with TransactionCanceledException.

Parity implications:
- Keep transaction state changes atomic and validate same-item duplication before mutation.
- Track idempotency tokens with a time window and request-parameter fingerprint.
- Preserve BatchWriteItem partial-success behaviour separately from transaction all-or-nothing behaviour.
- Treat ConsistentRead, index type, and stream/global-table reads as observable read-consistency controls, even if the emulator ultimately models consistency deterministically.

## Operation Groups

### Describe

- Operations: `DescribeBackup`, `DescribeContinuousBackups`, `DescribeContributorInsights`, `DescribeEndpoints`, `DescribeExport`, `DescribeGlobalTable`, `DescribeGlobalTableSettings`, `DescribeImport`, `DescribeKinesisStreamingDestination`, `DescribeLimits`, `DescribeTable`, `DescribeTableReplicaAutoScaling`, `DescribeTimeToLive`
- Common required input members in this group: `BackupArn`, `ExportArn`, `GlobalTableName`, `ImportArn`, `TableName`

### Update

- Operations: `UpdateContinuousBackups`, `UpdateContributorInsights`, `UpdateGlobalTable`, `UpdateGlobalTableSettings`, `UpdateItem`, `UpdateKinesisStreamingDestination`, `UpdateTable`, `UpdateTableReplicaAutoScaling`, `UpdateTimeToLive`
- Common required input members in this group: `ContributorInsightsAction`, `GlobalTableName`, `Key`, `PointInTimeRecoverySpecification`, `ReplicaUpdates`, `StreamArn`, `TableName`, `TimeToLiveSpecification`

### List

- Operations: `ListBackups`, `ListContributorInsights`, `ListExports`, `ListGlobalTables`, `ListImports`, `ListTables`, `ListTagsOfResource`
- Traits: `paginated` (4)
- Common required input members in this group: `ResourceArn`

### Delete

- Operations: `DeleteBackup`, `DeleteItem`, `DeleteResourcePolicy`, `DeleteTable`
- Common required input members in this group: `BackupArn`, `Key`, `ResourceArn`, `TableName`

### Batch

- Operations: `BatchExecuteStatement`, `BatchGetItem`, `BatchWriteItem`
- Common required input members in this group: `RequestItems`, `Statements`

### Create

- Operations: `CreateBackup`, `CreateGlobalTable`, `CreateTable`
- Common required input members in this group: `BackupName`, `GlobalTableName`, `ReplicationGroup`, `TableName`

### Execute

- Operations: `ExecuteStatement`, `ExecuteTransaction`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Statement`, `TransactStatements`

### Get

- Operations: `GetItem`, `GetResourcePolicy`
- Common required input members in this group: `Key`, `ResourceArn`, `TableName`

### Put

- Operations: `PutItem`, `PutResourcePolicy`
- Common required input members in this group: `Item`, `Policy`, `ResourceArn`, `TableName`

### Restore

- Operations: `RestoreTableFromBackup`, `RestoreTableToPointInTime`
- Common required input members in this group: `BackupArn`, `TargetTableName`

### Transact

- Operations: `TransactGetItems`, `TransactWriteItems`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `TransactItems`

### Disable

- Operations: `DisableKinesisStreamingDestination`
- Common required input members in this group: `StreamArn`, `TableName`

### Enable

- Operations: `EnableKinesisStreamingDestination`
- Common required input members in this group: `StreamArn`, `TableName`

### Export

- Operations: `ExportTableToPointInTime`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `S3Bucket`, `TableArn`

### Import

- Operations: `ImportTable`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `InputFormat`, `S3BucketSource`, `TableCreationParameters`

### Query

- Operations: `Query`
- Traits: `paginated` (1)
- Common required input members in this group: `TableName`

### Scan

- Operations: `Scan`
- Traits: `paginated` (1)
- Common required input members in this group: `TableName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchExecuteStatement` | - | - | `Statements` | - | `BatchExecuteStatementOutput` | `InternalServerError`, `RequestLimitExceeded`, `ThrottlingException` | This operation allows you to perform batch reads or writes on data stored in DynamoDB, using PartiQL. Each read statement in a `BatchExecuteStatement` must specify an equality condition on all key attributes. |
| `BatchGetItem` | - | - | `RequestItems` | - | `BatchGetItemOutput` | `InternalServerError`, `InvalidEndpointException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException` | The `BatchGetItem` operation returns the attributes of one or more items from one or more tables. You identify requested items by primary key. |
| `BatchWriteItem` | - | - | `RequestItems` | - | `BatchWriteItemOutput` | `InternalServerError`, `InvalidEndpointException`, `ItemCollectionSizeLimitExceededException`, `ProvisionedThroughputExceededException`, `ReplicatedWriteConflictException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException` | The `BatchWriteItem` operation puts or deletes multiple items in one or more tables. A single call to `BatchWriteItem` can transmit up to 16MB of data over the network, consisting of up to 25 item put or delete operations. |
| `CreateBackup` | - | - | `BackupName`, `TableName` | - | `CreateBackupOutput` | `BackupInUseException`, `ContinuousBackupsUnavailableException`, `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `TableInUseException`, `TableNotFoundException` | Creates a backup for an existing table. Each time you create an on-demand backup, the entire table data is backed up. |
| `CreateGlobalTable` | - | - | `GlobalTableName`, `ReplicationGroup` | - | `CreateGlobalTableOutput` | `GlobalTableAlreadyExistsException`, `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `TableNotFoundException` | Creates a global table from an existing table. A global table creates a replication relationship between two or more DynamoDB tables with the same table name in the provided Regions. |
| `CreateTable` | - | - | `TableName` | - | `CreateTableOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException` | The `CreateTable` operation adds a new table to your account. In an Amazon Web Services account, table names must be unique within each Region. |
| `DeleteBackup` | - | - | `BackupArn` | - | `DeleteBackupOutput` | `BackupInUseException`, `BackupNotFoundException`, `InternalServerError`, `InvalidEndpointException`, `LimitExceededException` | Deletes an existing backup of a table. You can call `DeleteBackup` at a maximum rate of 10 times per second. |
| `DeleteItem` | - | - | `Key`, `TableName` | - | `DeleteItemOutput` | `ConditionalCheckFailedException`, `InternalServerError`, `InvalidEndpointException`, `ItemCollectionSizeLimitExceededException`, `ProvisionedThroughputExceededException`, `ReplicatedWriteConflictException`, `RequestLimitExceeded`, `ResourceNotFoundException`, ... (+2) | Deletes a single item in a table by primary key. You can perform a conditional delete operation that deletes the item if it exists, or if it has an expected attribute value. |
| `DeleteResourcePolicy` | - | - | `ResourceArn` | - | `DeleteResourcePolicyOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `PolicyNotFoundException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the resource-based policy attached to the resource, which can be a table or stream. `DeleteResourcePolicy` is an idempotent operation; running it multiple times on the same resource doesn't result in an error response, unless you specify an... |
| `DeleteTable` | - | - | `TableName` | - | `DeleteTableOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | The `DeleteTable` operation deletes a table and all of its items. After a `DeleteTable` request, the specified table is in the `DELETING` state until DynamoDB completes the deletion. |
| `DescribeBackup` | - | - | `BackupArn` | - | `DescribeBackupOutput` | `BackupNotFoundException`, `InternalServerError`, `InvalidEndpointException` | Describes an existing backup of a table. You can call `DescribeBackup` at a maximum rate of 10 times per second. |
| `DescribeContinuousBackups` | - | - | `TableName` | - | `DescribeContinuousBackupsOutput` | `InternalServerError`, `InvalidEndpointException`, `TableNotFoundException` | Checks the status of continuous backups and point in time recovery on the specified table. Continuous backups are `ENABLED` on all tables at table creation. |
| `DescribeContributorInsights` | - | - | `TableName` | - | `DescribeContributorInsightsOutput` | `InternalServerError`, `ResourceNotFoundException` | Returns information about contributor insights for a given table or global secondary index. |
| `DescribeEndpoints` | - | - | - | - | `DescribeEndpointsResponse` | - | Returns the regional endpoint information. For more information on policy permissions, please see Internetwork traffic privacy. |
| `DescribeExport` | - | - | `ExportArn` | - | `DescribeExportOutput` | `ExportNotFoundException`, `InternalServerError`, `LimitExceededException` | Describes an existing table export. |
| `DescribeGlobalTable` | - | - | `GlobalTableName` | - | `DescribeGlobalTableOutput` | `GlobalTableNotFoundException`, `InternalServerError`, `InvalidEndpointException` | Returns information about the specified global table. This documentation is for version 2017.11.29 (Legacy) of global tables, which should be avoided for new global tables. |
| `DescribeGlobalTableSettings` | - | - | `GlobalTableName` | - | `DescribeGlobalTableSettingsOutput` | `GlobalTableNotFoundException`, `InternalServerError`, `InvalidEndpointException` | Describes Region-specific settings for a global table. This documentation is for version 2017.11.29 (Legacy) of global tables, which should be avoided for new global tables. |
| `DescribeImport` | - | - | `ImportArn` | - | `DescribeImportOutput` | `ImportNotFoundException` | Represents the properties of the import. |
| `DescribeKinesisStreamingDestination` | - | - | `TableName` | - | `DescribeKinesisStreamingDestinationOutput` | `InternalServerError`, `InvalidEndpointException`, `ResourceNotFoundException` | Returns information about the status of Kinesis streaming. |
| `DescribeLimits` | - | - | - | - | `DescribeLimitsOutput` | `InternalServerError`, `InvalidEndpointException` | Returns the current provisioned-capacity quotas for your Amazon Web Services account in a Region, both for the Region as a whole and for any one DynamoDB table that you create there. When you establish an Amazon Web Services account, the account has initial... |
| `DescribeTable` | - | - | `TableName` | - | `DescribeTableOutput` | `InternalServerError`, `InvalidEndpointException`, `ResourceNotFoundException` | Returns information about the table, including the current status of the table, when it was created, the primary key schema, and any indexes on the table. If you issue a `DescribeTable` request immediately after a `CreateTable` request, DynamoDB might return... |
| `DescribeTableReplicaAutoScaling` | - | - | `TableName` | - | `DescribeTableReplicaAutoScalingOutput` | `InternalServerError`, `ResourceNotFoundException` | Describes auto scaling settings across replicas of the global table at once. |
| `DescribeTimeToLive` | - | - | `TableName` | - | `DescribeTimeToLiveOutput` | `InternalServerError`, `InvalidEndpointException`, `ResourceNotFoundException` | Gives a description of the Time to Live (TTL) status on the specified table. |
| `DisableKinesisStreamingDestination` | - | - | `StreamArn`, `TableName` | - | `KinesisStreamingDestinationOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Stops replication from the DynamoDB table to the Kinesis data stream. This is done without deleting either of the resources. |
| `EnableKinesisStreamingDestination` | - | - | `StreamArn`, `TableName` | - | `KinesisStreamingDestinationOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Starts table data replication to the specified Kinesis data stream at a timestamp chosen during the enable workflow. If this operation doesn't return results immediately, use DescribeKinesisStreamingDestination to check if streaming to the Kinesis data stream... |
| `ExecuteStatement` | - | - | `Statement` | - | `ExecuteStatementOutput` | `ConditionalCheckFailedException`, `DuplicateItemException`, `InternalServerError`, `ItemCollectionSizeLimitExceededException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException`, ... (+1) | This operation allows you to perform reads and singleton writes on data stored in DynamoDB, using PartiQL. For PartiQL reads (`SELECT` statement), if the total number of processed items exceeds the maximum dataset size limit of 1 MB, the read stops and... |
| `ExecuteTransaction` | - | `idempotency-token` | `TransactStatements` | `ClientRequestToken` | `ExecuteTransactionOutput` | `IdempotentParameterMismatchException`, `InternalServerError`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException`, `TransactionCanceledException`, `TransactionInProgressException` | This operation allows you to perform transactional reads or writes on data stored in DynamoDB, using PartiQL. The entire transaction must consist of either read statements or write statements, you cannot mix both in one transaction. |
| `ExportTableToPointInTime` | - | `idempotency-token` | `S3Bucket`, `TableArn` | `ClientToken` | `ExportTableToPointInTimeOutput` | `ExportConflictException`, `InternalServerError`, `InvalidExportTimeException`, `LimitExceededException`, `PointInTimeRecoveryUnavailableException`, `TableNotFoundException` | Exports table data to an S3 bucket. The table must have point in time recovery enabled, and you can export data from any time within the point in time recovery window. |
| `GetItem` | - | - | `Key`, `TableName` | - | `GetItemOutput` | `InternalServerError`, `InvalidEndpointException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException` | The `GetItem` operation returns a set of attributes for the item with the given primary key. If there is no matching item, `GetItem` does not return any data and there will be no `Item` element in the response. |
| `GetResourcePolicy` | - | - | `ResourceArn` | - | `GetResourcePolicyOutput` | `InternalServerError`, `InvalidEndpointException`, `PolicyNotFoundException`, `ResourceNotFoundException` | Returns the resource-based policy document attached to the resource, which can be a table or stream, in JSON format. `GetResourcePolicy` follows an eventually consistent model. |
| `ImportTable` | - | `idempotency-token` | `InputFormat`, `S3BucketSource`, `TableCreationParameters` | `ClientToken` | `ImportTableOutput` | `ImportConflictException`, `LimitExceededException`, `ResourceInUseException` | Imports table data from an S3 bucket. |
| `ListBackups` | - | - | - | - | `ListBackupsOutput` | `InternalServerError`, `InvalidEndpointException` | List DynamoDB backups that are associated with an Amazon Web Services account and weren't made with Amazon Web Services Backup. To list these backups for a given table, specify `TableName`. |
| `ListContributorInsights` | - | `paginated` | - | - | `ListContributorInsightsOutput` | `InternalServerError`, `ResourceNotFoundException` | Returns a list of ContributorInsightsSummary for a table and all its global secondary indexes. |
| `ListExports` | - | `paginated` | - | - | `ListExportsOutput` | `InternalServerError`, `LimitExceededException` | Lists completed exports within the past 90 days, in reverse alphanumeric order of `ExportArn`. |
| `ListGlobalTables` | - | - | - | - | `ListGlobalTablesOutput` | `InternalServerError`, `InvalidEndpointException` | Lists all global tables that have a replica in the specified Region. This documentation is for version 2017.11.29 (Legacy) of global tables, which should be avoided for new global tables. |
| `ListImports` | - | `paginated` | - | - | `ListImportsOutput` | `LimitExceededException` | Lists completed imports within the past 90 days. |
| `ListTables` | - | `paginated` | - | - | `ListTablesOutput` | `InternalServerError`, `InvalidEndpointException` | Returns an array of table names associated with the current account and endpoint. The output from `ListTables` is paginated, with each page returning a maximum of 100 table names. |
| `ListTagsOfResource` | - | - | `ResourceArn` | - | `ListTagsOfResourceOutput` | `InternalServerError`, `InvalidEndpointException`, `ResourceNotFoundException` | List all tags on an Amazon DynamoDB resource. You can call ListTagsOfResource up to 10 times per second, per account. |
| `PutItem` | - | - | `Item`, `TableName` | - | `PutItemOutput` | `ConditionalCheckFailedException`, `InternalServerError`, `InvalidEndpointException`, `ItemCollectionSizeLimitExceededException`, `ProvisionedThroughputExceededException`, `ReplicatedWriteConflictException`, `RequestLimitExceeded`, `ResourceNotFoundException`, ... (+2) | Creates a new item, or replaces an old item with a new item. If an item that has the same primary key as the new item already exists in the specified table, the new item completely replaces the existing item. |
| `PutResourcePolicy` | - | - | `Policy`, `ResourceArn` | - | `PutResourcePolicyOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `PolicyNotFoundException`, `ResourceInUseException`, `ResourceNotFoundException` | Attaches a resource-based policy document to the resource, which can be a table or stream. When you attach a resource-based policy using this API, the policy application is eventually consistent . |
| `Query` | - | `paginated` | `TableName` | - | `QueryOutput` | `InternalServerError`, `InvalidEndpointException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException` | You must provide the name of the partition key attribute and a single value for that attribute. `Query` returns all items with that partition key value. |
| `RestoreTableFromBackup` | - | - | `BackupArn`, `TargetTableName` | - | `RestoreTableFromBackupOutput` | `BackupInUseException`, `BackupNotFoundException`, `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `TableAlreadyExistsException`, `TableInUseException` | Creates a new table from an existing backup. Any number of users can execute up to 50 concurrent restores (any type of restore) in a given account. |
| `RestoreTableToPointInTime` | - | - | `TargetTableName` | - | `RestoreTableToPointInTimeOutput` | `InternalServerError`, `InvalidEndpointException`, `InvalidRestoreTimeException`, `LimitExceededException`, `PointInTimeRecoveryUnavailableException`, `TableAlreadyExistsException`, `TableInUseException`, `TableNotFoundException` | Restores the specified table to the specified point in time within `EarliestRestorableDateTime` and `LatestRestorableDateTime`. You can restore your table to any point in time in the last 35 days. |
| `Scan` | - | `paginated` | `TableName` | - | `ScanOutput` | `InternalServerError`, `InvalidEndpointException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException` | The `Scan` operation returns one or more items and item attributes by accessing every item in a table or a secondary index. To have DynamoDB return fewer items, you can provide a `FilterExpression` operation. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `Unit` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Associate a set of tags with an Amazon DynamoDB resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. |
| `TransactGetItems` | - | - | `TransactItems` | - | `TransactGetItemsOutput` | `InternalServerError`, `InvalidEndpointException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException`, `TransactionCanceledException` | `TransactGetItems` is a synchronous operation that atomically retrieves multiple items from one or more tables (but not from indexes) in a single account and Region. A `TransactGetItems` call can contain up to 100 `TransactGetItem` objects, each of which... |
| `TransactWriteItems` | - | `idempotency-token` | `TransactItems` | `ClientRequestToken` | `TransactWriteItemsOutput` | `IdempotentParameterMismatchException`, `InternalServerError`, `InvalidEndpointException`, `ProvisionedThroughputExceededException`, `RequestLimitExceeded`, `ResourceNotFoundException`, `ThrottlingException`, `TransactionCanceledException`, ... (+1) | `TransactWriteItems` is a synchronous write operation that groups up to 100 action requests. These actions can target items in different tables, but not in different Amazon Web Services accounts or Regions, and no two actions can target the same item. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `Unit` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Removes the association of tags from an Amazon DynamoDB resource. You can call `UntagResource` up to five times per second, per account. |
| `UpdateContinuousBackups` | - | - | `PointInTimeRecoverySpecification`, `TableName` | - | `UpdateContinuousBackupsOutput` | `ContinuousBackupsUnavailableException`, `InternalServerError`, `InvalidEndpointException`, `TableNotFoundException` | `UpdateContinuousBackups` enables or disables point in time recovery for the specified table. A successful `UpdateContinuousBackups` call returns the current `ContinuousBackupsDescription`. |
| `UpdateContributorInsights` | - | - | `ContributorInsightsAction`, `TableName` | - | `UpdateContributorInsightsOutput` | `InternalServerError`, `ResourceNotFoundException` | Updates the status for contributor insights for a specific table or index. CloudWatch Contributor Insights for DynamoDB graphs display the partition key and (if applicable) sort key of frequently accessed items and frequently throttled items in plaintext. |
| `UpdateGlobalTable` | - | - | `GlobalTableName`, `ReplicaUpdates` | - | `UpdateGlobalTableOutput` | `GlobalTableNotFoundException`, `InternalServerError`, `InvalidEndpointException`, `ReplicaAlreadyExistsException`, `ReplicaNotFoundException`, `TableNotFoundException` | Adds or removes replicas in the specified global table. The global table must already exist to be able to use this operation. |
| `UpdateGlobalTableSettings` | - | - | `GlobalTableName` | - | `UpdateGlobalTableSettingsOutput` | `GlobalTableNotFoundException`, `IndexNotFoundException`, `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ReplicaNotFoundException`, `ResourceInUseException` | Updates settings for a global table. This documentation is for version 2017.11.29 (Legacy) of global tables, which should be avoided for new global tables. |
| `UpdateItem` | - | - | `Key`, `TableName` | - | `UpdateItemOutput` | `ConditionalCheckFailedException`, `InternalServerError`, `InvalidEndpointException`, `ItemCollectionSizeLimitExceededException`, `ProvisionedThroughputExceededException`, `ReplicatedWriteConflictException`, `RequestLimitExceeded`, `ResourceNotFoundException`, ... (+2) | Edits an existing item's attributes, or adds a new item to the table if it does not already exist. You can put, delete, or add attribute values. |
| `UpdateKinesisStreamingDestination` | - | - | `StreamArn`, `TableName` | - | `UpdateKinesisStreamingDestinationOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | The command to update the Kinesis stream destination. |
| `UpdateTable` | - | - | `TableName` | - | `UpdateTableOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Modifies the provisioned throughput settings, global secondary indexes, or DynamoDB Streams settings for a given table. You can only perform one of the following operations at once: Modify the provisioned throughput settings of the table. |
| `UpdateTableReplicaAutoScaling` | - | - | `TableName` | - | `UpdateTableReplicaAutoScalingOutput` | `InternalServerError`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates auto scaling settings on your global tables at once. |
| `UpdateTimeToLive` | - | - | `TableName`, `TimeToLiveSpecification` | - | `UpdateTimeToLiveOutput` | `InternalServerError`, `InvalidEndpointException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | The `UpdateTimeToLive` method enables or disables Time to Live (TTL) for the specified table. A successful `UpdateTimeToLive` call returns the current `TimeToLiveSpecification`. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerError` | `structure` | `message` | An error occurred on the server side. |
| `InvalidEndpointException` | `structure` | `Message` | - |
| `ResourceNotFoundException` | `structure` | `message` | The operation tried to access a nonexistent table or index. |
| `LimitExceededException` | `structure` | `message` | There is no limit to the number of daily on-demand backups that can be taken. |
| `ResourceInUseException` | `structure` | `message` | The operation conflicts with the resource's availability. |
| `RequestLimitExceeded` | `structure` | `ThrottlingReasons`, `message` | Throughput exceeds the current throughput quota for your account. |
| `ThrottlingException` | `structure` | `message`, `throttlingReasons` | The request was denied due to request throttling. |
| `ProvisionedThroughputExceededException` | `structure` | `ThrottlingReasons`, `message` | The request was denied due to request throttling. |
| `TableNotFoundException` | `structure` | `message` | A source table with the name `TableName` does not currently exist within the subscriber's account or the subscriber is operating in the wrong Amazon Web Services Region. |
| `ItemCollectionSizeLimitExceededException` | `structure` | `message` | An item collection is too large. |
| `ReplicatedWriteConflictException` | `structure` | `message` | The request was rejected because one or more items in the request are being modified by a request in another Region. |
| `ConditionalCheckFailedException` | `structure` | `Item`, `message` | A condition specified in the operation failed to be evaluated. |
| `TransactionConflictException` | `structure` | `message` | Operation was rejected because there is an ongoing transaction for the item. |
| `GlobalTableNotFoundException` | `structure` | `message` | The specified global table does not exist. |
| `BackupInUseException` | `structure` | `message` | There is another ongoing conflicting backup control plane operation on the table. |
| `TableInUseException` | `structure` | `message` | A target table with the specified name is either being created or deleted. |
| `BackupNotFoundException` | `structure` | `message` | Backup not found for the given BackupARN. |
| `PolicyNotFoundException` | `structure` | `message` | The operation tried to access a nonexistent resource-based policy. |
| `TransactionCanceledException` | `structure` | `CancellationReasons`, `Message` | The entire transaction request was canceled. |
| `ContinuousBackupsUnavailableException` | `structure` | `message` | Backups have not yet been enabled for this table. |
| `KinesisStreamingDestinationInput` | `structure` | `EnableKinesisStreamingConfiguration`, `StreamArn`, `TableName` | - |
| `KinesisStreamingDestinationOutput` | `structure` | `DestinationStatus`, `EnableKinesisStreamingConfiguration`, `StreamArn`, `TableName` | - |
| `IdempotentParameterMismatchException` | `structure` | `Message` | DynamoDB rejected the request because you retried a request with a different payload but with an idempotent token that was already used. |
| `TransactionInProgressException` | `structure` | `Message` | The transaction with the given request token is already in progress. |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/dynamodb-partiql-integration.md, .agents/docs/LTM/pluggable-backends-and-query-execution-synthesis.md, .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### State, Backends, and Views

- DynamoDB stores items as a typed `AttributeValue` enum in `crates/winterbaume-dynamodb/src/types.rs`, not as opaque JSON. Preserve the standard DynamoDB JSON wire shape through serde, but keep state and execution logic type-aware.
- DynamoDB secondary-index support is part of the backend contract: `create_table()` carries GSI and LSI definitions, `query()` accepts `index_name: Option<String>`, and custom backends must persist index definitions with backward-compatible defaults.
- `TableStateView` should keep secondary indexes typed through `SecondaryIndexView` rather than `Vec<serde_json::Value>` blobs so snapshots, restores, Redis persistence, and Terraform converter injection share one semantic model.
- Backend ownership matters for state fidelity. Snapshot, restore, and merge paths must route through the authoritative backend once alternative persistence is enabled; avoid hidden in-memory shadows.
- Any backend trait expansion must update `winterbaume-dynamodb-redis` in the same pass, with serde defaults for newly persisted fields.

### PartiQL

- PartiQL execution stays above the backend trait and decomposes into backend primitives through `execute_partiql_via_backend(...)`; do not add PartiQL itself as a storage-backend method.
- The `winterbaume-partiql` parser is hand-rolled in `crates/winterbaume-partiql/src/parser/`. Do not reintroduce `partiql-parser` / `partiql-ast`; the old lalrpop build script was the reason for the rewrite.
- The current IR is expression-based: `Expression` covers literals, paths, binary `+` / `-`, and unary negation; `Condition::Compare(Expression, CmpOp, Expression)` replaces the old comparison-specific variants.
- Parameter placeholders (`?`) are substituted at parse time at value positions, not through textual pre-processing.
- Runtime validation belongs in the DynamoDB handler/execution layer, where table schema and AWS-specific restrictions are known. The parser may represent broader syntax than DynamoDB accepts.

### AWS-Fidelity Rules

- `EXISTS(SELECT ...)` is a top-level `DdbOperation::Exists(Box<SelectOp>)`, not a `Condition` variant. AWS rejects EXISTS in SELECT projection and WHERE positions.
- `EXISTS` is accepted only in `ExecuteTransaction` write requests. `ExecuteStatement` and `BatchExecuteStatement` reject it with `ValidationException`; the inner SELECT must specify the full primary key by equality plus at least one non-key predicate.
- AWS rejects arithmetic in WHERE operands and rejects unary `-path`. Keep the parser expressive, but ensure DynamoDB runtime validation rejects those shapes with AWS-compatible errors.
- AWS accepts more SET arithmetic than the docs imply, including `path + path`, `literal + path`, chained arithmetic, parenthesised arithmetic, and negative literal operands. Preserve those SET RHS paths.
- `IS NULL` and `IS MISSING` are distinct: NULL matches `{"NULL": true}` attributes only, while MISSING matches absent attributes.
- `contains(path, val)` is overloaded for string substring matching, string/number/binary set membership, and list element equality.
- `attribute_type(path, 'TYPE')` accepts exactly DynamoDB's documented type names: `S`, `N`, `B`, `BOOL`, `NULL`, `SS`, `NS`, `BS`, `L`, and `M`.

### Streams and Cross-Service Boundaries

- DynamoDB write paths append stream change records for `PutItem`, `UpdateItem`, and `DeleteItem` when table streams are enabled. Records carry event name, sequence number, keys, old image, and new image.
- DynamoDB Streams is a derived service. Table metadata and stream payloads originate in DynamoDB; iterator state, positions, and delivery bookkeeping belong in `winterbaume-dynamodbstreams`.
- Step Functions' optimised DynamoDB integration is intentionally narrow: `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem`. Do not model it as general DynamoDB support.
- DynamoDB Streams to Lambda and DynamoDB Streams to EventBridge Pipes are high-value future integration seams; tests should seed tables and writes through DynamoDB rather than fabricating stream-only state.

### Files and Tests

- Core files: `crates/winterbaume-dynamodb/src/types.rs`, `backend.rs`, `partiql_exec.rs`, `views.rs`, and `state.rs`.
- PartiQL files: `crates/winterbaume-partiql/src/operation.rs`, `parser/lexer.rs`, `parser/expr.rs`, `parser/stmt.rs`, `parser/mod.rs`, and parser tests.
- Related services/backends: `crates/winterbaume-dynamodb-redis/src/lib.rs` and `crates/winterbaume-dynamodbstreams/src/handlers.rs`.
- Focused checks for DynamoDB LTM-sensitive work:
  ```bash
  .agents/bin/cargo.sh test -p winterbaume-partiql
  .agents/bin/cargo.sh test -p winterbaume-dynamodb --test integration_test -- execute_statement
  .agents/bin/cargo.sh test -p winterbaume-dynamodb --test integration_test -- partiql_fn
  .agents/bin/cargo.sh test -p winterbaume-dynamodbstreams
  .agents/bin/cargo.sh test -p winterbaume-dynamodb-redis
  ```

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

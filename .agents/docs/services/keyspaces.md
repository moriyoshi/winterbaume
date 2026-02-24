# Amazon Keyspaces

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Keyspaces (for Apache Cassandra) is a scalable, highly available, and managed Apache Cassandra-compatible database service. Amazon Keyspaces makes it easy to migrate, run, and scale Cassandra workloads in the Amazon Web Services Cloud. With just a few clicks on the Amazon Web Services Management Console or a few lines of code, you can create keyspaces and tables in Amazon Keyspaces, without deploying any infrastructure or installing software. In addition to supporting Cassandra Query Language (CQL) requests via open-source Cassandra drivers, Amazon Keyspaces supports data definition language (DDL) operations to manage keyspaces and tables using the Amazon Web Services SDK and CLI, as well as infrastructure as code (IaC) services and tools such as CloudFormation and Terraform. This API reference describes the supported DDL operations in detail.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-keyspaces/tests/scenario_test.rs`: create a keyspace, create a table with schema, inspect/update/list, and delete it.
- Backported from `scenario_test.rs`: tag and untag Keyspaces resources and verify tag state.
- Backported from `scenario_test.rs`: manage a user-defined type lifecycle inside a keyspace.
- From the AWS documentation and model: support Cassandra-compatible schema management, keyspaces, tables, user-defined types, capacity and encryption settings, restore/PITR-style operations, and tag-based resource organisation.

## Service Identity and Protocol

- AWS model slug: `keyspaces`
- AWS SDK for Rust slug: `keyspaces`
- Model version: `2022-02-10`
- Model file: `vendor/api-models-aws/models/keyspaces/service/2022-02-10/keyspaces-2022-02-10.json`
- SDK ID: `Keyspaces`
- Endpoint prefix: `cassandra`
- ARN namespace: `cassandra`
- CloudFormation name: `Cassandra`
- CloudTrail event source: `cassandra.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (4), `Create` (3), `Delete` (3), `Update` (2), `Restore` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateKeyspace`, `CreateTable`, `CreateType`, `DeleteKeyspace`, `DeleteTable`, `DeleteType`, `RestoreTable`, `TagResource`, `UntagResource`, `UpdateKeyspace`, `UpdateTable`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetKeyspace`, `GetTable`, `GetTableAutoScalingSettings`, `GetType`, `ListKeyspaces`, `ListTables`, `ListTagsForResource`, `ListTypes`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/keyspaces/latest/devguide/what-is-keyspaces.html
- https://docs.aws.amazon.com/keyspaces/latest/devguide/functional-differences.html
- https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.ddl.table.html

Research outcomes:
- Amazon Keyspaces is a managed serverless Cassandra-compatible database service.
- Keyspaces and tables are created and modified through CQL and AWS APIs, but DDL operations are asynchronous.
- Tables can configure partition keys, clustering columns, capacity mode, encryption, point-in-time recovery, TTL, CDC streams, and auto scaling.
- Keyspaces differs from Apache Cassandra in cluster management, authentication, authorisation, pagination, system tables, timestamps, range delete, UDTs, and other behaviours.
- TTL automatically deletes expired data and is billed according to deleted data size.
- Restore table creates a new table from backup/PITR state rather than overwriting the existing table.
- Lightweight transactions and batch behaviour have service-specific constraints.

Parity implications:
- Model keyspaces, tables, schema, asynchronous DDL status, capacity settings, PITR, TTL, CDC, encryption, and restore operations separately.
- CQL compatibility should include documented Keyspaces differences rather than assuming open-source Cassandra semantics.
- Delete and TTL expiry should be delayed/data-plane aware where possible.

## Operation Groups

### Get

- Operations: `GetKeyspace`, `GetTable`, `GetTableAutoScalingSettings`, `GetType`
- Common required input members in this group: `keyspaceName`, `tableName`, `typeName`

### List

- Operations: `ListKeyspaces`, `ListTables`, `ListTagsForResource`, `ListTypes`
- Traits: `paginated` (4)
- Common required input members in this group: `keyspaceName`, `resourceArn`

### Create

- Operations: `CreateKeyspace`, `CreateTable`, `CreateType`
- Common required input members in this group: `fieldDefinitions`, `keyspaceName`, `schemaDefinition`, `tableName`, `typeName`

### Delete

- Operations: `DeleteKeyspace`, `DeleteTable`, `DeleteType`
- Common required input members in this group: `keyspaceName`, `tableName`, `typeName`

### Update

- Operations: `UpdateKeyspace`, `UpdateTable`
- Common required input members in this group: `keyspaceName`, `replicationSpecification`, `tableName`

### Restore

- Operations: `RestoreTable`
- Common required input members in this group: `sourceKeyspaceName`, `sourceTableName`, `targetKeyspaceName`, `targetTableName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tags`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateKeyspace` | - | - | `keyspaceName` | - | `CreateKeyspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | The `CreateKeyspace` operation adds a new keyspace to your account. In an Amazon Web Services account, keyspace names must be unique within each Region. |
| `CreateTable` | - | - | `keyspaceName`, `schemaDefinition`, `tableName` | - | `CreateTableResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `CreateTable` operation adds a new table to the specified keyspace. Within a keyspace, table names must be unique. |
| `CreateType` | - | - | `fieldDefinitions`, `keyspaceName`, `typeName` | - | `CreateTypeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `CreateType` operation creates a new user-defined type in the specified keyspace. To configure the required permissions, see Permissions to create a UDT in the Amazon Keyspaces Developer Guide . |
| `DeleteKeyspace` | - | - | `keyspaceName` | - | `DeleteKeyspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `DeleteKeyspace` operation deletes a keyspace and all of its tables. |
| `DeleteTable` | - | - | `keyspaceName`, `tableName` | - | `DeleteTableResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `DeleteTable` operation deletes a table and all of its data. After a `DeleteTable` request is received, the specified table is in the `DELETING` state until Amazon Keyspaces completes the deletion. |
| `DeleteType` | - | - | `keyspaceName`, `typeName` | - | `DeleteTypeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `DeleteType` operation deletes a user-defined type (UDT). You can only delete a type that is not used in a table or another UDT. |
| `GetKeyspace` | - | - | `keyspaceName` | - | `GetKeyspaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns the name of the specified keyspace, the Amazon Resource Name (ARN), the replication strategy, the Amazon Web Services Regions of a multi-Region keyspace, and the status of newly added Regions after an `UpdateKeyspace` operation. |
| `GetTable` | - | - | `keyspaceName`, `tableName` | - | `GetTableResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns information about the table, including the table's name and current status, the keyspace name, configuration settings, and metadata. To read table metadata using `GetTable`, the IAM principal needs `Select` action permissions for the table and the... |
| `GetTableAutoScalingSettings` | - | - | `keyspaceName`, `tableName` | - | `GetTableAutoScalingSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns auto scaling related settings of the specified table in JSON format. If the table is a multi-Region table, the Amazon Web Services Region specific auto scaling settings of the table are included. |
| `GetType` | - | - | `keyspaceName`, `typeName` | - | `GetTypeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `GetType` operation returns information about the type, for example the field definitions, the timestamp when the type was last modified, the level of nesting, the status, and details about if the type is used in other types and tables. To read keyspace... |
| `ListKeyspaces` | - | `paginated` | - | - | `ListKeyspacesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `ListKeyspaces` operation returns a list of keyspaces. |
| `ListTables` | - | `paginated` | `keyspaceName` | - | `ListTablesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `ListTables` operation returns a list of tables for a specified keyspace. To read keyspace metadata using `ListTables`, the IAM principal needs `Select` action permissions for the system keyspace. |
| `ListTagsForResource` | - | `paginated` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns a list of all tags associated with the specified Amazon Keyspaces resource. To read keyspace metadata using `ListTagsForResource`, the IAM principal needs `Select` action permissions for the specified resource and the system keyspace. |
| `ListTypes` | - | `paginated` | `keyspaceName` | - | `ListTypesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | The `ListTypes` operation returns a list of types for a specified keyspace. To read keyspace metadata using `ListTypes`, the IAM principal needs `Select` action permissions for the system keyspace. |
| `RestoreTable` | - | - | `sourceKeyspaceName`, `sourceTableName`, `targetKeyspaceName`, `targetTableName` | - | `RestoreTableResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Restores the table to the specified point in time within the `earliest_restorable_timestamp` and the current time. For more information about restore points, see Time window for PITR continuous backups in the Amazon Keyspaces Developer Guide . |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Associates a set of tags with a Amazon Keyspaces resource. You can then activate these user-defined tags so that they appear on the Cost Management Console for cost allocation tracking. |
| `UntagResource` | - | - | `resourceArn`, `tags` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Removes the association of tags from a Amazon Keyspaces resource. |
| `UpdateKeyspace` | - | - | `keyspaceName`, `replicationSpecification` | - | `UpdateKeyspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds a new Amazon Web Services Region to the keyspace. You can add a new Region to a keyspace that is either a single or a multi-Region keyspace. |
| `UpdateTable` | - | - | `keyspaceName`, `tableName` | - | `UpdateTableResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Adds new columns to the table or updates one of the table's settings, for example capacity mode, auto scaling, encryption, point-in-time recovery, or ttl settings. Note that you can only update one specific table setting per update operation. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access permissions to perform this action. |
| `InternalServerException` | `structure` | `message` | Amazon Keyspaces was unable to fully process this request because of an internal server error. |
| `ServiceQuotaExceededException` | `structure` | `message` | The operation exceeded the service quota for this resource. |
| `ValidationException` | `structure` | `message` | The operation failed due to an invalid or malformed request. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceArn` | The operation tried to access a keyspace, table, or type that doesn't exist. |
| `ConflictException` | `structure` | `message` | Amazon Keyspaces couldn't complete the requested action. |
| `CreateKeyspaceRequest` | `structure` | `keyspaceName`, `replicationSpecification`, `tags` | - |
| `CreateKeyspaceResponse` | `structure` | `resourceArn` | - |
| `CreateTableRequest` | `structure` | `autoScalingSpecification`, `capacitySpecification`, `cdcSpecification`, `clientSideTimestamps`, `comment`, `defaultTimeToLive`, `encryptionSpecification`, `keyspaceName`, `pointInTimeRecovery`, `replicaSpecifications`, `schemaDefinition`, `tableName`, ... (+3) | - |
| `CreateTableResponse` | `structure` | `resourceArn` | - |
| `CreateTypeRequest` | `structure` | `fieldDefinitions`, `keyspaceName`, `typeName` | - |
| `CreateTypeResponse` | `structure` | `keyspaceArn`, `typeName` | - |
| `DeleteKeyspaceRequest` | `structure` | `keyspaceName` | - |
| `DeleteKeyspaceResponse` | `structure` | - | - |
| `DeleteTableRequest` | `structure` | `keyspaceName`, `tableName` | - |
| `DeleteTableResponse` | `structure` | - | - |
| `DeleteTypeRequest` | `structure` | `keyspaceName`, `typeName` | - |
| `DeleteTypeResponse` | `structure` | `keyspaceArn`, `typeName` | - |
| `GetKeyspaceRequest` | `structure` | `keyspaceName` | - |
| `GetKeyspaceResponse` | `structure` | `keyspaceName`, `replicationGroupStatuses`, `replicationRegions`, `replicationStrategy`, `resourceArn` | - |
| `GetTableRequest` | `structure` | `keyspaceName`, `tableName` | - |
| `GetTableResponse` | `structure` | `capacitySpecification`, `cdcSpecification`, `clientSideTimestamps`, `comment`, `creationTimestamp`, `defaultTimeToLive`, `encryptionSpecification`, `keyspaceName`, `latestStreamArn`, `pointInTimeRecovery`, `replicaSpecifications`, `resourceArn`, ... (+5) | - |
| `GetTableAutoScalingSettingsRequest` | `structure` | `keyspaceName`, `tableName` | - |
| `GetTableAutoScalingSettingsResponse` | `structure` | `autoScalingSpecification`, `keyspaceName`, `replicaSpecifications`, `resourceArn`, `tableName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

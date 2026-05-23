# Redshift Data API Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Amazon Redshift Data API to run queries on Amazon Redshift tables. You can run SQL statements, which are committed if the statement succeeds. For more information about the Amazon Redshift Data API and CLI usage examples, see Using the Amazon Redshift Data API in the Amazon Redshift Management Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Redshift Data API Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Describe`, `Get`, `Batch`, `Cancel` operation families, including `ListDatabases`, `ListSchemas`, `ListStatements`, `ListTables`, `DescribeStatement`, `DescribeTable`.

## Service Identity and Protocol

- AWS model slug: `redshift-data`
- AWS SDK for Rust slug: `redshiftdata`
- Model version: `2019-12-20`
- Model file: `vendor/api-models-aws/models/redshift-data/service/2019-12-20/redshift-data-2019-12-20.json`
- SDK ID: `Redshift Data`
- Endpoint prefix: `-`
- ARN namespace: `redshift-data`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Describe` (2), `Get` (2), `Batch` (1), `Cancel` (1), `Execute` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchExecuteStatement`, `CancelStatement`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeStatement`, `DescribeTable`, `GetStatementResult`, `GetStatementResultV2`, `ListDatabases`, `ListSchemas`, `ListStatements`, `ListTables`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelStatement`.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`, `ECS`, `Redshift`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/redshift/latest/mgmt/data-api.html

Research outcomes:
- The Redshift Data API runs SQL against provisioned Redshift clusters and Redshift Serverless workgroups without persistent client connections.
- SQL execution is asynchronous. `ExecuteStatement` and `BatchExecuteStatement` return a statement id, and later operations describe, cancel, or fetch the statement.
- A query can run for at most 24 hours. Query results are retained for at most 24 hours and can be at most 500 MB after gzip compression.
- A cluster can have up to 500 active Data API queries in `STARTED` or `SUBMITTED` state.
- `BatchExecuteStatement` executes its `Sqls` array serially as one transaction. If any statement fails, all work is rolled back.
- `ExecuteStatement` and `BatchExecuteStatement` support a `ClientToken` idempotency token retained for up to eight hours.
- SDKs automatically generate client tokens for retries; callers using Step Functions should persist a token across retries.
- Session reuse uses `SessionKeepAliveSeconds` and `SessionId`. The maximum keepalive and maximum session duration are 24 hours.
- A session can run only one query at a time. The Data API does not queue queries for a provided session.
- JSON result format is fetched with `GetStatementResult`; CSV result format is fetched with `GetStatementResultV2`. CSV results are returned in chunks with pagination.
- By default, users with the same IAM role as the statement runner can act on that statement. Cross-user access requires assuming the runner's IAM role or using IAM conditions for owner/session access.

Parity implications:
- Model statements, batch statements, status transitions, result formats, result pagination, idempotency tokens, sessions, cancellation, ownership, and authentication mode separately.
- Execute and batch calls should be asynchronous and idempotent within the client-token retention window.
- Session reuse should enforce one in-flight query per session, idle timeout changes, maximum lifetime, and no per-session queueing.

## Control-Plane / Data-Plane Coherence

- **Paired with `redshift` ( and `redshiftserverless` ).** This data plane executes SQL against Redshift provisioned clusters and Redshift Serverless workgroups that the Redshift control planes create and manage. `ExecuteStatement` requires either a `clusterIdentifier` ( provisioned ) or a `workgroupName` ( serverless ); in real AWS the call fails with `ValidationException` if neither resolves to an existing target.
- **Current Winterbaume status: deliberately separate.** This crate carries its own catalogue ( `databases`, `schemas`, `table_names`, `table_columns` ) and a statement record set, with no dependency on `winterbaume-redshift` or `winterbaume-redshiftserverless`. It does not validate that the `clusterIdentifier` exists. This crate plus the SQL execution backend ( `winterbaume-query-engine` family ) is what the data plane uses, separately.
- **What needs to change ( low priority ):** if a future workflow needs the data plane to validate cluster/workgroup existence and authentication mode, this crate should observe `winterbaume-redshift`'s `clusters` and `winterbaume-redshiftserverless`'s `workgroups`. The catalogue itself stays here ( Redshift's control plane does not expose a schema-catalog API ).

## Operation Groups

### List

- Operations: `ListDatabases`, `ListSchemas`, `ListStatements`, `ListTables`
- Traits: `paginated` (4), `readonly` (4)
- Common required input members in this group: `Database`

### Describe

- Operations: `DescribeStatement`, `DescribeTable`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `Database`, `Id`

### Get

- Operations: `GetStatementResult`, `GetStatementResultV2`
- Traits: `paginated` (2), `readonly` (2)
- Common required input members in this group: `Id`

### Batch

- Operations: `BatchExecuteStatement`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Sqls`

### Cancel

- Operations: `CancelStatement`
- Common required input members in this group: `Id`

### Execute

- Operations: `ExecuteStatement`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Sql`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchExecuteStatement` | - | `idempotency-token` | `Sqls` | `ClientToken` | `BatchExecuteStatementOutput` | `ActiveSessionsExceededException`, `ActiveStatementsExceededException`, `BatchExecuteStatementException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Runs one or more SQL statements, which can be data manipulation language (DML) or data definition language (DDL). Depending on the authorization method, use one of the following combinations of request parameters: Secrets Manager - when connecting to a... |
| `CancelStatement` | - | - | `Id` | - | `CancelStatementResponse` | `DatabaseConnectionException`, `InternalServerException`, `QueryTimeoutException`, `ResourceNotFoundException`, `ValidationException` | Cancels a running query. To be canceled, a query must be running. |
| `DescribeStatement` | - | `readonly` | `Id` | - | `DescribeStatementResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the details about a specific instance when a query was run by the Amazon Redshift Data API. The information includes when the query started, when it finished, the query status, the number of rows returned, and the SQL statement. |
| `DescribeTable` | - | `readonly`, `paginated` | `Database` | - | `DescribeTableResponse` | `DatabaseConnectionException`, `InternalServerException`, `QueryTimeoutException`, `ResourceNotFoundException`, `ValidationException` | Describes the detailed information about a table from metadata in the cluster. The information includes its columns. |
| `ExecuteStatement` | - | `idempotency-token` | `Sql` | `ClientToken` | `ExecuteStatementOutput` | `ActiveSessionsExceededException`, `ActiveStatementsExceededException`, `ExecuteStatementException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Runs an SQL statement, which can be data manipulation language (DML) or data definition language (DDL). This statement must be a single SQL statement. |
| `GetStatementResult` | - | `readonly`, `paginated` | `Id` | - | `GetStatementResultResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Fetches the temporarily cached result of an SQL statement in JSON format. The `ExecuteStatement` or `BatchExecuteStatement` operation that ran the SQL statement must have specified `ResultFormat` as `JSON` , or let the format default to JSON. |
| `GetStatementResultV2` | - | `readonly`, `paginated` | `Id` | - | `GetStatementResultV2Response` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Fetches the temporarily cached result of an SQL statement in CSV format. The `ExecuteStatement` or `BatchExecuteStatement` operation that ran the SQL statement must have specified `ResultFormat` as `CSV`. |
| `ListDatabases` | - | `readonly`, `paginated` | `Database` | - | `ListDatabasesResponse` | `DatabaseConnectionException`, `InternalServerException`, `QueryTimeoutException`, `ResourceNotFoundException`, `ValidationException` | List the databases in a cluster. A token is returned to page through the database list. |
| `ListSchemas` | - | `readonly`, `paginated` | `Database` | - | `ListSchemasResponse` | `DatabaseConnectionException`, `InternalServerException`, `QueryTimeoutException`, `ResourceNotFoundException`, `ValidationException` | Lists the schemas in a database. A token is returned to page through the schema list. |
| `ListStatements` | - | `readonly`, `paginated` | - | - | `ListStatementsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | List of SQL statements. By default, only finished statements are shown. |
| `ListTables` | - | `readonly`, `paginated` | `Database` | - | `ListTablesResponse` | `DatabaseConnectionException`, `InternalServerException`, `QueryTimeoutException`, `ResourceNotFoundException`, `ValidationException` | List the tables in a database. If neither `SchemaPattern` nor `TablePattern` are specified, then all tables in the database are returned. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | The Amazon Redshift Data API operation failed due to invalid input. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId` | The Amazon Redshift Data API operation failed due to a missing resource. |
| `ValidationException` | `structure` | `Message` | The Amazon Redshift Data API operation failed due to invalid input. |
| `DatabaseConnectionException` | `structure` | `Message` | Connection to a database failed. |
| `QueryTimeoutException` | `structure` | `Message` | The Amazon Redshift Data API operation failed due to timeout. |
| `ActiveSessionsExceededException` | `structure` | `Message` | The Amazon Redshift Data API operation failed because the maximum number of active sessions exceeded. |
| `ActiveStatementsExceededException` | `structure` | `Message` | The number of active statements exceeds the limit. |
| `BatchExecuteStatementInput` | `structure` | `ClientToken`, `ClusterIdentifier`, `Database`, `DbUser`, `ResultFormat`, `SecretArn`, `SessionId`, `SessionKeepAliveSeconds`, `Sqls`, `StatementName`, `WithEvent`, `WorkgroupName` | - |
| `BatchExecuteStatementOutput` | `structure` | `ClusterIdentifier`, `CreatedAt`, `Database`, `DbGroups`, `DbUser`, `Id`, `SecretArn`, `SessionId`, `WorkgroupName` | - |
| `BatchExecuteStatementException` | `structure` | `Message`, `StatementId` | An SQL statement encountered an environmental error while running. |
| `CancelStatementRequest` | `structure` | `Id` | - |
| `CancelStatementResponse` | `structure` | `Status` | - |
| `DescribeStatementRequest` | `structure` | `Id` | - |
| `DescribeStatementResponse` | `structure` | `ClusterIdentifier`, `CreatedAt`, `Database`, `DbUser`, `Duration`, `Error`, `HasResultSet`, `Id`, `QueryParameters`, `QueryString`, `RedshiftPid`, `RedshiftQueryId`, ... (+9) | - |
| `DescribeTableRequest` | `structure` | `ClusterIdentifier`, `ConnectedDatabase`, `Database`, `DbUser`, `MaxResults`, `NextToken`, `Schema`, `SecretArn`, `Table`, `WorkgroupName` | - |
| `DescribeTableResponse` | `structure` | `ColumnList`, `NextToken`, `TableName` | - |
| `ExecuteStatementInput` | `structure` | `ClientToken`, `ClusterIdentifier`, `Database`, `DbUser`, `Parameters`, `ResultFormat`, `SecretArn`, `SessionId`, `SessionKeepAliveSeconds`, `Sql`, `StatementName`, `WithEvent`, ... (+1) | - |
| `ExecuteStatementOutput` | `structure` | `ClusterIdentifier`, `CreatedAt`, `Database`, `DbGroups`, `DbUser`, `Id`, `SecretArn`, `SessionId`, `WorkgroupName` | - |
| `ExecuteStatementException` | `structure` | `Message`, `StatementId` | The SQL statement encountered an environmental error while running. |
| `GetStatementResultRequest` | `structure` | `Id`, `NextToken` | - |
| `GetStatementResultResponse` | `structure` | `ColumnMetadata`, `NextToken`, `Records`, `TotalNumRows` | - |
| `GetStatementResultV2Request` | `structure` | `Id`, `NextToken` | - |
| `GetStatementResultV2Response` | `structure` | `ColumnMetadata`, `NextToken`, `Records`, `ResultFormat`, `TotalNumRows` | - |
| `ListDatabasesRequest` | `structure` | `ClusterIdentifier`, `Database`, `DbUser`, `MaxResults`, `NextToken`, `SecretArn`, `WorkgroupName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

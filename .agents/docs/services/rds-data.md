# AWS RDS DataService

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

RDS Data API Amazon RDS provides an HTTP endpoint to run SQL statements on an Amazon Aurora DB cluster. To run these statements, you use the RDS Data API (Data API). Data API is available with the following types of Aurora databases: Aurora PostgreSQL - Serverless v2, provisioned, and Serverless v1 Aurora MySQL - Serverless v2, provisioned, and Serverless v1 For more information about the Data API, see Using RDS Data API in the Amazon Aurora User Guide .

## Possible Usage Scenarios
- Backported from `crates/winterbaume-rdsdata/tests/scenario_test.rs`: begin a transaction, execute a `SELECT`, and commit it.
- Backported from `scenario_test.rs`: begin a transaction, execute an `INSERT`, and roll it back.
- Backported from `scenario_test.rs`: run multiple statements and a batch execute inside a transaction before commit.
- From the AWS documentation and model: support Aurora Serverless Data API workflows with SQL execution, transactions, parameter binding, batch statements, result sets, and Secrets Manager-backed authentication references.

## Service Identity and Protocol

- AWS model slug: `rds-data`
- AWS SDK for Rust slug: `rdsdata`
- Model version: `2018-08-01`
- Model file: `vendor/api-models-aws/models/rds-data/service/2018-08-01/rds-data-2018-08-01.json`
- SDK ID: `RDS Data`
- Endpoint prefix: `-`
- ARN namespace: `rds-data`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Execute` (2), `Batch` (1), `Begin` (1), `Commit` (1), `Rollback` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchExecuteStatement`.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`, `ECS`, `RDS`, `Secrets Manager`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/rdsdataservice/latest/APIReference/API_ExecuteStatement.html
- https://docs.aws.amazon.com/rdsdataservice/latest/APIReference/API_BatchExecuteStatement.html
- https://docs.aws.amazon.com/rdsdataservice/latest/APIReference/API_CommitTransaction.html

Research outcomes:
- RDS Data API runs SQL statements against supported Aurora/RDS resources through HTTPS APIs rather than persistent database client connections.
- Requests identify the database resource ARN, secret ARN, database name, SQL, parameters, and optional transaction id.
- `BeginTransaction` starts a transaction and returns a transaction id used by subsequent statement calls.
- `CommitTransaction` ends a transaction and commits changes; rollback ends the transaction without committing.
- `ExecuteStatement` runs one SQL statement and can return result records and column metadata.
- `BatchExecuteStatement` runs a batch SQL statement over parameter sets.
- The HTTP endpoint must be enabled for supported clusters before Data API calls can succeed.

Parity implications:
- Model HTTP endpoint enablement, statements, result sets, transactions, transaction ids, secret/resource validation, and parameter binding separately.
- Transaction ids should scope statements until commit or rollback.
- Data API behaviour should be distinct from RDS cluster lifecycle APIs.

## Control-Plane / Data-Plane Coherence

- **Paired with `rds`.** This data plane executes SQL against Aurora Serverless v1/v2 and Aurora MySQL/PostgreSQL clusters that the RDS control plane ( `winterbaume-rds` ) creates and manages the lifecycle of. `ExecuteStatement` and `BeginTransaction` carry a `resourceArn` that must point to an existing cluster with the Data API enabled; in real AWS the call fails with `BadRequestException` ( "Cluster not enabled for Data API" ) if it is not.
- **Current Winterbaume status: deliberately separate.** This crate is a "bring-your-own-result" mock — callers `enqueue_result` ahead of time and `ExecuteStatement` dequeues. It does not validate `resourceArn` against `winterbaume-rds`'s clusters and never executes real SQL. This is acceptable for unit testing where the test author seeds expected results, but it is not parity behaviour.
- **What needs to change ( low priority ):** if a future workflow needs the data plane to validate cluster existence and "Data API enabled" state, this crate should observe `winterbaume-rds`'s `db_clusters` and reject unknown ARNs. Real SQL execution is not in scope for the mock.

## Operation Groups

### Execute

- Operations: `ExecuteSql`, `ExecuteStatement`
- Common required input members in this group: -

### Batch

- Operations: `BatchExecuteStatement`
- Common required input members in this group: -

### Begin

- Operations: `BeginTransaction`
- Common required input members in this group: -

### Commit

- Operations: `CommitTransaction`
- Common required input members in this group: -

### Rollback

- Operations: `RollbackTransaction`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchExecuteStatement` | `POST /BatchExecute` | - | `resourceArn`, `secretArn`, `sql` | - | `BatchExecuteStatementResponse` | `AccessDeniedException`, `BadRequestException`, `DatabaseErrorException`, `DatabaseNotFoundException`, `DatabaseResumingException`, `DatabaseUnavailableException`, `ForbiddenException`, `HttpEndpointNotEnabledException`, `InternalServerErrorException`, `InvalidResourceStateException`, `InvalidSecretException`, `SecretsErrorException`, `ServiceUnavailableError`, `StatementTimeoutException`, `TransactionNotFoundException` | Runs a batch SQL statement over an array of data. You can run bulk update and insert operations for multiple records using a DML statement with different parameter sets. Bulk operations can provide a significant perf ... |
| `BeginTransaction` | `POST /BeginTransaction` | - | `resourceArn`, `secretArn` | - | `BeginTransactionResponse` | `AccessDeniedException`, `BadRequestException`, `DatabaseErrorException`, `DatabaseNotFoundException`, `DatabaseResumingException`, `DatabaseUnavailableException`, `ForbiddenException`, `HttpEndpointNotEnabledException`, `InternalServerErrorException`, `InvalidResourceStateException`, `InvalidSecretException`, `SecretsErrorException`, `ServiceUnavailableError`, `StatementTimeoutException`, `TransactionNotFoundException` | Starts a SQL transaction. A transaction can run for a maximum of 24 hours. A transaction is terminated and rolled back automatically after 24 hours. A transaction times out if no calls use its transaction ID in three ... |
| `CommitTransaction` | `POST /CommitTransaction` | - | `resourceArn`, `secretArn`, `transactionId` | - | `CommitTransactionResponse` | `AccessDeniedException`, `BadRequestException`, `DatabaseErrorException`, `DatabaseNotFoundException`, `DatabaseUnavailableException`, `ForbiddenException`, `HttpEndpointNotEnabledException`, `InternalServerErrorException`, `InvalidResourceStateException`, `InvalidSecretException`, `NotFoundException`, `SecretsErrorException`, `ServiceUnavailableError`, `StatementTimeoutException`, `TransactionNotFoundException` | Ends a SQL transaction started with the BeginTransaction operation and commits the changes. |
| `ExecuteSql` | `POST /ExecuteSql` | - | `dbClusterOrInstanceArn`, `awsSecretStoreArn`, `sqlStatements` | - | `ExecuteSqlResponse` | `AccessDeniedException`, `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableError` | Runs one or more SQL statements. This operation isn't supported for Aurora Serverless v2 and provisioned DB clusters. For Aurora Serverless v1 DB clusters, the operation is deprecated. Use the BatchExecuteStatement o ... |
| `ExecuteStatement` | `POST /Execute` | - | `resourceArn`, `secretArn`, `sql` | - | `ExecuteStatementResponse` | `AccessDeniedException`, `BadRequestException`, `DatabaseErrorException`, `DatabaseNotFoundException`, `DatabaseResumingException`, `DatabaseUnavailableException`, `ForbiddenException`, `HttpEndpointNotEnabledException`, `InternalServerErrorException`, `InvalidResourceStateException`, `InvalidSecretException`, `SecretsErrorException`, `ServiceUnavailableError`, `StatementTimeoutException`, `TransactionNotFoundException`, `UnsupportedResultException` | Runs a SQL statement against a database. If a call isn't part of a transaction because it doesn't include the transactionID parameter, changes that result from the call are committed automatically. If the binary resp ... |
| `RollbackTransaction` | `POST /RollbackTransaction` | - | `resourceArn`, `secretArn`, `transactionId` | - | `RollbackTransactionResponse` | `AccessDeniedException`, `BadRequestException`, `DatabaseErrorException`, `DatabaseNotFoundException`, `DatabaseUnavailableException`, `ForbiddenException`, `HttpEndpointNotEnabledException`, `InternalServerErrorException`, `InvalidResourceStateException`, `InvalidSecretException`, `NotFoundException`, `SecretsErrorException`, `ServiceUnavailableError`, `StatementTimeoutException`, `TransactionNotFoundException` | Performs a rollback of a transaction. Rolling back a transaction cancels its changes. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient access to perform this action. |
| `BadRequestException` | `structure` | message | There is an error in the call or in a SQL statement. (This error only appears in calls from Aurora Serverless v1 databases.) |
| `DatabaseErrorException` | `structure` | message | There was an error in processing the SQL statement. |
| `DatabaseNotFoundException` | `structure` | message | The DB cluster doesn't have a DB instance. |
| `DatabaseResumingException` | `structure` | message | A request was cancelled because the Aurora Serverless v2 DB instance was paused. The Data API request automatically resumes the DB instance. Wait a few seco ... |
| `DatabaseUnavailableException` | `structure` | **empty (no members)** | The writer instance in the DB cluster isn't available. |
| `ForbiddenException` | `structure` | message | There are insufficient privileges to make the call. |
| `HttpEndpointNotEnabledException` | `structure` | message | The HTTP endpoint for using RDS Data API isn't enabled for the DB cluster. |
| `InternalServerErrorException` | `structure` | **empty (no members)** | An internal error occurred. |
| `InvalidResourceStateException` | `structure` | message | The resource is in an invalid state. |
| `InvalidSecretException` | `structure` | message | The Secrets Manager secret used with the request isn't valid. |
| `NotFoundException` | `structure` | message | The resourceArn , secretArn , or transactionId value can't be found. |
| `SecretsErrorException` | `structure` | message | There was a problem with the Secrets Manager secret used with the request, caused by one of the following conditions: RDS Data API timed out retrieving the ... |
| `ServiceUnavailableError` | `structure` | **empty (no members)** | The service specified by the resourceArn parameter isn't available. |
| `StatementTimeoutException` | `structure` | message, dbConnectionId | The execution of the SQL statement timed out. |
| `TransactionNotFoundException` | `structure` | message | The transaction ID wasn't found. |
| `UnsupportedResultException` | `structure` | message | There was a problem with the result because of one of the following conditions: It contained an unsupported data type. It contained a multidimensional array ... |
| `BatchExecuteStatementRequest` | `structure` | resourceArn, secretArn, sql, database, schema, parameterSets, transactionId | The request parameters represent the input of a SQL statement over an array of data. |
| `BatchExecuteStatementResponse` | `structure` | updateResults | The response elements represent the output of a SQL statement over an array of data. |
| `BeginTransactionRequest` | `structure` | resourceArn, secretArn, database, schema | The request parameters represent the input of a request to start a SQL transaction. |
| `BeginTransactionResponse` | `structure` | transactionId | The response elements represent the output of a request to start a SQL transaction. |
| `CommitTransactionRequest` | `structure` | resourceArn, secretArn, transactionId | The request parameters represent the input of a commit transaction request. |
| `CommitTransactionResponse` | `structure` | transactionStatus | The response elements represent the output of a commit transaction request. |
| `ExecuteSqlRequest` | `structure` | dbClusterOrInstanceArn, awsSecretStoreArn, sqlStatements, database, schema | The request parameters represent the input of a request to run one or more SQL statements. |
| `ExecuteSqlResponse` | `structure` | sqlStatementResults | The response elements represent the output of a request to run one or more SQL statements. |
| `ExecuteStatementRequest` | `structure` | resourceArn, secretArn, sql, database, schema, parameters, transactionId, includeResultMetadata, continueAfterTimeout, resultSetOptions, formatRecordsAs | The request parameters represent the input of a request to run a SQL statement against a database. |
| `ExecuteStatementResponse` | `structure` | records, columnMetadata, numberOfRecordsUpdated, generatedFields, formattedRecords | The response elements represent the output of a request to run a SQL statement against a database. |
| `RollbackTransactionRequest` | `structure` | resourceArn, secretArn, transactionId | The request parameters represent the input of a request to perform a rollback of a transaction. |
| `RollbackTransactionResponse` | `structure` | transactionStatus | The response elements represent the output of a request to perform a rollback of a transaction. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

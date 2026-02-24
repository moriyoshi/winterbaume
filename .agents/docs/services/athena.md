# Amazon Athena

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Athena is an interactive query service that lets you use standard SQL to analyze data directly in Amazon S3. You can point Athena at your data in Amazon S3 and run ad-hoc queries and get results in seconds. Athena is serverless, so there is no infrastructure to set up or manage. You pay only for the queries you run. Athena scales automatically—executing queries in parallel—so results are fast, even with large datasets and complex queries.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Athena resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: run SQL queries against data in S3, manage workgroups, data catalogues, prepared statements, named queries, and capacity reservations.
- From the operation surface: model query execution lifecycle, result retrieval, batch query metadata, tagging, and governance controls for analytics workloads.

## Service Identity and Protocol

- AWS model slug: `athena`
- AWS SDK for Rust slug: `athena`
- Model version: `2017-05-18`
- Model file: `vendor/api-models-aws/models/athena/service/2017-05-18/athena-2017-05-18.json`
- SDK ID: `Athena`
- Endpoint prefix: `athena`
- ARN namespace: `athena`
- CloudFormation name: `Athena`
- CloudTrail event source: `athena.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (19), `List` (16), `Create` (7), `Update` (7), `Delete` (6), `Batch` (3), `Start` (3), `Stop` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetNamedQuery`, `BatchGetPreparedStatement`, `BatchGetQueryExecution`, `CancelCapacityReservation`, `CreateCapacityReservation`, `CreateDataCatalog`, `CreateNamedQuery`, `CreateNotebook`, `CreatePreparedStatement`, `CreatePresignedNotebookUrl`, `CreateWorkGroup`, `DeleteCapacityReservation`, `DeleteDataCatalog`, `DeleteNamedQuery`, `DeleteNotebook`, `DeletePreparedStatement`, `DeleteWorkGroup`, `ImportNotebook`, `PutCapacityAssignmentConfiguration`, `StartCalculationExecution`, ... (+14).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCalculationExecution`, `GetCalculationExecutionCode`, `GetCalculationExecutionStatus`, `GetCapacityAssignmentConfiguration`, `GetCapacityReservation`, `GetDataCatalog`, `GetDatabase`, `GetNamedQuery`, `GetNotebookMetadata`, `GetPreparedStatement`, `GetQueryExecution`, `GetQueryResults`, `GetQueryRuntimeStatistics`, `GetResourceDashboard`, `GetSession`, `GetSessionEndpoint`, `GetSessionStatus`, `GetTableMetadata`, `GetWorkGroup`, `ListApplicationDPUSizes`, ... (+15).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 10 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchGetQueryExecution`, `CancelCapacityReservation`, `ExportNotebook`, `GetCalculationExecution`, `GetCalculationExecutionCode`, `GetCalculationExecutionStatus`, `GetQueryExecution`, `ImportNotebook`, `ListCalculationExecutions`, `ListQueryExecutions`, `StartCalculationExecution`, `StartQueryExecution`, `StartSession`, `StopCalculationExecution`, `StopQueryExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 70 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SQS`, `Lambda`, `Glue`, `EC2/VPC`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/athena/latest/APIReference/API_StartQueryExecution.html
- https://docs.aws.amazon.com/athena/latest/APIReference/API_GetQueryExecution.html
- https://docs.aws.amazon.com/athena/latest/APIReference/API_QueryExecutionStatus.html
- https://docs.aws.amazon.com/athena/latest/ug/query-results-specify-location.html

Research outcomes:
- StartQueryExecution is idempotent by ClientRequestToken. Reusing a token with the same parameters returns the same QueryExecutionId; changing parameters returns an error.
- SDKs can auto-generate ClientRequestToken, but direct API callers must supply one.
- Query execution requires access to the selected workgroup, and external catalog queries require GetDataCatalog permission.
- ResultConfiguration controls output location and encryption, but workgroup settings can override client-side settings when EnforceWorkGroupConfiguration is enabled.
- Athena no longer creates a default query result bucket for new Region/account usage; callers must specify an output location or use an overriding workgroup.
- QueryExecutionStatus states are QUEUED, RUNNING, SUCCEEDED, FAILED, and CANCELLED.
- Certain transient errors can move a query from RUNNING back to QUEUED. FAILED is terminal with no automatic retry.
- GetQueryExecution returns query text, context, result configuration, workgroup, status, statistics, engine version, result reuse data, and error metadata.

Parity implications:
- Model workgroups, data catalogues, query executions, result locations, result reuse, engine versions, statistics, and error metadata.
- Query execution should be an asynchronous state machine with QUEUED, RUNNING, terminal states, cancellation, transient requeue, and timestamps.
- StartQueryExecution needs token-based idempotency keyed by request parameters, workgroup override resolution, output-location validation, and permission-sensitive catalogue access.

## Cross-Service Integration Gaps

- **`athena-glue`**: `StartQueryExecution` does not resolve database/table metadata from `winterbaume-glue` state when the target catalogue is of type `Glue`. In real AWS, Athena's default catalogue is the Glue Data Catalogue. Look up the target catalogue type on query execution, resolve Glue database/table schemas when the catalogue type is `Glue`, feed schema info into `AthenaQueryBackend` or a layer above it, and add cross-service integration tests. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `athena-glue` ).

## Operation Groups

### Get

- Operations: `GetCalculationExecution`, `GetCalculationExecutionCode`, `GetCalculationExecutionStatus`, `GetCapacityAssignmentConfiguration`, `GetCapacityReservation`, `GetDataCatalog`, `GetDatabase`, `GetNamedQuery`, `GetNotebookMetadata`, `GetPreparedStatement`, `GetQueryExecution`, `GetQueryResults`, `GetQueryRuntimeStatistics`, `GetResourceDashboard`, `GetSession`, `GetSessionEndpoint`, `GetSessionStatus`, `GetTableMetadata`, `GetWorkGroup`
- Traits: `paginated` (1)
- Common required input members in this group: `CalculationExecutionId`, `CapacityReservationName`, `CatalogName`, `DatabaseName`, `Name`, `NamedQueryId`, `NotebookId`, `QueryExecutionId`, `ResourceARN`, `SessionId`, `StatementName`, `TableName`, `WorkGroup`

### List

- Operations: `ListApplicationDPUSizes`, `ListCalculationExecutions`, `ListCapacityReservations`, `ListDataCatalogs`, `ListDatabases`, `ListEngineVersions`, `ListExecutors`, `ListNamedQueries`, `ListNotebookMetadata`, `ListNotebookSessions`, `ListPreparedStatements`, `ListQueryExecutions`, `ListSessions`, `ListTableMetadata`, `ListTagsForResource`, `ListWorkGroups`
- Traits: `paginated` (14)
- Common required input members in this group: `CatalogName`, `DatabaseName`, `NotebookId`, `ResourceARN`, `SessionId`, `WorkGroup`

### Create

- Operations: `CreateCapacityReservation`, `CreateDataCatalog`, `CreateNamedQuery`, `CreateNotebook`, `CreatePreparedStatement`, `CreatePresignedNotebookUrl`, `CreateWorkGroup`
- Traits: `idempotency-token` (1), `idempotent` (2)
- Common required input members in this group: `Database`, `Name`, `QueryStatement`, `QueryString`, `SessionId`, `StatementName`, `TargetDpus`, `Type`, `WorkGroup`

### Update

- Operations: `UpdateCapacityReservation`, `UpdateDataCatalog`, `UpdateNamedQuery`, `UpdateNotebook`, `UpdateNotebookMetadata`, `UpdatePreparedStatement`, `UpdateWorkGroup`
- Traits: `idempotent` (1)
- Common required input members in this group: `Name`, `NamedQueryId`, `NotebookId`, `Payload`, `QueryStatement`, `QueryString`, `StatementName`, `TargetDpus`, `Type`, `WorkGroup`

### Delete

- Operations: `DeleteCapacityReservation`, `DeleteDataCatalog`, `DeleteNamedQuery`, `DeleteNotebook`, `DeletePreparedStatement`, `DeleteWorkGroup`
- Traits: `idempotency-token` (1), `idempotent` (3)
- Common required input members in this group: `Name`, `NamedQueryId`, `NotebookId`, `StatementName`, `WorkGroup`

### Batch

- Operations: `BatchGetNamedQuery`, `BatchGetPreparedStatement`, `BatchGetQueryExecution`
- Common required input members in this group: `NamedQueryIds`, `PreparedStatementNames`, `QueryExecutionIds`, `WorkGroup`

### Start

- Operations: `StartCalculationExecution`, `StartQueryExecution`, `StartSession`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `EngineConfiguration`, `QueryString`, `SessionId`, `WorkGroup`

### Stop

- Operations: `StopCalculationExecution`, `StopQueryExecution`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `CalculationExecutionId`, `QueryExecutionId`

### Cancel

- Operations: `CancelCapacityReservation`
- Traits: `idempotent` (1)
- Common required input members in this group: `Name`

### Export

- Operations: `ExportNotebook`
- Common required input members in this group: `NotebookId`

### Import

- Operations: `ImportNotebook`
- Common required input members in this group: `Name`, `Type`, `WorkGroup`

### Put

- Operations: `PutCapacityAssignmentConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `CapacityAssignments`, `CapacityReservationName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Terminate

- Operations: `TerminateSession`
- Common required input members in this group: `SessionId`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetNamedQuery` | - | - | `NamedQueryIds` | - | `BatchGetNamedQueryOutput` | `InternalServerException`, `InvalidRequestException` | Returns the details of a single named query or a list of up to 50 queries, which you provide as an array of query ID strings. Requires you to have access to the workgroup in which the queries were saved. |
| `BatchGetPreparedStatement` | - | - | `PreparedStatementNames`, `WorkGroup` | - | `BatchGetPreparedStatementOutput` | `InternalServerException`, `InvalidRequestException` | Returns the details of a single prepared statement or a list of up to 256 prepared statements for the array of prepared statement names that you provide. Requires you to have access to the workgroup to which the prepared statements belong. |
| `BatchGetQueryExecution` | - | - | `QueryExecutionIds` | - | `BatchGetQueryExecutionOutput` | `InternalServerException`, `InvalidRequestException` | Returns the details of a single query execution or a list of up to 50 query executions, which you provide as an array of query execution ID strings. Requires you to have access to the workgroup in which the queries ran. |
| `CancelCapacityReservation` | - | `idempotent` | `Name` | - | `CancelCapacityReservationOutput` | `InternalServerException`, `InvalidRequestException` | Cancels the capacity reservation with the specified name. Cancelled reservations remain in your account and will be deleted 45 days after cancellation. |
| `CreateCapacityReservation` | - | `idempotent` | `Name`, `TargetDpus` | - | `CreateCapacityReservationOutput` | `InternalServerException`, `InvalidRequestException` | Creates a capacity reservation with the specified name and number of requested data processing units. |
| `CreateDataCatalog` | - | - | `Name`, `Type` | - | `CreateDataCatalogOutput` | `InternalServerException`, `InvalidRequestException` | Creates (registers) a data catalog with the specified name and properties. Catalogs created are visible to all users of the same Amazon Web Services account. |
| `CreateNamedQuery` | - | `idempotent`, `idempotency-token` | `Database`, `Name`, `QueryString` | `ClientRequestToken` | `CreateNamedQueryOutput` | `InternalServerException`, `InvalidRequestException` | Creates a named query in the specified workgroup. Requires that you have access to the workgroup. |
| `CreateNotebook` | - | - | `Name`, `WorkGroup` | - | `CreateNotebookOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Creates an empty `ipynb` file in the specified Apache Spark enabled workgroup. Throws an error if a file in the workgroup with the same name already exists. |
| `CreatePreparedStatement` | - | - | `QueryStatement`, `StatementName`, `WorkGroup` | - | `CreatePreparedStatementOutput` | `InternalServerException`, `InvalidRequestException` | Creates a prepared statement for use with SQL queries in Athena. |
| `CreatePresignedNotebookUrl` | - | - | `SessionId` | - | `CreatePresignedNotebookUrlResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets an authentication token and the URL at which the notebook can be accessed. During programmatic access, `CreatePresignedNotebookUrl` must be called every 10 minutes to refresh the authentication token. |
| `CreateWorkGroup` | - | - | `Name` | - | `CreateWorkGroupOutput` | `InternalServerException`, `InvalidRequestException` | Creates a workgroup with the specified name. A workgroup can be an Apache Spark enabled workgroup or an Athena SQL workgroup. |
| `DeleteCapacityReservation` | - | `idempotent` | `Name` | - | `DeleteCapacityReservationOutput` | `InternalServerException`, `InvalidRequestException` | Deletes a cancelled capacity reservation. A reservation must be cancelled before it can be deleted. |
| `DeleteDataCatalog` | - | - | `Name` | - | `DeleteDataCatalogOutput` | `InternalServerException`, `InvalidRequestException` | Deletes a data catalog. |
| `DeleteNamedQuery` | - | `idempotent`, `idempotency-token` | `NamedQueryId` | `NamedQueryId` | `DeleteNamedQueryOutput` | `InternalServerException`, `InvalidRequestException` | Deletes the named query if you have access to the workgroup in which the query was saved. |
| `DeleteNotebook` | - | - | `NotebookId` | - | `DeleteNotebookOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Deletes the specified notebook. |
| `DeletePreparedStatement` | - | - | `StatementName`, `WorkGroup` | - | `DeletePreparedStatementOutput` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Deletes the prepared statement with the specified name from the specified workgroup. |
| `DeleteWorkGroup` | - | `idempotent` | `WorkGroup` | - | `DeleteWorkGroupOutput` | `InternalServerException`, `InvalidRequestException` | Deletes the workgroup with the specified name. The primary workgroup cannot be deleted. |
| `ExportNotebook` | - | - | `NotebookId` | - | `ExportNotebookOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Exports the specified notebook and its metadata. |
| `GetCalculationExecution` | - | - | `CalculationExecutionId` | - | `GetCalculationExecutionResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Describes a previously submitted calculation execution. |
| `GetCalculationExecutionCode` | - | - | `CalculationExecutionId` | - | `GetCalculationExecutionCodeResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Retrieves the unencrypted code that was executed for the calculation. |
| `GetCalculationExecutionStatus` | - | - | `CalculationExecutionId` | - | `GetCalculationExecutionStatusResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets the status of a current calculation. |
| `GetCapacityAssignmentConfiguration` | - | - | `CapacityReservationName` | - | `GetCapacityAssignmentConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Gets the capacity assignment configuration for a capacity reservation, if one exists. |
| `GetCapacityReservation` | - | - | `Name` | - | `GetCapacityReservationOutput` | `InternalServerException`, `InvalidRequestException` | Returns information about the capacity reservation with the specified name. |
| `GetDataCatalog` | - | - | `Name` | - | `GetDataCatalogOutput` | `InternalServerException`, `InvalidRequestException` | Returns the specified data catalog. |
| `GetDatabase` | - | - | `CatalogName`, `DatabaseName` | - | `GetDatabaseOutput` | `InternalServerException`, `InvalidRequestException`, `MetadataException` | Returns a database object for the specified database and data catalog. |
| `GetNamedQuery` | - | - | `NamedQueryId` | - | `GetNamedQueryOutput` | `InternalServerException`, `InvalidRequestException` | Returns information about a single query. Requires that you have access to the workgroup in which the query was saved. |
| `GetNotebookMetadata` | - | - | `NotebookId` | - | `GetNotebookMetadataOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Retrieves notebook metadata for the specified notebook ID. |
| `GetPreparedStatement` | - | - | `StatementName`, `WorkGroup` | - | `GetPreparedStatementOutput` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Retrieves the prepared statement with the specified name from the specified workgroup. |
| `GetQueryExecution` | - | - | `QueryExecutionId` | - | `GetQueryExecutionOutput` | `InternalServerException`, `InvalidRequestException` | Returns information about a single execution of a query if you have access to the workgroup in which the query ran. Each time a query executes, information about the query execution is saved with a unique ID. |
| `GetQueryResults` | - | `paginated` | `QueryExecutionId` | - | `GetQueryResultsOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Streams the results of a single query execution specified by `QueryExecutionId` from the Athena query results location in Amazon S3. For more information, see Working with query results, recent queries, and output files in the Amazon Athena User Guide . |
| `GetQueryRuntimeStatistics` | - | - | `QueryExecutionId` | - | `GetQueryRuntimeStatisticsOutput` | `InternalServerException`, `InvalidRequestException` | Returns query execution runtime statistics related to a single execution of a query if you have access to the workgroup in which the query ran. Statistics from the `Timeline` section of the response object are available as soon as QueryExecutionStatus$State... |
| `GetResourceDashboard` | - | - | `ResourceARN` | - | `GetResourceDashboardResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets the Live UI/Persistence UI for a session. |
| `GetSession` | - | - | `SessionId` | - | `GetSessionResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets the full details of a previously created session, including the session status and configuration. |
| `GetSessionEndpoint` | - | - | `SessionId` | - | `GetSessionEndpointResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets a connection endpoint and authentication token for a given session Id. |
| `GetSessionStatus` | - | - | `SessionId` | - | `GetSessionStatusResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Gets the current status of a session. |
| `GetTableMetadata` | - | - | `CatalogName`, `DatabaseName`, `TableName` | - | `GetTableMetadataOutput` | `InternalServerException`, `InvalidRequestException`, `MetadataException` | Returns table metadata for the specified catalog, database, and table. |
| `GetWorkGroup` | - | - | `WorkGroup` | - | `GetWorkGroupOutput` | `InternalServerException`, `InvalidRequestException` | Returns information about the workgroup with the specified name. |
| `ImportNotebook` | - | - | `Name`, `Type`, `WorkGroup` | - | `ImportNotebookOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Imports a single `ipynb` file to a Spark enabled workgroup. To import the notebook, the request must specify a value for either `Payload` or `NoteBookS3LocationUri`. |
| `ListApplicationDPUSizes` | - | `paginated` | - | - | `ListApplicationDPUSizesOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Returns the supported DPU sizes for the supported application runtimes (for example, `Athena notebook version 1`). |
| `ListCalculationExecutions` | - | `paginated` | `SessionId` | - | `ListCalculationExecutionsResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists the calculations that have been submitted to a session in descending order. Newer calculations are listed first; older calculations are listed later. |
| `ListCapacityReservations` | - | `paginated` | - | - | `ListCapacityReservationsOutput` | `InternalServerException`, `InvalidRequestException` | Lists the capacity reservations for the current account. |
| `ListDataCatalogs` | - | `paginated` | - | - | `ListDataCatalogsOutput` | `InternalServerException`, `InvalidRequestException` | Lists the data catalogs in the current Amazon Web Services account. In the Athena console, data catalogs are listed as "data sources" on the Data sources page under the Data source name column. |
| `ListDatabases` | - | `paginated` | `CatalogName` | - | `ListDatabasesOutput` | `InternalServerException`, `InvalidRequestException`, `MetadataException` | Lists the databases in the specified data catalog. |
| `ListEngineVersions` | - | `paginated` | - | - | `ListEngineVersionsOutput` | `InternalServerException`, `InvalidRequestException` | Returns a list of engine versions that are available to choose from, including the Auto option. |
| `ListExecutors` | - | `paginated` | `SessionId` | - | `ListExecutorsResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists, in descending order, the executors that joined a session. Newer executors are listed first; older executors are listed later. |
| `ListNamedQueries` | - | `paginated` | - | - | `ListNamedQueriesOutput` | `InternalServerException`, `InvalidRequestException` | Provides a list of available query IDs only for queries saved in the specified workgroup. Requires that you have access to the specified workgroup. |
| `ListNotebookMetadata` | - | - | `WorkGroup` | - | `ListNotebookMetadataOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Displays the notebook files for the specified workgroup in paginated format. |
| `ListNotebookSessions` | - | - | `NotebookId` | - | `ListNotebookSessionsResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists, in descending order, the sessions that have been created in a notebook that are in an active state like `CREATING`, `CREATED`, `IDLE` or `BUSY`. Newer sessions are listed first; older sessions are listed later. |
| `ListPreparedStatements` | - | `paginated` | `WorkGroup` | - | `ListPreparedStatementsOutput` | `InternalServerException`, `InvalidRequestException` | Lists the prepared statements in the specified workgroup. |
| `ListQueryExecutions` | - | `paginated` | - | - | `ListQueryExecutionsOutput` | `InternalServerException`, `InvalidRequestException` | Provides a list of available query execution IDs for the queries in the specified workgroup. Athena keeps a query history for 45 days. |
| `ListSessions` | - | `paginated` | `WorkGroup` | - | `ListSessionsResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists the sessions in a workgroup that are in an active state like `CREATING`, `CREATED`, `IDLE`, or `BUSY`. Newer sessions are listed first; older sessions are listed later. |
| `ListTableMetadata` | - | `paginated` | `CatalogName`, `DatabaseName` | - | `ListTableMetadataOutput` | `InternalServerException`, `InvalidRequestException`, `MetadataException` | Lists the metadata for the tables in the specified data catalog database. |
| `ListTagsForResource` | - | `paginated` | `ResourceARN` | - | `ListTagsForResourceOutput` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Lists the tags associated with an Athena resource. |
| `ListWorkGroups` | - | `paginated` | - | - | `ListWorkGroupsOutput` | `InternalServerException`, `InvalidRequestException` | Lists available workgroups for the account. |
| `PutCapacityAssignmentConfiguration` | - | `idempotent` | `CapacityAssignments`, `CapacityReservationName` | - | `PutCapacityAssignmentConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Puts a new capacity assignment configuration for a specified capacity reservation. If a capacity assignment configuration already exists for the capacity reservation, replaces the existing capacity assignment configuration. |
| `StartCalculationExecution` | - | - | `SessionId` | - | `StartCalculationExecutionResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Submits calculations for execution within a session. You can supply the code to run as an inline code block within the request. |
| `StartQueryExecution` | - | `idempotent`, `idempotency-token` | `QueryString` | `ClientRequestToken` | `StartQueryExecutionOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Runs the SQL query statements contained in the `Query`. Requires you to have access to the workgroup in which the query ran. |
| `StartSession` | - | - | `EngineConfiguration`, `WorkGroup` | - | `StartSessionResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException`, `SessionAlreadyExistsException`, `TooManyRequestsException` | Creates a session for running calculations within a workgroup. The session is ready when it reaches an `IDLE` state. |
| `StopCalculationExecution` | - | - | `CalculationExecutionId` | - | `StopCalculationExecutionResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Requests the cancellation of a calculation. A `StopCalculationExecution` call on a calculation that is already in a terminal state (for example, `STOPPED`, `FAILED`, or `COMPLETED`) succeeds but has no effect. |
| `StopQueryExecution` | - | `idempotent`, `idempotency-token` | `QueryExecutionId` | `QueryExecutionId` | `StopQueryExecutionOutput` | `InternalServerException`, `InvalidRequestException` | Stops a query execution. Requires you to have access to the workgroup in which the query ran. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceOutput` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Adds one or more tags to an Athena resource. A tag is a label that you assign to a resource. |
| `TerminateSession` | - | - | `SessionId` | - | `TerminateSessionResponse` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Terminates an active session. A `TerminateSession` call on a session that is already inactive (for example, in a `FAILED`, `TERMINATED` or `TERMINATING` state) succeeds but has no effect. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Removes one or more tags from an Athena resource. |
| `UpdateCapacityReservation` | - | - | `Name`, `TargetDpus` | - | `UpdateCapacityReservationOutput` | `InternalServerException`, `InvalidRequestException` | Updates the number of requested data processing units for the capacity reservation with the specified name. |
| `UpdateDataCatalog` | - | - | `Name`, `Type` | - | `UpdateDataCatalogOutput` | `InternalServerException`, `InvalidRequestException` | Updates the data catalog that has the specified name. |
| `UpdateNamedQuery` | - | `idempotent` | `Name`, `NamedQueryId`, `QueryString` | - | `UpdateNamedQueryOutput` | `InternalServerException`, `InvalidRequestException` | Updates a NamedQuery object. The database or workgroup cannot be updated. |
| `UpdateNotebook` | - | - | `NotebookId`, `Payload`, `Type` | - | `UpdateNotebookOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Updates the contents of a Spark notebook. |
| `UpdateNotebookMetadata` | - | - | `Name`, `NotebookId` | - | `UpdateNotebookMetadataOutput` | `InternalServerException`, `InvalidRequestException`, `TooManyRequestsException` | Updates the metadata for a notebook. |
| `UpdatePreparedStatement` | - | - | `QueryStatement`, `StatementName`, `WorkGroup` | - | `UpdatePreparedStatementOutput` | `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Updates a prepared statement. |
| `UpdateWorkGroup` | - | - | `WorkGroup` | - | `UpdateWorkGroupOutput` | `InternalServerException`, `InvalidRequestException` | Updates the workgroup with the specified name. The workgroup's name cannot be changed. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | Indicates a platform issue, which may be due to a transient condition or outage. |
| `InvalidRequestException` | `structure` | `AthenaErrorCode`, `Message` | Indicates that something is wrong with the input to the request. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceName` | A resource, such as a workgroup, was not found. |
| `TooManyRequestsException` | `structure` | `Message`, `Reason` | Indicates that the request was throttled. |
| `MetadataException` | `structure` | `Message` | An exception that Athena received when it called a custom metastore. |
| `BatchGetNamedQueryInput` | `structure` | `NamedQueryIds` | Contains an array of named query IDs. |
| `BatchGetNamedQueryOutput` | `structure` | `NamedQueries`, `UnprocessedNamedQueryIds` | - |
| `BatchGetPreparedStatementInput` | `structure` | `PreparedStatementNames`, `WorkGroup` | - |
| `BatchGetPreparedStatementOutput` | `structure` | `PreparedStatements`, `UnprocessedPreparedStatementNames` | - |
| `BatchGetQueryExecutionInput` | `structure` | `QueryExecutionIds` | Contains an array of query execution IDs. |
| `BatchGetQueryExecutionOutput` | `structure` | `QueryExecutions`, `UnprocessedQueryExecutionIds` | - |
| `CancelCapacityReservationInput` | `structure` | `Name` | - |
| `CancelCapacityReservationOutput` | `structure` | - | - |
| `CreateCapacityReservationInput` | `structure` | `Name`, `Tags`, `TargetDpus` | - |
| `CreateCapacityReservationOutput` | `structure` | - | - |
| `CreateDataCatalogInput` | `structure` | `Description`, `Name`, `Parameters`, `Tags`, `Type` | - |
| `CreateDataCatalogOutput` | `structure` | `DataCatalog` | - |
| `CreateNamedQueryInput` | `structure` | `ClientRequestToken`, `Database`, `Description`, `Name`, `QueryString`, `WorkGroup` | - |
| `CreateNamedQueryOutput` | `structure` | `NamedQueryId` | - |
| `CreateNotebookInput` | `structure` | `ClientRequestToken`, `Name`, `WorkGroup` | - |
| `CreateNotebookOutput` | `structure` | `NotebookId` | - |
| `CreatePreparedStatementInput` | `structure` | `Description`, `QueryStatement`, `StatementName`, `WorkGroup` | - |
| `CreatePreparedStatementOutput` | `structure` | - | - |
| `CreatePresignedNotebookUrlRequest` | `structure` | `SessionId` | - |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/pluggable-backends-and-query-execution-synthesis.md, .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### Query Backends

- Athena query execution is backend-injected through `AthenaQueryBackend`; the in-memory service path should not grow a hard dependency on DuckDB or any other SQL engine.
- DuckDB support belongs in `winterbaume-sqlengine-duckdb` and should stay opt-in. `DuckDbAthenaQueryBackend::new(conn: Connection)` stores a mutex-protected connection, and per-query execution clones a handle to the same underlying database, so tests can seed the connection before backend construction.
- Snapshot, restore, and merge fidelity should remain backend-facing. Avoid hidden service-local query state that would bypass the backend selected by `with_query_backend`.

### Glue Catalogue Integration

- `DataCatalogType::Glue` handling is catalogue metadata only unless query execution resolves schemas from Glue state. Real parity needs a catalogue-resolution layer that reads `winterbaume-glue` databases and tables and passes schema definitions into the query backend.
- The important parity boundary is query-time schema resolution, not just `CreateDataCatalog` / `GetDataCatalog` CRUD. Athena tests should prove that a Glue-managed table can drive SQL execution.
- Cross-service Athena and Glue tests should create or seed Glue database/table state, run Athena queries through the configured backend, and assert that missing databases, missing tables, and stale schemas fail through Athena-shaped errors.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

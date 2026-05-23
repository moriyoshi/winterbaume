# AWS Lake Formation

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Lake Formation Defines the public endpoint for the Lake Formation service.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Lake Formation workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Update`, `Delete`, `Create` operation families, including `GetDataCellsFilter`, `GetDataLakePrincipal`, `GetDataLakeSettings`, `GetEffectivePermissionsForPath`, `ListDataCellsFilter`, `ListLFTagExpressions`.

## Service Identity and Protocol

- AWS model slug: `lakeformation`
- AWS SDK for Rust slug: `lakeformation`
- Model version: `2017-03-31`
- Model file: `vendor/api-models-aws/models/lakeformation/service/2017-03-31/lakeformation-2017-03-31.json`
- SDK ID: `LakeFormation`
- Endpoint prefix: `lakeformation`
- ARN namespace: `lakeformation`
- CloudFormation name: `LakeFormation`
- CloudTrail event source: `lakeformation.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (15), `List` (8), `Update` (7), `Delete` (6), `Create` (5), `Describe` (3), `Batch` (2), `Search` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddLFTagsToResource`, `BatchGrantPermissions`, `BatchRevokePermissions`, `CancelTransaction`, `CreateDataCellsFilter`, `CreateLFTag`, `CreateLFTagExpression`, `CreateLakeFormationIdentityCenterConfiguration`, `CreateLakeFormationOptIn`, `DeleteDataCellsFilter`, `DeleteLFTag`, `DeleteLFTagExpression`, `DeleteLakeFormationIdentityCenterConfiguration`, `DeleteLakeFormationOptIn`, `DeleteObjectsOnCancel`, `DeregisterResource`, `PutDataLakeSettings`, `RegisterResource`, `RemoveLFTagsFromResource`, `RevokePermissions`, ... (+9).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeLakeFormationIdentityCenterConfiguration`, `DescribeResource`, `DescribeTransaction`, `GetDataCellsFilter`, `GetDataLakePrincipal`, `GetDataLakeSettings`, `GetEffectivePermissionsForPath`, `GetLFTag`, `GetLFTagExpression`, `GetQueryState`, `GetQueryStatistics`, `GetResourceLFTags`, `GetTableObjects`, `GetTemporaryDataLocationCredentials`, `GetTemporaryGluePartitionCredentials`, `GetTemporaryGlueTableCredentials`, `GetWorkUnitResults`, `GetWorkUnits`, `ListDataCellsFilter`, `ListLFTagExpressions`, ... (+8).
- Pagination is modelled for 13 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelTransaction`, `StartQueryPlanning`, `StartTransaction`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 61 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Glue`, `EC2/VPC`, `Redshift`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/lake-formation/latest/dg/what-is-lake-formation.html
- https://docs.aws.amazon.com/lake-formation/latest/dg/how-it-works.html
- https://docs.aws.amazon.com/lake-formation/latest/dg/metadata-permissions.html

Research outcomes:
- Lake Formation centrally manages access controls for data lakes across Data Catalog metadata and registered Amazon S3 data locations.
- Policies can restrict access at database, table, column, row, and cell levels for IAM principals and federated users or groups.
- Integrated engines such as Athena, AWS Glue, Amazon EMR, and Redshift Spectrum first request metadata from the Data Catalog, which checks Lake Formation permissions before returning authorised metadata.
- If underlying data is registered with Lake Formation, the analytic engine asks Lake Formation for temporary data access credentials. This credential vending path lets Lake Formation enforce column, row, or cell filtering.
- If a table is not managed by Lake Formation, access goes directly through S3 bucket policy and IAM policy evaluation.
- Permissions can be granted by named resources or by LF-Tag based access control. LF-Tags are Lake Formation-specific metadata and are not AWS resource tags.
- Effective Lake Formation permissions for a principal are the union of all matching named-resource, LF-Tag, data-filter, and row-level policies.
- Data location permissions let non-administrative users create databases or tables only at authorised S3 locations.
- Data lake administrators receive implicit describe, location, creation, grant, and revoke permissions. Database and table creators receive implicit permissions on the resources they create.
- Grantable permissions allow non-administrative principals to delegate permissions on resources they control.
- Hybrid access mode allows selected principals to use Lake Formation permissions while existing IAM/S3 access for other workloads continues.

Parity implications:
- Model data lake settings, administrators, registered resource locations, permissions, grantable permissions, LF-Tags, LF-Tag attachments, data filters, resource links, and cross-account shares separately.
- Authorisation should combine named-resource grants, LF-Tag policies, implicit permissions, grantable permissions, and registered-location state.
- Credential vending and metadata filtering should be treated as core service behaviour, even if data-plane query execution is approximated.

## Operation Groups

### Get

- Operations: `GetDataCellsFilter`, `GetDataLakePrincipal`, `GetDataLakeSettings`, `GetEffectivePermissionsForPath`, `GetLFTag`, `GetLFTagExpression`, `GetQueryState`, `GetQueryStatistics`, `GetResourceLFTags`, `GetTableObjects`, `GetTemporaryDataLocationCredentials`, `GetTemporaryGluePartitionCredentials`, `GetTemporaryGlueTableCredentials`, `GetWorkUnitResults`, `GetWorkUnits`
- Traits: `paginated` (3)
- Common required input members in this group: `DatabaseName`, `TableName`, `Name`, `QueryId`, `TableArn`

### List

- Operations: `ListDataCellsFilter`, `ListLakeFormationOptIns`, `ListLFTagExpressions`, `ListLFTags`, `ListPermissions`, `ListResources`, `ListTableStorageOptimizers`, `ListTransactions`
- Traits: `paginated` (8)
- Common required input members in this group: -

### Update

- Operations: `UpdateDataCellsFilter`, `UpdateLakeFormationIdentityCenterConfiguration`, `UpdateLFTag`, `UpdateLFTagExpression`, `UpdateResource`, `UpdateTableObjects`, `UpdateTableStorageOptimizer`
- Common required input members in this group: `DatabaseName`, `TableName`

### Delete

- Operations: `DeleteDataCellsFilter`, `DeleteLakeFormationIdentityCenterConfiguration`, `DeleteLakeFormationOptIn`, `DeleteLFTag`, `DeleteLFTagExpression`, `DeleteObjectsOnCancel`
- Common required input members in this group: -

### Create

- Operations: `CreateDataCellsFilter`, `CreateLakeFormationIdentityCenterConfiguration`, `CreateLakeFormationOptIn`, `CreateLFTag`, `CreateLFTagExpression`
- Common required input members in this group: -

### Describe

- Operations: `DescribeLakeFormationIdentityCenterConfiguration`, `DescribeResource`, `DescribeTransaction`
- Common required input members in this group: -

### Batch

- Operations: `BatchGrantPermissions`, `BatchRevokePermissions`
- Common required input members in this group: `Entries`

### Search

- Operations: `SearchDatabasesByLFTags`, `SearchTablesByLFTags`
- Traits: `paginated` (2)
- Common required input members in this group: `Expression`

### Start

- Operations: `StartQueryPlanning`, `StartTransaction`
- Common required input members in this group: -

### Add

- Operations: `AddLFTagsToResource`
- Common required input members in this group: -

### Assume

- Operations: `AssumeDecoratedRoleWithSAML`
- Common required input members in this group: -

### Cancel

- Operations: `CancelTransaction`
- Common required input members in this group: -

### Commit

- Operations: `CommitTransaction`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterResource`
- Common required input members in this group: -

### Extend

- Operations: `ExtendTransaction`
- Common required input members in this group: -

### Grant

- Operations: `GrantPermissions`
- Common required input members in this group: -

### Put

- Operations: `PutDataLakeSettings`
- Common required input members in this group: -

### Register

- Operations: `RegisterResource`
- Common required input members in this group: -

### Remove

- Operations: `RemoveLFTagsFromResource`
- Common required input members in this group: -

### Revoke

- Operations: `RevokePermissions`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddLFTagsToResource` | `POST /AddLFTagsToResource` | - | `Resource`, `LFTags` | - | `AddLFTagsToResourceResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Attaches one or more LF-tags to an existing resource. |
| `AssumeDecoratedRoleWithSAML` | `POST /AssumeDecoratedRoleWithSAML` | - | `SAMLAssertion`, `RoleArn`, `PrincipalArn` | - | `AssumeDecoratedRoleWithSAMLResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Allows a caller to assume an IAM role decorated as the SAML user specified in the SAML assertion included in the request. This decoration allows Lake Formation to enforce access policies against the SAML users and gr ... |
| `BatchGrantPermissions` | `POST /BatchGrantPermissions` | - | `Entries` | - | `BatchGrantPermissionsResponse` | `InvalidInputException`, `OperationTimeoutException` | Batch operation to grant permissions to the principal. |
| `BatchRevokePermissions` | `POST /BatchRevokePermissions` | - | `Entries` | - | `BatchRevokePermissionsResponse` | `InvalidInputException`, `OperationTimeoutException` | Batch operation to revoke permissions from the principal. |
| `CancelTransaction` | `POST /CancelTransaction` | - | `TransactionId` | - | `CancelTransactionResponse` | `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `TransactionCommitInProgressException`, `TransactionCommittedException` | Attempts to cancel the specified transaction. Returns an exception if the transaction was previously committed. |
| `CommitTransaction` | `POST /CommitTransaction` | - | `TransactionId` | - | `CommitTransactionResponse` | `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `TransactionCanceledException` | Attempts to commit the specified transaction. Returns an exception if the transaction was previously aborted. This API action is idempotent if called multiple times for the same transaction. |
| `CreateDataCellsFilter` | `POST /CreateDataCellsFilter` | - | `TableData` | - | `CreateDataCellsFilterResponse` | `AccessDeniedException`, `AlreadyExistsException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNumberLimitExceededException` | Creates a data cell filter to allow one to grant access to certain columns on certain rows. |
| `CreateLakeFormationIdentityCenterConfiguration` | `POST /CreateLakeFormationIdentityCenterConfiguration` | - | - | - | `CreateLakeFormationIdentityCenterConfigurationResponse` | `AccessDeniedException`, `AlreadyExistsException`, `ConcurrentModificationException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Creates an IAM Identity Center connection with Lake Formation to allow IAM Identity Center users and groups to access Data Catalog resources. |
| `CreateLakeFormationOptIn` | `POST /CreateLakeFormationOptIn` | - | `Principal`, `Resource` | - | `CreateLakeFormationOptInResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNumberLimitExceededException` | Enforce Lake Formation permissions for the given databases, tables, and principals. |
| `CreateLFTag` | `POST /CreateLFTag` | - | `TagKey`, `TagValues` | - | `CreateLFTagResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNumberLimitExceededException` | Creates an LF-tag with the specified name and values. |
| `CreateLFTagExpression` | `POST /CreateLFTagExpression` | - | `Name`, `Expression` | - | `CreateLFTagExpressionResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNumberLimitExceededException` | Creates a new LF-Tag expression with the provided name, description, catalog ID, and expression body. This call fails if a LF-Tag expression with the same name already exists in the caller’s account or if the underly ... |
| `DeleteDataCellsFilter` | `POST /DeleteDataCellsFilter` | - | - | - | `DeleteDataCellsFilterResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Deletes a data cell filter. |
| `DeleteLakeFormationIdentityCenterConfiguration` | `POST /DeleteLakeFormationIdentityCenterConfiguration` | - | - | - | `DeleteLakeFormationIdentityCenterConfigurationResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Deletes an IAM Identity Center connection with Lake Formation. |
| `DeleteLakeFormationOptIn` | `POST /DeleteLakeFormationOptIn` | - | `Principal`, `Resource` | - | `DeleteLakeFormationOptInResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Remove the Lake Formation permissions enforcement of the given databases, tables, and principals. |
| `DeleteLFTag` | `POST /DeleteLFTag` | - | `TagKey` | - | `DeleteLFTagResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Deletes an LF-tag by its key name. The operation fails if the specified tag key doesn't exist. When you delete an LF-Tag: The associated LF-Tag policy becomes invalid. Resources that had this tag assigned will no lon ... |
| `DeleteLFTagExpression` | `POST /DeleteLFTagExpression` | - | `Name` | - | `DeleteLFTagExpressionResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Deletes the LF-Tag expression. The caller must be a data lake admin or have DROP permissions on the LF-Tag expression. Deleting a LF-Tag expression will also delete all LFTagPolicy permissions referencing the LF-Tag ... |
| `DeleteObjectsOnCancel` | `POST /DeleteObjectsOnCancel` | - | `DatabaseName`, `TableName`, `TransactionId`, `Objects` | - | `DeleteObjectsOnCancelResponse` | `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNotReadyException`, `TransactionCanceledException`, `TransactionCommittedException` | For a specific governed table, provides a list of Amazon S3 objects that will be written during the current transaction and that can be automatically deleted if the transaction is canceled. Without this call, no Amaz ... |
| `DeregisterResource` | `POST /DeregisterResource` | - | `ResourceArn` | - | `DeregisterResourceResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Deregisters the resource as managed by the Data Catalog. When you deregister a path, Lake Formation removes the path from the inline policy attached to your service-linked role. |
| `DescribeLakeFormationIdentityCenterConfiguration` | `POST /DescribeLakeFormationIdentityCenterConfiguration` | - | - | - | `DescribeLakeFormationIdentityCenterConfigurationResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Retrieves the instance ARN and application ARN for the connection. |
| `DescribeResource` | `POST /DescribeResource` | - | `ResourceArn` | - | `DescribeResourceResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Retrieves the current data access role for the given resource registered in Lake Formation. |
| `DescribeTransaction` | `POST /DescribeTransaction` | - | `TransactionId` | - | `DescribeTransactionResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns the details of a single transaction. |
| `ExtendTransaction` | `POST /ExtendTransaction` | - | - | - | `ExtendTransactionResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `TransactionCanceledException`, `TransactionCommitInProgressException`, `TransactionCommittedException` | Indicates to the service that the specified transaction is still active and should not be treated as idle and aborted. Write transactions that remain idle for a long period are automatically aborted unless explicitly ... |
| `GetDataCellsFilter` | `POST /GetDataCellsFilter` | - | `TableCatalogId`, `DatabaseName`, `TableName`, `Name` | - | `GetDataCellsFilterResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns a data cells filter. |
| `GetDataLakePrincipal` | `POST /GetDataLakePrincipal` | - | - | - | `GetDataLakePrincipalResponse` | `AccessDeniedException`, `InternalServiceException`, `OperationTimeoutException` | Returns the identity of the invoking principal. |
| `GetDataLakeSettings` | `POST /GetDataLakeSettings` | - | - | - | `GetDataLakeSettingsResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException` | Retrieves the list of the data lake administrators of a Lake Formation-managed data lake. |
| `GetEffectivePermissionsForPath` | `POST /GetEffectivePermissionsForPath` | `paginated` | `ResourceArn` | - | `GetEffectivePermissionsForPathResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns the Lake Formation permissions for a specified table or database resource located at a path in Amazon S3. GetEffectivePermissionsForPath will not return databases and tables if the catalog is encrypted. |
| `GetLFTag` | `POST /GetLFTag` | - | `TagKey` | - | `GetLFTagResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns an LF-tag definition. |
| `GetLFTagExpression` | `POST /GetLFTagExpression` | - | `Name` | - | `GetLFTagExpressionResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns the details about the LF-Tag expression. The caller must be a data lake admin or must have DESCRIBE permission on the LF-Tag expression resource. |
| `GetQueryState` | `POST /GetQueryState` | - | `QueryId` | - | `GetQueryStateResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidInputException` | Returns the state of a query previously submitted. Clients are expected to poll GetQueryState to monitor the current state of the planning before retrieving the work units. A query state is only visible to the princi ... |
| `GetQueryStatistics` | `POST /GetQueryStatistics` | - | `QueryId` | - | `GetQueryStatisticsResponse` | `AccessDeniedException`, `ExpiredException`, `InternalServiceException`, `InvalidInputException`, `StatisticsNotReadyYetException`, `ThrottledException` | Retrieves statistics on the planning and execution of a query. |
| `GetResourceLFTags` | `POST /GetResourceLFTags` | - | `Resource` | - | `GetResourceLFTagsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `GlueEncryptionException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns the LF-tags applied to a resource. |
| `GetTableObjects` | `POST /GetTableObjects` | `paginated` | `DatabaseName`, `TableName` | - | `GetTableObjectsResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNotReadyException`, `TransactionCanceledException`, `TransactionCommittedException` | Returns the set of Amazon S3 objects that make up the specified governed table. A transaction ID or timestamp can be specified for time-travel queries. |
| `GetTemporaryDataLocationCredentials` | `POST /GetTemporaryDataLocationCredentials` | - | - | - | `GetTemporaryDataLocationCredentialsResponse` | `AccessDeniedException`, `ConflictException`, `EntityNotFoundException`, `GlueEncryptionException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Allows a user or application in a secure environment to access data in a specific Amazon S3 location registered with Lake Formation by providing temporary scoped credentials that are limited to the requested data loc ... |
| `GetTemporaryGluePartitionCredentials` | `POST /GetTemporaryGluePartitionCredentials` | - | `TableArn`, `Partition` | - | `GetTemporaryGluePartitionCredentialsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `PermissionTypeMismatchException` | This API is identical to GetTemporaryTableCredentials except that this is used when the target Data Catalog resource is of type Partition. Lake Formation restricts the permission of the vended credentials with the sa ... |
| `GetTemporaryGlueTableCredentials` | `POST /GetTemporaryGlueTableCredentials` | - | `TableArn` | - | `GetTemporaryGlueTableCredentialsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `PermissionTypeMismatchException` | Allows a caller in a secure environment to assume a role with permission to access Amazon S3. In order to vend such credentials, Lake Formation assumes the role associated with a registered location, for example an A ... |
| `GetWorkUnitResults` | `POST /GetWorkUnitResults` | - | `QueryId`, `WorkUnitId`, `WorkUnitToken` | - | `GetWorkUnitResultsResponse` | `AccessDeniedException`, `ExpiredException`, `InternalServiceException`, `InvalidInputException`, `ThrottledException` | Returns the work units resulting from the query. Work units can be executed in any order and in parallel. |
| `GetWorkUnits` | `POST /GetWorkUnits` | `paginated` | `QueryId` | - | `GetWorkUnitsResponse` | `AccessDeniedException`, `ExpiredException`, `InternalServiceException`, `InvalidInputException`, `WorkUnitsNotReadyYetException` | Retrieves the work units generated by the StartQueryPlanning operation. |
| `GrantPermissions` | `POST /GrantPermissions` | - | `Principal`, `Resource`, `Permissions` | - | `GrantPermissionsResponse` | `ConcurrentModificationException`, `EntityNotFoundException`, `InvalidInputException` | Grants permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. For information about permissions, see Security and Access Control to Metada ... |
| `ListDataCellsFilter` | `POST /ListDataCellsFilter` | `paginated` | - | - | `ListDataCellsFilterResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Lists all the data cell filters on a table. |
| `ListLakeFormationOptIns` | `POST /ListLakeFormationOptIns` | `paginated` | - | - | `ListLakeFormationOptInsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Retrieve the current list of resources and principals that are opt in to enforce Lake Formation permissions. |
| `ListLFTagExpressions` | `POST /ListLFTagExpressions` | `paginated` | - | - | `ListLFTagExpressionsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns the LF-Tag expressions in caller’s account filtered based on caller's permissions. Data Lake and read only admins implicitly can see all tag expressions in their account, else caller needs DESCRIBE permission ... |
| `ListLFTags` | `POST /ListLFTags` | `paginated` | - | - | `ListLFTagsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Lists LF-tags that the requester has permission to view. |
| `ListPermissions` | `POST /ListPermissions` | `paginated` | - | - | `ListPermissionsResponse` | `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns a list of the principal permissions on the resource, filtered by the permissions of the caller. For example, if you are granted an ALTER permission, you are able to see only the principal permissions for ALTE ... |
| `ListResources` | `POST /ListResources` | `paginated` | - | - | `ListResourcesResponse` | `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Lists the resources registered to be managed by the Data Catalog. |
| `ListTableStorageOptimizers` | `POST /ListTableStorageOptimizers` | `paginated` | `DatabaseName`, `TableName` | - | `ListTableStorageOptimizersResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException` | Returns the configuration of all storage optimizers associated with a specified table. |
| `ListTransactions` | `POST /ListTransactions` | `paginated` | - | - | `ListTransactionsResponse` | `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Returns metadata about transactions and their status. To prevent the response from growing indefinitely, only uncommitted transactions and those available for time-travel queries are returned. This operation can help ... |
| `PutDataLakeSettings` | `POST /PutDataLakeSettings` | - | `DataLakeSettings` | - | `PutDataLakeSettingsResponse` | `InternalServiceException`, `InvalidInputException` | Sets the list of data lake administrators who have admin privileges on all resources managed by Lake Formation. For more information on admin privileges, see Granting Lake Formation Permissions . This API replaces th ... |
| `RegisterResource` | `POST /RegisterResource` | - | `ResourceArn` | - | `RegisterResourceResponse` | `AccessDeniedException`, `AlreadyExistsException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNumberLimitExceededException` | Registers the resource as managed by the Data Catalog. To add or update data, Lake Formation needs read/write access to the chosen data location. Choose a role that you know has permission to do this, or choose the A ... |
| `RemoveLFTagsFromResource` | `POST /RemoveLFTagsFromResource` | - | `Resource`, `LFTags` | - | `RemoveLFTagsFromResourceResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `GlueEncryptionException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Removes an LF-tag from the resource. Only database, table, or tableWithColumns resource are allowed. To tag columns, use the column inclusion list in tableWithColumns to specify column input. |
| `RevokePermissions` | `POST /RevokePermissions` | - | `Principal`, `Resource`, `Permissions` | - | `RevokePermissionsResponse` | `ConcurrentModificationException`, `EntityNotFoundException`, `InvalidInputException` | Revokes permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. |
| `SearchDatabasesByLFTags` | `POST /SearchDatabasesByLFTags` | `paginated` | `Expression` | - | `SearchDatabasesByLFTagsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `GlueEncryptionException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | This operation allows a search on DATABASE resources by TagCondition . This operation is used by admins who want to grant user permissions on certain TagConditions . Before making a grant, the admin can use SearchDat ... |
| `SearchTablesByLFTags` | `POST /SearchTablesByLFTags` | `paginated` | `Expression` | - | `SearchTablesByLFTagsResponse` | `AccessDeniedException`, `EntityNotFoundException`, `GlueEncryptionException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | This operation allows a search on TABLE resources by LFTag s. This will be used by admins who want to grant user permissions on certain LF-tags. Before making a grant, the admin can use SearchTablesByLFTags to find a ... |
| `StartQueryPlanning` | `POST /StartQueryPlanning` | - | `QueryPlanningContext`, `QueryString` | - | `StartQueryPlanningResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidInputException`, `ThrottledException` | Submits a request to process a query statement. This operation generates work units that can be retrieved with the GetWorkUnits operation as soon as the query state is WORKUNITS_AVAILABLE or FINISHED. |
| `StartTransaction` | `POST /StartTransaction` | - | - | - | `StartTransactionResponse` | `InternalServiceException`, `OperationTimeoutException` | Starts a new transaction and returns its transaction ID. Transaction IDs are opaque objects that you can use to identify a transaction. |
| `UpdateDataCellsFilter` | `POST /UpdateDataCellsFilter` | - | `TableData` | - | `UpdateDataCellsFilterResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Updates a data cell filter. |
| `UpdateLakeFormationIdentityCenterConfiguration` | `POST /UpdateLakeFormationIdentityCenterConfiguration` | - | - | - | `UpdateLakeFormationIdentityCenterConfigurationResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Updates the IAM Identity Center connection parameters. |
| `UpdateLFTag` | `POST /UpdateLFTag` | - | `TagKey` | - | `UpdateLFTagResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Updates the list of possible values for the specified LF-tag key. If the LF-tag does not exist, the operation throws an EntityNotFoundException. The values in the delete key values will be deleted from list of possib ... |
| `UpdateLFTagExpression` | `POST /UpdateLFTagExpression` | - | `Name`, `Expression` | - | `UpdateLFTagExpressionResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNumberLimitExceededException` | Updates the name of the LF-Tag expression to the new description and expression body provided. Updating a LF-Tag expression immediately changes the permission boundaries of all existing LFTagPolicy permission grants ... |
| `UpdateResource` | `POST /UpdateResource` | - | `RoleArn`, `ResourceArn` | - | `UpdateResourceResponse` | `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException` | Updates the data access role used for vending access to the given (registered) resource in Lake Formation. |
| `UpdateTableObjects` | `POST /UpdateTableObjects` | - | `DatabaseName`, `TableName`, `WriteOperations` | - | `UpdateTableObjectsResponse` | `ConcurrentModificationException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException`, `OperationTimeoutException`, `ResourceNotReadyException`, `TransactionCanceledException`, `TransactionCommitInProgressException`, `TransactionCommittedException` | Updates the manifest of Amazon S3 objects that make up the specified governed table. |
| `UpdateTableStorageOptimizer` | `POST /UpdateTableStorageOptimizer` | - | `DatabaseName`, `TableName`, `StorageOptimizerConfig` | - | `UpdateTableStorageOptimizerResponse` | `AccessDeniedException`, `EntityNotFoundException`, `InternalServiceException`, `InvalidInputException` | Updates the configuration of the storage optimizers for a table. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Access to a resource was denied. |
| `AlreadyExistsException` | `structure` | Message | A resource to be created or added already exists. |
| `ConcurrentModificationException` | `structure` | Message | Two processes are trying to modify a resource simultaneously. |
| `ConflictException` | `structure` | Message | Multiple resources exist with the same Amazon S3 location |
| `EntityNotFoundException` | `structure` | Message | A specified entity does not exist. |
| `ExpiredException` | `structure` | Message | Contains details about an error where the query request expired. |
| `GlueEncryptionException` | `structure` | Message | An encryption operation failed. |
| `InternalServiceException` | `structure` | Message | An internal service error occurred. |
| `InvalidInputException` | `structure` | Message | The input provided was not valid. |
| `OperationTimeoutException` | `structure` | Message | The operation timed out. |
| `PermissionTypeMismatchException` | `structure` | Message | The engine does not support filtering data based on the enforced permissions. For example, if you call the GetTemporaryGlueTableCredentials operation with S ... |
| `ResourceNotReadyException` | `structure` | Message | Contains details about an error related to a resource which is not ready for a transaction. |
| `ResourceNumberLimitExceededException` | `structure` | Message | A resource numerical limit was exceeded. |
| `StatisticsNotReadyYetException` | `structure` | Message | Contains details about an error related to statistics not being ready. |
| `ThrottledException` | `structure` | Message | Contains details about an error where the query request was throttled. |
| `TransactionCanceledException` | `structure` | Message | Contains details about an error related to a transaction that was cancelled. |
| `TransactionCommitInProgressException` | `structure` | Message | Contains details about an error related to a transaction commit that was in progress. |
| `TransactionCommittedException` | `structure` | Message | Contains details about an error where the specified transaction has already been committed and cannot be used for UpdateTableObjects . |
| `WorkUnitsNotReadyYetException` | `structure` | Message | Contains details about an error related to work units not being ready. |
| `AddLFTagsToResourceRequest` | `structure` | CatalogId, Resource, LFTags | - |
| `AddLFTagsToResourceResponse` | `structure` | Failures | - |
| `AssumeDecoratedRoleWithSAMLRequest` | `structure` | SAMLAssertion, RoleArn, PrincipalArn, DurationSeconds | - |
| `AssumeDecoratedRoleWithSAMLResponse` | `structure` | AccessKeyId, SecretAccessKey, SessionToken, Expiration | - |
| `BatchGrantPermissionsRequest` | `structure` | CatalogId, Entries | - |
| `BatchGrantPermissionsResponse` | `structure` | Failures | - |
| `BatchRevokePermissionsRequest` | `structure` | CatalogId, Entries | - |
| `BatchRevokePermissionsResponse` | `structure` | Failures | - |
| `CancelTransactionRequest` | `structure` | TransactionId | - |
| `CancelTransactionResponse` | `structure` | **empty (no members)** | - |
| `CommitTransactionRequest` | `structure` | TransactionId | - |
| `CommitTransactionResponse` | `structure` | TransactionStatus | - |
| `CreateDataCellsFilterRequest` | `structure` | TableData | - |
| `CreateDataCellsFilterResponse` | `structure` | **empty (no members)** | - |
| `CreateLakeFormationIdentityCenterConfigurationRequest` | `structure` | CatalogId, InstanceArn, ExternalFiltering, ShareRecipients, ServiceIntegrations | - |
| `CreateLakeFormationIdentityCenterConfigurationResponse` | `structure` | ApplicationArn | - |
| `CreateLakeFormationOptInRequest` | `structure` | Principal, Resource, Condition | - |
| `CreateLakeFormationOptInResponse` | `structure` | **empty (no members)** | - |
| `CreateLFTagRequest` | `structure` | CatalogId, TagKey, TagValues | - |
| `CreateLFTagResponse` | `structure` | **empty (no members)** | - |
| `CreateLFTagExpressionRequest` | `structure` | Name, Description, CatalogId, Expression | - |
| `ApplicationStatus` | `enum` | ENABLED, DISABLED | - |
| `ComparisonOperator` | `enum` | EQ, NE, LE, LT, GE, GT, CONTAINS, NOT_CONTAINS, BEGINS_WITH, IN, BETWEEN | - |
| `CredentialsScope` | `enum` | READ, READWRITE | - |
| `DataLakeResourceType` | `enum` | CATALOG, DATABASE, TABLE, DATA_LOCATION, LF_TAG, LF_TAG_POLICY, LF_TAG_POLICY_DATABASE, LF_TAG_POLICY_TABLE, LF_NAMED_TAG_EXPRESSION | - |
| `EnableStatus` | `enum` | ENABLED, DISABLED | - |
| `FieldNameString` | `enum` | RESOURCE_ARN, ROLE_ARN, LAST_MODIFIED | - |
| `OptimizerType` | `enum` | COMPACTION, GARBAGE_COLLECTION, GENERIC | - |
| `Permission` | `enum` | ALL, SELECT, ALTER, DROP, DELETE, INSERT, DESCRIBE, CREATE_DATABASE, CREATE_TABLE, DATA_LOCATION_ACCESS, CREATE_LF_TAG, ASSOCIATE, ... (+4) | - |
| `PermissionType` | `enum` | COLUMN_PERMISSION, CELL_FILTER_PERMISSION, NESTED_PERMISSION, NESTED_CELL_PERMISSION | - |
| `QueryStateString` | `enum` | PENDING, WORKUNITS_AVAILABLE, ERROR, FINISHED, EXPIRED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

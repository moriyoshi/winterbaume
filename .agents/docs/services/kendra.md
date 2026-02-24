# AWSKendraFrontendService

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Kendra is a service for indexing large document sets.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWSKendraFrontendService where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWSKendraFrontendService by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWSKendraFrontendService resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWSKendraFrontendService workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Update` operation families, including `ListAccessControlConfigurations`, `ListDataSourceSyncJobs`, `ListDataSources`, `ListEntityPersonas`, `DescribeAccessControlConfiguration`, `DescribeDataSource`.

## Service Identity and Protocol

- AWS model slug: `kendra`
- AWS SDK for Rust slug: `kendra`
- Model version: `2019-02-03`
- Model file: `vendor/api-models-aws/models/kendra/service/2019-02-03/kendra-2019-02-03.json`
- SDK ID: `kendra`
- Endpoint prefix: `kendra`
- ARN namespace: `kendra`
- CloudFormation name: `Kendra`
- CloudTrail event source: `kendra.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (13), `Describe` (10), `Create` (8), `Delete` (8), `Update` (8), `Batch` (4), `Associate` (2), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateEntitiesToExperience`, `AssociatePersonasToEntities`, `BatchDeleteDocument`, `BatchDeleteFeaturedResultsSet`, `BatchGetDocumentStatus`, `BatchPutDocument`, `CreateAccessControlConfiguration`, `CreateDataSource`, `CreateExperience`, `CreateFaq`, `CreateFeaturedResultsSet`, `CreateIndex`, `CreateQuerySuggestionsBlockList`, `CreateThesaurus`, `DeleteAccessControlConfiguration`, `DeleteDataSource`, `DeleteExperience`, `DeleteFaq`, `DeleteIndex`, `DeletePrincipalMapping`, ... (+18).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccessControlConfiguration`, `DescribeDataSource`, `DescribeExperience`, `DescribeFaq`, `DescribeFeaturedResultsSet`, `DescribeIndex`, `DescribePrincipalMapping`, `DescribeQuerySuggestionsBlockList`, `DescribeQuerySuggestionsConfig`, `DescribeThesaurus`, `GetQuerySuggestions`, `GetSnapshots`, `ListAccessControlConfigurations`, `ListDataSourceSyncJobs`, `ListDataSources`, `ListEntityPersonas`, `ListExperienceEntities`, `ListExperiences`, `ListFaqs`, `ListFeaturedResultsSets`, ... (+5).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ListDataSourceSyncJobs`, `StartDataSourceSyncJob`, `StopDataSourceSyncJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 66 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `Lambda`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/kendra/latest/dg/what-is-kendra.html
- https://docs.aws.amazon.com/kendra/latest/dg/hiw-index-types.html
- https://docs.aws.amazon.com/kendra/latest/dg/tuning.html

Research outcomes:
- Amazon Kendra is an intelligent enterprise search service that indexes content and answers natural-language queries.
- Indexes can be Developer, Enterprise, or GenAI Enterprise edition, with different capabilities and intended workloads.
- Data sources crawl external repositories and can carry access-control metadata for user-context filtering.
- FAQs, documents, synonyms, query suggestions, featured results, and relevance tuning can influence query responses.
- Relevance tuning can be configured at the index level or overridden at query time for supported fields.
- Search analytics expose query and usage metrics through snapshots.
- Kendra supports Retrieve and Query-style operations for search and retrieval use cases, including RAG integrations.

Parity implications:
- Model indexes, editions, data sources, sync jobs, documents, FAQs, access-control lists, query suggestions, synonyms, and relevance tuning separately.
- Query responses should depend on indexed document state and user context.
- Data source sync should be asynchronous and should produce status and metrics independent of index creation.

## Operation Groups

### List

- Operations: `ListAccessControlConfigurations`, `ListDataSourceSyncJobs`, `ListDataSources`, `ListEntityPersonas`, `ListExperienceEntities`, `ListExperiences`, `ListFaqs`, `ListFeaturedResultsSets`, `ListGroupsOlderThanOrderingId`, `ListIndices`, `ListQuerySuggestionsBlockLists`, `ListTagsForResource`, `ListThesauri`
- Traits: `paginated` (11)
- Common required input members in this group: `Id`, `IndexId`, `OrderingId`, `ResourceARN`

### Describe

- Operations: `DescribeAccessControlConfiguration`, `DescribeDataSource`, `DescribeExperience`, `DescribeFaq`, `DescribeFeaturedResultsSet`, `DescribeIndex`, `DescribePrincipalMapping`, `DescribeQuerySuggestionsBlockList`, `DescribeQuerySuggestionsConfig`, `DescribeThesaurus`
- Common required input members in this group: `FeaturedResultsSetId`, `GroupId`, `Id`, `IndexId`

### Create

- Operations: `CreateAccessControlConfiguration`, `CreateDataSource`, `CreateExperience`, `CreateFaq`, `CreateFeaturedResultsSet`, `CreateIndex`, `CreateQuerySuggestionsBlockList`, `CreateThesaurus`
- Traits: `idempotency-token` (7)
- Common required input members in this group: `FeaturedResultsSetName`, `IndexId`, `Name`, `RoleArn`, `S3Path`, `SourceS3Path`, `Type`

### Delete

- Operations: `DeleteAccessControlConfiguration`, `DeleteDataSource`, `DeleteExperience`, `DeleteFaq`, `DeleteIndex`, `DeletePrincipalMapping`, `DeleteQuerySuggestionsBlockList`, `DeleteThesaurus`
- Common required input members in this group: `GroupId`, `Id`, `IndexId`

### Update

- Operations: `UpdateAccessControlConfiguration`, `UpdateDataSource`, `UpdateExperience`, `UpdateFeaturedResultsSet`, `UpdateIndex`, `UpdateQuerySuggestionsBlockList`, `UpdateQuerySuggestionsConfig`, `UpdateThesaurus`
- Common required input members in this group: `FeaturedResultsSetId`, `Id`, `IndexId`

### Batch

- Operations: `BatchDeleteDocument`, `BatchDeleteFeaturedResultsSet`, `BatchGetDocumentStatus`, `BatchPutDocument`
- Common required input members in this group: `DocumentIdList`, `DocumentInfoList`, `Documents`, `FeaturedResultsSetIds`, `IndexId`

### Associate

- Operations: `AssociateEntitiesToExperience`, `AssociatePersonasToEntities`
- Common required input members in this group: `EntityList`, `Id`, `IndexId`, `Personas`

### Disassociate

- Operations: `DisassociateEntitiesFromExperience`, `DisassociatePersonasFromEntities`
- Common required input members in this group: `EntityIds`, `EntityList`, `Id`, `IndexId`

### Get

- Operations: `GetQuerySuggestions`, `GetSnapshots`
- Traits: `paginated` (1)
- Common required input members in this group: `IndexId`, `Interval`, `MetricType`, `QueryText`

### Clear

- Operations: `ClearQuerySuggestions`
- Common required input members in this group: `IndexId`

### Put

- Operations: `PutPrincipalMapping`
- Common required input members in this group: `GroupId`, `GroupMembers`, `IndexId`

### Query

- Operations: `Query`
- Common required input members in this group: `IndexId`

### Retrieve

- Operations: `Retrieve`
- Common required input members in this group: `IndexId`, `QueryText`

### Start

- Operations: `StartDataSourceSyncJob`
- Common required input members in this group: `Id`, `IndexId`

### Stop

- Operations: `StopDataSourceSyncJob`
- Common required input members in this group: `Id`, `IndexId`

### Submit

- Operations: `SubmitFeedback`
- Common required input members in this group: `IndexId`, `QueryId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateEntitiesToExperience` | - | - | `EntityList`, `Id`, `IndexId` | - | `AssociateEntitiesToExperienceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceAlreadyExistException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Grants users or groups in your IAM Identity Center identity source access to your Amazon Kendra experience. You can create an Amazon Kendra experience such as a search application. |
| `AssociatePersonasToEntities` | - | - | `Id`, `IndexId`, `Personas` | - | `AssociatePersonasToEntitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceAlreadyExistException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Defines the specific permissions of users or groups in your IAM Identity Center identity source with access to your Amazon Kendra experience. You can create an Amazon Kendra experience such as a search application. |
| `BatchDeleteDocument` | - | - | `DocumentIdList`, `IndexId` | - | `BatchDeleteDocumentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more documents from an index. The documents must have been added with the `BatchPutDocument` API. |
| `BatchDeleteFeaturedResultsSet` | - | - | `FeaturedResultsSetIds`, `IndexId` | - | `BatchDeleteFeaturedResultsSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more sets of featured results. Features results are placed above all other results for certain queries. |
| `BatchGetDocumentStatus` | - | - | `DocumentInfoList`, `IndexId` | - | `BatchGetDocumentStatusResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the indexing status for one or more documents submitted with the BatchPutDocument API. When you use the `BatchPutDocument` API, documents are indexed asynchronously. |
| `BatchPutDocument` | - | - | `Documents`, `IndexId` | - | `BatchPutDocumentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds one or more documents to an index. The `BatchPutDocument` API enables you to ingest inline documents or a set of documents stored in an Amazon S3 bucket. |
| `ClearQuerySuggestions` | - | - | `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Clears existing query suggestions from an index. This deletes existing suggestions only, not the queries in the query log. |
| `CreateAccessControlConfiguration` | - | `idempotency-token` | `IndexId`, `Name` | `ClientToken` | `CreateAccessControlConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an access configuration for your documents. This includes user and group access information for your documents. |
| `CreateDataSource` | - | `idempotency-token` | `IndexId`, `Name`, `Type` | `ClientToken` | `CreateDataSourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceAlreadyExistException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a data source connector that you want to use with an Amazon Kendra index. You specify a name, data source connector type and description for your data source. |
| `CreateExperience` | - | `idempotency-token` | `IndexId`, `Name` | `ClientToken` | `CreateExperienceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Kendra experience such as a search application. For more information on creating a search application experience, including using the Python and Java SDKs, see Building a search experience with no code. |
| `CreateFaq` | - | `idempotency-token` | `IndexId`, `Name`, `RoleArn`, `S3Path` | `ClientToken` | `CreateFaqResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a set of frequently ask questions (FAQs) using a specified FAQ file stored in an Amazon S3 bucket. Adding FAQs to an index is an asynchronous operation. |
| `CreateFeaturedResultsSet` | - | - | `FeaturedResultsSetName`, `IndexId` | - | `CreateFeaturedResultsSetResponse` | `AccessDeniedException`, `ConflictException`, `FeaturedResultsConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a set of featured results to display at the top of the search results page. Featured results are placed above all other results for certain queries. |
| `CreateIndex` | - | `idempotency-token` | `Name`, `RoleArn` | `ClientToken` | `CreateIndexResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceAlreadyExistException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Kendra index. Index creation is an asynchronous API. |
| `CreateQuerySuggestionsBlockList` | - | `idempotency-token` | `IndexId`, `Name`, `RoleArn`, `SourceS3Path` | `ClientToken` | `CreateQuerySuggestionsBlockListResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a block list to exlcude certain queries from suggestions. Any query that contains words or phrases specified in the block list is blocked or filtered out from being shown as a suggestion. |
| `CreateThesaurus` | - | `idempotency-token` | `IndexId`, `Name`, `RoleArn`, `SourceS3Path` | `ClientToken` | `CreateThesaurusResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a thesaurus for an index. The thesaurus contains a list of synonyms in Solr format. |
| `DeleteAccessControlConfiguration` | - | - | `Id`, `IndexId` | - | `DeleteAccessControlConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an access control configuration that you created for your documents in an index. This includes user and group access information for your documents. |
| `DeleteDataSource` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Kendra data source connector. An exception is not thrown if the data source is already being deleted. |
| `DeleteExperience` | - | - | `Id`, `IndexId` | - | `DeleteExperienceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes your Amazon Kendra experience such as a search application. For more information on creating a search application experience, see Building a search experience with no code. |
| `DeleteFaq` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a FAQ from an index. |
| `DeleteIndex` | - | - | `Id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Kendra index. An exception is not thrown if the index is already being deleted. |
| `DeletePrincipalMapping` | - | - | `GroupId`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a group so that all users that belong to the group can no longer access documents only available to that group. For example, after deleting the group "Summer Interns", all interns who belonged to that group no longer see intern-only documents in their... |
| `DeleteQuerySuggestionsBlockList` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a block list used for query suggestions for an index. A deleted block list might not take effect right away. |
| `DeleteThesaurus` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Kendra thesaurus. |
| `DescribeAccessControlConfiguration` | - | - | `Id`, `IndexId` | - | `DescribeAccessControlConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an access control configuration that you created for your documents in an index. This includes user and group access information for your documents. |
| `DescribeDataSource` | - | - | `Id`, `IndexId` | - | `DescribeDataSourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an Amazon Kendra data source connector. |
| `DescribeExperience` | - | - | `Id`, `IndexId` | - | `DescribeExperienceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about your Amazon Kendra experience such as a search application. For more information on creating a search application experience, see Building a search experience with no code. |
| `DescribeFaq` | - | - | `Id`, `IndexId` | - | `DescribeFaqResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a FAQ. |
| `DescribeFeaturedResultsSet` | - | - | `FeaturedResultsSetId`, `IndexId` | - | `DescribeFeaturedResultsSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a set of featured results. Features results are placed above all other results for certain queries. |
| `DescribeIndex` | - | - | `Id` | - | `DescribeIndexResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an Amazon Kendra index. |
| `DescribePrincipalMapping` | - | - | `GroupId`, `IndexId` | - | `DescribePrincipalMappingResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the processing of `PUT` and `DELETE` actions for mapping users to their groups. This includes information on the status of actions currently processing or yet to be processed, when actions were last updated, when actions were received by Amazon... |
| `DescribeQuerySuggestionsBlockList` | - | - | `Id`, `IndexId` | - | `DescribeQuerySuggestionsBlockListResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a block list used for query suggestions for an index. This is used to check the current settings that are applied to a block list. |
| `DescribeQuerySuggestionsConfig` | - | - | `IndexId` | - | `DescribeQuerySuggestionsConfigResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information on the settings of query suggestions for an index. This is used to check the current settings applied to query suggestions. |
| `DescribeThesaurus` | - | - | `Id`, `IndexId` | - | `DescribeThesaurusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an Amazon Kendra thesaurus. |
| `DisassociateEntitiesFromExperience` | - | - | `EntityList`, `Id`, `IndexId` | - | `DisassociateEntitiesFromExperienceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Prevents users or groups in your IAM Identity Center identity source from accessing your Amazon Kendra experience. You can create an Amazon Kendra experience such as a search application. |
| `DisassociatePersonasFromEntities` | - | - | `EntityIds`, `Id`, `IndexId` | - | `DisassociatePersonasFromEntitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specific permissions of users or groups in your IAM Identity Center identity source with access to your Amazon Kendra experience. You can create an Amazon Kendra experience such as a search application. |
| `GetQuerySuggestions` | - | - | `IndexId`, `QueryText` | - | `GetQuerySuggestionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Fetches the queries that are suggested to your users. `GetQuerySuggestions` is currently not supported in the Amazon Web Services GovCloud (US-West) region. |
| `GetSnapshots` | - | `paginated` | `IndexId`, `Interval`, `MetricType` | - | `GetSnapshotsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidRequestException`, `ResourceNotFoundException` | Retrieves search metrics data. The data provides a snapshot of how your users interact with your search application and how effective the application is. |
| `ListAccessControlConfigurations` | - | `paginated` | `IndexId` | - | `ListAccessControlConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists one or more access control configurations for an index. This includes user and group access information for your documents. |
| `ListDataSourceSyncJobs` | - | `paginated` | `Id`, `IndexId` | - | `ListDataSourceSyncJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets statistics about synchronizing a data source connector. |
| `ListDataSources` | - | `paginated` | `IndexId` | - | `ListDataSourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the data source connectors that you have created. |
| `ListEntityPersonas` | - | `paginated` | `Id`, `IndexId` | - | `ListEntityPersonasResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists specific permissions of users and groups with access to your Amazon Kendra experience. |
| `ListExperienceEntities` | - | `paginated` | `Id`, `IndexId` | - | `ListExperienceEntitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists users or groups in your IAM Identity Center identity source that are granted access to your Amazon Kendra experience. You can create an Amazon Kendra experience such as a search application. |
| `ListExperiences` | - | `paginated` | `IndexId` | - | `ListExperiencesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists one or more Amazon Kendra experiences. You can create an Amazon Kendra experience such as a search application. |
| `ListFaqs` | - | `paginated` | `IndexId` | - | `ListFaqsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a list of FAQs associated with an index. |
| `ListFeaturedResultsSets` | - | - | `IndexId` | - | `ListFeaturedResultsSetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all your sets of featured results for a given index. Features results are placed above all other results for certain queries. |
| `ListGroupsOlderThanOrderingId` | - | `paginated` | `IndexId`, `OrderingId` | - | `ListGroupsOlderThanOrderingIdResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a list of groups that are mapped to users before a given ordering or timestamp identifier. `ListGroupsOlderThanOrderingId` is currently not supported in the Amazon Web Services GovCloud (US-West) region. |
| `ListIndices` | - | `paginated` | - | - | `ListIndicesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the Amazon Kendra indexes that you created. |
| `ListQuerySuggestionsBlockLists` | - | `paginated` | `IndexId` | - | `ListQuerySuggestionsBlockListsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the block lists used for query suggestions for an index. For information on the current quota limits for block lists, see Quotas for Amazon Kendra. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets a list of tags associated with a resource. Indexes, FAQs, data sources, and other resources can have tags associated with them. |
| `ListThesauri` | - | `paginated` | `IndexId` | - | `ListThesauriResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the thesauri for an index. |
| `PutPrincipalMapping` | - | - | `GroupId`, `GroupMembers`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Maps users to their groups so that you only need to provide the user ID when you issue the query. You can also map sub groups to groups. |
| `Query` | - | - | `IndexId` | - | `QueryResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Searches an index given an input query. If you are working with large language models (LLMs) or implementing retrieval augmented generation (RAG) systems, you can use Amazon Kendra's Retrieve API, which can return longer semantically relevant passages. |
| `Retrieve` | - | - | `IndexId`, `QueryText` | - | `RetrieveResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves relevant passages or text excerpts given an input query. This API is similar to the Query API. |
| `StartDataSourceSyncJob` | - | - | `Id`, `IndexId` | - | `StartDataSourceSyncJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a synchronization job for a data source connector. If a synchronization job is already in progress, Amazon Kendra returns a `ResourceInUseException` exception. |
| `StopDataSourceSyncJob` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a synchronization job that is currently running. You can't stop a scheduled synchronization job. |
| `SubmitFeedback` | - | - | `IndexId`, `QueryId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Enables you to provide feedback to Amazon Kendra to improve the performance of your index. `SubmitFeedback` is currently not supported in the Amazon Web Services GovCloud (US-West) region. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Adds the specified tag to the specified index, FAQ, data source, or other resource. If the tag already exists, the existing value is replaced with the new value. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceUnavailableException`, `ThrottlingException`, `ValidationException` | Removes a tag from an index, FAQ, data source, or other resource. |
| `UpdateAccessControlConfiguration` | - | - | `Id`, `IndexId` | - | `UpdateAccessControlConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an access control configuration for your documents in an index. This includes user and group access information for your documents. |
| `UpdateDataSource` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an Amazon Kendra data source connector. |
| `UpdateExperience` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates your Amazon Kendra experience such as a search application. For more information on creating a search application experience, see Building a search experience with no code. |
| `UpdateFeaturedResultsSet` | - | - | `FeaturedResultsSetId`, `IndexId` | - | `UpdateFeaturedResultsSetResponse` | `AccessDeniedException`, `FeaturedResultsConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a set of featured results. Features results are placed above all other results for certain queries. |
| `UpdateIndex` | - | - | `Id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an Amazon Kendra index. |
| `UpdateQuerySuggestionsBlockList` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a block list used for query suggestions for an index. Updates to a block list might not take effect right away. |
| `UpdateQuerySuggestionsConfig` | - | - | `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the settings of query suggestions for an index. Amazon Kendra supports partial updates, so you only need to provide the fields you want to update. |
| `UpdateThesaurus` | - | - | `Id`, `IndexId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a thesaurus for an index. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | An issue occurred with the internal server used for your Amazon Kendra service. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the constraints set by the Amazon Kendra service. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource you want to use doesn’t exist. |
| `ConflictException` | `structure` | `Message` | A conflict occurred with the request. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You have exceeded the set limits for your Amazon Kendra service. |
| `ResourceAlreadyExistException` | `structure` | `Message` | The resource you want to use already exists. |
| `ResourceUnavailableException` | `structure` | `Message` | The resource you want to use isn't available. |
| `FeaturedResultsConflictException` | `structure` | `ConflictingItems`, `Message` | An error message with a list of conflicting queries used across different sets of featured results. |
| `AssociateEntitiesToExperienceRequest` | `structure` | `EntityList`, `Id`, `IndexId` | - |
| `AssociateEntitiesToExperienceResponse` | `structure` | `FailedEntityList` | - |
| `AssociatePersonasToEntitiesRequest` | `structure` | `Id`, `IndexId`, `Personas` | - |
| `AssociatePersonasToEntitiesResponse` | `structure` | `FailedEntityList` | - |
| `BatchDeleteDocumentRequest` | `structure` | `DataSourceSyncJobMetricTarget`, `DocumentIdList`, `IndexId` | - |
| `BatchDeleteDocumentResponse` | `structure` | `FailedDocuments` | - |
| `BatchDeleteFeaturedResultsSetRequest` | `structure` | `FeaturedResultsSetIds`, `IndexId` | - |
| `BatchDeleteFeaturedResultsSetResponse` | `structure` | `Errors` | - |
| `BatchGetDocumentStatusRequest` | `structure` | `DocumentInfoList`, `IndexId` | - |
| `BatchGetDocumentStatusResponse` | `structure` | `DocumentStatusList`, `Errors` | - |
| `BatchPutDocumentRequest` | `structure` | `CustomDocumentEnrichmentConfiguration`, `Documents`, `IndexId`, `RoleArn` | - |
| `BatchPutDocumentResponse` | `structure` | `FailedDocuments` | - |
| `ClearQuerySuggestionsRequest` | `structure` | `IndexId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

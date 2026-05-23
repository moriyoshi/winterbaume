# AWS AppSync

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AppSync provides API actions for creating and interacting with data sources using GraphQL from your application.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS AppSync where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS AppSync by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: create GraphQL APIs, schemas, data sources, resolvers, functions, API keys, domains, and caching/logging configuration.
- From the operation surface: model API-backed application data access using DynamoDB, Lambda, OpenSearch, HTTP, EventBridge, and merged or source APIs.

## Service Identity and Protocol

- AWS model slug: `appsync`
- AWS SDK for Rust slug: `appsync`
- Model version: `2017-07-25`
- Model file: `vendor/api-models-aws/models/appsync/service/2017-07-25/appsync-2017-07-25.json`
- SDK ID: `AppSync`
- Endpoint prefix: `appsync`
- ARN namespace: `appsync`
- CloudFormation name: `AppSync`
- CloudTrail event source: `appsync.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (15), `List` (13), `Update` (11), `Create` (10), `Delete` (10), `Associate` (3), `Disassociate` (3), `Start` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateApi`, `AssociateMergedGraphqlApi`, `AssociateSourceGraphqlApi`, `CreateApi`, `CreateApiCache`, `CreateApiKey`, `CreateChannelNamespace`, `CreateDataSource`, `CreateDomainName`, `CreateFunction`, `CreateGraphqlApi`, `CreateResolver`, `CreateType`, `DeleteApi`, `DeleteApiCache`, `DeleteApiKey`, `DeleteChannelNamespace`, `DeleteDataSource`, `DeleteDomainName`, `DeleteFunction`, ... (+23).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApi`, `GetApiAssociation`, `GetApiCache`, `GetChannelNamespace`, `GetDataSource`, `GetDataSourceIntrospection`, `GetDomainName`, `GetFunction`, `GetGraphqlApi`, `GetGraphqlApiEnvironmentVariables`, `GetIntrospectionSchema`, `GetResolver`, `GetSchemaCreationStatus`, `GetSourceApiAssociation`, `GetType`, `ListApiKeys`, `ListApis`, `ListChannelNamespaces`, `ListDataSources`, `ListDomainNames`, ... (+8).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDataSourceIntrospection`, `StartSchemaCreation`, `StartSchemaMerge`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 74 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `Lambda`, `ECS`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/appsync/latest/devguide/resolver-components.html
- https://docs.aws.amazon.com/appsync/latest/devguide/pipeline-resolvers.html
- https://docs.aws.amazon.com/appsync/latest/devguide/security-authorization-use-cases.html

Research outcomes:
- AppSync resolvers can use APPSYNC_JS or Velocity Template Language runtime.
- Unit resolvers contain request and response handlers executed against one data source.
- Pipeline resolvers execute a before step, then functions serially in configured order, then an after step.
- Resolver context carries request arguments, identity, result data, errors, request metadata, and state from previous pipeline steps.
- GraphQL requests are parsed, validated against the schema, traversed, and resolved field by field.
- AppSync threads identity information into resolver context, so fine-grained access control can be implemented by resolver logic using data-source metadata such as owners or groups.
- AppSync itself does not store application data; authorization metadata for row or item-level access must live in the backing data source.
- Local or None data source resolvers can produce responses or pub/sub behaviour without calling an external data source.

Parity implications:
- Model APIs, schemas, data sources, resolvers, functions, runtimes, authorisation modes, API keys, and tracing/caching settings separately.
- Request execution needs schema validation, resolver lookup, context construction, serial pipeline execution, error propagation, and data-source invocation semantics.
- Authorisation should expose identity in resolver context and support resolver-level access decisions rather than only coarse API-level allow/deny.

## Cross-Service Integration Gaps

- **`appsync-resolvers`** ( primary ): AppSync implements only control-plane operations ( API management, schema creation, data source and resolver CRUD ). No resolver execution or data-source invocation exists for DynamoDB, Lambda, OpenSearch, RDS Data API, HTTP endpoints, or EventBridge data sources. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `appsync-resolvers` ).

## Operation Groups

### Get

- Operations: `GetApi`, `GetApiAssociation`, `GetApiCache`, `GetChannelNamespace`, `GetDataSource`, `GetDataSourceIntrospection`, `GetDomainName`, `GetFunction`, `GetGraphqlApi`, `GetGraphqlApiEnvironmentVariables`, `GetIntrospectionSchema`, `GetResolver`, `GetSchemaCreationStatus`, `GetSourceApiAssociation`, `GetType`
- Common required input members in this group: `apiId`, `domainName`, `name`, `format`, `typeName`

### List

- Operations: `ListApiKeys`, `ListApis`, `ListChannelNamespaces`, `ListDataSources`, `ListDomainNames`, `ListFunctions`, `ListGraphqlApis`, `ListResolvers`, `ListResolversByFunction`, `ListSourceApiAssociations`, `ListTagsForResource`, `ListTypes`, `ListTypesByAssociation`
- Traits: `paginated` (12)
- Common required input members in this group: `apiId`, `format`

### Update

- Operations: `UpdateApi`, `UpdateApiCache`, `UpdateApiKey`, `UpdateChannelNamespace`, `UpdateDataSource`, `UpdateDomainName`, `UpdateFunction`, `UpdateGraphqlApi`, `UpdateResolver`, `UpdateSourceApiAssociation`, `UpdateType`
- Common required input members in this group: `apiId`, `name`, `type`, `typeName`

### Create

- Operations: `CreateApi`, `CreateApiCache`, `CreateApiKey`, `CreateChannelNamespace`, `CreateDataSource`, `CreateDomainName`, `CreateFunction`, `CreateGraphqlApi`, `CreateResolver`, `CreateType`
- Common required input members in this group: `name`, `apiId`, `type`

### Delete

- Operations: `DeleteApi`, `DeleteApiCache`, `DeleteApiKey`, `DeleteChannelNamespace`, `DeleteDataSource`, `DeleteDomainName`, `DeleteFunction`, `DeleteGraphqlApi`, `DeleteResolver`, `DeleteType`
- Traits: `idempotent` (2)
- Common required input members in this group: `apiId`, `name`, `typeName`

### Associate

- Operations: `AssociateApi`, `AssociateMergedGraphqlApi`, `AssociateSourceGraphqlApi`
- Common required input members in this group: `sourceApiIdentifier`, `mergedApiIdentifier`

### Disassociate

- Operations: `DisassociateApi`, `DisassociateMergedGraphqlApi`, `DisassociateSourceGraphqlApi`
- Common required input members in this group: `associationId`

### Start

- Operations: `StartDataSourceIntrospection`, `StartSchemaCreation`, `StartSchemaMerge`
- Common required input members in this group: -

### Evaluate

- Operations: `EvaluateCode`, `EvaluateMappingTemplate`
- Common required input members in this group: `context`

### Flush

- Operations: `FlushApiCache`
- Common required input members in this group: -

### Put

- Operations: `PutGraphqlApiEnvironmentVariables`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateApi` | `POST /v1/domainnames/{domainName}/apiassociation` | - | `domainName`, `apiId` | - | `AssociateApiResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException` | Maps an endpoint to your custom domain. |
| `AssociateMergedGraphqlApi` | `POST /v1/sourceApis/{sourceApiIdentifier}/mergedApiAssociations` | - | `sourceApiIdentifier`, `mergedApiIdentifier` | - | `AssociateMergedGraphqlApiResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates an association between a Merged API and source API using the source API's identifier. |
| `AssociateSourceGraphqlApi` | `POST /v1/mergedApis/{mergedApiIdentifier}/sourceApiAssociations` | - | `mergedApiIdentifier`, `sourceApiIdentifier` | - | `AssociateSourceGraphqlApiResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates an association between a Merged API and source API using the Merged API's identifier. |
| `CreateApi` | `POST /v2/apis` | - | `name`, `eventConfig` | - | `CreateApiResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `ServiceQuotaExceededException`, `UnauthorizedException` | Creates an Api object. Use this operation to create an AppSync API with your preferred configuration, such as an Event API that provides real-time message publishing and message subscriptions over WebSockets. |
| `CreateApiCache` | `POST /v1/apis/{apiId}/ApiCaches` | - | `apiId`, `ttl`, `apiCachingBehavior`, `type` | - | `CreateApiCacheResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a cache for the GraphQL API. |
| `CreateApiKey` | `POST /v1/apis/{apiId}/apikeys` | - | `apiId` | - | `CreateApiKeyResponse` | `ApiKeyLimitExceededException`, `ApiKeyValidityOutOfBoundsException`, `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Creates a unique key that you can distribute to clients who invoke your API. |
| `CreateChannelNamespace` | `POST /v2/apis/{apiId}/channelNamespaces` | - | `apiId`, `name` | - | `CreateChannelNamespaceResponse` | `BadRequestException`, `ConcurrentModificationException`, `ConflictException`, `InternalFailureException`, `NotFoundException`, `ServiceQuotaExceededException`, `UnauthorizedException` | Creates a ChannelNamespace for an Api . |
| `CreateDataSource` | `POST /v1/apis/{apiId}/datasources` | - | `apiId`, `name`, `type` | - | `CreateDataSourceResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a DataSource object. |
| `CreateDomainName` | `POST /v1/domainnames` | - | `domainName`, `certificateArn` | - | `CreateDomainNameResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException` | Creates a custom DomainName object. |
| `CreateFunction` | `POST /v1/apis/{apiId}/functions` | - | `apiId`, `name`, `dataSourceName` | - | `CreateFunctionResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a Function object. A function is a reusable entity. You can use multiple functions to compose the resolver logic. |
| `CreateGraphqlApi` | `POST /v1/apis` | - | `name`, `authenticationType` | - | `CreateGraphqlApiResponse` | `ApiLimitExceededException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `LimitExceededException`, `UnauthorizedException` | Creates a GraphqlApi object. |
| `CreateResolver` | `POST /v1/apis/{apiId}/types/{typeName}/resolvers` | - | `apiId`, `typeName`, `fieldName` | - | `CreateResolverResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a Resolver object. A resolver converts incoming requests into a format that a data source can understand, and converts the data source's responses into GraphQL. |
| `CreateType` | `POST /v1/apis/{apiId}/types` | - | `apiId`, `definition`, `format` | - | `CreateTypeResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a Type object. |
| `DeleteApi` | `DELETE /v2/apis/{apiId}` | `idempotent` | `apiId` | - | `DeleteApiResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes an Api object |
| `DeleteApiCache` | `DELETE /v1/apis/{apiId}/ApiCaches` | - | `apiId` | - | `DeleteApiCacheResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes an ApiCache object. |
| `DeleteApiKey` | `DELETE /v1/apis/{apiId}/apikeys/{id}` | - | `apiId`, `id` | - | `DeleteApiKeyResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes an API key. |
| `DeleteChannelNamespace` | `DELETE /v2/apis/{apiId}/channelNamespaces/{name}` | `idempotent` | `apiId`, `name` | - | `DeleteChannelNamespaceResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a ChannelNamespace . |
| `DeleteDataSource` | `DELETE /v1/apis/{apiId}/datasources/{name}` | - | `apiId`, `name` | - | `DeleteDataSourceResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a DataSource object. |
| `DeleteDomainName` | `DELETE /v1/domainnames/{domainName}` | - | `domainName` | - | `DeleteDomainNameResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException` | Deletes a custom DomainName object. |
| `DeleteFunction` | `DELETE /v1/apis/{apiId}/functions/{functionId}` | - | `apiId`, `functionId` | - | `DeleteFunctionResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a Function . |
| `DeleteGraphqlApi` | `DELETE /v1/apis/{apiId}` | - | `apiId` | - | `DeleteGraphqlApiResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a GraphqlApi object. |
| `DeleteResolver` | `DELETE /v1/apis/{apiId}/types/{typeName}/resolvers/{fieldName}` | - | `apiId`, `typeName`, `fieldName` | - | `DeleteResolverResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a Resolver object. |
| `DeleteType` | `DELETE /v1/apis/{apiId}/types/{typeName}` | - | `apiId`, `typeName` | - | `DeleteTypeResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes a Type object. |
| `DisassociateApi` | `DELETE /v1/domainnames/{domainName}/apiassociation` | - | `domainName` | - | `DisassociateApiResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException` | Removes an ApiAssociation object from a custom domain. |
| `DisassociateMergedGraphqlApi` | `DELETE /v1/sourceApis/{sourceApiIdentifier}/mergedApiAssociations/{associationId}` | - | `sourceApiIdentifier`, `associationId` | - | `DisassociateMergedGraphqlApiResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes an association between a Merged API and source API using the source API's identifier and the association ID. |
| `DisassociateSourceGraphqlApi` | `DELETE /v1/mergedApis/{mergedApiIdentifier}/sourceApiAssociations/{associationId}` | - | `mergedApiIdentifier`, `associationId` | - | `DisassociateSourceGraphqlApiResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Deletes an association between a Merged API and source API using the Merged API's identifier and the association ID. |
| `EvaluateCode` | `POST /v1/dataplane-evaluatecode` | - | `runtime`, `code`, `context` | - | `EvaluateCodeResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException` | Evaluates the given code and returns the response. The code definition requirements depend on the specified runtime. For APPSYNC_JS runtimes, the code defines the request and response functions. The request function ... |
| `EvaluateMappingTemplate` | `POST /v1/dataplane-evaluatetemplate` | - | `template`, `context` | - | `EvaluateMappingTemplateResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException` | Evaluates a given template and returns the response. The mapping template can be a request or response template. Request templates take the incoming request after a GraphQL operation is parsed and convert it into a r ... |
| `FlushApiCache` | `DELETE /v1/apis/{apiId}/FlushCache` | - | `apiId` | - | `FlushApiCacheResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Flushes an ApiCache object. |
| `GetApi` | `GET /v2/apis/{apiId}` | - | `apiId` | - | `GetApiResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves an Api object. |
| `GetApiAssociation` | `GET /v1/domainnames/{domainName}/apiassociation` | - | `domainName` | - | `GetApiAssociationResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException` | Retrieves an ApiAssociation object. |
| `GetApiCache` | `GET /v1/apis/{apiId}/ApiCaches` | - | `apiId` | - | `GetApiCacheResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves an ApiCache object. |
| `GetChannelNamespace` | `GET /v2/apis/{apiId}/channelNamespaces/{name}` | - | `apiId`, `name` | - | `GetChannelNamespaceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves the channel namespace for a specified Api . |
| `GetDataSource` | `GET /v1/apis/{apiId}/datasources/{name}` | - | `apiId`, `name` | - | `GetDataSourceResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves a DataSource object. |
| `GetDataSourceIntrospection` | `GET /v1/datasources/introspections/{introspectionId}` | - | `introspectionId` | - | `GetDataSourceIntrospectionResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException` | Retrieves the record of an existing introspection. If the retrieval is successful, the result of the instrospection will also be returned. If the retrieval fails the operation, an error message will be returned instead. |
| `GetDomainName` | `GET /v1/domainnames/{domainName}` | - | `domainName` | - | `GetDomainNameResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException` | Retrieves a custom DomainName object. |
| `GetFunction` | `GET /v1/apis/{apiId}/functions/{functionId}` | - | `apiId`, `functionId` | - | `GetFunctionResponse` | `ConcurrentModificationException`, `NotFoundException`, `UnauthorizedException` | Get a Function . |
| `GetGraphqlApi` | `GET /v1/apis/{apiId}` | - | `apiId` | - | `GetGraphqlApiResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves a GraphqlApi object. |
| `GetGraphqlApiEnvironmentVariables` | `GET /v1/apis/{apiId}/environmentVariables` | - | `apiId` | - | `GetGraphqlApiEnvironmentVariablesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves the list of environmental variable key-value pairs associated with an API by its ID value. |
| `GetIntrospectionSchema` | `GET /v1/apis/{apiId}/schema` | - | `apiId`, `format` | - | `GetIntrospectionSchemaResponse` | `GraphQLSchemaException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves the introspection schema for a GraphQL API. |
| `GetResolver` | `GET /v1/apis/{apiId}/types/{typeName}/resolvers/{fieldName}` | - | `apiId`, `typeName`, `fieldName` | - | `GetResolverResponse` | `ConcurrentModificationException`, `NotFoundException`, `UnauthorizedException` | Retrieves a Resolver object. |
| `GetSchemaCreationStatus` | `GET /v1/apis/{apiId}/schemacreation` | - | `apiId` | - | `GetSchemaCreationStatusResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves the current status of a schema creation operation. |
| `GetSourceApiAssociation` | `GET /v1/mergedApis/{mergedApiIdentifier}/sourceApiAssociations/{associationId}` | - | `mergedApiIdentifier`, `associationId` | - | `GetSourceApiAssociationResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves a SourceApiAssociation object. |
| `GetType` | `GET /v1/apis/{apiId}/types/{typeName}` | - | `apiId`, `typeName`, `format` | - | `GetTypeResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Retrieves a Type object. |
| `ListApiKeys` | `GET /v1/apis/{apiId}/apikeys` | `paginated` | `apiId` | - | `ListApiKeysResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists the API keys for a given API. API keys are deleted automatically 60 days after they expire. However, they may still be included in the response until they have actually been deleted. You can safely call DeleteA ... |
| `ListApis` | `GET /v2/apis` | `paginated` | - | - | `ListApisResponse` | `BadRequestException`, `InternalFailureException`, `UnauthorizedException` | Lists the APIs in your AppSync account. ListApis returns only the high level API details. For more detailed information about an API, use GetApi . |
| `ListChannelNamespaces` | `GET /v2/apis/{apiId}/channelNamespaces` | `paginated` | `apiId` | - | `ListChannelNamespacesResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists the channel namespaces for a specified Api . ListChannelNamespaces returns only high level details for the channel namespace. To retrieve code handlers, use GetChannelNamespace . |
| `ListDataSources` | `GET /v1/apis/{apiId}/datasources` | `paginated` | `apiId` | - | `ListDataSourcesResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists the data sources for a given API. |
| `ListDomainNames` | `GET /v1/domainnames` | `paginated` | - | - | `ListDomainNamesResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException` | Lists multiple custom domain names. |
| `ListFunctions` | `GET /v1/apis/{apiId}/functions` | `paginated` | `apiId` | - | `ListFunctionsResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | List multiple functions. |
| `ListGraphqlApis` | `GET /v1/apis` | `paginated` | - | - | `ListGraphqlApisResponse` | `BadRequestException`, `InternalFailureException`, `UnauthorizedException` | Lists your GraphQL APIs. |
| `ListResolvers` | `GET /v1/apis/{apiId}/types/{typeName}/resolvers` | `paginated` | `apiId`, `typeName` | - | `ListResolversResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists the resolvers for a given API and type. |
| `ListResolversByFunction` | `GET /v1/apis/{apiId}/functions/{functionId}/resolvers` | `paginated` | `apiId`, `functionId` | - | `ListResolversByFunctionResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | List the resolvers that are associated with a specific function. |
| `ListSourceApiAssociations` | `GET /v1/apis/{apiId}/sourceApiAssociations` | `paginated` | `apiId` | - | `ListSourceApiAssociationsResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists the SourceApiAssociationSummary data. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Lists the tags for a resource. |
| `ListTypes` | `GET /v1/apis/{apiId}/types` | `paginated` | `apiId`, `format` | - | `ListTypesResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists the types for a given API. |
| `ListTypesByAssociation` | `GET /v1/mergedApis/{mergedApiIdentifier}/sourceApiAssociations/{associationId}/types` | `paginated` | `mergedApiIdentifier`, `associationId`, `format` | - | `ListTypesByAssociationResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Lists Type objects by the source API association ID. |
| `PutGraphqlApiEnvironmentVariables` | `PUT /v1/apis/{apiId}/environmentVariables` | - | `apiId`, `environmentVariables` | - | `PutGraphqlApiEnvironmentVariablesResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a list of environmental variables in an API by its ID value. When creating an environmental variable, it must follow the constraints below: Both JavaScript and VTL templates support environmental variables. E ... |
| `StartDataSourceIntrospection` | `POST /v1/datasources/introspections` | - | - | - | `StartDataSourceIntrospectionResponse` | `BadRequestException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Creates a new introspection. Returns the introspectionId of the new introspection after its creation. |
| `StartSchemaCreation` | `POST /v1/apis/{apiId}/schemacreation` | - | `apiId`, `definition` | - | `StartSchemaCreationResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Adds a new schema to your GraphQL API. This operation is asynchronous. Use to determine when it has completed. |
| `StartSchemaMerge` | `POST /v1/mergedApis/{mergedApiIdentifier}/sourceApiAssociations/{associationId}/merge` | - | `associationId`, `mergedApiIdentifier` | - | `StartSchemaMergeResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Initiates a merge operation. Returns a status that shows the result of the merge operation. |
| `TagResource` | `POST /v1/tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Tags a resource with user-supplied tags. |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Untags a resource. |
| `UpdateApi` | `POST /v2/apis/{apiId}` | - | `apiId`, `name`, `eventConfig` | - | `UpdateApiResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates an Api . |
| `UpdateApiCache` | `POST /v1/apis/{apiId}/ApiCaches/update` | - | `apiId`, `ttl`, `apiCachingBehavior`, `type` | - | `UpdateApiCacheResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates the cache for the GraphQL API. |
| `UpdateApiKey` | `POST /v1/apis/{apiId}/apikeys/{id}` | - | `apiId`, `id` | - | `UpdateApiKeyResponse` | `ApiKeyValidityOutOfBoundsException`, `BadRequestException`, `InternalFailureException`, `LimitExceededException`, `NotFoundException`, `UnauthorizedException` | Updates an API key. You can update the key as long as it's not deleted. |
| `UpdateChannelNamespace` | `POST /v2/apis/{apiId}/channelNamespaces/{name}` | - | `apiId`, `name` | - | `UpdateChannelNamespaceResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a ChannelNamespace associated with an Api . |
| `UpdateDataSource` | `POST /v1/apis/{apiId}/datasources/{name}` | - | `apiId`, `name`, `type` | - | `UpdateDataSourceResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a DataSource object. |
| `UpdateDomainName` | `POST /v1/domainnames/{domainName}` | - | `domainName` | - | `UpdateDomainNameResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException` | Updates a custom DomainName object. |
| `UpdateFunction` | `POST /v1/apis/{apiId}/functions/{functionId}` | - | `apiId`, `name`, `functionId`, `dataSourceName` | - | `UpdateFunctionResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a Function object. |
| `UpdateGraphqlApi` | `POST /v1/apis/{apiId}` | - | `apiId`, `name`, `authenticationType` | - | `UpdateGraphqlApiResponse` | `AccessDeniedException`, `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a GraphqlApi object. |
| `UpdateResolver` | `POST /v1/apis/{apiId}/types/{typeName}/resolvers/{fieldName}` | - | `apiId`, `typeName`, `fieldName` | - | `UpdateResolverResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a Resolver object. |
| `UpdateSourceApiAssociation` | `POST /v1/mergedApis/{mergedApiIdentifier}/sourceApiAssociations/{associationId}` | - | `associationId`, `mergedApiIdentifier` | - | `UpdateSourceApiAssociationResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates some of the configuration choices of a particular source API association. |
| `UpdateType` | `POST /v1/apis/{apiId}/types/{typeName}` | - | `apiId`, `typeName`, `format` | - | `UpdateTypeResponse` | `BadRequestException`, `ConcurrentModificationException`, `InternalFailureException`, `NotFoundException`, `UnauthorizedException` | Updates a Type object. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetDataSourceIntrospection` | - | `includeModelsSDL -> includeModelsSDL`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetIntrospectionSchema` | - | `format -> format`, `includeDirectives -> includeDirectives` | - | - |
| `GetType` | - | `format -> format` | - | - |
| `ListApiKeys` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListApis` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListChannelNamespaces` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDataSources` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDomainNames` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListFunctions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListGraphqlApis` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `apiType -> apiType`, `owner -> owner` | - | - |
| `ListResolvers` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListResolversByFunction` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListSourceApiAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTypes` | - | `format -> format`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTypesByAssociation` | - | `format -> format`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have access to perform this operation on this resource. |
| `ApiKeyLimitExceededException` | `structure` | message | The API key exceeded a limit. Try your request again. |
| `ApiKeyValidityOutOfBoundsException` | `structure` | message | The API key expiration must be set to a value between 1 and 365 days from creation (for CreateApiKey ) or from update (for UpdateApiKey ). |
| `ApiLimitExceededException` | `structure` | message | The GraphQL API exceeded a limit. Try your request again. |
| `BadRequestException` | `structure` | message, reason, detail | The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. |
| `ConcurrentModificationException` | `structure` | message | Another modification is in progress at this time and it must complete before you can make your change. |
| `ConflictException` | `structure` | message | A conflict with a previous successful update is detected. This typically occurs when the previous update did not have time to propagate before the next upda ... |
| `GraphQLSchemaException` | `structure` | message | The GraphQL schema is not valid. |
| `InternalFailureException` | `structure` | message | An internal AppSync error occurred. Try your request again. |
| `LimitExceededException` | `structure` | message | The request exceeded a limit. Try your request again. |
| `NotFoundException` | `structure` | message | The resource specified in the request was not found. Check the resource, and then try again. |
| `ServiceQuotaExceededException` | `structure` | message | The operation exceeded the service quota for this resource. |
| `UnauthorizedException` | `structure` | message | You aren't authorized to perform this operation. |
| `AssociateApiRequest` | `structure` | domainName, apiId | - |
| `AssociateApiResponse` | `structure` | apiAssociation | - |
| `AssociateMergedGraphqlApiRequest` | `structure` | sourceApiIdentifier, mergedApiIdentifier, description, sourceApiAssociationConfig | - |
| `AssociateMergedGraphqlApiResponse` | `structure` | sourceApiAssociation | - |
| `AssociateSourceGraphqlApiRequest` | `structure` | mergedApiIdentifier, sourceApiIdentifier, description, sourceApiAssociationConfig | - |
| `AssociateSourceGraphqlApiResponse` | `structure` | sourceApiAssociation | - |
| `CreateApiRequest` | `structure` | name, ownerContact, tags, eventConfig | - |
| `CreateApiResponse` | `structure` | api | - |
| `CreateApiCacheRequest` | `structure` | apiId, ttl, transitEncryptionEnabled, atRestEncryptionEnabled, apiCachingBehavior, type, healthMetricsConfig | Represents the input of a CreateApiCache operation. |
| `CreateApiCacheResponse` | `structure` | apiCache | Represents the output of a CreateApiCache operation. |
| `CreateApiKeyRequest` | `structure` | apiId, description, expires | - |
| `CreateApiKeyResponse` | `structure` | apiKey | - |
| `CreateChannelNamespaceRequest` | `structure` | apiId, name, subscribeAuthModes, publishAuthModes, codeHandlers, tags, handlerConfigs | - |
| `CreateChannelNamespaceResponse` | `structure` | channelNamespace | - |
| `CreateDataSourceRequest` | `structure` | apiId, name, description, type, serviceRoleArn, dynamodbConfig, lambdaConfig, elasticsearchConfig, openSearchServiceConfig, httpConfig, relationalDatabaseConfig, eventBridgeConfig, ... (+1) | - |
| `CreateDataSourceResponse` | `structure` | dataSource | - |
| `CreateDomainNameRequest` | `structure` | domainName, certificateArn, description, tags | - |
| `CreateDomainNameResponse` | `structure` | domainNameConfig | - |
| `CreateFunctionRequest` | `structure` | apiId, name, description, dataSourceName, requestMappingTemplate, responseMappingTemplate, functionVersion, syncConfig, maxBatchSize, runtime, code | - |
| `CreateFunctionResponse` | `structure` | functionConfiguration | - |
| `CreateGraphqlApiRequest` | `structure` | name, logConfig, authenticationType, userPoolConfig, openIDConnectConfig, tags, additionalAuthenticationProviders, xrayEnabled, lambdaAuthorizerConfig, apiType, mergedApiExecutionRoleArn, visibility, ... (+5) | - |
| `CreateGraphqlApiResponse` | `structure` | graphqlApi | - |
| `CreateResolverRequest` | `structure` | apiId, typeName, fieldName, dataSourceName, requestMappingTemplate, responseMappingTemplate, kind, pipelineConfig, syncConfig, cachingConfig, maxBatchSize, runtime, ... (+2) | - |
| `CreateResolverResponse` | `structure` | resolver | - |
| `CreateTypeRequest` | `structure` | apiId, definition, format | - |
| `CreateTypeResponse` | `structure` | type | - |
| `DeleteApiRequest` | `structure` | apiId | - |
| `ApiCacheStatus` | `enum` | AVAILABLE, CREATING, DELETING, MODIFYING, FAILED | - |
| `ApiCacheType` | `enum` | T2_SMALL, T2_MEDIUM, R4_LARGE, R4_XLARGE, R4_2XLARGE, R4_4XLARGE, R4_8XLARGE, SMALL, MEDIUM, LARGE, XLARGE, LARGE_2X, ... (+3) | - |
| `ApiCachingBehavior` | `enum` | FULL_REQUEST_CACHING, PER_RESOLVER_CACHING, OPERATION_LEVEL_CACHING | - |
| `AssociationStatus` | `enum` | Processing, Failed, Success | - |
| `AuthenticationType` | `enum` | API_KEY, AWS_IAM, AMAZON_COGNITO_USER_POOLS, OPENID_CONNECT, AWS_LAMBDA | - |
| `AuthorizationType` | `enum` | AWS_IAM | - |
| `BadRequestReason` | `enum` | CODE_ERROR | Provides context for the cause of the bad request. The only supported value is CODE_ERROR . |
| `CacheHealthMetricsConfig` | `enum` | ENABLED, DISABLED | - |
| `ConflictDetectionType` | `enum` | VERSION, NONE | - |
| `ConflictHandlerType` | `enum` | OPTIMISTIC_CONCURRENCY, LAMBDA, AUTOMERGE, NONE | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises why AppSync is a high-value integration hub. Open it when implementing resolver execution, because it identifies durable data-source paths to DynamoDB, Lambda, OpenSearch, Aurora or RDS through the Data API, HTTP endpoints, and EventBridge.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises the boundary between service control-plane state and execution engines. Open it before adding AppSync resolver behaviour that would otherwise become custom mock-only logic.
- Service implication: AppSync and EventBridge are bidirectional in AWS documentation. AppSync can emit events through an EventBridge data source, and EventBridge rules can target public AppSync GraphQL API mutations.
- Service implication: resolver execution work should model data sources, resolver batching, error propagation, and HTTP or API Gateway backed endpoint flows from AWS contracts.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

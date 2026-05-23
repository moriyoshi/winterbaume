# AWS Resource Explorer

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Resource Explorer is a resource search and discovery service. By using Resource Explorer, you can explore your resources using an internet search engine-like experience. Examples of resources include Amazon Relational Database Service (Amazon RDS) instances, Amazon Simple Storage Service (Amazon S3) buckets, or Amazon DynamoDB tables. You can search for your resources using resource metadata like names, tags, and IDs. Resource Explorer can search across all of the Amazon Web Services Regions in your account in which you turn the service on, to simplify your cross-Region workloads.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Resource Explorer where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Resource Explorer by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Resource Explorer by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Resource Explorer workflows in the local mock. Key resources include `CfnIndex`, `CfnView`, `DefaultViewAssociation`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListIndexes`, `ListIndexesForMembers`, `ListManagedViews`, `ListResources`, `GetAccountLevelServiceConfiguration`, `GetDefaultView`.

## Service Identity and Protocol

- AWS model slug: `resource-explorer-2`
- AWS SDK for Rust slug: `resourceexplorer2`
- Model version: `2022-07-28`
- Model file: `vendor/api-models-aws/models/resource-explorer-2/service/2022-07-28/resource-explorer-2-2022-07-28.json`
- SDK ID: `Resource Explorer 2`
- Endpoint prefix: `resource-explorer-2`
- ARN namespace: `resource-explorer-2`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (8), `Create` (3), `Delete` (3), `Update` (2), `Associate` (1), `Batch` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateDefaultView`, `BatchGetView`, `CreateIndex`, `CreateResourceExplorerSetup`, `CreateView`, `DeleteIndex`, `DeleteResourceExplorerSetup`, `DeleteView`, `DisassociateDefaultView`, `TagResource`, `UntagResource`, `UpdateIndexType`, `UpdateView`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetView`, `GetAccountLevelServiceConfiguration`, `GetDefaultView`, `GetIndex`, `GetManagedView`, `GetResourceExplorerSetup`, `GetServiceIndex`, `GetServiceView`, `GetView`, `ListIndexes`, `ListIndexesForMembers`, `ListManagedViews`, `ListResources`, `ListServiceIndexes`, `ListServiceViews`, `ListStreamingAccessForServices`, `ListSupportedResourceTypes`, `ListTagsForResource`, `ListViews`, `Search`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 32 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `RDS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CfnIndex` | `Arn` | create: `CreateIndex`; update: `UpdateIndexType`; delete: `DeleteIndex`; list: `ListIndexes` | - | - |
| `CfnView` | `ViewArn` | create: `CreateView`; read: `GetView`; update: `UpdateView`; delete: `DeleteView`; list: `ListViews` | - | - |
| `DefaultViewAssociation` | `ViewArn` | put: `AssociateDefaultView` | - | - |
## Operation Groups

### List

- Operations: `ListIndexesForMembers`, `ListManagedViews`, `ListResources`, `ListServiceIndexes`, `ListServiceViews`, `ListStreamingAccessForServices`, `ListSupportedResourceTypes`, `ListTagsForResource`
- Traits: `readonly` (6), `paginated` (7)
- Common required input members in this group: -

### Get

- Operations: `GetAccountLevelServiceConfiguration`, `GetDefaultView`, `GetIndex`, `GetManagedView`, `GetResourceExplorerSetup`, `GetServiceIndex`, `GetServiceView`
- Traits: `readonly` (6), `paginated` (1)
- Common required input members in this group: -

### Batch

- Operations: `BatchGetView`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateResourceExplorerSetup`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteResourceExplorerSetup`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateDefaultView`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Search

- Operations: `Search`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetView` | `POST /BatchGetView` | `readonly` | - | - | `BatchGetViewOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves details about a list of views. |
| `CreateResourceExplorerSetup` | `POST /CreateResourceExplorerSetup` | `idempotent` | `RegionList`, `ViewName` | - | `CreateResourceExplorerSetupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a Resource Explorer setup configuration across multiple Amazon Web Services Regions. This operation sets up indexes and views in the specified Regions. This operation can also be used to set an aggregator Reg ... |
| `DeleteResourceExplorerSetup` | `POST /DeleteResourceExplorerSetup` | `idempotent` | - | - | `DeleteResourceExplorerSetupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a Resource Explorer setup configuration. This operation removes indexes and views from the specified Regions or all Regions where Resource Explorer is configured. |
| `DisassociateDefaultView` | `POST /DisassociateDefaultView` | `idempotent` | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | After you call this operation, the affected Amazon Web Services Region no longer has a default view. All Search operations in that Region must explicitly specify a view or the operation fails. You can configure a new ... |
| `GetAccountLevelServiceConfiguration` | `POST /GetAccountLevelServiceConfiguration` | `readonly` | - | - | `GetAccountLevelServiceConfigurationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the status of your account's Amazon Web Services service access, and validates the service linked role required to access the multi-account search feature. Only the management account can invoke this API call. |
| `GetDefaultView` | `POST /GetDefaultView` | `readonly` | - | - | `GetDefaultViewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the Amazon Resource Name (ARN) of the view that is the default for the Amazon Web Services Region in which you call this operation. You can then call GetView to retrieve the details of that view. |
| `GetIndex` | `POST /GetIndex` | - | - | - | `GetIndexOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about the Amazon Web Services Resource Explorer index in the Amazon Web Services Region in which you invoked the operation. |
| `GetManagedView` | `POST /GetManagedView` | `readonly` | `ManagedViewArn` | - | `GetManagedViewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves details of the specified Amazon Web Services-managed view . |
| `GetResourceExplorerSetup` | `POST /GetResourceExplorerSetup` | `readonly`, `paginated` | `TaskId` | - | `GetResourceExplorerSetupOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the status and details of a Resource Explorer setup operation. This operation returns information about the progress of creating or deleting Resource Explorer configurations across Regions. |
| `GetServiceIndex` | `POST /GetServiceIndex` | `readonly` | - | - | `GetServiceIndexOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the Resource Explorer index in the current Amazon Web Services Region. This operation returns the ARN and type of the index if one exists. |
| `GetServiceView` | `POST /GetServiceView` | `readonly` | `ServiceViewArn` | - | `GetServiceViewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a specific Resource Explorer service view. This operation returns the configuration and properties of the specified view. |
| `ListIndexesForMembers` | `POST /ListIndexesForMembers` | `readonly`, `paginated` | `AccountIdList` | - | `ListIndexesForMembersOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of a member's indexes in all Amazon Web Services Regions that are currently collecting resource information for Amazon Web Services Resource Explorer. Only the management account or a delegated admin ... |
| `ListManagedViews` | `POST /ListManagedViews` | `paginated` | - | - | `ListManagedViewsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the Amazon resource names (ARNs) of the Amazon Web Services-managed views available in the Amazon Web Services Region in which you call this operation. |
| `ListResources` | `POST /ListResources` | `readonly`, `paginated` | - | - | `ListResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Returns a list of resources and their details that match the specified criteria. This query must use a view. If you don’t explicitly specify a view, then Resource Explorer uses the default view for the Amazon Web Ser ... |
| `ListServiceIndexes` | `POST /ListServiceIndexes` | `readonly`, `paginated` | - | - | `ListServiceIndexesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all Resource Explorer indexes across the specified Amazon Web Services Regions. This operation returns information about indexes including their ARNs, types, and Regions. |
| `ListServiceViews` | `POST /ListServiceViews` | `readonly`, `paginated` | - | - | `ListServiceViewsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all Resource Explorer service views available in the current Amazon Web Services account. This operation returns the ARNs of available service views. |
| `ListStreamingAccessForServices` | `POST /ListStreamingAccessForServices` | `paginated` | - | - | `ListStreamingAccessForServicesOutput` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of Amazon Web Services services that have been granted streaming access to your Resource Explorer data. Streaming access allows Amazon Web Services services to receive real-time updates about your reso ... |
| `ListSupportedResourceTypes` | `POST /ListSupportedResourceTypes` | `readonly`, `paginated` | - | - | `ListSupportedResourceTypesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all resource types currently supported by Amazon Web Services Resource Explorer. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the tags that are attached to the specified resource. |
| `Search` | `POST /Search` | `readonly`, `paginated` | `QueryString` | - | `SearchOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Searches for resources and displays details about all resources that match the specified criteria. You must specify a query string. All search queries must use a view. If you don't explicitly specify a view, then Ama ... |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Adds one or more tag key and value pairs to an Amazon Web Services Resource Explorer view or index. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Removes one or more tag key and value pairs from an Amazon Web Services Resource Explorer view or index. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | The credentials that you used to call this operation don't have the minimum required permissions. |
| `ConflictException` | `structure` | Message | If you attempted to create a view, then the request failed because either you specified parameters that didn’t match the original request, or you attempted ... |
| `InternalServerException` | `structure` | Message | The request failed because of internal service error. Try your request again later. |
| `ResourceNotFoundException` | `structure` | Message | You specified a resource that doesn't exist. Check the ID or ARN that you used to identity the resource, and try again. |
| `ServiceQuotaExceededException` | `structure` | Message, Name, Value | The request failed because it exceeds a service quota. |
| `ThrottlingException` | `structure` | Message | The request failed because you exceeded a rate limit for this operation. For more information, see Quotas for Resource Explorer . |
| `UnauthorizedException` | `structure` | Message | The principal making the request isn't permitted to perform the operation. |
| `ValidationException` | `structure` | Message, FieldList | You provided an invalid value for one of the operation's parameters. Check the syntax for the operation, and try again. |
| `BatchGetViewInput` | `structure` | ViewArns | - |
| `BatchGetViewOutput` | `structure` | Views, Errors | - |
| `CreateResourceExplorerSetupInput` | `structure` | RegionList, AggregatorRegions, ViewName | - |
| `CreateResourceExplorerSetupOutput` | `structure` | TaskId | - |
| `DeleteResourceExplorerSetupInput` | `structure` | RegionList, DeleteInAllRegions | - |
| `DeleteResourceExplorerSetupOutput` | `structure` | TaskId | - |
| `GetAccountLevelServiceConfigurationOutput` | `structure` | OrgConfiguration | - |
| `GetDefaultViewOutput` | `structure` | ViewArn | - |
| `GetIndexOutput` | `structure` | Arn, Type, State, ReplicatingFrom, ReplicatingTo, CreatedAt, LastUpdatedAt, Tags | - |
| `GetManagedViewInput` | `structure` | ManagedViewArn | - |
| `GetManagedViewOutput` | `structure` | ManagedView | - |
| `GetResourceExplorerSetupInput` | `structure` | TaskId, MaxResults, NextToken | - |
| `GetResourceExplorerSetupOutput` | `structure` | Regions, NextToken | - |
| `GetServiceIndexOutput` | `structure` | Arn, Type | - |
| `GetServiceViewInput` | `structure` | ServiceViewArn | - |
| `GetServiceViewOutput` | `structure` | View | - |
| `ListIndexesForMembersInput` | `structure` | AccountIdList, MaxResults, NextToken | - |
| `ListIndexesForMembersOutput` | `structure` | Indexes, NextToken | - |
| `ListManagedViewsInput` | `structure` | MaxResults, NextToken, ServicePrincipal | - |
| `ListManagedViewsOutput` | `structure` | NextToken, ManagedViews | - |
| `ListResourcesInput` | `structure` | Filters, MaxResults, ViewArn, NextToken | - |
| `ListResourcesOutput` | `structure` | Resources, NextToken, ViewArn | - |
| `ListServiceIndexesInput` | `structure` | Regions, MaxResults, NextToken | - |
| `ListServiceIndexesOutput` | `structure` | Indexes, NextToken | - |
| `ListServiceViewsInput` | `structure` | MaxResults, NextToken | - |
| `ListServiceViewsOutput` | `structure` | NextToken, ServiceViews | - |
| `ListStreamingAccessForServicesInput` | `structure` | MaxResults, NextToken | - |
| `ListStreamingAccessForServicesOutput` | `structure` | StreamingAccessForServices, NextToken | - |
| `ListSupportedResourceTypesInput` | `structure` | NextToken, MaxResults | - |
| `ListSupportedResourceTypesOutput` | `structure` | ResourceTypes, NextToken | - |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | Tags | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon OpenSearch Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use the Amazon OpenSearch Service configuration API to create, configure, and manage OpenSearch Service domains. The endpoint for configuration service requests is Region specific: es. region .amazonaws.com. For example, es.us-east-1.amazonaws.com. For a current list of supported Regions and endpoints, see Amazon Web Services service endpoints.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon OpenSearch Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon OpenSearch Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon OpenSearch Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon OpenSearch Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Get`, `Delete`, `Update` operation families, including `DescribeDomain`, `DescribeDomainAutoTunes`, `DescribeDomainChangeProgress`, `DescribeDomainConfig`, `ListApplications`, `ListDataSources`.

## Service Identity and Protocol

- AWS model slug: `opensearch`
- AWS SDK for Rust slug: `opensearch`
- Model version: `2021-01-01`
- Model file: `vendor/api-models-aws/models/opensearch/service/2021-01-01/opensearch-2021-01-01.json`
- SDK ID: `OpenSearch`
- Endpoint prefix: `es`
- ARN namespace: `es`
- CloudFormation name: `OpenSearch`
- CloudTrail event source: `opensearch.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (15), `List` (14), `Get` (10), `Delete` (9), `Update` (9), `Create` (6), `Add` (3), `Associate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptInboundConnection`, `AddDataSource`, `AddDirectQueryDataSource`, `AddTags`, `AssociatePackage`, `AssociatePackages`, `CancelDomainConfigChange`, `CancelServiceSoftwareUpdate`, `CreateApplication`, `CreateDomain`, `CreateIndex`, `CreateOutboundConnection`, `CreatePackage`, `CreateVpcEndpoint`, `DeleteApplication`, `DeleteDataSource`, `DeleteDirectQueryDataSource`, `DeleteDomain`, `DeleteInboundConnection`, `DeleteIndex`, ... (+18).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDomain`, `DescribeDomainAutoTunes`, `DescribeDomainChangeProgress`, `DescribeDomainConfig`, `DescribeDomainHealth`, `DescribeDomainNodes`, `DescribeDomains`, `DescribeDryRunProgress`, `DescribeInboundConnections`, `DescribeInstanceTypeLimits`, `DescribeOutboundConnections`, `DescribePackages`, `DescribeReservedInstanceOfferings`, `DescribeReservedInstances`, `DescribeVpcEndpoints`, `GetApplication`, `GetCompatibleVersions`, `GetDataSource`, `GetDefaultApplicationSetting`, `GetDirectQueryDataSource`, ... (+19).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelDomainConfigChange`, `CancelServiceSoftwareUpdate`, `StartDomainMaintenance`, `StartServiceSoftwareUpdate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 82 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `Glue`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/opensearch-service/latest/developerguide/managedomains-configuration-changes.html

Research outcomes:
- Many domain configuration changes usually cause blue/green deployments, including instance type, fine-grained access control, service software updates, dedicated master enablement, Multi-AZ without Standby, storage type or size, VPC networking, coordinator nodes, Cognito Dashboards auth, advanced settings, engine upgrade, encryption changes, UltraWarm/cold storage, plugin association, and audit logs.
- Changes that usually do not cause blue/green include access policy, custom endpoint, TLS policy, automated snapshot hour, Require HTTPS, Auto-Tune without rollback, many node-count changes with dedicated masters, some log settings, gp3 volume increases, and tags.
- Multi-AZ with Standby accepts only one change request at a time. A new request is rejected if a change is in progress.
- Domain processing statuses include `Active`, `Creating`, `Modifying`, `Upgrading engine version`, `Updating service software`, `Deleting`, and `Isolated`.
- Configuration change statuses include `Pending`, `Initializing`, `Validating`, `Awaiting user inputs`, `Applying changes`, `Cancelled`, `Completed`, and `Validation failed`.
- AWS recommends waiting for domain status `Active` before submitting additional changes.
- If validation fails, no configuration changes are applied. The user can cancel, retry, or edit the request.
- Blue/green stages include validation, creating a new environment, provisioning nodes, routing traffic, copying shards, terminating old nodes, deleting resources, dynamic update, and master or volume changes.
- During blue/green deployment node count can temporarily double, but AWS does not charge beyond the requested node count.

Parity implications:
- Model domains, cluster configuration, VPC options, endpoints, access policy, encryption, log publishing, software update state, change progress, blue/green deployment state, and validation failures separately.
- Mutating operations should distinguish dynamic in-place updates from blue/green deployments and expose domain/change status transitions.
- Multi-AZ with Standby should reject concurrent configuration changes.

## Current Network Resource Stub Semantics

OpenSearch currently stores VPC endpoints and access relationships inside OpenSearch state.

- VPC endpoint records keep the domain name, owner, VPC ID, subnet IDs, and security group IDs supplied by the request.
- Domain endpoint access principals are stored as domain-local lists.
- The service does not provision EC2 VPC endpoints or check subnet and security group membership.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeDomain`, `DescribeDomainAutoTunes`, `DescribeDomainChangeProgress`, `DescribeDomainConfig`, `DescribeDomainHealth`, `DescribeDomainNodes`, `DescribeDomains`, `DescribeDryRunProgress`, `DescribeInboundConnections`, `DescribeInstanceTypeLimits`, `DescribeOutboundConnections`, `DescribePackages`, `DescribeReservedInstanceOfferings`, `DescribeReservedInstances`, `DescribeVpcEndpoints`
- Traits: `paginated` (6)
- Common required input members in this group: `DomainName`, `DomainNames`, `EngineVersion`, `InstanceType`, `VpcEndpointIds`

### List

- Operations: `ListApplications`, `ListDataSources`, `ListDirectQueryDataSources`, `ListDomainMaintenances`, `ListDomainNames`, `ListDomainsForPackage`, `ListInstanceTypeDetails`, `ListPackagesForDomain`, `ListScheduledActions`, `ListTags`, `ListVersions`, `ListVpcEndpointAccess`, `ListVpcEndpoints`, `ListVpcEndpointsForDomain`
- Traits: `paginated` (7)
- Common required input members in this group: `ARN`, `DomainName`, `EngineVersion`, `PackageID`

### Get

- Operations: `GetApplication`, `GetCompatibleVersions`, `GetDataSource`, `GetDefaultApplicationSetting`, `GetDirectQueryDataSource`, `GetDomainMaintenanceStatus`, `GetIndex`, `GetPackageVersionHistory`, `GetUpgradeHistory`, `GetUpgradeStatus`
- Traits: `paginated` (2)
- Common required input members in this group: `DataSourceName`, `DomainName`, `IndexName`, `MaintenanceId`, `Name`, `PackageID`, `id`

### Delete

- Operations: `DeleteApplication`, `DeleteDataSource`, `DeleteDirectQueryDataSource`, `DeleteDomain`, `DeleteInboundConnection`, `DeleteIndex`, `DeleteOutboundConnection`, `DeletePackage`, `DeleteVpcEndpoint`
- Common required input members in this group: `ConnectionId`, `DataSourceName`, `DomainName`, `IndexName`, `Name`, `PackageID`, `VpcEndpointId`, `id`

### Update

- Operations: `UpdateApplication`, `UpdateDataSource`, `UpdateDirectQueryDataSource`, `UpdateDomainConfig`, `UpdateIndex`, `UpdatePackage`, `UpdatePackageScope`, `UpdateScheduledAction`, `UpdateVpcEndpoint`
- Common required input members in this group: `ActionID`, `ActionType`, `DataSourceName`, `DataSourceType`, `DomainName`, `IndexName`, `IndexSchema`, `Name`, `OpenSearchArns`, `Operation`, `PackageID`, `PackageSource`, `PackageUserList`, `ScheduleAt`, `VpcEndpointId`, `VpcOptions`, `id`

### Create

- Operations: `CreateApplication`, `CreateDomain`, `CreateIndex`, `CreateOutboundConnection`, `CreatePackage`, `CreateVpcEndpoint`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ConnectionAlias`, `DomainArn`, `DomainName`, `IndexName`, `IndexSchema`, `LocalDomainInfo`, `PackageName`, `PackageSource`, `PackageType`, `RemoteDomainInfo`, `VpcOptions`, `name`

### Add

- Operations: `AddDataSource`, `AddDirectQueryDataSource`, `AddTags`
- Common required input members in this group: `ARN`, `DataSourceName`, `DataSourceType`, `DomainName`, `Name`, `OpenSearchArns`, `TagList`

### Associate

- Operations: `AssociatePackage`, `AssociatePackages`
- Common required input members in this group: `DomainName`, `PackageID`, `PackageList`

### Cancel

- Operations: `CancelDomainConfigChange`, `CancelServiceSoftwareUpdate`
- Common required input members in this group: `DomainName`

### Dissociate

- Operations: `DissociatePackage`, `DissociatePackages`
- Common required input members in this group: `DomainName`, `PackageID`, `PackageList`

### Start

- Operations: `StartDomainMaintenance`, `StartServiceSoftwareUpdate`
- Common required input members in this group: `Action`, `DomainName`

### Accept

- Operations: `AcceptInboundConnection`
- Common required input members in this group: `ConnectionId`

### Authorize

- Operations: `AuthorizeVpcEndpointAccess`
- Common required input members in this group: `DomainName`

### Purchase

- Operations: `PurchaseReservedInstanceOffering`
- Common required input members in this group: `ReservationName`, `ReservedInstanceOfferingId`

### Put

- Operations: `PutDefaultApplicationSetting`
- Common required input members in this group: `applicationArn`, `setAsDefault`

### Reject

- Operations: `RejectInboundConnection`
- Common required input members in this group: `ConnectionId`

### Remove

- Operations: `RemoveTags`
- Common required input members in this group: `ARN`, `TagKeys`

### Revoke

- Operations: `RevokeVpcEndpointAccess`
- Common required input members in this group: `DomainName`

### Upgrade

- Operations: `UpgradeDomain`
- Common required input members in this group: `DomainName`, `TargetVersion`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptInboundConnection` | `PUT /2021-01-01/opensearch/cc/inboundConnection/{ConnectionId}/accept` | - | `ConnectionId` | - | `AcceptInboundConnectionResponse` | `DisabledOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Allows the destination Amazon OpenSearch Service domain owner to accept an inbound cross-cluster search connection request. For more information, see Cross-cluster search for Amazon OpenSearch Service. |
| `AddDataSource` | `POST /2021-01-01/opensearch/domain/{DomainName}/dataSource` | - | `DataSourceType`, `DomainName`, `Name` | - | `AddDataSourceResponse` | `BaseException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Creates a new direct-query data source to the specified domain. For more information, see Creating Amazon OpenSearch Service data source integrations with Amazon S3. |
| `AddDirectQueryDataSource` | `POST /2021-01-01/opensearch/directQueryDataSource` | - | `DataSourceName`, `DataSourceType`, `OpenSearchArns` | - | `AddDirectQueryDataSourceResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Adds a new data source in Amazon OpenSearch Service so that you can perform direct queries on external data. |
| `AddTags` | `POST /2021-01-01/tags` | - | `ARN`, `TagList` | - | `Unit` | `BaseException`, `InternalException`, `LimitExceededException`, `ValidationException` | Attaches tags to an existing Amazon OpenSearch Service domain, data source, or application. Tags are a set of case-sensitive key-value pairs. |
| `AssociatePackage` | `POST /2021-01-01/packages/associate/{PackageID}/{DomainName}` | - | `DomainName`, `PackageID` | - | `AssociatePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Associates a package with an Amazon OpenSearch Service domain. For more information, see Custom packages for Amazon OpenSearch Service. |
| `AssociatePackages` | `POST /2021-01-01/packages/associateMultiple` | - | `DomainName`, `PackageList` | - | `AssociatePackagesResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Operation in the Amazon OpenSearch Service API for associating multiple packages with a domain simultaneously. |
| `AuthorizeVpcEndpointAccess` | `POST /2021-01-01/opensearch/domain/{DomainName}/authorizeVpcEndpointAccess` | - | `DomainName` | - | `AuthorizeVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Provides access to an Amazon OpenSearch Service domain through the use of an interface VPC endpoint. |
| `CancelDomainConfigChange` | `POST /2021-01-01/opensearch/domain/{DomainName}/config/cancel` | - | `DomainName` | - | `CancelDomainConfigChangeResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Cancels a pending configuration change on an Amazon OpenSearch Service domain. |
| `CancelServiceSoftwareUpdate` | `POST /2021-01-01/opensearch/serviceSoftwareUpdate/cancel` | - | `DomainName` | - | `CancelServiceSoftwareUpdateResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Cancels a scheduled service software update for an Amazon OpenSearch Service domain. You can only perform this operation before the `AutomatedUpdateDate` and when the domain's `UpdateStatus` is `PENDING_UPDATE`. |
| `CreateApplication` | `POST /2021-01-01/opensearch/application` | `idempotency-token` | `name` | `clientToken` | `CreateApplicationResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ValidationException` | Creates an OpenSearch UI application. For more information, see Using the OpenSearch user interface in Amazon OpenSearch Service. |
| `CreateDomain` | `POST /2021-01-01/opensearch/domain` | - | `DomainName` | - | `CreateDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates an Amazon OpenSearch Service domain. For more information, see Creating and managing Amazon OpenSearch Service domains. |
| `CreateIndex` | `POST /2021-01-01/opensearch/domain/{DomainName}/index` | - | `DomainName`, `IndexName`, `IndexSchema` | - | `CreateIndexResponse` | `AccessDeniedException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an OpenSearch index with optional automatic semantic enrichment for specified text fields. Automatic semantic enrichment enables semantic search capabilities without requiring machine learning expertise, improving search relevance by up to 20% by... |
| `CreateOutboundConnection` | `POST /2021-01-01/opensearch/cc/outboundConnection` | - | `ConnectionAlias`, `LocalDomainInfo`, `RemoteDomainInfo` | - | `CreateOutboundConnectionResponse` | `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates a new cross-cluster search connection from a source Amazon OpenSearch Service domain to a destination domain. For more information, see Cross-cluster search for Amazon OpenSearch Service. |
| `CreatePackage` | `POST /2021-01-01/packages` | - | `PackageName`, `PackageSource`, `PackageType` | - | `CreatePackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates a package for use with Amazon OpenSearch Service domains. For more information, see Custom packages for Amazon OpenSearch Service. |
| `CreateVpcEndpoint` | `POST /2021-01-01/opensearch/vpcEndpoints` | - | `DomainArn`, `VpcOptions` | - | `CreateVpcEndpointResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ValidationException` | Creates an Amazon OpenSearch Service-managed VPC endpoint. |
| `DeleteApplication` | `DELETE /2021-01-01/opensearch/application/{id}` | - | `id` | - | `DeleteApplicationResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Deletes a specified OpenSearch application. |
| `DeleteDataSource` | `DELETE /2021-01-01/opensearch/domain/{DomainName}/dataSource/{Name}` | - | `DomainName`, `Name` | - | `DeleteDataSourceResponse` | `BaseException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Deletes a direct-query data source. For more information, see Deleting an Amazon OpenSearch Service data source with Amazon S3. |
| `DeleteDirectQueryDataSource` | `DELETE /2021-01-01/opensearch/directQueryDataSource/{DataSourceName}` | - | `DataSourceName` | - | `Unit` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Deletes a previously configured direct query data source from Amazon OpenSearch Service. |
| `DeleteDomain` | `DELETE /2021-01-01/opensearch/domain/{DomainName}` | - | `DomainName` | - | `DeleteDomainResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Deletes an Amazon OpenSearch Service domain and all of its data. You can't recover a domain after you delete it. |
| `DeleteInboundConnection` | `DELETE /2021-01-01/opensearch/cc/inboundConnection/{ConnectionId}` | - | `ConnectionId` | - | `DeleteInboundConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the destination Amazon OpenSearch Service domain owner to delete an existing inbound cross-cluster search connection. For more information, see Cross-cluster search for Amazon OpenSearch Service. |
| `DeleteIndex` | `DELETE /2021-01-01/opensearch/domain/{DomainName}/index/{IndexName}` | - | `DomainName`, `IndexName` | - | `DeleteIndexResponse` | `AccessDeniedException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an OpenSearch index. This operation permanently removes the index and cannot be undone. |
| `DeleteOutboundConnection` | `DELETE /2021-01-01/opensearch/cc/outboundConnection/{ConnectionId}` | - | `ConnectionId` | - | `DeleteOutboundConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the source Amazon OpenSearch Service domain owner to delete an existing outbound cross-cluster search connection. For more information, see Cross-cluster search for Amazon OpenSearch Service. |
| `DeletePackage` | `DELETE /2021-01-01/packages/{PackageID}` | - | `PackageID` | - | `DeletePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Deletes an Amazon OpenSearch Service package. For more information, see Custom packages for Amazon OpenSearch Service. |
| `DeleteVpcEndpoint` | `DELETE /2021-01-01/opensearch/vpcEndpoints/{VpcEndpointId}` | - | `VpcEndpointId` | - | `DeleteVpcEndpointResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Deletes an Amazon OpenSearch Service-managed interface VPC endpoint. |
| `DescribeDomain` | `GET /2021-01-01/opensearch/domain/{DomainName}` | - | `DomainName` | - | `DescribeDomainResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes the domain configuration for the specified Amazon OpenSearch Service domain, including the domain ID, domain service endpoint, and domain ARN. |
| `DescribeDomainAutoTunes` | `GET /2021-01-01/opensearch/domain/{DomainName}/autoTunes` | `paginated` | `DomainName` | - | `DescribeDomainAutoTunesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns the list of optimizations that Auto-Tune has made to an Amazon OpenSearch Service domain. For more information, see Auto-Tune for Amazon OpenSearch Service. |
| `DescribeDomainChangeProgress` | `GET /2021-01-01/opensearch/domain/{DomainName}/progress` | - | `DomainName` | - | `DescribeDomainChangeProgressResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about the current blue/green deployment happening on an Amazon OpenSearch Service domain. For more information, see Making configuration changes in Amazon OpenSearch Service. |
| `DescribeDomainConfig` | `GET /2021-01-01/opensearch/domain/{DomainName}/config` | - | `DomainName` | - | `DescribeDomainConfigResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns the configuration of an Amazon OpenSearch Service domain. |
| `DescribeDomainHealth` | `GET /2021-01-01/opensearch/domain/{DomainName}/health` | - | `DomainName` | - | `DescribeDomainHealthResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about domain and node health, the standby Availability Zone, number of nodes per Availability Zone, and shard count per node. |
| `DescribeDomainNodes` | `GET /2021-01-01/opensearch/domain/{DomainName}/nodes` | - | `DomainName` | - | `DescribeDomainNodesResponse` | `BaseException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about domain and nodes, including data nodes, master nodes, ultrawarm nodes, Availability Zone(s), standby nodes, node configurations, and node states. |
| `DescribeDomains` | `POST /2021-01-01/opensearch/domain-info` | - | `DomainNames` | - | `DescribeDomainsResponse` | `BaseException`, `InternalException`, `ValidationException` | Returns domain configuration information about the specified Amazon OpenSearch Service domains. |
| `DescribeDryRunProgress` | `GET /2021-01-01/opensearch/domain/{DomainName}/dryRun` | - | `DomainName` | - | `DescribeDryRunProgressResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes the progress of a pre-update dry run analysis on an Amazon OpenSearch Service domain. For more information, see Determining whether a change will cause a blue/green deployment. |
| `DescribeInboundConnections` | `POST /2021-01-01/opensearch/cc/inboundConnection/search` | `paginated` | - | - | `DescribeInboundConnectionsResponse` | `DisabledOperationException`, `InvalidPaginationTokenException` | Lists all the inbound cross-cluster search connections for a destination (remote) Amazon OpenSearch Service domain. For more information, see Cross-cluster search for Amazon OpenSearch Service. |
| `DescribeInstanceTypeLimits` | `GET /2021-01-01/opensearch/instanceTypeLimits/{EngineVersion}/{InstanceType}` | - | `EngineVersion`, `InstanceType` | - | `DescribeInstanceTypeLimitsResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Describes the instance count, storage, and master node limits for a given OpenSearch or Elasticsearch version and instance type. |
| `DescribeOutboundConnections` | `POST /2021-01-01/opensearch/cc/outboundConnection/search` | `paginated` | - | - | `DescribeOutboundConnectionsResponse` | `DisabledOperationException`, `InvalidPaginationTokenException` | Lists all the outbound cross-cluster connections for a local (source) Amazon OpenSearch Service domain. For more information, see Cross-cluster search for Amazon OpenSearch Service. |
| `DescribePackages` | `POST /2021-01-01/packages/describe` | `paginated` | - | - | `DescribePackagesResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes all packages available to OpenSearch Service. For more information, see Custom packages for Amazon OpenSearch Service. |
| `DescribeReservedInstanceOfferings` | `GET /2021-01-01/opensearch/reservedInstanceOfferings` | `paginated` | - | - | `DescribeReservedInstanceOfferingsResponse` | `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes the available Amazon OpenSearch Service Reserved Instance offerings for a given Region. For more information, see Reserved Instances in Amazon OpenSearch Service. |
| `DescribeReservedInstances` | `GET /2021-01-01/opensearch/reservedInstances` | `paginated` | - | - | `DescribeReservedInstancesResponse` | `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes the Amazon OpenSearch Service instances that you have reserved in a given Region. For more information, see Reserved Instances in Amazon OpenSearch Service. |
| `DescribeVpcEndpoints` | `POST /2021-01-01/opensearch/vpcEndpoints/describe` | - | `VpcEndpointIds` | - | `DescribeVpcEndpointsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ValidationException` | Describes one or more Amazon OpenSearch Service-managed VPC endpoints. |
| `DissociatePackage` | `POST /2021-01-01/packages/dissociate/{PackageID}/{DomainName}` | - | `DomainName`, `PackageID` | - | `DissociatePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Removes a package from the specified Amazon OpenSearch Service domain. The package can't be in use with any OpenSearch index for the dissociation to succeed. |
| `DissociatePackages` | `POST /2021-01-01/packages/dissociateMultiple` | - | `DomainName`, `PackageList` | - | `DissociatePackagesResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Dissociates multiple packages from a domain simultaneously. |
| `GetApplication` | `GET /2021-01-01/opensearch/application/{id}` | - | `id` | - | `GetApplicationResponse` | `AccessDeniedException`, `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the configuration and status of an existing OpenSearch application. |
| `GetCompatibleVersions` | `GET /2021-01-01/opensearch/compatibleVersions` | - | - | - | `GetCompatibleVersionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns a map of OpenSearch or Elasticsearch versions and the versions you can upgrade them to. |
| `GetDataSource` | `GET /2021-01-01/opensearch/domain/{DomainName}/dataSource/{Name}` | - | `DomainName`, `Name` | - | `GetDataSourceResponse` | `BaseException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about a direct query data source. |
| `GetDefaultApplicationSetting` | `GET /2021-01-01/opensearch/defaultApplicationSetting` | - | - | - | `GetDefaultApplicationSettingResponse` | `AccessDeniedException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Gets the ARN of the current default application. If the default application isn't set, the operation returns a resource not found error. |
| `GetDirectQueryDataSource` | `GET /2021-01-01/opensearch/directQueryDataSource/{DataSourceName}` | - | `DataSourceName` | - | `GetDirectQueryDataSourceResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns detailed configuration information for a specific direct query data source in Amazon OpenSearch Service. |
| `GetDomainMaintenanceStatus` | `GET /2021-01-01/opensearch/domain/{DomainName}/domainMaintenance` | - | `DomainName`, `MaintenanceId` | - | `GetDomainMaintenanceStatusResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | The status of the maintenance action. |
| `GetIndex` | `GET /2021-01-01/opensearch/domain/{DomainName}/index/{IndexName}` | - | `DomainName`, `IndexName` | - | `GetIndexResponse` | `AccessDeniedException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an OpenSearch index including its schema and semantic enrichment configuration. Use this operation to view the current index structure and semantic search settings. |
| `GetPackageVersionHistory` | `GET /2021-01-01/packages/{PackageID}/history` | `paginated` | `PackageID` | - | `GetPackageVersionHistoryResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of Amazon OpenSearch Service package versions, along with their creation time, commit message, and plugin properties (if the package is a zip plugin package). For more information, see Custom packages for Amazon OpenSearch Service. |
| `GetUpgradeHistory` | `GET /2021-01-01/opensearch/upgradeDomain/{DomainName}/history` | `paginated` | `DomainName` | - | `GetUpgradeHistoryResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the complete history of the last 10 upgrades performed on an Amazon OpenSearch Service domain. |
| `GetUpgradeStatus` | `GET /2021-01-01/opensearch/upgradeDomain/{DomainName}/status` | - | `DomainName` | - | `GetUpgradeStatusResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns the most recent status of the last upgrade or upgrade eligibility check performed on an Amazon OpenSearch Service domain. |
| `ListApplications` | `GET /2021-01-01/opensearch/list-applications` | `paginated` | - | - | `ListApplicationsResponse` | `AccessDeniedException`, `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all OpenSearch applications under your account. |
| `ListDataSources` | `GET /2021-01-01/opensearch/domain/{DomainName}/dataSource` | - | `DomainName` | - | `ListDataSourcesResponse` | `BaseException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists direct-query data sources for a specific domain. For more information, see For more information, see Working with Amazon OpenSearch Service direct queries with Amazon S3. |
| `ListDirectQueryDataSources` | `GET /2021-01-01/opensearch/directQueryDataSource` | - | - | - | `ListDirectQueryDataSourcesResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists an inventory of all the direct query data sources that you have configured within Amazon OpenSearch Service. |
| `ListDomainMaintenances` | `GET /2021-01-01/opensearch/domain/{DomainName}/domainMaintenances` | `paginated` | `DomainName` | - | `ListDomainMaintenancesResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | A list of maintenance actions for the domain. |
| `ListDomainNames` | `GET /2021-01-01/domain` | - | - | - | `ListDomainNamesResponse` | `BaseException`, `ValidationException` | Returns the names of all Amazon OpenSearch Service domains owned by the current user in the active Region. |
| `ListDomainsForPackage` | `GET /2021-01-01/packages/{PackageID}/domains` | `paginated` | `PackageID` | - | `ListDomainsForPackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all Amazon OpenSearch Service domains associated with a given package. For more information, see Custom packages for Amazon OpenSearch Service. |
| `ListInstanceTypeDetails` | `GET /2021-01-01/opensearch/instanceTypeDetails/{EngineVersion}` | `paginated` | `EngineVersion` | - | `ListInstanceTypeDetailsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all instance types and available features for a given OpenSearch or Elasticsearch version. |
| `ListPackagesForDomain` | `GET /2021-01-01/domain/{DomainName}/packages` | `paginated` | `DomainName` | - | `ListPackagesForDomainResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all packages associated with an Amazon OpenSearch Service domain. For more information, see Custom packages for Amazon OpenSearch Service. |
| `ListScheduledActions` | `GET /2021-01-01/opensearch/domain/{DomainName}/scheduledActions` | `paginated` | `DomainName` | - | `ListScheduledActionsResponse` | `BaseException`, `InternalException`, `InvalidPaginationTokenException`, `ResourceNotFoundException`, `ValidationException` | Retrieves a list of configuration changes that are scheduled for a domain. These changes can be service software updates or blue/green Auto-Tune enhancements. |
| `ListTags` | `GET /2021-01-01/tags` | - | `ARN` | - | `ListTagsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns all resource tags for an Amazon OpenSearch Service domain, data source, or application. For more information, see Tagging Amazon OpenSearch Service resources. |
| `ListVersions` | `GET /2021-01-01/opensearch/versions` | `paginated` | - | - | `ListVersionsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all versions of OpenSearch and Elasticsearch that Amazon OpenSearch Service supports. |
| `ListVpcEndpointAccess` | `GET /2021-01-01/opensearch/domain/{DomainName}/listVpcEndpointAccess` | - | `DomainName` | - | `ListVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Retrieves information about each Amazon Web Services principal that is allowed to access a given Amazon OpenSearch Service domain through the use of an interface VPC endpoint. |
| `ListVpcEndpoints` | `GET /2021-01-01/opensearch/vpcEndpoints` | - | - | - | `ListVpcEndpointsResponse` | `BaseException`, `DisabledOperationException`, `InternalException` | Retrieves all Amazon OpenSearch Service-managed VPC endpoints in the current Amazon Web Services account and Region. |
| `ListVpcEndpointsForDomain` | `GET /2021-01-01/opensearch/domain/{DomainName}/vpcEndpoints` | - | `DomainName` | - | `ListVpcEndpointsForDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Retrieves all Amazon OpenSearch Service-managed VPC endpoints associated with a particular domain. |
| `PurchaseReservedInstanceOffering` | `POST /2021-01-01/opensearch/purchaseReservedInstanceOffering` | - | `ReservationName`, `ReservedInstanceOfferingId` | - | `PurchaseReservedInstanceOfferingResponse` | `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ValidationException` | Allows you to purchase Amazon OpenSearch Service Reserved Instances. |
| `PutDefaultApplicationSetting` | `PUT /2021-01-01/opensearch/defaultApplicationSetting` | - | `applicationArn`, `setAsDefault` | - | `PutDefaultApplicationSettingResponse` | `AccessDeniedException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Sets the default application to the application with the specified ARN. To remove the default application, use the `GetDefaultApplicationSetting` operation to get the current default and then call the `PutDefaultApplicationSetting` with the current... |
| `RejectInboundConnection` | `PUT /2021-01-01/opensearch/cc/inboundConnection/{ConnectionId}/reject` | - | `ConnectionId` | - | `RejectInboundConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the remote Amazon OpenSearch Service domain owner to reject an inbound cross-cluster connection request. |
| `RemoveTags` | `POST /2021-01-01/tags-removal` | - | `ARN`, `TagKeys` | - | `Unit` | `BaseException`, `InternalException`, `ValidationException` | Removes the specified set of tags from an Amazon OpenSearch Service domain, data source, or application. For more information, see Tagging Amazon OpenSearch Service resources. |
| `RevokeVpcEndpointAccess` | `POST /2021-01-01/opensearch/domain/{DomainName}/revokeVpcEndpointAccess` | - | `DomainName` | - | `RevokeVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Revokes access to an Amazon OpenSearch Service domain that was provided through an interface VPC endpoint. |
| `StartDomainMaintenance` | `POST /2021-01-01/opensearch/domain/{DomainName}/domainMaintenance` | - | `Action`, `DomainName` | - | `StartDomainMaintenanceResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Starts the node maintenance process on the data node. These processes can include a node reboot, an Opensearch or Elasticsearch process restart, or a Dashboard or Kibana restart. |
| `StartServiceSoftwareUpdate` | `POST /2021-01-01/opensearch/serviceSoftwareUpdate/start` | - | `DomainName` | - | `StartServiceSoftwareUpdateResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Schedules a service software update for an Amazon OpenSearch Service domain. For more information, see Service software updates in Amazon OpenSearch Service. |
| `UpdateApplication` | `PUT /2021-01-01/opensearch/application/{id}` | - | `id` | - | `UpdateApplicationResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Updates the configuration and settings of an existing OpenSearch application. |
| `UpdateDataSource` | `PUT /2021-01-01/opensearch/domain/{DomainName}/dataSource/{Name}` | - | `DataSourceType`, `DomainName`, `Name` | - | `UpdateDataSourceResponse` | `BaseException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Updates a direct-query data source. For more information, see Working with Amazon OpenSearch Service data source integrations with Amazon S3. |
| `UpdateDirectQueryDataSource` | `PUT /2021-01-01/opensearch/directQueryDataSource/{DataSourceName}` | - | `DataSourceName`, `DataSourceType`, `OpenSearchArns` | - | `UpdateDirectQueryDataSourceResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Updates the configuration or properties of an existing direct query data source in Amazon OpenSearch Service. |
| `UpdateDomainConfig` | `POST /2021-01-01/opensearch/domain/{DomainName}/config` | - | `DomainName` | - | `UpdateDomainConfigResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Modifies the cluster configuration of the specified Amazon OpenSearch Service domain. |
| `UpdateIndex` | `PUT /2021-01-01/opensearch/domain/{DomainName}/index/{IndexName}` | - | `DomainName`, `IndexName`, `IndexSchema` | - | `UpdateIndexResponse` | `AccessDeniedException`, `DependencyFailureException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing OpenSearch index schema and semantic enrichment configuration. This operation allows modification of field mappings and semantic search settings for text fields. |
| `UpdatePackage` | `POST /2021-01-01/packages/update` | - | `PackageID`, `PackageSource` | - | `UpdatePackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Updates a package for use with Amazon OpenSearch Service domains. For more information, see Custom packages for Amazon OpenSearch Service. |
| `UpdatePackageScope` | `POST /2021-01-01/packages/updateScope` | - | `Operation`, `PackageID`, `PackageUserList` | - | `UpdatePackageScopeResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Updates the scope of a package. Scope of the package defines users who can view and associate a package. |
| `UpdateScheduledAction` | `PUT /2021-01-01/opensearch/domain/{DomainName}/scheduledAction/update` | - | `ActionID`, `ActionType`, `DomainName`, `ScheduleAt` | - | `UpdateScheduledActionResponse` | `BaseException`, `ConflictException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `SlotNotAvailableException`, `ValidationException` | Reschedules a planned domain configuration change for a later time. This change can be a scheduled service software update or a blue/green Auto-Tune enhancement. |
| `UpdateVpcEndpoint` | `POST /2021-01-01/opensearch/vpcEndpoints/update` | - | `VpcEndpointId`, `VpcOptions` | - | `UpdateVpcEndpointResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Modifies an Amazon OpenSearch Service-managed interface VPC endpoint. |
| `UpgradeDomain` | `POST /2021-01-01/opensearch/upgradeDomain` | - | `DomainName`, `TargetVersion` | - | `UpgradeDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ValidationException` | Allows you to either upgrade your Amazon OpenSearch Service domain or perform an upgrade eligibility check to a compatible version of OpenSearch or Elasticsearch. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeDomainAutoTunes` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `DescribeDomainChangeProgress` | - | `ChangeId -> changeid` | - | - |
| `DescribeDryRunProgress` | - | `DryRunId -> dryRunId`, `LoadDryRunConfig -> loadDryRunConfig` | - | - |
| `DescribeInstanceTypeLimits` | - | `DomainName -> domainName` | - | - |
| `DescribeReservedInstanceOfferings` | - | `ReservedInstanceOfferingId -> offeringId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `DescribeReservedInstances` | - | `ReservedInstanceId -> reservationId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetCompatibleVersions` | - | `DomainName -> domainName` | - | - |
| `GetDomainMaintenanceStatus` | - | `MaintenanceId -> maintenanceId` | - | - |
| `GetPackageVersionHistory` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetUpgradeHistory` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListApplications` | - | `nextToken -> nextToken`, `statuses -> statuses`, `maxResults -> maxResults` | - | - |
| `ListDirectQueryDataSources` | - | `NextToken -> nexttoken` | - | - |
| `ListDomainMaintenances` | - | `Action -> action`, `Status -> status`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListDomainNames` | - | `EngineType -> engineType` | - | - |
| `ListDomainsForPackage` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListInstanceTypeDetails` | - | `DomainName -> domainName`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `RetrieveAZs -> retrieveAZs`, `InstanceType -> instanceType` | - | - |
| `ListPackagesForDomain` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListScheduledActions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListTags` | - | `ARN -> arn` | - | - |
| `ListVersions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListVpcEndpointAccess` | - | `NextToken -> nextToken` | - | - |
| `ListVpcEndpoints` | - | `NextToken -> nextToken` | - | - |
| `ListVpcEndpointsForDomain` | - | `NextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | `message` | Request processing failed because of an unknown error, exception, or internal failure. |
| `ValidationException` | `structure` | `message` | An exception for accessing or deleting a resource that doesn't exist. |
| `ResourceNotFoundException` | `structure` | `message` | An exception for accessing or deleting a resource that doesn't exist. |
| `BaseException` | `structure` | `message` | An error occurred while processing the request. |
| `DisabledOperationException` | `structure` | `message` | An error occured because the client wanted to access an unsupported operation. |
| `AccessDeniedException` | `structure` | `message` | An error occurred because you don't have permissions to access the resource. |
| `LimitExceededException` | `structure` | `message` | An exception for trying to create more than the allowed number of resources or sub-resources. |
| `ConflictException` | `structure` | `message` | An error occurred because the client attempts to remove a resource that is currently in use. |
| `DependencyFailureException` | `structure` | `message` | An exception for when a failure in one of the dependencies results in the service being unable to fetch details about the resource. |
| `ResourceAlreadyExistsException` | `structure` | `message` | An exception for creating a resource that already exists. |
| `InvalidTypeException` | `structure` | `message` | An exception for trying to create or access a sub-resource that's either invalid or not supported. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `InvalidPaginationTokenException` | `structure` | `message` | Request processing failed because you provided an invalid pagination token. |
| `AcceptInboundConnectionRequest` | `structure` | `ConnectionId` | Container for the parameters to the `AcceptInboundConnection` operation. |
| `AcceptInboundConnectionResponse` | `structure` | `Connection` | Contains details about the accepted inbound connection. |
| `AddDataSourceRequest` | `structure` | `DataSourceType`, `Description`, `DomainName`, `Name` | Container for the parameters to the `AddDataSource` operation. |
| `AddDataSourceResponse` | `structure` | `Message` | The result of an `AddDataSource` operation. |
| `AddDirectQueryDataSourceRequest` | `structure` | `DataSourceAccessPolicy`, `DataSourceName`, `DataSourceType`, `Description`, `OpenSearchArns`, `TagList` | - |
| `AddDirectQueryDataSourceResponse` | `structure` | `DataSourceArn` | - |
| `AddTagsRequest` | `structure` | `ARN`, `TagList` | Container for the parameters to the `AddTags` operation. |
| `AssociatePackageRequest` | `structure` | `AssociationConfiguration`, `DomainName`, `PackageID`, `PrerequisitePackageIDList` | Container for the request parameters to the `AssociatePackage` operation. |
| `AssociatePackageResponse` | `structure` | `DomainPackageDetails` | Container for the response returned by the `AssociatePackage` operation. |
| `AssociatePackagesRequest` | `structure` | `DomainName`, `PackageList` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

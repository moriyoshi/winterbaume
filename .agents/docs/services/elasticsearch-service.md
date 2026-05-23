# Amazon Elasticsearch Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elasticsearch Configuration Service Use the Amazon Elasticsearch Configuration API to create, configure, and manage Elasticsearch domains. For sample code that uses the Configuration API, see the Amazon Elasticsearch Service Developer Guide. The guide also contains sample code for sending signed HTTP requests to the Elasticsearch APIs. The endpoint for configuration service requests is region-specific: es. region .amazonaws.com.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Elasticsearch Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Elasticsearch Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Elasticsearch Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Elasticsearch Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Delete`, `Create`, `Get` operation families, including `DescribeDomainAutoTunes`, `DescribeDomainChangeProgress`, `DescribeElasticsearchDomain`, `DescribeElasticsearchDomainConfig`, `ListDomainNames`, `ListDomainsForPackage`.

## Service Identity and Protocol

- AWS model slug: `elasticsearch-service`
- AWS SDK for Rust slug: `elasticsearchservice`
- Model version: `2015-01-01`
- Model file: `vendor/api-models-aws/models/elasticsearch-service/service/2015-01-01/elasticsearch-service-2015-01-01.json`
- SDK ID: `Elasticsearch Service`
- Endpoint prefix: `es`
- ARN namespace: `es`
- CloudFormation name: `Elasticsearch`
- CloudTrail event source: `elasticsearchservice.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (12), `List` (9), `Delete` (6), `Create` (4), `Get` (4), `Update` (3), `Cancel` (2), `Accept` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptInboundCrossClusterSearchConnection`, `AddTags`, `AssociatePackage`, `CancelDomainConfigChange`, `CancelElasticsearchServiceSoftwareUpdate`, `CreateElasticsearchDomain`, `CreateOutboundCrossClusterSearchConnection`, `CreatePackage`, `CreateVpcEndpoint`, `DeleteElasticsearchDomain`, `DeleteElasticsearchServiceRole`, `DeleteInboundCrossClusterSearchConnection`, `DeleteOutboundCrossClusterSearchConnection`, `DeletePackage`, `DeleteVpcEndpoint`, `RejectInboundCrossClusterSearchConnection`, `RemoveTags`, `RevokeVpcEndpointAccess`, `StartElasticsearchServiceSoftwareUpdate`, `UpdateElasticsearchDomainConfig`, ... (+2).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDomainAutoTunes`, `DescribeDomainChangeProgress`, `DescribeElasticsearchDomain`, `DescribeElasticsearchDomainConfig`, `DescribeElasticsearchDomains`, `DescribeElasticsearchInstanceTypeLimits`, `DescribeInboundCrossClusterSearchConnections`, `DescribeOutboundCrossClusterSearchConnections`, `DescribePackages`, `DescribeReservedElasticsearchInstanceOfferings`, `DescribeReservedElasticsearchInstances`, `DescribeVpcEndpoints`, `GetCompatibleElasticsearchVersions`, `GetPackageVersionHistory`, `GetUpgradeHistory`, `GetUpgradeStatus`, `ListDomainNames`, `ListDomainsForPackage`, `ListElasticsearchInstanceTypes`, `ListElasticsearchVersions`, ... (+5).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelDomainConfigChange`, `CancelElasticsearchServiceSoftwareUpdate`, `StartElasticsearchServiceSoftwareUpdate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 51 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECS`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/opensearch-service/latest/developerguide/what-is.html
- https://docs.aws.amazon.com/opensearch-service/latest/developerguide/managedomains-configuration-changes.html

Research outcomes:
- Amazon Elasticsearch Service is the predecessor API/service generation for Amazon OpenSearch Service domains.
- Domains encapsulate engine version, cluster configuration, storage, access policy, VPC options, endpoint, snapshots, encryption, and logging.
- Many configuration changes use blue/green deployment mechanics with domain status and change-progress tracking.
- Engine upgrades, storage changes, VPC options, encryption, and fine-grained access control can have different update paths and validation.
- Access to Dashboards and domain APIs depends on endpoint, network, IAM/resource policy, and fine-grained access control where enabled.

Parity implications:
- Preserve legacy Elasticsearch Service API naming while modelling OpenSearch-domain-like resources and lifecycle.
- Domain mutations should expose asynchronous processing and blue/green-style statuses.
- Legacy service aliases should not imply a separate data model from OpenSearch Service unless API shape differs.

## Operation Groups

### Describe

- Operations: `DescribeDomainAutoTunes`, `DescribeDomainChangeProgress`, `DescribeElasticsearchDomain`, `DescribeElasticsearchDomainConfig`, `DescribeElasticsearchDomains`, `DescribeElasticsearchInstanceTypeLimits`, `DescribeInboundCrossClusterSearchConnections`, `DescribeOutboundCrossClusterSearchConnections`, `DescribePackages`, `DescribeReservedElasticsearchInstanceOfferings`, `DescribeReservedElasticsearchInstances`, `DescribeVpcEndpoints`
- Traits: `paginated` (6)
- Common required input members in this group: `DomainName`, `DomainNames`, `ElasticsearchVersion`, `InstanceType`, `VpcEndpointIds`

### List

- Operations: `ListDomainNames`, `ListDomainsForPackage`, `ListElasticsearchInstanceTypes`, `ListElasticsearchVersions`, `ListPackagesForDomain`, `ListTags`, `ListVpcEndpointAccess`, `ListVpcEndpoints`, `ListVpcEndpointsForDomain`
- Traits: `paginated` (4)
- Common required input members in this group: `ARN`, `DomainName`, `ElasticsearchVersion`, `PackageID`

### Delete

- Operations: `DeleteElasticsearchDomain`, `DeleteElasticsearchServiceRole`, `DeleteInboundCrossClusterSearchConnection`, `DeleteOutboundCrossClusterSearchConnection`, `DeletePackage`, `DeleteVpcEndpoint`
- Common required input members in this group: `CrossClusterSearchConnectionId`, `DomainName`, `PackageID`, `VpcEndpointId`

### Create

- Operations: `CreateElasticsearchDomain`, `CreateOutboundCrossClusterSearchConnection`, `CreatePackage`, `CreateVpcEndpoint`
- Common required input members in this group: `ConnectionAlias`, `DestinationDomainInfo`, `DomainArn`, `DomainName`, `PackageName`, `PackageSource`, `PackageType`, `SourceDomainInfo`, `VpcOptions`

### Get

- Operations: `GetCompatibleElasticsearchVersions`, `GetPackageVersionHistory`, `GetUpgradeHistory`, `GetUpgradeStatus`
- Traits: `paginated` (2)
- Common required input members in this group: `DomainName`, `PackageID`

### Update

- Operations: `UpdateElasticsearchDomainConfig`, `UpdatePackage`, `UpdateVpcEndpoint`
- Common required input members in this group: `DomainName`, `PackageID`, `PackageSource`, `VpcEndpointId`, `VpcOptions`

### Cancel

- Operations: `CancelDomainConfigChange`, `CancelElasticsearchServiceSoftwareUpdate`
- Common required input members in this group: `DomainName`

### Accept

- Operations: `AcceptInboundCrossClusterSearchConnection`
- Common required input members in this group: `CrossClusterSearchConnectionId`

### Add

- Operations: `AddTags`
- Common required input members in this group: `ARN`, `TagList`

### Associate

- Operations: `AssociatePackage`
- Common required input members in this group: `DomainName`, `PackageID`

### Authorize

- Operations: `AuthorizeVpcEndpointAccess`
- Common required input members in this group: `Account`, `DomainName`

### Dissociate

- Operations: `DissociatePackage`
- Common required input members in this group: `DomainName`, `PackageID`

### Purchase

- Operations: `PurchaseReservedElasticsearchInstanceOffering`
- Common required input members in this group: `ReservationName`, `ReservedElasticsearchInstanceOfferingId`

### Reject

- Operations: `RejectInboundCrossClusterSearchConnection`
- Common required input members in this group: `CrossClusterSearchConnectionId`

### Remove

- Operations: `RemoveTags`
- Common required input members in this group: `ARN`, `TagKeys`

### Revoke

- Operations: `RevokeVpcEndpointAccess`
- Common required input members in this group: `Account`, `DomainName`

### Start

- Operations: `StartElasticsearchServiceSoftwareUpdate`
- Common required input members in this group: `DomainName`

### Upgrade

- Operations: `UpgradeElasticsearchDomain`
- Common required input members in this group: `DomainName`, `TargetVersion`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptInboundCrossClusterSearchConnection` | `PUT /2015-01-01/es/ccs/inboundConnection/{CrossClusterSearchConnectionId}/accept` | - | `CrossClusterSearchConnectionId` | - | `AcceptInboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Allows the destination domain owner to accept an inbound cross-cluster search connection request. |
| `AddTags` | `POST /2015-01-01/tags` | - | `ARN`, `TagList` | - | `Unit` | `BaseException`, `InternalException`, `LimitExceededException`, `ValidationException` | Attaches tags to an existing Elasticsearch domain. Tags are a set of case-sensitive key value pairs. |
| `AssociatePackage` | `POST /2015-01-01/packages/associate/{PackageID}/{DomainName}` | - | `DomainName`, `PackageID` | - | `AssociatePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Associates a package with an Amazon ES domain. |
| `AuthorizeVpcEndpointAccess` | `POST /2015-01-01/es/domain/{DomainName}/authorizeVpcEndpointAccess` | - | `Account`, `DomainName` | - | `AuthorizeVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Provides access to an Amazon OpenSearch Service domain through the use of an interface VPC endpoint. |
| `CancelDomainConfigChange` | `POST /2015-01-01/es/domain/{DomainName}/config/cancel` | - | `DomainName` | - | `CancelDomainConfigChangeResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Cancels a pending configuration change on an Amazon OpenSearch Service domain. |
| `CancelElasticsearchServiceSoftwareUpdate` | `POST /2015-01-01/es/serviceSoftwareUpdate/cancel` | - | `DomainName` | - | `CancelElasticsearchServiceSoftwareUpdateResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Cancels a scheduled service software update for an Amazon ES domain. You can only perform this operation before the `AutomatedUpdateDate` and when the `UpdateStatus` is in the `PENDING_UPDATE` state. |
| `CreateElasticsearchDomain` | `POST /2015-01-01/es/domain` | - | `DomainName` | - | `CreateElasticsearchDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates a new Elasticsearch domain. For more information, see Creating Elasticsearch Domains in the Amazon Elasticsearch Service Developer Guide . |
| `CreateOutboundCrossClusterSearchConnection` | `POST /2015-01-01/es/ccs/outboundConnection` | - | `ConnectionAlias`, `DestinationDomainInfo`, `SourceDomainInfo` | - | `CreateOutboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates a new cross-cluster search connection from a source domain to a destination domain. |
| `CreatePackage` | `POST /2015-01-01/packages` | - | `PackageName`, `PackageSource`, `PackageType` | - | `CreatePackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Create a package for use with Amazon ES domains. |
| `CreateVpcEndpoint` | `POST /2015-01-01/es/vpcEndpoints` | - | `DomainArn`, `VpcOptions` | - | `CreateVpcEndpointResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ValidationException` | Creates an Amazon OpenSearch Service-managed VPC endpoint. |
| `DeleteElasticsearchDomain` | `DELETE /2015-01-01/es/domain/{DomainName}` | - | `DomainName` | - | `DeleteElasticsearchDomainResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Permanently deletes the specified Elasticsearch domain and all of its data. Once a domain is deleted, it cannot be recovered. |
| `DeleteElasticsearchServiceRole` | `DELETE /2015-01-01/es/role` | - | - | - | `Unit` | `BaseException`, `InternalException`, `ValidationException` | Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. |
| `DeleteInboundCrossClusterSearchConnection` | `DELETE /2015-01-01/es/ccs/inboundConnection/{CrossClusterSearchConnectionId}` | - | `CrossClusterSearchConnectionId` | - | `DeleteInboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the destination domain owner to delete an existing inbound cross-cluster search connection. |
| `DeleteOutboundCrossClusterSearchConnection` | `DELETE /2015-01-01/es/ccs/outboundConnection/{CrossClusterSearchConnectionId}` | - | `CrossClusterSearchConnectionId` | - | `DeleteOutboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the source domain owner to delete an existing outbound cross-cluster search connection. |
| `DeletePackage` | `DELETE /2015-01-01/packages/{PackageID}` | - | `PackageID` | - | `DeletePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Delete the package. |
| `DeleteVpcEndpoint` | `DELETE /2015-01-01/es/vpcEndpoints/{VpcEndpointId}` | - | `VpcEndpointId` | - | `DeleteVpcEndpointResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Deletes an Amazon OpenSearch Service-managed interface VPC endpoint. |
| `DescribeDomainAutoTunes` | `GET /2015-01-01/es/domain/{DomainName}/autoTunes` | `paginated` | `DomainName` | - | `DescribeDomainAutoTunesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Provides scheduled Auto-Tune action details for the Elasticsearch domain, such as Auto-Tune action type, description, severity, and scheduled date. |
| `DescribeDomainChangeProgress` | `GET /2015-01-01/es/domain/{DomainName}/progress` | - | `DomainName` | - | `DescribeDomainChangeProgressResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about the current blue/green deployment happening on a domain, including a change ID, status, and progress stages. |
| `DescribeElasticsearchDomain` | `GET /2015-01-01/es/domain/{DomainName}` | - | `DomainName` | - | `DescribeElasticsearchDomainResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns domain configuration information about the specified Elasticsearch domain, including the domain ID, domain endpoint, and domain ARN. |
| `DescribeElasticsearchDomainConfig` | `GET /2015-01-01/es/domain/{DomainName}/config` | - | `DomainName` | - | `DescribeElasticsearchDomainConfigResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Provides cluster configuration information about the specified Elasticsearch domain, such as the state, creation date, update version, and update date for cluster options. |
| `DescribeElasticsearchDomains` | `POST /2015-01-01/es/domain-info` | - | `DomainNames` | - | `DescribeElasticsearchDomainsResponse` | `BaseException`, `InternalException`, `ValidationException` | Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN. |
| `DescribeElasticsearchInstanceTypeLimits` | `GET /2015-01-01/es/instanceTypeLimits/{ElasticsearchVersion}/{InstanceType}` | - | `ElasticsearchVersion`, `InstanceType` | - | `DescribeElasticsearchInstanceTypeLimitsResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Describe Elasticsearch Limits for a given InstanceType and ElasticsearchVersion. When modifying existing Domain, specify the ` DomainName ` to know what Limits are supported for modifying. |
| `DescribeInboundCrossClusterSearchConnections` | `POST /2015-01-01/es/ccs/inboundConnection/search` | `paginated` | - | - | `DescribeInboundCrossClusterSearchConnectionsResponse` | `DisabledOperationException`, `InvalidPaginationTokenException` | Lists all the inbound cross-cluster search connections for a destination domain. |
| `DescribeOutboundCrossClusterSearchConnections` | `POST /2015-01-01/es/ccs/outboundConnection/search` | `paginated` | - | - | `DescribeOutboundCrossClusterSearchConnectionsResponse` | `DisabledOperationException`, `InvalidPaginationTokenException` | Lists all the outbound cross-cluster search connections for a source domain. |
| `DescribePackages` | `POST /2015-01-01/packages/describe` | `paginated` | - | - | `DescribePackagesResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes all packages available to Amazon ES. Includes options for filtering, limiting the number of results, and pagination. |
| `DescribeReservedElasticsearchInstanceOfferings` | `GET /2015-01-01/es/reservedInstanceOfferings` | `paginated` | - | - | `DescribeReservedElasticsearchInstanceOfferingsResponse` | `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists available reserved Elasticsearch instance offerings. |
| `DescribeReservedElasticsearchInstances` | `GET /2015-01-01/es/reservedInstances` | `paginated` | - | - | `DescribeReservedElasticsearchInstancesResponse` | `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about reserved Elasticsearch instances for this account. |
| `DescribeVpcEndpoints` | `POST /2015-01-01/es/vpcEndpoints/describe` | - | `VpcEndpointIds` | - | `DescribeVpcEndpointsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ValidationException` | Describes one or more Amazon OpenSearch Service-managed VPC endpoints. |
| `DissociatePackage` | `POST /2015-01-01/packages/dissociate/{PackageID}/{DomainName}` | - | `DomainName`, `PackageID` | - | `DissociatePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Dissociates a package from the Amazon ES domain. |
| `GetCompatibleElasticsearchVersions` | `GET /2015-01-01/es/compatibleVersions` | - | - | - | `GetCompatibleElasticsearchVersionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of upgrade compatible Elastisearch versions. You can optionally pass a ` DomainName ` to get all upgrade compatible Elasticsearch versions for that specific domain. |
| `GetPackageVersionHistory` | `GET /2015-01-01/packages/{PackageID}/history` | `paginated` | `PackageID` | - | `GetPackageVersionHistoryResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of versions of the package, along with their creation time and commit message. |
| `GetUpgradeHistory` | `GET /2015-01-01/es/upgradeDomain/{DomainName}/history` | `paginated` | `DomainName` | - | `GetUpgradeHistoryResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the complete history of the last 10 upgrades that were performed on the domain. |
| `GetUpgradeStatus` | `GET /2015-01-01/es/upgradeDomain/{DomainName}/status` | - | `DomainName` | - | `GetUpgradeStatusResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the latest status of the last upgrade or upgrade eligibility check that was performed on the domain. |
| `ListDomainNames` | `GET /2015-01-01/domain` | - | - | - | `ListDomainNamesResponse` | `BaseException`, `ValidationException` | Returns the name of all Elasticsearch domains owned by the current user's account. |
| `ListDomainsForPackage` | `GET /2015-01-01/packages/{PackageID}/domains` | `paginated` | `PackageID` | - | `ListDomainsForPackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all Amazon ES domains associated with the package. |
| `ListElasticsearchInstanceTypes` | `GET /2015-01-01/es/instanceTypes/{ElasticsearchVersion}` | `paginated` | `ElasticsearchVersion` | - | `ListElasticsearchInstanceTypesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | List all Elasticsearch instance types that are supported for given ElasticsearchVersion |
| `ListElasticsearchVersions` | `GET /2015-01-01/es/versions` | `paginated` | - | - | `ListElasticsearchVersionsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | List all supported Elasticsearch versions |
| `ListPackagesForDomain` | `GET /2015-01-01/domain/{DomainName}/packages` | `paginated` | `DomainName` | - | `ListPackagesForDomainResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists all packages associated with the Amazon ES domain. |
| `ListTags` | `GET /2015-01-01/tags` | - | `ARN` | - | `ListTagsResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns all tags for the given Elasticsearch domain. |
| `ListVpcEndpointAccess` | `GET /2015-01-01/es/domain/{DomainName}/listVpcEndpointAccess` | - | `DomainName` | - | `ListVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Retrieves information about each principal that is allowed to access a given Amazon OpenSearch Service domain through the use of an interface VPC endpoint. |
| `ListVpcEndpoints` | `GET /2015-01-01/es/vpcEndpoints` | - | - | - | `ListVpcEndpointsResponse` | `BaseException`, `DisabledOperationException`, `InternalException` | Retrieves all Amazon OpenSearch Service-managed VPC endpoints in the current account and Region. |
| `ListVpcEndpointsForDomain` | `GET /2015-01-01/es/domain/{DomainName}/vpcEndpoints` | - | `DomainName` | - | `ListVpcEndpointsForDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Retrieves all Amazon OpenSearch Service-managed VPC endpoints associated with a particular domain. |
| `PurchaseReservedElasticsearchInstanceOffering` | `POST /2015-01-01/es/purchaseReservedInstanceOffering` | - | `ReservationName`, `ReservedElasticsearchInstanceOfferingId` | - | `PurchaseReservedElasticsearchInstanceOfferingResponse` | `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ValidationException` | Allows you to purchase reserved Elasticsearch instances. |
| `RejectInboundCrossClusterSearchConnection` | `PUT /2015-01-01/es/ccs/inboundConnection/{CrossClusterSearchConnectionId}/reject` | - | `CrossClusterSearchConnectionId` | - | `RejectInboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the destination domain owner to reject an inbound cross-cluster search connection request. |
| `RemoveTags` | `POST /2015-01-01/tags-removal` | - | `ARN`, `TagKeys` | - | `Unit` | `BaseException`, `InternalException`, `ValidationException` | Removes the specified set of tags from the specified Elasticsearch domain. |
| `RevokeVpcEndpointAccess` | `POST /2015-01-01/es/domain/{DomainName}/revokeVpcEndpointAccess` | - | `Account`, `DomainName` | - | `RevokeVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Revokes access to an Amazon OpenSearch Service domain that was provided through an interface VPC endpoint. |
| `StartElasticsearchServiceSoftwareUpdate` | `POST /2015-01-01/es/serviceSoftwareUpdate/start` | - | `DomainName` | - | `StartElasticsearchServiceSoftwareUpdateResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Schedules a service software update for an Amazon ES domain. |
| `UpdateElasticsearchDomainConfig` | `POST /2015-01-01/es/domain/{DomainName}/config` | - | `DomainName` | - | `UpdateElasticsearchDomainConfigResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Modifies the cluster configuration of the specified Elasticsearch domain, setting as setting the instance type and the number of instances. |
| `UpdatePackage` | `POST /2015-01-01/packages/update` | - | `PackageID`, `PackageSource` | - | `UpdatePackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Updates a package for use with Amazon ES domains. |
| `UpdateVpcEndpoint` | `POST /2015-01-01/es/vpcEndpoints/update` | - | `VpcEndpointId`, `VpcOptions` | - | `UpdateVpcEndpointResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Modifies an Amazon OpenSearch Service-managed interface VPC endpoint. |
| `UpgradeElasticsearchDomain` | `POST /2015-01-01/es/upgradeDomain` | - | `DomainName`, `TargetVersion` | - | `UpgradeElasticsearchDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ValidationException` | Allows you to either upgrade your domain or perform an Upgrade eligibility check to a compatible Elasticsearch version. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeDomainAutoTunes` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `DescribeDomainChangeProgress` | - | `ChangeId -> changeid` | - | - |
| `DescribeElasticsearchInstanceTypeLimits` | - | `DomainName -> domainName` | - | - |
| `DescribeReservedElasticsearchInstanceOfferings` | - | `ReservedElasticsearchInstanceOfferingId -> offeringId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `DescribeReservedElasticsearchInstances` | - | `ReservedElasticsearchInstanceId -> reservationId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetCompatibleElasticsearchVersions` | - | `DomainName -> domainName` | - | - |
| `GetPackageVersionHistory` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetUpgradeHistory` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListDomainNames` | - | `EngineType -> engineType` | - | - |
| `ListDomainsForPackage` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListElasticsearchInstanceTypes` | - | `DomainName -> domainName`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListElasticsearchVersions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListPackagesForDomain` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListTags` | - | `ARN -> arn` | - | - |
| `ListVpcEndpointAccess` | - | `NextToken -> nextToken` | - | - |
| `ListVpcEndpoints` | - | `NextToken -> nextToken` | - | - |
| `ListVpcEndpointsForDomain` | - | `NextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | `message` | The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . |
| `BaseException` | `structure` | `message` | An error occurred while processing the request. |
| `ValidationException` | `structure` | `message` | An exception for missing / invalid input fields. |
| `ResourceNotFoundException` | `structure` | `message` | An exception for accessing or deleting a resource that does not exist. |
| `DisabledOperationException` | `structure` | `message` | An error occured because the client wanted to access a not supported operation. |
| `LimitExceededException` | `structure` | `message` | An exception for trying to create more than allowed resources or sub-resources. |
| `AccessDeniedException` | `structure` | `message` | An error occurred because user does not have permissions to access the resource. |
| `ConflictException` | `structure` | `message` | An error occurred because the client attempts to remove a resource that is currently in use. |
| `ResourceAlreadyExistsException` | `structure` | `message` | An exception for creating a resource that already exists. |
| `InvalidTypeException` | `structure` | `message` | An exception for trying to create or access sub-resource that is either invalid or not supported. |
| `InvalidPaginationTokenException` | `structure` | `message` | The request processing has failed because of invalid pagination token provided by customer. |
| `AcceptInboundCrossClusterSearchConnectionRequest` | `structure` | `CrossClusterSearchConnectionId` | Container for the parameters to the `AcceptInboundCrossClusterSearchConnection` operation. |
| `AcceptInboundCrossClusterSearchConnectionResponse` | `structure` | `CrossClusterSearchConnection` | The result of a `AcceptInboundCrossClusterSearchConnection` operation. |
| `AddTagsRequest` | `structure` | `ARN`, `TagList` | Container for the parameters to the `AddTags` operation. |
| `AssociatePackageRequest` | `structure` | `DomainName`, `PackageID` | Container for request parameters to ` AssociatePackage ` operation. |
| `AssociatePackageResponse` | `structure` | `DomainPackageDetails` | Container for response returned by ` AssociatePackage ` operation. |
| `AuthorizeVpcEndpointAccessRequest` | `structure` | `Account`, `DomainName` | Container for request parameters to the `AuthorizeVpcEndpointAccess` operation. |
| `AuthorizeVpcEndpointAccessResponse` | `structure` | `AuthorizedPrincipal` | Container for response parameters to the `AuthorizeVpcEndpointAccess` operation. |
| `CancelDomainConfigChangeRequest` | `structure` | `DomainName`, `DryRun` | Container for parameters of the `CancelDomainConfigChange` operation. |
| `CancelDomainConfigChangeResponse` | `structure` | `CancelledChangeIds`, `CancelledChangeProperties`, `DryRun` | Contains the details of the cancelled domain config change. |
| `CancelElasticsearchServiceSoftwareUpdateRequest` | `structure` | `DomainName` | Container for the parameters to the `CancelElasticsearchServiceSoftwareUpdate` operation. |
| `CancelElasticsearchServiceSoftwareUpdateResponse` | `structure` | `ServiceSoftwareOptions` | The result of a `CancelElasticsearchServiceSoftwareUpdate` operation. |
| `CreateElasticsearchDomainRequest` | `structure` | `AccessPolicies`, `AdvancedOptions`, `AdvancedSecurityOptions`, `AutoTuneOptions`, `CognitoOptions`, `DeploymentStrategyOptions`, `DomainEndpointOptions`, `DomainName`, `EBSOptions`, `ElasticsearchClusterConfig`, `ElasticsearchVersion`, `EncryptionAtRestOptions`, ... (+5) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

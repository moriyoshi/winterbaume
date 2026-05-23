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
- Common required input members in this group: `DomainName`

### List

- Operations: `ListDomainNames`, `ListDomainsForPackage`, `ListElasticsearchInstanceTypes`, `ListElasticsearchVersions`, `ListPackagesForDomain`, `ListTags`, `ListVpcEndpointAccess`, `ListVpcEndpoints`, `ListVpcEndpointsForDomain`
- Traits: `paginated` (4)
- Common required input members in this group: `DomainName`

### Delete

- Operations: `DeleteElasticsearchDomain`, `DeleteElasticsearchServiceRole`, `DeleteInboundCrossClusterSearchConnection`, `DeleteOutboundCrossClusterSearchConnection`, `DeletePackage`, `DeleteVpcEndpoint`
- Common required input members in this group: `CrossClusterSearchConnectionId`

### Create

- Operations: `CreateElasticsearchDomain`, `CreateOutboundCrossClusterSearchConnection`, `CreatePackage`, `CreateVpcEndpoint`
- Common required input members in this group: -

### Get

- Operations: `GetCompatibleElasticsearchVersions`, `GetPackageVersionHistory`, `GetUpgradeHistory`, `GetUpgradeStatus`
- Traits: `paginated` (2)
- Common required input members in this group: `DomainName`

### Update

- Operations: `UpdateElasticsearchDomainConfig`, `UpdatePackage`, `UpdateVpcEndpoint`
- Common required input members in this group: -

### Cancel

- Operations: `CancelDomainConfigChange`, `CancelElasticsearchServiceSoftwareUpdate`
- Common required input members in this group: `DomainName`

### Accept

- Operations: `AcceptInboundCrossClusterSearchConnection`
- Common required input members in this group: -

### Add

- Operations: `AddTags`
- Common required input members in this group: -

### Associate

- Operations: `AssociatePackage`
- Common required input members in this group: -

### Authorize

- Operations: `AuthorizeVpcEndpointAccess`
- Common required input members in this group: -

### Dissociate

- Operations: `DissociatePackage`
- Common required input members in this group: -

### Purchase

- Operations: `PurchaseReservedElasticsearchInstanceOffering`
- Common required input members in this group: -

### Reject

- Operations: `RejectInboundCrossClusterSearchConnection`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTags`
- Common required input members in this group: -

### Revoke

- Operations: `RevokeVpcEndpointAccess`
- Common required input members in this group: -

### Start

- Operations: `StartElasticsearchServiceSoftwareUpdate`
- Common required input members in this group: -

### Upgrade

- Operations: `UpgradeElasticsearchDomain`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptInboundCrossClusterSearchConnection` | `PUT /2015-01-01/es/ccs/inboundConnection/{CrossClusterSearchConnectionId}/accept` | - | `CrossClusterSearchConnectionId` | - | `AcceptInboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `LimitExceededException`, `ResourceNotFoundException` | Allows the destination domain owner to accept an inbound cross-cluster search connection request. |
| `AddTags` | `POST /2015-01-01/tags` | - | `ARN`, `TagList` | - | `Unit` | `BaseException`, `InternalException`, `LimitExceededException`, `ValidationException` | Attaches tags to an existing Elasticsearch domain. Tags are a set of case-sensitive key value pairs. An Elasticsearch domain may have up to 10 tags. See Tagging Amazon Elasticsearch Service Domains for more information. |
| `AssociatePackage` | `POST /2015-01-01/packages/associate/{PackageID}/{DomainName}` | - | `PackageID`, `DomainName` | - | `AssociatePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Associates a package with an Amazon ES domain. |
| `AuthorizeVpcEndpointAccess` | `POST /2015-01-01/es/domain/{DomainName}/authorizeVpcEndpointAccess` | - | `DomainName`, `Account` | - | `AuthorizeVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Provides access to an Amazon OpenSearch Service domain through the use of an interface VPC endpoint. |
| `CancelDomainConfigChange` | `POST /2015-01-01/es/domain/{DomainName}/config/cancel` | - | `DomainName` | - | `CancelDomainConfigChangeResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Cancels a pending configuration change on an Amazon OpenSearch Service domain. |
| `CancelElasticsearchServiceSoftwareUpdate` | `POST /2015-01-01/es/serviceSoftwareUpdate/cancel` | - | `DomainName` | - | `CancelElasticsearchServiceSoftwareUpdateResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Cancels a scheduled service software update for an Amazon ES domain. You can only perform this operation before the AutomatedUpdateDate and when the UpdateStatus is in the PENDING_UPDATE state. |
| `CreateElasticsearchDomain` | `POST /2015-01-01/es/domain` | - | `DomainName` | - | `CreateElasticsearchDomainResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates a new Elasticsearch domain. For more information, see Creating Elasticsearch Domains in the Amazon Elasticsearch Service Developer Guide . |
| `CreateOutboundCrossClusterSearchConnection` | `POST /2015-01-01/es/ccs/outboundConnection` | - | `SourceDomainInfo`, `DestinationDomainInfo`, `ConnectionAlias` | - | `CreateOutboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates a new cross-cluster search connection from a source domain to a destination domain. |
| `CreatePackage` | `POST /2015-01-01/packages` | - | `PackageName`, `PackageType`, `PackageSource` | - | `CreatePackageResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Create a package for use with Amazon ES domains. |
| `CreateVpcEndpoint` | `POST /2015-01-01/es/vpcEndpoints` | - | `DomainArn`, `VpcOptions` | - | `CreateVpcEndpointResponse` | `BaseException`, `ConflictException`, `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ValidationException` | Creates an Amazon OpenSearch Service-managed VPC endpoint. |
| `DeleteElasticsearchDomain` | `DELETE /2015-01-01/es/domain/{DomainName}` | - | `DomainName` | - | `DeleteElasticsearchDomainResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Permanently deletes the specified Elasticsearch domain and all of its data. Once a domain is deleted, it cannot be recovered. |
| `DeleteElasticsearchServiceRole` | `DELETE /2015-01-01/es/role` | - | - | - | `Unit` | `BaseException`, `InternalException`, `ValidationException` | Deletes the service-linked role that Elasticsearch Service uses to manage and maintain VPC domains. Role deletion will fail if any existing VPC domains use the role. You must delete any such Elasticsearch domains bef ... |
| `DeleteInboundCrossClusterSearchConnection` | `DELETE /2015-01-01/es/ccs/inboundConnection/{CrossClusterSearchConnectionId}` | - | `CrossClusterSearchConnectionId` | - | `DeleteInboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the destination domain owner to delete an existing inbound cross-cluster search connection. |
| `DeleteOutboundCrossClusterSearchConnection` | `DELETE /2015-01-01/es/ccs/outboundConnection/{CrossClusterSearchConnectionId}` | - | `CrossClusterSearchConnectionId` | - | `DeleteOutboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the source domain owner to delete an existing outbound cross-cluster search connection. |
| `DeletePackage` | `DELETE /2015-01-01/packages/{PackageID}` | - | `PackageID` | - | `DeletePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Delete the package. |
| `DeleteVpcEndpoint` | `DELETE /2015-01-01/es/vpcEndpoints/{VpcEndpointId}` | - | `VpcEndpointId` | - | `DeleteVpcEndpointResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException` | Deletes an Amazon OpenSearch Service-managed interface VPC endpoint. |
| `DescribeDomainAutoTunes` | `GET /2015-01-01/es/domain/{DomainName}/autoTunes` | `paginated` | `DomainName` | - | `DescribeDomainAutoTunesResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Provides scheduled Auto-Tune action details for the Elasticsearch domain, such as Auto-Tune action type, description, severity, and scheduled date. |
| `DescribeDomainChangeProgress` | `GET /2015-01-01/es/domain/{DomainName}/progress` | - | `DomainName` | - | `DescribeDomainChangeProgressResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about the current blue/green deployment happening on a domain, including a change ID, status, and progress stages. |
| `DescribeElasticsearchDomain` | `GET /2015-01-01/es/domain/{DomainName}` | - | `DomainName` | - | `DescribeElasticsearchDomainResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns domain configuration information about the specified Elasticsearch domain, including the domain ID, domain endpoint, and domain ARN. |
| `DescribeElasticsearchDomainConfig` | `GET /2015-01-01/es/domain/{DomainName}/config` | - | `DomainName` | - | `DescribeElasticsearchDomainConfigResponse` | `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Provides cluster configuration information about the specified Elasticsearch domain, such as the state, creation date, update version, and update date for cluster options. |
| `DescribeElasticsearchDomains` | `POST /2015-01-01/es/domain-info` | - | `DomainNames` | - | `DescribeElasticsearchDomainsResponse` | `BaseException`, `InternalException`, `ValidationException` | Returns domain configuration information about the specified Elasticsearch domains, including the domain ID, domain endpoint, and domain ARN. |
| `DescribeElasticsearchInstanceTypeLimits` | `GET /2015-01-01/es/instanceTypeLimits/{ElasticsearchVersion}/{InstanceType}` | - | `InstanceType`, `ElasticsearchVersion` | - | `DescribeElasticsearchInstanceTypeLimitsResponse` | `BaseException`, `InternalException`, `InvalidTypeException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Describe Elasticsearch Limits for a given InstanceType and ElasticsearchVersion. When modifying existing Domain, specify the DomainName to know what Limits are supported for modifying. |
| `DescribeInboundCrossClusterSearchConnections` | `POST /2015-01-01/es/ccs/inboundConnection/search` | `paginated` | - | - | `DescribeInboundCrossClusterSearchConnectionsResponse` | `DisabledOperationException`, `InvalidPaginationTokenException` | Lists all the inbound cross-cluster search connections for a destination domain. |
| `DescribeOutboundCrossClusterSearchConnections` | `POST /2015-01-01/es/ccs/outboundConnection/search` | `paginated` | - | - | `DescribeOutboundCrossClusterSearchConnectionsResponse` | `DisabledOperationException`, `InvalidPaginationTokenException` | Lists all the outbound cross-cluster search connections for a source domain. |
| `DescribePackages` | `POST /2015-01-01/packages/describe` | `paginated` | - | - | `DescribePackagesResponse` | `AccessDeniedException`, `BaseException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Describes all packages available to Amazon ES. Includes options for filtering, limiting the number of results, and pagination. |
| `DescribeReservedElasticsearchInstanceOfferings` | `GET /2015-01-01/es/reservedInstanceOfferings` | `paginated` | - | - | `DescribeReservedElasticsearchInstanceOfferingsResponse` | `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Lists available reserved Elasticsearch instance offerings. |
| `DescribeReservedElasticsearchInstances` | `GET /2015-01-01/es/reservedInstances` | `paginated` | - | - | `DescribeReservedElasticsearchInstancesResponse` | `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns information about reserved Elasticsearch instances for this account. |
| `DescribeVpcEndpoints` | `POST /2015-01-01/es/vpcEndpoints/describe` | - | `VpcEndpointIds` | - | `DescribeVpcEndpointsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ValidationException` | Describes one or more Amazon OpenSearch Service-managed VPC endpoints. |
| `DissociatePackage` | `POST /2015-01-01/packages/dissociate/{PackageID}/{DomainName}` | - | `PackageID`, `DomainName` | - | `DissociatePackageResponse` | `AccessDeniedException`, `BaseException`, `ConflictException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Dissociates a package from the Amazon ES domain. |
| `GetCompatibleElasticsearchVersions` | `GET /2015-01-01/es/compatibleVersions` | - | - | - | `GetCompatibleElasticsearchVersionsResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of upgrade compatible Elastisearch versions. You can optionally pass a DomainName to get all upgrade compatible Elasticsearch versions for that specific domain. |
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
| `PurchaseReservedElasticsearchInstanceOffering` | `POST /2015-01-01/es/purchaseReservedInstanceOffering` | - | `ReservedElasticsearchInstanceOfferingId`, `ReservationName` | - | `PurchaseReservedElasticsearchInstanceOfferingResponse` | `DisabledOperationException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ValidationException` | Allows you to purchase reserved Elasticsearch instances. |
| `RejectInboundCrossClusterSearchConnection` | `PUT /2015-01-01/es/ccs/inboundConnection/{CrossClusterSearchConnectionId}/reject` | - | `CrossClusterSearchConnectionId` | - | `RejectInboundCrossClusterSearchConnectionResponse` | `DisabledOperationException`, `ResourceNotFoundException` | Allows the destination domain owner to reject an inbound cross-cluster search connection request. |
| `RemoveTags` | `POST /2015-01-01/tags-removal` | - | `ARN`, `TagKeys` | - | `Unit` | `BaseException`, `InternalException`, `ValidationException` | Removes the specified set of tags from the specified Elasticsearch domain. |
| `RevokeVpcEndpointAccess` | `POST /2015-01-01/es/domain/{DomainName}/revokeVpcEndpointAccess` | - | `DomainName`, `Account` | - | `RevokeVpcEndpointAccessResponse` | `BaseException`, `DisabledOperationException`, `InternalException`, `ResourceNotFoundException`, `ValidationException` | Revokes access to an Amazon OpenSearch Service domain that was provided through an interface VPC endpoint. |
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
| `AccessDeniedException` | `structure` | message | An error occurred because user does not have permissions to access the resource. Returns HTTP status code 403. |
| `BaseException` | `structure` | message | An error occurred while processing the request. |
| `ConflictException` | `structure` | message | An error occurred because the client attempts to remove a resource that is currently in use. Returns HTTP status code 409. |
| `DisabledOperationException` | `structure` | message | An error occured because the client wanted to access a not supported operation. Gives http status code of 409. |
| `InternalException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure (the failure is internal to the service) . Gives http status code of 500. |
| `InvalidPaginationTokenException` | `structure` | message | The request processing has failed because of invalid pagination token provided by customer. Returns an HTTP status code of 400. |
| `InvalidTypeException` | `structure` | message | An exception for trying to create or access sub-resource that is either invalid or not supported. Gives http status code of 409. |
| `LimitExceededException` | `structure` | message | An exception for trying to create more than allowed resources or sub-resources. Gives http status code of 409. |
| `ResourceAlreadyExistsException` | `structure` | message | An exception for creating a resource that already exists. Gives http status code of 400. |
| `ResourceNotFoundException` | `structure` | message | An exception for accessing or deleting a resource that does not exist. Gives http status code of 400. |
| `ValidationException` | `structure` | message | An exception for missing / invalid input fields. Gives http status code of 400. |
| `AcceptInboundCrossClusterSearchConnectionRequest` | `structure` | CrossClusterSearchConnectionId | Container for the parameters to the AcceptInboundCrossClusterSearchConnection operation. |
| `AcceptInboundCrossClusterSearchConnectionResponse` | `structure` | CrossClusterSearchConnection | The result of a AcceptInboundCrossClusterSearchConnection operation. Contains details of accepted inbound connection. |
| `AddTagsRequest` | `structure` | ARN, TagList | Container for the parameters to the AddTags operation. Specify the tags that you want to attach to the Elasticsearch domain. |
| `AssociatePackageRequest` | `structure` | PackageID, DomainName | Container for request parameters to AssociatePackage operation. |
| `AssociatePackageResponse` | `structure` | DomainPackageDetails | Container for response returned by AssociatePackage operation. |
| `AuthorizeVpcEndpointAccessRequest` | `structure` | DomainName, Account | Container for request parameters to the AuthorizeVpcEndpointAccess operation. Specifies the account to be permitted to manage VPC endpoints against the domain. |
| `AuthorizeVpcEndpointAccessResponse` | `structure` | AuthorizedPrincipal | Container for response parameters to the AuthorizeVpcEndpointAccess operation. Contains the account ID and the type of the account being authorized to acces ... |
| `CancelDomainConfigChangeRequest` | `structure` | DomainName, DryRun | Container for parameters of the CancelDomainConfigChange operation. |
| `CancelDomainConfigChangeResponse` | `structure` | DryRun, CancelledChangeIds, CancelledChangeProperties | Contains the details of the cancelled domain config change. |
| `CancelElasticsearchServiceSoftwareUpdateRequest` | `structure` | DomainName | Container for the parameters to the CancelElasticsearchServiceSoftwareUpdate operation. Specifies the name of the Elasticsearch domain that you wish to canc ... |
| `CancelElasticsearchServiceSoftwareUpdateResponse` | `structure` | ServiceSoftwareOptions | The result of a CancelElasticsearchServiceSoftwareUpdate operation. Contains the status of the update. |
| `CreateElasticsearchDomainRequest` | `structure` | DomainName, ElasticsearchVersion, ElasticsearchClusterConfig, EBSOptions, AccessPolicies, SnapshotOptions, VPCOptions, CognitoOptions, EncryptionAtRestOptions, NodeToNodeEncryptionOptions, AdvancedOptions, LogPublishingOptions, ... (+5) | - |
| `CreateElasticsearchDomainResponse` | `structure` | DomainStatus | The result of a CreateElasticsearchDomain operation. Contains the status of the newly created Elasticsearch domain. |
| `CreateOutboundCrossClusterSearchConnectionRequest` | `structure` | SourceDomainInfo, DestinationDomainInfo, ConnectionAlias | Container for the parameters to the CreateOutboundCrossClusterSearchConnection operation. |
| `CreateOutboundCrossClusterSearchConnectionResponse` | `structure` | SourceDomainInfo, DestinationDomainInfo, ConnectionAlias, ConnectionStatus, CrossClusterSearchConnectionId | The result of a CreateOutboundCrossClusterSearchConnection request. Contains the details of the newly created cross-cluster search connection. |
| `CreatePackageRequest` | `structure` | PackageName, PackageType, PackageDescription, PackageSource | Container for request parameters to CreatePackage operation. |
| `CreatePackageResponse` | `structure` | PackageDetails | Container for response returned by CreatePackage operation. |
| `CreateVpcEndpointRequest` | `structure` | DomainArn, VpcOptions, ClientToken | Container for the parameters to the CreateVpcEndpointRequest operation. |
| `CreateVpcEndpointResponse` | `structure` | VpcEndpoint | Container for response parameters to the CreateVpcEndpoint operation. Contains the configuration and status of the VPC Endpoint being created. |
| `DeleteElasticsearchDomainRequest` | `structure` | DomainName | Container for the parameters to the DeleteElasticsearchDomain operation. Specifies the name of the Elasticsearch domain that you want to delete. |
| `DeleteElasticsearchDomainResponse` | `structure` | DomainStatus | The result of a DeleteElasticsearchDomain request. Contains the status of the pending deletion, or no status if the domain and all of its resources have bee ... |
| `DeleteInboundCrossClusterSearchConnectionRequest` | `structure` | CrossClusterSearchConnectionId | Container for the parameters to the DeleteInboundCrossClusterSearchConnection operation. |
| `DeleteInboundCrossClusterSearchConnectionResponse` | `structure` | CrossClusterSearchConnection | The result of a DeleteInboundCrossClusterSearchConnection operation. Contains details of deleted inbound connection. |
| `DeleteOutboundCrossClusterSearchConnectionRequest` | `structure` | CrossClusterSearchConnectionId | Container for the parameters to the DeleteOutboundCrossClusterSearchConnection operation. |
| `DeleteOutboundCrossClusterSearchConnectionResponse` | `structure` | CrossClusterSearchConnection | The result of a DeleteOutboundCrossClusterSearchConnection operation. Contains details of deleted outbound connection. |
| `DeletePackageRequest` | `structure` | PackageID | Container for request parameters to DeletePackage operation. |
| `DeletePackageResponse` | `structure` | PackageDetails | Container for response parameters to DeletePackage operation. |
| `DeleteVpcEndpointRequest` | `structure` | VpcEndpointId | Deletes an Amazon OpenSearch Service-managed interface VPC endpoint. |
| `DeleteVpcEndpointResponse` | `structure` | VpcEndpointSummary | Container for response parameters to the DeleteVpcEndpoint operation. Contains the summarized detail of the VPC Endpoint being deleted. |
| `AutoTuneDesiredState` | `enum` | ENABLED, DISABLED | Specifies the Auto-Tune desired state. Valid values are ENABLED, DISABLED. |
| `AutoTuneState` | `enum` | ENABLED, DISABLED, ENABLE_IN_PROGRESS, DISABLE_IN_PROGRESS, DISABLED_AND_ROLLBACK_SCHEDULED, DISABLED_AND_ROLLBACK_IN_PROGRESS, DISABLED_AND_ROLLBACK_COMPLETE, DISABLED_AND_ROLLBACK_ERROR, ERROR | Specifies the Auto-Tune state for the Elasticsearch domain. For valid states see the Developer Guide . |
| `AutoTuneType` | `enum` | SCHEDULED_ACTION | Specifies Auto-Tune type. Valid value is SCHEDULED_ACTION. |
| `ConfigChangeStatus` | `enum` | PENDING, INITIALIZING, VALIDATING, VALIDATION_FAILED, APPLYING_CHANGES, COMPLETED, PENDING_USER_INPUT, CANCELLED | - |
| `DeploymentStatus` | `enum` | PENDING_UPDATE, IN_PROGRESS, COMPLETED, NOT_ELIGIBLE, ELIGIBLE | - |
| `DeploymentStrategy` | `enum` | DEFAULT, CAPACITY_OPTIMIZED | Specifies the deployment strategy for the domain. Valid values are Default and CapacityOptimized . |
| `DescribePackagesFilterName` | `enum` | PackageID, PackageName, PackageStatus | - |
| `DomainPackageStatus` | `enum` | ASSOCIATING, ASSOCIATION_FAILED, ACTIVE, DISSOCIATING, DISSOCIATION_FAILED | - |
| `DomainProcessingStatusType` | `enum` | CREATING, ACTIVE, MODIFYING, UPGRADING, UPDATING, ISOLATED, DELETING | - |
| `ESPartitionInstanceType` | `enum` | m3_medium_elasticsearch, m3_large_elasticsearch, m3_xlarge_elasticsearch, m3_2xlarge_elasticsearch, m4_large_elasticsearch, m4_xlarge_elasticsearch, m4_2xlarge_elasticsearch, m4_4xlarge_elasticsearch, m4_10xlarge_elasticsearch, m5_large_elasticsearch, m5_xlarge_elasticsearch, m5_2xlarge_elasticsearch, ... (+46) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Outposts

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Outposts is a fully managed service that extends Amazon Web Services infrastructure, APIs, and tools to customer premises. By providing local access to Amazon Web Services managed infrastructure, Amazon Web Services Outposts enables customers to build and run applications on premises using the same programming interfaces as in Amazon Web Services Regions, while using local compute and storage resources for lower latency and local data processing needs.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Outposts resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Outposts workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Update`, `Create`, `Start` operation families, including `GetCapacityTask`, `GetCatalogItem`, `GetConnection`, `GetOrder`, `ListAssetInstances`, `ListAssets`.

## Service Identity and Protocol

- AWS model slug: `outposts`
- AWS SDK for Rust slug: `outposts`
- Model version: `2019-12-03`
- Model file: `vendor/api-models-aws/models/outposts/service/2019-12-03/outposts-2019-12-03.json`
- SDK ID: `Outposts`
- Endpoint prefix: `outposts`
- ARN namespace: `outposts`
- CloudFormation name: `Outposts`
- CloudTrail event source: `outposts.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (10), `List` (9), `Update` (4), `Create` (3), `Start` (3), `Cancel` (2), `Delete` (2), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelCapacityTask`, `CancelOrder`, `CreateOrder`, `CreateOutpost`, `CreateSite`, `DeleteOutpost`, `DeleteSite`, `StartCapacityTask`, `StartConnection`, `StartOutpostDecommission`, `TagResource`, `UntagResource`, `UpdateOutpost`, `UpdateSite`, `UpdateSiteAddress`, `UpdateSiteRackPhysicalProperties`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCapacityTask`, `GetCatalogItem`, `GetConnection`, `GetOrder`, `GetOutpost`, `GetOutpostBillingInformation`, `GetOutpostInstanceTypes`, `GetOutpostSupportedInstanceTypes`, `GetSite`, `GetSiteAddress`, `ListAssetInstances`, `ListAssets`, `ListBlockingInstancesForCapacityTask`, `ListCapacityTasks`, `ListCatalogItems`, `ListOrders`, `ListOutposts`, `ListSites`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelCapacityTask`, `CancelOrder`, `GetCapacityTask`, `ListBlockingInstancesForCapacityTask`, `ListCapacityTasks`, `StartCapacityTask`, `StartConnection`, `StartOutpostDecommission`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 35 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/outposts/latest/userguide/what-is-outposts.html
- https://docs.aws.amazon.com/outposts/latest/userguide/how-outposts-works.html
- https://docs.aws.amazon.com/outposts/latest/userguide/outposts-capacity.html

Research outcomes:
- AWS Outposts extends AWS infrastructure and services to customer premises with racks or servers.
- Core resources include sites, Outposts, capacity, local gateways, local network interfaces, and service links.
- Outposts extends VPC subnets to on-premises locations and uses service links to connect back to the parent AWS Region.
- Local gateways provide connectivity between Outposts subnets and local networks.
- Capacity is finite on each Outpost and must be managed for EC2, EBS, S3 on Outposts, growth, and server failure mitigation.
- Supported AWS services and resource types depend on Outpost form factor and Region.

Parity implications:
- Model sites, Outposts, orders, capacity tasks, local gateways, VPC/subnet associations, service links, and supported service capacity separately.
- Resource creation on an Outpost should consume finite local capacity and validate supported services.
- Local gateway and service-link state should affect connectivity semantics.

## Operation Groups

### Get

- Operations: `GetCapacityTask`, `GetCatalogItem`, `GetConnection`, `GetOrder`, `GetOutpost`, `GetOutpostBillingInformation`, `GetOutpostInstanceTypes`, `GetOutpostSupportedInstanceTypes`, `GetSite`, `GetSiteAddress`
- Traits: `paginated` (3)
- Common required input members in this group: `AddressType`, `CapacityTaskId`, `CatalogItemId`, `ConnectionId`, `OrderId`, `OutpostId`, `OutpostIdentifier`, `SiteId`

### List

- Operations: `ListAssetInstances`, `ListAssets`, `ListBlockingInstancesForCapacityTask`, `ListCapacityTasks`, `ListCatalogItems`, `ListOrders`, `ListOutposts`, `ListSites`, `ListTagsForResource`
- Traits: `paginated` (8)
- Common required input members in this group: `CapacityTaskId`, `OutpostIdentifier`, `ResourceArn`

### Update

- Operations: `UpdateOutpost`, `UpdateSite`, `UpdateSiteAddress`, `UpdateSiteRackPhysicalProperties`
- Common required input members in this group: `Address`, `AddressType`, `OutpostId`, `SiteId`

### Create

- Operations: `CreateOrder`, `CreateOutpost`, `CreateSite`
- Common required input members in this group: `Name`, `OutpostIdentifier`, `PaymentOption`, `SiteId`

### Start

- Operations: `StartCapacityTask`, `StartConnection`, `StartOutpostDecommission`
- Common required input members in this group: `AssetId`, `ClientPublicKey`, `InstancePools`, `NetworkInterfaceDeviceIndex`, `OutpostIdentifier`

### Cancel

- Operations: `CancelCapacityTask`, `CancelOrder`
- Common required input members in this group: `CapacityTaskId`, `OrderId`, `OutpostIdentifier`

### Delete

- Operations: `DeleteOutpost`, `DeleteSite`
- Common required input members in this group: `OutpostId`, `SiteId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelCapacityTask` | `POST /outposts/{OutpostIdentifier}/capacity/{CapacityTaskId}` | - | `CapacityTaskId`, `OutpostIdentifier` | - | `CancelCapacityTaskOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Cancels the capacity task. |
| `CancelOrder` | `POST /orders/{OrderId}/cancel` | - | `OrderId` | - | `CancelOrderOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Cancels the specified order for an Outpost. |
| `CreateOrder` | `POST /orders` | - | `OutpostIdentifier`, `PaymentOption` | - | `CreateOrderOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an order for an Outpost. |
| `CreateOutpost` | `POST /outposts` | - | `Name`, `SiteId` | - | `CreateOutpostOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an Outpost. You can specify either an Availability one or an AZ ID. |
| `CreateSite` | `POST /sites` | - | `Name` | - | `CreateSiteOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a site for an Outpost. |
| `DeleteOutpost` | `DELETE /outposts/{OutpostId}` | - | `OutpostId` | - | `DeleteOutpostOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Deletes the specified Outpost. |
| `DeleteSite` | `DELETE /sites/{SiteId}` | - | `SiteId` | - | `DeleteSiteOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Deletes the specified site. |
| `GetCapacityTask` | `GET /outposts/{OutpostIdentifier}/capacity/{CapacityTaskId}` | - | `CapacityTaskId`, `OutpostIdentifier` | - | `GetCapacityTaskOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets details of the specified capacity task. |
| `GetCatalogItem` | `GET /catalog/item/{CatalogItemId}` | - | `CatalogItemId` | - | `GetCatalogItemOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets information about the specified catalog item. |
| `GetConnection` | `GET /connections/{ConnectionId}` | - | `ConnectionId` | - | `GetConnectionResponse` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Amazon Web Services uses this action to install Outpost servers. Gets information about the specified connection. |
| `GetOrder` | `GET /orders/{OrderId}` | - | `OrderId` | - | `GetOrderOutput` | `InternalServerException`, `NotFoundException`, `ValidationException` | Gets information about the specified order. |
| `GetOutpost` | `GET /outposts/{OutpostId}` | - | `OutpostId` | - | `GetOutpostOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets information about the specified Outpost. |
| `GetOutpostBillingInformation` | `GET /outpost/{OutpostIdentifier}/billing-information` | `paginated` | `OutpostIdentifier` | - | `GetOutpostBillingInformationOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException` | Gets current and historical billing information about the specified Outpost. |
| `GetOutpostInstanceTypes` | `GET /outposts/{OutpostId}/instanceTypes` | `paginated` | `OutpostId` | - | `GetOutpostInstanceTypesOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets the instance types for the specified Outpost. |
| `GetOutpostSupportedInstanceTypes` | `GET /outposts/{OutpostIdentifier}/supportedInstanceTypes` | `paginated` | `OutpostIdentifier` | - | `GetOutpostSupportedInstanceTypesOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets the instance types that an Outpost can support in `InstanceTypeCapacity`. This will generally include instance types that are not currently configured and therefore cannot be launched with the current Outpost capacity configuration. |
| `GetSite` | `GET /sites/{SiteId}` | - | `SiteId` | - | `GetSiteOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets information about the specified Outpost site. |
| `GetSiteAddress` | `GET /sites/{SiteId}/address` | - | `AddressType`, `SiteId` | - | `GetSiteAddressOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Gets the site address of the specified site. |
| `ListAssetInstances` | `GET /outposts/{OutpostIdentifier}/assetInstances` | `paginated` | `OutpostIdentifier` | - | `ListAssetInstancesOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | A list of Amazon EC2 instances, belonging to all accounts, running on the specified Outpost. Does not include Amazon EBS or Amazon S3 instances. |
| `ListAssets` | `GET /outposts/{OutpostIdentifier}/assets` | `paginated` | `OutpostIdentifier` | - | `ListAssetsOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Lists the hardware assets for the specified Outpost. Use filters to return specific results. |
| `ListBlockingInstancesForCapacityTask` | `GET /outposts/{OutpostIdentifier}/capacity/{CapacityTaskId}/blockingInstances` | `paginated` | `CapacityTaskId`, `OutpostIdentifier` | - | `ListBlockingInstancesForCapacityTaskOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | A list of Amazon EC2 instances running on the Outpost and belonging to the account that initiated the capacity task. Use this list to specify the instances you cannot stop to free up capacity to run the capacity task. |
| `ListCapacityTasks` | `GET /capacity/tasks` | `paginated` | - | - | `ListCapacityTasksOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Lists the capacity tasks for your Amazon Web Services account. Use filters to return specific results. |
| `ListCatalogItems` | `GET /catalog/items` | `paginated` | - | - | `ListCatalogItemsOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Lists the items in the catalog. Use filters to return specific results. |
| `ListOrders` | `GET /list-orders` | `paginated` | - | - | `ListOrdersOutput` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Lists the Outpost orders for your Amazon Web Services account. |
| `ListOutposts` | `GET /outposts` | `paginated` | - | - | `ListOutpostsOutput` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Lists the Outposts for your Amazon Web Services account. Use filters to return specific results. |
| `ListSites` | `GET /sites` | `paginated` | - | - | `ListSitesOutput` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Lists the Outpost sites for your Amazon Web Services account. Use filters to return specific results. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `NotFoundException`, `ValidationException` | Lists the tags for the specified resource. |
| `StartCapacityTask` | `POST /outposts/{OutpostIdentifier}/capacity` | - | `InstancePools`, `OutpostIdentifier` | - | `StartCapacityTaskOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Starts the specified capacity task. You can have one active capacity task for each order and each Outpost. |
| `StartConnection` | `POST /connections` | - | `AssetId`, `ClientPublicKey`, `NetworkInterfaceDeviceIndex` | - | `StartConnectionResponse` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Amazon Web Services uses this action to install Outpost servers. Starts the connection required for Outpost server installation. |
| `StartOutpostDecommission` | `POST /outposts/{OutpostIdentifier}/decommission` | - | `OutpostIdentifier` | - | `StartOutpostDecommissionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Starts the decommission process to return the Outposts racks or servers. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `NotFoundException`, `ValidationException` | Adds tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `NotFoundException`, `ValidationException` | Removes tags from the specified resource. |
| `UpdateOutpost` | `PATCH /outposts/{OutpostId}` | - | `OutpostId` | - | `UpdateOutpostOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Updates an Outpost. |
| `UpdateSite` | `PATCH /sites/{SiteId}` | - | `SiteId` | - | `UpdateSiteOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Updates the specified site. |
| `UpdateSiteAddress` | `PUT /sites/{SiteId}/address` | - | `Address`, `AddressType`, `SiteId` | - | `UpdateSiteAddressOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Updates the address of the specified site. You can't update a site address if there is an order in progress. |
| `UpdateSiteRackPhysicalProperties` | `PATCH /sites/{SiteId}/rackPhysicalProperties` | - | `SiteId` | - | `UpdateSiteRackPhysicalPropertiesOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ValidationException` | Update the physical and logistical details for a rack at a site. For more information about hardware requirements for racks, see Network readiness checklist in the Amazon Web Services Outposts User Guide. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetOutpostBillingInformation` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `GetOutpostInstanceTypes` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `GetOutpostSupportedInstanceTypes` | - | `OrderId -> OrderId`, `AssetId -> AssetId`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `GetSiteAddress` | - | `AddressType -> AddressType` | - | - |
| `ListAssetInstances` | - | `AssetIdFilter -> AssetIdFilter`, `InstanceTypeFilter -> InstanceTypeFilter`, `AccountIdFilter -> AccountIdFilter`, `AwsServiceFilter -> AwsServiceFilter`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListAssets` | - | `HostIdFilter -> HostIdFilter`, `MaxResults -> MaxResults`, `NextToken -> NextToken`, `StatusFilter -> StatusFilter`, `AssetTypeFilter -> AssetTypeFilter` | - | - |
| `ListBlockingInstancesForCapacityTask` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListCapacityTasks` | - | `OutpostIdentifierFilter -> OutpostIdentifierFilter`, `MaxResults -> MaxResults`, `NextToken -> NextToken`, `CapacityTaskStatusFilter -> CapacityTaskStatusFilter` | - | - |
| `ListCatalogItems` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults`, `ItemClassFilter -> ItemClassFilter`, `SupportedStorageFilter -> SupportedStorageFilter`, `EC2FamilyFilter -> EC2FamilyFilter` | - | - |
| `ListOrders` | - | `OutpostIdentifierFilter -> OutpostIdentifierFilter`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListOutposts` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults`, `LifeCycleStatusFilter -> LifeCycleStatusFilter`, `AvailabilityZoneFilter -> AvailabilityZoneFilter`, `AvailabilityZoneIdFilter -> AvailabilityZoneIdFilter` | - | - |
| `ListSites` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults`, `OperatingAddressCountryCodeFilter -> OperatingAddressCountryCodeFilter`, `OperatingAddressStateOrRegionFilter -> OperatingAddressStateOrRegionFilter`, `OperatingAddressCityFilter -> OperatingAddressCityFilter` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An internal error has occurred. |
| `ValidationException` | `structure` | `Message` | A parameter is not valid. |
| `NotFoundException` | `structure` | `Message` | The specified request is not valid. |
| `AccessDeniedException` | `structure` | `Message` | You do not have permission to perform this operation. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Updating or deleting this resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You have exceeded a service quota. |
| `CancelCapacityTaskInput` | `structure` | `CapacityTaskId`, `OutpostIdentifier` | - |
| `CancelCapacityTaskOutput` | `structure` | - | - |
| `CancelOrderInput` | `structure` | `OrderId` | - |
| `CancelOrderOutput` | `structure` | - | - |
| `CreateOrderInput` | `structure` | `LineItems`, `OutpostIdentifier`, `PaymentOption`, `PaymentTerm` | - |
| `CreateOrderOutput` | `structure` | `Order` | - |
| `CreateOutpostInput` | `structure` | `AvailabilityZone`, `AvailabilityZoneId`, `Description`, `Name`, `SiteId`, `SupportedHardwareType`, `Tags` | - |
| `CreateOutpostOutput` | `structure` | `Outpost` | - |
| `CreateSiteInput` | `structure` | `Description`, `Name`, `Notes`, `OperatingAddress`, `RackPhysicalProperties`, `ShippingAddress`, `Tags` | - |
| `CreateSiteOutput` | `structure` | `Site` | - |
| `DeleteOutpostInput` | `structure` | `OutpostId` | - |
| `DeleteOutpostOutput` | `structure` | - | - |
| `DeleteSiteInput` | `structure` | `SiteId` | - |
| `DeleteSiteOutput` | `structure` | - | - |
| `GetCapacityTaskInput` | `structure` | `CapacityTaskId`, `OutpostIdentifier` | - |
| `GetCapacityTaskOutput` | `structure` | `AssetId`, `CapacityTaskId`, `CapacityTaskStatus`, `CompletionDate`, `CreationDate`, `DryRun`, `Failed`, `InstancesToExclude`, `LastModifiedDate`, `OrderId`, `OutpostId`, `RequestedInstancePools`, ... (+1) | - |
| `GetCatalogItemInput` | `structure` | `CatalogItemId` | - |
| `GetCatalogItemOutput` | `structure` | `CatalogItem` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Cloud Map

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Cloud Map With Cloud Map, you can configure public DNS, private DNS, or HTTP namespaces that your microservice applications run in. When an instance becomes available, you can call the Cloud Map API to register the instance with Cloud Map. For public or private DNS namespaces, Cloud Map automatically creates DNS records and an optional health check. Clients that submit public or private DNS queries, or HTTP requests, for the service receive an answer that contains up to eight healthy records.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Cloud Map where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented AWS Cloud Map workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Update`, `List`, `Create`, `Delete` operation families, including `GetInstance`, `GetInstancesHealthStatus`, `GetNamespace`, `GetOperation`, `UpdateHttpNamespace`, `UpdateInstanceCustomHealthStatus`.

## Service Identity and Protocol

- AWS model slug: `servicediscovery`
- AWS SDK for Rust slug: `servicediscovery`
- Model version: `2017-03-14`
- Model file: `vendor/api-models-aws/models/servicediscovery/service/2017-03-14/servicediscovery-2017-03-14.json`
- SDK ID: `ServiceDiscovery`
- Endpoint prefix: `servicediscovery`
- ARN namespace: `servicediscovery`
- CloudFormation name: `ServiceDiscovery`
- CloudTrail event source: `servicediscovery.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `Update` (6), `List` (5), `Create` (4), `Delete` (3), `Discover` (2), `Deregister` (1), `Register` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateHttpNamespace`, `CreatePrivateDnsNamespace`, `CreatePublicDnsNamespace`, `CreateService`, `DeleteNamespace`, `DeleteService`, `DeleteServiceAttributes`, `DeregisterInstance`, `RegisterInstance`, `TagResource`, `UntagResource`, `UpdateHttpNamespace`, `UpdateInstanceCustomHealthStatus`, `UpdatePrivateDnsNamespace`, `UpdatePublicDnsNamespace`, `UpdateService`, `UpdateServiceAttributes`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetInstance`, `GetInstancesHealthStatus`, `GetNamespace`, `GetOperation`, `GetService`, `GetServiceAttributes`, `ListInstances`, `ListNamespaces`, `ListOperations`, `ListServices`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 30 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

Cloud Map service discovery currently stores private namespace VPC IDs inside Cloud Map state.

- Private DNS namespaces record the supplied VPC ID as raw namespace metadata.
- The state enforces service-discovery-local uniqueness for private DNS namespaces associated with the same VPC.
- Public DNS and HTTP namespaces have no VPC value, and instance registration is independent of EC2 networking.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetInstance`, `GetInstancesHealthStatus`, `GetNamespace`, `GetOperation`, `GetService`, `GetServiceAttributes`
- Traits: `paginated` (1)
- Common required input members in this group: `ServiceId`, `Id`

### Update

- Operations: `UpdateHttpNamespace`, `UpdateInstanceCustomHealthStatus`, `UpdatePrivateDnsNamespace`, `UpdatePublicDnsNamespace`, `UpdateService`, `UpdateServiceAttributes`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `Id`, `Namespace`, `ServiceId`

### List

- Operations: `ListInstances`, `ListNamespaces`, `ListOperations`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: -

### Create

- Operations: `CreateHttpNamespace`, `CreatePrivateDnsNamespace`, `CreatePublicDnsNamespace`, `CreateService`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `Name`

### Delete

- Operations: `DeleteNamespace`, `DeleteService`, `DeleteServiceAttributes`
- Common required input members in this group: `Id`

### Discover

- Operations: `DiscoverInstances`, `DiscoverInstancesRevision`
- Common required input members in this group: `NamespaceName`, `ServiceName`

### Deregister

- Operations: `DeregisterInstance`
- Common required input members in this group: -

### Register

- Operations: `RegisterInstance`
- Traits: `idempotency-token` (1)
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
| `CreateHttpNamespace` | `-` | `idempotency-token` | `Name` | `CreatorRequestId` | `CreateHttpNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceAlreadyExists`, `ResourceLimitExceeded`, `TooManyTagsException` | Creates an HTTP namespace. Service instances registered using an HTTP namespace can be discovered using a DiscoverInstances request but can't be discovered using DNS. For the current quota on the number of namespaces ... |
| `CreatePrivateDnsNamespace` | `-` | `idempotency-token` | `Name`, `Vpc` | `CreatorRequestId` | `CreatePrivateDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceAlreadyExists`, `ResourceLimitExceeded`, `TooManyTagsException` | Creates a private namespace based on DNS, which is visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. For example, if you name your namespace example.com and name your servi ... |
| `CreatePublicDnsNamespace` | `-` | `idempotency-token` | `Name` | `CreatorRequestId` | `CreatePublicDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceAlreadyExists`, `ResourceLimitExceeded`, `TooManyTagsException` | Creates a public namespace based on DNS, which is visible on the internet. The namespace defines your service naming scheme. For example, if you name your namespace example.com and name your service backend , the res ... |
| `CreateService` | `-` | `idempotency-token` | `Name` | `CreatorRequestId` | `CreateServiceResponse` | `InvalidInput`, `NamespaceNotFound`, `ResourceLimitExceeded`, `ServiceAlreadyExists`, `TooManyTagsException` | Creates a service. This action defines the configuration for the following entities: For public and private DNS namespaces, one of the following combinations of DNS records in Amazon Route 53: A AAAA A and AAAA SRV C ... |
| `DeleteNamespace` | `-` | - | `Id` | - | `DeleteNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Deletes a namespace from the current account. If the namespace still contains one or more services, the request fails. |
| `DeleteService` | `-` | - | `Id` | - | `DeleteServiceResponse` | `InvalidInput`, `ResourceInUse`, `ServiceNotFound` | Deletes a specified service and all associated service attributes. If the service still contains one or more registered instances, the request fails. |
| `DeleteServiceAttributes` | `-` | - | `ServiceId`, `Attributes` | - | `DeleteServiceAttributesResponse` | `InvalidInput`, `ServiceNotFound` | Deletes specific attributes associated with a service. |
| `DeregisterInstance` | `-` | - | `ServiceId`, `InstanceId` | - | `DeregisterInstanceResponse` | `DuplicateRequest`, `InstanceNotFound`, `InvalidInput`, `ResourceInUse`, `ServiceNotFound` | Deletes the Amazon Route 53 DNS records and health check, if any, that Cloud Map created for the specified instance. |
| `DiscoverInstances` | `-` | - | `NamespaceName`, `ServiceName` | - | `DiscoverInstancesResponse` | `InvalidInput`, `NamespaceNotFound`, `RequestLimitExceeded`, `ServiceNotFound` | Discovers registered instances for a specified namespace and service. You can use DiscoverInstances to discover instances for any type of namespace. DiscoverInstances returns a randomized list of instances allowing c ... |
| `DiscoverInstancesRevision` | `-` | - | `NamespaceName`, `ServiceName` | - | `DiscoverInstancesRevisionResponse` | `InvalidInput`, `NamespaceNotFound`, `RequestLimitExceeded`, `ServiceNotFound` | Discovers the increasing revision associated with an instance. |
| `GetInstance` | `-` | - | `ServiceId`, `InstanceId` | - | `GetInstanceResponse` | `InstanceNotFound`, `InvalidInput`, `ServiceNotFound` | Gets information about a specified instance. |
| `GetInstancesHealthStatus` | `-` | `paginated` | `ServiceId` | - | `GetInstancesHealthStatusResponse` | `InstanceNotFound`, `InvalidInput`, `ServiceNotFound` | Gets the current health status ( Healthy , Unhealthy , or Unknown ) of one or more instances that are associated with a specified service. There's a brief delay between when you register an instance and when the heal ... |
| `GetNamespace` | `-` | - | `Id` | - | `GetNamespaceResponse` | `InvalidInput`, `NamespaceNotFound` | Gets information about a namespace. |
| `GetOperation` | `-` | - | `OperationId` | - | `GetOperationResponse` | `InvalidInput`, `OperationNotFound` | Gets information about any operation that returns an operation ID in the response, such as a CreateHttpNamespace request. To get a list of operations that match specified criteria, see ListOperations . |
| `GetService` | `-` | - | `Id` | - | `GetServiceResponse` | `InvalidInput`, `ServiceNotFound` | Gets the settings for a specified service. |
| `GetServiceAttributes` | `-` | - | `ServiceId` | - | `GetServiceAttributesResponse` | `InvalidInput`, `ServiceNotFound` | Returns the attributes associated with a specified service. |
| `ListInstances` | `-` | `paginated` | `ServiceId` | - | `ListInstancesResponse` | `InvalidInput`, `ServiceNotFound` | Lists summary information about the instances that you registered by using a specified service. |
| `ListNamespaces` | `-` | `paginated` | - | - | `ListNamespacesResponse` | `InvalidInput` | Lists summary information about the namespaces that were created by the current Amazon Web Services account and shared with the current Amazon Web Services account. |
| `ListOperations` | `-` | `paginated` | - | - | `ListOperationsResponse` | `InvalidInput` | Lists operations that match the criteria that you specify. |
| `ListServices` | `-` | `paginated` | - | - | `ListServicesResponse` | `InvalidInput` | Lists summary information for all the services that are associated with one or more namespaces. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidInput`, `ResourceNotFoundException` | Lists tags for the specified resource. |
| `RegisterInstance` | `-` | `idempotency-token` | `ServiceId`, `InstanceId`, `Attributes` | `CreatorRequestId` | `RegisterInstanceResponse` | `DuplicateRequest`, `InvalidInput`, `ResourceInUse`, `ResourceLimitExceeded`, `ServiceNotFound` | Creates or updates one or more records and, optionally, creates a health check based on the settings in a specified service. When you submit a RegisterInstance request, the following occurs: For each DNS record that ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidInput`, `ResourceNotFoundException`, `TooManyTagsException` | Adds one or more tags to the specified resource. |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidInput`, `ResourceNotFoundException` | Removes one or more tags from the specified resource. |
| `UpdateHttpNamespace` | `-` | `idempotency-token` | `Id`, `Namespace` | `UpdaterRequestId` | `UpdateHttpNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Updates an HTTP namespace. |
| `UpdateInstanceCustomHealthStatus` | `-` | - | `ServiceId`, `InstanceId`, `Status` | - | `Unit` | `CustomHealthNotFound`, `InstanceNotFound`, `InvalidInput`, `ServiceNotFound` | Submits a request to change the health status of a custom health check to healthy or unhealthy. You can use UpdateInstanceCustomHealthStatus to change the status only for custom health checks, which you define using ... |
| `UpdatePrivateDnsNamespace` | `-` | `idempotency-token` | `Id`, `Namespace` | `UpdaterRequestId` | `UpdatePrivateDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Updates a private DNS namespace. |
| `UpdatePublicDnsNamespace` | `-` | `idempotency-token` | `Id`, `Namespace` | `UpdaterRequestId` | `UpdatePublicDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Updates a public DNS namespace. |
| `UpdateService` | `-` | - | `Id`, `Service` | - | `UpdateServiceResponse` | `DuplicateRequest`, `InvalidInput`, `ServiceNotFound` | Submits a request to perform the following operations: Update the TTL setting for existing DnsRecords configurations Add, update, or delete HealthCheckConfig for a specified service You can't add, update, or delete a ... |
| `UpdateServiceAttributes` | `-` | - | `ServiceId`, `Attributes` | - | `UpdateServiceAttributesResponse` | `InvalidInput`, `ServiceAttributesLimitExceededException`, `ServiceNotFound` | Submits a request to update a specified service to add service-level attributes. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CustomHealthNotFound` | `structure` | Message | The health check for the instance that's specified by ServiceId and InstanceId isn't a custom health check. |
| `DuplicateRequest` | `structure` | Message, DuplicateOperationId | The operation is already in progress. |
| `InstanceNotFound` | `structure` | Message | No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet. |
| `InvalidInput` | `structure` | Message | One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string v ... |
| `NamespaceAlreadyExists` | `structure` | Message, CreatorRequestId, NamespaceId | The namespace that you're trying to create already exists. |
| `NamespaceNotFound` | `structure` | Message | No namespace exists with the specified ID. |
| `OperationNotFound` | `structure` | Message | No operation exists with the specified ID. |
| `RequestLimitExceeded` | `structure` | Message | The operation can't be completed because you've reached the quota for the number of requests. For more information, see Cloud Map API request throttling quo ... |
| `ResourceInUse` | `structure` | Message | The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances. |
| `ResourceLimitExceeded` | `structure` | Message | The resource can't be created because you've reached the quota on the number of resources. |
| `ResourceNotFoundException` | `structure` | Message | The operation can't be completed because the resource was not found. |
| `ServiceAlreadyExists` | `structure` | Message, CreatorRequestId, ServiceId, ServiceArn | The service can't be created because a service with the same name already exists. |
| `ServiceAttributesLimitExceededException` | `structure` | Message | The attribute can't be added to the service because you've exceeded the quota for the number of attributes you can add to a service. |
| `ServiceNotFound` | `structure` | Message | No service exists with the specified ID. |
| `TooManyTagsException` | `structure` | Message, ResourceName | The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50. |
| `CreateHttpNamespaceRequest` | `structure` | Name, CreatorRequestId, Description, Tags | - |
| `CreateHttpNamespaceResponse` | `structure` | OperationId | - |
| `CreatePrivateDnsNamespaceRequest` | `structure` | Name, CreatorRequestId, Description, Vpc, Tags, Properties | - |
| `CreatePrivateDnsNamespaceResponse` | `structure` | OperationId | - |
| `CreatePublicDnsNamespaceRequest` | `structure` | Name, CreatorRequestId, Description, Tags, Properties | - |
| `CreatePublicDnsNamespaceResponse` | `structure` | OperationId | - |
| `CreateServiceRequest` | `structure` | Name, NamespaceId, CreatorRequestId, Description, DnsConfig, HealthCheckConfig, HealthCheckCustomConfig, Tags, Type | - |
| `CreateServiceResponse` | `structure` | Service | - |
| `DeleteNamespaceRequest` | `structure` | Id | - |
| `DeleteNamespaceResponse` | `structure` | OperationId | - |
| `DeleteServiceRequest` | `structure` | Id | - |
| `DeleteServiceResponse` | `structure` | **empty (no members)** | - |
| `DeleteServiceAttributesRequest` | `structure` | ServiceId, Attributes | - |
| `DeleteServiceAttributesResponse` | `structure` | **empty (no members)** | - |
| `DeregisterInstanceRequest` | `structure` | ServiceId, InstanceId | - |
| `DeregisterInstanceResponse` | `structure` | OperationId | - |
| `DiscoverInstancesRequest` | `structure` | NamespaceName, ServiceName, MaxResults, QueryParameters, OptionalParameters, HealthStatus, OwnerAccount | - |
| `DiscoverInstancesResponse` | `structure` | Instances, InstancesRevision | - |
| `DiscoverInstancesRevisionRequest` | `structure` | NamespaceName, ServiceName, OwnerAccount | - |
| `DiscoverInstancesRevisionResponse` | `structure` | InstancesRevision | - |
| `GetInstanceRequest` | `structure` | ServiceId, InstanceId | - |
| `GetInstanceResponse` | `structure` | ResourceOwner, Instance | - |
| `GetInstancesHealthStatusRequest` | `structure` | ServiceId, Instances, MaxResults, NextToken | - |
| `GetInstancesHealthStatusResponse` | `structure` | Status, NextToken | - |
| `GetNamespaceRequest` | `structure` | Id | - |
| `CustomHealthStatus` | `enum` | HEALTHY, UNHEALTHY | - |
| `FilterCondition` | `enum` | EQ, IN, BETWEEN, BEGINS_WITH | - |
| `HealthCheckType` | `enum` | HTTP, HTTPS, TCP | - |
| `HealthStatus` | `enum` | HEALTHY, UNHEALTHY, UNKNOWN | - |
| `HealthStatusFilter` | `enum` | HEALTHY, UNHEALTHY, ALL, HEALTHY_OR_ELSE_ALL | - |
| `NamespaceFilterName` | `enum` | TYPE, NAME, HTTP_NAME, RESOURCE_OWNER | - |
| `NamespaceType` | `enum` | DNS_PUBLIC, DNS_PRIVATE, HTTP | - |
| `OperationFilterName` | `enum` | NAMESPACE_ID, SERVICE_ID, STATUS, TYPE, UPDATE_DATE | - |
| `OperationStatus` | `enum` | SUBMITTED, PENDING, SUCCESS, FAIL | - |
| `OperationTargetType` | `enum` | NAMESPACE, SERVICE, INSTANCE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

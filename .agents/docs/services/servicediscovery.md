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
- Common required input members in this group: `Id`, `InstanceId`, `OperationId`, `ServiceId`

### Update

- Operations: `UpdateHttpNamespace`, `UpdateInstanceCustomHealthStatus`, `UpdatePrivateDnsNamespace`, `UpdatePublicDnsNamespace`, `UpdateService`, `UpdateServiceAttributes`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `Attributes`, `Id`, `InstanceId`, `Namespace`, `Service`, `ServiceId`, `Status`

### List

- Operations: `ListInstances`, `ListNamespaces`, `ListOperations`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `ResourceARN`, `ServiceId`

### Create

- Operations: `CreateHttpNamespace`, `CreatePrivateDnsNamespace`, `CreatePublicDnsNamespace`, `CreateService`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `Name`, `Vpc`

### Delete

- Operations: `DeleteNamespace`, `DeleteService`, `DeleteServiceAttributes`
- Common required input members in this group: `Attributes`, `Id`, `ServiceId`

### Discover

- Operations: `DiscoverInstances`, `DiscoverInstancesRevision`
- Traits: `endpoint-bound` (2)
- Common required input members in this group: `NamespaceName`, `ServiceName`

### Deregister

- Operations: `DeregisterInstance`
- Common required input members in this group: `InstanceId`, `ServiceId`

### Register

- Operations: `RegisterInstance`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Attributes`, `InstanceId`, `ServiceId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateHttpNamespace` | - | `idempotency-token` | `Name` | `CreatorRequestId` | `CreateHttpNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceAlreadyExists`, `ResourceLimitExceeded`, `TooManyTagsException` | Creates an HTTP namespace. Service instances registered using an HTTP namespace can be discovered using a `DiscoverInstances` request but can't be discovered using DNS. |
| `CreatePrivateDnsNamespace` | - | `idempotency-token` | `Name`, `Vpc` | `CreatorRequestId` | `CreatePrivateDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceAlreadyExists`, `ResourceLimitExceeded`, `TooManyTagsException` | Creates a private namespace based on DNS, which is visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. |
| `CreatePublicDnsNamespace` | - | `idempotency-token` | `Name` | `CreatorRequestId` | `CreatePublicDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceAlreadyExists`, `ResourceLimitExceeded`, `TooManyTagsException` | Creates a public namespace based on DNS, which is visible on the internet. The namespace defines your service naming scheme. |
| `CreateService` | - | `idempotency-token` | `Name` | `CreatorRequestId` | `CreateServiceResponse` | `InvalidInput`, `NamespaceNotFound`, `ResourceLimitExceeded`, `ServiceAlreadyExists`, `TooManyTagsException` | Creates a service. This action defines the configuration for the following entities: For public and private DNS namespaces, one of the following combinations of DNS records in Amazon Route 53: `A` `AAAA` `A` and `AAAA` `SRV` `CNAME` Optionally, a health check... |
| `DeleteNamespace` | - | - | `Id` | - | `DeleteNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Deletes a namespace from the current account. If the namespace still contains one or more services, the request fails. |
| `DeleteService` | - | - | `Id` | - | `DeleteServiceResponse` | `InvalidInput`, `ResourceInUse`, `ServiceNotFound` | Deletes a specified service and all associated service attributes. If the service still contains one or more registered instances, the request fails. |
| `DeleteServiceAttributes` | - | - | `Attributes`, `ServiceId` | - | `DeleteServiceAttributesResponse` | `InvalidInput`, `ServiceNotFound` | Deletes specific attributes associated with a service. |
| `DeregisterInstance` | - | - | `InstanceId`, `ServiceId` | - | `DeregisterInstanceResponse` | `DuplicateRequest`, `InstanceNotFound`, `InvalidInput`, `ResourceInUse`, `ServiceNotFound` | Deletes the Amazon Route 53 DNS records and health check, if any, that Cloud Map created for the specified instance. |
| `DiscoverInstances` | - | `endpoint-bound` | `NamespaceName`, `ServiceName` | - | `DiscoverInstancesResponse` | `InvalidInput`, `NamespaceNotFound`, `RequestLimitExceeded`, `ServiceNotFound` | Discovers registered instances for a specified namespace and service. You can use `DiscoverInstances` to discover instances for any type of namespace. |
| `DiscoverInstancesRevision` | - | `endpoint-bound` | `NamespaceName`, `ServiceName` | - | `DiscoverInstancesRevisionResponse` | `InvalidInput`, `NamespaceNotFound`, `RequestLimitExceeded`, `ServiceNotFound` | Discovers the increasing revision associated with an instance. |
| `GetInstance` | - | - | `InstanceId`, `ServiceId` | - | `GetInstanceResponse` | `InstanceNotFound`, `InvalidInput`, `ServiceNotFound` | Gets information about a specified instance. |
| `GetInstancesHealthStatus` | - | `paginated` | `ServiceId` | - | `GetInstancesHealthStatusResponse` | `InstanceNotFound`, `InvalidInput`, `ServiceNotFound` | Gets the current health status (`Healthy`, `Unhealthy`, or `Unknown`) of one or more instances that are associated with a specified service. There's a brief delay between when you register an instance and when the health status for the instance is available. |
| `GetNamespace` | - | - | `Id` | - | `GetNamespaceResponse` | `InvalidInput`, `NamespaceNotFound` | Gets information about a namespace. |
| `GetOperation` | - | - | `OperationId` | - | `GetOperationResponse` | `InvalidInput`, `OperationNotFound` | Gets information about any operation that returns an operation ID in the response, such as a `CreateHttpNamespace` request. To get a list of operations that match specified criteria, see ListOperations. |
| `GetService` | - | - | `Id` | - | `GetServiceResponse` | `InvalidInput`, `ServiceNotFound` | Gets the settings for a specified service. |
| `GetServiceAttributes` | - | - | `ServiceId` | - | `GetServiceAttributesResponse` | `InvalidInput`, `ServiceNotFound` | Returns the attributes associated with a specified service. |
| `ListInstances` | - | `paginated` | `ServiceId` | - | `ListInstancesResponse` | `InvalidInput`, `ServiceNotFound` | Lists summary information about the instances that you registered by using a specified service. |
| `ListNamespaces` | - | `paginated` | - | - | `ListNamespacesResponse` | `InvalidInput` | Lists summary information about the namespaces that were created by the current Amazon Web Services account and shared with the current Amazon Web Services account. |
| `ListOperations` | - | `paginated` | - | - | `ListOperationsResponse` | `InvalidInput` | Lists operations that match the criteria that you specify. |
| `ListServices` | - | `paginated` | - | - | `ListServicesResponse` | `InvalidInput` | Lists summary information for all the services that are associated with one or more namespaces. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidInput`, `ResourceNotFoundException` | Lists tags for the specified resource. |
| `RegisterInstance` | - | `idempotency-token` | `Attributes`, `InstanceId`, `ServiceId` | `CreatorRequestId` | `RegisterInstanceResponse` | `DuplicateRequest`, `InvalidInput`, `ResourceInUse`, `ResourceLimitExceeded`, `ServiceNotFound` | Creates or updates one or more records and, optionally, creates a health check based on the settings in a specified service. When you submit a `RegisterInstance` request, the following occurs: For each DNS record that you define in the service that's... |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidInput`, `ResourceNotFoundException`, `TooManyTagsException` | Adds one or more tags to the specified resource. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidInput`, `ResourceNotFoundException` | Removes one or more tags from the specified resource. |
| `UpdateHttpNamespace` | - | `idempotency-token` | `Id`, `Namespace` | `UpdaterRequestId` | `UpdateHttpNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Updates an HTTP namespace. |
| `UpdateInstanceCustomHealthStatus` | - | - | `InstanceId`, `ServiceId`, `Status` | - | `Unit` | `CustomHealthNotFound`, `InstanceNotFound`, `InvalidInput`, `ServiceNotFound` | Submits a request to change the health status of a custom health check to healthy or unhealthy. You can use `UpdateInstanceCustomHealthStatus` to change the status only for custom health checks, which you define using `HealthCheckCustomConfig` when you create... |
| `UpdatePrivateDnsNamespace` | - | `idempotency-token` | `Id`, `Namespace` | `UpdaterRequestId` | `UpdatePrivateDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Updates a private DNS namespace. |
| `UpdatePublicDnsNamespace` | - | `idempotency-token` | `Id`, `Namespace` | `UpdaterRequestId` | `UpdatePublicDnsNamespaceResponse` | `DuplicateRequest`, `InvalidInput`, `NamespaceNotFound`, `ResourceInUse` | Updates a public DNS namespace. |
| `UpdateService` | - | - | `Id`, `Service` | - | `UpdateServiceResponse` | `DuplicateRequest`, `InvalidInput`, `ServiceNotFound` | Submits a request to perform the following operations: Update the TTL setting for existing `DnsRecords` configurations Add, update, or delete `HealthCheckConfig` for a specified service You can't add, update, or delete a `HealthCheckCustomConfig`... |
| `UpdateServiceAttributes` | - | - | `Attributes`, `ServiceId` | - | `UpdateServiceAttributesResponse` | `InvalidInput`, `ServiceAttributesLimitExceededException`, `ServiceNotFound` | Submits a request to update a specified service to add service-level attributes. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInput` | `structure` | `Message` | One or more specified values aren't valid. |
| `ServiceNotFound` | `structure` | `Message` | No service exists with the specified ID. |
| `DuplicateRequest` | `structure` | `DuplicateOperationId`, `Message` | The operation is already in progress. |
| `NamespaceNotFound` | `structure` | `Message` | No namespace exists with the specified ID. |
| `ResourceInUse` | `structure` | `Message` | The specified resource can't be deleted because it contains other resources. |
| `ResourceLimitExceeded` | `structure` | `Message` | The resource can't be created because you've reached the quota on the number of resources. |
| `TooManyTagsException` | `structure` | `Message`, `ResourceName` | The list of tags on the resource is over the quota. |
| `InstanceNotFound` | `structure` | `Message` | No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet. |
| `NamespaceAlreadyExists` | `structure` | `CreatorRequestId`, `Message`, `NamespaceId` | The namespace that you're trying to create already exists. |
| `ResourceNotFoundException` | `structure` | `Message` | The operation can't be completed because the resource was not found. |
| `RequestLimitExceeded` | `structure` | `Message` | The operation can't be completed because you've reached the quota for the number of requests. |
| `CreateHttpNamespaceRequest` | `structure` | `CreatorRequestId`, `Description`, `Name`, `Tags` | - |
| `CreateHttpNamespaceResponse` | `structure` | `OperationId` | - |
| `CreatePrivateDnsNamespaceRequest` | `structure` | `CreatorRequestId`, `Description`, `Name`, `Properties`, `Tags`, `Vpc` | - |
| `CreatePrivateDnsNamespaceResponse` | `structure` | `OperationId` | - |
| `CreatePublicDnsNamespaceRequest` | `structure` | `CreatorRequestId`, `Description`, `Name`, `Properties`, `Tags` | - |
| `CreatePublicDnsNamespaceResponse` | `structure` | `OperationId` | - |
| `CreateServiceRequest` | `structure` | `CreatorRequestId`, `Description`, `DnsConfig`, `HealthCheckConfig`, `HealthCheckCustomConfig`, `Name`, `NamespaceId`, `Tags`, `Type` | - |
| `CreateServiceResponse` | `structure` | `Service` | - |
| `ServiceAlreadyExists` | `structure` | `CreatorRequestId`, `Message`, `ServiceArn`, `ServiceId` | The service can't be created because a service with the same name already exists. |
| `DeleteNamespaceRequest` | `structure` | `Id` | - |
| `DeleteNamespaceResponse` | `structure` | `OperationId` | - |
| `DeleteServiceRequest` | `structure` | `Id` | - |
| `DeleteServiceResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

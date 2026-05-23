# AWS App Mesh

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

App Mesh is a service mesh based on the Envoy proxy that makes it easy to monitor and control microservices. App Mesh standardizes how your microservices communicate, giving you end-to-end visibility and helping to ensure high availability for your applications. App Mesh gives you consistent visibility and network traffic controls for every microservice in an application. You can use App Mesh with Amazon Web Services Fargate, Amazon ECS, Amazon EKS, Kubernetes on Amazon Web Services, and Amazon EC2. App Mesh supports microservice applications that use service discovery naming for their components.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS App Mesh where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS App Mesh by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: define a service mesh with virtual nodes, routers, services, gateways, gateway routes, and routes.
- From the operation surface: support traffic shaping, canary routing, gateway ingress, mesh policy configuration, and tag-based service-mesh inventory.

## Service Identity and Protocol

- AWS model slug: `app-mesh`
- AWS SDK for Rust slug: `appmesh`
- Model version: `2019-01-25`
- Model file: `vendor/api-models-aws/models/app-mesh/service/2019-01-25/app-mesh-2019-01-25.json`
- SDK ID: `App Mesh`
- Endpoint prefix: `-`
- ARN namespace: `appmesh`
- CloudFormation name: `AppMesh`
- CloudTrail event source: `appmesh.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Create` (7), `Delete` (7), `Describe` (7), `Update` (7), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateGatewayRoute`, `CreateMesh`, `CreateRoute`, `CreateVirtualGateway`, `CreateVirtualNode`, `CreateVirtualRouter`, `CreateVirtualService`, `DeleteGatewayRoute`, `DeleteMesh`, `DeleteRoute`, `DeleteVirtualGateway`, `DeleteVirtualNode`, `DeleteVirtualRouter`, `DeleteVirtualService`, `TagResource`, `UntagResource`, `UpdateGatewayRoute`, `UpdateMesh`, `UpdateRoute`, `UpdateVirtualGateway`, ... (+3).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeGatewayRoute`, `DescribeMesh`, `DescribeRoute`, `DescribeVirtualGateway`, `DescribeVirtualNode`, `DescribeVirtualRouter`, `DescribeVirtualService`, `ListGatewayRoutes`, `ListMeshes`, `ListRoutes`, `ListTagsForResource`, `ListVirtualGateways`, `ListVirtualNodes`, `ListVirtualRouters`, `ListVirtualServices`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 23 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 38 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `GatewayRoute` | `gatewayRouteName`, `meshName`, `virtualGatewayName` | put: `CreateGatewayRoute`; read: `DescribeGatewayRoute`; update: `UpdateGatewayRoute`; delete: `DeleteGatewayRoute`; list: `ListGatewayRoutes` | - | - |
| `Mesh` | `meshName` | put: `CreateMesh`; read: `DescribeMesh`; update: `UpdateMesh`; delete: `DeleteMesh`; list: `ListMeshes` | - | - |
| `Route` | `meshName`, `routeName`, `virtualRouterName` | put: `CreateRoute`; read: `DescribeRoute`; update: `UpdateRoute`; delete: `DeleteRoute`; list: `ListRoutes` | - | - |
| `VirtualGateway` | `meshName`, `virtualGatewayName` | put: `CreateVirtualGateway`; read: `DescribeVirtualGateway`; update: `UpdateVirtualGateway`; delete: `DeleteVirtualGateway`; list: `ListVirtualGateways` | - | - |
| `VirtualNode` | `meshName`, `virtualNodeName` | put: `CreateVirtualNode`; read: `DescribeVirtualNode`; update: `UpdateVirtualNode`; delete: `DeleteVirtualNode`; list: `ListVirtualNodes` | - | - |
| `VirtualRouter` | `meshName`, `virtualRouterName` | put: `CreateVirtualRouter`; read: `DescribeVirtualRouter`; update: `UpdateVirtualRouter`; delete: `DeleteVirtualRouter`; list: `ListVirtualRouters` | - | - |
| `VirtualService` | `meshName`, `virtualServiceName` | put: `CreateVirtualService`; read: `DescribeVirtualService`; update: `UpdateVirtualService`; delete: `DeleteVirtualService`; list: `ListVirtualServices` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/app-mesh/latest/userguide/what-is-app-mesh.html
- https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_nodes.html
- https://docs.aws.amazon.com/app-mesh/latest/userguide/proxy-authorization.html

Research outcomes:
- App Mesh models service-to-service communication through meshes, virtual services, virtual nodes, virtual routers, routes, and virtual gateways.
- Applications send and receive traffic through Envoy proxies that receive configuration from the App Mesh control plane.
- A virtual service represents an abstract service name and is backed by either a virtual node or a virtual router.
- A virtual node represents a logical service task or deployment and defines listeners, service discovery, backend dependencies, TLS, health checks, outlier detection, and logging.
- Virtual routers and routes define how traffic is distributed among target virtual nodes, including weighted routing and protocol-specific route matching.
- Virtual gateways provide ingress into the mesh and also use Envoy proxy deployments.
- Envoy proxies require IAM authorisation such as `appmesh:StreamAggregatedResources` for the compute resource role.
- Connectivity to services not listed as a virtual node backend can still succeed depending on proxy and traffic interception configuration, so declared backends are control-plane intent rather than simple network firewall rules.

Parity implications:
- Model meshes, virtual services, virtual nodes, virtual routers, routes, virtual gateways, gateway routes, Envoy authorisation, and proxy status separately.
- Route resolution should follow virtual service providers, router route matches, and weighted targets.
- Proxy-facing configuration state should be derived from mesh resources and IAM authorisation rather than stored as inert metadata.

## Operation Groups

### List

- Operations: `ListGatewayRoutes`, `ListMeshes`, `ListRoutes`, `ListTagsForResource`, `ListVirtualGateways`, `ListVirtualNodes`, `ListVirtualRouters`, `ListVirtualServices`
- Traits: `paginated` (8), `readonly` (8)
- Common required input members in this group: `meshName`, `resourceArn`, `virtualGatewayName`, `virtualRouterName`

### Create

- Operations: `CreateGatewayRoute`, `CreateMesh`, `CreateRoute`, `CreateVirtualGateway`, `CreateVirtualNode`, `CreateVirtualRouter`, `CreateVirtualService`
- Traits: `idempotency-token` (7), `idempotent` (7)
- Common required input members in this group: `gatewayRouteName`, `meshName`, `routeName`, `spec`, `virtualGatewayName`, `virtualNodeName`, `virtualRouterName`, `virtualServiceName`

### Delete

- Operations: `DeleteGatewayRoute`, `DeleteMesh`, `DeleteRoute`, `DeleteVirtualGateway`, `DeleteVirtualNode`, `DeleteVirtualRouter`, `DeleteVirtualService`
- Traits: `idempotent` (7)
- Common required input members in this group: `gatewayRouteName`, `meshName`, `routeName`, `virtualGatewayName`, `virtualNodeName`, `virtualRouterName`, `virtualServiceName`

### Describe

- Operations: `DescribeGatewayRoute`, `DescribeMesh`, `DescribeRoute`, `DescribeVirtualGateway`, `DescribeVirtualNode`, `DescribeVirtualRouter`, `DescribeVirtualService`
- Traits: `readonly` (7)
- Common required input members in this group: `gatewayRouteName`, `meshName`, `routeName`, `virtualGatewayName`, `virtualNodeName`, `virtualRouterName`, `virtualServiceName`

### Update

- Operations: `UpdateGatewayRoute`, `UpdateMesh`, `UpdateRoute`, `UpdateVirtualGateway`, `UpdateVirtualNode`, `UpdateVirtualRouter`, `UpdateVirtualService`
- Traits: `idempotency-token` (7), `idempotent` (7)
- Common required input members in this group: `gatewayRouteName`, `meshName`, `routeName`, `spec`, `virtualGatewayName`, `virtualNodeName`, `virtualRouterName`, `virtualServiceName`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateGatewayRoute` | `PUT /v20190125/meshes/{meshName}/virtualGateway/{virtualGatewayName}/gatewayRoutes` | `idempotent`, `idempotency-token` | `gatewayRouteName`, `meshName`, `spec`, `virtualGatewayName` | `clientToken` | `CreateGatewayRouteOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a gateway route. A gateway route is attached to a virtual gateway and routes traffic to an existing virtual service. |
| `CreateMesh` | `PUT /v20190125/meshes` | `idempotent`, `idempotency-token` | `meshName` | `clientToken` | `CreateMeshOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a service mesh. A service mesh is a logical boundary for network traffic between services that are represented by resources within the mesh. |
| `CreateRoute` | `PUT /v20190125/meshes/{meshName}/virtualRouter/{virtualRouterName}/routes` | `idempotent`, `idempotency-token` | `meshName`, `routeName`, `spec`, `virtualRouterName` | `clientToken` | `CreateRouteOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a route that is associated with a virtual router. You can route several different protocols and define a retry policy for a route. |
| `CreateVirtualGateway` | `PUT /v20190125/meshes/{meshName}/virtualGateways` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualGatewayName` | `clientToken` | `CreateVirtualGatewayOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a virtual gateway. A virtual gateway allows resources outside your mesh to communicate to resources that are inside your mesh. |
| `CreateVirtualNode` | `PUT /v20190125/meshes/{meshName}/virtualNodes` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualNodeName` | `clientToken` | `CreateVirtualNodeOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a virtual node within a service mesh. A virtual node acts as a logical pointer to a particular task group, such as an Amazon ECS service or a Kubernetes deployment. |
| `CreateVirtualRouter` | `PUT /v20190125/meshes/{meshName}/virtualRouters` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualRouterName` | `clientToken` | `CreateVirtualRouterOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a virtual router within a service mesh. Specify a `listener` for any inbound traffic that your virtual router receives. |
| `CreateVirtualService` | `PUT /v20190125/meshes/{meshName}/virtualServices` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualServiceName` | `clientToken` | `CreateVirtualServiceOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a virtual service within a service mesh. A virtual service is an abstraction of a real service that is provided by a virtual node directly or indirectly by means of a virtual router. |
| `DeleteGatewayRoute` | `DELETE /v20190125/meshes/{meshName}/virtualGateway/{virtualGatewayName}/gatewayRoutes/{gatewayRouteName}` | `idempotent` | `gatewayRouteName`, `meshName`, `virtualGatewayName` | - | `DeleteGatewayRouteOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing gateway route. |
| `DeleteMesh` | `DELETE /v20190125/meshes/{meshName}` | `idempotent` | `meshName` | - | `DeleteMeshOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing service mesh. You must delete all resources (virtual services, routes, virtual routers, and virtual nodes) in the service mesh before you can delete the mesh itself. |
| `DeleteRoute` | `DELETE /v20190125/meshes/{meshName}/virtualRouter/{virtualRouterName}/routes/{routeName}` | `idempotent` | `meshName`, `routeName`, `virtualRouterName` | - | `DeleteRouteOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing route. |
| `DeleteVirtualGateway` | `DELETE /v20190125/meshes/{meshName}/virtualGateways/{virtualGatewayName}` | `idempotent` | `meshName`, `virtualGatewayName` | - | `DeleteVirtualGatewayOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing virtual gateway. You cannot delete a virtual gateway if any gateway routes are associated to it. |
| `DeleteVirtualNode` | `DELETE /v20190125/meshes/{meshName}/virtualNodes/{virtualNodeName}` | `idempotent` | `meshName`, `virtualNodeName` | - | `DeleteVirtualNodeOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing virtual node. You must delete any virtual services that list a virtual node as a service provider before you can delete the virtual node itself. |
| `DeleteVirtualRouter` | `DELETE /v20190125/meshes/{meshName}/virtualRouters/{virtualRouterName}` | `idempotent` | `meshName`, `virtualRouterName` | - | `DeleteVirtualRouterOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing virtual router. You must delete any routes associated with the virtual router before you can delete the router itself. |
| `DeleteVirtualService` | `DELETE /v20190125/meshes/{meshName}/virtualServices/{virtualServiceName}` | `idempotent` | `meshName`, `virtualServiceName` | - | `DeleteVirtualServiceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ResourceInUseException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes an existing virtual service. |
| `DescribeGatewayRoute` | `GET /v20190125/meshes/{meshName}/virtualGateway/{virtualGatewayName}/gatewayRoutes/{gatewayRouteName}` | `readonly` | `gatewayRouteName`, `meshName`, `virtualGatewayName` | - | `DescribeGatewayRouteOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing gateway route. |
| `DescribeMesh` | `GET /v20190125/meshes/{meshName}` | `readonly` | `meshName` | - | `DescribeMeshOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing service mesh. |
| `DescribeRoute` | `GET /v20190125/meshes/{meshName}/virtualRouter/{virtualRouterName}/routes/{routeName}` | `readonly` | `meshName`, `routeName`, `virtualRouterName` | - | `DescribeRouteOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing route. |
| `DescribeVirtualGateway` | `GET /v20190125/meshes/{meshName}/virtualGateways/{virtualGatewayName}` | `readonly` | `meshName`, `virtualGatewayName` | - | `DescribeVirtualGatewayOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing virtual gateway. |
| `DescribeVirtualNode` | `GET /v20190125/meshes/{meshName}/virtualNodes/{virtualNodeName}` | `readonly` | `meshName`, `virtualNodeName` | - | `DescribeVirtualNodeOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing virtual node. |
| `DescribeVirtualRouter` | `GET /v20190125/meshes/{meshName}/virtualRouters/{virtualRouterName}` | `readonly` | `meshName`, `virtualRouterName` | - | `DescribeVirtualRouterOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing virtual router. |
| `DescribeVirtualService` | `GET /v20190125/meshes/{meshName}/virtualServices/{virtualServiceName}` | `readonly` | `meshName`, `virtualServiceName` | - | `DescribeVirtualServiceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes an existing virtual service. |
| `ListGatewayRoutes` | `GET /v20190125/meshes/{meshName}/virtualGateway/{virtualGatewayName}/gatewayRoutes` | `readonly`, `paginated` | `meshName`, `virtualGatewayName` | - | `ListGatewayRoutesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing gateway routes that are associated to a virtual gateway. |
| `ListMeshes` | `GET /v20190125/meshes` | `readonly`, `paginated` | - | - | `ListMeshesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing service meshes. |
| `ListRoutes` | `GET /v20190125/meshes/{meshName}/virtualRouter/{virtualRouterName}/routes` | `readonly`, `paginated` | `meshName`, `virtualRouterName` | - | `ListRoutesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing routes in a service mesh. |
| `ListTagsForResource` | `GET /v20190125/tags` | `readonly`, `paginated` | `resourceArn` | - | `ListTagsForResourceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | List the tags for an App Mesh resource. |
| `ListVirtualGateways` | `GET /v20190125/meshes/{meshName}/virtualGateways` | `readonly`, `paginated` | `meshName` | - | `ListVirtualGatewaysOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing virtual gateways in a service mesh. |
| `ListVirtualNodes` | `GET /v20190125/meshes/{meshName}/virtualNodes` | `readonly`, `paginated` | `meshName` | - | `ListVirtualNodesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing virtual nodes. |
| `ListVirtualRouters` | `GET /v20190125/meshes/{meshName}/virtualRouters` | `readonly`, `paginated` | `meshName` | - | `ListVirtualRoutersOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing virtual routers in a service mesh. |
| `ListVirtualServices` | `GET /v20190125/meshes/{meshName}/virtualServices` | `readonly`, `paginated` | `meshName` | - | `ListVirtualServicesOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Returns a list of existing virtual services in a service mesh. |
| `TagResource` | `PUT /v20190125/tag` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException` | Associates the specified tags to a resource with the specified `resourceArn`. If existing tags on a resource aren't specified in the request parameters, they aren't changed. |
| `UntagResource` | `PUT /v20190125/untag` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes specified tags from a resource. |
| `UpdateGatewayRoute` | `PUT /v20190125/meshes/{meshName}/virtualGateway/{virtualGatewayName}/gatewayRoutes/{gatewayRouteName}` | `idempotent`, `idempotency-token` | `gatewayRouteName`, `meshName`, `spec`, `virtualGatewayName` | `clientToken` | `UpdateGatewayRouteOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing gateway route that is associated to a specified virtual gateway in a service mesh. |
| `UpdateMesh` | `PUT /v20190125/meshes/{meshName}` | `idempotent`, `idempotency-token` | `meshName` | `clientToken` | `UpdateMeshOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing service mesh. |
| `UpdateRoute` | `PUT /v20190125/meshes/{meshName}/virtualRouter/{virtualRouterName}/routes/{routeName}` | `idempotent`, `idempotency-token` | `meshName`, `routeName`, `spec`, `virtualRouterName` | `clientToken` | `UpdateRouteOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing route for a specified service mesh and virtual router. |
| `UpdateVirtualGateway` | `PUT /v20190125/meshes/{meshName}/virtualGateways/{virtualGatewayName}` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualGatewayName` | `clientToken` | `UpdateVirtualGatewayOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing virtual gateway in a specified service mesh. |
| `UpdateVirtualNode` | `PUT /v20190125/meshes/{meshName}/virtualNodes/{virtualNodeName}` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualNodeName` | `clientToken` | `UpdateVirtualNodeOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing virtual node in a specified service mesh. |
| `UpdateVirtualRouter` | `PUT /v20190125/meshes/{meshName}/virtualRouters/{virtualRouterName}` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualRouterName` | `clientToken` | `UpdateVirtualRouterOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing virtual router in a specified service mesh. |
| `UpdateVirtualService` | `PUT /v20190125/meshes/{meshName}/virtualServices/{virtualServiceName}` | `idempotent`, `idempotency-token` | `meshName`, `spec`, `virtualServiceName` | `clientToken` | `UpdateVirtualServiceOutput` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing virtual service in a specified service mesh. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListTagsForResource` | - | `resourceArn -> resourceArn`, `nextToken -> nextToken`, `limit -> limit` | - | - |
| `TagResource` | - | `resourceArn -> resourceArn` | - | - |
| `UntagResource` | - | `resourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `message` | The request syntax was malformed. |
| `ForbiddenException` | `structure` | `message` | You don't have permissions to perform this action. |
| `InternalServerErrorException` | `structure` | `message` | The request processing has failed because of an unknown error, exception, or failure. |
| `NotFoundException` | `structure` | `message` | The specified resource doesn't exist. |
| `ServiceUnavailableException` | `structure` | `message` | The request has failed due to a temporary failure of the service. |
| `TooManyRequestsException` | `structure` | `message` | The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. |
| `ConflictException` | `structure` | `message` | The request contains a client token that was used for a previous update resource call with different specifications. |
| `LimitExceededException` | `structure` | `message` | You have exceeded a service limit for your account. |
| `ResourceInUseException` | `structure` | `message` | You can't delete the specified resource because it's in use or required by another resource. |
| `CreateGatewayRouteInput` | `structure` | `clientToken`, `gatewayRouteName`, `meshName`, `meshOwner`, `spec`, `tags`, `virtualGatewayName` | - |
| `CreateGatewayRouteOutput` | `structure` | `gatewayRoute` | - |
| `CreateMeshInput` | `structure` | `clientToken`, `meshName`, `spec`, `tags` | - |
| `CreateMeshOutput` | `structure` | `mesh` | - |
| `CreateRouteInput` | `structure` | `clientToken`, `meshName`, `meshOwner`, `routeName`, `spec`, `tags`, `virtualRouterName` | - |
| `CreateRouteOutput` | `structure` | `route` | - |
| `CreateVirtualGatewayInput` | `structure` | `clientToken`, `meshName`, `meshOwner`, `spec`, `tags`, `virtualGatewayName` | - |
| `CreateVirtualGatewayOutput` | `structure` | `virtualGateway` | - |
| `CreateVirtualNodeInput` | `structure` | `clientToken`, `meshName`, `meshOwner`, `spec`, `tags`, `virtualNodeName` | - |
| `CreateVirtualNodeOutput` | `structure` | `virtualNode` | - |
| `CreateVirtualRouterInput` | `structure` | `clientToken`, `meshName`, `meshOwner`, `spec`, `tags`, `virtualRouterName` | - |
| `CreateVirtualRouterOutput` | `structure` | `virtualRouter` | - |
| `CreateVirtualServiceInput` | `structure` | `clientToken`, `meshName`, `meshOwner`, `spec`, `tags`, `virtualServiceName` | - |
| `CreateVirtualServiceOutput` | `structure` | `virtualService` | - |
| `DeleteGatewayRouteInput` | `structure` | `gatewayRouteName`, `meshName`, `meshOwner`, `virtualGatewayName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

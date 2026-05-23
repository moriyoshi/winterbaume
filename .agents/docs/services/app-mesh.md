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

- Operations: `ListTagsForResource`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /v20190125/tags` | `readonly`, `paginated` | `resourceArn` | - | `ListTagsForResourceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | List the tags for an App Mesh resource. |
| `TagResource` | `PUT /v20190125/tag` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `TooManyTagsException` | Associates the specified tags to a resource with the specified resourceArn . If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags associ ... |
| `UntagResource` | `PUT /v20190125/untag` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes specified tags from a resource. |

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
| `BadRequestException` | `structure` | message | The request syntax was malformed. Check your request syntax and try again. |
| `ConflictException` | `structure` | message | The request contains a client token that was used for a previous update resource call with different specifications. Try the request again with a new client ... |
| `ForbiddenException` | `structure` | message | You don't have permissions to perform this action. |
| `InternalServerErrorException` | `structure` | message | The request processing has failed because of an unknown error, exception, or failure. |
| `LimitExceededException` | `structure` | message | You have exceeded a service limit for your account. For more information, see Service Limits in the App Mesh User Guide . |
| `NotFoundException` | `structure` | message | The specified resource doesn't exist. Check your request syntax and try again. |
| `ResourceInUseException` | `structure` | message | You can't delete the specified resource because it's in use or required by another resource. |
| `ServiceUnavailableException` | `structure` | message | The request has failed due to a temporary failure of the service. |
| `TooManyRequestsException` | `structure` | message | The maximum request rate permitted by the App Mesh APIs has been exceeded for your account. For best results, use an increasing or variable sleep interval b ... |
| `TooManyTagsException` | `structure` | message | The request exceeds the maximum allowed number of tags allowed per resource. The current limit is 50 user tags per resource. You must reduce the number of t ... |
| `ListTagsForResourceInput` | `structure` | resourceArn, nextToken, limit | - |
| `ListTagsForResourceOutput` | `structure` | tags, nextToken | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

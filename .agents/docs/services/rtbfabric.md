# RTBFabric

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services RTB Fabric provides secure, low-latency infrastructure for connecting real-time bidding (RTB) applications. Rather than hosting applications directly, RTB Fabric acts as the connecting fabric that enables your applications to communicate efficiently over private networks instead of the public internet. You maintain complete control over your applications, data, and bidding decisions, while RTB Fabric provides the underlying infrastructure for secure, reliable connectivity. You can use these APIs to complete RTB Fabric tasks, such as setting up audit log ingestions or viewing user access. For more information about RTB Fabric, including the required permissions to use the service, see the Amazon Web Services RTB Fabric User Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented RTBFabric workflows in the local mock. Key resources include `Gateway`, `InboundExternalLink`, `Link`, `OutboundExternalLink`, `RequesterGateway`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Delete`, `Get`, `List`, `Update` operation families, including `CreateInboundExternalLink`, `CreateLink`, `CreateOutboundExternalLink`, `CreateRequesterGateway`, `DeleteInboundExternalLink`, `DeleteLink`.

## Service Identity and Protocol

- AWS model slug: `rtbfabric`
- AWS SDK for Rust slug: `rtbfabric`
- Model version: `2023-05-15`
- Model file: `vendor/api-models-aws/models/rtbfabric/service/2023-05-15/rtbfabric-2023-05-15.json`
- SDK ID: `RTBFabric`
- Endpoint prefix: `-`
- ARN namespace: `rtbfabric`
- CloudFormation name: `-`
- CloudTrail event source: `rtbfabric.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (5), `Delete` (5), `Get` (5), `List` (4), `Update` (4), `Accept` (1), `Reject` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptLink`, `CreateInboundExternalLink`, `CreateLink`, `CreateOutboundExternalLink`, `CreateRequesterGateway`, `CreateResponderGateway`, `DeleteInboundExternalLink`, `DeleteLink`, `DeleteOutboundExternalLink`, `DeleteRequesterGateway`, `DeleteResponderGateway`, `RejectLink`, `TagResource`, `UntagResource`, `UpdateLink`, `UpdateLinkModuleFlow`, `UpdateRequesterGateway`, `UpdateResponderGateway`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetInboundExternalLink`, `GetLink`, `GetOutboundExternalLink`, `GetRequesterGateway`, `GetResponderGateway`, `ListLinks`, `ListRequesterGateways`, `ListResponderGateways`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 18 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 27 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EKS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Gateway` | `gatewayId` | - | - | - |
| `InboundExternalLink` | `gatewayId`, `linkId` | create: `CreateInboundExternalLink` | `DeleteInboundExternalLink`, `GetInboundExternalLink` | - |
| `Link` | `gatewayId`, `linkId` | create: `CreateLink`; read: `GetLink`; delete: `DeleteLink`; list: `ListLinks` | `AcceptLink`, `RejectLink`, `UpdateLink`, `UpdateLinkModuleFlow` | - |
| `OutboundExternalLink` | `gatewayId`, `linkId` | create: `CreateOutboundExternalLink` | `DeleteOutboundExternalLink`, `GetOutboundExternalLink` | - |
| `RequesterGateway` | `gatewayId` | create: `CreateRequesterGateway`; read: `GetRequesterGateway`; delete: `DeleteRequesterGateway` | `UpdateRequesterGateway` | - |
| `ResponderGateway` | `gatewayId` | create: `CreateResponderGateway`; read: `GetResponderGateway`; delete: `DeleteResponderGateway` | `UpdateResponderGateway` | - |
## Operation Groups

### Create

- Operations: `CreateInboundExternalLink`, `CreateLink`, `CreateOutboundExternalLink`, `CreateRequesterGateway`, `CreateResponderGateway`
- Traits: `idempotency-token` (4), `idempotent` (5)
- Common required input members in this group: `clientToken`, `gatewayId`, `logSettings`, `peerGatewayId`, `port`, `protocol`, `publicEndpoint`, `securityGroupIds`, `subnetIds`, `vpcId`

### Delete

- Operations: `DeleteInboundExternalLink`, `DeleteLink`, `DeleteOutboundExternalLink`, `DeleteRequesterGateway`, `DeleteResponderGateway`
- Traits: `idempotent` (5)
- Common required input members in this group: `gatewayId`, `linkId`

### Get

- Operations: `GetInboundExternalLink`, `GetLink`, `GetOutboundExternalLink`, `GetRequesterGateway`, `GetResponderGateway`
- Traits: `readonly` (5)
- Common required input members in this group: `gatewayId`, `linkId`

### List

- Operations: `ListLinks`, `ListRequesterGateways`, `ListResponderGateways`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `gatewayId`, `resourceArn`

### Update

- Operations: `UpdateLink`, `UpdateLinkModuleFlow`, `UpdateRequesterGateway`, `UpdateResponderGateway`
- Traits: `idempotency-token` (3), `idempotent` (4)
- Common required input members in this group: `clientToken`, `gatewayId`, `linkId`, `modules`, `port`, `protocol`

### Accept

- Operations: `AcceptLink`
- Traits: `idempotent` (1)
- Common required input members in this group: `gatewayId`, `linkId`, `logSettings`

### Reject

- Operations: `RejectLink`
- Traits: `idempotent` (1)
- Common required input members in this group: `gatewayId`, `linkId`

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
| `AcceptLink` | `POST /gateway/{gatewayId}/link/{linkId}/accept` | `idempotent` | `gatewayId`, `linkId`, `logSettings` | - | `AcceptLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Accepts a link request between gateways. When a requester gateway requests to link with a responder gateway, the responder can use this operation to accept the link request and establish the connection. |
| `CreateInboundExternalLink` | `POST /responder-gateway/{gatewayId}/inbound-external-link` | `idempotent`, `idempotency-token` | `clientToken`, `gatewayId`, `logSettings` | `clientToken` | `CreateInboundExternalLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an inbound external link. |
| `CreateLink` | `POST /gateway/{gatewayId}/create-link` | `idempotent` | `gatewayId`, `logSettings`, `peerGatewayId` | - | `CreateLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new link between gateways. Establishes a connection that allows gateways to communicate and exchange bid requests and responses. |
| `CreateOutboundExternalLink` | `POST /requester-gateway/{gatewayId}/outbound-external-link` | `idempotent`, `idempotency-token` | `clientToken`, `gatewayId`, `logSettings`, `publicEndpoint` | `clientToken` | `CreateOutboundExternalLinkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an outbound external link. |
| `CreateRequesterGateway` | `POST /requester-gateway` | `idempotent`, `idempotency-token` | `clientToken`, `securityGroupIds`, `subnetIds`, `vpcId` | `clientToken` | `CreateRequesterGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a requester gateway. |
| `CreateResponderGateway` | `POST /responder-gateway` | `idempotent`, `idempotency-token` | `clientToken`, `port`, `protocol`, `securityGroupIds`, `subnetIds`, `vpcId` | `clientToken` | `CreateResponderGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a responder gateway. A domain name or managed endpoint is required. |
| `DeleteInboundExternalLink` | `DELETE /responder-gateway/{gatewayId}/inbound-external-link/{linkId}` | `idempotent` | `gatewayId`, `linkId` | - | `DeleteInboundExternalLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an inbound external link. |
| `DeleteLink` | `DELETE /gateway/{gatewayId}/link/{linkId}` | `idempotent` | `gatewayId`, `linkId` | - | `DeleteLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a link between gateways. Permanently removes the connection between gateways. |
| `DeleteOutboundExternalLink` | `DELETE /requester-gateway/{gatewayId}/outbound-external-link/{linkId}` | `idempotent` | `gatewayId`, `linkId` | - | `DeleteOutboundExternalLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an outbound external link. |
| `DeleteRequesterGateway` | `DELETE /requester-gateway/{gatewayId}` | `idempotent` | `gatewayId` | - | `DeleteRequesterGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a requester gateway. |
| `DeleteResponderGateway` | `DELETE /responder-gateway/{gatewayId}` | `idempotent` | `gatewayId` | - | `DeleteResponderGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a responder gateway. |
| `GetInboundExternalLink` | `GET /responder-gateway/{gatewayId}/inbound-external-link/{linkId}` | `readonly` | `gatewayId`, `linkId` | - | `GetInboundExternalLinkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an inbound external link. |
| `GetLink` | `GET /gateway/{gatewayId}/link/{linkId}` | `readonly` | `gatewayId`, `linkId` | - | `GetLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a link between gateways. Returns detailed information about the link configuration, status, and associated gateways. |
| `GetOutboundExternalLink` | `GET /requester-gateway/{gatewayId}/outbound-external-link/{linkId}` | `readonly` | `gatewayId`, `linkId` | - | `GetOutboundExternalLinkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an outbound external link. |
| `GetRequesterGateway` | `GET /requester-gateway/{gatewayId}` | `readonly` | `gatewayId` | - | `GetRequesterGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a requester gateway. |
| `GetResponderGateway` | `GET /responder-gateway/{gatewayId}` | `readonly` | `gatewayId` | - | `GetResponderGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a responder gateway. |
| `ListLinks` | `GET /gateway/{gatewayId}/links/` | `readonly`, `paginated` | `gatewayId` | - | `ListLinksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists links associated with gateways. Returns a list of all links for the specified gateways, including their status and configuration details. |
| `ListRequesterGateways` | `GET /requester-gateways` | `readonly`, `paginated` | - | - | `ListRequesterGatewaysResponse` | `InternalServerException`, `ValidationException` | Lists requester gateways. |
| `ListResponderGateways` | `GET /responder-gateways` | `readonly`, `paginated` | - | - | `ListResponderGatewaysResponse` | `InternalServerException`, `ValidationException` | Lists reponder gateways. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags for a resource. |
| `RejectLink` | `POST /gateway/{gatewayId}/link/{linkId}/reject` | `idempotent` | `gatewayId`, `linkId` | - | `RejectLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects a link request between gateways. When a requester gateway requests to link with a responder gateway, the responder can use this operation to decline the link request. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |
| `UpdateLink` | `PATCH /gateway/{gatewayId}/link/{linkId}` | `idempotent` | `gatewayId`, `linkId` | - | `UpdateLinkResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of a link between gateways. Allows you to modify settings and parameters for an existing link. |
| `UpdateLinkModuleFlow` | `POST /gateway/{gatewayId}/link/{linkId}/module-flow` | `idempotent`, `idempotency-token` | `clientToken`, `gatewayId`, `linkId`, `modules` | `clientToken` | `UpdateLinkModuleFlowResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a link module flow. |
| `UpdateRequesterGateway` | `POST /requester-gateway/{gatewayId}/update` | `idempotent`, `idempotency-token` | `clientToken`, `gatewayId` | `clientToken` | `UpdateRequesterGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a requester gateway. |
| `UpdateResponderGateway` | `POST /responder-gateway/{gatewayId}/update` | `idempotent`, `idempotency-token` | `clientToken`, `gatewayId`, `port`, `protocol` | `clientToken` | `UpdateResponderGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a responder gateway. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListRequesterGateways` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListResponderGateways` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | The request could not be completed because of an internal server error. |
| `ValidationException` | `structure` | `message` | The request could not be completed because it fails satisfy the constraints specified by the service. |
| `AccessDeniedException` | `structure` | `message` | The request could not be completed because you do not have sufficient access to perform this action. |
| `ResourceNotFoundException` | `structure` | `message` | The request could not be completed because the resource does not exist. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ConflictException` | `structure` | `message` | The request could not be completed because of a conflict in the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request could not be completed because you exceeded a service quota. |
| `AcceptLinkRequest` | `structure` | `attributes`, `gatewayId`, `linkId`, `logSettings` | - |
| `AcceptLinkResponse` | `structure` | `attributes`, `createdAt`, `direction`, `flowModules`, `gatewayId`, `linkId`, `peerGatewayId`, `pendingFlowModules`, `status`, `updatedAt` | - |
| `CreateInboundExternalLinkRequest` | `structure` | `attributes`, `clientToken`, `gatewayId`, `logSettings`, `tags` | - |
| `CreateInboundExternalLinkResponse` | `structure` | `domainName`, `gatewayId`, `linkId`, `status` | - |
| `CreateLinkRequest` | `structure` | `attributes`, `gatewayId`, `httpResponderAllowed`, `logSettings`, `peerGatewayId`, `tags` | - |
| `CreateLinkResponse` | `structure` | `attributes`, `createdAt`, `customerProvidedId`, `direction`, `flowModules`, `gatewayId`, `linkId`, `peerGatewayId`, `pendingFlowModules`, `status`, `updatedAt` | - |
| `CreateOutboundExternalLinkRequest` | `structure` | `attributes`, `clientToken`, `gatewayId`, `logSettings`, `publicEndpoint`, `tags` | - |
| `CreateOutboundExternalLinkResponse` | `structure` | `gatewayId`, `linkId`, `status` | - |
| `CreateRequesterGatewayRequest` | `structure` | `clientToken`, `description`, `securityGroupIds`, `subnetIds`, `tags`, `vpcId` | - |
| `CreateRequesterGatewayResponse` | `structure` | `domainName`, `gatewayId`, `status` | - |
| `CreateResponderGatewayRequest` | `structure` | `clientToken`, `description`, `domainName`, `managedEndpointConfiguration`, `port`, `protocol`, `securityGroupIds`, `subnetIds`, `tags`, `trustStoreConfiguration`, `vpcId` | - |
| `CreateResponderGatewayResponse` | `structure` | `gatewayId`, `status` | - |
| `DeleteInboundExternalLinkRequest` | `structure` | `gatewayId`, `linkId` | - |
| `DeleteInboundExternalLinkResponse` | `structure` | `linkId`, `status` | - |
| `DeleteLinkRequest` | `structure` | `gatewayId`, `linkId` | - |
| `DeleteLinkResponse` | `structure` | `linkId`, `status` | - |
| `DeleteOutboundExternalLinkRequest` | `structure` | `gatewayId`, `linkId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

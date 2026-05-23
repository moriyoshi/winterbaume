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

### List

- Operations: `ListRequesterGateways`, `ListResponderGateways`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
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
| `ListRequesterGateways` | `GET /requester-gateways` | `readonly`, `paginated` | - | - | `ListRequesterGatewaysResponse` | `InternalServerException`, `ValidationException` | Lists requester gateways. |
| `ListResponderGateways` | `GET /responder-gateways` | `readonly`, `paginated` | - | - | `ListResponderGatewaysResponse` | `InternalServerException`, `ValidationException` | Lists reponder gateways. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags for a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |

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
| `AccessDeniedException` | `structure` | message | The request could not be completed because you do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The request could not be completed because of a conflict in the current state of the resource. |
| `InternalServerException` | `structure` | message | The request could not be completed because of an internal server error. Try your call again. |
| `ResourceNotFoundException` | `structure` | message | The request could not be completed because the resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | The request could not be completed because you exceeded a service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The request could not be completed because it fails satisfy the constraints specified by the service. |
| `ListRequesterGatewaysRequest` | `structure` | maxResults, nextToken | - |
| `ListRequesterGatewaysResponse` | `structure` | gatewayIds, nextToken | - |
| `ListResponderGatewaysRequest` | `structure` | maxResults, nextToken | - |
| `ListResponderGatewaysResponse` | `structure` | gatewayIds, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ConnectivityType` | `enum` | DEFAULT, PUBLIC_INGRESS, PUBLIC_EGRESS, EXTERNAL_INBOUND | The connectivity type for a link or gateway. |
| `FilterType` | `enum` | INCLUDE, EXCLUDE | - |
| `GatewayType` | `enum` | EXTERNAL, INTERNAL | The type of gateway. |
| `LinkDirection` | `enum` | RESPONSE, REQUEST | - |
| `LinkStatus` | `enum` | PENDING_CREATION, PENDING_REQUEST, REQUESTED, ACCEPTED, ACTIVE, REJECTED, FAILED, PENDING_DELETION, DELETED, PENDING_UPDATE, PENDING_ISOLATION, ISOLATED, ... (+1) | - |
| `Protocol` | `enum` | HTTP, HTTPS | - |
| `RequesterGatewayStatus` | `enum` | PENDING_CREATION, ACTIVE, PENDING_DELETION, DELETED, ERROR, PENDING_UPDATE, ISOLATED, PENDING_ISOLATION, PENDING_RESTORATION | - |
| `ResponderErrorMaskingAction` | `enum` | NO_BID, PASSTHROUGH | - |
| `ResponderErrorMaskingLoggingType` | `enum` | NONE, METRIC, RESPONSE | - |
| `ResponderGatewayStatus` | `enum` | PENDING_CREATION, ACTIVE, PENDING_DELETION, DELETED, ERROR, PENDING_UPDATE, ISOLATED, PENDING_ISOLATION, PENDING_RESTORATION | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

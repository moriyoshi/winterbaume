# AWS IoT Secure Tunneling

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IoT Secure Tunneling IoT Secure Tunneling creates remote connections to devices deployed in the field. For more information about how IoT Secure Tunneling works, see IoT Secure Tunneling.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS IoT Secure Tunneling workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Close`, `Describe`, `Open`, `Rotate` operation families, including `ListTagsForResource`, `ListTunnels`, `CloseTunnel`, `DescribeTunnel`, `OpenTunnel`, `RotateTunnelAccessToken`.

## Service Identity and Protocol

- AWS model slug: `iotsecuretunneling`
- AWS SDK for Rust slug: `iotsecuretunneling`
- Model version: `2018-10-05`
- Model file: `vendor/api-models-aws/models/iotsecuretunneling/service/2018-10-05/iotsecuretunneling-2018-10-05.json`
- SDK ID: `IoTSecureTunneling`
- Endpoint prefix: `api.tunneling.iot`
- ARN namespace: `iotsecuredtunneling`
- CloudFormation name: `IoTSecureTunneling`
- CloudTrail event source: `iotsecuretunneling.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Close` (1), `Describe` (1), `Open` (1), `Rotate` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeTunnel`, `ListTagsForResource`, `ListTunnels`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 7 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### List

- Operations: `ListTagsForResource`, `ListTunnels`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Close

- Operations: `CloseTunnel`
- Common required input members in this group: -

### Describe

- Operations: `DescribeTunnel`
- Common required input members in this group: -

### Open

- Operations: `OpenTunnel`
- Common required input members in this group: -

### Rotate

- Operations: `RotateTunnelAccessToken`
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
| `CloseTunnel` | `DELETE /tunnels/{tunnelId}` | - | `tunnelId` | - | `CloseTunnelResponse` | `ResourceNotFoundException` | Closes a tunnel identified by the unique tunnel id. When a CloseTunnel request is received, we close the WebSocket connections between the client and proxy server so no data can be transmitted. Requires permission to ... |
| `DescribeTunnel` | `GET /tunnels/{tunnelId}` | - | `tunnelId` | - | `DescribeTunnelResponse` | `ResourceNotFoundException` | Gets information about a tunnel identified by the unique tunnel id. Requires permission to access the DescribeTunnel action. |
| `ListTagsForResource` | `GET /tags` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags for the specified resource. |
| `ListTunnels` | `GET /tunnels` | `paginated` | - | - | `ListTunnelsResponse` | - | List all tunnels for an Amazon Web Services account. Tunnels are listed by creation time in descending order, newer tunnels will be listed before older tunnels. Requires permission to access the ListTunnels action. |
| `OpenTunnel` | `POST /tunnels` | - | - | - | `OpenTunnelResponse` | `LimitExceededException` | Creates a new tunnel, and returns two client access tokens for clients to use to connect to the IoT Secure Tunneling proxy server. Requires permission to access the OpenTunnel action. |
| `RotateTunnelAccessToken` | `POST /tunnel/{tunnelId}/rotate` | - | `tunnelId`, `clientMode` | - | `RotateTunnelAccessTokenResponse` | `ResourceNotFoundException` | Revokes the current client access token (CAT) and returns new CAT for clients to use when reconnecting to secure tunneling to access the same tunnel. Requires permission to access the RotateTunnelAccessToken action. ... |
| `TagResource` | `POST /tags` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException` | A resource tag. |
| `UntagResource` | `POST /untag` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes a tag from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CloseTunnel` | - | `delete -> delete` | - | - |
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |
| `ListTunnels` | - | `thingName -> thingName`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `LimitExceededException` | `structure` | message | Thrown when a tunnel limit is exceeded. |
| `ResourceNotFoundException` | `structure` | message | Thrown when an operation is attempted on a resource that does not exist. |
| `CloseTunnelRequest` | `structure` | tunnelId, delete | - |
| `CloseTunnelResponse` | `structure` | **empty (no members)** | - |
| `DescribeTunnelRequest` | `structure` | tunnelId | - |
| `DescribeTunnelResponse` | `structure` | tunnel | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `ListTunnelsRequest` | `structure` | thingName, maxResults, nextToken | - |
| `ListTunnelsResponse` | `structure` | tunnelSummaries, nextToken | - |
| `OpenTunnelRequest` | `structure` | description, tags, destinationConfig, timeoutConfig | - |
| `OpenTunnelResponse` | `structure` | tunnelId, tunnelArn, sourceAccessToken, destinationAccessToken | - |
| `RotateTunnelAccessTokenRequest` | `structure` | tunnelId, clientMode, destinationConfig | - |
| `RotateTunnelAccessTokenResponse` | `structure` | tunnelArn, sourceAccessToken, destinationAccessToken | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ClientMode` | `enum` | SOURCE, DESTINATION, ALL | - |
| `ConnectionStatus` | `enum` | CONNECTED, DISCONNECTED | - |
| `TunnelStatus` | `enum` | OPEN, CLOSED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AmazonApiGatewayManagementApi

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon API Gateway Management API allows you to directly manage runtime aspects of your deployed APIs. To use it, you must explicitly set the SDK's endpoint to point to the endpoint of your deployed API. The endpoint will be of the form https://{api-id}.execute-api.{region}.amazonaws.com/{stage}, or will be the endpoint corresponding to your API's custom domain and base path, if applicable.

## Possible Usage Scenarios
- From the AWS documentation and model: manage WebSocket API client connections by posting data, retrieving connection metadata, and deleting stale connections.
- From the operation surface: support callback-style messaging from application backends to connected WebSocket clients.

## Service Identity and Protocol

- AWS model slug: `apigatewaymanagementapi`
- AWS SDK for Rust slug: `apigatewaymanagement`
- Model version: `2018-11-29`
- Model file: `vendor/api-models-aws/models/apigatewaymanagementapi/service/2018-11-29/apigatewaymanagementapi-2018-11-29.json`
- SDK ID: `ApiGatewayManagementApi`
- Endpoint prefix: `execute-api`
- ARN namespace: `apigateway`
- CloudFormation name: `ApiGatewayManagementApi`
- CloudTrail event source: `apigatewaymanagementapi.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Get` (1), `Post` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteConnection`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetConnection`.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Delete

- Operations: `DeleteConnection`
- Common required input members in this group: `ConnectionId`

### Get

- Operations: `GetConnection`
- Common required input members in this group: `ConnectionId`

### Post

- Operations: `PostToConnection`
- Common required input members in this group: `ConnectionId`, `Data`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteConnection` | `DELETE /@connections/{ConnectionId}` | - | `ConnectionId` | - | `Unit` | `ForbiddenException`, `GoneException`, `LimitExceededException` | Delete the connection with the provided id. |
| `GetConnection` | `GET /@connections/{ConnectionId}` | - | `ConnectionId` | - | `GetConnectionResponse` | `ForbiddenException`, `GoneException`, `LimitExceededException` | Get information about the connection with the provided id. |
| `PostToConnection` | `POST /@connections/{ConnectionId}` | - | `ConnectionId`, `Data` | - | `Unit` | `ForbiddenException`, `GoneException`, `LimitExceededException`, `PayloadTooLargeException` | Sends the provided data to the specified connection. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ForbiddenException` | `structure` | - | The caller is not authorized to invoke this operation. |
| `GoneException` | `structure` | - | The connection with the provided id no longer exists. |
| `LimitExceededException` | `structure` | - | The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full. |
| `DeleteConnectionRequest` | `structure` | `ConnectionId` | - |
| `GetConnectionRequest` | `structure` | `ConnectionId` | - |
| `GetConnectionResponse` | `structure` | `ConnectedAt`, `Identity`, `LastActiveAt` | - |
| `PostToConnectionRequest` | `structure` | `ConnectionId`, `Data` | - |
| `PayloadTooLargeException` | `structure` | `Message` | The data has exceeded the maximum size allowed. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

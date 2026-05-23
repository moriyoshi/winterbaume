# Amazon S3 on Outposts

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon S3 on Outposts provides access to S3 on Outposts operations.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon S3 on Outposts workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete` operation families, including `ListEndpoints`, `ListOutpostsWithS3`, `ListSharedEndpoints`, `CreateEndpoint`, `DeleteEndpoint`.

## Service Identity and Protocol

- AWS model slug: `s3outposts`
- AWS SDK for Rust slug: `s3outposts`
- Model version: `2017-07-25`
- Model file: `vendor/api-models-aws/models/s3outposts/service/2017-07-25/s3outposts-2017-07-25.json`
- SDK ID: `S3Outposts`
- Endpoint prefix: `s3-outposts`
- ARN namespace: `s3-outposts`
- CloudFormation name: `S3Outposts`
- CloudTrail event source: `s3outposts.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (3), `Create` (1), `Delete` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateEndpoint`, `DeleteEndpoint`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ListEndpoints`, `ListOutpostsWithS3`, `ListSharedEndpoints`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`.

## Current Network Resource Stub Semantics

S3 Outposts currently stores endpoint networking as endpoint-local metadata.

- Endpoint creation requires subnet ID and security group ID, then mints a synthetic VPC ID and network interface ID from the endpoint ID.
- Describe/list responses return the stored subnet, security group, VPC, and network interface fields.
- Endpoint lifecycle does not create EC2 network interfaces or validate Outpost subnet placement.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListEndpoints`, `ListOutpostsWithS3`, `ListSharedEndpoints`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Create

- Operations: `CreateEndpoint`
- Common required input members in this group: -

### Delete

- Operations: `DeleteEndpoint`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEndpoint` | `POST /S3Outposts/CreateEndpoint` | - | `OutpostId`, `SubnetId`, `SecurityGroupId` | - | `CreateEndpointResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `OutpostOfflineException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an endpoint and associates it with the specified Outpost. It can take up to 5 minutes for this action to finish. Related actions include: DeleteEndpoint ListEndpoints |
| `DeleteEndpoint` | `DELETE /S3Outposts/DeleteEndpoint` | - | `EndpointId`, `OutpostId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `OutpostOfflineException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an endpoint. It can take up to 5 minutes for this action to finish. Related actions include: CreateEndpoint ListEndpoints |
| `ListEndpoints` | `GET /S3Outposts/ListEndpoints` | `paginated` | - | - | `ListEndpointsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists endpoints associated with the specified Outpost. Related actions include: CreateEndpoint DeleteEndpoint |
| `ListOutpostsWithS3` | `GET /S3Outposts/ListOutpostsWithS3` | `paginated` | - | - | `ListOutpostsWithS3Result` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the Outposts with S3 on Outposts capacity for your Amazon Web Services account. Includes S3 on Outposts that you have access to as the Outposts owner, or as a shared user from Resource Access Manager (RAM). |
| `ListSharedEndpoints` | `GET /S3Outposts/ListSharedEndpoints` | `paginated` | `OutpostId` | - | `ListSharedEndpointsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all endpoints associated with an Outpost that has been shared by Amazon Web Services Resource Access Manager (RAM). Related actions include: CreateEndpoint DeleteEndpoint |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteEndpoint` | - | `EndpointId -> endpointId`, `OutpostId -> outpostId` | - | - |
| `ListEndpoints` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListOutpostsWithS3` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListSharedEndpoints` | - | `NextToken -> nextToken`, `MaxResults -> maxResults`, `OutpostId -> outpostId` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | Access was denied for this action. |
| `ConflictException` | `structure` | Message | There was a conflict with this action, and it could not be completed. |
| `InternalServerException` | `structure` | Message | There was an exception with the internal server. |
| `OutpostOfflineException` | `structure` | Message | The service link connection to your Outposts home Region is down. Check your connection and try again. |
| `ResourceNotFoundException` | `structure` | Message | The requested resource was not found. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message | There was an exception validating this data. |
| `CreateEndpointRequest` | `structure` | OutpostId, SubnetId, SecurityGroupId, AccessType, CustomerOwnedIpv4Pool | - |
| `CreateEndpointResult` | `structure` | EndpointArn | - |
| `DeleteEndpointRequest` | `structure` | EndpointId, OutpostId | - |
| `ListEndpointsRequest` | `structure` | NextToken, MaxResults | - |
| `ListEndpointsResult` | `structure` | Endpoints, NextToken | - |
| `ListOutpostsWithS3Request` | `structure` | NextToken, MaxResults | - |
| `ListOutpostsWithS3Result` | `structure` | Outposts, NextToken | - |
| `ListSharedEndpointsRequest` | `structure` | NextToken, MaxResults, OutpostId | - |
| `ListSharedEndpointsResult` | `structure` | Endpoints, NextToken | - |
| `EndpointAccessType` | `enum` | PRIVATE, CUSTOMER_OWNED_IP | - |
| `EndpointStatus` | `enum` | PENDING, AVAILABLE, DELETING, CREATE_FAILED, DELETE_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

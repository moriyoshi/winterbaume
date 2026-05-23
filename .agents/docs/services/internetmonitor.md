# Amazon CloudWatch Internet Monitor

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudWatch Internet Monitor provides visibility into how internet issues impact the performance and availability between your applications hosted on Amazon Web Services and your end users. It can reduce the time it takes for you to diagnose internet issues from days to minutes. Internet Monitor uses the connectivity data that Amazon Web Services captures from its global networking footprint to calculate a baseline of performance and availability for internet traffic. This is the same data that Amazon Web Services uses to monitor internet uptime and availability. With those measurements as a baseline, Internet Monitor raises awareness for you when there are significant problems for your end users in the different geographic locations where your application runs.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon CloudWatch Internet Monitor workflows in the local mock. Key resources include `HealthEventResource`, `InternetEventResource`, `MonitorResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Start` operation families, including `GetHealthEvent`, `GetInternetEvent`, `GetMonitor`, `GetQueryResults`, `ListHealthEvents`, `ListInternetEvents`.

## Service Identity and Protocol

- AWS model slug: `internetmonitor`
- AWS SDK for Rust slug: `internetmonitor`
- Model version: `2021-06-03`
- Model file: `vendor/api-models-aws/models/internetmonitor/service/2021-06-03/internetmonitor-2021-06-03.json`
- SDK ID: `InternetMonitor`
- Endpoint prefix: `-`
- ARN namespace: `internetmonitor`
- CloudFormation name: `InternetMonitor`
- CloudTrail event source: `internetmonitor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (4), `Create` (1), `Delete` (1), `Start` (1), `Stop` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateMonitor`, `DeleteMonitor`, `StartQuery`, `StopQuery`, `TagResource`, `UntagResource`, `UpdateMonitor`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetHealthEvent`, `GetInternetEvent`, `GetMonitor`, `GetQueryResults`, `GetQueryStatus`, `ListHealthEvents`, `ListInternetEvents`, `ListMonitors`, `ListTagsForResource`, `StartQuery`, `StopQuery`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartQuery`, `StopQuery`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `HealthEventResource` | `EventId`, `MonitorName` | read: `GetHealthEvent`; list: `ListHealthEvents` | - | - |
| `InternetEventResource` | `EventId` | read: `GetInternetEvent`; list: `ListInternetEvents` | - | Represents an internet event for a specific city+AS location |
| `MonitorResource` | `MonitorName` | put: `CreateMonitor`; read: `GetMonitor`; update: `UpdateMonitor`; delete: `DeleteMonitor`; list: `ListMonitors` | `GetQueryResults`, `GetQueryStatus`, `StartQuery`, `StopQuery` | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
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
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists the tags for a resource. Tags are supported only for monitors in Amazon CloudWatch Internet Monitor. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Adds a tag to a resource. Tags are supported only for monitors in Amazon CloudWatch Internet Monitor. You can add a maximum of 50 tags in Internet Monitor. A minimum of one tag is required for this call. It returns a ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Removes a tag from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permission to perform this action. |
| `BadRequestException` | `structure` | message | A bad request was received. |
| `ConflictException` | `structure` | message | The requested resource is in use. |
| `InternalServerErrorException` | `structure` | message | There was an internal server error. |
| `InternalServerException` | `structure` | message | An internal error occurred. |
| `LimitExceededException` | `structure` | message | The request exceeded a service quota. |
| `NotFoundException` | `structure` | message | The request specifies something that doesn't exist. |
| `ResourceNotFoundException` | `structure` | message | The request specifies a resource that doesn't exist. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `TooManyRequestsException` | `structure` | message | There were too many requests. |
| `ValidationException` | `structure` | message | Invalid request. |
| `ListTagsForResourceInput` | `structure` | ResourceArn | - |
| `ListTagsForResourceOutput` | `structure` | Tags | - |
| `TagResourceInput` | `structure` | ResourceArn, Tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

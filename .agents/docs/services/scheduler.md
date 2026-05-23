# Amazon EventBridge Scheduler

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EventBridge Scheduler is a serverless scheduler that allows you to create, run, and manage tasks from one central, managed service. EventBridge Scheduler delivers your tasks reliably, with built-in mechanisms that adjust your schedules based on the availability of downstream targets. The following reference lists the available API actions, and data types for EventBridge Scheduler.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon EventBridge Scheduler workflows in the local mock. Key resources include `Schedule`, `ScheduleGroup`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Tag` operation families, including `ListScheduleGroups`, `ListSchedules`, `ListTagsForResource`, `CreateSchedule`, `CreateScheduleGroup`, `DeleteSchedule`.

## Service Identity and Protocol

- AWS model slug: `scheduler`
- AWS SDK for Rust slug: `scheduler`
- Model version: `2021-06-30`
- Model file: `vendor/api-models-aws/models/scheduler/service/2021-06-30/scheduler-2021-06-30.json`
- SDK ID: `Scheduler`
- Endpoint prefix: `-`
- ARN namespace: `scheduler`
- CloudFormation name: `Scheduler`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (3), `Create` (2), `Delete` (2), `Get` (2), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSchedule`, `CreateScheduleGroup`, `DeleteSchedule`, `DeleteScheduleGroup`, `TagResource`, `UntagResource`, `UpdateSchedule`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetSchedule`, `GetScheduleGroup`, `ListScheduleGroups`, `ListSchedules`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`, `SQS`, `EC2/VPC`, `ECS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Schedule` | `Name` | put: `CreateSchedule`; read: `GetSchedule`; update: `UpdateSchedule`; delete: `DeleteSchedule`; list: `ListSchedules` | - | - |
| `ScheduleGroup` | `Name` | put: `CreateScheduleGroup`; read: `GetScheduleGroup`; delete: `DeleteScheduleGroup`; list: `ListScheduleGroups` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/eventbridge/latest/userguide/using-eventbridge-scheduler.html

Research outcomes:
- EventBridge Scheduler invokes targets by assuming an execution role. API, CLI, and CloudFormation users must provide an existing role.
- Schedules can be one-time or recurring. One-time schedules use a date, 24-hour time, and timezone. Recurring schedules use cron or rate expressions.
- Recurring schedules can specify timezone, start date/time, and end date/time.
- Flexible time windows are either off or a predefined delivery window. A 15-minute window means the target runs within 15 minutes after the scheduled time.
- Schedule groups group schedules for tagging and management.
- Schedule state can be enabled or disabled.
- Retry policy supports maximum event age up to 24 hours and maximum retry attempts up to 185.
- Dead-letter queues can be disabled, same-account SQS queues, or cross-account SQS queues by ARN.
- A customer managed KMS key can encrypt target input at rest.

Parity implications:
- Model schedules, groups, target payloads, execution roles, timezone-aware schedule expressions, flexible windows, retry policy, DLQ configuration, KMS settings, and enabled state.
- Target invocation should be asynchronous and driven by schedule due time plus flexible-window placement.
- Retry, event-age expiry, and DLQ routing should be modelled as target-delivery semantics rather than metadata only.

## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags associated with the Scheduler resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified EventBridge Scheduler resource. You can only assign tags to schedule groups. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from the specified EventBridge Scheduler schedule group. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> TagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | Message | Updating or deleting the resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | Message | Unexpected error encountered while processing the request. |
| `ResourceNotFoundException` | `structure` | Message | The request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | Message | The request exceeds a service quota. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message | The input fails to satisfy the constraints specified by an AWS service. |
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

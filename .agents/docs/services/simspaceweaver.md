# AWS SimSpace Weaver

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

SimSpace Weaver (SimSpace Weaver) is a service that you can use to build and run large-scale spatial simulations in the Amazon Web Services Cloud. For example, you can create crowd simulations, large real-world environments, and immersive and interactive experiences. For more information about SimSpace Weaver, see the SimSpace Weaver User Guide . This API reference describes the API operations and data types that you can use to communicate directly with SimSpace Weaver. SimSpace Weaver also provides the SimSpace Weaver app SDK, which you use for app development.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-simspaceweaver/tests/scenario_test.rs`: run a full simulation pipeline through create/start/describe/list/stop/delete style operations.
- Backported from `scenario_test.rs`: tag and untag simulation resources.
- Scenario insight from EC2: add full state-machine walks for AWS SimSpace Weaver resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: model spatial simulation lifecycle, app and simulation metadata, snapshot/clock style state, long-running simulation transitions, and tag-based administration.

## Service Identity and Protocol

- AWS model slug: `simspaceweaver`
- AWS SDK for Rust slug: `simspaceweaver`
- Model version: `2022-10-28`
- Model file: `vendor/api-models-aws/models/simspaceweaver/service/2022-10-28/simspaceweaver-2022-10-28.json`
- SDK ID: `SimSpaceWeaver`
- Endpoint prefix: `simspaceweaver`
- ARN namespace: `simspaceweaver`
- CloudFormation name: `SimSpaceWeaver`
- CloudTrail event source: `simspaceweaver.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (3), `Start` (3), `Stop` (3), `Delete` (2), `Describe` (2), `Create` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSnapshot`, `DeleteApp`, `DeleteSimulation`, `StartApp`, `StartClock`, `StartSimulation`, `StopApp`, `StopClock`, `StopSimulation`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApp`, `DescribeSimulation`, `ListApps`, `ListSimulations`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartApp`, `StartClock`, `StartSimulation`, `StopApp`, `StopClock`, `StopSimulation`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Simulation` | `SimulationName` | create: `StartSimulation`; read: `DescribeSimulation`; update: `StopSimulation`; delete: `DeleteSimulation`; list: `ListSimulations` | `CreateSnapshot`, `DeleteApp`, `DescribeApp`, `ListApps`, `StartApp`, `StartClock`, `StopApp`, `StopClock` | Represents a simulation |
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
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Lists all tags on a SimSpace Weaver resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Adds tags to a SimSpace Weaver resource. For more information about tags, see Tagging Amazon Web Services resources in the Amazon Web Services General Reference . |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Removes tags from a SimSpace Weaver resource. For more information about tags, see Tagging Amazon Web Services resources in the Amazon Web Services General Reference . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | - |
| `ConflictException` | `structure` | Message | - |
| `InternalServerException` | `structure` | Message | - |
| `ResourceNotFoundException` | `structure` | Message | - |
| `ServiceQuotaExceededException` | `structure` | Message | - |
| `TooManyTagsException` | `structure` | Message | - |
| `ValidationException` | `structure` | Message | - |
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

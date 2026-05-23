# AWS CloudTrail Data Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The CloudTrail Data Service lets you ingest events into CloudTrail from any source in your hybrid environments, such as in-house or SaaS applications hosted on-premises or in the cloud, virtual machines, or containers. You can store, access, analyze, troubleshoot and take action on this data without maintaining multiple log aggregators and reporting tools. After you run `PutAuditEvents` to ingest your application activity into CloudTrail, you can use CloudTrail Lake to search, query, and analyze the data that is logged from your applications.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS CloudTrail Data Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Put` operation families, including `PutAuditEvents`.

## Service Identity and Protocol

- AWS model slug: `cloudtrail-data`
- AWS SDK for Rust slug: `cloudtraildata`
- Model version: `2021-08-11`
- Model file: `vendor/api-models-aws/models/cloudtrail-data/service/2021-08-11/cloudtrail-data-2021-08-11.json`
- SDK ID: `CloudTrail Data`
- Endpoint prefix: `cloudtrail-data`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutAuditEvents`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `STS`.

## Control-Plane / Data-Plane Coherence

- **Paired with `cloudtrail`.** This data plane ingests partner-sourced events into a CloudTrail **channel** that is created by the CloudTrail control plane ( `winterbaume-cloudtrail` ) via `CreateChannel`. `PutAuditEvents` requires a valid channel ARN; in real AWS the call fails with `ChannelARNInvalidException` if the channel does not exist.
- **Current Winterbaume status: divergent.** `winterbaume-cloudtraildata` does not depend on `winterbaume-cloudtrail`; it records events with whatever `channel_arn` it is given without checking that the channel exists.
- **What needs to change:** this crate should observe `winterbaume-cloudtrail`'s `channels` state and reject `PutAuditEvents` for unknown channels. The reverse direction ( reading the ingested events back from the control plane ) is not part of real AWS — events go to the channel's destinations ( S3, CloudWatch Logs ), not back to CloudTrail's API surface.

## Operation Groups

### Put

- Operations: `PutAuditEvents`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `PutAuditEvents` | `POST /PutAuditEvents` | - | `auditEvents`, `channelArn` | - | `PutAuditEventsResponse` | `ChannelInsufficientPermission`, `ChannelNotFound`, `ChannelUnsupportedSchema`, `DuplicatedAuditEventId`, `InvalidChannelARN`, `UnsupportedOperationException` | Ingests your application events into CloudTrail Lake. A required parameter, auditEvents , accepts the JSON records (also called payload ) of events that you want CloudTrail to ingest. You can add up to 100 of these e ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `PutAuditEvents` | - | `channelArn -> channelArn`, `externalId -> externalId` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ChannelInsufficientPermission` | `structure` | message | The caller's account ID must be the same as the channel owner's account ID. |
| `ChannelNotFound` | `structure` | message | The channel could not be found. |
| `ChannelUnsupportedSchema` | `structure` | message | The schema type of the event is not supported. |
| `DuplicatedAuditEventId` | `structure` | message | Two or more entries in the request have the same event ID. |
| `InvalidChannelARN` | `structure` | message | The specified channel ARN is not a valid channel ARN. |
| `UnsupportedOperationException` | `structure` | message | The operation requested is not supported in this region or account. |
| `PutAuditEventsRequest` | `structure` | auditEvents, channelArn, externalId | - |
| `PutAuditEventsResponse` | `structure` | successful, failed | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

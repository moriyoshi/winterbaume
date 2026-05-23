# AWS SSM-GUIConnect

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Systems Manager GUI Connect Systems Manager GUI Connect, a component of Fleet Manager, lets you connect to your Window Server-type Amazon Elastic Compute Cloud (Amazon EC2) instances using the Remote Desktop Protocol (RDP). GUI Connect, which is powered by Amazon DCV, provides you with secure connectivity to your Windows Server instances directly from the Systems Manager console. You can have up to four simultaneous connections in a single browser window. In the console, GUI Connect is also referred to as Fleet Manager Remote Desktop. This reference is intended to be used with the Amazon Web Services Systems Manager User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS SSM-GUIConnect by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS SSM-GUIConnect workflows in the local mock. Key resources include `Connection`, `ConnectionAccess`, `ConnectionPreferences`, `ConnectionsCollection`, `ModifyConnectionPreferences`.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Delete`, `Get`, `Update` operation families, including `DeleteConnectionRecordingPreferences`, `GetConnectionRecordingPreferences`, `UpdateConnectionRecordingPreferences`.

## Service Identity and Protocol

- AWS model slug: `ssm-guiconnect`
- AWS SDK for Rust slug: `ssmguiconnect`
- Model version: `2021-05-01`
- Model file: `vendor/api-models-aws/models/ssm-guiconnect/service/2021-05-01/ssm-guiconnect-2021-05-01.json`
- SDK ID: `SSM GuiConnect`
- Endpoint prefix: `-`
- ARN namespace: `ssm-guiconnect`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Get` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteConnectionRecordingPreferences`, `UpdateConnectionRecordingPreferences`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetConnectionRecordingPreferences`.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Connection` | `ConnectionArn` | - | - | - |
| `ConnectionAccess` | `ConnectionToken` | - | - | - |
| `ConnectionPreferences` | `AccountId` | - | - | - |
| `ConnectionsCollection` | `AccountId` | - | - | - |
| `ModifyConnectionPreferences` | - | - | - | - |
| `ModifyRecordingPreferences` | - | read: `GetConnectionRecordingPreferences`; delete: `DeleteConnectionRecordingPreferences` | - | - |
| `RecordingPreferences` | `AccountId` | create: `UpdateConnectionRecordingPreferences` | - | - |
## Operation Groups

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | An error occurred due to a conflict. |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure. |
| `ResourceNotFoundException` | `structure` | message | The resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | message | Your request exceeds a service quota. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an AWS service. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Control Tower

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Control Tower offers application programming interface (API) operations that support programmatic interaction with these types of resources: Controls DisableControl EnableControl GetEnabledControl GetControlOperation ListControlOperations ListEnabledControls ResetEnabledControl UpdateEnabledControl Landing zones CreateLandingZone DeleteLandingZone GetLandingZone GetLandingZoneOperation ListLandingZones ListLandingZoneOperations ResetLandingZone UpdateLandingZone Baselines DisableBaseline EnableBaseline GetBaseline GetBaselineOperation GetEnabledBaseline ListBaselines ListEnabledBaselines ResetEnabledBaseline UpdateEnabledBaseline Tagging ListTagsForResource TagResource UntagResource For more information about these types of resources, see the Amazon Web Services Control Tower User Guide . About control APIs These interfaces allow you to apply the Amazon Web Services library of pre-defined controls to your organizational units, programmatically. In Amazon Web Services Control Tower, the terms "control" and "guardrail" are synonyms. To call these APIs, you'll need to know: the `controlIdentifier` for the control--or guardrail--you are targeting. the ARN associated with the target organizational unit (OU), which we call the `targetIdentifier`.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Control Tower by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS Control Tower workflows in the local mock. Key resources include `BaselineOperationResource`, `BaselineResource`, `ControlOperationResource`, `EnabledBaselineResource`, `EnabledControlResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Reset`, `Update`, `Disable` operation families, including `GetBaseline`, `GetBaselineOperation`, `GetControlOperation`, `GetEnabledBaseline`, `ListBaselines`, `ListControlOperations`.

## Service Identity and Protocol

- AWS model slug: `controltower`
- AWS SDK for Rust slug: `controltower`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/controltower/service/2018-05-10/controltower-2018-05-10.json`
- SDK ID: `ControlTower`
- Endpoint prefix: `controltower`
- ARN namespace: `controltower`
- CloudFormation name: `ControlTower`
- CloudTrail event source: `controltower.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (7), `Reset` (3), `Update` (3), `Disable` (2), `Enable` (2), `Create` (1), `Delete` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateLandingZone`, `DeleteLandingZone`, `DisableBaseline`, `DisableControl`, `EnableBaseline`, `EnableControl`, `TagResource`, `UntagResource`, `UpdateEnabledBaseline`, `UpdateEnabledControl`, `UpdateLandingZone`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBaseline`, `GetBaselineOperation`, `GetControlOperation`, `GetEnabledBaseline`, `GetEnabledControl`, `GetLandingZone`, `GetLandingZoneOperation`, `ListBaselines`, `ListControlOperations`, `ListEnabledBaselines`, `ListEnabledControls`, `ListLandingZoneOperations`, `ListLandingZones`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 28 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BaselineOperationResource` | `operationIdentifier` | read: `GetBaselineOperation` | - | - |
| `BaselineResource` | `baselineIdentifier` | read: `GetBaseline`; list: `ListBaselines` | - | - |
| `ControlOperationResource` | `operationIdentifier` | read: `GetControlOperation`; list: `ListControlOperations` | - | - |
| `EnabledBaselineResource` | `enabledBaselineIdentifier` | create: `EnableBaseline`; read: `GetEnabledBaseline`; update: `UpdateEnabledBaseline`; delete: `DisableBaseline`; list: `ListEnabledBaselines` | `ResetEnabledBaseline` | - |
| `EnabledControlResource` | `enabledControlIdentifier` | create: `EnableControl`; read: `GetEnabledControl`; update: `UpdateEnabledControl`; list: `ListEnabledControls` | `ResetEnabledControl` | - |
| `LandingZoneOperationResource` | `operationIdentifier` | read: `GetLandingZoneOperation`; list: `ListLandingZoneOperations` | - | - |
| `LandingZoneResource` | `landingZoneIdentifier` | create: `CreateLandingZone`; read: `GetLandingZone`; update: `UpdateLandingZone`; delete: `DeleteLandingZone`; list: `ListLandingZones` | `ResetLandingZone` | - |
| `TaggingResource` | `tagKey` | list: `ListTagsForResource` | `TagResource`, `UntagResource` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/controltower/latest/userguide/what-is-control-tower.html
- https://docs.aws.amazon.com/controltower/latest/userguide/aws-multi-account-landing-zone.html
- https://docs.aws.amazon.com/controltower/latest/userguide/importing-existing.html

Research outcomes:
- Control Tower automates setup and governance for multi-account AWS environments through a landing zone.
- Landing zones use AWS Organizations, organisational units, shared accounts, controls, and guardrails.
- Controls can prevent, detect, or guide configuration across governed accounts and OUs.
- Account Factory provisions new accounts into governed OUs with standardised baseline configuration.
- Existing OUs can be registered so their accounts are enrolled into landing-zone governance.
- Control Tower monitors and reports drift from expected governed state.

Parity implications:
- Model landing zones, governed OUs, enrolled accounts, controls, control operations, account factory requests, drift status, and baseline resources separately.
- Enrolment and control enablement should be asynchronous and organisation-aware.
- Drift should be represented as detected state against expected baseline rather than an arbitrary flag.

## Operation Groups

### Disable

- Operations: `DisableControl`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DisableControl` | `POST /disable-control` | - | - | - | `DisableControlOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This API call turns off a control. It starts an asynchronous operation that deletes Amazon Web Services resources on the specified organizational unit and the accounts it contains. The resources will vary according t ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | Updating or deleting the resource can cause an inconsistent state. |
| `InternalServerException` | `structure` | message | An unexpected error occurred during processing of a request. |
| `ResourceNotFoundException` | `structure` | message | The request references a resource that does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | The request would cause a service quota to be exceeded. See Service quotas . |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input does not satisfy the constraints specified by an Amazon Web Services service. |
| `DisableControlInput` | `structure` | controlIdentifier, targetIdentifier, enabledControlIdentifier | - |
| `DisableControlOutput` | `structure` | operationIdentifier | - |
| `BaselineOperationStatus` | `enum` | SUCCEEDED, FAILED, IN_PROGRESS | - |
| `BaselineOperationType` | `enum` | ENABLE_BASELINE, DISABLE_BASELINE, UPDATE_ENABLED_BASELINE, RESET_ENABLED_BASELINE | - |
| `ControlOperationStatus` | `enum` | SUCCEEDED, FAILED, IN_PROGRESS | - |
| `ControlOperationType` | `enum` | ENABLE_CONTROL, DISABLE_CONTROL, UPDATE_ENABLED_CONTROL, RESET_ENABLED_CONTROL | - |
| `DriftStatus` | `enum` | DRIFTED, IN_SYNC, NOT_CHECKING, UNKNOWN | - |
| `EnabledBaselineDriftStatus` | `enum` | IN_SYNC, DRIFTED | - |
| `EnablementStatus` | `enum` | SUCCEEDED, FAILED, UNDER_CHANGE | - |
| `LandingZoneDriftStatus` | `enum` | DRIFTED, IN_SYNC | - |
| `LandingZoneOperationStatus` | `enum` | SUCCEEDED, FAILED, IN_PROGRESS | - |
| `LandingZoneOperationType` | `enum` | DELETE, CREATE, UPDATE, RESET | - |
| `LandingZoneStatus` | `enum` | ACTIVE, PROCESSING, FAILED | - |
| `RemediationType` | `enum` | INHERITANCE_DRIFT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

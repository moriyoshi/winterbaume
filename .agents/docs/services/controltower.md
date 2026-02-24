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

### Get

- Operations: `GetBaseline`, `GetBaselineOperation`, `GetControlOperation`, `GetEnabledBaseline`, `GetEnabledControl`, `GetLandingZone`, `GetLandingZoneOperation`
- Traits: `readonly` (7)
- Common required input members in this group: `baselineIdentifier`, `enabledBaselineIdentifier`, `enabledControlIdentifier`, `landingZoneIdentifier`, `operationIdentifier`

### List

- Operations: `ListBaselines`, `ListControlOperations`, `ListEnabledBaselines`, `ListEnabledControls`, `ListLandingZoneOperations`, `ListLandingZones`, `ListTagsForResource`
- Traits: `paginated` (6), `readonly` (7)
- Common required input members in this group: `resourceArn`

### Reset

- Operations: `ResetEnabledBaseline`, `ResetEnabledControl`, `ResetLandingZone`
- Common required input members in this group: `enabledBaselineIdentifier`, `enabledControlIdentifier`, `landingZoneIdentifier`

### Update

- Operations: `UpdateEnabledBaseline`, `UpdateEnabledControl`, `UpdateLandingZone`
- Common required input members in this group: `baselineVersion`, `enabledBaselineIdentifier`, `enabledControlIdentifier`, `landingZoneIdentifier`, `parameters`, `version`

### Disable

- Operations: `DisableBaseline`, `DisableControl`
- Traits: `idempotent` (1)
- Common required input members in this group: `enabledBaselineIdentifier`

### Enable

- Operations: `EnableBaseline`, `EnableControl`
- Common required input members in this group: `baselineIdentifier`, `baselineVersion`, `controlIdentifier`, `targetIdentifier`

### Create

- Operations: `CreateLandingZone`
- Common required input members in this group: `version`

### Delete

- Operations: `DeleteLandingZone`
- Traits: `idempotent` (1)
- Common required input members in this group: `landingZoneIdentifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateLandingZone` | `POST /create-landingzone` | - | `version` | - | `CreateLandingZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a new landing zone. This API call starts an asynchronous operation that creates and configures a landing zone, based on the parameters specified in the manifest JSON file. |
| `DeleteLandingZone` | `POST /delete-landingzone` | `idempotent` | `landingZoneIdentifier` | - | `DeleteLandingZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Decommissions a landing zone. This API call starts an asynchronous operation that deletes Amazon Web Services Control Tower resources deployed in accounts managed by Amazon Web Services Control Tower. |
| `DisableBaseline` | `POST /disable-baseline` | `idempotent` | `enabledBaselineIdentifier` | - | `DisableBaselineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disable an `EnabledBaseline` resource on the specified Target. This API starts an asynchronous operation to remove all resources deployed as part of the baseline enablement. |
| `DisableControl` | `POST /disable-control` | - | - | - | `DisableControlOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This API call turns off a control. It starts an asynchronous operation that deletes Amazon Web Services resources on the specified organizational unit and the accounts it contains. |
| `EnableBaseline` | `POST /enable-baseline` | - | `baselineIdentifier`, `baselineVersion`, `targetIdentifier` | - | `EnableBaselineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enable (apply) a `Baseline` to a Target. This API starts an asynchronous operation to deploy resources specified by the `Baseline` to the specified Target. |
| `EnableControl` | `POST /enable-control` | - | `controlIdentifier`, `targetIdentifier` | - | `EnableControlOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This API call activates a control. It starts an asynchronous operation that creates Amazon Web Services resources on the specified organizational unit and the accounts it contains. |
| `GetBaseline` | `POST /get-baseline` | `readonly` | `baselineIdentifier` | - | `GetBaselineOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve details about an existing `Baseline` resource by specifying its identifier. For usage examples, see the Amazon Web Services Control Tower User Guide . |
| `GetBaselineOperation` | `POST /get-baseline-operation` | `readonly` | `operationIdentifier` | - | `GetBaselineOperationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the details of an asynchronous baseline operation, as initiated by any of these APIs: `EnableBaseline`, `DisableBaseline`, `UpdateEnabledBaseline`, `ResetEnabledBaseline`. A status message is displayed in case of operation failure. |
| `GetControlOperation` | `POST /get-control-operation` | `readonly` | `operationIdentifier` | - | `GetControlOperationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the status of a particular `EnableControl` or `DisableControl` operation. Displays a message in case of error. |
| `GetEnabledBaseline` | `POST /get-enabled-baseline` | `readonly` | `enabledBaselineIdentifier` | - | `GetEnabledBaselineOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve details of an `EnabledBaseline` resource by specifying its identifier. |
| `GetEnabledControl` | `POST /get-enabled-control` | `readonly` | `enabledControlIdentifier` | - | `GetEnabledControlOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about an enabled control. For usage examples, see the Controls Reference Guide . |
| `GetLandingZone` | `POST /get-landingzone` | `readonly` | `landingZoneIdentifier` | - | `GetLandingZoneOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about the landing zone. Displays a message in case of error. |
| `GetLandingZoneOperation` | `POST /get-landingzone-operation` | `readonly` | `operationIdentifier` | - | `GetLandingZoneOperationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the status of the specified landing zone operation. Details for an operation are available for 90 days. |
| `ListBaselines` | `POST /list-baselines` | `readonly`, `paginated` | - | - | `ListBaselinesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a summary list of all available baselines. For usage examples, see the Amazon Web Services Control Tower User Guide . |
| `ListControlOperations` | `POST /list-control-operations` | `readonly`, `paginated` | - | - | `ListControlOperationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provides a list of operations in progress or queued. For usage examples, see ListControlOperation examples. |
| `ListEnabledBaselines` | `POST /list-enabled-baselines` | `readonly`, `paginated` | - | - | `ListEnabledBaselinesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of summaries describing `EnabledBaseline` resources. You can filter the list by the corresponding `Baseline` or `Target` of the `EnabledBaseline` resources. |
| `ListEnabledControls` | `POST /list-enabled-controls` | `readonly`, `paginated` | - | - | `ListEnabledControlsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the controls enabled by Amazon Web Services Control Tower on the specified organizational unit and the accounts it contains. For usage examples, see the Controls Reference Guide . |
| `ListLandingZoneOperations` | `POST /list-landingzone-operations` | `readonly`, `paginated` | - | - | `ListLandingZoneOperationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all landing zone operations from the past 90 days. Results are sorted by time, with the most recent operation first. |
| `ListLandingZones` | `POST /list-landingzones` | `readonly`, `paginated` | - | - | `ListLandingZonesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the landing zone ARN for the landing zone deployed in your managed account. This API also creates an ARN for existing accounts that do not yet have a landing zone ARN. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags associated with the resource. For usage examples, see the Controls Reference Guide . |
| `ResetEnabledBaseline` | `POST /reset-enabled-baseline` | - | `enabledBaselineIdentifier` | - | `ResetEnabledBaselineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Re-enables an `EnabledBaseline` resource. For example, this API can re-apply the existing `Baseline` after a new member account is moved to the target OU. |
| `ResetEnabledControl` | `POST /reset-enabled-control` | - | `enabledControlIdentifier` | - | `ResetEnabledControlOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Resets an enabled control. Does not work for controls implemented with SCPs. |
| `ResetLandingZone` | `POST /reset-landingzone` | - | `landingZoneIdentifier` | - | `ResetLandingZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This API call resets a landing zone. It starts an asynchronous operation that resets the landing zone to the parameters specified in the original configuration, which you specified in the manifest file. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Applies tags to a resource. For usage examples, see the Controls Reference Guide . |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from a resource. For usage examples, see the Controls Reference Guide . |
| `UpdateEnabledBaseline` | `POST /update-enabled-baseline` | - | `baselineVersion`, `enabledBaselineIdentifier` | - | `UpdateEnabledBaselineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an `EnabledBaseline` resource's applied parameters or version. For usage examples, see the Amazon Web Services Control Tower User Guide . |
| `UpdateEnabledControl` | `POST /update-enabled-control` | - | `enabledControlIdentifier`, `parameters` | - | `UpdateEnabledControlOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an already enabled control. If the enabled control shows an `EnablementStatus` of SUCCEEDED, supply parameters that are different from the currently configured parameters. |
| `UpdateLandingZone` | `POST /update-landingzone` | - | `landingZoneIdentifier`, `version` | - | `UpdateLandingZoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This API call updates the landing zone. It starts an asynchronous operation that updates the landing zone based on the new landing zone version, or on the changed parameters specified in the updated manifest file. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An unexpected error occurred during processing of a request. |
| `ValidationException` | `structure` | `message` | The input does not satisfy the constraints specified by an Amazon Web Services service. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied due to request throttling. |
| `ResourceNotFoundException` | `structure` | `message` | The request references a resource that does not exist. |
| `ConflictException` | `structure` | `message` | Updating or deleting the resource can cause an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request would cause a service quota to be exceeded. |
| `CreateLandingZoneInput` | `structure` | `manifest`, `remediationTypes`, `tags`, `version` | - |
| `CreateLandingZoneOutput` | `structure` | `arn`, `operationIdentifier` | - |
| `DeleteLandingZoneInput` | `structure` | `landingZoneIdentifier` | - |
| `DeleteLandingZoneOutput` | `structure` | `operationIdentifier` | - |
| `DisableBaselineInput` | `structure` | `enabledBaselineIdentifier` | - |
| `DisableBaselineOutput` | `structure` | `operationIdentifier` | - |
| `DisableControlInput` | `structure` | `controlIdentifier`, `enabledControlIdentifier`, `targetIdentifier` | - |
| `DisableControlOutput` | `structure` | `operationIdentifier` | - |
| `EnableBaselineInput` | `structure` | `baselineIdentifier`, `baselineVersion`, `parameters`, `tags`, `targetIdentifier` | - |
| `EnableBaselineOutput` | `structure` | `arn`, `operationIdentifier` | - |
| `EnableControlInput` | `structure` | `controlIdentifier`, `parameters`, `tags`, `targetIdentifier` | - |
| `EnableControlOutput` | `structure` | `arn`, `operationIdentifier` | - |
| `GetBaselineInput` | `structure` | `baselineIdentifier` | - |
| `GetBaselineOutput` | `structure` | `arn`, `description`, `name` | - |
| `GetBaselineOperationInput` | `structure` | `operationIdentifier` | - |
| `GetBaselineOperationOutput` | `structure` | `baselineOperation` | - |
| `GetControlOperationInput` | `structure` | `operationIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Systems Manager Incident Manager Contacts

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Systems Manager Incident Manager is an incident management console designed to help users mitigate and recover from incidents affecting their Amazon Web Services-hosted applications. An incident is any unplanned interruption or reduction in quality of services. Incident Manager increases incident resolution by notifying responders of impact, highlighting relevant troubleshooting data, and providing collaboration tools to get services back up and running. To achieve the primary goal of reducing the time-to-resolution of critical incidents, Incident Manager automates response plans and enables responder team escalation.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Systems Manager Incident Manager Contacts resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Systems Manager Incident Manager Contacts workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListContactChannels`, `ListContacts`, `ListEngagements`, `ListPageReceipts`, `GetContact`, `GetContactChannel`.

## Service Identity and Protocol

- AWS model slug: `ssm-contacts`
- AWS SDK for Rust slug: `ssmcontacts`
- Model version: `2021-05-03`
- Model file: `vendor/api-models-aws/models/ssm-contacts/service/2021-05-03/ssm-contacts-2021-05-03.json`
- SDK ID: `SSM Contacts`
- Endpoint prefix: `ssm-contacts`
- ARN namespace: `ssm-contacts`
- CloudFormation name: `SSMContacts`
- CloudTrail event source: `ssmcontacts.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Get` (5), `Create` (4), `Delete` (4), `Update` (3), `Describe` (2), `Accept` (1), `Activate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptPage`, `CreateContact`, `CreateContactChannel`, `CreateRotation`, `CreateRotationOverride`, `DeleteContact`, `DeleteContactChannel`, `DeleteRotation`, `DeleteRotationOverride`, `PutContactPolicy`, `StartEngagement`, `StopEngagement`, `TagResource`, `UntagResource`, `UpdateContact`, `UpdateContactChannel`, `UpdateRotation`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeEngagement`, `DescribePage`, `GetContact`, `GetContactChannel`, `GetContactPolicy`, `GetRotation`, `GetRotationOverride`, `ListContactChannels`, `ListContacts`, `ListEngagements`, `ListPageReceipts`, `ListPageResolutions`, `ListPagesByContact`, `ListPagesByEngagement`, `ListPreviewRotationShifts`, `ListRotationOverrides`, `ListRotationShifts`, `ListRotations`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartEngagement`, `StopEngagement`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`.

## Operation Groups

### List

- Operations: `ListContactChannels`, `ListContacts`, `ListEngagements`, `ListPageReceipts`, `ListPageResolutions`, `ListPagesByContact`, `ListPagesByEngagement`, `ListPreviewRotationShifts`, `ListRotationOverrides`, `ListRotationShifts`, `ListRotations`, `ListTagsForResource`
- Traits: `paginated` (11)
- Common required input members in this group: `ContactId`, `EndTime`, `EngagementId`, `Members`, `PageId`, `Recurrence`, `ResourceARN`, `RotationId`, `StartTime`, `TimeZoneId`

### Get

- Operations: `GetContact`, `GetContactChannel`, `GetContactPolicy`, `GetRotation`, `GetRotationOverride`
- Common required input members in this group: `ContactArn`, `ContactChannelId`, `ContactId`, `RotationId`, `RotationOverrideId`

### Create

- Operations: `CreateContact`, `CreateContactChannel`, `CreateRotation`, `CreateRotationOverride`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `Alias`, `ContactId`, `ContactIds`, `DeliveryAddress`, `EndTime`, `Name`, `NewContactIds`, `Plan`, `Recurrence`, `RotationId`, `StartTime`, `TimeZoneId`, `Type`

### Delete

- Operations: `DeleteContact`, `DeleteContactChannel`, `DeleteRotation`, `DeleteRotationOverride`
- Common required input members in this group: `ContactChannelId`, `ContactId`, `RotationId`, `RotationOverrideId`

### Update

- Operations: `UpdateContact`, `UpdateContactChannel`, `UpdateRotation`
- Common required input members in this group: `ContactChannelId`, `ContactId`, `Recurrence`, `RotationId`

### Describe

- Operations: `DescribeEngagement`, `DescribePage`
- Common required input members in this group: `EngagementId`, `PageId`

### Accept

- Operations: `AcceptPage`
- Common required input members in this group: `AcceptCode`, `AcceptType`, `PageId`

### Activate

- Operations: `ActivateContactChannel`
- Common required input members in this group: `ActivationCode`, `ContactChannelId`

### Deactivate

- Operations: `DeactivateContactChannel`
- Common required input members in this group: `ContactChannelId`

### Put

- Operations: `PutContactPolicy`
- Common required input members in this group: `ContactArn`, `Policy`

### Send

- Operations: `SendActivationCode`
- Common required input members in this group: `ContactChannelId`

### Start

- Operations: `StartEngagement`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ContactId`, `Content`, `Sender`, `Subject`

### Stop

- Operations: `StopEngagement`
- Common required input members in this group: `EngagementId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptPage` | - | - | `AcceptCode`, `AcceptType`, `PageId` | - | `AcceptPageResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Used to acknowledge an engagement to a contact channel during an incident. |
| `ActivateContactChannel` | - | - | `ActivationCode`, `ContactChannelId` | - | `ActivateContactChannelResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Activates a contact's contact channel. Incident Manager can't engage a contact until the contact channel has been activated. |
| `CreateContact` | - | `idempotency-token` | `Alias`, `Plan`, `Type` | `IdempotencyToken` | `CreateContactResult` | `AccessDeniedException`, `ConflictException`, `DataEncryptionException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Contacts are either the contacts that Incident Manager engages during an incident or the escalation plans that Incident Manager uses to engage contacts in phases during an incident. |
| `CreateContactChannel` | - | `idempotency-token` | `ContactId`, `DeliveryAddress`, `Name`, `Type` | `IdempotencyToken` | `CreateContactChannelResult` | `AccessDeniedException`, `ConflictException`, `DataEncryptionException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | A contact channel is the method that Incident Manager uses to engage your contact. |
| `CreateRotation` | - | - | `ContactIds`, `Name`, `Recurrence`, `TimeZoneId` | - | `CreateRotationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a rotation in an on-call schedule. |
| `CreateRotationOverride` | - | - | `EndTime`, `NewContactIds`, `RotationId`, `StartTime` | - | `CreateRotationOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an override for a rotation in an on-call schedule. |
| `DeactivateContactChannel` | - | - | `ContactChannelId` | - | `DeactivateContactChannelResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | To no longer receive Incident Manager engagements to a contact channel, you can deactivate the channel. |
| `DeleteContact` | - | - | `ContactId` | - | `DeleteContactResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | To remove a contact from Incident Manager, you can delete the contact. However, deleting a contact does not remove it from escalation plans and related response plans. |
| `DeleteContactChannel` | - | - | `ContactChannelId` | - | `DeleteContactChannelResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | To stop receiving engagements on a contact channel, you can delete the channel from a contact. Deleting the contact channel does not remove it from the contact's engagement plan, but the stage that includes the channel will be ignored. |
| `DeleteRotation` | - | - | `RotationId` | - | `DeleteRotationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a rotation from the system. If a rotation belongs to more than one on-call schedule, this operation deletes it from all of them. |
| `DeleteRotationOverride` | - | - | `RotationId`, `RotationOverrideId` | - | `DeleteRotationOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing override for an on-call rotation. |
| `DescribeEngagement` | - | - | `EngagementId` | - | `DescribeEngagementResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Incident Manager uses engagements to engage contacts and escalation plans during an incident. Use this command to describe the engagement that occurred during an incident. |
| `DescribePage` | - | - | `PageId` | - | `DescribePageResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists details of the engagement to a contact channel. |
| `GetContact` | - | - | `ContactId` | - | `GetContactResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified contact or escalation plan. |
| `GetContactChannel` | - | - | `ContactChannelId` | - | `GetContactChannelResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List details about a specific contact channel. |
| `GetContactPolicy` | - | - | `ContactArn` | - | `GetContactPolicyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the resource policies attached to the specified contact or escalation plan. |
| `GetRotation` | - | - | `RotationId` | - | `GetRotationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an on-call rotation. |
| `GetRotationOverride` | - | - | `RotationId`, `RotationOverrideId` | - | `GetRotationOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an override to an on-call rotation. |
| `ListContactChannels` | - | `paginated` | `ContactId` | - | `ListContactChannelsResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all contact channels for the specified contact. |
| `ListContacts` | - | `paginated` | - | - | `ListContactsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all contacts and escalation plans in Incident Manager. |
| `ListEngagements` | - | `paginated` | - | - | `ListEngagementsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all engagements that have happened in an incident. |
| `ListPageReceipts` | - | `paginated` | `PageId` | - | `ListPageReceiptsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all of the engagements to contact channels that have been acknowledged. |
| `ListPageResolutions` | - | `paginated` | `PageId` | - | `ListPageResolutionsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the resolution path of an engagement. For example, the escalation plan engaged in an incident might target an on-call schedule that includes several contacts in a rotation, but just one contact on-call when the incident starts. |
| `ListPagesByContact` | - | `paginated` | `ContactId` | - | `ListPagesByContactResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the engagements to a contact's contact channels. |
| `ListPagesByEngagement` | - | `paginated` | `EngagementId` | - | `ListPagesByEngagementResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the engagements to contact channels that occurred by engaging a contact. |
| `ListPreviewRotationShifts` | - | `paginated` | `EndTime`, `Members`, `Recurrence`, `TimeZoneId` | - | `ListPreviewRotationShiftsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of shifts based on rotation configuration parameters. The Incident Manager primarily uses this operation to populate the Preview calendar. |
| `ListRotationOverrides` | - | `paginated` | `EndTime`, `RotationId`, `StartTime` | - | `ListRotationOverridesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of overrides currently specified for an on-call rotation. |
| `ListRotationShifts` | - | `paginated` | `EndTime`, `RotationId` | - | `ListRotationShiftsResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of shifts generated by an existing rotation in the system. |
| `ListRotations` | - | `paginated` | - | - | `ListRotationsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of on-call rotations. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags of a contact, escalation plan, rotation, or on-call schedule. |
| `PutContactPolicy` | - | - | `ContactArn`, `Policy` | - | `PutContactPolicyResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a resource policy to the specified contact or escalation plan. The resource policy is used to share the contact or escalation plan using Resource Access Manager (RAM). |
| `SendActivationCode` | - | - | `ContactChannelId` | - | `SendActivationCodeResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sends an activation code to a contact channel. The contact can use this code to activate the contact channel in the console or with the `ActivateChannel` operation. |
| `StartEngagement` | - | `idempotency-token` | `ContactId`, `Content`, `Sender`, `Subject` | `IdempotencyToken` | `StartEngagementResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts an engagement to a contact or escalation plan. The engagement engages each contact specified in the incident. |
| `StopEngagement` | - | - | `EngagementId` | - | `StopEngagementResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops an engagement before it finishes the final stage of the escalation plan or engagement plan. Further contacts aren't engaged. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tags a contact or escalation plan. You can tag only contacts and escalation plans in the first region of your replication set. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from the specified resource. |
| `UpdateContact` | - | - | `ContactId` | - | `UpdateContactResult` | `AccessDeniedException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the contact or escalation plan specified. |
| `UpdateContactChannel` | - | - | `ContactChannelId` | - | `UpdateContactChannelResult` | `AccessDeniedException`, `ConflictException`, `DataEncryptionException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a contact's contact channel. |
| `UpdateRotation` | - | - | `Recurrence`, `RotationId` | - | `UpdateRotationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the information specified for an on-call rotation. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient access to perform this operation. |
| `InternalServerException` | `structure` | `Message`, `RetryAfterSeconds` | Unexpected error occurred while processing the request. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `RetryAfterSeconds`, `ServiceCode` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Request references a resource that doesn't exist. |
| `DataEncryptionException` | `structure` | `Message` | The operation failed to due an encryption key error. |
| `ConflictException` | `structure` | `DependentEntities`, `Message`, `ResourceId`, `ResourceType` | Updating or deleting a resource causes an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | Request would cause a service quota to be exceeded. |
| `AcceptPageRequest` | `structure` | `AcceptCode`, `AcceptCodeValidation`, `AcceptType`, `ContactChannelId`, `Note`, `PageId` | - |
| `AcceptPageResult` | `structure` | - | - |
| `ActivateContactChannelRequest` | `structure` | `ActivationCode`, `ContactChannelId` | - |
| `ActivateContactChannelResult` | `structure` | - | - |
| `CreateContactRequest` | `structure` | `Alias`, `DisplayName`, `IdempotencyToken`, `Plan`, `Tags`, `Type` | - |
| `CreateContactResult` | `structure` | `ContactArn` | - |
| `CreateContactChannelRequest` | `structure` | `ContactId`, `DeferActivation`, `DeliveryAddress`, `IdempotencyToken`, `Name`, `Type` | - |
| `CreateContactChannelResult` | `structure` | `ContactChannelArn` | - |
| `CreateRotationRequest` | `structure` | `ContactIds`, `IdempotencyToken`, `Name`, `Recurrence`, `StartTime`, `Tags`, `TimeZoneId` | - |
| `CreateRotationResult` | `structure` | `RotationArn` | - |
| `CreateRotationOverrideRequest` | `structure` | `EndTime`, `IdempotencyToken`, `NewContactIds`, `RotationId`, `StartTime` | - |
| `CreateRotationOverrideResult` | `structure` | `RotationOverrideId` | - |
| `DeactivateContactChannelRequest` | `structure` | `ContactChannelId` | - |
| `DeactivateContactChannelResult` | `structure` | - | - |
| `DeleteContactRequest` | `structure` | `ContactId` | - |
| `DeleteContactResult` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

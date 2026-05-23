# Amazon Pinpoint SMS Voice V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the End User Messaging SMS, version 2 API Reference . This guide provides information about End User Messaging SMS, version 2 API resources, including supported HTTP methods, parameters, and schemas. Amazon Pinpoint is an Amazon Web Services service that you can use to engage with your recipients across multiple messaging channels. The End User Messaging SMS, version 2 API provides programmatic access to options that are unique to the SMS and voice channels. End User Messaging SMS, version 2 resources such as phone numbers, sender IDs, and opt-out lists can be used by the Amazon Pinpoint API.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Pinpoint SMS Voice V2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Pinpoint SMS Voice V2 by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Pinpoint SMS Voice V2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Pinpoint SMS Voice V2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Describe`, `Create`, `Set`, `Put` operation families, including `DeleteAccountDefaultProtectConfiguration`, `DeleteConfigurationSet`, `DeleteDefaultMessageType`, `DeleteDefaultSenderId`, `DescribeAccountAttributes`, `DescribeAccountLimits`.

## Service Identity and Protocol

- AWS model slug: `pinpoint-sms-voice-v2`
- AWS SDK for Rust slug: `pinpointsmsvoicev2`
- Model version: `2022-03-31`
- Model file: `vendor/api-models-aws/models/pinpoint-sms-voice-v2/service/2022-03-31/pinpoint-sms-voice-v2-2022-03-31.json`
- SDK ID: `Pinpoint SMS Voice V2`
- Endpoint prefix: `sms-voice`
- ARN namespace: `sms-voice`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (19), `Describe` (19), `Create` (10), `Set` (7), `Put` (6), `Update` (6), `List` (4), `Send` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateOriginationIdentity`, `AssociateProtectConfiguration`, `CreateConfigurationSet`, `CreateEventDestination`, `CreateOptOutList`, `CreatePool`, `CreateProtectConfiguration`, `CreateRegistration`, `CreateRegistrationAssociation`, `CreateRegistrationAttachment`, `CreateRegistrationVersion`, `CreateVerifiedDestinationNumber`, `DeleteAccountDefaultProtectConfiguration`, `DeleteConfigurationSet`, `DeleteDefaultMessageType`, `DeleteDefaultSenderId`, `DeleteEventDestination`, `DeleteKeyword`, `DeleteMediaMessageSpendLimitOverride`, `DeleteOptOutList`, ... (+35).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAttributes`, `DescribeAccountLimits`, `DescribeConfigurationSets`, `DescribeKeywords`, `DescribeOptOutLists`, `DescribeOptedOutNumbers`, `DescribePhoneNumbers`, `DescribePools`, `DescribeProtectConfigurations`, `DescribeRegistrationAttachments`, `DescribeRegistrationFieldDefinitions`, `DescribeRegistrationFieldValues`, `DescribeRegistrationSectionDefinitions`, `DescribeRegistrationTypeDefinitions`, `DescribeRegistrationVersions`, `DescribeRegistrations`, `DescribeSenderIds`, `DescribeSpendLimits`, `DescribeVerifiedDestinationNumbers`, `GetProtectConfigurationCountryRuleSet`, ... (+5).
- Pagination is modelled for 22 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 91 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Delete

- Operations: `DeleteAccountDefaultProtectConfiguration`, `DeleteConfigurationSet`, `DeleteDefaultMessageType`, `DeleteDefaultSenderId`, `DeleteEventDestination`, `DeleteKeyword`, `DeleteMediaMessageSpendLimitOverride`, `DeleteOptOutList`, `DeleteOptedOutNumber`, `DeletePool`, `DeleteProtectConfiguration`, `DeleteProtectConfigurationRuleSetNumberOverride`, `DeleteRegistration`, `DeleteRegistrationAttachment`, `DeleteRegistrationFieldValue`, `DeleteResourcePolicy`, `DeleteTextMessageSpendLimitOverride`, `DeleteVerifiedDestinationNumber`, `DeleteVoiceMessageSpendLimitOverride`
- Common required input members in this group: `ConfigurationSetName`, `DestinationPhoneNumber`, `EventDestinationName`, `FieldPath`, `Keyword`, `OptOutListName`, `OptedOutNumber`, `OriginationIdentity`, `PoolId`, `ProtectConfigurationId`, `RegistrationAttachmentId`, `RegistrationId`, `ResourceArn`, `VerifiedDestinationNumberId`

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeAccountLimits`, `DescribeConfigurationSets`, `DescribeKeywords`, `DescribeOptOutLists`, `DescribeOptedOutNumbers`, `DescribePhoneNumbers`, `DescribePools`, `DescribeProtectConfigurations`, `DescribeRegistrationAttachments`, `DescribeRegistrationFieldDefinitions`, `DescribeRegistrationFieldValues`, `DescribeRegistrationSectionDefinitions`, `DescribeRegistrationTypeDefinitions`, `DescribeRegistrationVersions`, `DescribeRegistrations`, `DescribeSenderIds`, `DescribeSpendLimits`, `DescribeVerifiedDestinationNumbers`
- Traits: `paginated` (19)
- Common required input members in this group: `OptOutListName`, `OriginationIdentity`, `RegistrationId`, `RegistrationType`

### Create

- Operations: `CreateConfigurationSet`, `CreateEventDestination`, `CreateOptOutList`, `CreatePool`, `CreateProtectConfiguration`, `CreateRegistration`, `CreateRegistrationAssociation`, `CreateRegistrationAttachment`, `CreateRegistrationVersion`, `CreateVerifiedDestinationNumber`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `ConfigurationSetName`, `DestinationPhoneNumber`, `EventDestinationName`, `IsoCountryCode`, `MatchingEventTypes`, `MessageType`, `OptOutListName`, `OriginationIdentity`, `RegistrationId`, `RegistrationType`, `ResourceId`

### Set

- Operations: `SetAccountDefaultProtectConfiguration`, `SetDefaultMessageFeedbackEnabled`, `SetDefaultMessageType`, `SetDefaultSenderId`, `SetMediaMessageSpendLimitOverride`, `SetTextMessageSpendLimitOverride`, `SetVoiceMessageSpendLimitOverride`
- Common required input members in this group: `ConfigurationSetName`, `MessageFeedbackEnabled`, `MessageType`, `MonthlyLimit`, `ProtectConfigurationId`, `SenderId`

### Put

- Operations: `PutKeyword`, `PutMessageFeedback`, `PutOptedOutNumber`, `PutProtectConfigurationRuleSetNumberOverride`, `PutRegistrationFieldValue`, `PutResourcePolicy`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Action`, `DestinationPhoneNumber`, `FieldPath`, `Keyword`, `KeywordMessage`, `MessageFeedbackStatus`, `MessageId`, `OptOutListName`, `OptedOutNumber`, `OriginationIdentity`, `Policy`, `ProtectConfigurationId`, `RegistrationId`, `ResourceArn`

### Update

- Operations: `UpdateEventDestination`, `UpdatePhoneNumber`, `UpdatePool`, `UpdateProtectConfiguration`, `UpdateProtectConfigurationCountryRuleSet`, `UpdateSenderId`
- Common required input members in this group: `ConfigurationSetName`, `CountryRuleSetUpdates`, `EventDestinationName`, `IsoCountryCode`, `NumberCapability`, `PhoneNumberId`, `PoolId`, `ProtectConfigurationId`, `SenderId`

### List

- Operations: `ListPoolOriginationIdentities`, `ListProtectConfigurationRuleSetNumberOverrides`, `ListRegistrationAssociations`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `PoolId`, `ProtectConfigurationId`, `RegistrationId`, `ResourceArn`

### Send

- Operations: `SendDestinationNumberVerificationCode`, `SendMediaMessage`, `SendTextMessage`, `SendVoiceMessage`
- Common required input members in this group: `DestinationPhoneNumber`, `OriginationIdentity`, `VerificationChannel`, `VerifiedDestinationNumberId`

### Associate

- Operations: `AssociateOriginationIdentity`, `AssociateProtectConfiguration`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ConfigurationSetName`, `IsoCountryCode`, `OriginationIdentity`, `PoolId`, `ProtectConfigurationId`

### Disassociate

- Operations: `DisassociateOriginationIdentity`, `DisassociateProtectConfiguration`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ConfigurationSetName`, `IsoCountryCode`, `OriginationIdentity`, `PoolId`, `ProtectConfigurationId`

### Get

- Operations: `GetProtectConfigurationCountryRuleSet`, `GetResourcePolicy`
- Common required input members in this group: `NumberCapability`, `ProtectConfigurationId`, `ResourceArn`

### Release

- Operations: `ReleasePhoneNumber`, `ReleaseSenderId`
- Common required input members in this group: `IsoCountryCode`, `PhoneNumberId`, `SenderId`

### Request

- Operations: `RequestPhoneNumber`, `RequestSenderId`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `IsoCountryCode`, `MessageType`, `NumberCapabilities`, `NumberType`, `SenderId`

### Carrier

- Operations: `CarrierLookup`
- Common required input members in this group: `PhoneNumber`

### Discard

- Operations: `DiscardRegistrationVersion`
- Common required input members in this group: `RegistrationId`

### Submit

- Operations: `SubmitRegistrationVersion`
- Common required input members in this group: `RegistrationId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Verify

- Operations: `VerifyDestinationNumber`
- Common required input members in this group: `VerificationCode`, `VerifiedDestinationNumberId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateOriginationIdentity` | - | `idempotency-token` | `IsoCountryCode`, `OriginationIdentity`, `PoolId` | `ClientToken` | `AssociateOriginationIdentityResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the specified origination identity with a pool. If the origination identity is a phone number and is already associated with another pool, an error is returned. |
| `AssociateProtectConfiguration` | - | - | `ConfigurationSetName`, `ProtectConfigurationId` | - | `AssociateProtectConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a protect configuration with a configuration set. This replaces the configuration sets current protect configuration. |
| `CarrierLookup` | - | - | `PhoneNumber` | - | `CarrierLookupResult` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Returns information about a destination phone number, including whether the number type and whether it is valid, the carrier, and more. |
| `CreateConfigurationSet` | - | `idempotency-token` | `ConfigurationSetName` | `ClientToken` | `CreateConfigurationSetResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new configuration set. After you create the configuration set, you can add one or more event destinations to it. |
| `CreateEventDestination` | - | `idempotency-token` | `ConfigurationSetName`, `EventDestinationName`, `MatchingEventTypes` | `ClientToken` | `CreateEventDestinationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new event destination in a configuration set. An event destination is a location where you send message events. |
| `CreateOptOutList` | - | `idempotency-token` | `OptOutListName` | `ClientToken` | `CreateOptOutListResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new opt-out list. If the opt-out list name already exists, an error is returned. |
| `CreatePool` | - | `idempotency-token` | `IsoCountryCode`, `MessageType`, `OriginationIdentity` | `ClientToken` | `CreatePoolResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new pool and associates the specified origination identity to the pool. A pool can include one or more phone numbers and SenderIds that are associated with your Amazon Web Services account. |
| `CreateProtectConfiguration` | - | `idempotency-token` | - | `ClientToken` | `CreateProtectConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new protect configuration. By default all country rule sets for each capability are set to `ALLOW`. |
| `CreateRegistration` | - | `idempotency-token` | `RegistrationType` | `ClientToken` | `CreateRegistrationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new registration based on the RegistrationType field. |
| `CreateRegistrationAssociation` | - | - | `RegistrationId`, `ResourceId` | - | `CreateRegistrationAssociationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associate the registration with an origination identity such as a phone number or sender ID. |
| `CreateRegistrationAttachment` | - | `idempotency-token` | - | `ClientToken` | `CreateRegistrationAttachmentResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new registration attachment to use for uploading a file or a URL to a file. The maximum file size is 500KB and valid file extensions are PDF, JPEG and PNG. |
| `CreateRegistrationVersion` | - | - | `RegistrationId` | - | `CreateRegistrationVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new version of the registration and increase the VersionNumber . The previous version of the registration becomes read-only. |
| `CreateVerifiedDestinationNumber` | - | `idempotency-token` | `DestinationPhoneNumber` | `ClientToken` | `CreateVerifiedDestinationNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | You can only send messages to verified destination numbers when your account is in the sandbox. You can add up to 10 verified destination numbers. |
| `DeleteAccountDefaultProtectConfiguration` | - | - | - | - | `DeleteAccountDefaultProtectConfigurationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the current account default protect configuration. |
| `DeleteConfigurationSet` | - | - | `ConfigurationSetName` | - | `DeleteConfigurationSetResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing configuration set. A configuration set is a set of rules that you apply to voice and SMS messages that you send. |
| `DeleteDefaultMessageType` | - | - | `ConfigurationSetName` | - | `DeleteDefaultMessageTypeResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing default message type on a configuration set. A message type is a type of messages that you plan to send. |
| `DeleteDefaultSenderId` | - | - | `ConfigurationSetName` | - | `DeleteDefaultSenderIdResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing default sender ID on a configuration set. A default sender ID is the identity that appears on recipients' devices when they receive SMS messages. |
| `DeleteEventDestination` | - | - | `ConfigurationSetName`, `EventDestinationName` | - | `DeleteEventDestinationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing event destination. An event destination is a location where you send response information about the messages that you send. |
| `DeleteKeyword` | - | - | `Keyword`, `OriginationIdentity` | - | `DeleteKeywordResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing keyword from an origination phone number or pool. A keyword is a word that you can search for on a particular phone number or pool. |
| `DeleteMediaMessageSpendLimitOverride` | - | - | - | - | `DeleteMediaMessageSpendLimitOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an account-level monthly spending limit override for sending multimedia messages (MMS). Deleting a spend limit override will set the `EnforcedLimit` to equal the `MaxLimit`, which is controlled by Amazon Web Services. |
| `DeleteOptOutList` | - | - | `OptOutListName` | - | `DeleteOptOutListResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing opt-out list. All opted out phone numbers in the opt-out list are deleted. |
| `DeleteOptedOutNumber` | - | - | `OptOutListName`, `OptedOutNumber` | - | `DeleteOptedOutNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing opted out destination phone number from the specified opt-out list. Each destination phone number can only be deleted once every 30 days. |
| `DeletePool` | - | - | `PoolId` | - | `DeletePoolResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing pool. Deleting a pool disassociates all origination identities from that pool. |
| `DeleteProtectConfiguration` | - | - | `ProtectConfigurationId` | - | `DeleteProtectConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently delete the protect configuration. The protect configuration must have deletion protection disabled and must not be associated as the account default protect configuration or associated with a configuration set. |
| `DeleteProtectConfigurationRuleSetNumberOverride` | - | - | `DestinationPhoneNumber`, `ProtectConfigurationId` | - | `DeleteProtectConfigurationRuleSetNumberOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently delete the protect configuration rule set number override. |
| `DeleteRegistration` | - | - | `RegistrationId` | - | `DeleteRegistrationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently delete an existing registration from your account. |
| `DeleteRegistrationAttachment` | - | - | `RegistrationAttachmentId` | - | `DeleteRegistrationAttachmentResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently delete the specified registration attachment. |
| `DeleteRegistrationFieldValue` | - | - | `FieldPath`, `RegistrationId` | - | `DeleteRegistrationFieldValueResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete the value in a registration form field. |
| `DeleteResourcePolicy` | - | - | `ResourceArn` | - | `DeleteResourcePolicyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource-based policy document attached to the End User Messaging SMS resource. A shared resource can be a Pool, Opt-out list, Sender Id, or Phone number. |
| `DeleteTextMessageSpendLimitOverride` | - | - | - | - | `DeleteTextMessageSpendLimitOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an account-level monthly spending limit override for sending text messages. Deleting a spend limit override will set the `EnforcedLimit` to equal the `MaxLimit`, which is controlled by Amazon Web Services. |
| `DeleteVerifiedDestinationNumber` | - | - | `VerifiedDestinationNumberId` | - | `DeleteVerifiedDestinationNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a verified destination phone number. |
| `DeleteVoiceMessageSpendLimitOverride` | - | - | - | - | `DeleteVoiceMessageSpendLimitOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an account level monthly spend limit override for sending voice messages. Deleting a spend limit override sets the `EnforcedLimit` equal to the `MaxLimit`, which is controlled by Amazon Web Services. |
| `DescribeAccountAttributes` | - | `paginated` | - | - | `DescribeAccountAttributesResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Describes attributes of your Amazon Web Services account. The supported account attributes include account tier, which indicates whether your account is in the sandbox or production environment. |
| `DescribeAccountLimits` | - | `paginated` | - | - | `DescribeAccountLimitsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Describes the current End User Messaging SMS SMS Voice V2 resource quotas for your account. The description for a quota includes the quota name, current usage toward that quota, and the quota's maximum value. |
| `DescribeConfigurationSets` | - | `paginated` | - | - | `DescribeConfigurationSetsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified configuration sets or all in your account. If you specify configuration set names, the output includes information for only the specified configuration sets. |
| `DescribeKeywords` | - | `paginated` | `OriginationIdentity` | - | `DescribeKeywordsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified keywords or all keywords on your origination phone number or pool. A keyword is a word that you can search for on a particular phone number or pool. |
| `DescribeOptOutLists` | - | `paginated` | - | - | `DescribeOptOutListsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified opt-out list or all opt-out lists in your account. If you specify opt-out list names, the output includes information for only the specified opt-out lists. |
| `DescribeOptedOutNumbers` | - | `paginated` | `OptOutListName` | - | `DescribeOptedOutNumbersResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified opted out destination numbers or all opted out destination numbers in an opt-out list. If you specify opted out numbers, the output includes information for only the specified opted out numbers. |
| `DescribePhoneNumbers` | - | `paginated` | - | - | `DescribePhoneNumbersResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified origination phone number, or all the phone numbers in your account. If you specify phone number IDs, the output includes information for only the specified phone numbers. |
| `DescribePools` | - | `paginated` | - | - | `DescribePoolsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified pools or all pools associated with your Amazon Web Services account. If you specify pool IDs, the output includes information for only the specified pools. |
| `DescribeProtectConfigurations` | - | `paginated` | - | - | `DescribeProtectConfigurationsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the protect configurations that match any of filters. If a filter isn’t provided then all protect configurations are returned. |
| `DescribeRegistrationAttachments` | - | `paginated` | - | - | `DescribeRegistrationAttachmentsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registration attachments or all registration attachments associated with your Amazon Web Services account. |
| `DescribeRegistrationFieldDefinitions` | - | `paginated` | `RegistrationType` | - | `DescribeRegistrationFieldDefinitionsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registration type field definitions. You can use DescribeRegistrationFieldDefinitions to view the requirements for creating, filling out, and submitting each registration type. |
| `DescribeRegistrationFieldValues` | - | `paginated` | `RegistrationId` | - | `DescribeRegistrationFieldValuesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registration field values. |
| `DescribeRegistrationSectionDefinitions` | - | `paginated` | `RegistrationType` | - | `DescribeRegistrationSectionDefinitionsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registration section definitions. You can use DescribeRegistrationSectionDefinitions to view the requirements for creating, filling out, and submitting each registration type. |
| `DescribeRegistrationTypeDefinitions` | - | `paginated` | - | - | `DescribeRegistrationTypeDefinitionsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registration type definitions. You can use DescribeRegistrationTypeDefinitions to view the requirements for creating, filling out, and submitting each registration type. |
| `DescribeRegistrationVersions` | - | `paginated` | `RegistrationId` | - | `DescribeRegistrationVersionsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registration version. |
| `DescribeRegistrations` | - | `paginated` | - | - | `DescribeRegistrationsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified registrations. |
| `DescribeSenderIds` | - | `paginated` | - | - | `DescribeSenderIdsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the specified SenderIds or all SenderIds associated with your Amazon Web Services account. If you specify SenderIds, the output includes information for only the specified SenderIds. |
| `DescribeSpendLimits` | - | `paginated` | - | - | `DescribeSpendLimitsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Describes the current monthly spend limits for sending voice and text messages. When you establish an Amazon Web Services account, the account has initial monthly spend limit in a given Region. |
| `DescribeVerifiedDestinationNumbers` | - | `paginated` | - | - | `DescribeVerifiedDestinationNumbersResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the specified verified destination numbers. |
| `DisassociateOriginationIdentity` | - | `idempotency-token` | `IsoCountryCode`, `OriginationIdentity`, `PoolId` | `ClientToken` | `DisassociateOriginationIdentityResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified origination identity from an existing pool. If the origination identity isn't associated with the specified pool, an error is returned. |
| `DisassociateProtectConfiguration` | - | - | `ConfigurationSetName`, `ProtectConfigurationId` | - | `DisassociateProtectConfigurationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociate a protect configuration from a configuration set. |
| `DiscardRegistrationVersion` | - | - | `RegistrationId` | - | `DiscardRegistrationVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Discard the current version of the registration. |
| `GetProtectConfigurationCountryRuleSet` | - | - | `NumberCapability`, `ProtectConfigurationId` | - | `GetProtectConfigurationCountryRuleSetResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve the CountryRuleSet for the specified NumberCapability from a protect configuration. |
| `GetResourcePolicy` | - | - | `ResourceArn` | - | `GetResourcePolicyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the JSON text of the resource-based policy document attached to the End User Messaging SMS resource. A shared resource can be a Pool, Opt-out list, Sender Id, or Phone number. |
| `ListPoolOriginationIdentities` | - | `paginated` | `PoolId` | - | `ListPoolOriginationIdentitiesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all associated origination identities in your pool. If you specify filters, the output includes information for only those origination identities that meet the filter criteria. |
| `ListProtectConfigurationRuleSetNumberOverrides` | - | `paginated` | `ProtectConfigurationId` | - | `ListProtectConfigurationRuleSetNumberOverridesResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve all of the protect configuration rule set number overrides that match the filters. |
| `ListRegistrationAssociations` | - | `paginated` | `RegistrationId` | - | `ListRegistrationAssociationsResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve all of the origination identities that are associated with a registration. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags associated with a resource. |
| `PutKeyword` | - | - | `Keyword`, `KeywordMessage`, `OriginationIdentity` | - | `PutKeywordResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates or updates a keyword configuration on an origination phone number or pool. A keyword is a word that you can search for on a particular phone number or pool. |
| `PutMessageFeedback` | - | - | `MessageFeedbackStatus`, `MessageId` | - | `PutMessageFeedbackResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Set the MessageFeedbackStatus as `RECEIVED` or `FAILED` for the passed in MessageId. If you use message feedback then you must update message feedback record. |
| `PutOptedOutNumber` | - | - | `OptOutListName`, `OptedOutNumber` | - | `PutOptedOutNumberResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an opted out destination phone number in the opt-out list. If the destination phone number isn't valid or if the specified opt-out list doesn't exist, an error is returned. |
| `PutProtectConfigurationRuleSetNumberOverride` | - | `idempotency-token` | `Action`, `DestinationPhoneNumber`, `ProtectConfigurationId` | `ClientToken` | `PutProtectConfigurationRuleSetNumberOverrideResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create or update a phone number rule override and associate it with a protect configuration. |
| `PutRegistrationFieldValue` | - | - | `FieldPath`, `RegistrationId` | - | `PutRegistrationFieldValueResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates a field value for a registration. |
| `PutResourcePolicy` | - | - | `Policy`, `ResourceArn` | - | `PutResourcePolicyResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a resource-based policy to a End User Messaging SMS resource(phone number, sender Id, phone poll, or opt-out list) that is used for sharing the resource. A shared resource can be a Pool, Opt-out list, Sender Id, or Phone number. |
| `ReleasePhoneNumber` | - | - | `PhoneNumberId` | - | `ReleasePhoneNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Releases an existing origination phone number in your account. Once released, a phone number is no longer available for sending messages. |
| `ReleaseSenderId` | - | - | `IsoCountryCode`, `SenderId` | - | `ReleaseSenderIdResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Releases an existing sender ID in your account. |
| `RequestPhoneNumber` | - | `idempotency-token` | `IsoCountryCode`, `MessageType`, `NumberCapabilities`, `NumberType` | `ClientToken` | `RequestPhoneNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Request an origination phone number for use in your account. For more information on phone number request see Request a phone number in the End User Messaging SMS User Guide . |
| `RequestSenderId` | - | `idempotency-token` | `IsoCountryCode`, `SenderId` | `ClientToken` | `RequestSenderIdResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Request a new sender ID that doesn't require registration. |
| `SendDestinationNumberVerificationCode` | - | - | `VerificationChannel`, `VerifiedDestinationNumberId` | - | `SendDestinationNumberVerificationCodeResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Before you can send test messages to a verified destination phone number you need to opt-in the verified destination phone number. Creates a new text message with a verification code and send it to a verified destination phone number. |
| `SendMediaMessage` | - | - | `DestinationPhoneNumber`, `OriginationIdentity` | - | `SendMediaMessageResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new multimedia message (MMS) and sends it to a recipient's phone number. |
| `SendTextMessage` | - | - | `DestinationPhoneNumber` | - | `SendTextMessageResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new text message and sends it to a recipient's phone number. SendTextMessage only sends an SMS message to one recipient each time it is invoked. |
| `SendVoiceMessage` | - | - | `DestinationPhoneNumber`, `OriginationIdentity` | - | `SendVoiceMessageResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Allows you to send a request that sends a voice message. This operation uses Amazon Polly to convert a text script into a voice message. |
| `SetAccountDefaultProtectConfiguration` | - | - | `ProtectConfigurationId` | - | `SetAccountDefaultProtectConfigurationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Set a protect configuration as your account default. You can only have one account default protect configuration at a time. |
| `SetDefaultMessageFeedbackEnabled` | - | - | `ConfigurationSetName`, `MessageFeedbackEnabled` | - | `SetDefaultMessageFeedbackEnabledResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets a configuration set's default for message feedback. |
| `SetDefaultMessageType` | - | - | `ConfigurationSetName`, `MessageType` | - | `SetDefaultMessageTypeResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets the default message type on a configuration set. Choose the category of SMS messages that you plan to send from this account. |
| `SetDefaultSenderId` | - | - | `ConfigurationSetName`, `SenderId` | - | `SetDefaultSenderIdResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets default sender ID on a configuration set. When sending a text message to a destination country that supports sender IDs, the default sender ID on the configuration set specified will be used if no dedicated origination phone numbers or registered sender... |
| `SetMediaMessageSpendLimitOverride` | - | - | `MonthlyLimit` | - | `SetMediaMessageSpendLimitOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sets an account level monthly spend limit override for sending MMS messages. The requested spend limit must be less than or equal to the `MaxLimit`, which is set by Amazon Web Services. |
| `SetTextMessageSpendLimitOverride` | - | - | `MonthlyLimit` | - | `SetTextMessageSpendLimitOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sets an account level monthly spend limit override for sending text messages. The requested spend limit must be less than or equal to the `MaxLimit`, which is set by Amazon Web Services. |
| `SetVoiceMessageSpendLimitOverride` | - | - | `MonthlyLimit` | - | `SetVoiceMessageSpendLimitOverrideResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sets an account level monthly spend limit override for sending voice messages. The requested spend limit must be less than or equal to the `MaxLimit`, which is set by Amazon Web Services. |
| `SubmitRegistrationVersion` | - | - | `RegistrationId` | - | `SubmitRegistrationVersionResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Submit the specified registration for review and approval. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds or overwrites only the specified tags for the specified resource. When you specify an existing tag key, the value is overwritten with the new value. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association of the specified tags from a resource. For more information on tags see Tags in the End User Messaging SMS User Guide . |
| `UpdateEventDestination` | - | - | `ConfigurationSetName`, `EventDestinationName` | - | `UpdateEventDestinationResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing event destination in a configuration set. You can update the IAM role ARN for CloudWatch Logs and Firehose. |
| `UpdatePhoneNumber` | - | - | `PhoneNumberId` | - | `UpdatePhoneNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing origination phone number. You can update the opt-out list, enable or disable two-way messaging, change the TwoWayChannelArn, enable or disable self-managed opt-outs, and enable or disable deletion protection. |
| `UpdatePool` | - | - | `PoolId` | - | `UpdatePoolResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing pool. You can update the opt-out list, enable or disable two-way messaging, change the `TwoWayChannelArn`, enable or disable self-managed opt-outs, enable or disable deletion protection, and enable or disable shared... |
| `UpdateProtectConfiguration` | - | - | `ProtectConfigurationId` | - | `UpdateProtectConfigurationResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the setting for an existing protect configuration. |
| `UpdateProtectConfigurationCountryRuleSet` | - | - | `CountryRuleSetUpdates`, `NumberCapability`, `ProtectConfigurationId` | - | `UpdateProtectConfigurationCountryRuleSetResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a country rule set to `ALLOW`, `BLOCK`, `MONITOR`, or `FILTER` messages to be sent to the specified destination counties. You can update one or multiple countries at a time. |
| `UpdateSenderId` | - | - | `IsoCountryCode`, `SenderId` | - | `UpdateSenderIdResult` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration of an existing sender ID. |
| `VerifyDestinationNumber` | - | - | `VerificationCode`, `VerifiedDestinationNumberId` | - | `VerifyDestinationNumberResult` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use the verification code that was received by the verified destination phone number to opt-in the verified destination phone number to receive more messages. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message`, `Reason` | The request was denied because you don't have sufficient permissions to access the resource. |
| `InternalServerException` | `structure` | `Message`, `RequestId` | The API encountered an unexpected error and couldn't complete the request. |
| `ThrottlingException` | `structure` | `Message` | An error that occurred because too many requests were sent during a certain amount of time. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | A validation exception for a field. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | A requested resource couldn't be found. |
| `ConflictException` | `structure` | `Message`, `Reason`, `ResourceId`, `ResourceType` | Your request has conflicting operations. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `Reason` | The request would cause a service quota to be exceeded. |
| `AssociateOriginationIdentityRequest` | `structure` | `ClientToken`, `IsoCountryCode`, `OriginationIdentity`, `PoolId` | - |
| `AssociateOriginationIdentityResult` | `structure` | `IsoCountryCode`, `OriginationIdentity`, `OriginationIdentityArn`, `PoolArn`, `PoolId` | - |
| `AssociateProtectConfigurationRequest` | `structure` | `ConfigurationSetName`, `ProtectConfigurationId` | - |
| `AssociateProtectConfigurationResult` | `structure` | `ConfigurationSetArn`, `ConfigurationSetName`, `ProtectConfigurationArn`, `ProtectConfigurationId` | - |
| `CarrierLookupRequest` | `structure` | `PhoneNumber` | - |
| `CarrierLookupResult` | `structure` | `Carrier`, `Country`, `DialingCountryCode`, `E164PhoneNumber`, `IsoCountryCode`, `MCC`, `MNC`, `PhoneNumberType` | - |
| `CreateConfigurationSetRequest` | `structure` | `ClientToken`, `ConfigurationSetName`, `Tags` | - |
| `CreateConfigurationSetResult` | `structure` | `ConfigurationSetArn`, `ConfigurationSetName`, `CreatedTimestamp`, `Tags` | - |
| `CreateEventDestinationRequest` | `structure` | `ClientToken`, `CloudWatchLogsDestination`, `ConfigurationSetName`, `EventDestinationName`, `KinesisFirehoseDestination`, `MatchingEventTypes`, `SnsDestination` | - |
| `CreateEventDestinationResult` | `structure` | `ConfigurationSetArn`, `ConfigurationSetName`, `EventDestination` | - |
| `CreateOptOutListRequest` | `structure` | `ClientToken`, `OptOutListName`, `Tags` | - |
| `CreateOptOutListResult` | `structure` | `CreatedTimestamp`, `OptOutListArn`, `OptOutListName`, `Tags` | - |
| `CreatePoolRequest` | `structure` | `ClientToken`, `DeletionProtectionEnabled`, `IsoCountryCode`, `MessageType`, `OriginationIdentity`, `Tags` | - |
| `CreatePoolResult` | `structure` | `CreatedTimestamp`, `DeletionProtectionEnabled`, `MessageType`, `OptOutListName`, `PoolArn`, `PoolId`, `SelfManagedOptOutsEnabled`, `SharedRoutesEnabled`, `Status`, `Tags`, `TwoWayChannelArn`, `TwoWayChannelRole`, ... (+1) | - |
| `CreateProtectConfigurationRequest` | `structure` | `ClientToken`, `DeletionProtectionEnabled`, `Tags` | - |
| `CreateProtectConfigurationResult` | `structure` | `AccountDefault`, `CreatedTimestamp`, `DeletionProtectionEnabled`, `ProtectConfigurationArn`, `ProtectConfigurationId`, `Tags` | - |
| `CreateRegistrationRequest` | `structure` | `ClientToken`, `RegistrationType`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

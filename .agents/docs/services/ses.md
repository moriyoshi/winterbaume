# Amazon Simple Email Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Simple Email Service This document contains reference information for the Amazon Simple Email Service (Amazon SES) API, version 2010-12-01. This document is best used in conjunction with the Amazon SES Developer Guide. For a list of Amazon SES endpoints to use in service requests, see Regions and Amazon SES in the Amazon SES Developer Guide. This documentation contains reference information related to the following: Amazon SES API Actions Amazon SES API Data Types Common Parameters Common Errors

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon Simple Email Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon Simple Email Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Get`, `Create`, `List`, `Update` operation families, including `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`, `DeleteConfigurationSetTrackingOptions`, `DeleteCustomVerificationEmailTemplate`, `GetAccountSendingEnabled`, `GetCustomVerificationEmailTemplate`.

## Service Identity and Protocol

- AWS model slug: `ses`
- AWS SDK for Rust slug: `ses`
- Model version: `2010-12-01`
- Model file: `vendor/api-models-aws/models/ses/service/2010-12-01/ses-2010-12-01.json`
- SDK ID: `SES`
- Endpoint prefix: `email`
- ARN namespace: `ses`
- CloudFormation name: `SES`
- CloudTrail event source: `ses.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (11), `Get` (10), `Create` (8), `List` (8), `Update` (8), `Set` (7), `Send` (6), `Describe` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConfigurationSet`, `CreateConfigurationSetEventDestination`, `CreateConfigurationSetTrackingOptions`, `CreateCustomVerificationEmailTemplate`, `CreateReceiptFilter`, `CreateReceiptRule`, `CreateReceiptRuleSet`, `CreateTemplate`, `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`, `DeleteConfigurationSetTrackingOptions`, `DeleteCustomVerificationEmailTemplate`, `DeleteIdentity`, `DeleteIdentityPolicy`, `DeleteReceiptFilter`, `DeleteReceiptRule`, `DeleteReceiptRuleSet`, `DeleteTemplate`, `DeleteVerifiedEmailAddress`, `PutConfigurationSetDeliveryOptions`, ... (+16).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeActiveReceiptRuleSet`, `DescribeConfigurationSet`, `DescribeReceiptRule`, `DescribeReceiptRuleSet`, `GetAccountSendingEnabled`, `GetCustomVerificationEmailTemplate`, `GetIdentityDkimAttributes`, `GetIdentityMailFromDomainAttributes`, `GetIdentityNotificationAttributes`, `GetIdentityPolicies`, `GetIdentityVerificationAttributes`, `GetSendQuota`, `GetSendStatistics`, `GetTemplate`, `ListConfigurationSets`, `ListCustomVerificationEmailTemplates`, `ListIdentities`, `ListIdentityPolicies`, `ListReceiptFilters`, `ListReceiptRuleSets`, ... (+2).
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 38 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SNS`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## v1/v2 State Coherence

- **Paired with `sesv2` (same SDK slug `ses`, same endpoint prefix `email`, same ARN namespace `ses`).** In real AWS, SES v1 ( `awsQuery`, this crate ) and SESv2 ( `restJson1` ) are two protocol surfaces over the same backend. Resources created via one API are observable and mutable via the other.
- AWS docs explicitly note shared management: `aws sesv2 list-configuration-sets` lists configuration sets created via the v1 `CreateConfigurationSet`, and verified identities created via v1 `VerifyEmailIdentity` / `VerifyDomainIdentity` are returned by v2 `ListEmailIdentities`. Templates, suppression list entries, dedicated IP pools, and account-level sending settings are likewise shared.
- **Current Winterbaume status: divergent.** `winterbaume-ses` owns its own `SesV1State` ( `Identity`, `ConfigurationSet`, `Template`, `ReceiptRuleSet`, `SentEmail` ); `winterbaume-sesv2` owns its own `SesState` ( `EmailIdentity`, `ConfigurationSet`, `EmailTemplate`, suppression list, dedicated IP pools, etc. ). Neither crate depends on the other, so resources created via v1 are invisible to v2 and vice versa — a behavioural gap relative to real AWS.
- **What needs to change:** the shared resource families ( identities, configuration sets, templates, suppression list, account-level settings ) must live in a single state owner — typically a shared inner struct exposed by one crate and consumed by the other, or an extracted common state crate. Receipt rule sets are v1-only ( no v2 equivalent ), and the v2-only families ( contact lists, import/export jobs, multi-region endpoints, tenants, deliverability test reports, reputation entities ) stay where they are. Cross-API integration tests that exercise create-via-v1 / read-via-v2 ( and the reverse ) are the right regression guard.

## Operation Groups

### Delete

- Operations: `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`, `DeleteConfigurationSetTrackingOptions`, `DeleteCustomVerificationEmailTemplate`, `DeleteIdentity`, `DeleteIdentityPolicy`, `DeleteReceiptFilter`, `DeleteReceiptRule`, `DeleteReceiptRuleSet`, `DeleteTemplate`, `DeleteVerifiedEmailAddress`
- Common required input members in this group: `ConfigurationSetName`, `TemplateName`, `Identity`, `RuleSetName`

### Get

- Operations: `GetAccountSendingEnabled`, `GetCustomVerificationEmailTemplate`, `GetIdentityDkimAttributes`, `GetIdentityMailFromDomainAttributes`, `GetIdentityNotificationAttributes`, `GetIdentityPolicies`, `GetIdentityVerificationAttributes`, `GetSendQuota`, `GetSendStatistics`, `GetTemplate`
- Common required input members in this group: `TemplateName`, `Identities`

### Create

- Operations: `CreateConfigurationSet`, `CreateConfigurationSetEventDestination`, `CreateConfigurationSetTrackingOptions`, `CreateCustomVerificationEmailTemplate`, `CreateReceiptFilter`, `CreateReceiptRule`, `CreateReceiptRuleSet`, `CreateTemplate`
- Common required input members in this group: `ConfigurationSetName`, `RuleSetName`

### List

- Operations: `ListConfigurationSets`, `ListCustomVerificationEmailTemplates`, `ListIdentities`, `ListIdentityPolicies`, `ListReceiptFilters`, `ListReceiptRuleSets`, `ListTemplates`, `ListVerifiedEmailAddresses`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateAccountSendingEnabled`, `UpdateConfigurationSetEventDestination`, `UpdateConfigurationSetReputationMetricsEnabled`, `UpdateConfigurationSetSendingEnabled`, `UpdateConfigurationSetTrackingOptions`, `UpdateCustomVerificationEmailTemplate`, `UpdateReceiptRule`, `UpdateTemplate`
- Common required input members in this group: `ConfigurationSetName`, `Enabled`

### Set

- Operations: `SetActiveReceiptRuleSet`, `SetIdentityDkimEnabled`, `SetIdentityFeedbackForwardingEnabled`, `SetIdentityHeadersInNotificationsEnabled`, `SetIdentityMailFromDomain`, `SetIdentityNotificationTopic`, `SetReceiptRulePosition`
- Common required input members in this group: `Identity`, `NotificationType`

### Send

- Operations: `SendBounce`, `SendBulkTemplatedEmail`, `SendCustomVerificationEmail`, `SendEmail`, `SendRawEmail`, `SendTemplatedEmail`
- Common required input members in this group: `Source`, `Template`, `Destination`

### Describe

- Operations: `DescribeActiveReceiptRuleSet`, `DescribeConfigurationSet`, `DescribeReceiptRule`, `DescribeReceiptRuleSet`
- Common required input members in this group: `RuleSetName`

### Verify

- Operations: `VerifyDomainDkim`, `VerifyDomainIdentity`, `VerifyEmailAddress`, `VerifyEmailIdentity`
- Common required input members in this group: `Domain`, `EmailAddress`

### Put

- Operations: `PutConfigurationSetDeliveryOptions`, `PutIdentityPolicy`
- Common required input members in this group: -

### Clone

- Operations: `CloneReceiptRuleSet`
- Common required input members in this group: -

### Reorder

- Operations: `ReorderReceiptRuleSet`
- Common required input members in this group: -

### Test

- Operations: `TestRenderTemplate`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CloneReceiptRuleSet` | `-` | - | `RuleSetName`, `OriginalRuleSetName` | - | `CloneReceiptRuleSetResponse` | `AlreadyExistsException`, `LimitExceededException`, `RuleSetDoesNotExistException` | Creates a receipt rule set by cloning an existing one. All receipt rules and configurations are copied to the new receipt rule set and are completely independent of the source rule set. For information about setting ... |
| `CreateConfigurationSet` | `-` | - | `ConfigurationSet` | - | `CreateConfigurationSetResponse` | `ConfigurationSetAlreadyExistsException`, `InvalidConfigurationSetException`, `LimitExceededException` | Creates a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the Amazon SES Developer Guide . You can execute this operation no more ... |
| `CreateConfigurationSetEventDestination` | `-` | - | `ConfigurationSetName`, `EventDestination` | - | `CreateConfigurationSetEventDestinationResponse` | `ConfigurationSetDoesNotExistException`, `EventDestinationAlreadyExistsException`, `InvalidCloudWatchDestinationException`, `InvalidFirehoseDestinationException`, `InvalidSNSDestinationException`, `LimitExceededException` | Creates a configuration set event destination. When you create or update an event destination, you must provide one, and only one, destination. The destination can be CloudWatch, Amazon Kinesis Firehose, or Amazon Si ... |
| `CreateConfigurationSetTrackingOptions` | `-` | - | `ConfigurationSetName`, `TrackingOptions` | - | `CreateConfigurationSetTrackingOptionsResponse` | `ConfigurationSetDoesNotExistException`, `InvalidTrackingOptionsException`, `TrackingOptionsAlreadyExistsException` | Creates an association between a configuration set and a custom domain for open and click event tracking. By default, images and links used for tracking open and click events are hosted on domains operated by Amazon ... |
| `CreateCustomVerificationEmailTemplate` | `-` | - | `TemplateName`, `FromEmailAddress`, `TemplateSubject`, `TemplateContent`, `SuccessRedirectionURL`, `FailureRedirectionURL` | - | `Unit` | `CustomVerificationEmailInvalidContentException`, `CustomVerificationEmailTemplateAlreadyExistsException`, `FromEmailAddressNotVerifiedException`, `LimitExceededException` | Creates a new custom verification email template. For more information about custom verification email templates, see Using Custom Verification Email Templates in the Amazon SES Developer Guide . You can execute this ... |
| `CreateReceiptFilter` | `-` | - | `Filter` | - | `CreateReceiptFilterResponse` | `AlreadyExistsException`, `LimitExceededException` | Creates a new IP address filter. For information about setting up IP address filters, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `CreateReceiptRule` | `-` | - | `RuleSetName`, `Rule` | - | `CreateReceiptRuleResponse` | `AlreadyExistsException`, `InvalidLambdaFunctionException`, `InvalidS3ConfigurationException`, `InvalidSnsTopicException`, `LimitExceededException`, `RuleDoesNotExistException`, `RuleSetDoesNotExistException` | Creates a receipt rule. For information about setting up receipt rules, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `CreateReceiptRuleSet` | `-` | - | `RuleSetName` | - | `CreateReceiptRuleSetResponse` | `AlreadyExistsException`, `LimitExceededException` | Creates an empty receipt rule set. For information about setting up receipt rule sets, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `CreateTemplate` | `-` | - | `Template` | - | `CreateTemplateResponse` | `AlreadyExistsException`, `InvalidTemplateException`, `LimitExceededException` | Creates an email template. Email templates enable you to send personalized email to one or more destinations in a single operation. For more information, see the Amazon SES Developer Guide . You can execute this oper ... |
| `DeleteConfigurationSet` | `-` | - | `ConfigurationSetName` | - | `DeleteConfigurationSetResponse` | `ConfigurationSetDoesNotExistException` | Deletes a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration sets, see the Amazon SES Developer Guide . You can execute this operation no more ... |
| `DeleteConfigurationSetEventDestination` | `-` | - | `ConfigurationSetName`, `EventDestinationName` | - | `DeleteConfigurationSetEventDestinationResponse` | `ConfigurationSetDoesNotExistException`, `EventDestinationDoesNotExistException` | Deletes a configuration set event destination. Configuration set event destinations are associated with configuration sets, which enable you to publish email sending events. For information about using configuration ... |
| `DeleteConfigurationSetTrackingOptions` | `-` | - | `ConfigurationSetName` | - | `DeleteConfigurationSetTrackingOptionsResponse` | `ConfigurationSetDoesNotExistException`, `TrackingOptionsDoesNotExistException` | Deletes an association between a configuration set and a custom domain for open and click event tracking. By default, images and links used for tracking open and click events are hosted on domains operated by Amazon ... |
| `DeleteCustomVerificationEmailTemplate` | `-` | - | `TemplateName` | - | `Unit` | - | Deletes an existing custom verification email template. For more information about custom verification email templates, see Using Custom Verification Email Templates in the Amazon SES Developer Guide . You can execut ... |
| `DeleteIdentity` | `-` | - | `Identity` | - | `DeleteIdentityResponse` | - | Deletes the specified identity (an email address or a domain) from the list of verified identities. You can execute this operation no more than once per second. |
| `DeleteIdentityPolicy` | `-` | - | `Identity`, `PolicyName` | - | `DeleteIdentityPolicyResponse` | - | Deletes the specified sending authorization policy for the given identity (an email address or a domain). This operation returns successfully even if a policy with the specified name does not exist. This operation is ... |
| `DeleteReceiptFilter` | `-` | - | `FilterName` | - | `DeleteReceiptFilterResponse` | - | Deletes the specified IP address filter. For information about managing IP address filters, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `DeleteReceiptRule` | `-` | - | `RuleSetName`, `RuleName` | - | `DeleteReceiptRuleResponse` | `RuleSetDoesNotExistException` | Deletes the specified receipt rule. For information about managing receipt rules, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `DeleteReceiptRuleSet` | `-` | - | `RuleSetName` | - | `DeleteReceiptRuleSetResponse` | `CannotDeleteException` | Deletes the specified receipt rule set and all of the receipt rules it contains. The currently active rule set cannot be deleted. For information about managing receipt rule sets, see the Amazon SES Developer Guide . ... |
| `DeleteTemplate` | `-` | - | `TemplateName` | - | `DeleteTemplateResponse` | - | Deletes an email template. You can execute this operation no more than once per second. |
| `DeleteVerifiedEmailAddress` | `-` | - | `EmailAddress` | - | `Unit` | - | Deprecated. Use the DeleteIdentity operation to delete email addresses and domains. |
| `DescribeActiveReceiptRuleSet` | `-` | - | - | - | `DescribeActiveReceiptRuleSetResponse` | - | Returns the metadata and receipt rules for the receipt rule set that is currently active. For information about setting up receipt rule sets, see the Amazon SES Developer Guide . You can execute this operation no mor ... |
| `DescribeConfigurationSet` | `-` | - | `ConfigurationSetName` | - | `DescribeConfigurationSetResponse` | `ConfigurationSetDoesNotExistException` | Returns the details of the specified configuration set. For information about using configuration sets, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `DescribeReceiptRule` | `-` | - | `RuleSetName`, `RuleName` | - | `DescribeReceiptRuleResponse` | `RuleDoesNotExistException`, `RuleSetDoesNotExistException` | Returns the details of the specified receipt rule. For information about setting up receipt rules, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `DescribeReceiptRuleSet` | `-` | - | `RuleSetName` | - | `DescribeReceiptRuleSetResponse` | `RuleSetDoesNotExistException` | Returns the details of the specified receipt rule set. For information about managing receipt rule sets, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `GetAccountSendingEnabled` | `-` | - | - | - | `GetAccountSendingEnabledResponse` | - | Returns the email sending status of the Amazon SES account for the current Region. You can execute this operation no more than once per second. |
| `GetCustomVerificationEmailTemplate` | `-` | - | `TemplateName` | - | `GetCustomVerificationEmailTemplateResponse` | `CustomVerificationEmailTemplateDoesNotExistException` | Returns the custom email verification template for the template name you specify. For more information about custom verification email templates, see Using Custom Verification Email Templates in the Amazon SES Develo ... |
| `GetIdentityDkimAttributes` | `-` | - | `Identities` | - | `GetIdentityDkimAttributesResponse` | - | Returns the current status of Easy DKIM signing for an entity. For domain name identities, this operation also returns the DKIM tokens that are required for Easy DKIM signing, and whether Amazon SES has successfully ... |
| `GetIdentityMailFromDomainAttributes` | `-` | - | `Identities` | - | `GetIdentityMailFromDomainAttributesResponse` | - | Returns the custom MAIL FROM attributes for a list of identities (email addresses : domains). This operation is throttled at one request per second and can only get custom MAIL FROM attributes for up to 100 identitie ... |
| `GetIdentityNotificationAttributes` | `-` | - | `Identities` | - | `GetIdentityNotificationAttributesResponse` | - | Given a list of verified identities (email addresses and/or domains), returns a structure describing identity notification attributes. This operation is throttled at one request per second and can only get notificati ... |
| `GetIdentityPolicies` | `-` | - | `Identity`, `PolicyNames` | - | `GetIdentityPoliciesResponse` | - | Returns the requested sending authorization policies for the given identity (an email address or a domain). The policies are returned as a map of policy names to policy contents. You can retrieve a maximum of 20 poli ... |
| `GetIdentityVerificationAttributes` | `-` | - | `Identities` | - | `GetIdentityVerificationAttributesResponse` | - | Given a list of identities (email addresses and/or domains), returns the verification status and (for domain identities) the verification token for each identity. The verification status of an email address is "Pendi ... |
| `GetSendQuota` | `-` | - | - | - | `GetSendQuotaResponse` | - | Provides the sending limits for the Amazon SES account. You can execute this operation no more than once per second. |
| `GetSendStatistics` | `-` | - | - | - | `GetSendStatisticsResponse` | - | Provides sending statistics for the current Amazon Web Services Region. The result is a list of data points, representing the last two weeks of sending activity. Each data point in the list contains statistics for a ... |
| `GetTemplate` | `-` | - | `TemplateName` | - | `GetTemplateResponse` | `TemplateDoesNotExistException` | Displays the template object (which includes the Subject line, HTML part and text part) for the template you specify. You can execute this operation no more than once per second. |
| `ListConfigurationSets` | `-` | - | - | - | `ListConfigurationSetsResponse` | - | Provides a list of the configuration sets associated with your Amazon SES account in the current Amazon Web Services Region. For information about using configuration sets, see Monitoring Your Amazon SES Sending Acti ... |
| `ListCustomVerificationEmailTemplates` | `-` | `paginated` | - | - | `ListCustomVerificationEmailTemplatesResponse` | - | Lists the existing custom verification email templates for your account in the current Amazon Web Services Region. For more information about custom verification email templates, see Using Custom Verification Email T ... |
| `ListIdentities` | `-` | `paginated` | - | - | `ListIdentitiesResponse` | - | Returns a list containing all of the identities (email addresses and domains) for your Amazon Web Services account in the current Amazon Web Services Region, regardless of verification status. You can execute this op ... |
| `ListIdentityPolicies` | `-` | - | `Identity` | - | `ListIdentityPoliciesResponse` | - | Returns a list of sending authorization policies that are attached to the given identity (an email address or a domain). This operation returns only a list. To get the actual policy content, use GetIdentityPolicies . ... |
| `ListReceiptFilters` | `-` | - | - | - | `ListReceiptFiltersResponse` | - | Lists the IP address filters associated with your Amazon Web Services account in the current Amazon Web Services Region. For information about managing IP address filters, see the Amazon SES Developer Guide . You can ... |
| `ListReceiptRuleSets` | `-` | - | - | - | `ListReceiptRuleSetsResponse` | - | Lists the receipt rule sets that exist under your Amazon Web Services account in the current Amazon Web Services Region. If there are additional receipt rule sets to be retrieved, you receive a NextToken that you can ... |
| `ListTemplates` | `-` | - | - | - | `ListTemplatesResponse` | - | Lists the email templates present in your Amazon SES account in the current Amazon Web Services Region. You can execute this operation no more than once per second. |
| `ListVerifiedEmailAddresses` | `-` | - | - | - | `ListVerifiedEmailAddressesResponse` | - | Deprecated. Use the ListIdentities operation to list the email addresses and domains associated with your account. |
| `PutConfigurationSetDeliveryOptions` | `-` | - | `ConfigurationSetName` | - | `PutConfigurationSetDeliveryOptionsResponse` | `ConfigurationSetDoesNotExistException`, `InvalidDeliveryOptionsException` | Adds or updates the delivery options for a configuration set. |
| `PutIdentityPolicy` | `-` | - | `Identity`, `PolicyName`, `Policy` | - | `PutIdentityPolicyResponse` | `InvalidPolicyException` | Adds or updates a sending authorization policy for the specified identity (an email address or a domain). This operation is for the identity owner only. If you have not verified the identity, it returns an error. Sen ... |
| `ReorderReceiptRuleSet` | `-` | - | `RuleSetName`, `RuleNames` | - | `ReorderReceiptRuleSetResponse` | `RuleDoesNotExistException`, `RuleSetDoesNotExistException` | Reorders the receipt rules within a receipt rule set. All of the rules in the rule set must be represented in this request. That is, it is error if the reorder request doesn't explicitly position all of the rules. Fo ... |
| `SendBounce` | `-` | - | `OriginalMessageId`, `BounceSender`, `BouncedRecipientInfoList` | - | `SendBounceResponse` | `MessageRejected` | Generates and sends a bounce message to the sender of an email you received through Amazon SES. You can only use this operation on an email up to 24 hours after you receive it. You cannot use this operation to send g ... |
| `SendBulkTemplatedEmail` | `-` | - | `Source`, `Template`, `DefaultTemplateData`, `Destinations` | - | `SendBulkTemplatedEmailResponse` | `AccountSendingPausedException`, `ConfigurationSetDoesNotExistException`, `ConfigurationSetSendingPausedException`, `MailFromDomainNotVerifiedException`, `MessageRejected`, `TemplateDoesNotExistException` | Composes an email message to multiple destinations. The message body is created using an email template. To send email using this operation, your call must meet the following requirements: The call must refer to an e ... |
| `SendCustomVerificationEmail` | `-` | - | `EmailAddress`, `TemplateName` | - | `SendCustomVerificationEmailResponse` | `ConfigurationSetDoesNotExistException`, `CustomVerificationEmailTemplateDoesNotExistException`, `FromEmailAddressNotVerifiedException`, `MessageRejected`, `ProductionAccessNotGrantedException` | Adds an email address to the list of identities for your Amazon SES account in the current Amazon Web Services Region and attempts to verify it. As a result of executing this operation, a customized verification emai ... |
| `SendEmail` | `-` | - | `Source`, `Destination`, `Message` | - | `SendEmailResponse` | `AccountSendingPausedException`, `ConfigurationSetDoesNotExistException`, `ConfigurationSetSendingPausedException`, `MailFromDomainNotVerifiedException`, `MessageRejected` | Composes an email message and immediately queues it for sending. To send email using this operation, your message must meet the following requirements: The message must be sent from a verified email address or domain ... |
| `SendRawEmail` | `-` | - | `RawMessage` | - | `SendRawEmailResponse` | `AccountSendingPausedException`, `ConfigurationSetDoesNotExistException`, `ConfigurationSetSendingPausedException`, `MailFromDomainNotVerifiedException`, `MessageRejected` | Composes an email message and immediately queues it for sending. This operation is more flexible than the SendEmail operation. When you use the SendRawEmail operation, you can specify the headers of the message as we ... |
| `SendTemplatedEmail` | `-` | - | `Source`, `Destination`, `Template`, `TemplateData` | - | `SendTemplatedEmailResponse` | `AccountSendingPausedException`, `ConfigurationSetDoesNotExistException`, `ConfigurationSetSendingPausedException`, `MailFromDomainNotVerifiedException`, `MessageRejected`, `TemplateDoesNotExistException` | Composes an email message using an email template and immediately queues it for sending. To send email using this operation, your call must meet the following requirements: The call must refer to an existing email te ... |
| `SetActiveReceiptRuleSet` | `-` | - | - | - | `SetActiveReceiptRuleSetResponse` | `RuleSetDoesNotExistException` | Sets the specified receipt rule set as the active receipt rule set. To disable your email-receiving through Amazon SES completely, you can call this operation with RuleSetName set to null. For information about manag ... |
| `SetIdentityDkimEnabled` | `-` | - | `Identity`, `DkimEnabled` | - | `SetIdentityDkimEnabledResponse` | - | Enables or disables Easy DKIM signing of email sent from an identity. If Easy DKIM signing is enabled for a domain, then Amazon SES uses DKIM to sign all email that it sends from addresses on that domain. If Easy DKI ... |
| `SetIdentityFeedbackForwardingEnabled` | `-` | - | `Identity`, `ForwardingEnabled` | - | `SetIdentityFeedbackForwardingEnabledResponse` | - | Given an identity (an email address or a domain), enables or disables whether Amazon SES forwards bounce and complaint notifications as email. Feedback forwarding can only be disabled when Amazon Simple Notification ... |
| `SetIdentityHeadersInNotificationsEnabled` | `-` | - | `Identity`, `NotificationType`, `Enabled` | - | `SetIdentityHeadersInNotificationsEnabledResponse` | - | Given an identity (an email address or a domain), sets whether Amazon SES includes the original email headers in the Amazon Simple Notification Service (Amazon SNS) notifications of a specified type. You can execute ... |
| `SetIdentityMailFromDomain` | `-` | - | `Identity` | - | `SetIdentityMailFromDomainResponse` | - | Enables or disables the custom MAIL FROM domain setup for a verified identity (an email address or a domain). To send emails using the specified MAIL FROM domain, you must add an MX record to your MAIL FROM domain's ... |
| `SetIdentityNotificationTopic` | `-` | - | `Identity`, `NotificationType` | - | `SetIdentityNotificationTopicResponse` | - | Sets an Amazon Simple Notification Service (Amazon SNS) topic to use when delivering notifications. When you use this operation, you specify a verified identity, such as an email address or domain. When you send an e ... |
| `SetReceiptRulePosition` | `-` | - | `RuleSetName`, `RuleName` | - | `SetReceiptRulePositionResponse` | `RuleDoesNotExistException`, `RuleSetDoesNotExistException` | Sets the position of the specified receipt rule in the receipt rule set. For information about managing receipt rules, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `TestRenderTemplate` | `-` | - | `TemplateName`, `TemplateData` | - | `TestRenderTemplateResponse` | `InvalidRenderingParameterException`, `MissingRenderingAttributeException`, `TemplateDoesNotExistException` | Creates a preview of the MIME content of an email when provided with a template and a set of replacement data. You can execute this operation no more than once per second. |
| `UpdateAccountSendingEnabled` | `-` | - | - | - | `Unit` | - | Enables or disables email sending across your entire Amazon SES account in the current Amazon Web Services Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pause email se ... |
| `UpdateConfigurationSetEventDestination` | `-` | - | `ConfigurationSetName`, `EventDestination` | - | `UpdateConfigurationSetEventDestinationResponse` | `ConfigurationSetDoesNotExistException`, `EventDestinationDoesNotExistException`, `InvalidCloudWatchDestinationException`, `InvalidFirehoseDestinationException`, `InvalidSNSDestinationException` | Updates the event destination of a configuration set. Event destinations are associated with configuration sets, which enable you to publish email sending events to Amazon CloudWatch, Amazon Kinesis Firehose, or Amaz ... |
| `UpdateConfigurationSetReputationMetricsEnabled` | `-` | - | `ConfigurationSetName`, `Enabled` | - | `Unit` | `ConfigurationSetDoesNotExistException` | Enables or disables the publishing of reputation metrics for emails sent using a specific configuration set in a given Amazon Web Services Region. Reputation metrics include bounce and complaint rates. These metrics ... |
| `UpdateConfigurationSetSendingEnabled` | `-` | - | `ConfigurationSetName`, `Enabled` | - | `Unit` | `ConfigurationSetDoesNotExistException` | Enables or disables email sending for messages sent using a specific configuration set in a given Amazon Web Services Region. You can use this operation in conjunction with Amazon CloudWatch alarms to temporarily pau ... |
| `UpdateConfigurationSetTrackingOptions` | `-` | - | `ConfigurationSetName`, `TrackingOptions` | - | `UpdateConfigurationSetTrackingOptionsResponse` | `ConfigurationSetDoesNotExistException`, `InvalidTrackingOptionsException`, `TrackingOptionsDoesNotExistException` | Modifies an association between a configuration set and a custom domain for open and click event tracking. By default, images and links used for tracking open and click events are hosted on domains operated by Amazon ... |
| `UpdateCustomVerificationEmailTemplate` | `-` | - | `TemplateName` | - | `Unit` | `CustomVerificationEmailInvalidContentException`, `CustomVerificationEmailTemplateDoesNotExistException`, `FromEmailAddressNotVerifiedException` | Updates an existing custom verification email template. For more information about custom verification email templates, see Using Custom Verification Email Templates in the Amazon SES Developer Guide . You can execut ... |
| `UpdateReceiptRule` | `-` | - | `RuleSetName`, `Rule` | - | `UpdateReceiptRuleResponse` | `InvalidLambdaFunctionException`, `InvalidS3ConfigurationException`, `InvalidSnsTopicException`, `LimitExceededException`, `RuleDoesNotExistException`, `RuleSetDoesNotExistException` | Updates a receipt rule. For information about managing receipt rules, see the Amazon SES Developer Guide . You can execute this operation no more than once per second. |
| `UpdateTemplate` | `-` | - | `Template` | - | `UpdateTemplateResponse` | `InvalidTemplateException`, `TemplateDoesNotExistException` | Updates an email template. Email templates enable you to send personalized email to one or more destinations in a single operation. For more information, see the Amazon SES Developer Guide . You can execute this oper ... |
| `VerifyDomainDkim` | `-` | - | `Domain` | - | `VerifyDomainDkimResponse` | - | Returns a set of DKIM tokens for a domain identity. When you execute the VerifyDomainDkim operation, the domain that you specify is added to the list of identities that are associated with your account. This is true ... |
| `VerifyDomainIdentity` | `-` | - | `Domain` | - | `VerifyDomainIdentityResponse` | - | Adds a domain to the list of identities for your Amazon SES account in the current Amazon Web Services Region and attempts to verify it. For more information about verifying domains, see Verifying Email Addresses and ... |
| `VerifyEmailAddress` | `-` | - | `EmailAddress` | - | `Unit` | - | Deprecated. Use the VerifyEmailIdentity operation to verify a new email address. |
| `VerifyEmailIdentity` | `-` | - | `EmailAddress` | - | `VerifyEmailIdentityResponse` | - | Adds an email address to the list of identities for your Amazon SES account in the current Amazon Web Services Region and attempts to verify it. As a result of executing this operation, a verification email is sent t ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccountSendingPausedException` | `structure` | message | Indicates that email sending is disabled for your entire Amazon SES account. You can enable or disable email sending for your Amazon SES account using Updat ... |
| `AlreadyExistsException` | `structure` | Name, message | Indicates that a resource could not be created because of a naming conflict. |
| `CannotDeleteException` | `structure` | Name, message | Indicates that the delete operation could not be completed. |
| `ConfigurationSetAlreadyExistsException` | `structure` | ConfigurationSetName, message | Indicates that the configuration set could not be created because of a naming conflict. |
| `ConfigurationSetDoesNotExistException` | `structure` | ConfigurationSetName, message | Indicates that the configuration set does not exist. |
| `ConfigurationSetSendingPausedException` | `structure` | ConfigurationSetName, message | Indicates that email sending is disabled for the configuration set. You can enable or disable email sending for a configuration set using UpdateConfiguratio ... |
| `CustomVerificationEmailInvalidContentException` | `structure` | message | Indicates that custom verification email template provided content is invalid. |
| `CustomVerificationEmailTemplateAlreadyExistsException` | `structure` | CustomVerificationEmailTemplateName, message | Indicates that a custom verification email template with the name you specified already exists. |
| `CustomVerificationEmailTemplateDoesNotExistException` | `structure` | CustomVerificationEmailTemplateName, message | Indicates that a custom verification email template with the name you specified does not exist. |
| `EventDestinationAlreadyExistsException` | `structure` | ConfigurationSetName, EventDestinationName, message | Indicates that the event destination could not be created because of a naming conflict. |
| `EventDestinationDoesNotExistException` | `structure` | ConfigurationSetName, EventDestinationName, message | Indicates that the event destination does not exist. |
| `FromEmailAddressNotVerifiedException` | `structure` | FromEmailAddress, message | Indicates that the sender address specified for a custom verification email is not verified, and is therefore not eligible to send the custom verification e ... |
| `InvalidCloudWatchDestinationException` | `structure` | ConfigurationSetName, EventDestinationName, message | Indicates that the Amazon CloudWatch destination is invalid. See the error message for details. |
| `InvalidConfigurationSetException` | `structure` | message | Indicates that the configuration set is invalid. See the error message for details. |
| `InvalidDeliveryOptionsException` | `structure` | message | Indicates that provided delivery option is invalid. |
| `InvalidFirehoseDestinationException` | `structure` | ConfigurationSetName, EventDestinationName, message | Indicates that the Amazon Kinesis Firehose destination is invalid. See the error message for details. |
| `InvalidLambdaFunctionException` | `structure` | FunctionArn, message | Indicates that the provided Amazon Web Services Lambda function is invalid, or that Amazon SES could not execute the provided function, possibly due to perm ... |
| `InvalidPolicyException` | `structure` | message | Indicates that the provided policy is invalid. Check the error stack for more information about what caused the error. |
| `InvalidRenderingParameterException` | `structure` | TemplateName, message | Indicates that one or more of the replacement values you provided is invalid. This error may occur when the TemplateData object contains invalid JSON. |
| `InvalidS3ConfigurationException` | `structure` | Bucket, message | Indicates that the provided Amazon S3 bucket or Amazon Web Services KMS encryption key is invalid, or that Amazon SES could not publish to the bucket, possi ... |
| `InvalidSNSDestinationException` | `structure` | ConfigurationSetName, EventDestinationName, message | Indicates that the Amazon Simple Notification Service (Amazon SNS) destination is invalid. See the error message for details. |
| `InvalidSnsTopicException` | `structure` | Topic, message | Indicates that the provided Amazon SNS topic is invalid, or that Amazon SES could not publish to the topic, possibly due to permissions issues. For informat ... |
| `InvalidTemplateException` | `structure` | TemplateName, message | Indicates that the template that you specified could not be rendered. This issue may occur when a template refers to a partial that does not exist. |
| `InvalidTrackingOptionsException` | `structure` | message | Indicates that the custom domain to be used for open and click tracking redirects is invalid. This error appears most often in the following situations: Whe ... |
| `LimitExceededException` | `structure` | message | Indicates that a resource could not be created because of service limits. For a list of Amazon SES limits, see the Amazon SES Developer Guide . |
| `MailFromDomainNotVerifiedException` | `structure` | message | Indicates that the message could not be sent because Amazon SES could not read the MX record required to use the specified MAIL FROM domain. For information ... |
| `MessageRejected` | `structure` | message | Indicates that the action failed, and the message could not be sent. Check the error stack for more information about what caused the error. |
| `MissingRenderingAttributeException` | `structure` | TemplateName, message | Indicates that one or more of the replacement values for the specified template was not specified. Ensure that the TemplateData object contains references t ... |
| `ProductionAccessNotGrantedException` | `structure` | message | Indicates that the account has not been granted production access. |
| `RuleDoesNotExistException` | `structure` | Name, message | Indicates that the provided receipt rule does not exist. |
| `RuleSetDoesNotExistException` | `structure` | Name, message | Indicates that the provided receipt rule set does not exist. |
| `TemplateDoesNotExistException` | `structure` | TemplateName, message | Indicates that the Template object you specified does not exist in your Amazon SES account. |
| `TrackingOptionsAlreadyExistsException` | `structure` | ConfigurationSetName, message | Indicates that the configuration set you specified already contains a TrackingOptions object. |
| `TrackingOptionsDoesNotExistException` | `structure` | ConfigurationSetName, message | Indicates that the TrackingOptions object you specified does not exist. |
| `CloneReceiptRuleSetRequest` | `structure` | RuleSetName, OriginalRuleSetName | Represents a request to create a receipt rule set by cloning an existing one. You use receipt rule sets to receive email with Amazon SES. For more informati ... |
| `CloneReceiptRuleSetResponse` | `structure` | **empty (no members)** | An empty element returned on a successful request. |
| `CreateConfigurationSetRequest` | `structure` | ConfigurationSet | Represents a request to create a configuration set. Configuration sets enable you to publish email sending events. For information about using configuration ... |
| `CreateConfigurationSetResponse` | `structure` | **empty (no members)** | An empty element returned on a successful request. |
| `CreateConfigurationSetEventDestinationRequest` | `structure` | ConfigurationSetName, EventDestination | Represents a request to create a configuration set event destination. A configuration set event destination, which can be either Amazon CloudWatch or Amazon ... |
| `CreateConfigurationSetEventDestinationResponse` | `structure` | **empty (no members)** | An empty element returned on a successful request. |
| `BehaviorOnMXFailure` | `enum` | UseDefaultValue, RejectMessage | - |
| `BounceType` | `enum` | DoesNotExist, MessageTooLarge, ExceededQuota, ContentRejected, Undefined, TemporaryFailure | - |
| `BulkEmailStatus` | `enum` | Success, MessageRejected, MailFromDomainNotVerified, ConfigurationSetDoesNotExist, TemplateDoesNotExist, AccountSuspended, AccountThrottled, AccountDailyQuotaExceeded, InvalidSendingPoolName, AccountSendingPaused, ConfigurationSetSendingPaused, InvalidParameterValue, ... (+2) | - |
| `ConfigurationSetAttribute` | `enum` | EVENT_DESTINATIONS, TRACKING_OPTIONS, DELIVERY_OPTIONS, REPUTATION_OPTIONS | - |
| `CustomMailFromStatus` | `enum` | Pending, Success, Failed, TemporaryFailure | - |
| `DimensionValueSource` | `enum` | MESSAGE_TAG, EMAIL_HEADER, LINK_TAG | - |
| `DsnAction` | `enum` | FAILED, DELAYED, DELIVERED, RELAYED, EXPANDED | - |
| `EventType` | `enum` | SEND, REJECT, BOUNCE, COMPLAINT, DELIVERY, OPEN, CLICK, RENDERING_FAILURE | - |
| `IdentityType` | `enum` | EmailAddress, Domain | - |
| `InvocationType` | `enum` | Event, RequestResponse | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# Amazon Simple Notification Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Simple Notification Service Amazon Simple Notification Service (Amazon SNS) is a web service that enables you to build distributed web-enabled applications. Applications can use Amazon SNS to easily push real-time notification messages to interested subscribers over multiple delivery protocols. For more information about this product see the Amazon SNS product page. For detailed information about Amazon SNS features and their associated API calls, see the Amazon SNS Developer Guide. For information on the permissions you need to use this API, see Identity and access management in Amazon SNS in the Amazon SNS Developer Guide.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-sns/tests/scenario_test.rs`: build a multi-subscriber notification hub with topic creation, subscriptions, publish, and cleanup.
- Backported from `scenario_test.rs`: publish batches of alarm or event messages.
- Backported from `scenario_test.rs`: manage topic attributes and subscription filter policy attributes.
- Scenario insight from EC2: include mutable binding failover for Amazon Simple Notification Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: support pub/sub topics, subscriptions, filter policies, FIFO topics, batch publish, SMS/mobile push surfaces, delivery policies, tagging, and permissions.

## Service Identity and Protocol

- AWS model slug: `sns`
- AWS SDK for Rust slug: `sns`
- Model version: `2010-03-31`
- Model file: `vendor/api-models-aws/models/sns/service/2010-03-31/sns-2010-03-31.json`
- SDK ID: `SNS`
- Endpoint prefix: `sns`
- ARN namespace: `sns`
- CloudFormation name: `SNS`
- CloudTrail event source: `sns.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (9), `Get` (7), `Set` (5), `Create` (4), `Delete` (4), `Publish` (2), `Add` (1), `Check` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddPermission`, `CreatePlatformApplication`, `CreatePlatformEndpoint`, `CreateSMSSandboxPhoneNumber`, `CreateTopic`, `DeleteEndpoint`, `DeletePlatformApplication`, `DeleteSMSSandboxPhoneNumber`, `DeleteTopic`, `PutDataProtectionPolicy`, `RemovePermission`, `SetEndpointAttributes`, `SetPlatformApplicationAttributes`, `SetSMSAttributes`, `SetSubscriptionAttributes`, `SetTopicAttributes`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckIfPhoneNumberIsOptedOut`, `GetDataProtectionPolicy`, `GetEndpointAttributes`, `GetPlatformApplicationAttributes`, `GetSMSAttributes`, `GetSMSSandboxAccountStatus`, `GetSubscriptionAttributes`, `GetTopicAttributes`, `ListEndpointsByPlatformApplication`, `ListOriginationNumbers`, `ListPhoneNumbersOptedOut`, `ListPlatformApplications`, `ListSMSSandboxPhoneNumbers`, `ListSubscriptions`, `ListSubscriptionsByTopic`, `ListTagsForResource`, `ListTopics`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 42 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/sns/latest/dg/sns-message-filtering.html
- https://docs.aws.amazon.com/sns/latest/dg/sns-message-delivery-retries.html
- https://docs.aws.amazon.com/sns/latest/dg/sns-dead-letter-queues.html

Research outcomes:
- By default, every subscription to a topic receives every message published to that topic.
- A subscription filter policy is a JSON object. Depending on filter policy scope, it matches either message attributes or JSON message body properties.
- If a subscription has a filter policy, SNS compares each published message against that subscription's policy and only delivers messages satisfying all required policy conditions.
- SNS retry policies vary by endpoint protocol. HTTP/S is the main protocol with customer-configurable delivery policy; the total HTTP/S retry policy time cannot exceed 3600 seconds.
- SNS applies jitter to delivery retries.
- AWS managed endpoints such as Lambda and SQS have long SNS-managed retry schedules; customer-managed endpoints such as HTTP/S, SMS, email, and mobile push have different retry behaviour.
- Dead-letter queues capture messages that cannot be delivered after retry handling.

Parity implications:
- Subscription delivery needs per-subscription filter-policy evaluation before fanout.
- Retry and DLQ behaviour should be modelled per protocol rather than as one generic failure path.
- Raw message delivery, message attributes, FIFO topic fields, and message body filtering should remain distinct in the publish pipeline.

## Operation Groups

### List

- Operations: `ListEndpointsByPlatformApplication`, `ListOriginationNumbers`, `ListPhoneNumbersOptedOut`, `ListPlatformApplications`, `ListSMSSandboxPhoneNumbers`, `ListSubscriptions`, `ListSubscriptionsByTopic`, `ListTagsForResource`, `ListTopics`
- Traits: `paginated` (8)
- Common required input members in this group: `PlatformApplicationArn`, `ResourceArn`, `TopicArn`

### Get

- Operations: `GetDataProtectionPolicy`, `GetEndpointAttributes`, `GetPlatformApplicationAttributes`, `GetSMSAttributes`, `GetSMSSandboxAccountStatus`, `GetSubscriptionAttributes`, `GetTopicAttributes`
- Common required input members in this group: `EndpointArn`, `PlatformApplicationArn`, `ResourceArn`, `SubscriptionArn`, `TopicArn`

### Set

- Operations: `SetEndpointAttributes`, `SetPlatformApplicationAttributes`, `SetSMSAttributes`, `SetSubscriptionAttributes`, `SetTopicAttributes`
- Common required input members in this group: `AttributeName`, `Attributes`, `EndpointArn`, `PlatformApplicationArn`, `SubscriptionArn`, `TopicArn`, `attributes`

### Create

- Operations: `CreatePlatformApplication`, `CreatePlatformEndpoint`, `CreateSMSSandboxPhoneNumber`, `CreateTopic`
- Common required input members in this group: `Attributes`, `Name`, `PhoneNumber`, `Platform`, `PlatformApplicationArn`, `Token`

### Delete

- Operations: `DeleteEndpoint`, `DeletePlatformApplication`, `DeleteSMSSandboxPhoneNumber`, `DeleteTopic`
- Common required input members in this group: `EndpointArn`, `PhoneNumber`, `PlatformApplicationArn`, `TopicArn`

### Publish

- Operations: `Publish`, `PublishBatch`
- Common required input members in this group: `Message`, `PublishBatchRequestEntries`, `TopicArn`

### Add

- Operations: `AddPermission`
- Common required input members in this group: `AWSAccountId`, `ActionName`, `Label`, `TopicArn`

### Check

- Operations: `CheckIfPhoneNumberIsOptedOut`
- Common required input members in this group: `phoneNumber`

### Confirm

- Operations: `ConfirmSubscription`
- Common required input members in this group: `Token`, `TopicArn`

### Opt

- Operations: `OptInPhoneNumber`
- Common required input members in this group: `phoneNumber`

### Put

- Operations: `PutDataProtectionPolicy`
- Common required input members in this group: `DataProtectionPolicy`, `ResourceArn`

### Remove

- Operations: `RemovePermission`
- Common required input members in this group: `Label`, `TopicArn`

### Subscribe

- Operations: `Subscribe`
- Common required input members in this group: `Protocol`, `TopicArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Unsubscribe

- Operations: `Unsubscribe`
- Common required input members in this group: `SubscriptionArn`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Verify

- Operations: `VerifySMSSandboxPhoneNumber`
- Common required input members in this group: `OneTimePassword`, `PhoneNumber`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddPermission` | - | - | `AWSAccountId`, `ActionName`, `Label`, `TopicArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Adds a statement to a topic's access control policy, granting access for the specified Amazon Web Services accounts to the specified actions. To remove the ability to change topic permissions, you must deny permissions to the `AddPermission`... |
| `CheckIfPhoneNumberIsOptedOut` | - | - | `phoneNumber` | - | `CheckIfPhoneNumberIsOptedOutResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ThrottledException` | Accepts a phone number and indicates whether the phone holder has opted out of receiving SMS messages from your Amazon Web Services account. You cannot send SMS messages to a number that is opted out. |
| `ConfirmSubscription` | - | - | `Token`, `TopicArn` | - | `ConfirmSubscriptionResponse` | `AuthorizationErrorException`, `FilterPolicyLimitExceededException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ReplayLimitExceededException`, `SubscriptionLimitExceededException` | Verifies an endpoint owner's intent to receive messages by validating the token sent to the endpoint by an earlier `Subscribe` action. If the token is valid, the action creates a new subscription and returns its Amazon Resource Name (ARN). |
| `CreatePlatformApplication` | - | - | `Attributes`, `Name`, `Platform` | - | `CreatePlatformApplicationResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException` | Creates a platform application object for one of the supported push notification services, such as APNS and GCM (Firebase Cloud Messaging), to which devices and mobile apps may register. You must specify `PlatformPrincipal` and `PlatformCredential` attributes... |
| `CreatePlatformEndpoint` | - | - | `PlatformApplicationArn`, `Token` | - | `CreateEndpointResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Creates an endpoint for a device and mobile app on one of the supported push notification services, such as GCM (Firebase Cloud Messaging) and APNS. `CreatePlatformEndpoint` requires the `PlatformApplicationArn` that is returned from... |
| `CreateSMSSandboxPhoneNumber` | - | - | `PhoneNumber` | - | `CreateSMSSandboxPhoneNumberResult` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `OptedOutException`, `ThrottledException`, `UserErrorException` | Adds a destination phone number to an Amazon Web Services account in the SMS sandbox and sends a one-time password (OTP) to that phone number. When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the SMS sandbox . |
| `CreateTopic` | - | - | `Name` | - | `CreateTopicResponse` | `AuthorizationErrorException`, `ConcurrentAccessException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `StaleTagException`, `TagLimitExceededException`, `TagPolicyException`, ... (+1) | Creates a topic to which notifications can be published. Users can create at most 100,000 standard topics (at most 1,000 FIFO topics). |
| `DeleteEndpoint` | - | - | `EndpointArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException` | Deletes the endpoint for a device and mobile app from Amazon SNS. This action is idempotent. |
| `DeletePlatformApplication` | - | - | `PlatformApplicationArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException` | Deletes a platform application object for one of the supported push notification services, such as APNS and GCM (Firebase Cloud Messaging). For more information, see Using Amazon SNS Mobile Push Notifications. |
| `DeleteSMSSandboxPhoneNumber` | - | - | `PhoneNumber` | - | `DeleteSMSSandboxPhoneNumberResult` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottledException`, `UserErrorException` | Deletes an Amazon Web Services account's verified or pending phone number from the SMS sandbox. When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the SMS sandbox . |
| `DeleteTopic` | - | - | `TopicArn` | - | `Unit` | `AuthorizationErrorException`, `ConcurrentAccessException`, `InternalErrorException`, `InvalidParameterException`, `InvalidStateException`, `NotFoundException`, `StaleTagException`, `TagPolicyException` | Deletes a topic and all its subscriptions. Deleting a topic might prevent some messages previously sent to the topic from being delivered to subscribers. |
| `GetDataProtectionPolicy` | - | - | `ResourceArn` | - | `GetDataProtectionPolicyResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `NotFoundException` | Retrieves the specified inline `DataProtectionPolicy` document that is stored in the specified Amazon SNS topic. |
| `GetEndpointAttributes` | - | - | `EndpointArn` | - | `GetEndpointAttributesResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Retrieves the endpoint attributes for a device on one of the supported push notification services, such as GCM (Firebase Cloud Messaging) and APNS. For more information, see Using Amazon SNS Mobile Push Notifications. |
| `GetPlatformApplicationAttributes` | - | - | `PlatformApplicationArn` | - | `GetPlatformApplicationAttributesResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Retrieves the attributes of the platform application object for the supported push notification services, such as APNS and GCM (Firebase Cloud Messaging). For more information, see Using Amazon SNS Mobile Push Notifications. |
| `GetSMSAttributes` | - | - | - | - | `GetSMSAttributesResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ThrottledException` | Returns the settings for sending SMS messages from your Amazon Web Services account. These settings are set with the `SetSMSAttributes` action. |
| `GetSMSSandboxAccountStatus` | - | - | - | - | `GetSMSSandboxAccountStatusResult` | `AuthorizationErrorException`, `InternalErrorException`, `ThrottledException` | Retrieves the SMS sandbox status for the calling Amazon Web Services account in the target Amazon Web Services Region. When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the SMS sandbox . |
| `GetSubscriptionAttributes` | - | - | `SubscriptionArn` | - | `GetSubscriptionAttributesResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Returns all of the properties of a subscription. |
| `GetTopicAttributes` | - | - | `TopicArn` | - | `GetTopicAttributesResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `NotFoundException` | Returns all of the properties of a topic. Topic properties returned might differ based on the authorization of the user. |
| `ListEndpointsByPlatformApplication` | - | `paginated` | `PlatformApplicationArn` | - | `ListEndpointsByPlatformApplicationResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Lists the endpoints and endpoint attributes for devices in a supported push notification service, such as GCM (Firebase Cloud Messaging) and APNS. The results for `ListEndpointsByPlatformApplication` are paginated and return a limited list of endpoints, up to... |
| `ListOriginationNumbers` | - | `paginated` | - | - | `ListOriginationNumbersResult` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ThrottledException`, `ValidationException` | Lists the calling Amazon Web Services account's dedicated origination numbers and their metadata. For more information about origination numbers, see Origination numbers in the Amazon SNS Developer Guide . |
| `ListPhoneNumbersOptedOut` | - | `paginated` | - | - | `ListPhoneNumbersOptedOutResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ThrottledException` | Returns a list of phone numbers that are opted out, meaning you cannot send SMS messages to them. The results for `ListPhoneNumbersOptedOut` are paginated, and each page returns up to 100 phone numbers. |
| `ListPlatformApplications` | - | `paginated` | - | - | `ListPlatformApplicationsResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException` | Lists the platform application objects for the supported push notification services, such as APNS and GCM (Firebase Cloud Messaging). The results for `ListPlatformApplications` are paginated and return a limited list of applications, up to 100. |
| `ListSMSSandboxPhoneNumbers` | - | `paginated` | - | - | `ListSMSSandboxPhoneNumbersResult` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottledException` | Lists the calling Amazon Web Services account's current verified and pending destination phone numbers in the SMS sandbox. When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the SMS sandbox . |
| `ListSubscriptions` | - | `paginated` | - | - | `ListSubscriptionsResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException` | Returns a list of the requester's subscriptions. Each call returns a limited list of subscriptions, up to 100. |
| `ListSubscriptionsByTopic` | - | `paginated` | `TopicArn` | - | `ListSubscriptionsByTopicResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Returns a list of the subscriptions to a specific topic. Each call returns a limited list of subscriptions, up to 100. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AuthorizationErrorException`, `ConcurrentAccessException`, `InvalidParameterException`, `ResourceNotFoundException`, `TagPolicyException` | List all tags added to the specified Amazon SNS topic. For an overview, see Amazon SNS Tags in the Amazon Simple Notification Service Developer Guide . |
| `ListTopics` | - | `paginated` | - | - | `ListTopicsResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException` | Returns a list of the requester's topics. Each call returns a limited list of topics, up to 100. |
| `OptInPhoneNumber` | - | - | `phoneNumber` | - | `OptInPhoneNumberResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ThrottledException` | Use this request to opt in a phone number that is opted out, which enables you to resume sending SMS messages to the number. You can opt in a phone number only once every 30 days. |
| `Publish` | - | - | `Message` | - | `PublishResponse` | `AuthorizationErrorException`, `EndpointDisabledException`, `InternalErrorException`, `InvalidParameterException`, `InvalidParameterValueException`, `InvalidSecurityException`, `KMSAccessDeniedException`, `KMSDisabledException`, ... (+7) | Sends a message to an Amazon SNS topic, a text message (SMS message) directly to a phone number, or a message to a mobile platform endpoint (when you specify the `TargetArn`). If you send a message to a topic, Amazon SNS delivers the message to each endpoint... |
| `PublishBatch` | - | - | `PublishBatchRequestEntries`, `TopicArn` | - | `PublishBatchResponse` | `AuthorizationErrorException`, `BatchEntryIdsNotDistinctException`, `BatchRequestTooLongException`, `EmptyBatchRequestException`, `EndpointDisabledException`, `InternalErrorException`, `InvalidBatchEntryIdException`, `InvalidParameterException`, ... (+12) | Publishes up to 10 messages to the specified topic in a single batch. This is a batch version of the `Publish` API. |
| `PutDataProtectionPolicy` | - | - | `DataProtectionPolicy`, `ResourceArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `NotFoundException` | Adds or updates an inline policy document that is stored in the specified Amazon SNS topic. |
| `RemovePermission` | - | - | `Label`, `TopicArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Removes a statement from a topic's access control policy. To remove the ability to change topic permissions, you must deny permissions to the `AddPermission`, `RemovePermission`, and `SetTopicAttributes` actions in your IAM policy. |
| `SetEndpointAttributes` | - | - | `Attributes`, `EndpointArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Sets the attributes for an endpoint for a device on one of the supported push notification services, such as GCM (Firebase Cloud Messaging) and APNS. For more information, see Using Amazon SNS Mobile Push Notifications. |
| `SetPlatformApplicationAttributes` | - | - | `Attributes`, `PlatformApplicationArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException` | Sets the attributes of the platform application object for the supported push notification services, such as APNS and GCM (Firebase Cloud Messaging). For more information, see Using Amazon SNS Mobile Push Notifications. |
| `SetSMSAttributes` | - | - | `attributes` | - | `SetSMSAttributesResponse` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ThrottledException` | Use this request to set the default settings for sending SMS messages and receiving daily SMS usage reports. You can override some of these settings for a single message when you use the `Publish` action with the `MessageAttributes.entry.N` parameter. |
| `SetSubscriptionAttributes` | - | - | `AttributeName`, `SubscriptionArn` | - | `Unit` | `AuthorizationErrorException`, `FilterPolicyLimitExceededException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ReplayLimitExceededException` | Allows a subscription owner to set an attribute of the subscription to a new value. |
| `SetTopicAttributes` | - | - | `AttributeName`, `TopicArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `NotFoundException` | Allows a topic owner to set an attribute of the topic to a new value. To remove the ability to change topic permissions, you must deny permissions to the `AddPermission`, `RemovePermission`, and `SetTopicAttributes` actions in your IAM policy. |
| `Subscribe` | - | - | `Protocol`, `TopicArn` | - | `SubscribeResponse` | `AuthorizationErrorException`, `FilterPolicyLimitExceededException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `NotFoundException`, `ReplayLimitExceededException`, `SubscriptionLimitExceededException` | Subscribes an endpoint to an Amazon SNS topic. If the endpoint type is HTTP/S or email, or if the endpoint and the topic are not in the same Amazon Web Services account, the endpoint owner must run the `ConfirmSubscription` action to confirm the subscription. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AuthorizationErrorException`, `ConcurrentAccessException`, `InvalidParameterException`, `ResourceNotFoundException`, `StaleTagException`, `TagLimitExceededException`, `TagPolicyException` | Add tags to the specified Amazon SNS topic. For an overview, see Amazon SNS Tags in the Amazon SNS Developer Guide . |
| `Unsubscribe` | - | - | `SubscriptionArn` | - | `Unit` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `InvalidSecurityException`, `NotFoundException` | Deletes a subscription. If the subscription requires authentication for deletion, only the owner of the subscription or the topic's owner can unsubscribe, and an Amazon Web Services signature is required. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AuthorizationErrorException`, `ConcurrentAccessException`, `InvalidParameterException`, `ResourceNotFoundException`, `StaleTagException`, `TagLimitExceededException`, `TagPolicyException` | Remove tags from the specified Amazon SNS topic. For an overview, see Amazon SNS Tags in the Amazon SNS Developer Guide . |
| `VerifySMSSandboxPhoneNumber` | - | - | `OneTimePassword`, `PhoneNumber` | - | `VerifySMSSandboxPhoneNumberResult` | `AuthorizationErrorException`, `InternalErrorException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottledException`, `VerificationException` | Verifies a destination phone number with a one-time password (OTP) for the calling Amazon Web Services account. When you start using Amazon SNS to send SMS messages, your Amazon Web Services account is in the SMS sandbox . |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AuthorizationErrorException` | `structure` | `message` | Indicates that the user has been denied access to the requested resource. |
| `InvalidParameterException` | `structure` | `message` | Indicates that a request parameter does not comply with the associated constraints. |
| `InternalErrorException` | `structure` | `message` | Indicates an internal service error. |
| `NotFoundException` | `structure` | `message` | Indicates that the requested resource does not exist. |
| `ThrottledException` | `structure` | `message` | Indicates that the rate at which requests have been submitted for this action exceeds the limit for your Amazon Web Services account. |
| `InvalidSecurityException` | `structure` | `message` | The credential signature isn't valid. |
| `ResourceNotFoundException` | `structure` | `message` | Can’t perform the action on the specified resource. |
| `ConcurrentAccessException` | `structure` | `message` | Can't perform multiple operations on a tag simultaneously. |
| `TagPolicyException` | `structure` | `message` | The request doesn't comply with the IAM tag policy. |
| `StaleTagException` | `structure` | `message` | A tag has been added to a resource with the same ARN as a deleted resource. |
| `FilterPolicyLimitExceededException` | `structure` | `message` | Indicates that the number of filter polices in your Amazon Web Services account exceeds the limit. |
| `ReplayLimitExceededException` | `structure` | `message` | Indicates that the request parameter has exceeded the maximum number of concurrent message replays. |
| `TagLimitExceededException` | `structure` | `message` | Can't add more than 50 tags to a topic. |
| `ValidationException` | `structure` | `Message` | Indicates that a parameter in the request is invalid. |
| `SubscriptionLimitExceededException` | `structure` | `message` | Indicates that the customer already owns the maximum allowed number of subscriptions. |
| `UserErrorException` | `structure` | `message` | Indicates that a request parameter does not comply with the associated constraints. |
| `EndpointDisabledException` | `structure` | `message` | Exception error indicating endpoint disabled. |
| `InvalidParameterValueException` | `structure` | `message` | Indicates that a request parameter does not comply with the associated constraints. |
| `KMSAccessDeniedException` | `structure` | `message` | The ciphertext references a key that doesn't exist or that you don't have access to. |
| `KMSDisabledException` | `structure` | `message` | The request was rejected because the specified Amazon Web Services KMS key isn't enabled. |
| `KMSInvalidStateException` | `structure` | `message` | The request was rejected because the state of the specified resource isn't valid for this request. |
| `KMSNotFoundException` | `structure` | `message` | The request was rejected because the specified entity or resource can't be found. |
| `KMSOptInRequired` | `structure` | `message` | The Amazon Web Services access key ID needs a subscription for the service. |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/pluggable-backends-and-query-execution-synthesis.md, .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/pluggable-backends-and-query-execution-synthesis.md`: summarises the SNS backend boundary. Open it before changing topic or subscription persistence, because topic, subscription, delivery, snapshot, restore, and merge semantics should stay backend-facing.
- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises high-value SNS integration paths. Open it for Lambda subscriptions or triggers and Step Functions direct SNS publish integration.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises cross-service execution boundaries. Open it before adding delivery behaviour that calls into Lambda or orchestration services.
- Service implication: the default service path should remain in-memory while optional external persistence stays outside the core service crate.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

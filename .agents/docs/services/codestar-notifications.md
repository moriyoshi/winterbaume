# AWS CodeStar Notifications

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This CodeStar Notifications API Reference provides descriptions and usage examples of the operations and data types for the CodeStar Notifications API. You can use the CodeStar Notifications API to work with the following objects: Notification rules, by calling the following: CreateNotificationRule, which creates a notification rule for a resource in your account. DeleteNotificationRule, which deletes a notification rule. DescribeNotificationRule, which provides information about a notification rule. ListNotificationRules, which lists the notification rules associated with your account.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS CodeStar Notifications where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented AWS CodeStar Notifications workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Delete`, `Create`, `Describe`, `Subscribe` operation families, including `ListEventTypes`, `ListNotificationRules`, `ListTagsForResource`, `ListTargets`, `DeleteNotificationRule`, `DeleteTarget`.

## Service Identity and Protocol

- AWS model slug: `codestar-notifications`
- AWS SDK for Rust slug: `codestarnotifications`
- Model version: `2019-10-15`
- Model file: `vendor/api-models-aws/models/codestar-notifications/service/2019-10-15/codestar-notifications-2019-10-15.json`
- SDK ID: `codestar notifications`
- Endpoint prefix: `codestar-notifications`
- ARN namespace: `codestar-notifications`
- CloudFormation name: `CodeStarNotifications`
- CloudTrail event source: `codestarnotifications.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Delete` (2), `Create` (1), `Describe` (1), `Subscribe` (1), `Tag` (1), `Unsubscribe` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateNotificationRule`, `DeleteNotificationRule`, `DeleteTarget`, `TagResource`, `UntagResource`, `UpdateNotificationRule`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeNotificationRule`, `ListEventTypes`, `ListNotificationRules`, `ListTagsForResource`, `ListTargets`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/dtconsole/latest/userguide/notification-rule-enable-disable.html
- https://docs.aws.amazon.com/dtconsole/latest/userguide/set-up-sns.html
- https://docs.aws.amazon.com/codecommit/latest/userguide/how-to-repository-email.html

Research outcomes:
- CodeStar Notifications uses notification rules to send events from Developer Tools resources to notification targets.
- Notification rules have enabled or disabled status. Disabled rules do not send notifications.
- Rules select one source resource, event types, detail type, and one or more targets.
- Supported targets include Amazon SNS topics, with topic policies granting the notifications service permission to publish.
- Notification rules can be used with services such as CodeCommit, CodeBuild, CodeDeploy, and CodePipeline.
- EventBridge records CodeStar Notifications service events through CloudTrail.

Parity implications:
- Model notification rules, source resource ARN, event type ids, targets, detail type, enabled state, tags, and SNS policy dependency separately.
- Rule evaluation should check enabled state and matching event type before publishing.
- Target validation should distinguish rule creation from eventual publish failure caused by missing SNS permissions.

## Operation Groups

### List

- Operations: `ListEventTypes`, `ListNotificationRules`, `ListTagsForResource`, `ListTargets`
- Traits: `paginated` (3)
- Common required input members in this group: `Arn`

### Delete

- Operations: `DeleteNotificationRule`, `DeleteTarget`
- Common required input members in this group: `Arn`, `TargetAddress`

### Create

- Operations: `CreateNotificationRule`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DetailType`, `EventTypeIds`, `Name`, `Resource`, `Targets`

### Describe

- Operations: `DescribeNotificationRule`
- Common required input members in this group: `Arn`

### Subscribe

- Operations: `Subscribe`
- Common required input members in this group: `Arn`, `Target`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Arn`, `Tags`

### Unsubscribe

- Operations: `Unsubscribe`
- Common required input members in this group: `Arn`, `TargetAddress`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `Arn`, `TagKeys`

### Update

- Operations: `UpdateNotificationRule`
- Common required input members in this group: `Arn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateNotificationRule` | `POST /createNotificationRule` | `idempotency-token` | `DetailType`, `EventTypeIds`, `Name`, `Resource`, `Targets` | `ClientRequestToken` | `CreateNotificationRuleResult` | `AccessDeniedException`, `ConcurrentModificationException`, `ConfigurationException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ValidationException` | Creates a notification rule for a resource. The rule specifies the events you want notifications about and the targets (such as Amazon Q Developer in chat applications topics or Amazon Q Developer in chat applications clients configured for Slack) where you... |
| `DeleteNotificationRule` | `POST /deleteNotificationRule` | - | `Arn` | - | `DeleteNotificationRuleResult` | `ConcurrentModificationException`, `LimitExceededException`, `ValidationException` | Deletes a notification rule for a resource. |
| `DeleteTarget` | `POST /deleteTarget` | - | `TargetAddress` | - | `DeleteTargetResult` | `ValidationException` | Deletes a specified target for notifications. |
| `DescribeNotificationRule` | `POST /describeNotificationRule` | - | `Arn` | - | `DescribeNotificationRuleResult` | `ResourceNotFoundException`, `ValidationException` | Returns information about a specified notification rule. |
| `ListEventTypes` | `POST /listEventTypes` | `paginated` | - | - | `ListEventTypesResult` | `InvalidNextTokenException`, `ValidationException` | Returns information about the event types available for configuring notifications. |
| `ListNotificationRules` | `POST /listNotificationRules` | `paginated` | - | - | `ListNotificationRulesResult` | `InvalidNextTokenException`, `ValidationException` | Returns a list of the notification rules for an Amazon Web Services account. |
| `ListTagsForResource` | `POST /listTagsForResource` | - | `Arn` | - | `ListTagsForResourceResult` | `ResourceNotFoundException`, `ValidationException` | Returns a list of the tags associated with a notification rule. |
| `ListTargets` | `POST /listTargets` | `paginated` | - | - | `ListTargetsResult` | `InvalidNextTokenException`, `ValidationException` | Returns a list of the notification rule targets for an Amazon Web Services account. |
| `Subscribe` | `POST /subscribe` | - | `Arn`, `Target` | - | `SubscribeResult` | `ConfigurationException`, `ResourceNotFoundException`, `ValidationException` | Creates an association between a notification rule and an Amazon Q Developer in chat applications topic or Amazon Q Developer in chat applications client so that the associated target can receive notifications when the events described in the rule are... |
| `TagResource` | `POST /tagResource` | - | `Arn`, `Tags` | - | `TagResourceResult` | `ConcurrentModificationException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Associates a set of provided tags with a notification rule. |
| `Unsubscribe` | `POST /unsubscribe` | - | `Arn`, `TargetAddress` | - | `UnsubscribeResult` | `ValidationException` | Removes an association between a notification rule and an Amazon Q Developer in chat applications topic so that subscribers to that topic stop receiving notifications when the events described in the rule are triggered. |
| `UntagResource` | `POST /untagResource/{Arn}` | - | `Arn`, `TagKeys` | - | `UntagResourceResult` | `ConcurrentModificationException`, `LimitExceededException`, `ResourceNotFoundException`, `ValidationException` | Removes the association between one or more provided tags and a notification rule. |
| `UpdateNotificationRule` | `POST /updateNotificationRule` | - | `Arn` | - | `UpdateNotificationRuleResult` | `ConfigurationException`, `ResourceNotFoundException`, `ValidationException` | Updates a notification rule for a resource. You can change the events that trigger the notification rule, the status of the rule, and the targets that receive the notifications. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | One or more parameter values are not valid. |
| `ResourceNotFoundException` | `structure` | `Message` | CodeStar Notifications can't find a resource that matches the provided ARN. |
| `ConcurrentModificationException` | `structure` | `Message` | CodeStar Notifications can't complete the request because the resource is being modified by another process. |
| `LimitExceededException` | `structure` | `Message` | One of the CodeStar Notifications limits has been exceeded. |
| `ConfigurationException` | `structure` | `Message` | Some or all of the configuration is incomplete, missing, or not valid. |
| `InvalidNextTokenException` | `structure` | `Message` | The value for the enumeration token used in the request to return the next batch of the results is not valid. |
| `CreateNotificationRuleRequest` | `structure` | `ClientRequestToken`, `DetailType`, `EventTypeIds`, `Name`, `Resource`, `Status`, `Tags`, `Targets` | - |
| `CreateNotificationRuleResult` | `structure` | `Arn` | - |
| `AccessDeniedException` | `structure` | `Message` | CodeStar Notifications can't create the notification rule because you do not have sufficient permissions. |
| `ResourceAlreadyExistsException` | `structure` | `Message` | A resource with the same name or ID already exists. |
| `DeleteNotificationRuleRequest` | `structure` | `Arn` | - |
| `DeleteNotificationRuleResult` | `structure` | `Arn` | - |
| `DeleteTargetRequest` | `structure` | `ForceUnsubscribeAll`, `TargetAddress` | - |
| `DeleteTargetResult` | `structure` | - | - |
| `DescribeNotificationRuleRequest` | `structure` | `Arn` | - |
| `DescribeNotificationRuleResult` | `structure` | `Arn`, `CreatedBy`, `CreatedTimestamp`, `DetailType`, `EventTypes`, `LastModifiedTimestamp`, `Name`, `Resource`, `Status`, `Tags`, `Targets` | - |
| `ListEventTypesRequest` | `structure` | `Filters`, `MaxResults`, `NextToken` | - |
| `ListEventTypesResult` | `structure` | `EventTypes`, `NextToken` | - |
| `ListNotificationRulesRequest` | `structure` | `Filters`, `MaxResults`, `NextToken` | - |
| `ListNotificationRulesResult` | `structure` | `NextToken`, `NotificationRules` | - |
| `ListTagsForResourceRequest` | `structure` | `Arn` | - |
| `ListTagsForResourceResult` | `structure` | `Tags` | - |
| `ListTargetsRequest` | `structure` | `Filters`, `MaxResults`, `NextToken` | - |
| `ListTargetsResult` | `structure` | `NextToken`, `Targets` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

# AWS Budgets

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use the Amazon Web Services Budgets API to plan your service usage, service costs, and instance reservations. This API reference provides descriptions, syntax, and usage examples for each of the actions and data types for the Amazon Web Services Budgets feature. Budgets provide you with a way to see the following information: How close your plan is to your budgeted amount or to the free tier limits Your usage-to-date, including how much you've used of your Reserved Instances (RIs) Your current estimated charges from Amazon Web Services, and how much your predicted usage will accrue in charges by the end of the month How much of your budget has been used Amazon Web Services updates your budget status several times a day. Budgets track your unblended costs, subscriptions, refunds, and RIs. You can create the following types of budgets: Cost budgets - Plan how much you want to spend on a service.

## Possible Usage Scenarios
- From the AWS documentation and model: create budgets, actions, notifications, subscribers, and budget reports for cost governance.
- From the operation surface: model threshold alerts, budget action approval/execution, historical performance lookup, and account-level budget lifecycle management.

## Service Identity and Protocol

- AWS model slug: `budgets`
- AWS SDK for Rust slug: `budgets`
- Model version: `2016-10-20`
- Model file: `vendor/api-models-aws/models/budgets/service/2016-10-20/budgets-2016-10-20.json`
- SDK ID: `Budgets`
- Endpoint prefix: `budgets`
- ARN namespace: `budgets`
- CloudFormation name: `Budgets`
- CloudTrail event source: `budgets.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (10), `Create` (4), `Delete` (4), `Update` (4), `Execute` (1), `List` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateBudget`, `CreateBudgetAction`, `CreateNotification`, `CreateSubscriber`, `DeleteBudget`, `DeleteBudgetAction`, `DeleteNotification`, `DeleteSubscriber`, `TagResource`, `UntagResource`, `UpdateBudget`, `UpdateBudgetAction`, `UpdateNotification`, `UpdateSubscriber`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBudget`, `DescribeBudgetAction`, `DescribeBudgetActionHistories`, `DescribeBudgetActionsForAccount`, `DescribeBudgetActionsForBudget`, `DescribeBudgetNotificationsForAccount`, `DescribeBudgetPerformanceHistory`, `DescribeBudgets`, `DescribeNotificationsForBudget`, `DescribeSubscribersForNotification`, `ListTagsForResource`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `SNS`, `EC2/VPC`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-managing-costs.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-create.html
- https://docs.aws.amazon.com/savingsplans/latest/userguide/sp-create-savingsplans-budget.html

Research outcomes:
- AWS Budgets tracks cost, usage, Reserved Instance utilisation or coverage, and Savings Plans utilisation or coverage against user-defined thresholds.
- Budget periods and amounts define the monitored target, with filters and cost types controlling which charges or usage are included.
- Budgets can send notifications through email and Amazon SNS when actual or forecasted values cross thresholds.
- Budgets can be scoped to billing views and, for management accounts, can monitor cross-account data.
- Budget actions can apply IAM policies, SCPs, or target running resources when configured and approved.
- Savings Plans budgets can track utilisation or coverage thresholds and notify email or SNS subscribers.
- Budgets are strongly tied to Cost Explorer and Billing data, so reported values reflect billing data availability rather than immediate resource changes.

Parity implications:
- Model budgets, budget types, cost filters, cost types, time periods, notifications, subscribers, actions, action execution state, and forecast/actual metrics separately.
- Notification evaluation should distinguish actual and forecasted threshold types.
- Budget action execution should be stateful and approval-aware rather than an immediate side effect of creating the budget.

## Operation Groups

### Describe

- Operations: `DescribeBudget`, `DescribeBudgetAction`, `DescribeBudgetActionHistories`, `DescribeBudgetActionsForAccount`, `DescribeBudgetActionsForBudget`, `DescribeBudgetNotificationsForAccount`, `DescribeBudgetPerformanceHistory`, `DescribeBudgets`, `DescribeNotificationsForBudget`, `DescribeSubscribersForNotification`
- Traits: `paginated` (8)
- Common required input members in this group: `AccountId`, `ActionId`, `BudgetName`, `Notification`

### Create

- Operations: `CreateBudget`, `CreateBudgetAction`, `CreateNotification`, `CreateSubscriber`
- Common required input members in this group: `AccountId`, `ActionThreshold`, `ActionType`, `ApprovalModel`, `Budget`, `BudgetName`, `Definition`, `ExecutionRoleArn`, `Notification`, `NotificationType`, `Subscriber`, `Subscribers`

### Delete

- Operations: `DeleteBudget`, `DeleteBudgetAction`, `DeleteNotification`, `DeleteSubscriber`
- Common required input members in this group: `AccountId`, `ActionId`, `BudgetName`, `Notification`, `Subscriber`

### Update

- Operations: `UpdateBudget`, `UpdateBudgetAction`, `UpdateNotification`, `UpdateSubscriber`
- Common required input members in this group: `AccountId`, `ActionId`, `BudgetName`, `NewBudget`, `NewNotification`, `NewSubscriber`, `Notification`, `OldNotification`, `OldSubscriber`

### Execute

- Operations: `ExecuteBudgetAction`
- Common required input members in this group: `AccountId`, `ActionId`, `BudgetName`, `ExecutionType`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `ResourceARN`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `ResourceTags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `ResourceTagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateBudget` | - | - | `AccountId`, `Budget` | - | `CreateBudgetResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `CreationLimitExceededException`, `DuplicateRecordException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ServiceQuotaExceededException`, ... (+1) | Creates a budget and, if included, notifications and subscribers. Only one of `BudgetLimit` or `PlannedBudgetLimits` can be present in the syntax at one time. |
| `CreateBudgetAction` | - | - | `AccountId`, `ActionThreshold`, `ActionType`, `ApprovalModel`, `BudgetName`, `Definition`, `ExecutionRoleArn`, `NotificationType`, `Subscribers` | - | `CreateBudgetActionResponse` | `AccessDeniedException`, `CreationLimitExceededException`, `DuplicateRecordException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Creates a budget action. |
| `CreateNotification` | - | - | `AccountId`, `BudgetName`, `Notification`, `Subscribers` | - | `CreateNotificationResponse` | `AccessDeniedException`, `CreationLimitExceededException`, `DuplicateRecordException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Creates a notification. You must create the budget before you create the associated notification. |
| `CreateSubscriber` | - | - | `AccountId`, `BudgetName`, `Notification`, `Subscriber` | - | `CreateSubscriberResponse` | `AccessDeniedException`, `CreationLimitExceededException`, `DuplicateRecordException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Creates a subscriber. You must create the associated budget and notification before you create the subscriber. |
| `DeleteBudget` | - | - | `AccountId`, `BudgetName` | - | `DeleteBudgetResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Deletes a budget. You can delete your budget at any time. |
| `DeleteBudgetAction` | - | - | `AccountId`, `ActionId`, `BudgetName` | - | `DeleteBudgetActionResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ResourceLockedException`, `ThrottlingException` | Deletes a budget action. |
| `DeleteNotification` | - | - | `AccountId`, `BudgetName`, `Notification` | - | `DeleteNotificationResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Deletes a notification. Deleting a notification also deletes the subscribers that are associated with the notification. |
| `DeleteSubscriber` | - | - | `AccountId`, `BudgetName`, `Notification`, `Subscriber` | - | `DeleteSubscriberResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Deletes a subscriber. Deleting the last subscriber to a notification also deletes the notification. |
| `DescribeBudget` | - | - | `AccountId`, `BudgetName` | - | `DescribeBudgetResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Describes a budget. The Request Syntax section shows the `BudgetLimit` syntax. |
| `DescribeBudgetAction` | - | - | `AccountId`, `ActionId`, `BudgetName` | - | `DescribeBudgetActionResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Describes a budget action detail. |
| `DescribeBudgetActionHistories` | - | `paginated` | `AccountId`, `ActionId`, `BudgetName` | - | `DescribeBudgetActionHistoriesResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Describes a budget action history detail. |
| `DescribeBudgetActionsForAccount` | - | `paginated` | `AccountId` | - | `DescribeBudgetActionsForAccountResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `ThrottlingException` | Describes all of the budget actions for an account. |
| `DescribeBudgetActionsForBudget` | - | `paginated` | `AccountId`, `BudgetName` | - | `DescribeBudgetActionsForBudgetResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Describes all of the budget actions for a budget. |
| `DescribeBudgetNotificationsForAccount` | - | `paginated` | `AccountId` | - | `DescribeBudgetNotificationsForAccountResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Lists the budget names and notifications that are associated with an account. |
| `DescribeBudgetPerformanceHistory` | - | `paginated` | `AccountId`, `BudgetName` | - | `DescribeBudgetPerformanceHistoryResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Describes the history for `DAILY`, `MONTHLY`, and `QUARTERLY` budgets. Budget history isn't available for `ANNUAL` budgets. |
| `DescribeBudgets` | - | `paginated` | `AccountId` | - | `DescribeBudgetsResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Lists the budgets that are associated with an account. The Request Syntax section shows the `BudgetLimit` syntax. |
| `DescribeNotificationsForBudget` | - | `paginated` | `AccountId`, `BudgetName` | - | `DescribeNotificationsForBudgetResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Lists the notifications that are associated with a budget. |
| `DescribeSubscribersForNotification` | - | `paginated` | `AccountId`, `BudgetName`, `Notification` | - | `DescribeSubscribersForNotificationResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Lists the subscribers that are associated with a notification. |
| `ExecuteBudgetAction` | - | - | `AccountId`, `ActionId`, `BudgetName`, `ExecutionType` | - | `ExecuteBudgetActionResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ResourceLockedException`, `ThrottlingException` | Executes a budget action. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Lists tags associated with a budget or budget action resource. |
| `TagResource` | - | - | `ResourceARN`, `ResourceTags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Creates tags for a budget or budget action resource. |
| `UntagResource` | - | - | `ResourceARN`, `ResourceTagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Deletes tags associated with a budget or budget action resource. |
| `UpdateBudget` | - | - | `AccountId`, `NewBudget` | - | `UpdateBudgetResponse` | `AccessDeniedException`, `BillingViewHealthStatusException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Updates a budget. You can change every part of a budget except for the `budgetName` and the `calculatedSpend`. |
| `UpdateBudgetAction` | - | - | `AccountId`, `ActionId`, `BudgetName` | - | `UpdateBudgetActionResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ResourceLockedException`, `ThrottlingException` | Updates a budget action. |
| `UpdateNotification` | - | - | `AccountId`, `BudgetName`, `NewNotification`, `OldNotification` | - | `UpdateNotificationResponse` | `AccessDeniedException`, `DuplicateRecordException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Updates a notification. |
| `UpdateSubscriber` | - | - | `AccountId`, `BudgetName`, `NewSubscriber`, `Notification`, `OldSubscriber` | - | `UpdateSubscriberResponse` | `AccessDeniedException`, `DuplicateRecordException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Updates a subscriber. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You are not authorized to use this operation with the given parameters. |
| `InternalErrorException` | `structure` | `Message` | An error on the server occurred during the processing of your request. |
| `InvalidParameterException` | `structure` | `Message` | An error on the client occurred. |
| `ThrottlingException` | `structure` | `Message` | The number of API requests has exceeded the maximum allowed API request throttling limit for the account. |
| `NotFoundException` | `structure` | `Message` | We can’t locate the resource that you specified. |
| `InvalidNextTokenException` | `structure` | `Message` | The pagination token is invalid. |
| `DuplicateRecordException` | `structure` | `Message` | The budget name already exists. |
| `ExpiredNextTokenException` | `structure` | `Message` | The pagination token expired. |
| `CreationLimitExceededException` | `structure` | `Message` | You've exceeded the notification or subscriber limit. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You've reached a Service Quota limit on this resource. |
| `BillingViewHealthStatusException` | `structure` | `Message` | The billing view status must be HEALTHY to perform this action. |
| `ResourceLockedException` | `structure` | `Message` | The request was received and recognized by the server, but the server rejected that particular method for the requested resource. |
| `CreateBudgetRequest` | `structure` | `AccountId`, `Budget`, `NotificationsWithSubscribers`, `ResourceTags` | Request of CreateBudget |
| `CreateBudgetResponse` | `structure` | - | Response of CreateBudget |
| `CreateBudgetActionRequest` | `structure` | `AccountId`, `ActionThreshold`, `ActionType`, `ApprovalModel`, `BudgetName`, `Definition`, `ExecutionRoleArn`, `NotificationType`, `ResourceTags`, `Subscribers` | - |
| `CreateBudgetActionResponse` | `structure` | `AccountId`, `ActionId`, `BudgetName` | - |
| `CreateNotificationRequest` | `structure` | `AccountId`, `BudgetName`, `Notification`, `Subscribers` | Request of CreateNotification |
| `CreateNotificationResponse` | `structure` | - | Response of CreateNotification |
| `CreateSubscriberRequest` | `structure` | `AccountId`, `BudgetName`, `Notification`, `Subscriber` | Request of CreateSubscriber |
| `CreateSubscriberResponse` | `structure` | - | Response of CreateSubscriber |
| `DeleteBudgetRequest` | `structure` | `AccountId`, `BudgetName` | Request of DeleteBudget |
| `DeleteBudgetResponse` | `structure` | - | Response of DeleteBudget |
| `DeleteBudgetActionRequest` | `structure` | `AccountId`, `ActionId`, `BudgetName` | - |
| `DeleteBudgetActionResponse` | `structure` | `AccountId`, `Action`, `BudgetName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

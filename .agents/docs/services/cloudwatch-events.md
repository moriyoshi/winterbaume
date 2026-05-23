# Amazon CloudWatch Events

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EventBridge helps you to respond to state changes in your Amazon Web Services resources. When your resources change state, they automatically send events to an event stream. You can create rules that match selected events in the stream and route them to targets to take action. You can also use rules to take action on a predetermined schedule. For example, you can configure rules to: Automatically invoke an Lambda function to update DNS entries when an event notifies you that Amazon EC2 instance enters the running state.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon CloudWatch Events where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon CloudWatch Events by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon CloudWatch Events workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Delete`, `Create`, `Put` operation families, including `ListApiDestinations`, `ListArchives`, `ListConnections`, `ListEventBuses`, `DescribeApiDestination`, `DescribeArchive`.

## Service Identity and Protocol

- AWS model slug: `cloudwatch-events`
- AWS SDK for Rust slug: `cloudwatchevents`
- Model version: `2015-10-07`
- Model file: `vendor/api-models-aws/models/cloudwatch-events/service/2015-10-07/cloudwatch-events-2015-10-07.json`
- SDK ID: `CloudWatch Events`
- Endpoint prefix: `events`
- ARN namespace: `events`
- CloudFormation name: `Events`
- CloudTrail event source: `events.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Describe` (8), `Delete` (6), `Create` (5), `Put` (5), `Update` (3), `Remove` (2), `Activate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelReplay`, `CreateApiDestination`, `CreateArchive`, `CreateConnection`, `CreateEventBus`, `CreatePartnerEventSource`, `DeleteApiDestination`, `DeleteArchive`, `DeleteConnection`, `DeleteEventBus`, `DeletePartnerEventSource`, `DeleteRule`, `DisableRule`, `EnableRule`, `PutEvents`, `PutPartnerEvents`, `PutPermission`, `PutRule`, `PutTargets`, `RemovePermission`, ... (+7).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApiDestination`, `DescribeArchive`, `DescribeConnection`, `DescribeEventBus`, `DescribeEventSource`, `DescribePartnerEventSource`, `DescribeReplay`, `DescribeRule`, `ListApiDestinations`, `ListArchives`, `ListConnections`, `ListEventBuses`, `ListEventSources`, `ListPartnerEventSourceAccounts`, `ListPartnerEventSources`, `ListReplays`, `ListRuleNamesByTarget`, `ListRules`, `ListTagsForResource`, `ListTargetsByRule`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelReplay`, `StartReplay`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 51 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SQS`, `Lambda`, `EC2/VPC`, `ECS`, `Redshift`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rule-retry-policy.html
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rule-dlq.html

Research outcomes:
- EventBridge event patterns match event source fields, event metadata, and event detail fields on an event bus.
- EventBridge retries target delivery for retriable errors. Defaults are a 24-hour maximum event age and up to 185 attempts with exponential backoff and jitter.
- If all retry attempts are exhausted, EventBridge drops the event unless a dead-letter queue is configured.
- EventBridge rule dead-letter queues support standard SQS queues, not FIFO queues, and the queue must be in the same Region as the rule.
- DLQ messages include attributes such as error code, error message, exhausted retry condition, rule ARN, retry attempts, and target ARN.
- The console can add the SQS resource policy for same-account DLQs. API users and cross-account DLQs must attach an SQS policy that allows `sqs:SendMessage` from the rule or event bus source ARN.

Parity implications:
- Model event buses, rules, event patterns, targets, retry policy, target delivery attempts, DLQ configuration, and SQS policy dependencies separately.
- PutEvents should evaluate rule patterns against event metadata and detail fields, then deliver matching events to targets.
- Delivery failure handling should include retry exhaustion, event-age expiry, DLQ attributes, and silent drop when no DLQ exists.

## Operation Groups

### List

- Operations: `ListApiDestinations`, `ListArchives`, `ListConnections`, `ListEventBuses`, `ListEventSources`, `ListPartnerEventSourceAccounts`, `ListPartnerEventSources`, `ListReplays`, `ListRuleNamesByTarget`, `ListRules`, `ListTagsForResource`, `ListTargetsByRule`
- Common required input members in this group: `EventSourceName`, `NamePrefix`, `ResourceARN`, `Rule`, `TargetArn`

### Describe

- Operations: `DescribeApiDestination`, `DescribeArchive`, `DescribeConnection`, `DescribeEventBus`, `DescribeEventSource`, `DescribePartnerEventSource`, `DescribeReplay`, `DescribeRule`
- Common required input members in this group: `ArchiveName`, `Name`, `ReplayName`

### Delete

- Operations: `DeleteApiDestination`, `DeleteArchive`, `DeleteConnection`, `DeleteEventBus`, `DeletePartnerEventSource`, `DeleteRule`
- Common required input members in this group: `Account`, `ArchiveName`, `Name`

### Create

- Operations: `CreateApiDestination`, `CreateArchive`, `CreateConnection`, `CreateEventBus`, `CreatePartnerEventSource`
- Common required input members in this group: `Account`, `ArchiveName`, `AuthParameters`, `AuthorizationType`, `ConnectionArn`, `EventSourceArn`, `HttpMethod`, `InvocationEndpoint`, `Name`

### Put

- Operations: `PutEvents`, `PutPartnerEvents`, `PutPermission`, `PutRule`, `PutTargets`
- Common required input members in this group: `Entries`, `Name`, `Rule`, `Targets`

### Update

- Operations: `UpdateApiDestination`, `UpdateArchive`, `UpdateConnection`
- Common required input members in this group: `ArchiveName`, `Name`

### Remove

- Operations: `RemovePermission`, `RemoveTargets`
- Common required input members in this group: `Ids`, `Rule`

### Activate

- Operations: `ActivateEventSource`
- Common required input members in this group: `Name`

### Cancel

- Operations: `CancelReplay`
- Common required input members in this group: `ReplayName`

### Deactivate

- Operations: `DeactivateEventSource`
- Common required input members in this group: `Name`

### Deauthorize

- Operations: `DeauthorizeConnection`
- Common required input members in this group: `Name`

### Disable

- Operations: `DisableRule`
- Common required input members in this group: `Name`

### Enable

- Operations: `EnableRule`
- Common required input members in this group: `Name`

### Start

- Operations: `StartReplay`
- Common required input members in this group: `Destination`, `EventEndTime`, `EventSourceArn`, `EventStartTime`, `ReplayName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Test

- Operations: `TestEventPattern`
- Common required input members in this group: `Event`, `EventPattern`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateEventSource` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `InvalidStateException`, `OperationDisabledException`, `ResourceNotFoundException` | Activates a partner event source that has been deactivated. Once activated, your matching event bus will start receiving events from the event source. |
| `CancelReplay` | - | - | `ReplayName` | - | `CancelReplayResponse` | `ConcurrentModificationException`, `IllegalStatusException`, `InternalException`, `ResourceNotFoundException` | Cancels the specified replay. |
| `CreateApiDestination` | - | - | `ConnectionArn`, `HttpMethod`, `InvocationEndpoint`, `Name` | - | `CreateApiDestinationResponse` | `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates an API destination, which is an HTTP invocation endpoint configured as a target for events. |
| `CreateArchive` | - | - | `ArchiveName`, `EventSourceArn` | - | `CreateArchiveResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates an archive of events with the specified settings. When you create an archive, incoming events might not immediately start being sent to the archive. |
| `CreateConnection` | - | - | `AuthParameters`, `AuthorizationType`, `Name` | - | `CreateConnectionResponse` | `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates a connection. A connection defines the authorization type and credentials to use for authorization with an API destination HTTP endpoint. |
| `CreateEventBus` | - | - | `Name` | - | `CreateEventBusResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidStateException`, `LimitExceededException`, `OperationDisabledException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a new event bus within your account. This can be a custom event bus which you can use to receive events from your custom applications and services, or it can be a partner event bus which can be matched to a partner event source. |
| `CreatePartnerEventSource` | - | - | `Account`, `Name` | - | `CreatePartnerEventSourceResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `OperationDisabledException`, `ResourceAlreadyExistsException` | Called by an SaaS partner to create a partner event source. This operation is not used by Amazon Web Services customers. |
| `DeactivateEventSource` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `InvalidStateException`, `OperationDisabledException`, `ResourceNotFoundException` | You can use this operation to temporarily stop receiving events from the specified partner event source. The matching event bus is not deleted. |
| `DeauthorizeConnection` | - | - | `Name` | - | `DeauthorizeConnectionResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Removes all authorization parameters from the connection. This lets you remove the secret from the connection so you can reuse it without having to create a new connection. |
| `DeleteApiDestination` | - | - | `Name` | - | `DeleteApiDestinationResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Deletes the specified API destination. |
| `DeleteArchive` | - | - | `ArchiveName` | - | `DeleteArchiveResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Deletes the specified archive. |
| `DeleteConnection` | - | - | `Name` | - | `DeleteConnectionResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Deletes a connection. |
| `DeleteEventBus` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException` | Deletes the specified custom event bus or partner event bus. All rules associated with this event bus need to be deleted. |
| `DeletePartnerEventSource` | - | - | `Account`, `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException` | This operation is used by SaaS partners to delete a partner event source. This operation is not used by Amazon Web Services customers. |
| `DeleteRule` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Deletes the specified rule. Before you can delete the rule, you must remove all targets, using RemoveTargets. |
| `DescribeApiDestination` | - | - | `Name` | - | `DescribeApiDestinationResponse` | `InternalException`, `ResourceNotFoundException` | Retrieves details about an API destination. |
| `DescribeArchive` | - | - | `ArchiveName` | - | `DescribeArchiveResponse` | `InternalException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Retrieves details about an archive. |
| `DescribeConnection` | - | - | `Name` | - | `DescribeConnectionResponse` | `InternalException`, `ResourceNotFoundException` | Retrieves details about a connection. |
| `DescribeEventBus` | - | - | - | - | `DescribeEventBusResponse` | `InternalException`, `ResourceNotFoundException` | Displays details about an event bus in your account. This can include the external Amazon Web Services accounts that are permitted to write events to your default event bus, and the associated policy. |
| `DescribeEventSource` | - | - | `Name` | - | `DescribeEventSourceResponse` | `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | This operation lists details about a partner event source that is shared with your account. |
| `DescribePartnerEventSource` | - | - | `Name` | - | `DescribePartnerEventSourceResponse` | `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | An SaaS partner can use this operation to list details about a partner event source that they have created. Amazon Web Services customers do not use this operation. |
| `DescribeReplay` | - | - | `ReplayName` | - | `DescribeReplayResponse` | `InternalException`, `ResourceNotFoundException` | Retrieves details about a replay. Use `DescribeReplay` to determine the progress of a running replay. |
| `DescribeRule` | - | - | `Name` | - | `DescribeRuleResponse` | `InternalException`, `ResourceNotFoundException` | Describes the specified rule. DescribeRule does not list the targets of a rule. |
| `DisableRule` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression. |
| `EnableRule` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Enables the specified rule. If the rule does not exist, the operation fails. |
| `ListApiDestinations` | - | - | - | - | `ListApiDestinationsResponse` | `InternalException` | Retrieves a list of API destination in the account in the current Region. |
| `ListArchives` | - | - | - | - | `ListArchivesResponse` | `InternalException`, `ResourceNotFoundException` | Lists your archives. You can either list all the archives or you can provide a prefix to match to the archive names. |
| `ListConnections` | - | - | - | - | `ListConnectionsResponse` | `InternalException` | Retrieves a list of connections from the account. |
| `ListEventBuses` | - | - | - | - | `ListEventBusesResponse` | `InternalException` | Lists all the event buses in your account, including the default event bus, custom event buses, and partner event buses. |
| `ListEventSources` | - | - | - | - | `ListEventSourcesResponse` | `InternalException`, `OperationDisabledException` | You can use this to see all the partner event sources that have been shared with your Amazon Web Services account. For more information about partner event sources, see CreateEventBus. |
| `ListPartnerEventSourceAccounts` | - | - | `EventSourceName` | - | `ListPartnerEventSourceAccountsResponse` | `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | An SaaS partner can use this operation to display the Amazon Web Services account ID that a particular partner event source name is associated with. This operation is not used by Amazon Web Services customers. |
| `ListPartnerEventSources` | - | - | `NamePrefix` | - | `ListPartnerEventSourcesResponse` | `InternalException`, `OperationDisabledException` | An SaaS partner can use this operation to list all the partner event source names that they have created. This operation is not used by Amazon Web Services customers. |
| `ListReplays` | - | - | - | - | `ListReplaysResponse` | `InternalException` | Lists your replays. You can either list all the replays or you can provide a prefix to match to the replay names. |
| `ListRuleNamesByTarget` | - | - | `TargetArn` | - | `ListRuleNamesByTargetResponse` | `InternalException`, `ResourceNotFoundException` | Lists the rules for the specified target. You can see which of the rules in Amazon EventBridge can invoke a specific target in your account. |
| `ListRules` | - | - | - | - | `ListRulesResponse` | `InternalException`, `ResourceNotFoundException` | Lists your Amazon EventBridge rules. You can either list all the rules or you can provide a prefix to match to the rule names. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InternalException`, `ResourceNotFoundException` | Displays the tags associated with an EventBridge resource. In EventBridge, rules and event buses can be tagged. |
| `ListTargetsByRule` | - | - | `Rule` | - | `ListTargetsByRuleResponse` | `InternalException`, `ResourceNotFoundException` | Lists the targets assigned to the specified rule. |
| `PutEvents` | - | - | `Entries` | - | `PutEventsResponse` | `InternalException` | Sends custom events to Amazon EventBridge so that they can be matched to rules. |
| `PutPartnerEvents` | - | - | `Entries` | - | `PutPartnerEventsResponse` | `InternalException`, `OperationDisabledException` | This is used by SaaS partners to write events to a customer's partner event bus. Amazon Web Services customers do not use this operation. |
| `PutPermission` | - | - | - | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException`, `PolicyLengthExceededException`, `ResourceNotFoundException` | Running `PutPermission` permits the specified Amazon Web Services account or Amazon Web Services organization to put events to the specified event bus . Amazon EventBridge (CloudWatch Events) rules in your account are triggered by these events arriving to an... |
| `PutRule` | - | - | `Name` | - | `PutRuleResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ManagedRuleException`, `ResourceNotFoundException` | Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. |
| `PutTargets` | - | - | `Rule`, `Targets` | - | `PutTargetsResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `ManagedRuleException`, `ResourceNotFoundException` | Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule. Targets are the resources that are invoked when a rule is triggered. |
| `RemovePermission` | - | - | - | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | Revokes the permission of another Amazon Web Services account to be able to put events to the specified event bus. Specify the account to revoke by the `StatementId` value that you associated with the account when you granted it permission with... |
| `RemoveTargets` | - | - | `Ids`, `Rule` | - | `RemoveTargetsResponse` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked. |
| `StartReplay` | - | - | `Destination`, `EventEndTime`, `EventSourceArn`, `EventStartTime`, `ReplayName` | - | `StartReplayResponse` | `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Starts the specified replay. Events are not necessarily replayed in the exact same order that they were added to the archive. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Assigns one or more tags (key-value pairs) to the specified EventBridge resource. Tags can help you organize and categorize your resources. |
| `TestEventPattern` | - | - | `Event`, `EventPattern` | - | `TestEventPatternResponse` | `InternalException`, `InvalidEventPatternException` | Tests whether the specified event pattern matches the provided event. Most services in Amazon Web Services treat : or / as the same character in Amazon Resource Names (ARNs). |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Removes one or more tags from the specified EventBridge resource. In Amazon EventBridge (CloudWatch Events), rules and event buses can be tagged. |
| `UpdateApiDestination` | - | - | `Name` | - | `UpdateApiDestinationResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException` | Updates an API destination. |
| `UpdateArchive` | - | - | `ArchiveName` | - | `UpdateArchiveResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ResourceNotFoundException` | Updates the specified archive. |
| `UpdateConnection` | - | - | `Name` | - | `UpdateConnectionResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException` | Updates settings for a connection. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | `message` | This exception occurs due to unexpected causes. |
| `ResourceNotFoundException` | `structure` | `message` | An entity that you specified does not exist. |
| `ConcurrentModificationException` | `structure` | `message` | There is concurrent modification on a rule, target, archive, or replay. |
| `OperationDisabledException` | `structure` | `message` | The operation you are attempting is not available in this region. |
| `LimitExceededException` | `structure` | `message` | The request failed because it attempted to create resource beyond the allowed service quota. |
| `ManagedRuleException` | `structure` | `message` | This rule was created by an Amazon Web Services service on behalf of your account. |
| `ResourceAlreadyExistsException` | `structure` | `message` | The resource you are trying to create already exists. |
| `InvalidEventPatternException` | `structure` | `message` | The event pattern is not valid. |
| `InvalidStateException` | `structure` | `message` | The specified state is not a valid state for an event source. |
| `ActivateEventSourceRequest` | `structure` | `Name` | - |
| `CancelReplayRequest` | `structure` | `ReplayName` | - |
| `CancelReplayResponse` | `structure` | `ReplayArn`, `State`, `StateReason` | - |
| `IllegalStatusException` | `structure` | `message` | An error occurred because a replay can be canceled only when the state is Running or Starting. |
| `CreateApiDestinationRequest` | `structure` | `ConnectionArn`, `Description`, `HttpMethod`, `InvocationEndpoint`, `InvocationRateLimitPerSecond`, `Name` | - |
| `CreateApiDestinationResponse` | `structure` | `ApiDestinationArn`, `ApiDestinationState`, `CreationTime`, `LastModifiedTime` | - |
| `CreateArchiveRequest` | `structure` | `ArchiveName`, `Description`, `EventPattern`, `EventSourceArn`, `RetentionDays` | - |
| `CreateArchiveResponse` | `structure` | `ArchiveArn`, `CreationTime`, `State`, `StateReason` | - |
| `CreateConnectionRequest` | `structure` | `AuthParameters`, `AuthorizationType`, `Description`, `Name` | - |
| `CreateConnectionResponse` | `structure` | `ConnectionArn`, `ConnectionState`, `CreationTime`, `LastModifiedTime` | - |
| `CreateEventBusRequest` | `structure` | `EventSourceName`, `Name`, `Tags` | - |
| `CreateEventBusResponse` | `structure` | `EventBusArn` | - |
| `CreatePartnerEventSourceRequest` | `structure` | `Account`, `Name` | - |
| `CreatePartnerEventSourceResponse` | `structure` | `EventSourceArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

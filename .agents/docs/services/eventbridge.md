# Amazon EventBridge

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EventBridge helps you to respond to state changes in your Amazon Web Services resources. When your resources change state, they automatically send events to an event stream. You can create rules that match selected events in the stream and route them to targets to take action. You can also use rules to take action on a predetermined schedule. For example, you can configure rules to: Automatically invoke an Lambda function to update DNS entries when an event notifies you that Amazon EC2 instance enters the running state.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-eventbridge/tests/scenario_test.rs`: create a custom event bus, add a rule and targets, publish events, inspect routing configuration, and clean up.
- Backported from `scenario_test.rs`: manage a connection and API destination lifecycle for external HTTP targets.
- Backported from `scenario_test.rs`: create an event archive, tag it, inspect it, and delete it.
- Scenario insight from EC2: include mutable binding failover for Amazon EventBridge where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon EventBridge by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon EventBridge resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support event-driven routing, partner/custom buses, rules, targets, archives/replays, schema integrations, API destinations, permissions, and tag-based resource management.

## Service Identity and Protocol

- AWS model slug: `eventbridge`
- AWS SDK for Rust slug: `eventbridge`
- Model version: `2015-10-07`
- Model file: `vendor/api-models-aws/models/eventbridge/service/2015-10-07/eventbridge-2015-10-07.json`
- SDK ID: `EventBridge`
- Endpoint prefix: `events`
- ARN namespace: `events`
- CloudFormation name: `Events`
- CloudTrail event source: `events.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `EndpointId`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (13), `Describe` (9), `Delete` (7), `Create` (6), `Put` (5), `Update` (5), `Remove` (2), `Activate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelReplay`, `CreateApiDestination`, `CreateArchive`, `CreateConnection`, `CreateEndpoint`, `CreateEventBus`, `CreatePartnerEventSource`, `DeleteApiDestination`, `DeleteArchive`, `DeleteConnection`, `DeleteEndpoint`, `DeleteEventBus`, `DeletePartnerEventSource`, `DeleteRule`, `DisableRule`, `EnableRule`, `PutEvents`, `PutPartnerEvents`, `PutPermission`, `PutRule`, ... (+11).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApiDestination`, `DescribeArchive`, `DescribeConnection`, `DescribeEndpoint`, `DescribeEventBus`, `DescribeEventSource`, `DescribePartnerEventSource`, `DescribeReplay`, `DescribeRule`, `ListApiDestinations`, `ListArchives`, `ListConnections`, `ListEndpoints`, `ListEventBuses`, `ListEventSources`, `ListPartnerEventSourceAccounts`, `ListPartnerEventSources`, `ListReplays`, `ListRuleNamesByTarget`, `ListRules`, ... (+2).
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelReplay`, `StartReplay`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 57 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`, `SQS`, `Lambda`, `EC2/VPC`, `ECS`, `Redshift`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-what-is-how-it-works-concepts.html
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rule-retry-policy.html
- https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-targets.html

Research outcomes:
- An event bus routes JSON events from sources to zero or more targets. Accounts have a default event bus and can create custom and partner event buses.
- Rules are scoped to a specific event bus and match either event patterns or schedules. A single rule can send an event to up to five targets.
- Event patterns can match event metadata, event data properties, and event content values.
- EventBridge can transform target input before delivery.
- Failed delivery to targets is retried for retriable errors. By default, EventBridge retries for 24 hours and up to 185 attempts using exponential backoff with jitter.
- If delivery still fails after retries, the event is dropped unless a dead-letter queue is configured.
- Archives and replays are first-class event bus features and can be used to replay stored events later.

Parity implications:
- Model event buses, rules, targets, input transforms, retry policy, DLQ configuration, archive, and replay as separate state.
- Rule matching needs event-pattern semantics, not substring matching.
- Target delivery should distinguish retriable failures, exhausted retries, and DLQ handoff.

## Cross-Service Integration Gaps

- **`eventbridge-targets`** ( primary ): `put_events()` generates event IDs but does not match events against rules or invoke any targets ( Lambda, SQS, SNS, Step Functions, ECS, Batch, API Gateway, CloudWatch Logs, AppSync, Kinesis, Firehose ). Target metadata is stored via `PutTargets` but execution semantics are absent. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `eventbridge-targets` ).
- **Secondary partner of `eventbridge-pipes`**, `lambda-event-sources`, `sfn-execution`, `appsync-resolvers` — those gaps cite EventBridge as either a source or a target service. See the same TODO section.

## Operation Groups

### List

- Operations: `ListApiDestinations`, `ListArchives`, `ListConnections`, `ListEndpoints`, `ListEventBuses`, `ListEventSources`, `ListPartnerEventSourceAccounts`, `ListPartnerEventSources`, `ListReplays`, `ListRuleNamesByTarget`, `ListRules`, `ListTagsForResource`, `ListTargetsByRule`
- Common required input members in this group: `EventSourceName`, `NamePrefix`, `ResourceARN`, `Rule`, `TargetArn`

### Describe

- Operations: `DescribeApiDestination`, `DescribeArchive`, `DescribeConnection`, `DescribeEndpoint`, `DescribeEventBus`, `DescribeEventSource`, `DescribePartnerEventSource`, `DescribeReplay`, `DescribeRule`
- Common required input members in this group: `ArchiveName`, `Name`, `ReplayName`

### Delete

- Operations: `DeleteApiDestination`, `DeleteArchive`, `DeleteConnection`, `DeleteEndpoint`, `DeleteEventBus`, `DeletePartnerEventSource`, `DeleteRule`
- Common required input members in this group: `Account`, `ArchiveName`, `Name`

### Create

- Operations: `CreateApiDestination`, `CreateArchive`, `CreateConnection`, `CreateEndpoint`, `CreateEventBus`, `CreatePartnerEventSource`
- Common required input members in this group: `Account`, `ArchiveName`, `AuthParameters`, `AuthorizationType`, `ConnectionArn`, `EventBuses`, `EventSourceArn`, `HttpMethod`, `InvocationEndpoint`, `Name`, `RoutingConfig`

### Put

- Operations: `PutEvents`, `PutPartnerEvents`, `PutPermission`, `PutRule`, `PutTargets`
- Common required input members in this group: `Entries`, `Name`, `Rule`, `Targets`

### Update

- Operations: `UpdateApiDestination`, `UpdateArchive`, `UpdateConnection`, `UpdateEndpoint`, `UpdateEventBus`
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
| `CreateApiDestination` | - | - | `ConnectionArn`, `HttpMethod`, `InvocationEndpoint`, `Name` | - | `CreateApiDestinationResponse` | `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates an API destination, which is an HTTP invocation endpoint configured as a target for events. API destinations do not support private destinations, such as interface VPC endpoints. |
| `CreateArchive` | - | - | `ArchiveName`, `EventSourceArn` | - | `CreateArchiveResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates an archive of events with the specified settings. When you create an archive, incoming events might not immediately start being sent to the archive. |
| `CreateConnection` | - | - | `AuthParameters`, `AuthorizationType`, `Name` | - | `CreateConnectionResponse` | `AccessDeniedException`, `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException`, `ThrottlingException` | Creates a connection. A connection defines the authorization type and credentials to use for authorization with an API destination HTTP endpoint. |
| `CreateEndpoint` | - | - | `EventBuses`, `Name`, `RoutingConfig` | - | `CreateEndpointResponse` | `InternalException`, `LimitExceededException`, `ResourceAlreadyExistsException` | Creates a global endpoint. Global endpoints improve your application's availability by making it regional-fault tolerant. |
| `CreateEventBus` | - | - | `Name` | - | `CreateEventBusResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidStateException`, `LimitExceededException`, `OperationDisabledException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a new event bus within your account. This can be a custom event bus which you can use to receive events from your custom applications and services, or it can be a partner event bus which can be matched to a partner event source. |
| `CreatePartnerEventSource` | - | - | `Account`, `Name` | - | `CreatePartnerEventSourceResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `OperationDisabledException`, `ResourceAlreadyExistsException` | Called by an SaaS partner to create a partner event source. This operation is not used by Amazon Web Services customers. |
| `DeactivateEventSource` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `InvalidStateException`, `OperationDisabledException`, `ResourceNotFoundException` | You can use this operation to temporarily stop receiving events from the specified partner event source. The matching event bus is not deleted. |
| `DeauthorizeConnection` | - | - | `Name` | - | `DeauthorizeConnectionResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Removes all authorization parameters from the connection. This lets you remove the secret from the connection so you can reuse it without having to create a new connection. |
| `DeleteApiDestination` | - | - | `Name` | - | `DeleteApiDestinationResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Deletes the specified API destination. |
| `DeleteArchive` | - | - | `ArchiveName` | - | `DeleteArchiveResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Deletes the specified archive. |
| `DeleteConnection` | - | - | `Name` | - | `DeleteConnectionResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Deletes a connection. |
| `DeleteEndpoint` | - | - | `Name` | - | `DeleteEndpointResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Delete an existing global endpoint. For more information about global endpoints, see Making applications Regional-fault tolerant with global endpoints and event replication in the Amazon EventBridge User Guide . |
| `DeleteEventBus` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException` | Deletes the specified custom event bus or partner event bus. All rules associated with this event bus need to be deleted. |
| `DeletePartnerEventSource` | - | - | `Account`, `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException` | This operation is used by SaaS partners to delete a partner event source. This operation is not used by Amazon Web Services customers. |
| `DeleteRule` | - | - | `Name` | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Deletes the specified rule. Before you can delete the rule, you must remove all targets, using RemoveTargets. |
| `DescribeApiDestination` | - | - | `Name` | - | `DescribeApiDestinationResponse` | `InternalException`, `ResourceNotFoundException` | Retrieves details about an API destination. |
| `DescribeArchive` | - | - | `ArchiveName` | - | `DescribeArchiveResponse` | `InternalException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Retrieves details about an archive. |
| `DescribeConnection` | - | - | `Name` | - | `DescribeConnectionResponse` | `InternalException`, `ResourceNotFoundException` | Retrieves details about a connection. |
| `DescribeEndpoint` | - | - | `Name` | - | `DescribeEndpointResponse` | `InternalException`, `ResourceNotFoundException` | Get the information about an existing global endpoint. For more information about global endpoints, see Making applications Regional-fault tolerant with global endpoints and event replication in the Amazon EventBridge User Guide . |
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
| `ListEndpoints` | - | - | - | - | `ListEndpointsResponse` | `InternalException` | List the global endpoints associated with this account. For more information about global endpoints, see Making applications Regional-fault tolerant with global endpoints and event replication in the Amazon EventBridge User Guide . |
| `ListEventBuses` | - | - | - | - | `ListEventBusesResponse` | `InternalException` | Lists all the event buses in your account, including the default event bus, custom event buses, and partner event buses. |
| `ListEventSources` | - | - | - | - | `ListEventSourcesResponse` | `InternalException`, `OperationDisabledException` | You can use this to see all the partner event sources that have been shared with your Amazon Web Services account. For more information about partner event sources, see CreateEventBus. |
| `ListPartnerEventSourceAccounts` | - | - | `EventSourceName` | - | `ListPartnerEventSourceAccountsResponse` | `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | An SaaS partner can use this operation to display the Amazon Web Services account ID that a particular partner event source name is associated with. This operation is not used by Amazon Web Services customers. |
| `ListPartnerEventSources` | - | - | `NamePrefix` | - | `ListPartnerEventSourcesResponse` | `InternalException`, `OperationDisabledException` | An SaaS partner can use this operation to list all the partner event source names that they have created. This operation is not used by Amazon Web Services customers. |
| `ListReplays` | - | - | - | - | `ListReplaysResponse` | `InternalException` | Lists your replays. You can either list all the replays or you can provide a prefix to match to the replay names. |
| `ListRuleNamesByTarget` | - | - | `TargetArn` | - | `ListRuleNamesByTargetResponse` | `InternalException`, `ResourceNotFoundException` | Lists the rules for the specified target. You can see which of the rules in Amazon EventBridge can invoke a specific target in your account. |
| `ListRules` | - | - | - | - | `ListRulesResponse` | `InternalException`, `ResourceNotFoundException` | Lists your Amazon EventBridge rules. You can either list all the rules or you can provide a prefix to match to the rule names. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InternalException`, `ResourceNotFoundException` | Displays the tags associated with an EventBridge resource. In EventBridge, rules and event buses can be tagged. |
| `ListTargetsByRule` | - | - | `Rule` | - | `ListTargetsByRuleResponse` | `InternalException`, `ResourceNotFoundException` | Lists the targets assigned to the specified rule. The maximum number of results per page for requests is 100. |
| `PutEvents` | - | - | `Entries` | - | `PutEventsResponse` | `InternalException` | Sends custom events to Amazon EventBridge so that they can be matched to rules. You can batch multiple event entries into one request for efficiency. |
| `PutPartnerEvents` | - | - | `Entries` | - | `PutPartnerEventsResponse` | `InternalException`, `OperationDisabledException` | This is used by SaaS partners to write events to a customer's partner event bus. Amazon Web Services customers do not use this operation. |
| `PutPermission` | - | - | - | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException`, `PolicyLengthExceededException`, `ResourceNotFoundException` | Running `PutPermission` permits the specified Amazon Web Services account or Amazon Web Services organization to put events to the specified event bus . Amazon EventBridge rules in your account are triggered by these events arriving to an event bus in your... |
| `PutRule` | - | - | `Name` | - | `PutRuleResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ManagedRuleException`, `ResourceNotFoundException` | Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. |
| `PutTargets` | - | - | `Rule`, `Targets` | - | `PutTargetsResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `ManagedRuleException`, `ResourceNotFoundException` | Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule. Targets are the resources that are invoked when a rule is triggered. |
| `RemovePermission` | - | - | - | - | `Unit` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | Revokes the permission of another Amazon Web Services account to be able to put events to the specified event bus. Specify the account to revoke by the `StatementId` value that you associated with the account when you granted it permission with... |
| `RemoveTargets` | - | - | `Ids`, `Rule` | - | `RemoveTargetsResponse` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked. |
| `StartReplay` | - | - | `Destination`, `EventEndTime`, `EventSourceArn`, `EventStartTime`, `ReplayName` | - | `StartReplayResponse` | `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Starts the specified replay. Events are not necessarily replayed in the exact same order that they were added to the archive. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Assigns one or more tags (key-value pairs) to the specified EventBridge resource. Tags can help you organize and categorize your resources. |
| `TestEventPattern` | - | - | `Event`, `EventPattern` | - | `TestEventPatternResponse` | `InternalException`, `InvalidEventPatternException` | Tests whether the specified event pattern matches the provided event. Most services in Amazon Web Services treat : or / as the same character in Amazon Resource Names (ARNs). |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ConcurrentModificationException`, `InternalException`, `ManagedRuleException`, `ResourceNotFoundException` | Removes one or more tags from the specified EventBridge resource. In Amazon EventBridge, rules and event buses can be tagged. |
| `UpdateApiDestination` | - | - | `Name` | - | `UpdateApiDestinationResponse` | `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException` | Updates an API destination. |
| `UpdateArchive` | - | - | `ArchiveName` | - | `UpdateArchiveResponse` | `ConcurrentModificationException`, `InternalException`, `InvalidEventPatternException`, `LimitExceededException`, `ResourceNotFoundException` | Updates the specified archive. |
| `UpdateConnection` | - | - | `Name` | - | `UpdateConnectionResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Updates settings for a connection. |
| `UpdateEndpoint` | - | - | `Name` | - | `UpdateEndpointResponse` | `ConcurrentModificationException`, `InternalException`, `ResourceNotFoundException` | Update an existing endpoint. For more information about global endpoints, see Making applications Regional-fault tolerant with global endpoints and event replication in the Amazon EventBridge User Guide . |
| `UpdateEventBus` | - | - | - | - | `UpdateEventBusResponse` | `ConcurrentModificationException`, `InternalException`, `OperationDisabledException`, `ResourceNotFoundException` | Updates the specified event bus. |

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
| `ResourceAlreadyExistsException` | `structure` | `message` | The resource you are trying to create already exists. |
| `ManagedRuleException` | `structure` | `message` | This rule was created by an Amazon Web Services service on behalf of your account. |
| `InvalidEventPatternException` | `structure` | `message` | The event pattern is not valid. |
| `InvalidStateException` | `structure` | `message` | The specified state is not a valid state for an event source. |
| `AccessDeniedException` | `structure` | `message` | You do not have the necessary permissions for this action. |
| `ThrottlingException` | `structure` | `message` | This request cannot be completed due to throttling issues. |
| `ActivateEventSourceRequest` | `structure` | `Name` | - |
| `CancelReplayRequest` | `structure` | `ReplayName` | - |
| `CancelReplayResponse` | `structure` | `ReplayArn`, `State`, `StateReason` | - |
| `IllegalStatusException` | `structure` | `message` | An error occurred because a replay can be canceled only when the state is Running or Starting. |
| `CreateApiDestinationRequest` | `structure` | `ConnectionArn`, `Description`, `HttpMethod`, `InvocationEndpoint`, `InvocationRateLimitPerSecond`, `Name` | - |
| `CreateApiDestinationResponse` | `structure` | `ApiDestinationArn`, `ApiDestinationState`, `CreationTime`, `LastModifiedTime` | - |
| `CreateArchiveRequest` | `structure` | `ArchiveName`, `Description`, `EventPattern`, `EventSourceArn`, `KmsKeyIdentifier`, `RetentionDays` | - |
| `CreateArchiveResponse` | `structure` | `ArchiveArn`, `CreationTime`, `State`, `StateReason` | - |
| `CreateConnectionRequest` | `structure` | `AuthParameters`, `AuthorizationType`, `Description`, `InvocationConnectivityParameters`, `KmsKeyIdentifier`, `Name` | - |
| `CreateConnectionResponse` | `structure` | `ConnectionArn`, `ConnectionState`, `CreationTime`, `LastModifiedTime` | - |
| `CreateEndpointRequest` | `structure` | `Description`, `EventBuses`, `Name`, `ReplicationConfig`, `RoleArn`, `RoutingConfig` | - |
| `CreateEndpointResponse` | `structure` | `Arn`, `EventBuses`, `Name`, `ReplicationConfig`, `RoleArn`, `RoutingConfig`, `State` | - |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises EventBridge as a fan-out hub. Open it when adding targets because it lists durable target paths for Lambda, SQS, SNS, Step Functions, Batch, ECS, API Gateway, CloudWatch Logs, AppSync, Kinesis, and Firehose.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises where fan-out metadata ends and target execution starts. Open it before adding delivery behaviour that spans multiple service crates.
- Service implication: `PutTargets` metadata is the prerequisite for target execution semantics. Preserve target metadata first, then add behaviour-level tests for target delivery.
- Service implication: EventBridge and AppSync are bidirectional in AWS documentation. EventBridge can target AppSync mutations, and AppSync can use EventBridge as a data source.
- Service implication: Pipes-related work should share source-adapter concepts with DynamoDB Streams, Kinesis streams, and SQS queues rather than implementing each bridge as an unrelated one-off.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

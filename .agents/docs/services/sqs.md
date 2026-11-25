# Amazon Simple Queue Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon SQS API Reference . Amazon SQS is a reliable, highly-scalable hosted queue for storing messages as they travel between applications or microservices. Amazon SQS moves data between distributed application components and helps you decouple these components. For information on the permissions you need to use this API, see Identity and access management in the Amazon SQS Developer Guide. You can use Amazon Web Services SDKs to access Amazon SQS using your favorite programming language.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-sqs/tests/scenario_test.rs`: process batch jobs through queue creation, send/receive/delete batches, and queue cleanup.
- Backported from `scenario_test.rs`: configure a dead-letter queue and redrive policy.
- Backported from `scenario_test.rs`: extend visibility timeout for long-running jobs and preserve FIFO ordering for message groups.
- Scenario insight from EC2: add full state-machine walks for Amazon Simple Queue Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support standard and FIFO queues, message visibility, batching, long polling, redrive/DLQ policies, queue attributes, tags, and ordered or at-least-once delivery behaviour.

## Service Identity and Protocol

- AWS model slug: `sqs`
- AWS SDK for Rust slug: `sqs`
- Model version: `2012-11-05`
- Model file: `vendor/api-models-aws/models/sqs/service/2012-11-05/sqs-2012-11-05.json`
- SDK ID: `SQS`
- Endpoint prefix: `sqs`
- ARN namespace: `sqs`
- CloudFormation name: `SQS`
- CloudTrail event source: `sqs.amazonaws.com`
- Protocols: `awsJson1_0`, `awsQueryCompatible`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Delete` (3), `Change` (2), `Get` (2), `Send` (2), `Add` (1), `Cancel` (1), `Create` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddPermission`, `CancelMessageMoveTask`, `CreateQueue`, `DeleteMessage`, `DeleteMessageBatch`, `DeleteQueue`, `RemovePermission`, `SetQueueAttributes`, `StartMessageMoveTask`, `TagQueue`, `UntagQueue`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetQueueAttributes`, `GetQueueUrl`, `ListDeadLetterSourceQueues`, `ListMessageMoveTasks`, `ListQueueTags`, `ListQueues`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelMessageMoveTask`, `ListMessageMoveTasks`, `StartMessageMoveTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `SNS`, `SQS`, `Lambda`, `ECS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html
- https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/standard-queues-at-least-once-delivery.html
- https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/best-practices-message-deduplication.html

Research outcomes:
- Receiving a message starts its visibility timeout. If the consumer does not delete the message before timeout expiry, the message becomes visible again and can be received by another consumer.
- Queue default visibility timeout is 30 seconds. ChangeMessageVisibility can extend, shorten, or terminate the timeout for an individual received message; setting it to 0 makes the message visible immediately.
- Visibility timeout has a maximum 12-hour limit measured from first receipt; extending visibility does not reset that limit.
- Standard queues use at-least-once delivery. Duplicate delivery can happen, so consumers are expected to be idempotent.
- Standard queue in-flight messages have an approximate 120,000-message limit. Short polling can return OverLimit; long polling instead withholds new messages until in-flight count drops.
- FIFO queues hold later messages in the same message group while a prior message in that group is in flight; ordering is per message group, not global across the queue.

Parity implications:
- Model receipt handles and in-flight visibility as time-dependent state, not as a property of the stored message alone.
- Preserve receive count, redrive eligibility, and DLQ movement across visibility expirations and delete failures.
- FIFO queues require per-group availability gates and deduplication tracking independent of standard queue delivery.

### FIFO and attribute-map parity ( issues #64 – #66, 2026-11-22 )

- **Attributes are map entries, not members.** `CreateQueue` and `SetQueueAttributes` take `Attributes: map<QueueAttributeName, String>`, and `ReceiveMessage` returns `Message.Attributes: map<MessageSystemAttributeName, String>`. Every queue setting and every system attribute is a key of an enum-keyed map. `.agents/bin/audit-wire-members.py` checks structure members only, so a dropped key is invisible to it; walk the `QueueAttributeName` and `MessageSystemAttributeName` enum values by hand instead.
- **Queue attributes the handler must store.** On 2026-11-22 `create_queue` kept `VisibilityTimeout`, `DelaySeconds`, `MaximumMessageSize`, `MessageRetentionPeriod`, `ReceiveMessageWaitTimeSeconds`, `ContentBasedDeduplication`, `RedrivePolicy` and `Policy`. `GetQueueAttributes` returned `DeduplicationScope=queue` and `FifoThroughputLimit=perQueue` as constants ( #64 ). A constant default passes a presence test and moto's `test_create_fifo_queue`, which only asserts the defaults. High-throughput FIFO is `DeduplicationScope=messageGroup` with `FifoThroughputLimit=perMessageGroupId`; AWS rejects `perMessageGroupId` unless the scope is `messageGroup`. `KmsMasterKeyId`, `KmsDataKeyReusePeriodSeconds`, `SqsManagedSseEnabled` ( hard-coded `true` ) and `RedriveAllowPolicy` are not stored either; check each before claiming the attribute. The `MaximumMessageSize` default is now 1,048,576 bytes ( 1 MiB ) per the `SetQueueAttributes` reference; winterbaume still defaults to 262,144. Source: https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SetQueueAttributes.html
- **Group gating ( #65 ).** `send_message` discarded `MessageGroupId` ( `let _ = message_group_id; // currently informational; FIFO ordering not yet enforced`, present since the initial commit and never filed in `TODO.md` ). A receive must skip every message whose group has a message in flight, and must return messages of one group in send order. `ChangeMessageVisibility` to 0 and visibility expiry release the group; so does `DeleteMessage` of the in-flight message.
- **FIFO system attributes ( #66 ).** A FIFO message carries `MessageGroupId`, `MessageDeduplicationId` ( the explicit id, or the SHA-256 of the body under content-based deduplication ) and `SequenceNumber`. winterbaume keys content-based deduplication on the body MD5, so an explicit id equal to the SHA-256 of an earlier body is not treated as a duplicate, as AWS documents it is ( moto `test_fifo_queue_send_deduplicationid_same_as_sha256_of_old_message` ). `SendMessage`, `SendMessageBatch` entries and received messages report the same `SequenceNumber`; it is a 128-bit decimal string that increases per queue. `ReceiveMessage` returns only the attributes named in `MessageSystemAttributeNames` ( or the deprecated `AttributeNames` ), with `All` meaning every one; the handler ignored both and always returned four standard attributes.
- **Testing traps.** `scenario_test.rs::test_fifo_queue_ordered_processing` deletes each message before it receives the next, so it passes without any gating. Assert the negative case: two messages in one group, receive one with a long visibility timeout, and the next receive returns nothing. The port of moto's `test_send_message_with_message_group_id` dropped the two assertions on `MessageGroupId` and `MessageDeduplicationId` that are the point of the test. Of moto's 38 SQS tests that touch FIFO queues, group ids or sequence numbers, 8 are ported; the unported ones include `test_create_fifo_queue_with_high_throughput`, the three `test_receive_messages_with_message_group_id*` tests and `test_fifo_send_receive_message_with_attribute_name`.
- **Terraform.** `aws_sqs_queue` reads `deduplication_scope` and `fifo_throughput_limit` back after apply; the E2E FIFO case sets neither and stops at apply.

## Operation Groups

### List

- Operations: `ListDeadLetterSourceQueues`, `ListMessageMoveTasks`, `ListQueues`, `ListQueueTags`
- Traits: `paginated` (2)
- Common required input members in this group: `QueueUrl`

### Delete

- Operations: `DeleteMessage`, `DeleteMessageBatch`, `DeleteQueue`
- Common required input members in this group: `QueueUrl`

### Change

- Operations: `ChangeMessageVisibility`, `ChangeMessageVisibilityBatch`
- Common required input members in this group: `QueueUrl`

### Get

- Operations: `GetQueueAttributes`, `GetQueueUrl`
- Common required input members in this group: -

### Send

- Operations: `SendMessage`, `SendMessageBatch`
- Common required input members in this group: `QueueUrl`

### Add

- Operations: `AddPermission`
- Common required input members in this group: -

### Cancel

- Operations: `CancelMessageMoveTask`
- Common required input members in this group: -

### Create

- Operations: `CreateQueue`
- Common required input members in this group: -

### Purge

- Operations: `PurgeQueue`
- Common required input members in this group: -

### Receive

- Operations: `ReceiveMessage`
- Common required input members in this group: -

### Remove

- Operations: `RemovePermission`
- Common required input members in this group: -

### Set

- Operations: `SetQueueAttributes`
- Common required input members in this group: -

### Start

- Operations: `StartMessageMoveTask`
- Common required input members in this group: -

### Tag

- Operations: `TagQueue`
- Common required input members in this group: -

### Untag

- Operations: `UntagQueue`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddPermission` | `-` | - | `QueueUrl`, `Label`, `AWSAccountIds`, `Actions` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `OverLimit`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Adds a permission to a queue for a specific principal . This allows sharing access to the queue. When you create a queue, you have full control access rights for the queue. Only you, the owner of the queue, can grant ... |
| `CancelMessageMoveTask` | `-` | - | `TaskHandle` | - | `CancelMessageMoveTaskResult` | `InvalidAddress`, `InvalidSecurity`, `RequestThrottled`, `ResourceNotFoundException`, `UnsupportedOperation` | Cancels a specified message movement task. A message movement can only be cancelled when the current status is RUNNING. Cancelling a message movement task does not revert the messages that have already been moved. It ... |
| `ChangeMessageVisibility` | `-` | - | `QueueUrl`, `ReceiptHandle`, `VisibilityTimeout` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `MessageNotInflight`, `QueueDoesNotExist`, `ReceiptHandleIsInvalid`, `RequestThrottled`, `UnsupportedOperation` | Changes the visibility timeout of a specified message in a queue to a new value. The default visibility timeout for a message is 30 seconds. The minimum is 0 seconds. The maximum is 12 hours. For more information, se ... |
| `ChangeMessageVisibilityBatch` | `-` | - | `QueueUrl`, `Entries` | - | `ChangeMessageVisibilityBatchResult` | `BatchEntryIdsNotDistinct`, `EmptyBatchRequest`, `InvalidAddress`, `InvalidBatchEntryId`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `TooManyEntriesInBatchRequest`, `UnsupportedOperation` | Changes the visibility timeout of multiple messages. This is a batch version of ChangeMessageVisibility . The result of the action on each message is reported individually in the response. You can send up to 10 Chang ... |
| `CreateQueue` | `-` | - | `QueueName` | - | `CreateQueueResult` | `InvalidAddress`, `InvalidAttributeName`, `InvalidAttributeValue`, `InvalidSecurity`, `QueueDeletedRecently`, `QueueNameExists`, `RequestThrottled`, `UnsupportedOperation` | Creates a new standard or FIFO queue. You can pass one or more attributes in the request. Keep the following in mind: If you don't specify the FifoQueue attribute, Amazon SQS creates a standard queue. You can't chang ... |
| `DeleteMessage` | `-` | - | `QueueUrl`, `ReceiptHandle` | - | `Unit` | `InvalidAddress`, `InvalidIdFormat`, `InvalidSecurity`, `QueueDoesNotExist`, `ReceiptHandleIsInvalid`, `RequestThrottled`, `UnsupportedOperation` | Deletes the specified message from the specified queue. To select the message to delete, use the ReceiptHandle of the message ( not the MessageId which you receive when you send the message). Amazon SQS can delete a ... |
| `DeleteMessageBatch` | `-` | - | `QueueUrl`, `Entries` | - | `DeleteMessageBatchResult` | `BatchEntryIdsNotDistinct`, `EmptyBatchRequest`, `InvalidAddress`, `InvalidBatchEntryId`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `TooManyEntriesInBatchRequest`, `UnsupportedOperation` | Deletes up to ten messages from the specified queue. This is a batch version of DeleteMessage . The result of the action on each message is reported individually in the response. Because the batch request can result ... |
| `DeleteQueue` | `-` | - | `QueueUrl` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Deletes the queue specified by the QueueUrl , regardless of the queue's contents. Be careful with the DeleteQueue action: When you delete a queue, any messages in the queue are no longer available. When you delete a ... |
| `GetQueueAttributes` | `-` | - | `QueueUrl` | - | `GetQueueAttributesResult` | `InvalidAddress`, `InvalidAttributeName`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Gets attributes for the specified queue. To determine whether a queue is FIFO , you can check whether QueueName ends with the .fifo suffix. |
| `GetQueueUrl` | `-` | - | `QueueName` | - | `GetQueueUrlResult` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | The GetQueueUrl API returns the URL of an existing Amazon SQS queue. This is useful when you know the queue's name but need to retrieve its URL for further operations. To access a queue owned by another Amazon Web Se ... |
| `ListDeadLetterSourceQueues` | `-` | `paginated` | `QueueUrl` | - | `ListDeadLetterSourceQueuesResult` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Returns a list of your queues that have the RedrivePolicy queue attribute configured with a dead-letter queue. The ListDeadLetterSourceQueues methods supports pagination. Set parameter MaxResults in the request to sp ... |
| `ListMessageMoveTasks` | `-` | - | `SourceArn` | - | `ListMessageMoveTasksResult` | `InvalidAddress`, `InvalidSecurity`, `RequestThrottled`, `ResourceNotFoundException`, `UnsupportedOperation` | Gets the most recent message movement tasks (up to 10) under a specific source queue. This action is currently limited to supporting message redrive from dead-letter queues (DLQs) only. In this context, the source qu ... |
| `ListQueues` | `-` | `paginated` | - | - | `ListQueuesResult` | `InvalidAddress`, `InvalidSecurity`, `RequestThrottled`, `UnsupportedOperation` | Returns a list of your queues in the current region. The response includes a maximum of 1,000 results. If you specify a value for the optional QueueNamePrefix parameter, only queues with a name that begins with the s ... |
| `ListQueueTags` | `-` | - | `QueueUrl` | - | `ListQueueTagsResult` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | List all cost allocation tags added to the specified Amazon SQS queue. For an overview, see Tagging Your Amazon SQS Queues in the Amazon SQS Developer Guide . Cross-account permissions don't apply to this action. For ... |
| `PurgeQueue` | `-` | - | `QueueUrl` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `PurgeQueueInProgress`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Deletes available messages in a queue (including in-flight messages) specified by the QueueURL parameter. When you use the PurgeQueue action, you can't retrieve any messages deleted from a queue. The message deletion ... |
| `ReceiveMessage` | `-` | - | `QueueUrl` | - | `ReceiveMessageResult` | `InvalidAddress`, `InvalidSecurity`, `KmsAccessDenied`, `KmsDisabled`, `KmsInvalidKeyUsage`, `KmsInvalidState`, `KmsNotFound`, `KmsOptInRequired`, `KmsThrottled`, `OverLimit`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Retrieves one or more messages (up to 10), from the specified queue. Using the WaitTimeSeconds parameter enables long-poll support. For more information, see Amazon SQS Long Polling in the Amazon SQS Developer Guide ... |
| `RemovePermission` | `-` | - | `QueueUrl`, `Label` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Revokes any permissions in the queue policy that matches the specified Label parameter. Only the owner of a queue can remove permissions from it. Cross-account permissions don't apply to this action. For more informa ... |
| `SendMessage` | `-` | - | `QueueUrl`, `MessageBody` | - | `SendMessageResult` | `InvalidAddress`, `InvalidMessageContents`, `InvalidSecurity`, `KmsAccessDenied`, `KmsDisabled`, `KmsInvalidKeyUsage`, `KmsInvalidState`, `KmsNotFound`, `KmsOptInRequired`, `KmsThrottled`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Delivers a message to the specified queue. A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed. For more information, see the W3C specification for characters . #x ... |
| `SendMessageBatch` | `-` | - | `QueueUrl`, `Entries` | - | `SendMessageBatchResult` | `BatchEntryIdsNotDistinct`, `BatchRequestTooLong`, `EmptyBatchRequest`, `InvalidAddress`, `InvalidBatchEntryId`, `InvalidSecurity`, `KmsAccessDenied`, `KmsDisabled`, `KmsInvalidKeyUsage`, `KmsInvalidState`, `KmsNotFound`, `KmsOptInRequired`, `KmsThrottled`, `QueueDoesNotExist`, `RequestThrottled`, `TooManyEntriesInBatchRequest`, `UnsupportedOperation` | You can use SendMessageBatch to send up to 10 messages to the specified queue by assigning either identical or different values to each message (or by not assigning values at all). This is a batch version of SendMess ... |
| `SetQueueAttributes` | `-` | - | `QueueUrl`, `Attributes` | - | `Unit` | `InvalidAddress`, `InvalidAttributeName`, `InvalidAttributeValue`, `InvalidSecurity`, `OverLimit`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Sets the value of one or more queue attributes, like a policy. When you change a queue's attributes, the change can take up to 60 seconds for most of the attributes to propagate throughout the Amazon SQS system. Chan ... |
| `StartMessageMoveTask` | `-` | - | `SourceArn` | - | `StartMessageMoveTaskResult` | `InvalidAddress`, `InvalidSecurity`, `RequestThrottled`, `ResourceNotFoundException`, `UnsupportedOperation` | Starts an asynchronous task to move messages from a specified source queue to a specified destination queue. This action is currently limited to supporting message redrive from queues that are configured as dead-lett ... |
| `TagQueue` | `-` | - | `QueueUrl`, `Tags` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Add cost allocation tags to the specified Amazon SQS queue. For an overview, see Tagging Your Amazon SQS Queues in the Amazon SQS Developer Guide . When you use queue tags, keep the following guidelines in mind: Addi ... |
| `UntagQueue` | `-` | - | `QueueUrl`, `TagKeys` | - | `Unit` | `InvalidAddress`, `InvalidSecurity`, `QueueDoesNotExist`, `RequestThrottled`, `UnsupportedOperation` | Remove cost allocation tags from the specified Amazon SQS queue. For an overview, see Tagging Your Amazon SQS Queues in the Amazon SQS Developer Guide . Cross-account permissions don't apply to this action. For more ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BatchEntryIdsNotDistinct` | `structure` | message | Two or more batch entries in the request have the same Id . |
| `BatchRequestTooLong` | `structure` | message | The length of all the messages put together is more than the limit. |
| `EmptyBatchRequest` | `structure` | message | The batch request doesn't contain any entries. |
| `InvalidAddress` | `structure` | message | The specified ID is invalid. |
| `InvalidAttributeName` | `structure` | message | The specified attribute doesn't exist. |
| `InvalidAttributeValue` | `structure` | message | A queue attribute value is invalid. |
| `InvalidBatchEntryId` | `structure` | message | The Id of a batch entry in a batch request doesn't abide by the specification. |
| `InvalidIdFormat` | `structure` | **empty (no members)** | The specified receipt handle isn't valid for the current version. |
| `InvalidMessageContents` | `structure` | message | The message contains characters outside the allowed set. |
| `InvalidSecurity` | `structure` | message | The request was not made over HTTPS or did not use SigV4 for signing. |
| `KmsAccessDenied` | `structure` | message | The caller doesn't have the required KMS access. |
| `KmsDisabled` | `structure` | message | The request was denied due to request throttling. |
| `KmsInvalidKeyUsage` | `structure` | message | The request was rejected for one of the following reasons: The KeyUsage value of the KMS key is incompatible with the API operation. The encryption algorith ... |
| `KmsInvalidState` | `structure` | message | The request was rejected because the state of the specified resource is not valid for this request. |
| `KmsNotFound` | `structure` | message | The request was rejected because the specified entity or resource could not be found. |
| `KmsOptInRequired` | `structure` | message | The request was rejected because the specified key policy isn't syntactically or semantically correct. |
| `KmsThrottled` | `structure` | message | Amazon Web Services KMS throttles requests for the following conditions. |
| `MessageNotInflight` | `structure` | **empty (no members)** | The specified message isn't in flight. |
| `OverLimit` | `structure` | message | The specified action violates a limit. For example, ReceiveMessage returns this error if the maximum number of in flight messages is reached and AddPermissi ... |
| `PurgeQueueInProgress` | `structure` | message | Indicates that the specified queue previously received a PurgeQueue request within the last 60 seconds (the time it can take to delete the messages in the q ... |
| `QueueDeletedRecently` | `structure` | message | You must wait 60 seconds after deleting a queue before you can create another queue with the same name. |
| `QueueDoesNotExist` | `structure` | message | Ensure that the QueueUrl is correct and that the queue has not been deleted. |
| `QueueNameExists` | `structure` | message | A queue with this name already exists. Amazon SQS returns this error only if the request includes attributes whose values differ from those of the existing ... |
| `ReceiptHandleIsInvalid` | `structure` | message | The specified receipt handle isn't valid. |
| `RequestThrottled` | `structure` | message | The request was denied due to request throttling. Exceeds the permitted request rate for the queue or for the recipient of the request. Ensure that the requ ... |
| `ResourceNotFoundException` | `structure` | message | One or more specified resources don't exist. |
| `TooManyEntriesInBatchRequest` | `structure` | message | The batch request contains more entries than permissible. For Amazon SQS, the maximum number of entries you can include in a single SendMessageBatch , Delet ... |
| `UnsupportedOperation` | `structure` | message | Error code 400. Unsupported operation. |
| `AddPermissionRequest` | `structure` | QueueUrl, Label, AWSAccountIds, Actions | - |
| `CancelMessageMoveTaskRequest` | `structure` | TaskHandle | - |
| `CancelMessageMoveTaskResult` | `structure` | ApproximateNumberOfMessagesMoved | - |
| `ChangeMessageVisibilityRequest` | `structure` | QueueUrl, ReceiptHandle, VisibilityTimeout | - |
| `ChangeMessageVisibilityBatchRequest` | `structure` | QueueUrl, Entries | - |
| `ChangeMessageVisibilityBatchResult` | `structure` | Successful, Failed | For each message in the batch, the response contains a ChangeMessageVisibilityBatchResultEntry tag if the message succeeds or a BatchResultErrorEntry tag if ... |
| `CreateQueueRequest` | `structure` | QueueName, Attributes, tags | - |
| `CreateQueueResult` | `structure` | QueueUrl | Returns the QueueUrl attribute of the created queue. |
| `DeleteMessageRequest` | `structure` | QueueUrl, ReceiptHandle | - |
| `DeleteMessageBatchRequest` | `structure` | QueueUrl, Entries | - |
| `DeleteMessageBatchResult` | `structure` | Successful, Failed | For each message in the batch, the response contains a DeleteMessageBatchResultEntry tag if the message is deleted or a BatchResultErrorEntry tag if the mes ... |
| `DeleteQueueRequest` | `structure` | QueueUrl | - |
| `MessageSystemAttributeName` | `enum` | All, SenderId, SentTimestamp, ApproximateReceiveCount, ApproximateFirstReceiveTimestamp, SequenceNumber, MessageDeduplicationId, MessageGroupId, AWSTraceHeader, DeadLetterQueueSourceArn | - |
| `MessageSystemAttributeNameForSends` | `enum` | AWSTraceHeader | - |
| `QueueAttributeName` | `enum` | All, Policy, VisibilityTimeout, MaximumMessageSize, MessageRetentionPeriod, ApproximateNumberOfMessages, ApproximateNumberOfMessagesNotVisible, CreatedTimestamp, LastModifiedTimestamp, QueueArn, ApproximateNumberOfMessagesDelayed, DelaySeconds, ... (+10) | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/pluggable-backends-and-query-execution-synthesis.md, .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/pluggable-backends-and-query-execution-synthesis.md`: summarises the SQS backend boundary. Open it before changing queue state persistence, because snapshot, restore, and merge fidelity should route through the selected backend rather than hidden in-memory state.
- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises high-value SQS integration paths. Open it for Lambda event-source mappings, EventBridge Pipes sources, and Step Functions direct SQS send-message integration.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises cross-service execution boundaries. Open it before wiring SQS delivery to another service crate.
- Service implication: the default service path should remain in-memory while Redis-backed persistence stays in `winterbaume-sqs-redis`.

## Member Audit

Coverage is counted per operation, `Operation Detail Matrix` lists required inputs only, and `HTTP Bindings` lists header and query bindings only. This section records what happens to the remaining members. For SQS most of the surface is in the two attribute maps, which the audit does not see; `FIFO and attribute-map parity` above covers those.

### Known defects

- #64: `DeduplicationScope` and `FifoThroughputLimit` are discarded on create and reported as constants.
- #65: `MessageGroupId` does not gate delivery.
- #66: `ReceiveMessage` ignores `MessageSystemAttributeNames` and `AttributeNames` ( both listed below ) and never reports the FIFO system attributes. `SendMessageResult.SequenceNumber` and `SendMessageBatchResultEntry.SequenceNumber` are never populated.

### Open candidates ( audited 2026-11-25 )

Generated by `.agents/bin/audit-wire-members.py sqs --dossier`. Rows are unclassified candidates, not confirmed defects: each one is resolved as honoured, rejected with the AWS error, ignored because AWS ignores it, or filed in `TODO.md` ( QUALITY_GATE.md §5.3 and §5.4 ). Regenerate after changing handlers.

- Unread request members: 12 in 7 operations.
- Operations whose generated request deserialiser is never called: 0.
- Never-populated response members: 11 in 5 structures ( run the script for the list ).
- Response structures populated unevenly across literals: 1.

#### Unread request members

- `get_queue_attributes` ( 1 of 2 ): `attribute_names`
- `get_queue_url` ( 1 of 2 ): `queue_owner_a_w_s_account_id`
- `list_dead_letter_source_queues` ( 2 of 3 ): `max_results`, `next_token`
- `list_message_move_tasks` ( 1 of 2 ): `max_results`
- `list_queues` ( 2 of 3 ): `max_results`, `next_token`
- `receive_message` ( 4 of 8 ): `attribute_names`, `message_attribute_names`, `message_system_attribute_names`, `receive_request_attempt_id`
- `send_message` ( 1 of 7 ): `message_system_attributes`

#### Never-populated response members

- `ListMessageMoveTasksResultEntry`: `failure_reason`
- `Message`: `m_d5_of_message_attributes`
- `MessageAttributeValue`: `binary_list_values`, `binary_value`, `string_list_values`
- `SendMessageBatchResultEntry`: `m_d5_of_message_attributes`, `m_d5_of_message_system_attributes`, `sequence_number`
- `SendMessageResult`: `m_d5_of_message_attributes`, `m_d5_of_message_system_attributes`, `sequence_number`

#### Unevenly populated response structures

- `Message` ( 2 literals ): `attributes` 1/2, `m_d5_of_body` 1/2, `message_attributes` 1/2

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

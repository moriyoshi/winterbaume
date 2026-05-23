# Amazon DynamoDB Streams

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon DynamoDB Amazon DynamoDB Streams provides API actions for accessing streams and processing stream records. To learn more about application development with Streams, see Capturing Table Activity with DynamoDB Streams in the Amazon DynamoDB Developer Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon DynamoDB Streams workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `Describe`, `List` operation families, including `GetRecords`, `GetShardIterator`, `DescribeStream`, `ListStreams`.

## Service Identity and Protocol

- AWS model slug: `dynamodb-streams`
- AWS SDK for Rust slug: `dynamodbstreams`
- Model version: `2012-08-10`
- Model file: `vendor/api-models-aws/models/dynamodb-streams/service/2012-08-10/dynamodb-streams-2012-08-10.json`
- SDK ID: `DynamoDB Streams`
- Endpoint prefix: `streams.dynamodb`
- ARN namespace: `dynamodb`
- CloudFormation name: `DynamoDBStreams`
- CloudTrail event source: `dynamodbstreams.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `Describe` (1), `List` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeStream`, `GetRecords`, `GetShardIterator`, `ListStreams`.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.KCLAdapter.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Streams.Lambda.html

Research outcomes:
- DynamoDB Streams captures item-level changes for a DynamoDB table.
- Stream records are retained for 24 hours.
- Stream view type controls whether records include keys only, new image, old image, or both images.
- Streams are read through shards, shard iterators, sequence numbers, and GetRecords-style pagination.
- Lambda can consume DynamoDB Streams as an event source with retry and throttling considerations.
- The Kinesis adapter can process stream records and manage shard-worker balancing.

Parity implications:
- Model stream descriptors, stream view type, shards, sequence numbers, records, shard iterators, trim horizon/latest positions, and retention expiry separately.
- Table writes should append stream records only when streams are enabled.
- Stream reading should enforce iterator position, expiry, and shard boundaries.

## Cross-Service Integration Gaps

- **`dynamodbstreams-lambda`** ( primary ): DynamoDB Streams captures change records in `stream_records` ( on `winterbaume-dynamodb`'s tables, observed here ) but never dispatches them to Lambda event-source mappings registered via `winterbaume-lambda`'s `CreateEventSourceMapping`. This overlaps with the broader `lambda-event-sources` gap but highlights the DynamoDB Streams → Lambda path specifically. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `dynamodbstreams-lambda` and `lambda-event-sources` ).

## Operation Groups

### Get

- Operations: `GetRecords`, `GetShardIterator`
- Common required input members in this group: -

### Describe

- Operations: `DescribeStream`
- Common required input members in this group: -

### List

- Operations: `ListStreams`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeStream` | `-` | - | `StreamArn` | - | `DescribeStreamOutput` | `InternalServerError`, `ResourceNotFoundException` | Returns information about a stream, including the current status of the stream, its Amazon Resource Name (ARN), the composition of its shards, and its corresponding DynamoDB table. You can call DescribeStream at a ma ... |
| `GetRecords` | `-` | - | `ShardIterator` | - | `GetRecordsOutput` | `ExpiredIteratorException`, `InternalServerError`, `LimitExceededException`, `ResourceNotFoundException`, `TrimmedDataAccessException` | Retrieves the stream records from a given shard. Specify a shard iterator using the ShardIterator parameter. The shard iterator specifies the position in the shard from which you want to start reading stream records ... |
| `GetShardIterator` | `-` | - | `StreamArn`, `ShardId`, `ShardIteratorType` | - | `GetShardIteratorOutput` | `InternalServerError`, `ResourceNotFoundException`, `TrimmedDataAccessException` | Returns a shard iterator. A shard iterator provides information about how to retrieve the stream records from within a shard. Use the shard iterator in a subsequent GetRecords request to read the stream records from ... |
| `ListStreams` | `-` | - | - | - | `ListStreamsOutput` | `InternalServerError`, `ResourceNotFoundException` | Returns an array of stream ARNs associated with the current account and endpoint. If the TableName parameter is present, then ListStreams will return only the streams ARNs for that table. You can call ListStreams at ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ExpiredIteratorException` | `structure` | message | The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the Get ... |
| `InternalServerError` | `structure` | message | An error occurred on the server side. |
| `LimitExceededException` | `structure` | message | There is no limit to the number of daily on-demand backups that can be taken. For most purposes, up to 500 simultaneous table operations are allowed per acc ... |
| `ResourceNotFoundException` | `structure` | message | The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be ACTIVE . |
| `TrimmedDataAccessException` | `structure` | message | The operation attempted to read past the oldest stream record in a shard. In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records wh ... |
| `DescribeStreamInput` | `structure` | StreamArn, Limit, ExclusiveStartShardId, ShardFilter | Represents the input of a DescribeStream operation. |
| `DescribeStreamOutput` | `structure` | StreamDescription | Represents the output of a DescribeStream operation. |
| `GetRecordsInput` | `structure` | ShardIterator, Limit | Represents the input of a GetRecords operation. |
| `GetRecordsOutput` | `structure` | Records, NextShardIterator | Represents the output of a GetRecords operation. |
| `GetShardIteratorInput` | `structure` | StreamArn, ShardId, ShardIteratorType, SequenceNumber | Represents the input of a GetShardIterator operation. |
| `GetShardIteratorOutput` | `structure` | ShardIterator | Represents the output of a GetShardIterator operation. |
| `ListStreamsInput` | `structure` | TableName, Limit, ExclusiveStartStreamArn | Represents the input of a ListStreams operation. |
| `ListStreamsOutput` | `structure` | Streams, LastEvaluatedStreamArn | Represents the output of a ListStreams operation. |
| `KeyType` | `enum` | HASH, RANGE | - |
| `OperationType` | `enum` | INSERT, MODIFY, REMOVE | - |
| `ShardFilterType` | `enum` | CHILD_SHARDS | - |
| `ShardIteratorType` | `enum` | TRIM_HORIZON, LATEST, AT_SEQUENCE_NUMBER, AFTER_SEQUENCE_NUMBER | - |
| `StreamStatus` | `enum` | ENABLING, ENABLED, DISABLING, DISABLED | - |
| `StreamViewType` | `enum` | NEW_IMAGE, OLD_IMAGE, NEW_AND_OLD_IMAGES, KEYS_ONLY | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/runtime-state-and-service-infrastructure-synthesis.md, .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### Derived Service Boundary

- DynamoDB Streams is a derived service. Table metadata and stream payloads originate in DynamoDB tables and write paths; iterator state, shard positions, and delivery bookkeeping belong in `winterbaume-dynamodbstreams`.
- DynamoDB write paths should append stream change records for `PutItem`, `UpdateItem`, and `DeleteItem` when table streams are enabled. Records carry event name, sequence number, keys, old image, and new image.
- Stream APIs should not fabricate independent table or item state. They should expose the stream view of DynamoDB-owned changes and keep stream-local state limited to consumption mechanics.

### Integration and Tests

- Lambda event-source mappings and EventBridge Pipes consumption are the highest-value downstream integrations for DynamoDB Streams.
- Cross-service tests should seed DynamoDB tables and writes through the DynamoDB service rather than fabricating stream-only state.
- Source-adapter work for EventBridge Pipes should preserve DynamoDB Streams cursor and payload semantics even if it shares infrastructure with Kinesis or SQS adapters.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

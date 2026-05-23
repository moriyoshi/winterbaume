# Amazon Kinesis

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Kinesis Data Streams Service API Reference Amazon Kinesis Data Streams is a managed service that scales elastically for real-time processing of streaming big data.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Kinesis where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Kinesis by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon Kinesis resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Kinesis workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Update`, `Get`, `Put` operation families, including `DescribeAccountSettings`, `DescribeLimits`, `DescribeStream`, `DescribeStreamConsumer`, `ListShards`, `ListStreamConsumers`.

## Service Identity and Protocol

- AWS model slug: `kinesis`
- AWS SDK for Rust slug: `kinesis`
- Model version: `2013-12-02`
- Model file: `vendor/api-models-aws/models/kinesis/service/2013-12-02/kinesis-2013-12-02.json`
- SDK ID: `Kinesis`
- Endpoint prefix: `kinesis`
- ARN namespace: `kinesis`
- CloudFormation name: `Kinesis`
- CloudTrail event source: `kinesis.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `ConsumerARN`, `Endpoint`, `OperationType`, `Region`, `ResourceARN`, `StreamARN`, `StreamId`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (5), `List` (5), `Update` (5), `Get` (3), `Put` (3), `Delete` (2), `Add` (1), `Create` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToStream`, `CreateStream`, `DeleteResourcePolicy`, `DeleteStream`, `DeregisterStreamConsumer`, `DisableEnhancedMonitoring`, `EnableEnhancedMonitoring`, `PutRecord`, `PutRecords`, `PutResourcePolicy`, `RegisterStreamConsumer`, `RemoveTagsFromStream`, `StartStreamEncryption`, `StopStreamEncryption`, `TagResource`, `UntagResource`, `UpdateAccountSettings`, `UpdateMaxRecordSize`, `UpdateShardCount`, `UpdateStreamMode`, ... (+1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountSettings`, `DescribeLimits`, `DescribeStream`, `DescribeStreamConsumer`, `DescribeStreamSummary`, `GetRecords`, `GetResourcePolicy`, `GetShardIterator`, `ListShards`, `ListStreamConsumers`, `ListStreams`, `ListTagsForResource`, `ListTagsForStream`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartStreamEncryption`, `StopStreamEncryption`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/streams/latest/dev/key-concepts.html
- https://docs.aws.amazon.com/streams/latest/dev/service-sizes-and-limits.html
- https://docs.aws.amazon.com/streams/latest/dev/kinesis-extended-retention.html

Research outcomes:
- A stream is a set of shards. Each shard contains an ordered sequence of immutable data records.
- A data record has a sequence number, partition key, and blob. Kinesis does not inspect or alter the blob.
- Partition keys are Unicode strings up to 256 characters. An MD5 hash maps each partition key to a shard hash-key range.
- Sequence numbers are assigned after PutRecord or PutRecords and are unique per partition key within a shard; they generally increase over time for the same partition key.
- Retention defaults to 24 hours and can be increased up to 8760 hours or decreased down to 24 hours.
- Increasing retention stops making old records inaccessible within several minutes. Decreasing retention makes records older than the new period inaccessible almost immediately.
- Provisioned shards each support fixed write and read rates, while on-demand mode manages shard capacity automatically.
- GetRecords can return up to 10 MB or 10,000 records from one shard, and a large read can throttle subsequent reads for several seconds.

Parity implications:
- Model streams, capacity mode, shards, hash-key ranges, records, sequence numbers, iterators, registered consumers, encryption, and retention windows.
- PutRecord and PutRecords need partition-key hashing, sequence ordering, per-shard throughput limits, and partial failure behaviour for batch writes.
- Read APIs should enforce iterator position, retention expiry, per-shard limits, read throttling, and shard split/merge lineage where implemented.

## Operation Groups

### Describe

- Operations: `DescribeAccountSettings`, `DescribeLimits`, `DescribeStream`, `DescribeStreamConsumer`, `DescribeStreamSummary`
- Common required input members in this group: -

### List

- Operations: `ListShards`, `ListStreamConsumers`, `ListStreams`, `ListTagsForResource`, `ListTagsForStream`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateAccountSettings`, `UpdateMaxRecordSize`, `UpdateShardCount`, `UpdateStreamMode`, `UpdateStreamWarmThroughput`
- Common required input members in this group: -

### Get

- Operations: `GetRecords`, `GetResourcePolicy`, `GetShardIterator`
- Common required input members in this group: -

### Put

- Operations: `PutRecord`, `PutRecords`, `PutResourcePolicy`
- Common required input members in this group: -

### Delete

- Operations: `DeleteResourcePolicy`, `DeleteStream`
- Common required input members in this group: -

### Add

- Operations: `AddTagsToStream`
- Common required input members in this group: -

### Create

- Operations: `CreateStream`
- Common required input members in this group: -

### Decrease

- Operations: `DecreaseStreamRetentionPeriod`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterStreamConsumer`
- Common required input members in this group: -

### Disable

- Operations: `DisableEnhancedMonitoring`
- Common required input members in this group: -

### Enable

- Operations: `EnableEnhancedMonitoring`
- Common required input members in this group: -

### Increase

- Operations: `IncreaseStreamRetentionPeriod`
- Common required input members in this group: -

### Merge

- Operations: `MergeShards`
- Common required input members in this group: -

### Register

- Operations: `RegisterStreamConsumer`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromStream`
- Common required input members in this group: -

### Split

- Operations: `SplitShard`
- Common required input members in this group: -

### Start

- Operations: `StartStreamEncryption`
- Common required input members in this group: -

### Stop

- Operations: `StopStreamEncryption`
- Common required input members in this group: -

### Subscribe

- Operations: `SubscribeToShard`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToStream` | `-` | - | `Tags` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds or updates tags for the specified Kinesis data stream. You can assign up to 50 tags to a data stream. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recomme ... |
| `CreateStream` | `-` | - | `StreamName` | - | `Unit` | `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ValidationException` | Creates a Kinesis data stream. A stream captures and transports data records that are continuously emitted from different data sources or producers . Scale-out within a stream is explicitly supported by means of shar ... |
| `DecreaseStreamRetentionPeriod` | `-` | - | `RetentionPeriodHours` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Decreases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The minimum value of a stream's retention period is 24 hours. When inv ... |
| `DeleteResourcePolicy` | `-` | - | `ResourceARN` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Delete a policy for the specified data stream or consumer. Request patterns can be one of the following: Data stream pattern: arn:aws.*:kinesis:.*:\d{12}:.*stream/\S+ Consumer pattern: ^(arn):aws.*:kinesis:.*:\d{12}: ... |
| `DeleteStream` | `-` | - | - | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a Kinesis data stream and all its shards and data. You must shut down any applications that are operating on the stream before you delete the stream. If an application attempts to operate on a deleted stream, ... |
| `DeregisterStreamConsumer` | `-` | - | - | - | `Unit` | `InvalidArgumentException`, `LimitExceededException`, `ResourceNotFoundException` | To deregister a consumer, provide its ARN. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also provide all three parameters, as long as th ... |
| `DescribeAccountSettings` | `-` | - | - | - | `DescribeAccountSettingsOutput` | `LimitExceededException` | Describes the account-level settings for Amazon Kinesis Data Streams. This operation returns information about the minimum throughput billing commitments and other account-level configurations. This API has a call li ... |
| `DescribeLimits` | `-` | - | - | - | `DescribeLimitsOutput` | `LimitExceededException` | Describes the shard limits and usage for the account. If you update your account limits, the old limits might be returned for a few minutes. This operation has a limit of one transaction per second per account. |
| `DescribeStream` | `-` | - | - | - | `DescribeStreamOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceNotFoundException` | Describes the specified Kinesis data stream. This API has been revised. It's highly recommended that you use the DescribeStreamSummary API to get a summarized description of the specified Kinesis data stream and the ... |
| `DescribeStreamConsumer` | `-` | - | - | - | `DescribeStreamConsumerOutput` | `InvalidArgumentException`, `LimitExceededException`, `ResourceNotFoundException` | To get the description of a registered consumer, provide the ARN of the consumer. Alternatively, you can provide the ARN of the data stream and the name you gave the consumer when you registered it. You may also prov ... |
| `DescribeStreamSummary` | `-` | - | - | - | `DescribeStreamSummaryOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceNotFoundException` | Provides a summarized description of the specified Kinesis data stream without the shard list. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recommended that yo ... |
| `DisableEnhancedMonitoring` | `-` | - | `ShardLevelMetrics` | - | `EnhancedMonitoringOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Disables enhanced monitoring. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recommended that you use the StreamARN input parameter when you invoke this API. |
| `EnableEnhancedMonitoring` | `-` | - | `ShardLevelMetrics` | - | `EnhancedMonitoringOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Enables enhanced Kinesis data stream monitoring for shard-level metrics. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recommended that you use the StreamARN in ... |
| `GetRecords` | `-` | - | `ShardIterator` | - | `GetRecordsOutput` | `AccessDeniedException`, `ExpiredIteratorException`, `InternalFailureException`, `InvalidArgumentException`, `KMSAccessDeniedException`, `KMSDisabledException`, `KMSInvalidStateException`, `KMSNotFoundException`, `KMSOptInRequired`, `KMSThrottlingException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException` | Gets data records from a Kinesis data stream's shard. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recommended that you use the StreamARN input parameter when ... |
| `GetResourcePolicy` | `-` | - | `ResourceARN` | - | `GetResourcePolicyOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Returns a policy attached to the specified data stream or consumer. Request patterns can be one of the following: Data stream pattern: arn:aws.*:kinesis:.*:\d{12}:.*stream/\S+ Consumer pattern: ^(arn):aws.*:kinesis:. ... |
| `GetShardIterator` | `-` | - | `ShardId`, `ShardIteratorType` | - | `GetShardIteratorOutput` | `AccessDeniedException`, `InternalFailureException`, `InvalidArgumentException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException` | Gets an Amazon Kinesis shard iterator. A shard iterator expires 5 minutes after it is returned to the requester. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is r ... |
| `IncreaseStreamRetentionPeriod` | `-` | - | `RetentionPeriodHours` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Increases the Kinesis data stream's retention period, which is the length of time data records are accessible after they are added to the stream. The maximum value of a stream's retention period is 8760 hours (365 da ... |
| `ListShards` | `-` | - | - | - | `ListShardsOutput` | `AccessDeniedException`, `ExpiredNextTokenException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Lists the shards in a stream and provides information about each shard. This operation has a limit of 1000 transactions per second per data stream. When invoking this API, you must use either the StreamARN or the Str ... |
| `ListStreamConsumers` | `-` | `paginated` | `StreamARN` | - | `ListStreamConsumersOutput` | `ExpiredNextTokenException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Lists the consumers registered to receive data from a stream using enhanced fan-out, and provides information about each consumer. This operation has a limit of 5 transactions per second per stream. |
| `ListStreams` | `-` | `paginated` | - | - | `ListStreamsOutput` | `ExpiredNextTokenException`, `InvalidArgumentException`, `LimitExceededException` | Lists your Kinesis data streams. The number of streams may be too large to return from a single call to ListStreams . You can limit the number of returned streams using the Limit parameter. If you do not specify a va ... |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | List all tags added to the specified Kinesis resource. Each tag is a label consisting of a user-defined key and value. Tags can help you manage, identify, organize, search for, and filter resources. For more informat ... |
| `ListTagsForStream` | `-` | - | - | - | `ListTagsForStreamOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceNotFoundException` | Lists the tags for the specified Kinesis data stream. This operation has a limit of five transactions per second per account. When invoking this API, you must use either the StreamARN or the StreamName parameter, or ... |
| `MergeShards` | `-` | - | `ShardToMerge`, `AdjacentShardToMerge` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Merges two adjacent shards in a Kinesis data stream and combines them into a single shard to reduce the stream's capacity to ingest and transport data. This API is only supported for the data streams with the provisi ... |
| `PutRecord` | `-` | - | `Data`, `PartitionKey` | - | `PutRecordOutput` | `AccessDeniedException`, `InternalFailureException`, `InvalidArgumentException`, `KMSAccessDeniedException`, `KMSDisabledException`, `KMSInvalidStateException`, `KMSNotFoundException`, `KMSOptInRequired`, `KMSThrottlingException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException` | Writes a single data record into an Amazon Kinesis data stream. Call PutRecord to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to ... |
| `PutRecords` | `-` | - | `Records` | - | `PutRecordsOutput` | `AccessDeniedException`, `InternalFailureException`, `InvalidArgumentException`, `KMSAccessDeniedException`, `KMSDisabledException`, `KMSInvalidStateException`, `KMSNotFoundException`, `KMSOptInRequired`, `KMSThrottlingException`, `ProvisionedThroughputExceededException`, `ResourceNotFoundException` | Writes multiple data records into a Kinesis data stream in a single call (also referred to as a PutRecords request). Use this operation to send data into the stream for data ingestion and processing. When invoking th ... |
| `PutResourcePolicy` | `-` | - | `ResourceARN`, `Policy` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Attaches a resource-based policy to a data stream or registered consumer. If you are using an identity other than the root user of the Amazon Web Services account that owns the resource, the calling identity must hav ... |
| `RegisterStreamConsumer` | `-` | - | `StreamARN`, `ConsumerName` | - | `RegisterStreamConsumerOutput` | `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Registers a consumer with a Kinesis data stream. When you use this operation, the consumer you register can then call SubscribeToShard to receive data from the stream using enhanced fan-out, at a rate of up to 2 MiB ... |
| `RemoveTagsFromStream` | `-` | - | `TagKeys` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Removes tags from the specified Kinesis data stream. Removed tags are deleted and cannot be recovered after this operation successfully completes. When invoking this API, you must use either the StreamARN or the Stre ... |
| `SplitShard` | `-` | - | `ShardToSplit`, `NewStartingHashKey` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Splits a shard into two new shards in the Kinesis data stream, to increase the stream's capacity to ingest and transport data. SplitShard is called when there is a need to increase the overall capacity of a stream be ... |
| `StartStreamEncryption` | `-` | - | `EncryptionType`, `KeyId` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `KMSAccessDeniedException`, `KMSDisabledException`, `KMSInvalidStateException`, `KMSNotFoundException`, `KMSOptInRequired`, `KMSThrottlingException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Enables or updates server-side encryption using an Amazon Web Services KMS key for a specified stream. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recommended ... |
| `StopStreamEncryption` | `-` | - | `EncryptionType`, `KeyId` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Disables server-side encryption for a specified stream. When invoking this API, you must use either the StreamARN or the StreamName parameter, or both. It is recommended that you use the StreamARN input parameter whe ... |
| `SubscribeToShard` | `-` | - | `ConsumerARN`, `ShardId`, `StartingPosition` | - | `SubscribeToShardOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | This operation establishes an HTTP/2 connection between the consumer you specify in the ConsumerARN parameter and the shard you specify in the ShardId parameter. After the connection is successfully established, Kine ... |
| `TagResource` | `-` | - | `Tags`, `ResourceARN` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds or updates tags for the specified Kinesis resource. Each tag is a label consisting of a user-defined key and value. Tags can help you manage, identify, organize, search for, and filter resources. You can assign ... |
| `UntagResource` | `-` | - | `TagKeys`, `ResourceARN` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Removes tags from the specified Kinesis resource. Removed tags are deleted and can't be recovered after this operation completes successfully. |
| `UpdateAccountSettings` | `-` | - | `MinimumThroughputBillingCommitment` | - | `UpdateAccountSettingsOutput` | `InvalidArgumentException`, `LimitExceededException`, `ValidationException` | Updates the account-level settings for Amazon Kinesis Data Streams. Updating account settings is a synchronous operation. Upon receiving the request, Kinesis Data Streams will return immediately with your account’s u ... |
| `UpdateMaxRecordSize` | `-` | - | `MaxRecordSizeInKiB` | - | `Unit` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | This allows you to update the MaxRecordSize of a single record that you can write to, and read from a stream. You can ingest and digest single records up to 10240 KiB. |
| `UpdateShardCount` | `-` | - | `TargetShardCount`, `ScalingType` | - | `UpdateShardCountOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the shard count of the specified stream to the specified number of shards. This API is only supported for the data streams with the provisioned capacity mode. When invoking this API, you must use either the S ... |
| `UpdateStreamMode` | `-` | - | `StreamARN`, `StreamModeDetails` | - | `Unit` | `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the capacity mode of the data stream. Currently, in Kinesis Data Streams, you can choose between an on-demand capacity mode and a provisioned capacity mode for your data stream. If you'd still like to proacti ... |
| `UpdateStreamWarmThroughput` | `-` | - | `WarmThroughputMiBps` | - | `UpdateStreamWarmThroughputOutput` | `AccessDeniedException`, `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the warm throughput configuration for the specified Amazon Kinesis Data Streams on-demand data stream. This operation allows you to proactively scale your on-demand data stream to a specified throughput level ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | Specifies that you do not have the permissions required to perform this operation. |
| `ExpiredIteratorException` | `structure` | message | The provided iterator exceeds the maximum age allowed. |
| `ExpiredNextTokenException` | `structure` | message | The pagination token passed to the operation is expired. |
| `InternalFailureException` | `structure` | message | The processing of the request failed because of an unknown error, exception, or failure. |
| `InvalidArgumentException` | `structure` | message | A specified parameter exceeds its restrictions, is not supported, or can't be used. For more information, see the returned message. |
| `KMSAccessDeniedException` | `structure` | message | The ciphertext references a key that doesn't exist or that you don't have access to. |
| `KMSDisabledException` | `structure` | message | The request was rejected because the specified customer master key (CMK) isn't enabled. |
| `KMSInvalidStateException` | `structure` | message | The request was rejected because the state of the specified resource isn't valid for this request. For more information, see How Key State Affects Use of a ... |
| `KMSNotFoundException` | `structure` | message | The request was rejected because the specified entity or resource can't be found. |
| `KMSOptInRequired` | `structure` | message | The Amazon Web Services access key ID needs a subscription for the service. |
| `KMSThrottlingException` | `structure` | message | The request was denied due to request throttling. For more information about throttling, see Limits in the Amazon Web Services Key Management Service Develo ... |
| `LimitExceededException` | `structure` | message | The requested resource exceeds the maximum number allowed, or the number of concurrent stream requests exceeds the maximum number allowed. |
| `ProvisionedThroughputExceededException` | `structure` | message | The request rate for the stream is too high, or the requested data is too large for the available throughput. Reduce the frequency or size of your requests. ... |
| `ResourceInUseException` | `structure` | message | The resource is not available for this operation. For successful operation, the resource must be in the ACTIVE state. |
| `ResourceNotFoundException` | `structure` | message | The requested resource could not be found. The stream might not be specified correctly. |
| `ValidationException` | `structure` | message | Specifies that you tried to invoke this API for a data stream with the on-demand capacity mode. This API is only supported for data streams with the provisi ... |
| `AddTagsToStreamInput` | `structure` | StreamName, Tags, StreamARN, StreamId | Represents the input for AddTagsToStream . |
| `CreateStreamInput` | `structure` | StreamName, ShardCount, StreamModeDetails, Tags, WarmThroughputMiBps, MaxRecordSizeInKiB | Represents the input for CreateStream . |
| `DecreaseStreamRetentionPeriodInput` | `structure` | StreamName, RetentionPeriodHours, StreamARN, StreamId | Represents the input for DecreaseStreamRetentionPeriod . |
| `DeleteResourcePolicyInput` | `structure` | ResourceARN, StreamId | - |
| `DeleteStreamInput` | `structure` | StreamName, EnforceConsumerDeletion, StreamARN, StreamId | Represents the input for DeleteStream . |
| `DeregisterStreamConsumerInput` | `structure` | StreamARN, ConsumerName, ConsumerARN, StreamId | - |
| `DescribeAccountSettingsInput` | `structure` | **empty (no members)** | - |
| `DescribeAccountSettingsOutput` | `structure` | MinimumThroughputBillingCommitment | - |
| `DescribeLimitsInput` | `structure` | **empty (no members)** | - |
| `DescribeLimitsOutput` | `structure` | ShardLimit, OpenShardCount, OnDemandStreamCount, OnDemandStreamCountLimit | - |
| `DescribeStreamInput` | `structure` | StreamName, Limit, ExclusiveStartShardId, StreamARN, StreamId | Represents the input for DescribeStream . |
| `DescribeStreamOutput` | `structure` | StreamDescription | Represents the output for DescribeStream . |
| `DescribeStreamConsumerInput` | `structure` | StreamARN, ConsumerName, ConsumerARN, StreamId | - |
| `DescribeStreamConsumerOutput` | `structure` | ConsumerDescription | - |
| `DescribeStreamSummaryInput` | `structure` | StreamName, StreamARN, StreamId | - |
| `DescribeStreamSummaryOutput` | `structure` | StreamDescriptionSummary | - |
| `DisableEnhancedMonitoringInput` | `structure` | StreamName, ShardLevelMetrics, StreamARN, StreamId | Represents the input for DisableEnhancedMonitoring . |
| `EnhancedMonitoringOutput` | `structure` | StreamName, CurrentShardLevelMetrics, DesiredShardLevelMetrics, StreamARN | Represents the output for EnableEnhancedMonitoring and DisableEnhancedMonitoring . |
| `EnableEnhancedMonitoringInput` | `structure` | StreamName, ShardLevelMetrics, StreamARN, StreamId | Represents the input for EnableEnhancedMonitoring . |
| `GetRecordsInput` | `structure` | ShardIterator, Limit, StreamARN, StreamId | Represents the input for GetRecords . |
| `GetRecordsOutput` | `structure` | Records, NextShardIterator, MillisBehindLatest, ChildShards | Represents the output for GetRecords . |
| `GetResourcePolicyInput` | `structure` | ResourceARN, StreamId | - |
| `GetResourcePolicyOutput` | `structure` | Policy | - |
| `GetShardIteratorInput` | `structure` | StreamName, ShardId, ShardIteratorType, StartingSequenceNumber, Timestamp, StreamARN, StreamId | Represents the input for GetShardIterator . |
| `ConsumerStatus` | `enum` | CREATING, DELETING, ACTIVE | - |
| `EncryptionType` | `enum` | NONE, KMS | - |
| `MetricsName` | `enum` | INCOMING_BYTES, INCOMING_RECORDS, OUTGOING_BYTES, OUTGOING_RECORDS, WRITE_PROVISIONED_THROUGHPUT_EXCEEDED, READ_PROVISIONED_THROUGHPUT_EXCEEDED, ITERATOR_AGE_MILLISECONDS, ALL | - |
| `MinimumThroughputBillingCommitmentInputStatus` | `enum` | ENABLED, DISABLED | - |
| `MinimumThroughputBillingCommitmentOutputStatus` | `enum` | ENABLED, DISABLED, ENABLED_UNTIL_EARLIEST_ALLOWED_END | - |
| `ScalingType` | `enum` | UNIFORM_SCALING | - |
| `ShardFilterType` | `enum` | AFTER_SHARD_ID, AT_TRIM_HORIZON, FROM_TRIM_HORIZON, AT_LATEST, AT_TIMESTAMP, FROM_TIMESTAMP | - |
| `ShardIteratorType` | `enum` | AT_SEQUENCE_NUMBER, AFTER_SEQUENCE_NUMBER, TRIM_HORIZON, LATEST, AT_TIMESTAMP | - |
| `StreamMode` | `enum` | PROVISIONED, ON_DEMAND | - |
| `StreamStatus` | `enum` | CREATING, DELETING, ACTIVE, UPDATING | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises Kinesis' high-value cross-service paths. Open it for Lambda event-source mappings, EventBridge Pipes sources, and EventBridge rule targets.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises source-consumption versus target-delivery boundaries. Open it before implementing Kinesis as either an event source or a target sink.
- Service implication: EventBridge can target Kinesis or Firehose from rules, so target execution work should preserve stream payload and partition-key semantics instead of treating Kinesis as a generic sink.
- Service implication: cross-service tests should cover source consumption and target delivery separately because AWS models those contracts through different services.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

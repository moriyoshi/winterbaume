# Amazon Kinesis Firehose

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Data Firehose Amazon Data Firehose was previously known as Amazon Kinesis Data Firehose. Amazon Data Firehose is a fully managed service that delivers real-time streaming data to destinations such as Amazon Simple Storage Service (Amazon S3), Amazon OpenSearch Service, Amazon Redshift, Splunk, and various other supported destinations.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Kinesis Firehose resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Kinesis Firehose workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Put`, `Create`, `Delete`, `Describe` operation families, including `ListDeliveryStreams`, `ListTagsForDeliveryStream`, `PutRecord`, `PutRecordBatch`, `CreateDeliveryStream`, `DeleteDeliveryStream`.

## Service Identity and Protocol

- AWS model slug: `firehose`
- AWS SDK for Rust slug: `firehose`
- Model version: `2015-08-04`
- Model file: `vendor/api-models-aws/models/firehose/service/2015-08-04/firehose-2015-08-04.json`
- SDK ID: `Firehose`
- Endpoint prefix: `firehose`
- ARN namespace: `firehose`
- CloudFormation name: `KinesisFirehose`
- CloudTrail event source: `firehose.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Put` (2), `Create` (1), `Delete` (1), `Describe` (1), `Start` (1), `Stop` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDeliveryStream`, `DeleteDeliveryStream`, `PutRecord`, `PutRecordBatch`, `StartDeliveryStreamEncryption`, `StopDeliveryStreamEncryption`, `TagDeliveryStream`, `UntagDeliveryStream`, `UpdateDestination`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDeliveryStream`, `ListDeliveryStreams`, `ListTagsForDeliveryStream`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDeliveryStreamEncryption`, `StopDeliveryStreamEncryption`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `Glue`, `EC2/VPC`, `ECS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/firehose/latest/dev/create-configure-backup.html
- https://docs.aws.amazon.com/firehose/latest/dev/retry.html
- https://docs.aws.amazon.com/firehose/latest/dev/data-transformation-source-record-backup.html

Research outcomes:
- Firehose buffers incoming records by size and interval before delivery. The first satisfied buffer condition triggers delivery.
- If either buffering size or buffering interval is specified, both must be provided.
- Buffering hints are destination-specific and applied on a shard or partition level. Dynamic partitioning buffering hints are applied at stream or topic level.
- Zero-second buffering is supported for application destinations, but not for S3 backup destinations or dynamic partitioning.
- For S3 delivery failures, Firehose retries for up to 24 hours for DirectPut sources, and for Kinesis Data Streams or MSK sources retries up to the source retention policy.
- If S3 delivery fails for longer than the maximum storage/retention window, data is lost.
- For Redshift, OpenSearch, Splunk, HTTP endpoints, and other destinations, retry durations and backup/error output are destination-specific.
- If Firehose cannot deliver to the destination and also cannot write failed records to the backup S3 bucket, stream delivery is effectively paused until one path succeeds.

Parity implications:
- Model delivery streams, sources, destinations, buffering state, Lambda transformation, format conversion, dynamic partitioning, backup destinations, retry windows, and error prefixes.
- Record ingestion should accumulate per-partition buffers and flush on size or interval.
- Delivery failure handling must be destination-specific and preserve S3 backup/error behaviour, retry expiry, paused delivery, and data-loss conditions.

## Operation Groups

### List

- Operations: `ListDeliveryStreams`, `ListTagsForDeliveryStream`
- Common required input members in this group: -

### Put

- Operations: `PutRecord`, `PutRecordBatch`
- Common required input members in this group: `DeliveryStreamName`

### Create

- Operations: `CreateDeliveryStream`
- Common required input members in this group: -

### Delete

- Operations: `DeleteDeliveryStream`
- Common required input members in this group: -

### Describe

- Operations: `DescribeDeliveryStream`
- Common required input members in this group: -

### Start

- Operations: `StartDeliveryStreamEncryption`
- Common required input members in this group: -

### Stop

- Operations: `StopDeliveryStreamEncryption`
- Common required input members in this group: -

### Tag

- Operations: `TagDeliveryStream`
- Common required input members in this group: -

### Untag

- Operations: `UntagDeliveryStream`
- Common required input members in this group: -

### Update

- Operations: `UpdateDestination`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateDeliveryStream` | `-` | - | `DeliveryStreamName` | - | `CreateDeliveryStreamOutput` | `InvalidArgumentException`, `InvalidKMSResourceException`, `LimitExceededException`, `ResourceInUseException` | Creates a Firehose stream. By default, you can create up to 5,000 Firehose streams per Amazon Web Services Region. This is an asynchronous operation that immediately returns. The initial status of the Firehose stream ... |
| `DeleteDeliveryStream` | `-` | - | `DeliveryStreamName` | - | `DeleteDeliveryStreamOutput` | `ResourceInUseException`, `ResourceNotFoundException` | Deletes a Firehose stream and its data. You can delete a Firehose stream only if it is in one of the following states: ACTIVE , DELETING , CREATING_FAILED , or DELETING_FAILED . You can't delete a Firehose stream tha ... |
| `DescribeDeliveryStream` | `-` | - | `DeliveryStreamName` | - | `DescribeDeliveryStreamOutput` | `ResourceNotFoundException` | Describes the specified Firehose stream and its status. For example, after your Firehose stream is created, call DescribeDeliveryStream to see whether the Firehose stream is ACTIVE and therefore ready for data to be ... |
| `ListDeliveryStreams` | `-` | - | - | - | `ListDeliveryStreamsOutput` | - | Lists your Firehose streams in alphabetical order of their names. The number of Firehose streams might be too large to return using a single call to ListDeliveryStreams . You can limit the number of Firehose streams ... |
| `ListTagsForDeliveryStream` | `-` | - | `DeliveryStreamName` | - | `ListTagsForDeliveryStreamOutput` | `InvalidArgumentException`, `LimitExceededException`, `ResourceNotFoundException` | Lists the tags for the specified Firehose stream. This operation has a limit of five transactions per second per account. |
| `PutRecord` | `-` | - | `DeliveryStreamName`, `Record` | - | `PutRecordOutput` | `InvalidArgumentException`, `InvalidKMSResourceException`, `InvalidSourceException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Writes a single data record into an Firehose stream. To write multiple data records into a Firehose stream, use PutRecordBatch . Applications using these operations are referred to as producers. By default, each Fire ... |
| `PutRecordBatch` | `-` | - | `DeliveryStreamName`, `Records` | - | `PutRecordBatchOutput` | `InvalidArgumentException`, `InvalidKMSResourceException`, `InvalidSourceException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Writes multiple data records into a Firehose stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a Firehose stream, use PutRec ... |
| `StartDeliveryStreamEncryption` | `-` | - | `DeliveryStreamName` | - | `StartDeliveryStreamEncryptionOutput` | `InvalidArgumentException`, `InvalidKMSResourceException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Enables server-side encryption (SSE) for the Firehose stream. This operation is asynchronous. It returns immediately. When you invoke it, Firehose first sets the encryption status of the stream to ENABLING , and then ... |
| `StopDeliveryStreamEncryption` | `-` | - | `DeliveryStreamName` | - | `StopDeliveryStreamEncryptionOutput` | `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Disables server-side encryption (SSE) for the Firehose stream. This operation is asynchronous. It returns immediately. When you invoke it, Firehose first sets the encryption status of the stream to DISABLING , and th ... |
| `TagDeliveryStream` | `-` | - | `DeliveryStreamName`, `Tags` | - | `TagDeliveryStreamOutput` | `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Adds or updates tags for the specified Firehose stream. A tag is a key-value pair that you can define and assign to Amazon Web Services resources. If you specify a tag that already exists, the tag value is replaced w ... |
| `UntagDeliveryStream` | `-` | - | `DeliveryStreamName`, `TagKeys` | - | `UntagDeliveryStreamOutput` | `InvalidArgumentException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Removes tags from the specified Firehose stream. Removed tags are deleted, and you can't recover them after this operation successfully completes. If you specify a tag that doesn't exist, the operation ignores it. Th ... |
| `UpdateDestination` | `-` | - | `DeliveryStreamName`, `CurrentDeliveryStreamVersionId`, `DestinationId` | - | `UpdateDestinationOutput` | `ConcurrentModificationException`, `InvalidArgumentException`, `ResourceInUseException`, `ResourceNotFoundException` | Updates the specified destination of the specified Firehose stream. Use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentModificationException` | `structure` | message | Another modification has already happened. Fetch VersionId again and use it to update the destination. |
| `InvalidArgumentException` | `structure` | message | The specified input parameter has a value that is not valid. |
| `InvalidKMSResourceException` | `structure` | code, message | Firehose throws this exception when an attempt to put records or to start or stop Firehose stream encryption fails. This happens when the KMS service throws ... |
| `InvalidSourceException` | `structure` | code, message | Only requests from CloudWatch Logs are supported when CloudWatch Logs decompression is enabled. |
| `LimitExceededException` | `structure` | message | You have already reached the limit for a requested resource. |
| `ResourceInUseException` | `structure` | message | The resource is already in use and not available for this operation. |
| `ResourceNotFoundException` | `structure` | message | The specified resource could not be found. |
| `ServiceUnavailableException` | `structure` | message | The service is unavailable. Back off and retry the operation. If you continue to see the exception, throughput limits for the Firehose stream may have been ... |
| `CreateDeliveryStreamInput` | `structure` | DeliveryStreamName, DeliveryStreamType, DirectPutSourceConfiguration, KinesisStreamSourceConfiguration, DeliveryStreamEncryptionConfigurationInput, S3DestinationConfiguration, ExtendedS3DestinationConfiguration, RedshiftDestinationConfiguration, ElasticsearchDestinationConfiguration, AmazonopensearchserviceDestinationConfiguration, SplunkDestinationConfiguration, HttpEndpointDestinationConfiguration, ... (+6) | - |
| `CreateDeliveryStreamOutput` | `structure` | DeliveryStreamARN | - |
| `DeleteDeliveryStreamInput` | `structure` | DeliveryStreamName, AllowForceDelete | - |
| `DeleteDeliveryStreamOutput` | `structure` | **empty (no members)** | - |
| `DescribeDeliveryStreamInput` | `structure` | DeliveryStreamName, Limit, ExclusiveStartDestinationId | - |
| `DescribeDeliveryStreamOutput` | `structure` | DeliveryStreamDescription | - |
| `ListDeliveryStreamsInput` | `structure` | Limit, DeliveryStreamType, ExclusiveStartDeliveryStreamName | - |
| `ListDeliveryStreamsOutput` | `structure` | DeliveryStreamNames, HasMoreDeliveryStreams | - |
| `ListTagsForDeliveryStreamInput` | `structure` | DeliveryStreamName, ExclusiveStartTagKey, Limit | - |
| `ListTagsForDeliveryStreamOutput` | `structure` | Tags, HasMoreTags | - |
| `PutRecordInput` | `structure` | DeliveryStreamName, Record | - |
| `PutRecordOutput` | `structure` | RecordId, Encrypted | - |
| `PutRecordBatchInput` | `structure` | DeliveryStreamName, Records | - |
| `PutRecordBatchOutput` | `structure` | FailedPutCount, Encrypted, RequestResponses | - |
| `StartDeliveryStreamEncryptionInput` | `structure` | DeliveryStreamName, DeliveryStreamEncryptionConfigurationInput | - |
| `StartDeliveryStreamEncryptionOutput` | `structure` | **empty (no members)** | - |
| `StopDeliveryStreamEncryptionInput` | `structure` | DeliveryStreamName | - |
| `StopDeliveryStreamEncryptionOutput` | `structure` | **empty (no members)** | - |
| `TagDeliveryStreamInput` | `structure` | DeliveryStreamName, Tags | - |
| `TagDeliveryStreamOutput` | `structure` | **empty (no members)** | - |
| `UntagDeliveryStreamInput` | `structure` | DeliveryStreamName, TagKeys | - |
| `UntagDeliveryStreamOutput` | `structure` | **empty (no members)** | - |
| `UpdateDestinationInput` | `structure` | DeliveryStreamName, CurrentDeliveryStreamVersionId, DestinationId, S3DestinationUpdate, ExtendedS3DestinationUpdate, RedshiftDestinationUpdate, ElasticsearchDestinationUpdate, AmazonopensearchserviceDestinationUpdate, SplunkDestinationUpdate, HttpEndpointDestinationUpdate, AmazonOpenSearchServerlessDestinationUpdate, SnowflakeDestinationUpdate, ... (+1) | - |
| `UpdateDestinationOutput` | `structure` | **empty (no members)** | - |
| `AmazonOpenSearchServerlessS3BackupMode` | `enum` | FailedDocumentsOnly, AllDocuments | - |
| `AmazonopensearchserviceIndexRotationPeriod` | `enum` | NoRotation, OneHour, OneDay, OneWeek, OneMonth | - |
| `AmazonopensearchserviceS3BackupMode` | `enum` | FailedDocumentsOnly, AllDocuments | - |
| `CompressionFormat` | `enum` | UNCOMPRESSED, GZIP, ZIP, SNAPPY, HADOOP_SNAPPY | - |
| `Connectivity` | `enum` | PUBLIC, PRIVATE | - |
| `ContentEncoding` | `enum` | NONE, GZIP | - |
| `DatabaseType` | `enum` | MySQL, PostgreSQL | - |
| `DefaultDocumentIdFormat` | `enum` | FIREHOSE_DEFAULT, NO_DOCUMENT_ID | - |
| `DeliveryStreamEncryptionStatus` | `enum` | ENABLED, ENABLING, ENABLING_FAILED, DISABLED, DISABLING, DISABLING_FAILED | - |
| `DeliveryStreamFailureType` | `enum` | VPC_ENDPOINT_SERVICE_NAME_NOT_FOUND, VPC_INTERFACE_ENDPOINT_SERVICE_ACCESS_DENIED, RETIRE_KMS_GRANT_FAILED, CREATE_KMS_GRANT_FAILED, KMS_ACCESS_DENIED, DISABLED_KMS_KEY, INVALID_KMS_KEY, KMS_KEY_NOT_FOUND, KMS_OPT_IN_REQUIRED, CREATE_ENI_FAILED, DELETE_ENI_FAILED, SUBNET_NOT_FOUND, ... (+5) | - |
| `DeliveryStreamStatus` | `enum` | CREATING, CREATING_FAILED, DELETING, DELETING_FAILED, ACTIVE | - |
| `DeliveryStreamType` | `enum` | DirectPut, KinesisStreamAsSource, MSKAsSource, DatabaseAsSource | - |
| `ElasticsearchIndexRotationPeriod` | `enum` | NoRotation, OneHour, OneDay, OneWeek, OneMonth | - |
| `ElasticsearchS3BackupMode` | `enum` | FailedDocumentsOnly, AllDocuments | - |
| `HECEndpointType` | `enum` | Raw, Event | - |
| `HttpEndpointS3BackupMode` | `enum` | FailedDataOnly, AllData | - |
| `IcebergS3BackupMode` | `enum` | FailedDataOnly, AllData | - |
| `KeyType` | `enum` | AWS_OWNED_CMK, CUSTOMER_MANAGED_CMK | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

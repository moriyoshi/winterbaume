# Amazon Keyspaces Streams

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Keyspaces (for Apache Cassandra) change data capture (CDC) records change events for Amazon Keyspaces tables. The change events captured in a stream are time-ordered and de-duplicated write operations. Using stream data you can build event driven applications that incorporate near-real time change events from Amazon Keyspaces tables. Amazon Keyspaces CDC is serverless and scales the infrastructure for change events automatically based on the volume of changes on your table. This API reference describes the Amazon Keyspaces CDC stream API in detail.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Keyspaces Streams workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `List` operation families, including `GetRecords`, `GetShardIterator`, `GetStream`, `ListStreams`.

## Service Identity and Protocol

- AWS model slug: `keyspacesstreams`
- AWS SDK for Rust slug: `keyspacesstreams`
- Model version: `2024-09-09`
- Model file: `vendor/api-models-aws/models/keyspacesstreams/service/2024-09-09/keyspacesstreams-2024-09-09.json`
- SDK ID: `KeyspacesStreams`
- Endpoint prefix: `cassandra-streams`
- ARN namespace: `cassandra`
- CloudFormation name: `-`
- CloudTrail event source: `cassandra-streams.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `List` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRecords`, `GetShardIterator`, `GetStream`, `ListStreams`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.

## Operation Groups

### Get

- Operations: `GetRecords`, `GetShardIterator`, `GetStream`
- Traits: `paginated` (1)
- Common required input members in this group: `shardId`, `shardIterator`, `shardIteratorType`, `streamArn`

### List

- Operations: `ListStreams`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetRecords` | - | - | `shardIterator` | - | `GetRecordsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves data records from a specified shard in an Amazon Keyspaces data stream. This operation returns a collection of data records from the shard, including the primary key columns and information about modifications made to the captured table data. |
| `GetShardIterator` | - | - | `shardId`, `shardIteratorType`, `streamArn` | - | `GetShardIteratorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a shard iterator that serves as a bookmark for reading data from a specific position in an Amazon Keyspaces data stream's shard. The shard iterator specifies the shard position from which to start reading data records sequentially. |
| `GetStream` | - | `paginated` | `streamArn` | - | `GetStreamOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns detailed information about a specific data capture stream for an Amazon Keyspaces table. The information includes the stream's Amazon Resource Name (ARN), creation time, current status, retention period, shard composition, and associated table details. |
| `ListStreams` | - | `paginated` | - | - | `ListStreamsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all data capture streams associated with your Amazon Keyspaces account or for a specific keyspace or table. The response includes information such as stream ARNs, table associations, creation timestamps, and current status. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access permissions to perform this operation. |
| `InternalServerException` | `structure` | `message` | The Amazon Keyspaces service encountered an unexpected error while processing the request. |
| `ResourceNotFoundException` | `structure` | `message` | The requested resource doesn't exist or could not be found. |
| `ThrottlingException` | `structure` | `message` | The request rate is too high and exceeds the service's throughput limits. |
| `ValidationException` | `structure` | `errorCode`, `message` | The request validation failed because one or more input parameters failed validation. |
| `GetRecordsInput` | `structure` | `maxResults`, `shardIterator` | - |
| `GetRecordsOutput` | `structure` | `changeRecords`, `nextShardIterator` | - |
| `GetShardIteratorInput` | `structure` | `sequenceNumber`, `shardId`, `shardIteratorType`, `streamArn` | - |
| `GetShardIteratorOutput` | `structure` | `shardIterator` | - |
| `GetStreamInput` | `structure` | `maxResults`, `nextToken`, `shardFilter`, `streamArn` | - |
| `GetStreamOutput` | `structure` | `creationRequestDateTime`, `keyspaceName`, `nextToken`, `shards`, `streamArn`, `streamLabel`, `streamStatus`, `streamViewType`, `tableName` | - |
| `ListStreamsInput` | `structure` | `keyspaceName`, `maxResults`, `nextToken`, `tableName` | - |
| `ListStreamsOutput` | `structure` | `nextToken`, `streams` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

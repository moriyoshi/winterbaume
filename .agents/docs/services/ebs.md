# Amazon Elastic Block Store

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Amazon Elastic Block Store (Amazon EBS) direct APIs to create Amazon EBS snapshots, write data directly to your snapshots, read data on your snapshots, and identify the differences or changes between two snapshots. If you’re an independent software vendor (ISV) who offers backup services for Amazon EBS, the EBS direct APIs make it more efficient and cost-effective to track incremental changes on your Amazon EBS volumes through snapshots. This can be done without having to create new volumes from snapshots, and then use Amazon Elastic Compute Cloud (Amazon EC2) instances to compare the differences. You can create incremental snapshots directly from data on-premises into volumes and the cloud to use for quick disaster recovery. With the ability to write and read snapshots, you can write your on-premises data to an snapshot during a disaster.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-ebs/tests/scenario_test.rs`: create a snapshot write workflow with block writes, completion, and readback validation.
- Backported from `scenario_test.rs`: compute incremental snapshot differences between snapshots.
- Backported from `scenario_test.rs`: enforce snapshot state-machine guardrails for invalid block and lifecycle operations.
- From the AWS documentation and model: model direct snapshot access, changed-block discovery, put/list/get block operations, checksum validation, completion state, and error paths around in-progress or unavailable snapshots.

## Service Identity and Protocol

- AWS model slug: `ebs`
- AWS SDK for Rust slug: `ebs`
- Model version: `2019-11-02`
- Model file: `vendor/api-models-aws/models/ebs/service/2019-11-02/ebs-2019-11-02.json`
- SDK ID: `EBS`
- Endpoint prefix: `ebs`
- ARN namespace: `ebs`
- CloudFormation name: `EBS`
- CloudTrail event source: `ebs.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Complete` (1), `Get` (1), `Put` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutSnapshotBlock`, `StartSnapshot`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetSnapshotBlock`, `ListChangedBlocks`, `ListSnapshotBlocks`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartSnapshot`.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/ebs/latest/userguide/ebs-accessing-snapshot.html
- https://docs.aws.amazon.com/ebs/latest/userguide/ebs-fast-snapshot-restore.html
- https://docs.aws.amazon.com/ebs/latest/userguide/ebsapis-using-encryption.html

Research outcomes:
- EBS direct APIs access snapshot block data and changed blocks for backup and disaster recovery workflows.
- Fast snapshot restore is enabled for a snapshot and Availability Zone pair, and billing begins immediately when enabled.
- Fast snapshot restore uses credits to support full-performance volume creation from snapshots.
- EBS direct API encryption outcomes depend on `Encrypted`, `KmsKeyArn`, `ParentSnapshotId`, and account encryption defaults.
- Snapshots can be incremental and can be used to create volumes or AMIs.
- Snapshot and volume behaviour is Region and Availability Zone sensitive.

Parity implications:
- Model snapshots, blocks, changed-block tokens, volumes, encryption state, KMS keys, fast snapshot restore state per AZ, and credit state separately.
- StartSnapshot/PutSnapshotBlock/CompleteSnapshot should preserve direct-API snapshot lifecycle.
- Volume creation from snapshots should consider snapshot/AZ fast-restore state.

## Operation Groups

### List

- Operations: `ListChangedBlocks`, `ListSnapshotBlocks`
- Traits: `paginated` (2)
- Common required input members in this group: `SecondSnapshotId`, `SnapshotId`

### Complete

- Operations: `CompleteSnapshot`
- Common required input members in this group: `ChangedBlocksCount`, `SnapshotId`

### Get

- Operations: `GetSnapshotBlock`
- Common required input members in this group: `BlockIndex`, `BlockToken`, `SnapshotId`

### Put

- Operations: `PutSnapshotBlock`
- Common required input members in this group: `BlockData`, `BlockIndex`, `Checksum`, `ChecksumAlgorithm`, `DataLength`, `SnapshotId`

### Start

- Operations: `StartSnapshot`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `VolumeSize`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CompleteSnapshot` | `POST /snapshots/completion/{SnapshotId}` | - | `ChangedBlocksCount`, `SnapshotId` | - | `CompleteSnapshotResponse` | `AccessDeniedException`, `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Seals and completes the snapshot after all of the required blocks of data have been written to it. Completing the snapshot changes the status to `completed`. |
| `GetSnapshotBlock` | `GET /snapshots/{SnapshotId}/blocks/{BlockIndex}` | - | `BlockIndex`, `BlockToken`, `SnapshotId` | - | `GetSnapshotBlockResponse` | `AccessDeniedException`, `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns the data in a block in an Amazon Elastic Block Store snapshot. You should always retry requests that receive server (`5xx`) error responses, and `ThrottlingException` and `RequestThrottledException` client error responses. |
| `ListChangedBlocks` | `GET /snapshots/{SecondSnapshotId}/changedblocks` | `paginated` | `SecondSnapshotId` | - | `ListChangedBlocksResponse` | `AccessDeniedException`, `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns information about the blocks that are different between two Amazon Elastic Block Store snapshots of the same volume/snapshot lineage. You should always retry requests that receive server (`5xx`) error responses, and `ThrottlingException` and... |
| `ListSnapshotBlocks` | `GET /snapshots/{SnapshotId}/blocks` | `paginated` | `SnapshotId` | - | `ListSnapshotBlocksResponse` | `AccessDeniedException`, `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Returns information about the blocks in an Amazon Elastic Block Store snapshot. You should always retry requests that receive server (`5xx`) error responses, and `ThrottlingException` and `RequestThrottledException` client error responses. |
| `PutSnapshotBlock` | `PUT /snapshots/{SnapshotId}/blocks/{BlockIndex}` | - | `BlockData`, `BlockIndex`, `Checksum`, `ChecksumAlgorithm`, `DataLength`, `SnapshotId` | - | `PutSnapshotBlockResponse` | `AccessDeniedException`, `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Writes a block of data to a snapshot. If the specified block contains data, the existing data is overwritten. |
| `StartSnapshot` | `POST /snapshots` | `idempotency-token` | `VolumeSize` | `ClientToken` | `StartSnapshotResponse` | `AccessDeniedException`, `ConcurrentLimitExceededException`, `ConflictException`, `InternalServerException`, `RequestThrottledException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new Amazon EBS snapshot. The new snapshot enters the `pending` state after the request completes. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message`, `Reason` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | An internal error has occurred. |
| `RequestThrottledException` | `structure` | `Message`, `Reason` | The number of API requests has exceeded the maximum allowed API request throttling limit for the snapshot. |
| `ResourceNotFoundException` | `structure` | `Message`, `Reason` | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `Reason` | Your current service quotas do not allow you to perform this action. |
| `ValidationException` | `structure` | `Message`, `Reason` | The input fails to satisfy the constraints of the EBS direct APIs. |
| `CompleteSnapshotRequest` | `structure` | `ChangedBlocksCount`, `Checksum`, `ChecksumAggregationMethod`, `ChecksumAlgorithm`, `SnapshotId` | - |
| `CompleteSnapshotResponse` | `structure` | `Status` | - |
| `GetSnapshotBlockRequest` | `structure` | `BlockIndex`, `BlockToken`, `SnapshotId` | - |
| `GetSnapshotBlockResponse` | `structure` | `BlockData`, `Checksum`, `ChecksumAlgorithm`, `DataLength` | - |
| `ListChangedBlocksRequest` | `structure` | `FirstSnapshotId`, `MaxResults`, `NextToken`, `SecondSnapshotId`, `StartingBlockIndex` | - |
| `ListChangedBlocksResponse` | `structure` | `BlockSize`, `ChangedBlocks`, `ExpiryTime`, `NextToken`, `VolumeSize` | - |
| `ListSnapshotBlocksRequest` | `structure` | `MaxResults`, `NextToken`, `SnapshotId`, `StartingBlockIndex` | - |
| `ListSnapshotBlocksResponse` | `structure` | `BlockSize`, `Blocks`, `ExpiryTime`, `NextToken`, `VolumeSize` | - |
| `PutSnapshotBlockRequest` | `structure` | `BlockData`, `BlockIndex`, `Checksum`, `ChecksumAlgorithm`, `DataLength`, `Progress`, `SnapshotId` | - |
| `PutSnapshotBlockResponse` | `structure` | `Checksum`, `ChecksumAlgorithm` | - |
| `StartSnapshotRequest` | `structure` | `ClientToken`, `Description`, `Encrypted`, `KmsKeyArn`, `ParentSnapshotId`, `Tags`, `Timeout`, `VolumeSize` | - |
| `StartSnapshotResponse` | `structure` | `BlockSize`, `Description`, `KmsKeyArn`, `OwnerId`, `ParentSnapshotId`, `SnapshotId`, `SseType`, `StartTime`, `Status`, `Tags`, `VolumeSize` | - |
| `ConcurrentLimitExceededException` | `structure` | `Message` | You have reached the limit for concurrent API requests. |
| `ConflictException` | `structure` | `Message` | The request uses the same client token as a previous, but non-identical request. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

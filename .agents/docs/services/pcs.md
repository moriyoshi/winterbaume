# AWS Parallel Computing Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Parallel Computing Service (PCS) is a managed service that makes it easier for you to run and scale your high performance computing (HPC) workloads, and build scientific and engineering models on Amazon Web Services using Slurm. For more information, see the Parallel Computing Service User Guide. This reference describes the actions and data types of the service management API. You can use the Amazon Web Services SDKs to call the API actions in software, or use the Command Line Interface (CLI) to call the API actions manually. These API actions manage the service through an Amazon Web Services account.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Parallel Computing Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented AWS Parallel Computing Service workflows in the local mock. Key resources include `ClusterResource`, `ComputeNodeGroupResource`, `QueueResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Update` operation families, including `ListClusters`, `ListComputeNodeGroups`, `ListQueues`, `ListTagsForResource`, `CreateCluster`, `CreateComputeNodeGroup`.

## Service Identity and Protocol

- AWS model slug: `pcs`
- AWS SDK for Rust slug: `pcs`
- Model version: `2023-02-10`
- Model file: `vendor/api-models-aws/models/pcs/service/2023-02-10/pcs-2023-02-10.json`
- SDK ID: `PCS`
- Endpoint prefix: `-`
- ARN namespace: `pcs`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (3), `Delete` (3), `Get` (3), `Update` (3), `Register` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCluster`, `CreateComputeNodeGroup`, `CreateQueue`, `DeleteCluster`, `DeleteComputeNodeGroup`, `DeleteQueue`, `RegisterComputeNodeGroupInstance`, `TagResource`, `UntagResource`, `UpdateCluster`, `UpdateComputeNodeGroup`, `UpdateQueue`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCluster`, `GetComputeNodeGroup`, `GetQueue`, `ListClusters`, `ListComputeNodeGroups`, `ListQueues`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SQS`, `EC2/VPC`, `ECS`, `Secrets Manager`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ClusterResource` | `clusterIdentifier` | create: `CreateCluster`; update: `UpdateCluster` | `ListClusters`, `DeleteCluster`, `GetCluster`, `RegisterComputeNodeGroupInstance` | - |
| `ComputeNodeGroupResource` | `clusterIdentifier`, `computeNodeGroupIdentifier` | create: `CreateComputeNodeGroup`; update: `UpdateComputeNodeGroup` | `ListComputeNodeGroups`, `DeleteComputeNodeGroup`, `GetComputeNodeGroup` | - |
| `QueueResource` | `clusterIdentifier`, `queueIdentifier` | create: `CreateQueue`; update: `UpdateQueue` | `ListQueues`, `DeleteQueue`, `GetQueue` | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Returns a list of all tags on an PCS resource. |
| `TagResource` | `-` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Adds or edits tags on an PCS resource. Each tag consists of a tag key and a tag value. The tag key and tag value are case-sensitive strings. The tag value can be an empty (null) string. To add a tag, specify a new ta ... |
| `UntagResource` | `-` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Deletes tags from an PCS resource. To delete a tag, specify the tag key and the Amazon Resource Name (ARN) of the PCS resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have permission to perform the action. Examples The launch template instance profile doesn't pass iam:PassRole verification. There is a mismatch b ... |
| `ConflictException` | `structure` | message, resourceId, resourceType | Your request has conflicting operations. This can occur if you're trying to perform more than 1 operation on the same resource at the same time. Examples A ... |
| `InternalServerException` | `structure` | message | PCS can't process your request right now. Try again later. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The requested resource can't be found. The cluster, node group, or queue you're attempting to get, update, list, or delete doesn't exist. Examples |
| `ServiceQuotaExceededException` | `structure` | message, serviceCode, resourceId, resourceType, quotaCode | You exceeded your service quota. Service quotas, also referred to as limits, are the maximum number of service resources or operations for your Amazon Web S ... |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | Your request exceeded a request rate quota. Check the resource's request rate quota and try again. |
| `ValidationException` | `structure` | message, reason, fieldList | The request isn't valid. Examples Your request contains malformed JSON or unsupported characters. The scheduler version isn't supported. There are networkin ... |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AccountingMode` | `enum` | STANDARD, NONE | - |
| `ClusterStatus` | `enum` | CREATING, ACTIVE, UPDATING, DELETING, CREATE_FAILED, DELETE_FAILED, UPDATE_FAILED, SUSPENDING, SUSPENDED, RESUMING | - |
| `ComputeNodeGroupStatus` | `enum` | CREATING, ACTIVE, UPDATING, DELETING, CREATE_FAILED, DELETE_FAILED, UPDATE_FAILED, DELETED, SUSPENDING, SUSPENDED, RESUMING | - |
| `EndpointType` | `enum` | SLURMCTLD, SLURMDBD, SLURMRESTD | - |
| `NetworkType` | `enum` | IPV4, IPV6 | - |
| `PurchaseOption` | `enum` | ONDEMAND, SPOT, CAPACITY_BLOCK | - |
| `QueueStatus` | `enum` | CREATING, ACTIVE, UPDATING, DELETING, CREATE_FAILED, DELETE_FAILED, UPDATE_FAILED, SUSPENDING, SUSPENDED, RESUMING | - |
| `SchedulerType` | `enum` | SLURM | - |
| `Size` | `enum` | SMALL, MEDIUM, LARGE | - |
| `SlurmRestMode` | `enum` | STANDARD, NONE | - |
| `SpotAllocationStrategy` | `enum` | LOWEST_PRICE, CAPACITY_OPTIMIZED, PRICE_CAPACITY_OPTIMIZED | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

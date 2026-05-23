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

- Operations: `ListClusters`, `ListComputeNodeGroups`, `ListQueues`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `clusterIdentifier`, `resourceArn`

### Create

- Operations: `CreateCluster`, `CreateComputeNodeGroup`, `CreateQueue`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `clusterIdentifier`, `clusterName`, `computeNodeGroupName`, `customLaunchTemplate`, `iamInstanceProfileArn`, `instanceConfigs`, `networking`, `queueName`, `scalingConfiguration`, `scheduler`, `size`, `subnetIds`

### Delete

- Operations: `DeleteCluster`, `DeleteComputeNodeGroup`, `DeleteQueue`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `clusterIdentifier`, `computeNodeGroupIdentifier`, `queueIdentifier`

### Get

- Operations: `GetCluster`, `GetComputeNodeGroup`, `GetQueue`
- Traits: `readonly` (3)
- Common required input members in this group: `clusterIdentifier`, `computeNodeGroupIdentifier`, `queueIdentifier`

### Update

- Operations: `UpdateCluster`, `UpdateComputeNodeGroup`, `UpdateQueue`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `clusterIdentifier`, `computeNodeGroupIdentifier`, `queueIdentifier`

### Register

- Operations: `RegisterComputeNodeGroupInstance`
- Common required input members in this group: `bootstrapId`, `clusterIdentifier`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCluster` | - | `idempotent`, `idempotency-token` | `clusterName`, `networking`, `scheduler`, `size` | `clientToken` | `CreateClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a cluster in your account. PCS creates the cluster controller in a service-owned account. |
| `CreateComputeNodeGroup` | - | `idempotent`, `idempotency-token` | `clusterIdentifier`, `computeNodeGroupName`, `customLaunchTemplate`, `iamInstanceProfileArn`, `instanceConfigs`, `scalingConfiguration`, `subnetIds` | `clientToken` | `CreateComputeNodeGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a managed set of compute nodes. You associate a compute node group with a cluster through 1 or more PCS queues or as part of the login fleet. |
| `CreateQueue` | - | `idempotent`, `idempotency-token` | `clusterIdentifier`, `queueName` | `clientToken` | `CreateQueueResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a job queue. You must associate 1 or more compute node groups with the queue. |
| `DeleteCluster` | - | `idempotent`, `idempotency-token` | `clusterIdentifier` | `clientToken` | `DeleteClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a cluster and all its linked resources. You must delete all queues and compute node groups associated with the cluster before you can delete the cluster. |
| `DeleteComputeNodeGroup` | - | `idempotent`, `idempotency-token` | `clusterIdentifier`, `computeNodeGroupIdentifier` | `clientToken` | `DeleteComputeNodeGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a compute node group. You must delete all queues associated with the compute node group first. |
| `DeleteQueue` | - | `idempotent`, `idempotency-token` | `clusterIdentifier`, `queueIdentifier` | `clientToken` | `DeleteQueueResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a job queue. If the compute node group associated with this queue isn't associated with any other queues, PCS terminates all the compute nodes for this queue. |
| `GetCluster` | - | `readonly` | `clusterIdentifier` | - | `GetClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns detailed information about a running cluster in your account. This API action provides networking information, endpoint information for communication with the scheduler, and provisioning status. |
| `GetComputeNodeGroup` | - | `readonly` | `clusterIdentifier`, `computeNodeGroupIdentifier` | - | `GetComputeNodeGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns detailed information about a compute node group. This API action provides networking information, EC2 instance type, compute node group status, and scheduler (such as Slurm) configuration. |
| `GetQueue` | - | `readonly` | `clusterIdentifier`, `queueIdentifier` | - | `GetQueueResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns detailed information about a queue. The information includes the compute node groups that the queue uses to schedule jobs. |
| `ListClusters` | - | `readonly`, `paginated` | - | - | `ListClustersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of running clusters in your account. |
| `ListComputeNodeGroups` | - | `readonly`, `paginated` | `clusterIdentifier` | - | `ListComputeNodeGroupsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all compute node groups associated with a cluster. |
| `ListQueues` | - | `readonly`, `paginated` | `clusterIdentifier` | - | `ListQueuesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all queues associated with a cluster. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Returns a list of all tags on an PCS resource. |
| `RegisterComputeNodeGroupInstance` | - | - | `bootstrapId`, `clusterIdentifier` | - | `RegisterComputeNodeGroupInstanceResponse` | `AccessDeniedException`, `InternalServerException` | This API action isn't intended for you to use. PCS uses this API action to register the compute nodes it launches in your account. |
| `TagResource` | - | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Adds or edits tags on an PCS resource. Each tag consists of a tag key and a tag value. |
| `UntagResource` | - | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Deletes tags from an PCS resource. To delete a tag, specify the tag key and the Amazon Resource Name (ARN) of the PCS resource. |
| `UpdateCluster` | - | `idempotent`, `idempotency-token` | `clusterIdentifier` | `clientToken` | `UpdateClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a cluster configuration. You can modify Slurm scheduler settings, accounting configuration, and security groups for an existing cluster. |
| `UpdateComputeNodeGroup` | - | `idempotent`, `idempotency-token` | `clusterIdentifier`, `computeNodeGroupIdentifier` | `clientToken` | `UpdateComputeNodeGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a compute node group. You can update many of the fields related to your compute node group including the configurations for networking, compute nodes, and settings specific to your scheduler (such as Slurm). |
| `UpdateQueue` | - | `idempotent`, `idempotency-token` | `clusterIdentifier`, `queueIdentifier` | `clientToken` | `UpdateQueueResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the compute node group configuration of a queue. Use this API to change the compute node groups that the queue can send jobs to. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The requested resource can't be found. |
| `AccessDeniedException` | `structure` | `message` | You don't have permission to perform the action. |
| `InternalServerException` | `structure` | `message` | PCS can't process your request right now. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | Your request has conflicting operations. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | Your request exceeded a request rate quota. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The request isn't valid. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | You exceeded your service quota. |
| `CreateClusterRequest` | `structure` | `clientToken`, `clusterName`, `networking`, `scheduler`, `size`, `slurmConfiguration`, `tags` | - |
| `CreateClusterResponse` | `structure` | `cluster` | - |
| `CreateComputeNodeGroupRequest` | `structure` | `amiId`, `clientToken`, `clusterIdentifier`, `computeNodeGroupName`, `customLaunchTemplate`, `iamInstanceProfileArn`, `instanceConfigs`, `purchaseOption`, `scalingConfiguration`, `slurmConfiguration`, `spotOptions`, `subnetIds`, ... (+1) | - |
| `CreateComputeNodeGroupResponse` | `structure` | `computeNodeGroup` | - |
| `CreateQueueRequest` | `structure` | `clientToken`, `clusterIdentifier`, `computeNodeGroupConfigurations`, `queueName`, `slurmConfiguration`, `tags` | - |
| `CreateQueueResponse` | `structure` | `queue` | - |
| `DeleteClusterRequest` | `structure` | `clientToken`, `clusterIdentifier` | - |
| `DeleteClusterResponse` | `structure` | - | - |
| `DeleteComputeNodeGroupRequest` | `structure` | `clientToken`, `clusterIdentifier`, `computeNodeGroupIdentifier` | - |
| `DeleteComputeNodeGroupResponse` | `structure` | - | - |
| `DeleteQueueRequest` | `structure` | `clientToken`, `clusterIdentifier`, `queueIdentifier` | - |
| `DeleteQueueResponse` | `structure` | - | - |
| `GetClusterRequest` | `structure` | `clusterIdentifier` | - |
| `GetClusterResponse` | `structure` | `cluster` | - |
| `GetComputeNodeGroupRequest` | `structure` | `clusterIdentifier`, `computeNodeGroupIdentifier` | - |
| `GetComputeNodeGroupResponse` | `structure` | `computeNodeGroup` | - |
| `GetQueueRequest` | `structure` | `clusterIdentifier`, `queueIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

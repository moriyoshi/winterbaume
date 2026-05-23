# AWS CloudHSM V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

For more information about CloudHSM, see CloudHSM and the CloudHSM User Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS CloudHSM V2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Create`, `Describe`, `Modify`, `Copy` operation families, including `DeleteBackup`, `DeleteCluster`, `DeleteHsm`, `DeleteResourcePolicy`, `CreateCluster`, `CreateHsm`.

## Service Identity and Protocol

- AWS model slug: `cloudhsm-v2`
- AWS SDK for Rust slug: `cloudhsmv2`
- Model version: `2017-04-28`
- Model file: `vendor/api-models-aws/models/cloudhsm-v2/service/2017-04-28/cloudhsm-v2-2017-04-28.json`
- SDK ID: `CloudHSM V2`
- Endpoint prefix: `cloudhsmv2`
- ARN namespace: `cloudhsm`
- CloudFormation name: `CloudHSMV2`
- CloudTrail event source: `cloudhsmv2.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (4), `Create` (2), `Describe` (2), `Modify` (2), `Copy` (1), `Get` (1), `Initialize` (1), `List` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCluster`, `CreateHsm`, `DeleteBackup`, `DeleteCluster`, `DeleteHsm`, `DeleteResourcePolicy`, `ModifyBackupAttributes`, `ModifyCluster`, `PutResourcePolicy`, `RestoreBackup`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBackups`, `DescribeClusters`, `GetResourcePolicy`, `ListTags`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 18 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`, `ECS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html
- https://docs.aws.amazon.com/cloudhsm/latest/userguide/hsm-users.html
- https://docs.aws.amazon.com/cloudhsm/latest/userguide/bp-cluster-management.html

Research outcomes:
- A CloudHSM cluster is a group of HSMs that synchronise key material and users across Availability Zones.
- Clusters should be architected across Availability Zones for high availability and redundancy.
- AWS recommends at least three HSMs for durability of newly generated keys.
- HSM users are separate from IAM users. They authenticate directly to the cluster hardware and manage permissions through CloudHSM tools.
- CloudHSM supports quorum authentication, where M of N HSM users must approve selected operations.
- CloudHSM clusters can be scaled by adding or removing HSMs according to traffic and availability requirements.
- Cluster access should be secured with private subnet placement and controlled network access.
- Backups and cluster lifecycle operations are distinct from HSM user and key-management operations.

Parity implications:
- Model clusters, HSM instances, backups, subnet/security group placement, cluster state, HSM users, key state, quorum settings, and client configuration separately.
- Cluster operations should propagate state to member HSMs and expose asynchronous lifecycle transitions.
- IAM authorisation and HSM user authentication should be treated as separate layers.

## Current Network Resource Stub Semantics

CloudHSM V2 currently synthesises cluster network state from request metadata.

- `CreateCluster` records the supplied subnet IDs but mints a synthetic VPC ID of the form `vpc-<uuid-prefix>` instead of deriving it from EC2 subnet state.
- Cluster filters can match the stored synthetic VPC ID, and later describe calls return that local value.
- HSM and cluster lifecycle is not tied to subnet reachability, security groups, route tables, or ENIs.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Delete

- Operations: `DeleteBackup`, `DeleteCluster`, `DeleteHsm`, `DeleteResourcePolicy`
- Common required input members in this group: `ClusterId`

### Create

- Operations: `CreateCluster`, `CreateHsm`
- Common required input members in this group: -

### Describe

- Operations: `DescribeBackups`, `DescribeClusters`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Modify

- Operations: `ModifyBackupAttributes`, `ModifyCluster`
- Common required input members in this group: -

### Copy

- Operations: `CopyBackupToRegion`
- Common required input members in this group: -

### Get

- Operations: `GetResourcePolicy`
- Common required input members in this group: -

### Initialize

- Operations: `InitializeCluster`
- Common required input members in this group: -

### List

- Operations: `ListTags`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: -

### Restore

- Operations: `RestoreBackup`
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
| `CopyBackupToRegion` | `-` | - | `DestinationRegion`, `BackupId` | - | `CopyBackupToRegionResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Copy an CloudHSM cluster backup to a different region. Cross-account use: No. You cannot perform this operation on an CloudHSM backup in a different Amazon Web Services account. |
| `CreateCluster` | `-` | - | `HsmType`, `SubnetIds` | - | `CreateClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Creates a new CloudHSM cluster. Cross-account use: Yes. To perform this operation with an CloudHSM backup in a different AWS account, specify the full backup ARN in the value of the SourceBackupId parameter. |
| `CreateHsm` | `-` | - | `ClusterId`, `AvailabilityZone` | - | `CreateHsmResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Creates a new hardware security module (HSM) in the specified CloudHSM cluster. Cross-account use: No. You cannot perform this operation on an CloudHSM cluster in a different Amazon Web Service account. |
| `DeleteBackup` | `-` | - | `BackupId` | - | `DeleteBackupResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Deletes a specified CloudHSM backup. A backup can be restored up to 7 days after the DeleteBackup request is made. For more information on restoring a backup, see RestoreBackup . Cross-account use: No. You cannot per ... |
| `DeleteCluster` | `-` | - | `ClusterId` | - | `DeleteClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Deletes the specified CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use DescribeClusters . To delete an HSM, use DeleteHsm . Cros ... |
| `DeleteHsm` | `-` | - | `ClusterId` | - | `DeleteHsmResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To ... |
| `DeleteResourcePolicy` | `-` | - | - | - | `DeleteResourcePolicyResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Deletes an CloudHSM resource policy. Deleting a resource policy will result in the resource being unshared and removed from any RAM resource shares. Deleting the resource policy attached to a backup will not impact a ... |
| `DescribeBackups` | `-` | `paginated` | - | - | `DescribeBackupsResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Gets information about backups of CloudHSM clusters. Lists either the backups you own or the backups shared with you when the Shared parameter is true. This is a paginated operation, which means that each response mi ... |
| `DescribeClusters` | `-` | `paginated` | - | - | `DescribeClustersResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmServiceException`, `CloudHsmTagException` | Gets information about CloudHSM clusters. This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includ ... |
| `GetResourcePolicy` | `-` | - | - | - | `GetResourcePolicyResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Retrieves the resource policy document attached to a given resource. Cross-account use: No. You cannot perform this operation on an CloudHSM resource in a different Amazon Web Services account. |
| `InitializeCluster` | `-` | - | `ClusterId`, `SignedCert`, `TrustAnchor` | - | `InitializeClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Claims an CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificat ... |
| `ListTags` | `-` | `paginated` | `ResourceId` | - | `ListTagsResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Gets a list of tags for the specified CloudHSM cluster. This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it ... |
| `ModifyBackupAttributes` | `-` | - | `BackupId`, `NeverExpires` | - | `ModifyBackupAttributesResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Modifies attributes for CloudHSM backup. Cross-account use: No. You cannot perform this operation on an CloudHSM backup in a different Amazon Web Services account. |
| `ModifyCluster` | `-` | - | `ClusterId` | - | `ModifyClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Modifies CloudHSM cluster. Cross-account use: No. You cannot perform this operation on an CloudHSM cluster in a different Amazon Web Services account. |
| `PutResourcePolicy` | `-` | - | - | - | `PutResourcePolicyResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Creates or updates an CloudHSM resource policy. A resource policy helps you to define the IAM entity (for example, an Amazon Web Services account) that can manage your CloudHSM resources. The following resources supp ... |
| `RestoreBackup` | `-` | - | `BackupId` | - | `RestoreBackupResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Restores a specified CloudHSM backup that is in the PENDING_DELETION state. For more information on deleting a backup, see DeleteBackup . Cross-account use: No. You cannot perform this operation on an CloudHSM backup ... |
| `TagResource` | `-` | - | `ResourceId`, `TagList` | - | `TagResourceResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceLimitExceededException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Adds or overwrites one or more tags for the specified CloudHSM cluster. Cross-account use: No. You cannot perform this operation on an CloudHSM resource in a different Amazon Web Services account. |
| `UntagResource` | `-` | - | `ResourceId`, `TagKeyList` | - | `UntagResourceResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Removes the specified tag or tags from the specified CloudHSM cluster. Cross-account use: No. You cannot perform this operation on an CloudHSM resource in a different Amazon Web Services account. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CloudHsmAccessDeniedException` | `structure` | Message | The request was rejected because the requester does not have permission to perform the requested operation. |
| `CloudHsmInternalFailureException` | `structure` | Message | The request was rejected because of an CloudHSM internal failure. The request can be retried. |
| `CloudHsmInvalidRequestException` | `structure` | Message | The request was rejected because it is not a valid request. |
| `CloudHsmResourceLimitExceededException` | `structure` | Message | The request was rejected because it exceeds an CloudHSM limit. |
| `CloudHsmResourceNotFoundException` | `structure` | Message | The request was rejected because it refers to a resource that cannot be found. |
| `CloudHsmServiceException` | `structure` | Message | The request was rejected because an error occurred. |
| `CloudHsmTagException` | `structure` | Message | The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request. |
| `CopyBackupToRegionRequest` | `structure` | DestinationRegion, BackupId, TagList | - |
| `CopyBackupToRegionResponse` | `structure` | DestinationBackup | - |
| `CreateClusterRequest` | `structure` | BackupRetentionPolicy, HsmType, SourceBackupId, SubnetIds, NetworkType, TagList, Mode | - |
| `CreateClusterResponse` | `structure` | Cluster | - |
| `CreateHsmRequest` | `structure` | ClusterId, AvailabilityZone, IpAddress | - |
| `CreateHsmResponse` | `structure` | Hsm | - |
| `DeleteBackupRequest` | `structure` | BackupId | - |
| `DeleteBackupResponse` | `structure` | Backup | - |
| `DeleteClusterRequest` | `structure` | ClusterId | - |
| `DeleteClusterResponse` | `structure` | Cluster | - |
| `DeleteHsmRequest` | `structure` | ClusterId, HsmId, EniId, EniIp | - |
| `DeleteHsmResponse` | `structure` | HsmId | - |
| `DeleteResourcePolicyRequest` | `structure` | ResourceArn | - |
| `DeleteResourcePolicyResponse` | `structure` | ResourceArn, Policy | - |
| `DescribeBackupsRequest` | `structure` | NextToken, MaxResults, Filters, Shared, SortAscending | - |
| `DescribeBackupsResponse` | `structure` | Backups, NextToken | - |
| `DescribeClustersRequest` | `structure` | Filters, NextToken, MaxResults | - |
| `DescribeClustersResponse` | `structure` | Clusters, NextToken | - |
| `GetResourcePolicyRequest` | `structure` | ResourceArn | - |
| `GetResourcePolicyResponse` | `structure` | Policy | - |
| `InitializeClusterRequest` | `structure` | ClusterId, SignedCert, TrustAnchor | - |
| `InitializeClusterResponse` | `structure` | State, StateMessage | - |
| `ListTagsRequest` | `structure` | ResourceId, NextToken, MaxResults | - |
| `ListTagsResponse` | `structure` | TagList, NextToken | - |
| `ModifyBackupAttributesRequest` | `structure` | BackupId, NeverExpires | - |
| `ModifyBackupAttributesResponse` | `structure` | Backup | - |
| `ModifyClusterRequest` | `structure` | HsmType, BackupRetentionPolicy, ClusterId | - |
| `ModifyClusterResponse` | `structure` | Cluster | - |
| `PutResourcePolicyRequest` | `structure` | ResourceArn, Policy | - |
| `PutResourcePolicyResponse` | `structure` | ResourceArn, Policy | - |
| `RestoreBackupRequest` | `structure` | BackupId | - |
| `RestoreBackupResponse` | `structure` | Backup | - |
| `TagResourceRequest` | `structure` | ResourceId, TagList | - |
| `BackupPolicy` | `enum` | DEFAULT | - |
| `BackupRetentionType` | `enum` | DAYS | - |
| `BackupState` | `enum` | CREATE_IN_PROGRESS, READY, DELETED, PENDING_DELETION | - |
| `ClusterMode` | `enum` | FIPS, NON_FIPS | - |
| `ClusterState` | `enum` | CREATE_IN_PROGRESS, UNINITIALIZED, INITIALIZE_IN_PROGRESS, INITIALIZED, ACTIVE, UPDATE_IN_PROGRESS, MODIFY_IN_PROGRESS, ROLLBACK_IN_PROGRESS, DELETE_IN_PROGRESS, DELETED, DEGRADED | - |
| `HsmState` | `enum` | CREATE_IN_PROGRESS, ACTIVE, DEGRADED, DELETE_IN_PROGRESS, DELETED | - |
| `NetworkType` | `enum` | IPV4, DUALSTACK | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

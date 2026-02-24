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
- Common required input members in this group: `BackupId`, `ClusterId`

### Create

- Operations: `CreateCluster`, `CreateHsm`
- Common required input members in this group: `AvailabilityZone`, `ClusterId`, `HsmType`, `SubnetIds`

### Describe

- Operations: `DescribeBackups`, `DescribeClusters`
- Traits: `paginated` (2)

### Modify

- Operations: `ModifyBackupAttributes`, `ModifyCluster`
- Common required input members in this group: `BackupId`, `ClusterId`, `NeverExpires`

### Copy

- Operations: `CopyBackupToRegion`
- Common required input members in this group: `BackupId`, `DestinationRegion`

### Get

- Operations: `GetResourcePolicy`

### Initialize

- Operations: `InitializeCluster`
- Common required input members in this group: `ClusterId`, `SignedCert`, `TrustAnchor`

### List

- Operations: `ListTags`
- Traits: `paginated` (1)
- Common required input members in this group: `ResourceId`

### Put

- Operations: `PutResourcePolicy`

### Restore

- Operations: `RestoreBackup`
- Common required input members in this group: `BackupId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceId`, `TagList`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceId`, `TagKeyList`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CopyBackupToRegion` | - | - | `BackupId`, `DestinationRegion` | - | `CopyBackupToRegionResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Copy an CloudHSM cluster backup to a different region. Cross-account use: No. |
| `CreateCluster` | - | - | `HsmType`, `SubnetIds` | - | `CreateClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Creates a new CloudHSM cluster. Cross-account use: Yes. |
| `CreateHsm` | - | - | `AvailabilityZone`, `ClusterId` | - | `CreateHsmResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Creates a new hardware security module (HSM) in the specified CloudHSM cluster. Cross-account use: No. |
| `DeleteBackup` | - | - | `BackupId` | - | `DeleteBackupResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Deletes a specified CloudHSM backup. A backup can be restored up to 7 days after the DeleteBackup request is made. |
| `DeleteCluster` | - | - | `ClusterId` | - | `DeleteClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Deletes the specified CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. |
| `DeleteHsm` | - | - | `ClusterId` | - | `DeleteHsmResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. |
| `DeleteResourcePolicy` | - | - | - | - | `DeleteResourcePolicyResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Deletes an CloudHSM resource policy. Deleting a resource policy will result in the resource being unshared and removed from any RAM resource shares. |
| `DescribeBackups` | - | `paginated` | - | - | `DescribeBackupsResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Gets information about backups of CloudHSM clusters. Lists either the backups you own or the backups shared with you when the Shared parameter is true. |
| `DescribeClusters` | - | `paginated` | - | - | `DescribeClustersResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmServiceException`, `CloudHsmTagException` | Gets information about CloudHSM clusters. This is a paginated operation, which means that each response might contain only a subset of all the clusters. |
| `GetResourcePolicy` | - | - | - | - | `GetResourcePolicyResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Retrieves the resource policy document attached to a given resource. Cross-account use: No. |
| `InitializeCluster` | - | - | `ClusterId`, `SignedCert`, `TrustAnchor` | - | `InitializeClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Claims an CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing... |
| `ListTags` | - | `paginated` | `ResourceId` | - | `ListTagsResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Gets a list of tags for the specified CloudHSM cluster. This is a paginated operation, which means that each response might contain only a subset of all the tags. |
| `ModifyBackupAttributes` | - | - | `BackupId`, `NeverExpires` | - | `ModifyBackupAttributesResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Modifies attributes for CloudHSM backup. Cross-account use: No. |
| `ModifyCluster` | - | - | `ClusterId` | - | `ModifyClusterResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Modifies CloudHSM cluster. Cross-account use: No. |
| `PutResourcePolicy` | - | - | - | - | `PutResourcePolicyResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Creates or updates an CloudHSM resource policy. A resource policy helps you to define the IAM entity (for example, an Amazon Web Services account) that can manage your CloudHSM resources. |
| `RestoreBackup` | - | - | `BackupId` | - | `RestoreBackupResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException` | Restores a specified CloudHSM backup that is in the `PENDING_DELETION` state. For more information on deleting a backup, see DeleteBackup. |
| `TagResource` | - | - | `ResourceId`, `TagList` | - | `TagResourceResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceLimitExceededException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Adds or overwrites one or more tags for the specified CloudHSM cluster. Cross-account use: No. |
| `UntagResource` | - | - | `ResourceId`, `TagKeyList` | - | `UntagResourceResponse` | `CloudHsmAccessDeniedException`, `CloudHsmInternalFailureException`, `CloudHsmInvalidRequestException`, `CloudHsmResourceNotFoundException`, `CloudHsmServiceException`, `CloudHsmTagException` | Removes the specified tag or tags from the specified CloudHSM cluster. Cross-account use: No. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CloudHsmAccessDeniedException` | `structure` | `Message` | The request was rejected because the requester does not have permission to perform the requested operation. |
| `CloudHsmInternalFailureException` | `structure` | `Message` | The request was rejected because of an CloudHSM internal failure. |
| `CloudHsmInvalidRequestException` | `structure` | `Message` | The request was rejected because it is not a valid request. |
| `CloudHsmServiceException` | `structure` | `Message` | The request was rejected because an error occurred. |
| `CloudHsmResourceNotFoundException` | `structure` | `Message` | The request was rejected because it refers to a resource that cannot be found. |
| `CloudHsmTagException` | `structure` | `Message` | The request was rejected because of a tagging failure. |
| `CopyBackupToRegionRequest` | `structure` | `BackupId`, `DestinationRegion`, `TagList` | - |
| `CopyBackupToRegionResponse` | `structure` | `DestinationBackup` | - |
| `CreateClusterRequest` | `structure` | `BackupRetentionPolicy`, `HsmType`, `Mode`, `NetworkType`, `SourceBackupId`, `SubnetIds`, `TagList` | - |
| `CreateClusterResponse` | `structure` | `Cluster` | - |
| `CreateHsmRequest` | `structure` | `AvailabilityZone`, `ClusterId`, `IpAddress` | - |
| `CreateHsmResponse` | `structure` | `Hsm` | - |
| `DeleteBackupRequest` | `structure` | `BackupId` | - |
| `DeleteBackupResponse` | `structure` | `Backup` | - |
| `DeleteClusterRequest` | `structure` | `ClusterId` | - |
| `DeleteClusterResponse` | `structure` | `Cluster` | - |
| `DeleteHsmRequest` | `structure` | `ClusterId`, `EniId`, `EniIp`, `HsmId` | - |
| `DeleteHsmResponse` | `structure` | `HsmId` | - |
| `DeleteResourcePolicyRequest` | `structure` | `ResourceArn` | - |
| `DeleteResourcePolicyResponse` | `structure` | `Policy`, `ResourceArn` | - |
| `DescribeBackupsRequest` | `structure` | `Filters`, `MaxResults`, `NextToken`, `Shared`, `SortAscending` | - |
| `DescribeBackupsResponse` | `structure` | `Backups`, `NextToken` | - |
| `DescribeClustersRequest` | `structure` | `Filters`, `MaxResults`, `NextToken` | - |
| `DescribeClustersResponse` | `structure` | `Clusters`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

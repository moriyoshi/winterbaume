# Amazon Aurora DSQL

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is an interface reference for Amazon Aurora DSQL. It contains documentation for one of the programming or command line interfaces you can use to manage Amazon Aurora DSQL. Amazon Aurora DSQL is a serverless, distributed SQL database suitable for workloads of any size. is available in both single-Region and multi-Region configurations, so your clusters and databases are always available even if an Availability Zone or an Amazon Web Services Region are unavailable. lets you focus on using your data to acquire new insights for your business and customers.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Aurora DSQL workflows in the local mock. Key resources include `Cluster`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Delete`, `List`, `Create`, `Put` operation families, including `GetCluster`, `GetClusterPolicy`, `GetVpcEndpointServiceName`, `DeleteCluster`, `DeleteClusterPolicy`, `ListClusters`.

## Service Identity and Protocol

- AWS model slug: `dsql`
- AWS SDK for Rust slug: `dsql`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/dsql/service/2018-05-10/dsql-2018-05-10.json`
- SDK ID: `DSQL`
- Endpoint prefix: `dsql`
- ARN namespace: `dsql`
- CloudFormation name: `-`
- CloudTrail event source: `dsql.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `Delete` (2), `List` (2), `Create` (1), `Put` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCluster`, `DeleteCluster`, `DeleteClusterPolicy`, `PutClusterPolicy`, `TagResource`, `UntagResource`, `UpdateCluster`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCluster`, `GetClusterPolicy`, `GetVpcEndpointServiceName`, `ListClusters`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Cluster` | `identifier` | create: `CreateCluster`; read: `GetCluster`; update: `UpdateCluster`; delete: `DeleteCluster`; list: `ListClusters` | `DeleteClusterPolicy`, `GetClusterPolicy`, `GetVpcEndpointServiceName`, `PutClusterPolicy` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/aurora-dsql/latest/userguide/what-is-aurora-dsql.html
- https://docs.aws.amazon.com/aurora-dsql/latest/userguide/configuring-single-region-clusters.html
- https://docs.aws.amazon.com/aurora-dsql/latest/userguide/disaster-recovery-resiliency.html

Research outcomes:
- Aurora DSQL is a serverless distributed SQL database with PostgreSQL compatibility and ACID transactions.
- Clusters can be single-Region or active-active multi-Region with linked cluster configuration and witness Region properties.
- Aurora DSQL automatically scales and provides high availability with synchronous replication for multi-Region configurations.
- Backup and restore, replication, fault injection, and recovery are documented resilience concepts.
- Cluster management APIs are separate from SQL data-plane access.

Parity implications:
- Model clusters, single-Region/multi-Region properties, linked clusters, witness Region, endpoints, backup/restore state, tags, and resource policies separately.
- Multi-Region cluster state should preserve linked-cluster relationships and replication status.
- SQL transaction semantics should be considered data-plane behaviour, distinct from cluster control-plane APIs.

## Current Network Resource Stub Semantics

DSQL currently exposes a VPC endpoint service-name API shape but does not implement the network path.

- `GetVpcEndpointServiceName` is routed as not implemented and does not return a service name derived from cluster state.
- Cluster records do not allocate or track VPC endpoints, subnets, security groups, or PrivateLink attachments.
- The generated model contains `clusterVpcEndpoint`, but current handlers do not populate it from EC2 or DSQL state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetCluster`, `GetClusterPolicy`, `GetVpcEndpointServiceName`
- Traits: `readonly` (3)
- Common required input members in this group: `identifier`

### Delete

- Operations: `DeleteCluster`, `DeleteClusterPolicy`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `identifier`

### List

- Operations: `ListClusters`, `ListTagsForResource`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateCluster`
- Traits: `idempotency-token` (1)

### Put

- Operations: `PutClusterPolicy`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `identifier`, `policy`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateCluster`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `identifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCluster` | `POST /cluster` | `idempotency-token` | - | `clientToken` | `CreateClusterOutput` | `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | The CreateCluster API allows you to create both single-Region clusters and multi-Region clusters. With the addition of the multiRegionProperties parameter, you can create a cluster with witness Region support and establish peer relationships with clusters in... |
| `DeleteCluster` | `DELETE /cluster/{identifier}` | `idempotent`, `idempotency-token` | `identifier` | `clientToken` | `DeleteClusterOutput` | `ConflictException`, `ResourceNotFoundException` | Deletes a cluster in Amazon Aurora DSQL. |
| `DeleteClusterPolicy` | `DELETE /cluster/{identifier}/policy` | `idempotent`, `idempotency-token` | `identifier` | `clientToken` | `DeleteClusterPolicyOutput` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes the resource-based policy attached to a cluster. This removes all access permissions defined by the policy, reverting to default access controls. |
| `GetCluster` | `GET /cluster/{identifier}` | `readonly` | `identifier` | - | `GetClusterOutput` | `ResourceNotFoundException` | Retrieves information about a cluster. |
| `GetClusterPolicy` | `GET /cluster/{identifier}/policy` | `readonly` | `identifier` | - | `GetClusterPolicyOutput` | `ResourceNotFoundException`, `ValidationException` | Retrieves the resource-based policy document attached to a cluster. This policy defines the access permissions and conditions for the cluster. |
| `GetVpcEndpointServiceName` | `GET /clusters/{identifier}/vpc-endpoint-service-name` | `readonly` | `identifier` | - | `GetVpcEndpointServiceNameOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the VPC endpoint service name. |
| `ListClusters` | `GET /cluster` | `readonly`, `paginated` | - | - | `ListClustersOutput` | `ResourceNotFoundException` | Retrieves information about a list of clusters. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException` | Lists all of the tags for a resource. |
| `PutClusterPolicy` | `POST /cluster/{identifier}/policy` | `idempotent`, `idempotency-token` | `identifier`, `policy` | `clientToken` | `PutClusterPolicyOutput` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Attaches a resource-based policy to a cluster. This policy defines access permissions and conditions for the cluster, allowing you to control which principals can perform actions on the cluster. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Tags a resource with a map of key and value pairs. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `ResourceNotFoundException` | Removes a tag from a resource. |
| `UpdateCluster` | `POST /cluster/{identifier}` | `idempotency-token` | `identifier` | `clientToken` | `UpdateClusterOutput` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | The UpdateCluster API allows you to modify both single-Region and multi-Region cluster configurations. With the multiRegionProperties parameter, you can add or modify witness Region support and manage peer relationships with clusters in other Regions. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource could not be found. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input failed to satisfy the constraints specified by an Amazon Web Services service. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The submitted action has conflicts. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The service limit was exceeded. |
| `CreateClusterInput` | `structure` | `bypassPolicyLockoutSafetyCheck`, `clientToken`, `deletionProtectionEnabled`, `kmsEncryptionKey`, `multiRegionProperties`, `policy`, `tags` | - |
| `CreateClusterOutput` | `structure` | `arn`, `creationTime`, `deletionProtectionEnabled`, `encryptionDetails`, `endpoint`, `identifier`, `multiRegionProperties`, `status` | The output of a created cluster. |
| `DeleteClusterInput` | `structure` | `clientToken`, `identifier` | - |
| `DeleteClusterOutput` | `structure` | `arn`, `creationTime`, `identifier`, `status` | The output from a deleted cluster. |
| `DeleteClusterPolicyInput` | `structure` | `clientToken`, `expectedPolicyVersion`, `identifier` | - |
| `DeleteClusterPolicyOutput` | `structure` | `policyVersion` | - |
| `GetClusterInput` | `structure` | `identifier` | - |
| `GetClusterOutput` | `structure` | `arn`, `creationTime`, `deletionProtectionEnabled`, `encryptionDetails`, `endpoint`, `identifier`, `multiRegionProperties`, `status`, `tags` | The output of a cluster. |
| `GetClusterPolicyInput` | `structure` | `identifier` | - |
| `GetClusterPolicyOutput` | `structure` | `policy`, `policyVersion` | - |
| `GetVpcEndpointServiceNameInput` | `structure` | `identifier` | - |
| `GetVpcEndpointServiceNameOutput` | `structure` | `clusterVpcEndpoint`, `serviceName` | - |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied due to request throttling. |
| `ListClustersInput` | `structure` | `maxResults`, `nextToken` | - |
| `ListClustersOutput` | `structure` | `clusters`, `nextToken` | - |
| `ListTagsForResourceInput` | `structure` | `resourceArn` | - |
| `ListTagsForResourceOutput` | `structure` | `tags` | - |
| `PutClusterPolicyInput` | `structure` | `bypassPolicyLockoutSafetyCheck`, `clientToken`, `expectedPolicyVersion`, `identifier`, `policy` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

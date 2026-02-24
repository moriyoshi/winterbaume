# Timestream InfluxDB

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Timestream for InfluxDB is a managed time-series database engine that makes it easy for application developers and DevOps teams to run InfluxDB databases on Amazon Web Services for near real-time time-series applications using open-source APIs. With Amazon Timestream for InfluxDB, it is easy to set up, operate, and scale time-series workloads that can answer queries with single-digit millisecond query response time.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Timestream InfluxDB resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Timestream InfluxDB workflows in the local mock. Key resources include `DbClusterResource`, `DbInstanceResource`, `DbParameterGroupResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Get`, `Delete`, `Reboot` operation families, including `ListDbClusters`, `ListDbInstances`, `ListDbInstancesForCluster`, `ListDbParameterGroups`, `CreateDbCluster`, `CreateDbInstance`.

## Service Identity and Protocol

- AWS model slug: `timestream-influxdb`
- AWS SDK for Rust slug: `timestreaminfluxdb`
- Model version: `2023-01-27`
- Model file: `vendor/api-models-aws/models/timestream-influxdb/service/2023-01-27/timestream-influxdb-2023-01-27.json`
- SDK ID: `Timestream InfluxDB`
- Endpoint prefix: `-`
- ARN namespace: `timestream-influxdb`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Create` (3), `Get` (3), `Delete` (2), `Reboot` (2), `Update` (2), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDbCluster`, `CreateDbInstance`, `CreateDbParameterGroup`, `DeleteDbCluster`, `DeleteDbInstance`, `TagResource`, `UntagResource`, `UpdateDbCluster`, `UpdateDbInstance`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDbCluster`, `GetDbInstance`, `GetDbParameterGroup`, `ListDbClusters`, `ListDbInstances`, `ListDbInstancesForCluster`, `ListDbParameterGroups`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`, `ECS`, `RDS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DbClusterResource` | `dbClusterId` | create: `CreateDbCluster`; read: `GetDbCluster`; update: `UpdateDbCluster`; delete: `DeleteDbCluster`; list: `ListDbClusters` | `ListDbInstancesForCluster`, `RebootDbCluster` | Represents a Timestream InfluxDB cluster |
| `DbInstanceResource` | `dbInstanceIdentifier` | create: `CreateDbInstance`; read: `GetDbInstance`; update: `UpdateDbInstance`; delete: `DeleteDbInstance`; list: `ListDbInstances` | `RebootDbInstance` | Represents a Timestream InfluxDB instance |
| `DbParameterGroupResource` | `dbParameterGroupIdentifier` | create: `CreateDbParameterGroup`; read: `GetDbParameterGroup`; list: `ListDbParameterGroups` | - | Represents a Timestream InfluxDB parameter group |

## Current Network Resource Stub Semantics

Timestream for InfluxDB currently stores VPC subnet and security group lists directly on instances and clusters.

- Instance creation requires VPC subnet IDs and VPC security group IDs and stores both vectors in state.
- Cluster creation and update paths similarly store subnet and security group lists as service-local fields.
- Describe and list responses echo those stored vectors without deriving VPC membership or provisioning network interfaces.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListDbClusters`, `ListDbInstances`, `ListDbInstancesForCluster`, `ListDbParameterGroups`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `dbClusterId`, `resourceArn`

### Create

- Operations: `CreateDbCluster`, `CreateDbInstance`, `CreateDbParameterGroup`
- Traits: `idempotent` (3)
- Common required input members in this group: `allocatedStorage`, `dbInstanceType`, `name`, `password`, `vpcSecurityGroupIds`, `vpcSubnetIds`

### Get

- Operations: `GetDbCluster`, `GetDbInstance`, `GetDbParameterGroup`
- Traits: `readonly` (3)
- Common required input members in this group: `dbClusterId`, `identifier`

### Delete

- Operations: `DeleteDbCluster`, `DeleteDbInstance`
- Traits: `idempotent` (2)
- Common required input members in this group: `dbClusterId`, `identifier`

### Reboot

- Operations: `RebootDbCluster`, `RebootDbInstance`
- Traits: `idempotent` (2)
- Common required input members in this group: `dbClusterId`, `identifier`

### Update

- Operations: `UpdateDbCluster`, `UpdateDbInstance`
- Traits: `idempotent` (2)
- Common required input members in this group: `dbClusterId`, `identifier`

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
| `CreateDbCluster` | - | `idempotent` | `dbInstanceType`, `name`, `vpcSecurityGroupIds`, `vpcSubnetIds` | - | `CreateDbClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream for InfluxDB cluster. |
| `CreateDbInstance` | - | `idempotent` | `allocatedStorage`, `dbInstanceType`, `name`, `password`, `vpcSecurityGroupIds`, `vpcSubnetIds` | - | `CreateDbInstanceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream for InfluxDB DB instance. |
| `CreateDbParameterGroup` | - | `idempotent` | `name` | - | `CreateDbParameterGroupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Timestream for InfluxDB DB parameter group to associate with DB instances. |
| `DeleteDbCluster` | - | `idempotent` | `dbClusterId` | - | `DeleteDbClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Timestream for InfluxDB cluster. |
| `DeleteDbInstance` | - | `idempotent` | `identifier` | - | `DeleteDbInstanceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Timestream for InfluxDB DB instance. |
| `GetDbCluster` | - | `readonly` | `dbClusterId` | - | `GetDbClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a Timestream for InfluxDB cluster. |
| `GetDbInstance` | - | `readonly` | `identifier` | - | `GetDbInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a Timestream for InfluxDB DB instance. |
| `GetDbParameterGroup` | - | `readonly` | `identifier` | - | `GetDbParameterGroupOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a Timestream for InfluxDB DB parameter group. |
| `ListDbClusters` | - | `readonly`, `paginated` | - | - | `ListDbClustersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Timestream for InfluxDB DB clusters. |
| `ListDbInstances` | - | `readonly`, `paginated` | - | - | `ListDbInstancesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Timestream for InfluxDB DB instances. |
| `ListDbInstancesForCluster` | - | `readonly`, `paginated` | `dbClusterId` | - | `ListDbInstancesForClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Timestream for InfluxDB clusters. |
| `ListDbParameterGroups` | - | `readonly`, `paginated` | - | - | `ListDbParameterGroupsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of Timestream for InfluxDB DB parameter groups. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | A list of tags applied to the resource. |
| `RebootDbCluster` | - | `idempotent` | `dbClusterId` | - | `RebootDbClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Reboots a Timestream for InfluxDB cluster. |
| `RebootDbInstance` | - | `idempotent` | `identifier` | - | `RebootDbInstanceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Reboots a Timestream for InfluxDB instance. |
| `TagResource` | - | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Tags are composed of a Key/Value pairs. You can use tags to categorize and track your Timestream for InfluxDB resources. |
| `UntagResource` | - | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `ResourceNotFoundException` | Removes the tag from the specified resource. |
| `UpdateDbCluster` | - | `idempotent` | `dbClusterId` | - | `UpdateDbClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a Timestream for InfluxDB cluster. |
| `UpdateDbInstance` | - | `idempotent` | `identifier` | - | `UpdateDbInstanceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a Timestream for InfluxDB DB instance. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The requested resource was not found or does not exist. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message`, `reason` | The input fails to satisfy the constraints specified by Timestream for InfluxDB. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request conflicts with an existing resource in Timestream for InfluxDB. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request exceeds the service quota. |
| `CreateDbClusterInput` | `structure` | `allocatedStorage`, `bucket`, `dbInstanceType`, `dbParameterGroupIdentifier`, `dbStorageType`, `deploymentType`, `failoverMode`, `logDeliveryConfiguration`, `name`, `networkType`, `organization`, `password`, ... (+6) | - |
| `CreateDbClusterOutput` | `structure` | `dbClusterId`, `dbClusterStatus` | - |
| `CreateDbInstanceInput` | `structure` | `allocatedStorage`, `bucket`, `dbInstanceType`, `dbParameterGroupIdentifier`, `dbStorageType`, `deploymentType`, `logDeliveryConfiguration`, `name`, `networkType`, `organization`, `password`, `port`, ... (+5) | - |
| `CreateDbInstanceOutput` | `structure` | `allocatedStorage`, `arn`, `availabilityZone`, `dbClusterId`, `dbInstanceType`, `dbParameterGroupIdentifier`, `dbStorageType`, `deploymentType`, `endpoint`, `id`, `influxAuthParametersSecretArn`, `instanceMode`, ... (+10) | - |
| `CreateDbParameterGroupInput` | `structure` | `description`, `name`, `parameters`, `tags` | - |
| `CreateDbParameterGroupOutput` | `structure` | `arn`, `description`, `id`, `name`, `parameters` | - |
| `DeleteDbClusterInput` | `structure` | `dbClusterId` | - |
| `DeleteDbClusterOutput` | `structure` | `dbClusterStatus` | - |
| `DeleteDbInstanceInput` | `structure` | `identifier` | - |
| `DeleteDbInstanceOutput` | `structure` | `allocatedStorage`, `arn`, `availabilityZone`, `dbClusterId`, `dbInstanceType`, `dbParameterGroupIdentifier`, `dbStorageType`, `deploymentType`, `endpoint`, `id`, `influxAuthParametersSecretArn`, `instanceMode`, ... (+10) | - |
| `GetDbClusterInput` | `structure` | `dbClusterId` | - |
| `GetDbClusterOutput` | `structure` | `allocatedStorage`, `arn`, `dbInstanceType`, `dbParameterGroupIdentifier`, `dbStorageType`, `deploymentType`, `endpoint`, `engineType`, `failoverMode`, `id`, `influxAuthParametersSecretArn`, `logDeliveryConfiguration`, ... (+8) | - |
| `GetDbInstanceInput` | `structure` | `identifier` | - |
| `GetDbInstanceOutput` | `structure` | `allocatedStorage`, `arn`, `availabilityZone`, `dbClusterId`, `dbInstanceType`, `dbParameterGroupIdentifier`, `dbStorageType`, `deploymentType`, `endpoint`, `id`, `influxAuthParametersSecretArn`, `instanceMode`, ... (+10) | - |
| `GetDbParameterGroupInput` | `structure` | `identifier` | - |
| `GetDbParameterGroupOutput` | `structure` | `arn`, `description`, `id`, `name`, `parameters` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

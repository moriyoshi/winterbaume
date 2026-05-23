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
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | A list of tags applied to the resource. |
| `TagResource` | `-` | `idempotent` | `resourceArn`, `tags` | - | `Unit` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Tags are composed of a Key/Value pairs. You can use tags to categorize and track your Timestream for InfluxDB resources. |
| `UntagResource` | `-` | `idempotent` | `resourceArn`, `tagKeys` | - | `Unit` | `ResourceNotFoundException` | Removes the tag from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request conflicts with an existing resource in Timestream for InfluxDB. |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception or failure. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The requested resource was not found or does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | The request exceeds the service quota. |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason | The input fails to satisfy the constraints specified by Timestream for InfluxDB. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `ClusterDeploymentType` | `enum` | MULTI_NODE_READ_REPLICAS | - |
| `ClusterStatus` | `enum` | CREATING, UPDATING, DELETING, AVAILABLE, FAILED, DELETED, MAINTENANCE, UPDATING_INSTANCE_TYPE, REBOOTING, REBOOT_FAILED, PARTIALLY_AVAILABLE | - |
| `DataFusionRuntimeType` | `enum` | MULTI_THREAD, MULTI_THREAD_ALT | - |
| `DbInstanceType` | `enum` | DB_INFLUX_MEDIUM, DB_INFLUX_LARGE, DB_INFLUX_XLARGE, DB_INFLUX_2XLARGE, DB_INFLUX_4XLARGE, DB_INFLUX_8XLARGE, DB_INFLUX_12XLARGE, DB_INFLUX_16XLARGE, DB_INFLUX_24XLARGE | - |
| `DbStorageType` | `enum` | INFLUX_IO_INCLUDED_T1, INFLUX_IO_INCLUDED_T2, INFLUX_IO_INCLUDED_T3 | - |
| `DeploymentType` | `enum` | SINGLE_AZ, WITH_MULTIAZ_STANDBY | - |
| `DurationType` | `enum` | HOURS, MINUTES, SECONDS, MILLISECONDS, DAYS | - |
| `EngineType` | `enum` | INFLUXDB_V2, INFLUXDB_V3_CORE, INFLUXDB_V3_ENTERPRISE | - |
| `FailoverMode` | `enum` | AUTOMATIC, NO_FAILOVER | - |
| `InstanceMode` | `enum` | PRIMARY, STANDBY, REPLICA, INGEST, QUERY, COMPACT, PROCESS | - |
| `LogFormats` | `enum` | FULL | - |
| `LogLevel` | `enum` | DEBUG, INFO, ERROR | - |
| `NetworkType` | `enum` | IPV4, DUAL | - |
| `Status` | `enum` | CREATING, AVAILABLE, DELETING, MODIFYING, UPDATING, DELETED, FAILED, UPDATING_DEPLOYMENT_TYPE, UPDATING_INSTANCE_TYPE, MAINTENANCE, REBOOTING, REBOOT_FAILED | - |
| `TracingType` | `enum` | LOG, JAEGER, DISABLED | - |
| `ValidationExceptionReason` | `enum` | FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

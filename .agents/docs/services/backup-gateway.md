# AWS Backup Gateway

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Backup gateway Backup gateway connects Backup to your hypervisor, so you can create, store, and restore backups of your virtual machines (VMs) anywhere, whether on-premises or in the VMware Cloud (VMC) on Amazon Web Services. Add on-premises resources by connecting to a hypervisor through a gateway. Backup will automatically discover the resources in your hypervisor. Use Backup to assign virtual or on-premises resources to a backup plan, or run on-demand backups. Once you have backed up your resources, you can view them and restore them like any resource supported by Backup.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Backup Gateway where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Backup Gateway by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: manage backup gateways, hypervisors, virtual machines, and gateway associations for hybrid backup.
- From the operation surface: support VM discovery, backup gateway connection state, hypervisor credentials, test connectivity, and tag-based hybrid inventory.

## Service Identity and Protocol

- AWS model slug: `backup-gateway`
- AWS SDK for Rust slug: `backupgateway`
- Model version: `2021-01-01`
- Model file: `vendor/api-models-aws/models/backup-gateway/service/2021-01-01/backup-gateway-2021-01-01.json`
- SDK ID: `Backup Gateway`
- Endpoint prefix: `backup-gateway`
- ARN namespace: `backup-gateway`
- CloudFormation name: `BackupGateway`
- CloudTrail event source: `backup-gateway.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (4), `Put` (3), `Update` (3), `Delete` (2), `Associate` (1), `Create` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateGatewayToServer`, `CreateGateway`, `DeleteGateway`, `DeleteHypervisor`, `DisassociateGatewayFromServer`, `ImportHypervisorConfiguration`, `PutBandwidthRateLimitSchedule`, `PutHypervisorPropertyMappings`, `PutMaintenanceStartTime`, `StartVirtualMachinesMetadataSync`, `TagResource`, `UntagResource`, `UpdateGatewayInformation`, `UpdateGatewaySoftwareNow`, `UpdateHypervisor`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBandwidthRateLimitSchedule`, `GetGateway`, `GetHypervisor`, `GetHypervisorPropertyMappings`, `GetVirtualMachine`, `ListGateways`, `ListHypervisors`, `ListTagsForResource`, `ListVirtualMachines`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportHypervisorConfiguration`, `StartVirtualMachinesMetadataSync`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 21 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BandwidthRateLimitScheduleResource` | `GatewayArn` | put: `PutBandwidthRateLimitSchedule`; read: `GetBandwidthRateLimitSchedule` | - | - |
| `GatewayResource` | `GatewayArn` | create: `CreateGateway`; read: `GetGateway`; update: `UpdateGatewayInformation`; delete: `DeleteGateway`; list: `ListGateways` | `AssociateGatewayToServer`, `DisassociateGatewayFromServer`, `PutMaintenanceStartTime`, `TestHypervisorConfiguration`, `UpdateGatewaySoftwareNow` | - |
| `HypervisorPropertyMappingResource` | `HypervisorArn` | put: `PutHypervisorPropertyMappings`; read: `GetHypervisorPropertyMappings` | - | - |
| `HypervisorResource` | `HypervisorArn` | create: `ImportHypervisorConfiguration`; read: `GetHypervisor`; update: `UpdateHypervisor`; delete: `DeleteHypervisor`; list: `ListHypervisors` | `StartVirtualMachinesMetadataSync` | - |
| `VirtualMachineResource` | `ResourceArn` | read: `GetVirtualMachine`; list: `ListVirtualMachines` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/aws-backup/latest/devguide/working-with-hypervisors.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html

Research outcomes:
- AWS Backup Gateway connects AWS Backup to on-premises virtualised workloads through gateways and hypervisor configurations.
- Hypervisors can be added, viewed, edited, deleted, and connected to additional gateways.
- Backup Gateway discovers virtual machines managed by connected hypervisors.
- Hypervisor credentials can be encrypted with KMS.
- Gateway and hypervisor status determine whether virtual machines can be discovered and protected.
- Backup jobs, vaults, recovery points, and restore jobs are still AWS Backup concepts, with Backup Gateway providing the on-premises connectivity layer.

Parity implications:
- Model gateways, hypervisors, gateway-hypervisor links, virtual machines, credentials/KMS settings, and status separately from backup plans and vaults.
- Virtual machine discovery should depend on connected gateway and hypervisor health.
- Backup Gateway resources should feed AWS Backup protected-resource selection rather than duplicating vault/recovery point logic.

## Current Network Resource Stub Semantics

Backup Gateway currently treats VPC endpoint information as gateway metadata.

- Gateway records include an optional `vpc_endpoint` string, and describe/list responses echo that value.
- There is no service-local VPC endpoint resource map and no lifecycle for endpoint creation or deletion.
- Gateway health and connection state are independent of the VPC endpoint value.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListTagsForResource`
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
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException` | Lists the tags applied to the resource identified by its Amazon Resource Name (ARN). |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceOutput` | `ResourceNotFoundException` | Tag the resource. |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceOutput` | `ResourceNotFoundException` | Removes tags from the resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | ErrorCode, Message | The operation cannot proceed because you have insufficient permissions. |
| `ConflictException` | `structure` | ErrorCode, Message | The operation cannot proceed because it is not supported. |
| `InternalServerException` | `structure` | ErrorCode, Message | The operation did not succeed because an internal error occurred. Try again later. |
| `ResourceNotFoundException` | `structure` | ErrorCode, Message | A resource that is required for the action wasn't found. |
| `ThrottlingException` | `structure` | ErrorCode, Message | TPS has been limited to protect against intentional or unintentional high request volumes. |
| `ValidationException` | `structure` | ErrorCode, Message | The operation did not succeed because a validation error occurred. |
| `ListTagsForResourceInput` | `structure` | ResourceArn | - |
| `ListTagsForResourceOutput` | `structure` | ResourceArn, Tags | - |
| `TagResourceInput` | `structure` | ResourceARN, Tags | - |
| `TagResourceOutput` | `structure` | ResourceARN | - |
| `UntagResourceInput` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceOutput` | `structure` | ResourceARN | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

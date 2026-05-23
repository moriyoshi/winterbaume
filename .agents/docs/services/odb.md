# odb

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Oracle Database@Amazon Web Services is an offering that enables you to access Oracle Exadata infrastructure managed by Oracle Cloud Infrastructure (OCI) inside Amazon Web Services data centers. You can migrate your Oracle Exadata workloads, establish low-latency connectivity with applications running on Amazon Web Services, and integrate with Amazon Web Services services. For example, you can run application servers in a Virtual Private Cloud (VPC) and access an Oracle Exadata system running in Oracle Database@Amazon Web Services. You can get started with Oracle Database@Amazon Web Services by using the familiar Amazon Web Services Management Console, APIs, or CLI. This interface reference for Oracle Database@Amazon Web Services contains documentation for a programming or command line interface that you can use to manage Oracle Database@Amazon Web Services.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for odb where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for odb by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for odb resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented odb workflows in the local mock. Key resources include `CloudAutonomousVmClusterResource`, `CloudExadataInfrastructureResource`, `CloudVmClusterResource`, `DbNodeResource`, `OdbNetworkResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListAutonomousVirtualMachines`, `ListCloudAutonomousVmClusters`, `ListCloudExadataInfrastructures`, `ListCloudVmClusters`, `GetCloudAutonomousVmCluster`, `GetCloudExadataInfrastructure`.

## Service Identity and Protocol

- AWS model slug: `odb`
- AWS SDK for Rust slug: `odb`
- Model version: `2024-08-20`
- Model file: `vendor/api-models-aws/models/odb/service/2024-08-20/odb-2024-08-20.json`
- SDK ID: `odb`
- Endpoint prefix: `-`
- ARN namespace: `odb`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Get` (9), `Create` (5), `Delete` (5), `Update` (3), `Accept` (1), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptMarketplaceRegistration`, `AssociateIamRoleToResource`, `CreateCloudAutonomousVmCluster`, `CreateCloudExadataInfrastructure`, `CreateCloudVmCluster`, `CreateOdbNetwork`, `CreateOdbPeeringConnection`, `DeleteCloudAutonomousVmCluster`, `DeleteCloudExadataInfrastructure`, `DeleteCloudVmCluster`, `DeleteOdbNetwork`, `DeleteOdbPeeringConnection`, `DisassociateIamRoleFromResource`, `StartDbNode`, `StopDbNode`, `TagResource`, `UntagResource`, `UpdateCloudExadataInfrastructure`, `UpdateOdbNetwork`, `UpdateOdbPeeringConnection`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCloudAutonomousVmCluster`, `GetCloudExadataInfrastructure`, `GetCloudExadataInfrastructureUnallocatedResources`, `GetCloudVmCluster`, `GetDbNode`, `GetDbServer`, `GetOciOnboardingStatus`, `GetOdbNetwork`, `GetOdbPeeringConnection`, `ListAutonomousVirtualMachines`, `ListCloudAutonomousVmClusters`, `ListCloudExadataInfrastructures`, `ListCloudVmClusters`, `ListDbNodes`, `ListDbServers`, `ListDbSystemShapes`, `ListGiVersions`, `ListOdbNetworks`, `ListOdbPeeringConnections`, `ListSystemVersions`, ... (+1).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDbNode`, `StopDbNode`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 43 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`, `ECS`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CloudAutonomousVmClusterResource` | `cloudAutonomousVmClusterId` | create: `CreateCloudAutonomousVmCluster`; read: `GetCloudAutonomousVmCluster`; delete: `DeleteCloudAutonomousVmCluster`; list: `ListCloudAutonomousVmClusters` | `ListAutonomousVirtualMachines` | - |
| `CloudExadataInfrastructureResource` | `cloudExadataInfrastructureId` | create: `CreateCloudExadataInfrastructure`; read: `GetCloudExadataInfrastructure`; update: `UpdateCloudExadataInfrastructure`; delete: `DeleteCloudExadataInfrastructure`; list: `ListCloudExadataInfrastructures` | `GetCloudExadataInfrastructureUnallocatedResources`, `GetDbServer`, `ListDbServers` | - |
| `CloudVmClusterResource` | `cloudVmClusterId` | create: `CreateCloudVmCluster`; read: `GetCloudVmCluster`; delete: `DeleteCloudVmCluster`; list: `ListCloudVmClusters` | - | - |
| `DbNodeResource` | `dbNodeId` | read: `GetDbNode`; list: `ListDbNodes` | `RebootDbNode`, `StartDbNode`, `StopDbNode` | - |
| `OdbNetworkResource` | `odbNetworkId` | create: `CreateOdbNetwork`; read: `GetOdbNetwork`; update: `UpdateOdbNetwork`; delete: `DeleteOdbNetwork`; list: `ListOdbNetworks` | - | - |
| `OdbPeeringConnectionResource` | `odbPeeringConnectionId` | create: `CreateOdbPeeringConnection`; read: `GetOdbPeeringConnection`; update: `UpdateOdbPeeringConnection`; delete: `DeleteOdbPeeringConnection`; list: `ListOdbPeeringConnections` | - | - |
## Operation Groups

### List

- Operations: `ListDbSystemShapes`, `ListGiVersions`, `ListSystemVersions`, `ListTagsForResource`
- Traits: `readonly` (4), `paginated` (3)
- Common required input members in this group: -

### Accept

- Operations: `AcceptMarketplaceRegistration`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Associate

- Operations: `AssociateIamRoleToResource`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateIamRoleFromResource`
- Common required input members in this group: -

### Get

- Operations: `GetOciOnboardingStatus`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Initialize

- Operations: `InitializeService`
- Traits: `idempotent` (1)
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
| `AcceptMarketplaceRegistration` | `-` | `idempotent` | `marketplaceRegistrationToken` | - | `AcceptMarketplaceRegistrationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Registers the Amazon Web Services Marketplace token for your Amazon Web Services account to activate your Oracle Database@Amazon Web Services subscription. |
| `AssociateIamRoleToResource` | `-` | - | `iamRoleArn`, `awsIntegration`, `resourceArn` | - | `AssociateIamRoleToResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates an Amazon Web Services Identity and Access Management (IAM) service role with a specified resource to enable Amazon Web Services service integration. |
| `DisassociateIamRoleFromResource` | `-` | - | `iamRoleArn`, `awsIntegration`, `resourceArn` | - | `DisassociateIamRoleFromResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an Amazon Web Services Identity and Access Management (IAM) service role from a specified resource to disable Amazon Web Services service integration. |
| `GetOciOnboardingStatus` | `-` | `readonly` | - | - | `GetOciOnboardingStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the tenancy activation link and onboarding status for your Amazon Web Services account. |
| `InitializeService` | `-` | `idempotent` | - | - | `InitializeServiceOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Initializes the ODB service for the first time in an account. |
| `ListDbSystemShapes` | `-` | `readonly`, `paginated` | - | - | `ListDbSystemShapesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about the shapes that are available for an Exadata infrastructure. |
| `ListGiVersions` | `-` | `readonly`, `paginated` | - | - | `ListGiVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about Oracle Grid Infrastructure (GI) software versions that are available for a VM cluster for the specified shape. |
| `ListSystemVersions` | `-` | `readonly`, `paginated` | `giVersion`, `shape` | - | `ListSystemVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the system versions that are available for a VM cluster for the specified giVersion and shape . |
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Returns information about the tags applied to this resource. |
| `TagResource` | `-` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Applies tags to the specified resource. |
| `UntagResource` | `-` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListDbSystemShapes` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListGiVersions` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListSystemVersions` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient access to perform this action. Make sure you have the required permissions and try again. |
| `ConflictException` | `structure` | message, resourceId, resourceType | Occurs when a conflict with the current status of your resource. Fix any inconsistencies with your resource and try again. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | Occurs when there is an internal failure in the Oracle Database@Amazon Web Services service. Wait and try again. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The operation tried to access a resource that doesn't exist. Make sure you provided the correct resource and try again. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, quotaCode | You have exceeded the service quota. |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The request has failed validation because it is missing required fields or has invalid inputs. |
| `AcceptMarketplaceRegistrationInput` | `structure` | marketplaceRegistrationToken | - |
| `AcceptMarketplaceRegistrationOutput` | `structure` | **empty (no members)** | - |
| `AssociateIamRoleToResourceInput` | `structure` | iamRoleArn, awsIntegration, resourceArn | - |
| `AssociateIamRoleToResourceOutput` | `structure` | **empty (no members)** | - |
| `DisassociateIamRoleFromResourceInput` | `structure` | iamRoleArn, awsIntegration, resourceArn | - |
| `DisassociateIamRoleFromResourceOutput` | `structure` | **empty (no members)** | - |
| `GetOciOnboardingStatusInput` | `structure` | **empty (no members)** | - |
| `GetOciOnboardingStatusOutput` | `structure` | status, existingTenancyActivationLink, newTenancyActivationLink, ociIdentityDomain | - |
| `InitializeServiceInput` | `structure` | ociIdentityDomain | - |
| `InitializeServiceOutput` | `structure` | **empty (no members)** | - |
| `ListDbSystemShapesInput` | `structure` | maxResults, nextToken, availabilityZone, availabilityZoneId | - |
| `ListDbSystemShapesOutput` | `structure` | nextToken, dbSystemShapes | - |
| `ListGiVersionsInput` | `structure` | maxResults, nextToken, shape | - |
| `ListGiVersionsOutput` | `structure` | nextToken, giVersions | - |
| `ListSystemVersionsInput` | `structure` | maxResults, nextToken, giVersion, shape | - |
| `ListSystemVersionsOutput` | `structure` | nextToken, systemVersions | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `Access` | `enum` | ENABLED, DISABLED | - |
| `ComputeModel` | `enum` | ECPU, OCPU | - |
| `DayOfWeekName` | `enum` | MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY | - |
| `DbNodeMaintenanceType` | `enum` | VMDB_REBOOT_MIGRATION | - |
| `DbNodeResourceStatus` | `enum` | AVAILABLE, FAILED, PROVISIONING, TERMINATED, TERMINATING, UPDATING, STOPPING, STOPPED, STARTING | - |
| `DbServerPatchingStatus` | `enum` | COMPLETE, FAILED, MAINTENANCE_IN_PROGRESS, SCHEDULED | - |
| `DiskRedundancy` | `enum` | HIGH, NORMAL | - |
| `IamRoleStatus` | `enum` | ASSOCIATING, DISASSOCIATING, FAILED, CONNECTED, DISCONNECTED, PARTIALLY_CONNECTED, UNKNOWN | - |
| `IormLifecycleState` | `enum` | BOOTSTRAPPING, DISABLED, ENABLED, FAILED, UPDATING | - |
| `LicenseModel` | `enum` | BRING_YOUR_OWN_LICENSE, LICENSE_INCLUDED | - |
| `ManagedResourceStatus` | `enum` | ENABLED, ENABLING, DISABLED, DISABLING | - |
| `MonthName` | `enum` | JANUARY, FEBRUARY, MARCH, APRIL, MAY, JUNE, JULY, AUGUST, SEPTEMBER, OCTOBER, NOVEMBER, DECEMBER | - |
| `Objective` | `enum` | AUTO, BALANCED, BASIC, HIGH_THROUGHPUT, LOW_LATENCY | - |
| `OciOnboardingStatus` | `enum` | NOT_STARTED, PENDING_LINK_GENERATION, PENDING_CUSTOMER_ACTION, PENDING_INITIALIZATION, ACTIVATING, ACTIVE_IN_HOME_REGION, ACTIVE, ACTIVE_LIMITED, FAILED, PUBLIC_OFFER_UNSUPPORTED, SUSPENDED, CANCELED | - |
| `PatchingModeType` | `enum` | ROLLING, NONROLLING | - |
| `PreferenceType` | `enum` | NO_PREFERENCE, CUSTOM_PREFERENCE | - |
| `ResourceStatus` | `enum` | AVAILABLE, FAILED, PROVISIONING, TERMINATED, TERMINATING, UPDATING, MAINTENANCE_IN_PROGRESS | - |
| `ShapeType` | `enum` | AMD, INTEL, INTEL_FLEX_X9, AMPERE_FLEX_A1 | - |
| `SupportedAwsIntegration` | `enum` | KmsTde | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
| `VpcEndpointType` | `enum` | SERVICENETWORK | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

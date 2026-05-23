# Amazon Workspaces Instances

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon WorkSpaces Instances provides an API framework for managing virtual workspace environments across multiple AWS regions, enabling programmatic creation and configuration of desktop infrastructure.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Workspaces Instances where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Workspaces Instances by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Workspaces Instances workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Associate`, `Disassociate` operation families, including `ListInstanceTypes`, `ListRegions`, `ListTagsForResource`, `ListWorkspaceInstances`, `CreateVolume`, `CreateWorkspaceInstance`.

## Service Identity and Protocol

- AWS model slug: `workspaces-instances`
- AWS SDK for Rust slug: `workspacesinstances`
- Model version: `2022-07-26`
- Model file: `vendor/api-models-aws/models/workspaces-instances/service/2022-07-26/workspaces-instances-2022-07-26.json`
- SDK ID: `Workspaces Instances`
- Endpoint prefix: `-`
- ARN namespace: `workspaces-instances`
- CloudFormation name: `WorkspacesInstances`
- CloudTrail event source: `workspaces-instances.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (2), `Delete` (2), `Associate` (1), `Disassociate` (1), `Get` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateVolume`, `CreateVolume`, `CreateWorkspaceInstance`, `DeleteVolume`, `DeleteWorkspaceInstance`, `DisassociateVolume`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetWorkspaceInstance`, `ListInstanceTypes`, `ListRegions`, `ListTagsForResource`, `ListWorkspaceInstances`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListInstanceTypes`, `ListRegions`, `ListTagsForResource`, `ListWorkspaceInstances`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `WorkspaceInstanceId`

### Create

- Operations: `CreateVolume`, `CreateWorkspaceInstance`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `AvailabilityZone`, `ManagedInstance`

### Delete

- Operations: `DeleteVolume`, `DeleteWorkspaceInstance`
- Common required input members in this group: `VolumeId`, `WorkspaceInstanceId`

### Associate

- Operations: `AssociateVolume`
- Common required input members in this group: `Device`, `VolumeId`, `WorkspaceInstanceId`

### Disassociate

- Operations: `DisassociateVolume`
- Common required input members in this group: `VolumeId`, `WorkspaceInstanceId`

### Get

- Operations: `GetWorkspaceInstance`
- Traits: `readonly` (1)
- Common required input members in this group: `WorkspaceInstanceId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Tags`, `WorkspaceInstanceId`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `TagKeys`, `WorkspaceInstanceId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateVolume` | - | - | `Device`, `VolumeId`, `WorkspaceInstanceId` | - | `AssociateVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a volume to a WorkSpace Instance. |
| `CreateVolume` | - | `idempotent`, `idempotency-token` | `AvailabilityZone` | `ClientToken` | `CreateVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new volume for WorkSpace Instances. |
| `CreateWorkspaceInstance` | - | `idempotent`, `idempotency-token` | `ManagedInstance` | `ClientToken` | `CreateWorkspaceInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Launches a new WorkSpace Instance with specified configuration parameters, enabling programmatic workspace deployment. |
| `DeleteVolume` | - | - | `VolumeId` | - | `DeleteVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a specified volume. |
| `DeleteWorkspaceInstance` | - | - | `WorkspaceInstanceId` | - | `DeleteWorkspaceInstanceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified WorkSpace Usage of this API will result in deletion of the resource in question. |
| `DisassociateVolume` | - | - | `VolumeId`, `WorkspaceInstanceId` | - | `DisassociateVolumeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Detaches a volume from a WorkSpace Instance. |
| `GetWorkspaceInstance` | - | `readonly` | `WorkspaceInstanceId` | - | `GetWorkspaceInstanceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific WorkSpace Instance. |
| `ListInstanceTypes` | - | `readonly`, `paginated` | - | - | `ListInstanceTypesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of instance types supported by Amazon WorkSpaces Instances, enabling precise workspace infrastructure configuration. |
| `ListRegions` | - | `readonly`, `paginated` | - | - | `ListRegionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of AWS regions supported by Amazon WorkSpaces Instances, enabling region discovery for workspace deployments. |
| `ListTagsForResource` | - | `readonly` | `WorkspaceInstanceId` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves tags for a WorkSpace Instance. |
| `ListWorkspaceInstances` | - | `readonly`, `paginated` | - | - | `ListWorkspaceInstancesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a collection of WorkSpaces Instances based on specified filters. |
| `TagResource` | - | - | `Tags`, `WorkspaceInstanceId` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to a WorkSpace Instance. |
| `UntagResource` | - | - | `TagKeys`, `WorkspaceInstanceId` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a WorkSpace Instance. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | Indicates insufficient permissions to perform the requested action. |
| `InternalServerException` | `structure` | `Message`, `RetryAfterSeconds` | Indicates an unexpected server-side error occurred. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `RetryAfterSeconds`, `ServiceCode` | Indicates the request rate has exceeded limits. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | Indicates invalid input parameters in the request. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Indicates the requested resource could not be found. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Signals a conflict with the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | Indicates that a service quota has been exceeded. |
| `AssociateVolumeRequest` | `structure` | `Device`, `VolumeId`, `WorkspaceInstanceId` | Specifies volume attachment parameters. |
| `AssociateVolumeResponse` | `structure` | - | Confirms volume attachment. |
| `CreateVolumeRequest` | `structure` | `AvailabilityZone`, `ClientToken`, `Encrypted`, `Iops`, `KmsKeyId`, `SizeInGB`, `SnapshotId`, `TagSpecifications`, `Throughput`, `VolumeType` | Specifies volume creation parameters. |
| `CreateVolumeResponse` | `structure` | `VolumeId` | Returns the created volume identifier. |
| `CreateWorkspaceInstanceRequest` | `structure` | `BillingConfiguration`, `ClientToken`, `ManagedInstance`, `Tags` | Defines the configuration parameters for creating a new WorkSpaces Instance. |
| `CreateWorkspaceInstanceResponse` | `structure` | `WorkspaceInstanceId` | Returns the unique identifier for the newly created WorkSpaces Instance. |
| `DeleteVolumeRequest` | `structure` | `VolumeId` | Specifies the volume to delete. |
| `DeleteVolumeResponse` | `structure` | - | Confirms volume deletion. |
| `DeleteWorkspaceInstanceRequest` | `structure` | `WorkspaceInstanceId` | The WorkSpace to delete |
| `DeleteWorkspaceInstanceResponse` | `structure` | - | Confirms the successful deletion of the specified WorkSpace Instance. |
| `DisassociateVolumeRequest` | `structure` | `Device`, `DisassociateMode`, `VolumeId`, `WorkspaceInstanceId` | Specifies volume detachment parameters. |
| `DisassociateVolumeResponse` | `structure` | - | Confirms volume detachment. |
| `GetWorkspaceInstanceRequest` | `structure` | `WorkspaceInstanceId` | Identifies the WorkSpaces Instance to retrieve detailed information for. |
| `GetWorkspaceInstanceResponse` | `structure` | `BillingConfiguration`, `EC2InstanceErrors`, `EC2ManagedInstance`, `ProvisionState`, `WorkspaceInstanceErrors`, `WorkspaceInstanceId` | Provides comprehensive details about the requested WorkSpaces Instance. |
| `ListInstanceTypesRequest` | `structure` | `InstanceConfigurationFilter`, `MaxResults`, `NextToken` | Defines input parameters for retrieving supported WorkSpaces Instances instance types. |
| `ListInstanceTypesResponse` | `structure` | `InstanceTypes`, `NextToken` | Contains the list of instance types supported by WorkSpaces Instances. |
| `ListRegionsRequest` | `structure` | `MaxResults`, `NextToken` | Defines input parameters for retrieving supported WorkSpaces Instances regions. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

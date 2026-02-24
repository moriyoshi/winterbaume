# Amazon Elastic VMware Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic VMware Service (Amazon EVS) is a service that you can use to deploy a VMware Cloud Foundation (VCF) software environment directly on EC2 bare metal instances within an Amazon Virtual Private Cloud (VPC). Workloads running on Amazon EVS are fully compatible with workloads running on any standard VMware vSphere environment. This means that you can migrate any VMware-based workload to Amazon EVS without workload modification.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Elastic VMware Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Elastic VMware Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Elastic VMware Service workflows in the local mock. Key resources include `EnvironmentResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Associate` operation families, including `ListEnvironmentHosts`, `ListEnvironmentVlans`, `ListEnvironments`, `ListTagsForResource`, `CreateEnvironment`, `CreateEnvironmentHost`.

## Service Identity and Protocol

- AWS model slug: `evs`
- AWS SDK for Rust slug: `evs`
- Model version: `2023-07-27`
- Model file: `vendor/api-models-aws/models/evs/service/2023-07-27/evs-2023-07-27.json`
- SDK ID: `evs`
- Endpoint prefix: `-`
- ARN namespace: `evs`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Create` (2), `Delete` (2), `Get` (2), `Associate` (1), `Disassociate` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateEipToVlan`, `CreateEnvironment`, `CreateEnvironmentHost`, `DeleteEnvironment`, `DeleteEnvironmentHost`, `DisassociateEipFromVlan`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEnvironment`, `GetVersions`, `ListEnvironmentHosts`, `ListEnvironmentVlans`, `ListEnvironments`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `EnvironmentResource` | `environmentId` | create: `CreateEnvironment`; read: `GetEnvironment`; delete: `DeleteEnvironment`; list: `ListEnvironments` | `AssociateEipToVlan`, `CreateEnvironmentHost`, `DeleteEnvironmentHost`, `DisassociateEipFromVlan`, `ListEnvironmentHosts`, `ListEnvironmentVlans` | - |
## Operation Groups

### List

- Operations: `ListEnvironmentHosts`, `ListEnvironmentVlans`, `ListEnvironments`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `environmentId`, `resourceArn`

### Create

- Operations: `CreateEnvironment`, `CreateEnvironmentHost`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `connectivityInfo`, `environmentId`, `host`, `hosts`, `initialVlans`, `licenseInfo`, `serviceAccessSubnetId`, `siteId`, `termsAccepted`, `vcfHostnames`, `vcfVersion`, `vpcId`

### Delete

- Operations: `DeleteEnvironment`, `DeleteEnvironmentHost`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `environmentId`, `hostName`

### Get

- Operations: `GetEnvironment`, `GetVersions`
- Traits: `readonly` (2)
- Common required input members in this group: `environmentId`

### Associate

- Operations: `AssociateEipToVlan`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `allocationId`, `environmentId`, `vlanName`

### Disassociate

- Operations: `DisassociateEipFromVlan`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `associationId`, `environmentId`, `vlanName`

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
| `AssociateEipToVlan` | - | `idempotent`, `idempotency-token` | `allocationId`, `environmentId`, `vlanName` | `clientToken` | `AssociateEipToVlanResponse` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates an Elastic IP address with a public HCX VLAN. This operation is only allowed for public HCX VLANs at this time. |
| `CreateEnvironment` | - | `idempotent`, `idempotency-token` | `connectivityInfo`, `hosts`, `initialVlans`, `licenseInfo`, `serviceAccessSubnetId`, `siteId`, `termsAccepted`, `vcfHostnames`, `vcfVersion`, `vpcId` | `clientToken` | `CreateEnvironmentResponse` | `ValidationException` | Creates an Amazon EVS environment that runs VCF software, such as SDDC Manager, NSX Manager, and vCenter Server. During environment creation, Amazon EVS performs validations on DNS settings, provisions VLAN subnets and hosts, and deploys the supplied version... |
| `CreateEnvironmentHost` | - | `idempotent`, `idempotency-token` | `environmentId`, `host` | `clientToken` | `CreateEnvironmentHostResponse` | `ThrottlingException`, `ValidationException` | Creates an ESX host and adds it to an Amazon EVS environment. Amazon EVS supports 4-16 hosts per environment. |
| `DeleteEnvironment` | - | `idempotent`, `idempotency-token` | `environmentId` | `clientToken` | `DeleteEnvironmentResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes an Amazon EVS environment. Amazon EVS environments will only be enabled for deletion once the hosts are deleted. |
| `DeleteEnvironmentHost` | - | `idempotent`, `idempotency-token` | `environmentId`, `hostName` | `clientToken` | `DeleteEnvironmentHostResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes a host from an Amazon EVS environment. Before deleting a host, you must unassign and decommission the host from within the SDDC Manager user interface. |
| `DisassociateEipFromVlan` | - | `idempotent`, `idempotency-token` | `associationId`, `environmentId`, `vlanName` | `clientToken` | `DisassociateEipFromVlanResponse` | `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an Elastic IP address from a public HCX VLAN. This operation is only allowed for public HCX VLANs at this time. |
| `GetEnvironment` | - | `readonly` | `environmentId` | - | `GetEnvironmentResponse` | `ResourceNotFoundException`, `ValidationException` | Returns a description of the specified environment. |
| `GetVersions` | - | `readonly` | - | - | `GetVersionsResponse` | `InternalServerException`, `ThrottlingException` | Returns information about VCF versions, ESX versions and EC2 instance types provided by Amazon EVS. For each VCF version, the response also includes the default ESX version and provided EC2 instance types. |
| `ListEnvironmentHosts` | - | `readonly`, `paginated` | `environmentId` | - | `ListEnvironmentHostsResponse` | `ResourceNotFoundException`, `ValidationException` | List the hosts within an environment. |
| `ListEnvironmentVlans` | - | `readonly`, `paginated` | `environmentId` | - | `ListEnvironmentVlansResponse` | `ResourceNotFoundException`, `ValidationException` | Lists environment VLANs that are associated with the specified environment. |
| `ListEnvironments` | - | `readonly`, `paginated` | - | - | `ListEnvironmentsResponse` | `ValidationException` | Lists the Amazon EVS environments in your Amazon Web Services account in the specified Amazon Web Services Region. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags for an Amazon EVS resource. |
| `TagResource` | - | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TagPolicyException`, `TooManyTagsException` | Associates the specified tags to an Amazon EVS resource with the specified `resourceArn`. If existing tags on a resource are not specified in the request parameters, they aren't changed. |
| `UntagResource` | - | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `TagPolicyException` | Deletes specified tags from an Amazon EVS resource. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | A service resource associated with the request could not be found. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the specified constraints. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | The operation could not be performed because the service is throttling requests. |
| `TagPolicyException` | `structure` | `message` | `TagPolicyException` is deprecated. |
| `AssociateEipToVlanRequest` | `structure` | `allocationId`, `clientToken`, `environmentId`, `vlanName` | - |
| `AssociateEipToVlanResponse` | `structure` | `vlan` | - |
| `CreateEnvironmentRequest` | `structure` | `clientToken`, `connectivityInfo`, `environmentName`, `hosts`, `initialVlans`, `kmsKeyId`, `licenseInfo`, `serviceAccessSecurityGroups`, `serviceAccessSubnetId`, `siteId`, `tags`, `termsAccepted`, ... (+3) | - |
| `CreateEnvironmentResponse` | `structure` | `environment` | - |
| `CreateEnvironmentHostRequest` | `structure` | `clientToken`, `environmentId`, `esxVersion`, `host` | - |
| `CreateEnvironmentHostResponse` | `structure` | `environmentSummary`, `host` | - |
| `DeleteEnvironmentRequest` | `structure` | `clientToken`, `environmentId` | - |
| `DeleteEnvironmentResponse` | `structure` | `environment` | - |
| `DeleteEnvironmentHostRequest` | `structure` | `clientToken`, `environmentId`, `hostName` | - |
| `DeleteEnvironmentHostResponse` | `structure` | `environmentSummary`, `host` | - |
| `DisassociateEipFromVlanRequest` | `structure` | `associationId`, `clientToken`, `environmentId`, `vlanName` | - |
| `DisassociateEipFromVlanResponse` | `structure` | `vlan` | - |
| `GetEnvironmentRequest` | `structure` | `environmentId` | - |
| `GetEnvironmentResponse` | `structure` | `environment` | - |
| `GetVersionsRequest` | `structure` | - | - |
| `GetVersionsResponse` | `structure` | `instanceTypeEsxVersions`, `vcfVersions` | - |
| `InternalServerException` | `structure` | `message` | An internal server error occurred. |
| `ListEnvironmentHostsRequest` | `structure` | `environmentId`, `maxResults`, `nextToken` | - |
| `ListEnvironmentHostsResponse` | `structure` | `environmentHosts`, `nextToken` | - |
| `ListEnvironmentVlansRequest` | `structure` | `environmentId`, `maxResults`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

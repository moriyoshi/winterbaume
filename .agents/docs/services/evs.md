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

### Get

- Operations: `GetVersions`
- Traits: `readonly` (1)
- Common required input members in this group: -

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
| `GetVersions` | `-` | `readonly` | - | - | `GetVersionsResponse` | `InternalServerException`, `ThrottlingException` | Returns information about VCF versions, ESX versions and EC2 instance types provided by Amazon EVS. For each VCF version, the response also includes the default ESX version and provided EC2 instance types. |
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags for an Amazon EVS resource. |
| `TagResource` | `-` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TagPolicyException`, `TooManyTagsException` | Associates the specified tags to an Amazon EVS resource with the specified resourceArn . If existing tags on a resource are not specified in the request parameters, they aren't changed. When a resource is deleted, th ... |
| `UntagResource` | `-` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `TagPolicyException` | Deletes specified tags from an Amazon EVS resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | message | An internal server error occurred. Retry your request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | A service resource associated with the request could not be found. The resource might not be specified correctly, or it may have a state of DELETED . |
| `ServiceQuotaExceededException` | `structure` | message | The number of one or more Amazon EVS resources exceeds the maximum allowed. For a list of Amazon EVS quotas, see Amazon EVS endpoints and quotas in the Amaz ... |
| `TagPolicyException` | `structure` | message | TagPolicyException is deprecated. See ValidationException instead. The request doesn't comply with IAM tag policy. Correct your request and then retry it. |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | The operation could not be performed because the service is throttling requests. This exception is thrown when the service endpoint receives too many concur ... |
| `TooManyTagsException` | `structure` | message | TooManyTagsException is deprecated. See ServiceQuotaExceededException instead. A service resource associated with the request has more than 200 tags. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the specified constraints. You will see this exception if invalid inputs are provided for any of the Amazon EVS environment opera ... |
| `GetVersionsRequest` | `structure` | **empty (no members)** | - |
| `GetVersionsResponse` | `structure` | vcfVersions, instanceTypeEsxVersions | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `CheckResult` | `enum` | PASSED, FAILED, UNKNOWN | - |
| `CheckType` | `enum` | KEY_REUSE, KEY_COVERAGE, REACHABILITY, HOST_COUNT, VCENTER_REACHABILITY, VCENTER_VM_SYNC, VCENTER_VM_EVENT | - |
| `ConnectorState` | `enum` | CREATING, CREATE_FAILED, ACTIVE, UPDATING, UPDATE_FAILED, DELETING, DELETED | - |
| `ConnectorType` | `enum` | VCENTER | - |
| `EntitlementStatus` | `enum` | CREATING, CREATED, DELETED, AT_RISK, ENTITLEMENT_REMOVED, CREATE_FAILED | - |
| `EntitlementType` | `enum` | WINDOWS_SERVER | - |
| `EnvironmentState` | `enum` | CREATING, CREATED, DELETING, DELETED, CREATE_FAILED | - |
| `HostState` | `enum` | CREATING, CREATED, UPDATING, DELETING, DELETED, CREATE_FAILED, UPDATE_FAILED | - |
| `InstanceType` | `enum` | I4I_METAL, I7I_METAL_24XL | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
| `VcfVersion` | `enum` | VCF_5_2_1, VCF_5_2_2 | - |
| `VlanState` | `enum` | CREATING, CREATED, DELETING, DELETED, CREATE_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

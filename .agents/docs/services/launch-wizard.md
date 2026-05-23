# AWS Launch Wizard

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Launch Wizard offers a guided way of sizing, configuring, and deploying Amazon Web Services resources for third party applications, such as Microsoft SQL Server Always On and HANA based SAP systems, without the need to manually identify and provision individual Amazon Web Services resources.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Launch Wizard workflows in the local mock. Key resources include `Deployment`, `DeploymentEvent`, `DeploymentPatternVersion`, `SettingsSet`, `Workload`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Tag` operation families, including `ListDeploymentEvents`, `ListDeploymentPatternVersions`, `ListDeployments`, `ListTagsForResource`, `GetDeployment`, `GetDeploymentPatternVersion`.

## Service Identity and Protocol

- AWS model slug: `launch-wizard`
- AWS SDK for Rust slug: `launchwizard`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/launch-wizard/service/2018-05-10/launch-wizard-2018-05-10.json`
- SDK ID: `Launch Wizard`
- Endpoint prefix: `launchwizard`
- ARN namespace: `launchwizard`
- CloudFormation name: `LaunchWizard`
- CloudTrail event source: `launchwizard.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Get` (4), `Create` (1), `Delete` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateDeployment`, `DeleteDeployment`, `TagResource`, `UntagResource`, `UpdateDeployment`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDeployment`, `GetDeploymentPatternVersion`, `GetWorkload`, `GetWorkloadDeploymentPattern`, `ListDeploymentEvents`, `ListDeploymentPatternVersions`, `ListDeployments`, `ListTagsForResource`, `ListWorkloadDeploymentPatterns`, `ListWorkloads`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 15 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Deployment` | `deploymentId` | create: `CreateDeployment`; read: `GetDeployment`; update: `UpdateDeployment`; delete: `DeleteDeployment`; list: `ListDeployments` | - | - |
| `DeploymentEvent` | `deploymentId`, `eventId` | list: `ListDeploymentEvents` | - | - |
| `DeploymentPatternVersion` | `deploymentPatternName`, `deploymentPatternVersionName`, `workloadName` | read: `GetDeploymentPatternVersion`; list: `ListDeploymentPatternVersions` | - | - |
| `SettingsSet` | `name` | - | - | - |
| `Workload` | `workloadName` | read: `GetWorkload`; list: `ListWorkloads` | - | - |
| `WorkloadDeploymentPattern` | `deploymentPatternName`, `workloadName` | read: `GetWorkloadDeploymentPattern`; list: `ListWorkloadDeploymentPatterns` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags associated with a specified resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds the specified tags to the given resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified tags from the given resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | message | An internal error has occurred. Retry your request, but if the problem persists, contact us with details by posting a question on re:Post . |
| `ResourceLimitException` | `structure` | message | You have exceeded an Launch Wizard resource limit. For example, you might have too many deployments in progress. |
| `ResourceNotFoundException` | `structure` | message | The specified workload or deployment resource can't be found. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `DeploymentFilterKey` | `enum` | WORKLOAD_NAME, DEPLOYMENT_STATUS | - |
| `DeploymentPatternVersionFilterKey` | `enum` | DEPLOYMENT_PATTERN_VERSION_NAME | - |
| `DeploymentStatus` | `enum` | COMPLETED, CREATING, DELETE_IN_PROGRESS, DELETE_INITIATING, DELETE_FAILED, DELETED, FAILED, IN_PROGRESS, VALIDATING, UPDATE_IN_PROGRESS, UPDATE_COMPLETED, UPDATE_FAILED, ... (+2) | - |
| `EventStatus` | `enum` | CANCELED, CANCELING, COMPLETED, CREATED, FAILED, IN_PROGRESS, PENDING, TIMED_OUT | - |
| `WorkloadDeploymentPatternStatus` | `enum` | ACTIVE, INACTIVE, DISABLED, DELETED | - |
| `WorkloadStatus` | `enum` | ACTIVE, INACTIVE, DISABLED, DELETED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

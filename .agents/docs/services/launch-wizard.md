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

- Operations: `ListDeploymentEvents`, `ListDeploymentPatternVersions`, `ListDeployments`, `ListTagsForResource`, `ListWorkloadDeploymentPatterns`, `ListWorkloads`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `deploymentId`, `deploymentPatternName`, `resourceArn`, `workloadName`

### Get

- Operations: `GetDeployment`, `GetDeploymentPatternVersion`, `GetWorkload`, `GetWorkloadDeploymentPattern`
- Traits: `readonly` (4)
- Common required input members in this group: `deploymentId`, `deploymentPatternName`, `deploymentPatternVersionName`, `workloadName`

### Create

- Operations: `CreateDeployment`
- Common required input members in this group: `deploymentPatternName`, `name`, `specifications`, `workloadName`

### Delete

- Operations: `DeleteDeployment`
- Traits: `idempotent` (1)
- Common required input members in this group: `deploymentId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateDeployment`
- Traits: `idempotent` (1)
- Common required input members in this group: `deploymentId`, `specifications`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateDeployment` | `POST /createDeployment` | - | `deploymentPatternName`, `name`, `specifications`, `workloadName` | - | `CreateDeploymentOutput` | `InternalServerException`, `ResourceLimitException`, `ResourceNotFoundException`, `ValidationException` | Creates a deployment for the given workload. Deployments created by this operation are not available in the Launch Wizard console to use the `Clone deployment` action on. |
| `DeleteDeployment` | `POST /deleteDeployment` | `idempotent` | `deploymentId` | - | `DeleteDeploymentOutput` | `InternalServerException`, `ResourceLimitException`, `ResourceNotFoundException`, `ValidationException` | Deletes a deployment. |
| `GetDeployment` | `POST /getDeployment` | `readonly` | `deploymentId` | - | `GetDeploymentOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about the deployment. |
| `GetDeploymentPatternVersion` | `POST /getDeploymentPatternVersion` | `readonly` | `deploymentPatternName`, `deploymentPatternVersionName`, `workloadName` | - | `GetDeploymentPatternVersionOutput` | `InternalServerException`, `ResourceNotFoundException` | Returns information about a deployment pattern version. |
| `GetWorkload` | `POST /getWorkload` | `readonly` | `workloadName` | - | `GetWorkloadOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns information about a workload. |
| `GetWorkloadDeploymentPattern` | `POST /getWorkloadDeploymentPattern` | `readonly` | `deploymentPatternName`, `workloadName` | - | `GetWorkloadDeploymentPatternOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns details for a given workload and deployment pattern, including the available specifications. You can use the ListWorkloads operation to discover the available workload names and the ListWorkloadDeploymentPatterns operation to discover the available... |
| `ListDeploymentEvents` | `POST /listDeploymentEvents` | `readonly`, `paginated` | `deploymentId` | - | `ListDeploymentEventsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the events of a deployment. |
| `ListDeploymentPatternVersions` | `POST /listDeploymentPatternVersions` | `readonly`, `paginated` | `deploymentPatternName`, `workloadName` | - | `ListDeploymentPatternVersionsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the deployment pattern versions. |
| `ListDeployments` | `POST /listDeployments` | `readonly`, `paginated` | - | - | `ListDeploymentsOutput` | `InternalServerException`, `ValidationException` | Lists the deployments that have been created. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags associated with a specified resource. |
| `ListWorkloadDeploymentPatterns` | `POST /listWorkloadDeploymentPatterns` | `readonly`, `paginated` | `workloadName` | - | `ListWorkloadDeploymentPatternsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the workload deployment patterns for a given workload name. You can use the ListWorkloads operation to discover the available workload names. |
| `ListWorkloads` | `POST /listWorkloads` | `readonly`, `paginated` | - | - | `ListWorkloadsOutput` | `InternalServerException`, `ValidationException` | Lists the available workload names. You can use the ListWorkloadDeploymentPatterns operation to discover the available deployment patterns for a given workload. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds the specified tags to the given resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified tags from the given resource. |
| `UpdateDeployment` | `POST /updateDeployment` | `idempotent` | `deploymentId`, `specifications` | - | `UpdateDeploymentOutput` | `InternalServerException`, `ResourceLimitException`, `ResourceNotFoundException`, `ValidationException` | Updates a deployment. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An internal error has occurred. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message` | The specified workload or deployment resource can't be found. |
| `ResourceLimitException` | `structure` | `message` | You have exceeded an Launch Wizard resource limit. |
| `CreateDeploymentInput` | `structure` | `deploymentPatternName`, `dryRun`, `name`, `specifications`, `tags`, `workloadName` | - |
| `CreateDeploymentOutput` | `structure` | `deploymentId` | - |
| `DeleteDeploymentInput` | `structure` | `deploymentId` | - |
| `DeleteDeploymentOutput` | `structure` | `status`, `statusReason` | - |
| `GetDeploymentInput` | `structure` | `deploymentId` | - |
| `GetDeploymentOutput` | `structure` | `deployment` | - |
| `GetDeploymentPatternVersionInput` | `structure` | `deploymentPatternName`, `deploymentPatternVersionName`, `workloadName` | - |
| `GetDeploymentPatternVersionOutput` | `structure` | `deploymentPatternVersion` | - |
| `GetWorkloadInput` | `structure` | `workloadName` | - |
| `GetWorkloadOutput` | `structure` | `workload` | - |
| `GetWorkloadDeploymentPatternInput` | `structure` | `deploymentPatternName`, `workloadName` | - |
| `GetWorkloadDeploymentPatternOutput` | `structure` | `workloadDeploymentPattern` | - |
| `ListDeploymentEventsInput` | `structure` | `deploymentId`, `maxResults`, `nextToken` | - |
| `ListDeploymentEventsOutput` | `structure` | `deploymentEvents`, `nextToken` | - |
| `ListDeploymentPatternVersionsInput` | `structure` | `deploymentPatternName`, `filters`, `maxResults`, `nextToken`, `workloadName` | - |
| `ListDeploymentPatternVersionsOutput` | `structure` | `deploymentPatternVersions`, `nextToken` | - |
| `ListDeploymentsInput` | `structure` | `filters`, `maxResults`, `nextToken` | - |
| `ListDeploymentsOutput` | `structure` | `deployments`, `nextToken` | - |
| `ListTagsForResourceInput` | `structure` | `resourceArn` | - |
| `ListTagsForResourceOutput` | `structure` | `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

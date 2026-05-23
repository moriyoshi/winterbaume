# AWS Proton

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Proton Service API Reference. It provides descriptions, syntax and usage examples for each of the actions and data types for the Proton service. The documentation for each action shows the Query API request parameters and the XML response. Alternatively, you can use the Amazon Web Services CLI to access an API. For more information, see the Amazon Web Services Command Line Interface User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Proton by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Proton resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Proton workflows in the local mock. Key resources include `AccountSettingsResource`, `ComponentOutputResource`, `ComponentProvisionedResourceResource`, `ComponentResource`, `DeploymentResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListComponentOutputs`, `ListComponentProvisionedResources`, `ListComponents`, `ListDeployments`, `GetAccountSettings`, `GetComponent`.

## Service Identity and Protocol

- AWS model slug: `proton`
- AWS SDK for Rust slug: `proton`
- Model version: `2020-07-20`
- Model file: `vendor/api-models-aws/models/proton/service/2020-07-20/proton-2020-07-20.json`
- SDK ID: `Proton`
- Endpoint prefix: `proton`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (21), `Get` (19), `Update` (14), `Create` (12), `Delete` (12), `Cancel` (4), `Accept` (1), `Notify` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptEnvironmentAccountConnection`, `CancelComponentDeployment`, `CancelEnvironmentDeployment`, `CancelServiceInstanceDeployment`, `CancelServicePipelineDeployment`, `CreateComponent`, `CreateEnvironment`, `CreateEnvironmentAccountConnection`, `CreateEnvironmentTemplate`, `CreateEnvironmentTemplateVersion`, `CreateRepository`, `CreateService`, `CreateServiceInstance`, `CreateServiceSyncConfig`, `CreateServiceTemplate`, `CreateServiceTemplateVersion`, `CreateTemplateSyncConfig`, `DeleteComponent`, `DeleteDeployment`, `DeleteEnvironment`, ... (+26).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountSettings`, `GetComponent`, `GetDeployment`, `GetEnvironment`, `GetEnvironmentAccountConnection`, `GetEnvironmentTemplate`, `GetEnvironmentTemplateVersion`, `GetRepository`, `GetRepositorySyncStatus`, `GetResourcesSummary`, `GetService`, `GetServiceInstance`, `GetServiceInstanceSyncStatus`, `GetServiceSyncBlockerSummary`, `GetServiceSyncConfig`, `GetServiceTemplate`, `GetServiceTemplateVersion`, `GetTemplateSyncConfig`, `GetTemplateSyncStatus`, `ListComponentOutputs`, ... (+20).
- Pagination is modelled for 21 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 31 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelComponentDeployment`, `CancelEnvironmentDeployment`, `CancelServiceInstanceDeployment`, `CancelServicePipelineDeployment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 87 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccountSettingsResource` | - | read: `GetAccountSettings`; update: `UpdateAccountSettings` | - | - |
| `ComponentOutputResource` | `componentName`, `key` | list: `ListComponentOutputs` | - | - |
| `ComponentProvisionedResourceResource` | `componentName`, `name` | list: `ListComponentProvisionedResources` | - | - |
| `ComponentResource` | `name` | put: `CreateComponent`; read: `GetComponent`; update: `UpdateComponent`; delete: `DeleteComponent`; list: `ListComponents` | - | - |
| `DeploymentResource` | `id` | read: `GetDeployment`; delete: `DeleteDeployment`; list: `ListDeployments` | - | - |
| `EnvironmentAccountConnectionResource` | `id` | create: `CreateEnvironmentAccountConnection`; read: `GetEnvironmentAccountConnection`; update: `UpdateEnvironmentAccountConnection`; delete: `DeleteEnvironmentAccountConnection`; list: `ListEnvironmentAccountConnections` | `AcceptEnvironmentAccountConnection`, `RejectEnvironmentAccountConnection` | - |
| `EnvironmentOutputResource` | `environmentName`, `key` | list: `ListEnvironmentOutputs` | - | - |
| `EnvironmentProvisionedResourceResource` | `environmentName`, `name` | list: `ListEnvironmentProvisionedResources` | - | - |
| `EnvironmentResource` | `name` | put: `CreateEnvironment`; read: `GetEnvironment`; update: `UpdateEnvironment`; delete: `DeleteEnvironment`; list: `ListEnvironments` | - | - |
| `EnvironmentTemplateResource` | `name` | put: `CreateEnvironmentTemplate`; read: `GetEnvironmentTemplate`; update: `UpdateEnvironmentTemplate`; delete: `DeleteEnvironmentTemplate`; list: `ListEnvironmentTemplates` | - | - |
| `EnvironmentTemplateVersionResource` | `majorVersion`, `minorVersion`, `templateName` | create: `CreateEnvironmentTemplateVersion`; read: `GetEnvironmentTemplateVersion`; update: `UpdateEnvironmentTemplateVersion`; delete: `DeleteEnvironmentTemplateVersion`; list: `ListEnvironmentTemplateVersions` | - | - |
| `RepositoryResource` | `name`, `provider` | put: `CreateRepository`; read: `GetRepository`; delete: `DeleteRepository`; list: `ListRepositories` | - | - |
| `ServiceInstanceOutputResource` | `key`, `serviceInstanceName`, `serviceName` | list: `ListServiceInstanceOutputs` | - | - |
| `ServiceInstanceProvisionedResourceResource` | `name`, `serviceInstanceName`, `serviceName` | list: `ListServiceInstanceProvisionedResources` | - | - |
| `ServiceInstanceResource` | `name`, `serviceName` | put: `CreateServiceInstance`; read: `GetServiceInstance`; update: `UpdateServiceInstance`; list: `ListServiceInstances` | - | - |
| `ServicePipelineOutputResource` | `key`, `serviceName` | list: `ListServicePipelineOutputs` | - | - |
| `ServicePipelineProvisionedResourceResource` | `name`, `serviceName` | list: `ListServicePipelineProvisionedResources` | - | - |
| `ServicePipelineResource` | `serviceName` | update: `UpdateServicePipeline` | - | - |
| `ServiceResource` | `name` | put: `CreateService`; read: `GetService`; update: `UpdateService`; delete: `DeleteService`; list: `ListServices` | - | - |
| `ServiceSyncBlockerResource` | - | read: `GetServiceSyncBlockerSummary`; update: `UpdateServiceSyncBlocker` | - | - |
| `ServiceSyncConfigResource` | - | put: `CreateServiceSyncConfig`; read: `GetServiceSyncConfig`; update: `UpdateServiceSyncConfig`; delete: `DeleteServiceSyncConfig` | - | - |
| `ServiceTemplateResource` | `name` | put: `CreateServiceTemplate`; read: `GetServiceTemplate`; update: `UpdateServiceTemplate`; delete: `DeleteServiceTemplate`; list: `ListServiceTemplates` | - | - |
| `ServiceTemplateVersionResource` | `majorVersion`, `minorVersion`, `templateName` | create: `CreateServiceTemplateVersion`; read: `GetServiceTemplateVersion`; update: `UpdateServiceTemplateVersion`; delete: `DeleteServiceTemplateVersion`; list: `ListServiceTemplateVersions` | - | - |
| `TemplateSyncConfigResource` | - | put: `CreateTemplateSyncConfig`; read: `GetTemplateSyncConfig`; update: `UpdateTemplateSyncConfig`; delete: `DeleteTemplateSyncConfig` | - | - |
## Operation Groups

### Cancel

- Operations: `CancelComponentDeployment`, `CancelEnvironmentDeployment`, `CancelServiceInstanceDeployment`, `CancelServicePipelineDeployment`
- Common required input members in this group: `serviceName`

### Get

- Operations: `GetRepositorySyncStatus`, `GetResourcesSummary`, `GetServiceInstanceSyncStatus`, `GetTemplateSyncStatus`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListRepositorySyncDefinitions`, `ListTagsForResource`
- Traits: `readonly` (2), `paginated` (2)
- Common required input members in this group: -

### Notify

- Operations: `NotifyResourceDeploymentStatusChange`
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
| `CancelComponentDeployment` | `-` | - | `componentName` | - | `CancelComponentDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel a component deployment (for a component that is in the IN_PROGRESS deployment status). For more information about components, see Proton components in the Proton User Guide . |
| `CancelEnvironmentDeployment` | `-` | - | `environmentName` | - | `CancelEnvironmentDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel an environment deployment on an UpdateEnvironment action, if the deployment is IN_PROGRESS . For more information, see Update an environment in the Proton User guide . The following list includes p ... |
| `CancelServiceInstanceDeployment` | `-` | - | `serviceInstanceName`, `serviceName` | - | `CancelServiceInstanceDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel a service instance deployment on an UpdateServiceInstance action, if the deployment is IN_PROGRESS . For more information, see Update a service instance in the Proton User guide . The following lis ... |
| `CancelServicePipelineDeployment` | `-` | - | `serviceName` | - | `CancelServicePipelineDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel a service pipeline deployment on an UpdateServicePipeline action, if the deployment is IN_PROGRESS . For more information, see Update a service pipeline in the Proton User guide . The following lis ... |
| `GetRepositorySyncStatus` | `-` | - | `repositoryName`, `repositoryProvider`, `branch`, `syncType` | - | `GetRepositorySyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the sync status of a repository used for Proton template sync. For more information about template sync, see . A repository sync status isn't tied to the Proton Repository resource (or any other Proton resource). ... |
| `GetResourcesSummary` | `-` | `readonly` | - | - | `GetResourcesSummaryOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Get counts of Proton resources. For infrastructure-provisioning resources (environments, services, service instances, pipelines), the action returns staleness counts. A resource is stale when it's behind the recommen ... |
| `GetServiceInstanceSyncStatus` | `-` | - | `serviceName`, `serviceInstanceName` | - | `GetServiceInstanceSyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the status of the synced service instance. |
| `GetTemplateSyncStatus` | `-` | - | `templateName`, `templateType`, `templateVersion` | - | `GetTemplateSyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the status of a template sync. |
| `ListRepositorySyncDefinitions` | `-` | `readonly`, `paginated` | `repositoryName`, `repositoryProvider`, `syncType` | - | `ListRepositorySyncDefinitionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List repository sync definitions with detail data. |
| `ListTagsForResource` | `-` | `readonly`, `paginated` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for a resource. For more information, see Proton resources and tagging in the Proton User Guide . |
| `NotifyResourceDeploymentStatusChange` | `-` | - | `resourceArn` | - | `NotifyResourceDeploymentStatusChangeOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Notify Proton of status changes to a provisioned resource when you use self-managed provisioning. For more information, see Self-managed provisioning in the Proton User Guide . |
| `TagResource` | `-` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tag a resource. A tag is a key-value pair of metadata that you associate with an Proton resource. For more information, see Proton resources and tagging in the Proton User Guide . |
| `UntagResource` | `-` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a customer tag from a resource. A tag is a key-value pair of metadata associated with an Proton resource. For more information, see Proton resources and tagging in the Proton User Guide . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListTagsForResource` | - | `resourceArn -> resourceArn`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `TagResource` | - | `resourceArn -> resourceArn` | - | - |
| `UntagResource` | - | `resourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | There isn't sufficient access for performing this action. |
| `ConflictException` | `structure` | message | The request couldn't be made due to a conflicting operation or resource. |
| `InternalServerException` | `structure` | message | The request failed to register with the service. |
| `ResourceNotFoundException` | `structure` | message | The requested resource wasn't found. |
| `ServiceQuotaExceededException` | `structure` | message | A quota was exceeded. For more information, see Proton Quotas in the Proton User Guide . |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input is invalid or an out-of-range value was supplied for the input parameter. |
| `CancelComponentDeploymentInput` | `structure` | componentName | - |
| `CancelComponentDeploymentOutput` | `structure` | component | - |
| `CancelEnvironmentDeploymentInput` | `structure` | environmentName | - |
| `CancelEnvironmentDeploymentOutput` | `structure` | environment | - |
| `CancelServiceInstanceDeploymentInput` | `structure` | serviceInstanceName, serviceName | - |
| `CancelServiceInstanceDeploymentOutput` | `structure` | serviceInstance | - |
| `CancelServicePipelineDeploymentInput` | `structure` | serviceName | - |
| `CancelServicePipelineDeploymentOutput` | `structure` | pipeline | - |
| `GetRepositorySyncStatusInput` | `structure` | repositoryName, repositoryProvider, branch, syncType | - |
| `GetRepositorySyncStatusOutput` | `structure` | latestSync | - |
| `GetResourcesSummaryInput` | `structure` | **empty (no members)** | - |
| `GetResourcesSummaryOutput` | `structure` | counts | - |
| `GetServiceInstanceSyncStatusInput` | `structure` | serviceName, serviceInstanceName | - |
| `GetServiceInstanceSyncStatusOutput` | `structure` | latestSync, latestSuccessfulSync, desiredState | - |
| `GetTemplateSyncStatusInput` | `structure` | templateName, templateType, templateVersion | - |
| `GetTemplateSyncStatusOutput` | `structure` | latestSync, latestSuccessfulSync, desiredState | - |
| `ListRepositorySyncDefinitionsInput` | `structure` | repositoryName, repositoryProvider, syncType, nextToken | - |
| `ListRepositorySyncDefinitionsOutput` | `structure` | nextToken, syncDefinitions | - |
| `ListTagsForResourceInput` | `structure` | resourceArn, nextToken, maxResults | - |
| `ListTagsForResourceOutput` | `structure` | tags, nextToken | - |
| `NotifyResourceDeploymentStatusChangeInput` | `structure` | resourceArn, status, outputs, deploymentId, statusMessage | - |
| `NotifyResourceDeploymentStatusChangeOutput` | `structure` | **empty (no members)** | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

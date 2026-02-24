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

### List

- Operations: `ListComponentOutputs`, `ListComponentProvisionedResources`, `ListComponents`, `ListDeployments`, `ListEnvironmentAccountConnections`, `ListEnvironmentOutputs`, `ListEnvironmentProvisionedResources`, `ListEnvironmentTemplateVersions`, `ListEnvironmentTemplates`, `ListEnvironments`, `ListRepositories`, `ListRepositorySyncDefinitions`, `ListServiceInstanceOutputs`, `ListServiceInstanceProvisionedResources`, `ListServiceInstances`, `ListServicePipelineOutputs`, `ListServicePipelineProvisionedResources`, `ListServiceTemplateVersions`, `ListServiceTemplates`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (21), `readonly` (21)
- Common required input members in this group: `componentName`, `environmentName`, `repositoryName`, `repositoryProvider`, `requestedBy`, `resourceArn`, `serviceInstanceName`, `serviceName`, `syncType`, `templateName`

### Get

- Operations: `GetAccountSettings`, `GetComponent`, `GetDeployment`, `GetEnvironment`, `GetEnvironmentAccountConnection`, `GetEnvironmentTemplate`, `GetEnvironmentTemplateVersion`, `GetRepository`, `GetRepositorySyncStatus`, `GetResourcesSummary`, `GetService`, `GetServiceInstance`, `GetServiceInstanceSyncStatus`, `GetServiceSyncBlockerSummary`, `GetServiceSyncConfig`, `GetServiceTemplate`, `GetServiceTemplateVersion`, `GetTemplateSyncConfig`, `GetTemplateSyncStatus`
- Traits: `readonly` (16)
- Common required input members in this group: `branch`, `id`, `majorVersion`, `minorVersion`, `name`, `provider`, `repositoryName`, `repositoryProvider`, `serviceInstanceName`, `serviceName`, `syncType`, `templateName`, `templateType`, `templateVersion`

### Update

- Operations: `UpdateAccountSettings`, `UpdateComponent`, `UpdateEnvironment`, `UpdateEnvironmentAccountConnection`, `UpdateEnvironmentTemplate`, `UpdateEnvironmentTemplateVersion`, `UpdateService`, `UpdateServiceInstance`, `UpdateServicePipeline`, `UpdateServiceSyncBlocker`, `UpdateServiceSyncConfig`, `UpdateServiceTemplate`, `UpdateServiceTemplateVersion`, `UpdateTemplateSyncConfig`
- Traits: `idempotency-token` (2), `idempotent` (1)
- Common required input members in this group: `branch`, `deploymentType`, `filePath`, `id`, `majorVersion`, `minorVersion`, `name`, `repositoryName`, `repositoryProvider`, `resolvedReason`, `serviceName`, `spec`, `templateName`, `templateType`

### Create

- Operations: `CreateComponent`, `CreateEnvironment`, `CreateEnvironmentAccountConnection`, `CreateEnvironmentTemplate`, `CreateEnvironmentTemplateVersion`, `CreateRepository`, `CreateService`, `CreateServiceInstance`, `CreateServiceSyncConfig`, `CreateServiceTemplate`, `CreateServiceTemplateVersion`, `CreateTemplateSyncConfig`
- Traits: `idempotency-token` (5), `idempotent` (12)
- Common required input members in this group: `branch`, `compatibleEnvironmentTemplates`, `connectionArn`, `environmentName`, `filePath`, `managementAccountId`, `manifest`, `name`, `provider`, `repositoryName`, `repositoryProvider`, `serviceName`, `source`, `spec`, `templateFile`, `templateMajorVersion`, `templateName`, `templateType`

### Delete

- Operations: `DeleteComponent`, `DeleteDeployment`, `DeleteEnvironment`, `DeleteEnvironmentAccountConnection`, `DeleteEnvironmentTemplate`, `DeleteEnvironmentTemplateVersion`, `DeleteRepository`, `DeleteService`, `DeleteServiceSyncConfig`, `DeleteServiceTemplate`, `DeleteServiceTemplateVersion`, `DeleteTemplateSyncConfig`
- Traits: `idempotent` (12)
- Common required input members in this group: `id`, `majorVersion`, `minorVersion`, `name`, `provider`, `serviceName`, `templateName`, `templateType`

### Cancel

- Operations: `CancelComponentDeployment`, `CancelEnvironmentDeployment`, `CancelServiceInstanceDeployment`, `CancelServicePipelineDeployment`
- Common required input members in this group: `componentName`, `environmentName`, `serviceInstanceName`, `serviceName`

### Accept

- Operations: `AcceptEnvironmentAccountConnection`
- Traits: `idempotent` (1)
- Common required input members in this group: `id`

### Notify

- Operations: `NotifyResourceDeploymentStatusChange`
- Common required input members in this group: `resourceArn`

### Reject

- Operations: `RejectEnvironmentAccountConnection`
- Traits: `idempotent` (1)
- Common required input members in this group: `id`

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
| `AcceptEnvironmentAccountConnection` | - | `idempotent` | `id` | - | `AcceptEnvironmentAccountConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | In a management account, an environment account connection request is accepted. When the environment account connection request is accepted, Proton can use the associated IAM role to provision environment infrastructure resources in the associated environment... |
| `CancelComponentDeployment` | - | - | `componentName` | - | `CancelComponentDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel a component deployment (for a component that is in the `IN_PROGRESS` deployment status). For more information about components, see Proton components in the Proton User Guide . |
| `CancelEnvironmentDeployment` | - | - | `environmentName` | - | `CancelEnvironmentDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel an environment deployment on an UpdateEnvironment action, if the deployment is `IN_PROGRESS`. For more information, see Update an environment in the Proton User guide . |
| `CancelServiceInstanceDeployment` | - | - | `serviceInstanceName`, `serviceName` | - | `CancelServiceInstanceDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel a service instance deployment on an UpdateServiceInstance action, if the deployment is `IN_PROGRESS`. For more information, see Update a service instance in the Proton User guide . |
| `CancelServicePipelineDeployment` | - | - | `serviceName` | - | `CancelServicePipelineDeploymentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attempts to cancel a service pipeline deployment on an UpdateServicePipeline action, if the deployment is `IN_PROGRESS`. For more information, see Update a service pipeline in the Proton User guide . |
| `CreateComponent` | - | `idempotent`, `idempotency-token` | `manifest`, `name`, `templateFile` | `clientToken` | `CreateComponentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create an Proton component. A component is an infrastructure extension for a service instance. |
| `CreateEnvironment` | - | `idempotent` | `name`, `spec`, `templateMajorVersion`, `templateName` | - | `CreateEnvironmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deploy a new environment. An Proton environment is created from an environment template that defines infrastructure and resources that can be shared across services. |
| `CreateEnvironmentAccountConnection` | - | `idempotent`, `idempotency-token` | `environmentName`, `managementAccountId` | `clientToken` | `CreateEnvironmentAccountConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create an environment account connection in an environment account so that environment infrastructure resources can be provisioned in the environment account from a management account. An environment account connection is a secure bi-directional connection... |
| `CreateEnvironmentTemplate` | - | `idempotent` | `name` | - | `CreateEnvironmentTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create an environment template for Proton. For more information, see Environment Templates in the Proton User Guide . |
| `CreateEnvironmentTemplateVersion` | - | `idempotent`, `idempotency-token` | `source`, `templateName` | `clientToken` | `CreateEnvironmentTemplateVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new major or minor version of an environment template. A major version of an environment template is a version that isn't backwards compatible. |
| `CreateRepository` | - | `idempotent` | `connectionArn`, `name`, `provider` | - | `CreateRepositoryOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create and register a link to a repository. Proton uses the link to repeatedly access the repository, to either push to it (self-managed provisioning) or pull from it (template sync). |
| `CreateService` | - | `idempotent` | `name`, `spec`, `templateMajorVersion`, `templateName` | - | `CreateServiceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create an Proton service. An Proton service is an instantiation of a service template and often includes several service instances and pipeline. |
| `CreateServiceInstance` | - | `idempotent`, `idempotency-token` | `name`, `serviceName`, `spec` | `clientToken` | `CreateServiceInstanceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Create a service instance. |
| `CreateServiceSyncConfig` | - | `idempotent` | `branch`, `filePath`, `repositoryName`, `repositoryProvider`, `serviceName` | - | `CreateServiceSyncConfigOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create the Proton Ops configuration file. |
| `CreateServiceTemplate` | - | `idempotent` | `name` | - | `CreateServiceTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a service template. The administrator creates a service template to define standardized infrastructure and an optional CI/CD service pipeline. |
| `CreateServiceTemplateVersion` | - | `idempotent`, `idempotency-token` | `compatibleEnvironmentTemplates`, `source`, `templateName` | `clientToken` | `CreateServiceTemplateVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new major or minor version of a service template. A major version of a service template is a version that isn't backward compatible. |
| `CreateTemplateSyncConfig` | - | `idempotent` | `branch`, `repositoryName`, `repositoryProvider`, `templateName`, `templateType` | - | `CreateTemplateSyncConfigOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Set up a template to create new template versions automatically by tracking a linked repository. A linked repository is a repository that has been registered with Proton. |
| `DeleteComponent` | - | `idempotent` | `name` | - | `DeleteComponentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an Proton component resource. For more information about components, see Proton components in the Proton User Guide . |
| `DeleteDeployment` | - | `idempotent` | `id` | - | `DeleteDeploymentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete the deployment. |
| `DeleteEnvironment` | - | `idempotent` | `name` | - | `DeleteEnvironmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an environment. |
| `DeleteEnvironmentAccountConnection` | - | `idempotent` | `id` | - | `DeleteEnvironmentAccountConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | In an environment account, delete an environment account connection. After you delete an environment account connection that’s in use by an Proton environment, Proton can’t manage the environment infrastructure resources until a new environment account... |
| `DeleteEnvironmentTemplate` | - | `idempotent` | `name` | - | `DeleteEnvironmentTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | If no other major or minor versions of an environment template exist, delete the environment template. |
| `DeleteEnvironmentTemplateVersion` | - | `idempotent` | `majorVersion`, `minorVersion`, `templateName` | - | `DeleteEnvironmentTemplateVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | If no other minor versions of an environment template exist, delete a major version of the environment template if it's not the `Recommended` version. Delete the `Recommended` version of the environment template if no other major versions or minor versions of... |
| `DeleteRepository` | - | `idempotent` | `name`, `provider` | - | `DeleteRepositoryOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | De-register and unlink your repository. |
| `DeleteService` | - | `idempotent` | `name` | - | `DeleteServiceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a service, with its instances and pipeline. You can't delete a service if it has any service instances that have components attached to them. |
| `DeleteServiceSyncConfig` | - | `idempotent` | `serviceName` | - | `DeleteServiceSyncConfigOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete the Proton Ops file. |
| `DeleteServiceTemplate` | - | `idempotent` | `name` | - | `DeleteServiceTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | If no other major or minor versions of the service template exist, delete the service template. |
| `DeleteServiceTemplateVersion` | - | `idempotent` | `majorVersion`, `minorVersion`, `templateName` | - | `DeleteServiceTemplateVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | If no other minor versions of a service template exist, delete a major version of the service template if it's not the `Recommended` version. Delete the `Recommended` version of the service template if no other major versions or minor versions of the service... |
| `DeleteTemplateSyncConfig` | - | `idempotent` | `templateName`, `templateType` | - | `DeleteTemplateSyncConfigOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a template sync configuration. |
| `GetAccountSettings` | - | `readonly` | - | - | `GetAccountSettingsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detail data for Proton account-wide settings. |
| `GetComponent` | - | `readonly` | `name` | - | `GetComponentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a component. For more information about components, see Proton components in the Proton User Guide . |
| `GetDeployment` | - | `readonly` | `id` | - | `GetDeploymentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a deployment. |
| `GetEnvironment` | - | `readonly` | `name` | - | `GetEnvironmentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for an environment. |
| `GetEnvironmentAccountConnection` | - | `readonly` | `id` | - | `GetEnvironmentAccountConnectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | In an environment account, get the detailed data for an environment account connection. For more information, see Environment account connections in the Proton User guide . |
| `GetEnvironmentTemplate` | - | `readonly` | `name` | - | `GetEnvironmentTemplateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for an environment template. |
| `GetEnvironmentTemplateVersion` | - | `readonly` | `majorVersion`, `minorVersion`, `templateName` | - | `GetEnvironmentTemplateVersionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a major or minor version of an environment template. |
| `GetRepository` | - | `readonly` | `name`, `provider` | - | `GetRepositoryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detail data for a linked repository. |
| `GetRepositorySyncStatus` | - | - | `branch`, `repositoryName`, `repositoryProvider`, `syncType` | - | `GetRepositorySyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the sync status of a repository used for Proton template sync. For more information about template sync, see . |
| `GetResourcesSummary` | - | `readonly` | - | - | `GetResourcesSummaryOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Get counts of Proton resources. For infrastructure-provisioning resources (environments, services, service instances, pipelines), the action returns staleness counts. |
| `GetService` | - | `readonly` | `name` | - | `GetServiceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a service. |
| `GetServiceInstance` | - | `readonly` | `name`, `serviceName` | - | `GetServiceInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a service instance. A service instance is an instantiation of service template and it runs in a specific environment. |
| `GetServiceInstanceSyncStatus` | - | - | `serviceInstanceName`, `serviceName` | - | `GetServiceInstanceSyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the status of the synced service instance. |
| `GetServiceSyncBlockerSummary` | - | `readonly` | `serviceName` | - | `GetServiceSyncBlockerSummaryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for the service sync blocker summary. |
| `GetServiceSyncConfig` | - | `readonly` | `serviceName` | - | `GetServiceSyncConfigOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed information for the service sync configuration. |
| `GetServiceTemplate` | - | `readonly` | `name` | - | `GetServiceTemplateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a service template. |
| `GetServiceTemplateVersion` | - | `readonly` | `majorVersion`, `minorVersion`, `templateName` | - | `GetServiceTemplateVersionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detailed data for a major or minor version of a service template. |
| `GetTemplateSyncConfig` | - | `readonly` | `templateName`, `templateType` | - | `GetTemplateSyncConfigOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get detail data for a template sync configuration. |
| `GetTemplateSyncStatus` | - | - | `templateName`, `templateType`, `templateVersion` | - | `GetTemplateSyncStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the status of a template sync. |
| `ListComponentOutputs` | - | `readonly`, `paginated` | `componentName` | - | `ListComponentOutputsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a list of component Infrastructure as Code (IaC) outputs. For more information about components, see Proton components in the Proton User Guide . |
| `ListComponentProvisionedResources` | - | `readonly`, `paginated` | `componentName` | - | `ListComponentProvisionedResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List provisioned resources for a component with details. For more information about components, see Proton components in the Proton User Guide . |
| `ListComponents` | - | `readonly`, `paginated` | - | - | `ListComponentsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List components with summary data. You can filter the result list by environment, service, or a single service instance. |
| `ListDeployments` | - | `readonly`, `paginated` | - | - | `ListDeploymentsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List deployments. You can filter the result list by environment, service, or a single service instance. |
| `ListEnvironmentAccountConnections` | - | `readonly`, `paginated` | `requestedBy` | - | `ListEnvironmentAccountConnectionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | View a list of environment account connections. For more information, see Environment account connections in the Proton User guide . |
| `ListEnvironmentOutputs` | - | `readonly`, `paginated` | `environmentName` | - | `ListEnvironmentOutputsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the infrastructure as code outputs for your environment. |
| `ListEnvironmentProvisionedResources` | - | `readonly`, `paginated` | `environmentName` | - | `ListEnvironmentProvisionedResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the provisioned resources for your environment. |
| `ListEnvironmentTemplateVersions` | - | `readonly`, `paginated` | `templateName` | - | `ListEnvironmentTemplateVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List major or minor versions of an environment template with detail data. |
| `ListEnvironmentTemplates` | - | `readonly`, `paginated` | - | - | `ListEnvironmentTemplatesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List environment templates. |
| `ListEnvironments` | - | `readonly`, `paginated` | - | - | `ListEnvironmentsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List environments with detail data summaries. |
| `ListRepositories` | - | `readonly`, `paginated` | - | - | `ListRepositoriesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List linked repositories with detail data. |
| `ListRepositorySyncDefinitions` | - | `readonly`, `paginated` | `repositoryName`, `repositoryProvider`, `syncType` | - | `ListRepositorySyncDefinitionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List repository sync definitions with detail data. |
| `ListServiceInstanceOutputs` | - | `readonly`, `paginated` | `serviceInstanceName`, `serviceName` | - | `ListServiceInstanceOutputsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a list service of instance Infrastructure as Code (IaC) outputs. |
| `ListServiceInstanceProvisionedResources` | - | `readonly`, `paginated` | `serviceInstanceName`, `serviceName` | - | `ListServiceInstanceProvisionedResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List provisioned resources for a service instance with details. |
| `ListServiceInstances` | - | `readonly`, `paginated` | - | - | `ListServiceInstancesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List service instances with summary data. This action lists service instances of all services in the Amazon Web Services account. |
| `ListServicePipelineOutputs` | - | `readonly`, `paginated` | `serviceName` | - | `ListServicePipelineOutputsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a list of service pipeline Infrastructure as Code (IaC) outputs. |
| `ListServicePipelineProvisionedResources` | - | `readonly`, `paginated` | `serviceName` | - | `ListServicePipelineProvisionedResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List provisioned resources for a service and pipeline with details. |
| `ListServiceTemplateVersions` | - | `readonly`, `paginated` | `templateName` | - | `ListServiceTemplateVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List major or minor versions of a service template with detail data. |
| `ListServiceTemplates` | - | `readonly`, `paginated` | - | - | `ListServiceTemplatesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List service templates with detail data. |
| `ListServices` | - | `readonly`, `paginated` | - | - | `ListServicesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List services with summaries of detail data. |
| `ListTagsForResource` | - | `readonly`, `paginated` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for a resource. For more information, see Proton resources and tagging in the Proton User Guide . |
| `NotifyResourceDeploymentStatusChange` | - | - | `resourceArn` | - | `NotifyResourceDeploymentStatusChangeOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Notify Proton of status changes to a provisioned resource when you use self-managed provisioning. For more information, see Self-managed provisioning in the Proton User Guide . |
| `RejectEnvironmentAccountConnection` | - | `idempotent` | `id` | - | `RejectEnvironmentAccountConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | In a management account, reject an environment account connection from another environment account. After you reject an environment account connection request, you can't accept or use the rejected environment account connection. |
| `TagResource` | - | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tag a resource. A tag is a key-value pair of metadata that you associate with an Proton resource. |
| `UntagResource` | - | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a customer tag from a resource. A tag is a key-value pair of metadata associated with an Proton resource. |
| `UpdateAccountSettings` | - | - | - | - | `UpdateAccountSettingsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update Proton settings that are used for multiple services in the Amazon Web Services account. |
| `UpdateComponent` | - | `idempotency-token` | `deploymentType`, `name` | `clientToken` | `UpdateComponentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update a component. There are a few modes for updating a component. |
| `UpdateEnvironment` | - | - | `deploymentType`, `name` | - | `UpdateEnvironmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an environment. If the environment is associated with an environment account connection, don't update or include the `protonServiceRoleArn` and `provisioningRepository` parameter to update or connect to an environment account connection. |
| `UpdateEnvironmentAccountConnection` | - | `idempotent` | `id` | - | `UpdateEnvironmentAccountConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | In an environment account, update an environment account connection to use a new IAM role. For more information, see Environment account connections in the Proton User guide . |
| `UpdateEnvironmentTemplate` | - | - | `name` | - | `UpdateEnvironmentTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an environment template. |
| `UpdateEnvironmentTemplateVersion` | - | - | `majorVersion`, `minorVersion`, `templateName` | - | `UpdateEnvironmentTemplateVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a major or minor version of an environment template. |
| `UpdateService` | - | - | `name` | - | `UpdateServiceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Edit a service description or use a spec to add and delete service instances. Existing service instances and the service pipeline can't be edited using this API. |
| `UpdateServiceInstance` | - | `idempotency-token` | `deploymentType`, `name`, `serviceName` | `clientToken` | `UpdateServiceInstanceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a service instance. There are a few modes for updating a service instance. |
| `UpdateServicePipeline` | - | - | `deploymentType`, `serviceName`, `spec` | - | `UpdateServicePipelineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the service pipeline. There are four modes for updating a service pipeline. |
| `UpdateServiceSyncBlocker` | - | - | `id`, `resolvedReason` | - | `UpdateServiceSyncBlockerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the service sync blocker by resolving it. |
| `UpdateServiceSyncConfig` | - | - | `branch`, `filePath`, `repositoryName`, `repositoryProvider`, `serviceName` | - | `UpdateServiceSyncConfigOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the Proton Ops config file. |
| `UpdateServiceTemplate` | - | - | `name` | - | `UpdateServiceTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a service template. |
| `UpdateServiceTemplateVersion` | - | - | `majorVersion`, `minorVersion`, `templateName` | - | `UpdateServiceTemplateVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a major or minor version of a service template. |
| `UpdateTemplateSyncConfig` | - | - | `branch`, `repositoryName`, `repositoryProvider`, `templateName`, `templateType` | - | `UpdateTemplateSyncConfigOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update template sync configuration parameters, except for the `templateName` and `templateType`. Repository details (branch, name, and provider) should be of a linked repository. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | There isn't sufficient access for performing this action. |
| `InternalServerException` | `structure` | `message` | The request failed to register with the service. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | The input is invalid or an out-of-range value was supplied for the input parameter. |
| `ResourceNotFoundException` | `structure` | `message` | The requested resource wasn't found. |
| `ConflictException` | `structure` | `message` | The request couldn't be made due to a conflicting operation or resource. |
| `ServiceQuotaExceededException` | `structure` | `message` | A quota was exceeded. |
| `AcceptEnvironmentAccountConnectionInput` | `structure` | `id` | - |
| `AcceptEnvironmentAccountConnectionOutput` | `structure` | `environmentAccountConnection` | - |
| `CancelComponentDeploymentInput` | `structure` | `componentName` | - |
| `CancelComponentDeploymentOutput` | `structure` | `component` | - |
| `CancelEnvironmentDeploymentInput` | `structure` | `environmentName` | - |
| `CancelEnvironmentDeploymentOutput` | `structure` | `environment` | - |
| `CancelServiceInstanceDeploymentInput` | `structure` | `serviceInstanceName`, `serviceName` | - |
| `CancelServiceInstanceDeploymentOutput` | `structure` | `serviceInstance` | - |
| `CancelServicePipelineDeploymentInput` | `structure` | `serviceName` | - |
| `CancelServicePipelineDeploymentOutput` | `structure` | `pipeline` | - |
| `CreateComponentInput` | `structure` | `clientToken`, `description`, `environmentName`, `manifest`, `name`, `serviceInstanceName`, `serviceName`, `serviceSpec`, `tags`, `templateFile` | - |
| `CreateComponentOutput` | `structure` | `component` | - |
| `CreateEnvironmentInput` | `structure` | `codebuildRoleArn`, `componentRoleArn`, `description`, `environmentAccountConnectionId`, `name`, `protonServiceRoleArn`, `provisioningRepository`, `spec`, `tags`, `templateMajorVersion`, `templateMinorVersion`, `templateName` | - |
| `CreateEnvironmentOutput` | `structure` | `environment` | - |
| `CreateEnvironmentAccountConnectionInput` | `structure` | `clientToken`, `codebuildRoleArn`, `componentRoleArn`, `environmentName`, `managementAccountId`, `roleArn`, `tags` | - |
| `CreateEnvironmentAccountConnectionOutput` | `structure` | `environmentAccountConnection` | - |
| `CreateEnvironmentTemplateInput` | `structure` | `description`, `displayName`, `encryptionKey`, `name`, `provisioning`, `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

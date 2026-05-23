# Amazon AppConfig

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AppConfig feature flags and dynamic configurations help software builders quickly and securely adjust application behavior in production environments without full code deployments. AppConfig speeds up software release frequency, improves application resiliency, and helps you address emergent issues more quickly. With feature flags, you can gradually release new capabilities to users and measure the impact of those changes before fully deploying the new capabilities to all users. With operational flags and dynamic configurations, you can update block lists, allow lists, throttling limits, logging verbosity, and perform other operational tuning to quickly respond to issues in production environments. AppConfig is a tool in Amazon Web Services Systems Manager.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-appconfig/tests/scenario_test.rs`: build a configuration deployment pipeline with application, environment, profile, hosted version, deployment strategy, and deployment lifecycle operations.
- Backported from `scenario_test.rs`: associate and remove extensions for AppConfig resources.
- Backported from `scenario_test.rs`: manage multiple hosted configuration versions and retrieve the expected version metadata.
- Scenario insight from EC2: include mutable binding failover for Amazon AppConfig where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon AppConfig by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon AppConfig by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: support progressive configuration rollout, feature-flag governance, validators, deployment monitoring, rollback-oriented state, and extension hooks into external systems.

## Service Identity and Protocol

- AWS model slug: `appconfig`
- AWS SDK for Rust slug: `appconfig`
- Model version: `2019-10-09`
- Model file: `vendor/api-models-aws/models/appconfig/service/2019-10-09/appconfig-2019-10-09.json`
- SDK ID: `AppConfig`
- Endpoint prefix: `appconfig`
- ARN namespace: `appconfig`
- CloudFormation name: `AppConfig`
- CloudTrail event source: `appconfig.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (10), `List` (9), `Create` (7), `Delete` (7), `Update` (7), `Start` (1), `Stop` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApplication`, `CreateConfigurationProfile`, `CreateDeploymentStrategy`, `CreateEnvironment`, `CreateExtension`, `CreateExtensionAssociation`, `CreateHostedConfigurationVersion`, `DeleteApplication`, `DeleteConfigurationProfile`, `DeleteDeploymentStrategy`, `DeleteEnvironment`, `DeleteExtension`, `DeleteExtensionAssociation`, `DeleteHostedConfigurationVersion`, `StartDeployment`, `StopDeployment`, `TagResource`, `UntagResource`, `UpdateAccountSettings`, `UpdateApplication`, ... (+5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountSettings`, `GetApplication`, `GetConfiguration`, `GetConfigurationProfile`, `GetDeployment`, `GetDeploymentStrategy`, `GetEnvironment`, `GetExtension`, `GetExtensionAssociation`, `GetHostedConfigurationVersion`, `ListApplications`, `ListConfigurationProfiles`, `ListDeploymentStrategies`, `ListDeployments`, `ListEnvironments`, `ListExtensionAssociations`, `ListExtensions`, `ListHostedConfigurationVersions`, `ListTagsForResource`, `ValidateConfiguration`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDeployment`, `StopDeployment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 45 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `SNS`, `Lambda`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/appconfig/latest/userguide/what-is-appconfig.html
- https://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-deploying.html
- https://docs.aws.amazon.com/appconfig/2019-10-09/APIReference/API_StartDeployment.html

Research outcomes:
- AppConfig resources are organised as applications, environments, configuration profiles, hosted or external configuration versions, deployment strategies, and deployments.
- Configuration profiles can be feature flags or freeform configurations, and can reference configuration data stored in AppConfig, S3, Parameter Store, or Secrets Manager.
- Validators run when a deployment starts. If validation fails, the deployment rolls back to the previous configuration data.
- StartDeployment requires application, environment, configuration profile, configuration version, and deployment strategy identifiers.
- Hosted configuration deployments can specify a version number or version label. Other configuration sources require the version number.
- Only one configuration can be deployed at a time to a given environment, but different environments can receive deployments concurrently.
- Deploying secrets or customer-managed-key encrypted configuration requires KmsKeyIdentifier and caller permission for `kms:GenerateDataKey`.
- AppConfig monitors distribution and reports deployment status. If distribution fails, AppConfig rolls back the configuration.

Parity implications:
- Model applications, environments, configuration profiles, hosted versions, validators, deployment strategies, deployment state, and rollback target separately.
- StartDeployment should enforce one active deployment per environment, version/label rules, source-specific KMS requirements, validator execution, and asynchronous rollout state.
- Deployment strategies should influence gradual exposure and rollback timing rather than being inert metadata.

## Control-Plane / Data-Plane Coherence

- **Paired with `appconfigdata`.** AppConfig Data ( `winterbaume-appconfigdata` ) reads the configuration that this control plane deploys: `StartConfigurationSession` references a `(application, environment, configuration_profile)` tuple owned here, and `GetLatestConfiguration` should return the active deployment's content for that environment+profile.
- **Current Winterbaume status: divergent.** `winterbaume-appconfigdata` does not depend on `winterbaume-appconfig`; it holds only opaque session tokens and never looks up the deployed configuration. As a result `GetLatestConfiguration` returns an empty payload regardless of what was deployed via `StartDeployment` here.
- **What needs to change:** `winterbaume-appconfigdata` should observe this crate's deployment state ( latest active deployment per environment, including the resolved configuration content and version label ) so the data plane returns what was actually deployed. Cross-crate integration tests that `StartDeployment` here and `GetLatestConfiguration` via the data plane are the right regression guard.

## Operation Groups

### Get

- Operations: `GetAccountSettings`, `GetApplication`, `GetConfiguration`, `GetConfigurationProfile`, `GetDeployment`, `GetDeploymentStrategy`, `GetEnvironment`, `GetExtension`, `GetExtensionAssociation`, `GetHostedConfigurationVersion`
- Common required input members in this group: `Application`, `ApplicationId`, `ClientId`, `Configuration`, `ConfigurationProfileId`, `DeploymentNumber`, `DeploymentStrategyId`, `Environment`, `EnvironmentId`, `ExtensionAssociationId`, `ExtensionIdentifier`, `VersionNumber`

### List

- Operations: `ListApplications`, `ListConfigurationProfiles`, `ListDeploymentStrategies`, `ListDeployments`, `ListEnvironments`, `ListExtensionAssociations`, `ListExtensions`, `ListHostedConfigurationVersions`, `ListTagsForResource`
- Traits: `paginated` (8)
- Common required input members in this group: `ApplicationId`, `ConfigurationProfileId`, `EnvironmentId`, `ResourceArn`

### Create

- Operations: `CreateApplication`, `CreateConfigurationProfile`, `CreateDeploymentStrategy`, `CreateEnvironment`, `CreateExtension`, `CreateExtensionAssociation`, `CreateHostedConfigurationVersion`
- Common required input members in this group: `Actions`, `ApplicationId`, `ConfigurationProfileId`, `Content`, `ContentType`, `DeploymentDurationInMinutes`, `ExtensionIdentifier`, `GrowthFactor`, `LocationUri`, `Name`, `ResourceIdentifier`

### Delete

- Operations: `DeleteApplication`, `DeleteConfigurationProfile`, `DeleteDeploymentStrategy`, `DeleteEnvironment`, `DeleteExtension`, `DeleteExtensionAssociation`, `DeleteHostedConfigurationVersion`
- Common required input members in this group: `ApplicationId`, `ConfigurationProfileId`, `DeploymentStrategyId`, `EnvironmentId`, `ExtensionAssociationId`, `ExtensionIdentifier`, `VersionNumber`

### Update

- Operations: `UpdateAccountSettings`, `UpdateApplication`, `UpdateConfigurationProfile`, `UpdateDeploymentStrategy`, `UpdateEnvironment`, `UpdateExtension`, `UpdateExtensionAssociation`
- Common required input members in this group: `ApplicationId`, `ConfigurationProfileId`, `DeploymentStrategyId`, `EnvironmentId`, `ExtensionAssociationId`, `ExtensionIdentifier`

### Start

- Operations: `StartDeployment`
- Common required input members in this group: `ApplicationId`, `ConfigurationProfileId`, `ConfigurationVersion`, `DeploymentStrategyId`, `EnvironmentId`

### Stop

- Operations: `StopDeployment`
- Common required input members in this group: `ApplicationId`, `DeploymentNumber`, `EnvironmentId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Validate

- Operations: `ValidateConfiguration`
- Common required input members in this group: `ApplicationId`, `ConfigurationProfileId`, `ConfigurationVersion`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApplication` | `POST /applications` | - | `Name` | - | `Application` | `BadRequestException`, `InternalServerException`, `ServiceQuotaExceededException` | Creates an application. In AppConfig, an application is simply an organizational construct like a folder. |
| `CreateConfigurationProfile` | `POST /applications/{ApplicationId}/configurationprofiles` | - | `ApplicationId`, `LocationUri`, `Name` | - | `ConfigurationProfile` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates a configuration profile, which is information that enables AppConfig to access the configuration source. Valid configuration sources include the following: Configuration data in YAML, JSON, and other formats stored in the AppConfig hosted... |
| `CreateDeploymentStrategy` | `POST /deploymentstrategies` | - | `DeploymentDurationInMinutes`, `GrowthFactor`, `Name` | - | `DeploymentStrategy` | `BadRequestException`, `InternalServerException`, `ServiceQuotaExceededException` | Creates a deployment strategy that defines important criteria for rolling out your configuration to the designated targets. A deployment strategy includes the overall duration required, a percentage of targets to receive the deployment during each interval... |
| `CreateEnvironment` | `POST /applications/{ApplicationId}/environments` | - | `ApplicationId`, `Name` | - | `Environment` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates an environment. For each application, you define one or more environments. |
| `CreateExtension` | `POST /extensions` | - | `Actions`, `Name` | - | `Extension` | `BadRequestException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException` | Creates an AppConfig extension. An extension augments your ability to inject logic or behavior at different points during the AppConfig workflow of creating or deploying a configuration. |
| `CreateExtensionAssociation` | `POST /extensionassociations` | - | `ExtensionIdentifier`, `ResourceIdentifier` | - | `ExtensionAssociation` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | When you create an extension or configure an Amazon Web Services authored extension, you associate the extension with an AppConfig application, environment, or configuration profile. For example, you can choose to run the `AppConfig deployment events to... |
| `CreateHostedConfigurationVersion` | `POST /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}/hostedconfigurationversions` | - | `ApplicationId`, `ConfigurationProfileId`, `Content`, `ContentType` | - | `HostedConfigurationVersion` | `BadRequestException`, `ConflictException`, `InternalServerException`, `PayloadTooLargeException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates a new configuration in the AppConfig hosted configuration store. If you're creating a feature flag, we recommend you familiarize yourself with the JSON schema for feature flag data. |
| `DeleteApplication` | `DELETE /applications/{ApplicationId}` | - | `ApplicationId` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Deletes an application. |
| `DeleteConfigurationProfile` | `DELETE /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}` | - | `ApplicationId`, `ConfigurationProfileId` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Deletes a configuration profile. To prevent users from unintentionally deleting actively-used configuration profiles, enable deletion protection. |
| `DeleteDeploymentStrategy` | `DELETE /deployementstrategies/{DeploymentStrategyId}` | - | `DeploymentStrategyId` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Deletes a deployment strategy. |
| `DeleteEnvironment` | `DELETE /applications/{ApplicationId}/environments/{EnvironmentId}` | - | `ApplicationId`, `EnvironmentId` | - | `Unit` | `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Deletes an environment. To prevent users from unintentionally deleting actively-used environments, enable deletion protection. |
| `DeleteExtension` | `DELETE /extensions/{ExtensionIdentifier}` | - | `ExtensionIdentifier` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Deletes an AppConfig extension. You must delete all associations to an extension before you delete the extension. |
| `DeleteExtensionAssociation` | `DELETE /extensionassociations/{ExtensionAssociationId}` | - | `ExtensionAssociationId` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Deletes an extension association. This action doesn't delete extensions defined in the association. |
| `DeleteHostedConfigurationVersion` | `DELETE /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}/hostedconfigurationversions/{VersionNumber}` | - | `ApplicationId`, `ConfigurationProfileId`, `VersionNumber` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Deletes a version of a configuration from the AppConfig hosted configuration store. |
| `GetAccountSettings` | `GET /settings` | - | - | - | `AccountSettings` | `BadRequestException`, `InternalServerException` | Returns information about the status of the `DeletionProtection` parameter. |
| `GetApplication` | `GET /applications/{ApplicationId}` | - | `ApplicationId` | - | `Application` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves information about an application. |
| `GetConfiguration` | `GET /applications/{Application}/environments/{Environment}/configurations/{Configuration}` | - | `Application`, `ClientId`, `Configuration`, `Environment` | - | `Configuration` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | (Deprecated) Retrieves the latest deployed configuration. Note the following important information. |
| `GetConfigurationProfile` | `GET /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}` | - | `ApplicationId`, `ConfigurationProfileId` | - | `ConfigurationProfile` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves information about a configuration profile. |
| `GetDeployment` | `GET /applications/{ApplicationId}/environments/{EnvironmentId}/deployments/{DeploymentNumber}` | - | `ApplicationId`, `DeploymentNumber`, `EnvironmentId` | - | `Deployment` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves information about a configuration deployment. |
| `GetDeploymentStrategy` | `GET /deploymentstrategies/{DeploymentStrategyId}` | - | `DeploymentStrategyId` | - | `DeploymentStrategy` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves information about a deployment strategy. A deployment strategy defines important criteria for rolling out your configuration to the designated targets. |
| `GetEnvironment` | `GET /applications/{ApplicationId}/environments/{EnvironmentId}` | - | `ApplicationId`, `EnvironmentId` | - | `Environment` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves information about an environment. An environment is a deployment group of AppConfig applications, such as applications in a `Production` environment or in an `EU_Region` environment. |
| `GetExtension` | `GET /extensions/{ExtensionIdentifier}` | - | `ExtensionIdentifier` | - | `Extension` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Returns information about an AppConfig extension. |
| `GetExtensionAssociation` | `GET /extensionassociations/{ExtensionAssociationId}` | - | `ExtensionAssociationId` | - | `ExtensionAssociation` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Returns information about an AppConfig extension association. For more information about extensions and associations, see Extending workflows in the AppConfig User Guide . |
| `GetHostedConfigurationVersion` | `GET /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}/hostedconfigurationversions/{VersionNumber}` | - | `ApplicationId`, `ConfigurationProfileId`, `VersionNumber` | - | `HostedConfigurationVersion` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves information about a specific configuration version. |
| `ListApplications` | `GET /applications` | `paginated` | - | - | `Applications` | `BadRequestException`, `InternalServerException` | Lists all applications in your Amazon Web Services account. |
| `ListConfigurationProfiles` | `GET /applications/{ApplicationId}/configurationprofiles` | `paginated` | `ApplicationId` | - | `ConfigurationProfiles` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Lists the configuration profiles for an application. |
| `ListDeploymentStrategies` | `GET /deploymentstrategies` | `paginated` | - | - | `DeploymentStrategies` | `BadRequestException`, `InternalServerException` | Lists deployment strategies. |
| `ListDeployments` | `GET /applications/{ApplicationId}/environments/{EnvironmentId}/deployments` | `paginated` | `ApplicationId`, `EnvironmentId` | - | `Deployments` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Lists the deployments for an environment in descending deployment number order. |
| `ListEnvironments` | `GET /applications/{ApplicationId}/environments` | `paginated` | `ApplicationId` | - | `Environments` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Lists the environments for an application. |
| `ListExtensionAssociations` | `GET /extensionassociations` | `paginated` | - | - | `ExtensionAssociations` | `BadRequestException`, `InternalServerException` | Lists all AppConfig extension associations in the account. For more information about extensions and associations, see Extending workflows in the AppConfig User Guide . |
| `ListExtensions` | `GET /extensions` | `paginated` | - | - | `Extensions` | `BadRequestException`, `InternalServerException` | Lists all custom and Amazon Web Services authored AppConfig extensions in the account. For more information about extensions, see Extending workflows in the AppConfig User Guide . |
| `ListHostedConfigurationVersions` | `GET /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}/hostedconfigurationversions` | `paginated` | `ApplicationId`, `ConfigurationProfileId` | - | `HostedConfigurationVersions` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Lists configurations stored in the AppConfig hosted configuration store by version. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ResourceTags` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Retrieves the list of key-value tags assigned to the resource. |
| `StartDeployment` | `POST /applications/{ApplicationId}/environments/{EnvironmentId}/deployments` | - | `ApplicationId`, `ConfigurationProfileId`, `ConfigurationVersion`, `DeploymentStrategyId`, `EnvironmentId` | - | `Deployment` | `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Starts a deployment. |
| `StopDeployment` | `DELETE /applications/{ApplicationId}/environments/{EnvironmentId}/deployments/{DeploymentNumber}` | - | `ApplicationId`, `DeploymentNumber`, `EnvironmentId` | - | `Deployment` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Stops a deployment. This API action works only on deployments that have a status of `DEPLOYING`, unless an `AllowRevert` parameter is supplied. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Assigns metadata to an AppConfig resource. Tags help organize and categorize your AppConfig resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Deletes a tag key and value from an AppConfig resource. |
| `UpdateAccountSettings` | `PATCH /settings` | - | - | - | `AccountSettings` | `BadRequestException`, `InternalServerException` | Updates the value of the `DeletionProtection` parameter. |
| `UpdateApplication` | `PATCH /applications/{ApplicationId}` | - | `ApplicationId` | - | `Application` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Updates an application. |
| `UpdateConfigurationProfile` | `PATCH /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}` | - | `ApplicationId`, `ConfigurationProfileId` | - | `ConfigurationProfile` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Updates a configuration profile. |
| `UpdateDeploymentStrategy` | `PATCH /deploymentstrategies/{DeploymentStrategyId}` | - | `DeploymentStrategyId` | - | `DeploymentStrategy` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Updates a deployment strategy. |
| `UpdateEnvironment` | `PATCH /applications/{ApplicationId}/environments/{EnvironmentId}` | - | `ApplicationId`, `EnvironmentId` | - | `Environment` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Updates an environment. |
| `UpdateExtension` | `PATCH /extensions/{ExtensionIdentifier}` | - | `ExtensionIdentifier` | - | `Extension` | `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Updates an AppConfig extension. For more information about extensions, see Extending workflows in the AppConfig User Guide . |
| `UpdateExtensionAssociation` | `PATCH /extensionassociations/{ExtensionAssociationId}` | - | `ExtensionAssociationId` | - | `ExtensionAssociation` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Updates an association. For more information about extensions and associations, see Extending workflows in the AppConfig User Guide . |
| `ValidateConfiguration` | `POST /applications/{ApplicationId}/configurationprofiles/{ConfigurationProfileId}/validators` | - | `ApplicationId`, `ConfigurationProfileId`, `ConfigurationVersion` | - | `Unit` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException` | Uses the validators in a configuration profile to validate a configuration. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateExtension` | `LatestVersionNumber -> Latest-Version-Number` | - | - | - |
| `CreateHostedConfigurationVersion` | `Description -> Description`, `ContentType -> Content-Type`, `LatestVersionNumber -> Latest-Version-Number`, `VersionLabel -> VersionLabel` | - | - | `Content` |
| `DeleteConfigurationProfile` | `DeletionProtectionCheck -> x-amzn-deletion-protection-check` | - | - | - |
| `DeleteEnvironment` | `DeletionProtectionCheck -> x-amzn-deletion-protection-check` | - | - | - |
| `DeleteExtension` | - | `VersionNumber -> version` | - | - |
| `GetConfiguration` | - | `ClientId -> client_id`, `ClientConfigurationVersion -> client_configuration_version` | - | - |
| `GetExtension` | - | `VersionNumber -> version_number` | - | - |
| `ListApplications` | - | `MaxResults -> max_results`, `NextToken -> next_token` | - | - |
| `ListConfigurationProfiles` | - | `MaxResults -> max_results`, `NextToken -> next_token`, `Type -> type` | - | - |
| `ListDeployments` | - | `MaxResults -> max_results`, `NextToken -> next_token` | - | - |
| `ListDeploymentStrategies` | - | `MaxResults -> max_results`, `NextToken -> next_token` | - | - |
| `ListEnvironments` | - | `MaxResults -> max_results`, `NextToken -> next_token` | - | - |
| `ListExtensionAssociations` | - | `ResourceIdentifier -> resource_identifier`, `ExtensionIdentifier -> extension_identifier`, `ExtensionVersionNumber -> extension_version_number`, `MaxResults -> max_results`, `NextToken -> next_token` | - | - |
| `ListExtensions` | - | `MaxResults -> max_results`, `NextToken -> next_token`, `Name -> name` | - | - |
| `ListHostedConfigurationVersions` | - | `MaxResults -> max_results`, `NextToken -> next_token`, `VersionLabel -> version_label` | - | - |
| `StopDeployment` | `AllowRevert -> Allow-Revert` | - | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |
| `ValidateConfiguration` | - | `ConfigurationVersion -> configuration_version` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Details`, `Message`, `Reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `InternalServerException` | `structure` | `Message` | There was an internal failure in the AppConfig service. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceName` | The requested resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The number of one more AppConfig resources exceeds the maximum allowed. |
| `ConflictException` | `structure` | `Message` | The request could not be processed because of conflict in the current state of the resource. |
| `Application` | `structure` | `Description`, `Id`, `Name` | - |
| `ConfigurationProfile` | `structure` | `ApplicationId`, `Description`, `Id`, `KmsKeyArn`, `KmsKeyIdentifier`, `LocationUri`, `Name`, `RetrievalRoleArn`, `Type`, `Validators` | - |
| `DeploymentStrategy` | `structure` | `DeploymentDurationInMinutes`, `Description`, `FinalBakeTimeInMinutes`, `GrowthFactor`, `GrowthType`, `Id`, `Name`, `ReplicateTo` | - |
| `Environment` | `structure` | `ApplicationId`, `Description`, `Id`, `Monitors`, `Name`, `State` | - |
| `Extension` | `structure` | `Actions`, `Arn`, `Description`, `Id`, `Name`, `Parameters`, `VersionNumber` | - |
| `ExtensionAssociation` | `structure` | `Arn`, `ExtensionArn`, `ExtensionVersionNumber`, `Id`, `Parameters`, `ResourceArn` | - |
| `Deployment` | `structure` | `ApplicationId`, `AppliedExtensions`, `CompletedAt`, `ConfigurationLocationUri`, `ConfigurationName`, `ConfigurationProfileId`, `ConfigurationVersion`, `DeploymentDurationInMinutes`, `DeploymentNumber`, `DeploymentStrategyId`, `Description`, `EnvironmentId`, ... (+10) | - |
| `HostedConfigurationVersion` | `structure` | `ApplicationId`, `ConfigurationProfileId`, `Content`, `ContentType`, `Description`, `KmsKeyArn`, `VersionLabel`, `VersionNumber` | - |
| `AccountSettings` | `structure` | `DeletionProtection` | - |
| `CreateApplicationRequest` | `structure` | `Description`, `Name`, `Tags` | - |
| `CreateConfigurationProfileRequest` | `structure` | `ApplicationId`, `Description`, `KmsKeyIdentifier`, `LocationUri`, `Name`, `RetrievalRoleArn`, `Tags`, `Type`, `Validators` | - |
| `CreateDeploymentStrategyRequest` | `structure` | `DeploymentDurationInMinutes`, `Description`, `FinalBakeTimeInMinutes`, `GrowthFactor`, `GrowthType`, `Name`, `ReplicateTo`, `Tags` | - |
| `CreateEnvironmentRequest` | `structure` | `ApplicationId`, `Description`, `Monitors`, `Name`, `Tags` | - |
| `CreateExtensionRequest` | `structure` | `Actions`, `Description`, `LatestVersionNumber`, `Name`, `Parameters`, `Tags` | - |
| `CreateExtensionAssociationRequest` | `structure` | `ExtensionIdentifier`, `ExtensionVersionNumber`, `Parameters`, `ResourceIdentifier`, `Tags` | - |
| `CreateHostedConfigurationVersionRequest` | `structure` | `ApplicationId`, `ConfigurationProfileId`, `Content`, `ContentType`, `Description`, `LatestVersionNumber`, `VersionLabel` | - |
| `PayloadTooLargeException` | `structure` | `Limit`, `Measure`, `Message`, `Size` | The configuration size is too large. |
| `DeleteApplicationRequest` | `structure` | `ApplicationId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

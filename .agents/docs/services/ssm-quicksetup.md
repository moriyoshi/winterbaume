# AWS Systems Manager QuickSetup

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Quick Setup helps you quickly configure frequently used services and features with recommended best practices. Quick Setup simplifies setting up services, including Systems Manager, by automating common or recommended tasks.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-ssmquicksetup/tests/scenario_test.rs`: run the configuration manager full lifecycle, including create, describe/list, update, status inspection, and deletion.
- From the AWS documentation and model: support Systems Manager Quick Setup configuration managers, configuration definitions, deployment status, target accounts/regions, and operational setup automation.

## Service Identity and Protocol

- AWS model slug: `ssm-quicksetup`
- AWS SDK for Rust slug: `ssmquicksetup`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/ssm-quicksetup/service/2018-05-10/ssm-quicksetup-2018-05-10.json`
- SDK ID: `SSM QuickSetup`
- Endpoint prefix: `-`
- ARN namespace: `ssm-quicksetup`
- CloudFormation name: `-`
- CloudTrail event source: `ssm-quicksetup.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Update` (3), `Create` (1), `Delete` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConfigurationManager`, `DeleteConfigurationManager`, `TagResource`, `UntagResource`, `UpdateConfigurationDefinition`, `UpdateConfigurationManager`, `UpdateServiceSettings`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetConfiguration`, `GetConfigurationManager`, `GetServiceSettings`, `ListConfigurationManagers`, `ListConfigurations`, `ListQuickSetupTypes`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## Operation Groups

### List

- Operations: `ListConfigurationManagers`, `ListConfigurations`, `ListQuickSetupTypes`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `ResourceArn`

### Get

- Operations: `GetConfiguration`, `GetConfigurationManager`, `GetServiceSettings`
- Traits: `readonly` (3)
- Common required input members in this group: `ConfigurationId`, `ManagerArn`

### Update

- Operations: `UpdateConfigurationDefinition`, `UpdateConfigurationManager`, `UpdateServiceSettings`
- Traits: `idempotent` (3)
- Common required input members in this group: `Id`, `ManagerArn`

### Create

- Operations: `CreateConfigurationManager`
- Common required input members in this group: `ConfigurationDefinitions`

### Delete

- Operations: `DeleteConfigurationManager`
- Traits: `idempotent` (1)
- Common required input members in this group: `ManagerArn`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateConfigurationManager` | `POST /configurationManager` | - | `ConfigurationDefinitions` | - | `CreateConfigurationManagerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a Quick Setup configuration manager resource. This object is a collection of desired state configurations for multiple configuration definitions and summaries describing the deployments of those definitions. |
| `DeleteConfigurationManager` | `DELETE /configurationManager/{ManagerArn}` | `idempotent` | `ManagerArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a configuration manager. |
| `GetConfiguration` | `GET /getConfiguration/{ConfigurationId}` | `readonly` | `ConfigurationId` | - | `GetConfigurationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about the specified configuration. |
| `GetConfigurationManager` | `GET /configurationManager/{ManagerArn}` | `readonly` | `ManagerArn` | - | `GetConfigurationManagerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a configuration manager. |
| `GetServiceSettings` | `GET /serviceSettings` | `readonly` | - | - | `GetServiceSettingsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException` | Returns settings configured for Quick Setup in the requesting Amazon Web Services account and Amazon Web Services Region. |
| `ListConfigurationManagers` | `POST /listConfigurationManagers` | `paginated` | - | - | `ListConfigurationManagersOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns Quick Setup configuration managers. |
| `ListConfigurations` | `POST /listConfigurations` | `readonly`, `paginated` | - | - | `ListConfigurationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns configurations deployed by Quick Setup in the requesting Amazon Web Services account and Amazon Web Services Region. |
| `ListQuickSetupTypes` | `GET /listQuickSetupTypes` | `readonly` | - | - | `ListQuickSetupTypesOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException` | Returns the available Quick Setup types. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns tags assigned to the resource. |
| `TagResource` | `PUT /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns key-value pairs of metadata to Amazon Web Services resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from the specified resource. |
| `UpdateConfigurationDefinition` | `PUT /configurationDefinition/{ManagerArn}/{Id}` | `idempotent` | `Id`, `ManagerArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a Quick Setup configuration definition. |
| `UpdateConfigurationManager` | `PUT /configurationManager/{ManagerArn}` | `idempotent` | `ManagerArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a Quick Setup configuration manager. |
| `UpdateServiceSettings` | `PUT /serviceSettings` | `idempotent` | - | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates settings configured for Quick Setup. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | The requester has insufficient permissions to perform the operation. |
| `InternalServerException` | `structure` | `Message` | An error occurred on the server side. |
| `ThrottlingException` | `structure` | `Message` | The request or operation exceeds the maximum allowed request rate per Amazon Web Services account and Amazon Web Services Region. |
| `ConflictException` | `structure` | `Message` | Another request is being processed. |
| `ValidationException` | `structure` | `Message` | The request is invalid. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource couldn't be found. |
| `CreateConfigurationManagerInput` | `structure` | `ConfigurationDefinitions`, `Description`, `Name`, `Tags` | - |
| `CreateConfigurationManagerOutput` | `structure` | `ManagerArn` | - |
| `DeleteConfigurationManagerInput` | `structure` | `ManagerArn` | - |
| `GetConfigurationInput` | `structure` | `ConfigurationId` | - |
| `GetConfigurationOutput` | `structure` | `Account`, `ConfigurationDefinitionId`, `CreatedAt`, `Id`, `LastModifiedAt`, `ManagerArn`, `Parameters`, `Region`, `StatusSummaries`, `Type`, `TypeVersion` | - |
| `GetConfigurationManagerInput` | `structure` | `ManagerArn` | - |
| `GetConfigurationManagerOutput` | `structure` | `ConfigurationDefinitions`, `CreatedAt`, `Description`, `LastModifiedAt`, `ManagerArn`, `Name`, `StatusSummaries`, `Tags` | - |
| `GetServiceSettingsOutput` | `structure` | `ServiceSettings` | - |
| `ListConfigurationManagersInput` | `structure` | `Filters`, `MaxItems`, `StartingToken` | - |
| `ListConfigurationManagersOutput` | `structure` | `ConfigurationManagersList`, `NextToken` | - |
| `ListConfigurationsInput` | `structure` | `ConfigurationDefinitionId`, `Filters`, `ManagerArn`, `MaxItems`, `StartingToken` | - |
| `ListConfigurationsOutput` | `structure` | `ConfigurationsList`, `NextToken` | - |
| `ListQuickSetupTypesOutput` | `structure` | `QuickSetupTypeList` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `Tags` | - |
| `TagResourceInput` | `structure` | `ResourceArn`, `Tags` | - |
| `UntagResourceInput` | `structure` | `ResourceArn`, `TagKeys` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

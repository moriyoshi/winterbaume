# CloudWatch Observability Admin Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use Amazon CloudWatch Observability Admin to discover and understand the state of telemetry configuration in CloudWatch for your Amazon Web Services Organization or account. This simplifies the process of auditing your telemetry collection configurations across multiple resource types within your Amazon Web Services Organization or account. By providing a consolidated view, it allows you to easily review and manage telemetry settings, helping you ensure proper monitoring and data collection across your Amazon Web Services environment. For more information, see Auditing CloudWatch telemetry conﬁgurations in the CloudWatch User Guide. For information on the permissions you need to use this API, see Identity and access management for Amazon CloudWatch in the CloudWatch User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for CloudWatch Observability Admin Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented CloudWatch Observability Admin Service workflows in the local mock. Key resources include `TelemetryPipelineResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetCentralizationRuleForOrganization`, `GetS3TableIntegration`, `GetTelemetryEnrichmentStatus`, `GetTelemetryEvaluationStatus`, `ListCentralizationRulesForOrganization`, `ListResourceTelemetry`.

## Service Identity and Protocol

- AWS model slug: `observabilityadmin`
- AWS SDK for Rust slug: `observabilityadmin`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/observabilityadmin/service/2018-05-10/observabilityadmin-2018-05-10.json`
- SDK ID: `ObservabilityAdmin`
- Endpoint prefix: `-`
- ARN namespace: `observabilityadmin`
- CloudFormation name: `ObservabilityAdmin`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (8), `List` (8), `Create` (5), `Delete` (5), `Update` (4), `Start` (3), `Stop` (3), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCentralizationRuleForOrganization`, `CreateS3TableIntegration`, `CreateTelemetryPipeline`, `CreateTelemetryRule`, `CreateTelemetryRuleForOrganization`, `DeleteCentralizationRuleForOrganization`, `DeleteS3TableIntegration`, `DeleteTelemetryPipeline`, `DeleteTelemetryRule`, `DeleteTelemetryRuleForOrganization`, `StartTelemetryEnrichment`, `StartTelemetryEvaluation`, `StartTelemetryEvaluationForOrganization`, `StopTelemetryEnrichment`, `StopTelemetryEvaluation`, `StopTelemetryEvaluationForOrganization`, `TagResource`, `UntagResource`, `UpdateCentralizationRuleForOrganization`, `UpdateTelemetryPipeline`, ... (+2).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCentralizationRuleForOrganization`, `GetS3TableIntegration`, `GetTelemetryEnrichmentStatus`, `GetTelemetryEvaluationStatus`, `GetTelemetryEvaluationStatusForOrganization`, `GetTelemetryPipeline`, `GetTelemetryRule`, `GetTelemetryRuleForOrganization`, `ListCentralizationRulesForOrganization`, `ListResourceTelemetry`, `ListResourceTelemetryForOrganization`, `ListS3TableIntegrations`, `ListTagsForResource`, `ListTelemetryPipelines`, `ListTelemetryRules`, `ListTelemetryRulesForOrganization`, `ValidateTelemetryPipelineConfiguration`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartTelemetryEnrichment`, `StartTelemetryEvaluation`, `StartTelemetryEvaluationForOrganization`, `StopTelemetryEnrichment`, `StopTelemetryEvaluation`, `StopTelemetryEvaluationForOrganization`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 40 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECS`, `EKS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `TelemetryPipelineResource` | `PipelineIdentifier` | create: `CreateTelemetryPipeline`; read: `GetTelemetryPipeline`; update: `UpdateTelemetryPipeline`; delete: `DeleteTelemetryPipeline`; list: `ListTelemetryPipelines` | - | - |
## Operation Groups

### Get

- Operations: `GetCentralizationRuleForOrganization`, `GetS3TableIntegration`, `GetTelemetryEnrichmentStatus`, `GetTelemetryEvaluationStatus`, `GetTelemetryEvaluationStatusForOrganization`, `GetTelemetryPipeline`, `GetTelemetryRule`, `GetTelemetryRuleForOrganization`
- Traits: `readonly` (8)
- Common required input members in this group: `Arn`, `PipelineIdentifier`, `RuleIdentifier`

### List

- Operations: `ListCentralizationRulesForOrganization`, `ListResourceTelemetry`, `ListResourceTelemetryForOrganization`, `ListS3TableIntegrations`, `ListTagsForResource`, `ListTelemetryPipelines`, `ListTelemetryRules`, `ListTelemetryRulesForOrganization`
- Traits: `paginated` (7), `readonly` (8)
- Common required input members in this group: `ResourceARN`

### Create

- Operations: `CreateCentralizationRuleForOrganization`, `CreateS3TableIntegration`, `CreateTelemetryPipeline`, `CreateTelemetryRule`, `CreateTelemetryRuleForOrganization`
- Common required input members in this group: `Configuration`, `Encryption`, `Name`, `RoleArn`, `Rule`, `RuleName`

### Delete

- Operations: `DeleteCentralizationRuleForOrganization`, `DeleteS3TableIntegration`, `DeleteTelemetryPipeline`, `DeleteTelemetryRule`, `DeleteTelemetryRuleForOrganization`
- Traits: `idempotent` (1)
- Common required input members in this group: `Arn`, `PipelineIdentifier`, `RuleIdentifier`

### Update

- Operations: `UpdateCentralizationRuleForOrganization`, `UpdateTelemetryPipeline`, `UpdateTelemetryRule`, `UpdateTelemetryRuleForOrganization`
- Common required input members in this group: `Configuration`, `PipelineIdentifier`, `Rule`, `RuleIdentifier`

### Start

- Operations: `StartTelemetryEnrichment`, `StartTelemetryEvaluation`, `StartTelemetryEvaluationForOrganization`

### Stop

- Operations: `StopTelemetryEnrichment`, `StopTelemetryEvaluation`, `StopTelemetryEvaluationForOrganization`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Test

- Operations: `TestTelemetryPipeline`
- Common required input members in this group: `Configuration`, `Records`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

### Validate

- Operations: `ValidateTelemetryPipelineConfiguration`
- Common required input members in this group: `Configuration`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCentralizationRuleForOrganization` | `POST /CreateCentralizationRuleForOrganization` | - | `Rule`, `RuleName` | - | `CreateCentralizationRuleForOrganizationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Creates a centralization rule that applies across an Amazon Web Services Organization. This operation can only be called by the organization's management account or a delegated administrator account. |
| `CreateS3TableIntegration` | `POST /CreateS3TableIntegration` | - | `Encryption`, `RoleArn` | - | `CreateS3TableIntegrationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Creates an integration between CloudWatch and S3 Tables for analytics. This integration enables querying CloudWatch telemetry data using analytics engines like Amazon Athena, Amazon Redshift, and Apache Spark. |
| `CreateTelemetryPipeline` | `POST /CreateTelemetryPipeline` | - | `Configuration`, `Name` | - | `CreateTelemetryPipelineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Creates a telemetry pipeline for processing and transforming telemetry data. The pipeline defines how data flows from sources through processors to destinations, enabling data transformation and delivering capabilities. |
| `CreateTelemetryRule` | `POST /CreateTelemetryRule` | - | `Rule`, `RuleName` | - | `CreateTelemetryRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Creates a telemetry rule that defines how telemetry should be configured for Amazon Web Services resources in your account. The rule specifies which resources should have telemetry enabled and how that telemetry data should be collected based on resource... |
| `CreateTelemetryRuleForOrganization` | `POST /CreateTelemetryRuleForOrganization` | - | `Rule`, `RuleName` | - | `CreateTelemetryRuleForOrganizationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Creates a telemetry rule that applies across an Amazon Web Services Organization. This operation can only be called by the organization's management account or a delegated administrator account. |
| `DeleteCentralizationRuleForOrganization` | `POST /DeleteCentralizationRuleForOrganization` | - | `RuleIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Deletes an organization-wide centralization rule. This operation can only be called by the organization's management account or a delegated administrator account. |
| `DeleteS3TableIntegration` | `POST /DeleteS3TableIntegration` | - | `Arn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidStateException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Deletes an S3 Table integration and its associated data. This operation removes the connection between CloudWatch Observability Admin and S3 Tables. |
| `DeleteTelemetryPipeline` | `POST /DeleteTelemetryPipeline` | `idempotent` | `PipelineIdentifier` | - | `DeleteTelemetryPipelineOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Deletes a telemetry pipeline and its associated resources. This operation stops data processing and removes the pipeline configuration. |
| `DeleteTelemetryRule` | `POST /DeleteTelemetryRule` | - | `RuleIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Deletes a telemetry rule from your account. Any telemetry configurations previously created by the rule will remain but no new resources will be configured by this rule. |
| `DeleteTelemetryRuleForOrganization` | `POST /DeleteTelemetryRuleForOrganization` | - | `RuleIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Deletes an organization-wide telemetry rule. This operation can only be called by the organization's management account or a delegated administrator account. |
| `GetCentralizationRuleForOrganization` | `POST /GetCentralizationRuleForOrganization` | `readonly` | `RuleIdentifier` | - | `GetCentralizationRuleForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Retrieves the details of a specific organization centralization rule. This operation can only be called by the organization's management account or a delegated administrator account. |
| `GetS3TableIntegration` | `POST /GetS3TableIntegration` | `readonly` | `Arn` | - | `GetS3TableIntegrationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Retrieves information about a specific S3 Table integration, including its configuration, status, and metadata. |
| `GetTelemetryEnrichmentStatus` | `POST /GetTelemetryEnrichmentStatus` | `readonly` | - | - | `GetTelemetryEnrichmentStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns the current status of the resource tags for telemetry feature, which enhances telemetry data with additional resource metadata from Resource Explorer. |
| `GetTelemetryEvaluationStatus` | `POST /GetTelemetryEvaluationStatus` | `readonly` | - | - | `GetTelemetryEvaluationStatusOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException` | Returns the current onboarding status of the telemetry config feature, including the status of the feature and reason the feature failed to start or stop. |
| `GetTelemetryEvaluationStatusForOrganization` | `POST /GetTelemetryEvaluationStatusForOrganization` | `readonly` | - | - | `GetTelemetryEvaluationStatusForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | This returns the onboarding status of the telemetry configuration feature for the organization. It can only be called by a Management Account of an Amazon Web Services Organization or an assigned Delegated Admin Account of Amazon CloudWatch telemetry config. |
| `GetTelemetryPipeline` | `POST /GetTelemetryPipeline` | `readonly` | `PipelineIdentifier` | - | `GetTelemetryPipelineOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Retrieves information about a specific telemetry pipeline, including its configuration, status, and metadata. |
| `GetTelemetryRule` | `POST /GetTelemetryRule` | `readonly` | `RuleIdentifier` | - | `GetTelemetryRuleOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Retrieves the details of a specific telemetry rule in your account. |
| `GetTelemetryRuleForOrganization` | `POST /GetTelemetryRuleForOrganization` | `readonly` | `RuleIdentifier` | - | `GetTelemetryRuleForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Retrieves the details of a specific organization telemetry rule. This operation can only be called by the organization's management account or a delegated administrator account. |
| `ListCentralizationRulesForOrganization` | `POST /ListCentralizationRulesForOrganization` | `readonly`, `paginated` | - | - | `ListCentralizationRulesForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Lists all centralization rules in your organization. This operation can only be called by the organization's management account or a delegated administrator account. |
| `ListResourceTelemetry` | `POST /ListResourceTelemetry` | `readonly`, `paginated` | - | - | `ListResourceTelemetryOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns a list of telemetry configurations for Amazon Web Services resources supported by telemetry config. For more information, see Auditing CloudWatch telemetry configurations. |
| `ListResourceTelemetryForOrganization` | `POST /ListResourceTelemetryForOrganization` | `readonly`, `paginated` | - | - | `ListResourceTelemetryForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns a list of telemetry configurations for Amazon Web Services resources supported by telemetry config in the organization. |
| `ListS3TableIntegrations` | `POST /ListS3TableIntegrations` | `readonly`, `paginated` | - | - | `ListS3TableIntegrationsOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Lists all S3 Table integrations in your account. We recommend using pagination to ensure that the operation returns quickly and successfully. |
| `ListTagsForResource` | `POST /ListTagsForResource` | `readonly` | `ResourceARN` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Lists all tags attached to the specified resource. Supports telemetry rule resources and telemetry pipeline resources. |
| `ListTelemetryPipelines` | `POST /ListTelemetryPipelines` | `readonly`, `paginated` | - | - | `ListTelemetryPipelinesOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns a list of telemetry pipelines in your account. Returns up to 100 results. |
| `ListTelemetryRules` | `POST /ListTelemetryRules` | `readonly`, `paginated` | - | - | `ListTelemetryRulesOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Lists all telemetry rules in your account. You can filter the results by specifying a rule name prefix. |
| `ListTelemetryRulesForOrganization` | `POST /ListTelemetryRulesForOrganization` | `readonly`, `paginated` | - | - | `ListTelemetryRulesForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Lists all telemetry rules in your organization. This operation can only be called by the organization's management account or a delegated administrator account. |
| `StartTelemetryEnrichment` | `POST /StartTelemetryEnrichment` | - | - | - | `StartTelemetryEnrichmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `TooManyRequestsException` | Enables the resource tags for telemetry feature for your account, which enhances telemetry data with additional resource metadata from Resource Explorer to provide richer context for monitoring and observability. |
| `StartTelemetryEvaluation` | `POST /StartTelemetryEvaluation` | - | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | This action begins onboarding the caller Amazon Web Services account to the telemetry config feature. |
| `StartTelemetryEvaluationForOrganization` | `POST /StartTelemetryEvaluationForOrganization` | - | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | This actions begins onboarding the organization and all member accounts to the telemetry config feature. |
| `StopTelemetryEnrichment` | `POST /StopTelemetryEnrichment` | - | - | - | `StopTelemetryEnrichmentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `TooManyRequestsException` | Disables the resource tags for telemetry feature for your account, stopping the enhancement of telemetry data with additional resource metadata. |
| `StopTelemetryEvaluation` | `POST /StopTelemetryEvaluation` | - | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | This action begins offboarding the caller Amazon Web Services account from the telemetry config feature. |
| `StopTelemetryEvaluationForOrganization` | `POST /StopTelemetryEvaluationForOrganization` | - | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | This action offboards the Organization of the caller Amazon Web Services account from the telemetry config feature. |
| `TagResource` | `POST /TagResource` | - | `ResourceARN`, `Tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Adds or updates tags for a resource. Supports telemetry rule resources and telemetry pipeline resources. |
| `TestTelemetryPipeline` | `POST /TestTelemetryPipeline` | - | `Configuration`, `Records` | - | `TestTelemetryPipelineOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Tests a pipeline configuration with sample records to validate data processing before deployment. This operation helps ensure your pipeline configuration works as expected. |
| `UntagResource` | `POST /UntagResource` | - | `ResourceARN`, `TagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Removes tags from a resource. Supports telemetry rule resources and telemetry pipeline resources. |
| `UpdateCentralizationRuleForOrganization` | `POST /UpdateCentralizationRuleForOrganization` | - | `Rule`, `RuleIdentifier` | - | `UpdateCentralizationRuleForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Updates an existing centralization rule that applies across an Amazon Web Services Organization. This operation can only be called by the organization's management account or a delegated administrator account. |
| `UpdateTelemetryPipeline` | `POST /UpdateTelemetryPipeline` | - | `Configuration`, `PipelineIdentifier` | - | `UpdateTelemetryPipelineOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Updates the configuration of an existing telemetry pipeline. The following attributes cannot be updated after pipeline creation: Pipeline name - The pipeline name is immutable Pipeline ARN - The ARN is automatically generated and cannot be changed Source type... |
| `UpdateTelemetryRule` | `POST /UpdateTelemetryRule` | - | `Rule`, `RuleIdentifier` | - | `UpdateTelemetryRuleOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Updates an existing telemetry rule in your account. If multiple users attempt to modify the same telemetry rule simultaneously, a ConflictException is returned to provide specific error information for concurrent modification scenarios. |
| `UpdateTelemetryRuleForOrganization` | `POST /UpdateTelemetryRuleForOrganization` | - | `Rule`, `RuleIdentifier` | - | `UpdateTelemetryRuleForOrganizationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException`, `ValidationException` | Updates an existing telemetry rule that applies across an Amazon Web Services Organization. This operation can only be called by the organization's management account or a delegated administrator account. |
| `ValidateTelemetryPipelineConfiguration` | `POST /ValidateTelemetryPipelineConfiguration` | - | `Configuration` | - | `ValidateTelemetryPipelineConfigurationOutput` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Validates a pipeline configuration without creating the pipeline. This operation checks the configuration for syntax errors and compatibility issues. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message`, `amznErrorType` | Indicates you don't have permissions to perform the requested operation. |
| `InternalServerException` | `structure` | `Message`, `amznErrorType`, `retryAfterSeconds` | Indicates the request has failed to process because of an unknown server error, exception, or failure. |
| `TooManyRequestsException` | `structure` | `Message` | The request throughput limit was exceeded. |
| `ValidationException` | `structure` | `Errors`, `Message` | Indicates input validation failed. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The specified resource (such as a telemetry rule) could not be found. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode`, `amznErrorType` | The requested operation would exceed the allowed quota for the specified resource type. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The requested operation conflicts with the current state of the specified resource or with another request. |
| `CreateCentralizationRuleForOrganizationInput` | `structure` | `Rule`, `RuleName`, `Tags` | - |
| `CreateCentralizationRuleForOrganizationOutput` | `structure` | `RuleArn` | - |
| `CreateS3TableIntegrationInput` | `structure` | `Encryption`, `RoleArn`, `Tags` | - |
| `CreateS3TableIntegrationOutput` | `structure` | `Arn` | - |
| `CreateTelemetryPipelineInput` | `structure` | `Configuration`, `Name`, `Tags` | - |
| `CreateTelemetryPipelineOutput` | `structure` | `Arn` | - |
| `CreateTelemetryRuleInput` | `structure` | `Rule`, `RuleName`, `Tags` | - |
| `CreateTelemetryRuleOutput` | `structure` | `RuleArn` | - |
| `CreateTelemetryRuleForOrganizationInput` | `structure` | `Rule`, `RuleName`, `Tags` | - |
| `CreateTelemetryRuleForOrganizationOutput` | `structure` | `RuleArn` | - |
| `DeleteCentralizationRuleForOrganizationInput` | `structure` | `RuleIdentifier` | - |
| `DeleteS3TableIntegrationInput` | `structure` | `Arn` | - |
| `InvalidStateException` | `structure` | `Message` | The requested operation cannot be completed on the specified resource in the current state. |
| `DeleteTelemetryPipelineInput` | `structure` | `PipelineIdentifier` | - |
| `DeleteTelemetryPipelineOutput` | `structure` | - | - |
| `DeleteTelemetryRuleInput` | `structure` | `RuleIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

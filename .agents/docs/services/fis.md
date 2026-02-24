# AWS Fault Injection Simulator

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Fault Injection Service is a managed service that enables you to perform fault injection experiments on your Amazon Web Services workloads. For more information, see the Fault Injection Service User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Fault Injection Simulator by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Fault Injection Simulator by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Fault Injection Simulator workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListActions`, `ListExperimentResolvedTargets`, `ListExperimentTargetAccountConfigurations`, `ListExperimentTemplates`, `GetAction`, `GetExperiment`.

## Service Identity and Protocol

- AWS model slug: `fis`
- AWS SDK for Rust slug: `fis`
- Model version: `2020-12-01`
- Model file: `vendor/api-models-aws/models/fis/service/2020-12-01/fis-2020-12-01.json`
- SDK ID: `fis`
- Endpoint prefix: `fis`
- ARN namespace: `fis`
- CloudFormation name: `Fis`
- CloudTrail event source: `fis.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (7), `Update` (3), `Create` (2), `Delete` (2), `Start` (1), `Stop` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateExperimentTemplate`, `CreateTargetAccountConfiguration`, `DeleteExperimentTemplate`, `DeleteTargetAccountConfiguration`, `StartExperiment`, `StopExperiment`, `TagResource`, `UntagResource`, `UpdateExperimentTemplate`, `UpdateSafetyLeverState`, `UpdateTargetAccountConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAction`, `GetExperiment`, `GetExperimentTargetAccountConfiguration`, `GetExperimentTemplate`, `GetSafetyLever`, `GetTargetAccountConfiguration`, `GetTargetResourceType`, `ListActions`, `ListExperimentResolvedTargets`, `ListExperimentTargetAccountConfigurations`, `ListExperimentTemplates`, `ListExperiments`, `ListTagsForResource`, `ListTargetAccountConfigurations`, `ListTargetResourceTypes`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartExperiment`, `StopExperiment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/fis/latest/userguide/what-is.html
- https://docs.aws.amazon.com/fis/latest/userguide/create-template.html
- https://docs.aws.amazon.com/fis/latest/userguide/run-experiment.html
- https://docs.aws.amazon.com/fis/latest/userguide/stop-conditions.html

Research outcomes:
- AWS Fault Injection Service runs controlled experiments that inject faults into supported AWS resources.
- Experiment templates define actions, targets, stop conditions, IAM role, logging, tags, and optional experiment options.
- Actions define the fault to inject. Targets select affected resources by ids, tags, filters, or resource type depending on the action.
- Stop conditions use CloudWatch alarms as guardrails. If a stop condition alarm breaches, the experiment stops.
- Experiments are started from templates and can be tracked through progress/status.
- Experiments can be scheduled through EventBridge and monitored through FIS console, CloudWatch metrics, and logging.
- IAM policies can restrict which actions, templates, tags, and resources users can run.

Parity implications:
- Model experiment templates, actions, targets, stop conditions, experiments, action execution state, target resolution, logs, and IAM role use separately.
- StartExperiment should resolve targets at execution time and create an asynchronous experiment record.
- Stop conditions should be evaluated during execution and cause stop transitions rather than only template validation.

## Operation Groups

### List

- Operations: `ListActions`, `ListExperimentResolvedTargets`, `ListExperimentTargetAccountConfigurations`, `ListExperimentTemplates`, `ListExperiments`, `ListTagsForResource`, `ListTargetAccountConfigurations`, `ListTargetResourceTypes`
- Traits: `paginated` (6)
- Common required input members in this group: `experimentId`, `experimentTemplateId`, `resourceArn`

### Get

- Operations: `GetAction`, `GetExperiment`, `GetExperimentTargetAccountConfiguration`, `GetExperimentTemplate`, `GetSafetyLever`, `GetTargetAccountConfiguration`, `GetTargetResourceType`
- Common required input members in this group: `accountId`, `experimentId`, `experimentTemplateId`, `id`, `resourceType`

### Update

- Operations: `UpdateExperimentTemplate`, `UpdateSafetyLeverState`, `UpdateTargetAccountConfiguration`
- Common required input members in this group: `accountId`, `experimentTemplateId`, `id`, `state`

### Create

- Operations: `CreateExperimentTemplate`, `CreateTargetAccountConfiguration`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `accountId`, `actions`, `clientToken`, `description`, `experimentTemplateId`, `roleArn`, `stopConditions`

### Delete

- Operations: `DeleteExperimentTemplate`, `DeleteTargetAccountConfiguration`
- Common required input members in this group: `accountId`, `experimentTemplateId`, `id`

### Start

- Operations: `StartExperiment`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `clientToken`, `experimentTemplateId`

### Stop

- Operations: `StopExperiment`
- Common required input members in this group: `id`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateExperimentTemplate` | `POST /experimentTemplates` | `idempotency-token` | `actions`, `clientToken`, `description`, `roleArn`, `stopConditions` | `clientToken` | `CreateExperimentTemplateResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an experiment template. An experiment template includes the following components: Targets : A target can be a specific resource in your Amazon Web Services environment, or one or more resources that match criteria that you specify, for example... |
| `CreateTargetAccountConfiguration` | `POST /experimentTemplates/{experimentTemplateId}/targetAccountConfigurations/{accountId}` | `idempotency-token` | `accountId`, `experimentTemplateId`, `roleArn` | `clientToken` | `CreateTargetAccountConfigurationResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a target account configuration for the experiment template. A target account configuration is required when `accountTargeting` of `experimentOptions` is set to `multi-account`. |
| `DeleteExperimentTemplate` | `DELETE /experimentTemplates/{id}` | - | `id` | - | `DeleteExperimentTemplateResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes the specified experiment template. |
| `DeleteTargetAccountConfiguration` | `DELETE /experimentTemplates/{experimentTemplateId}/targetAccountConfigurations/{accountId}` | - | `accountId`, `experimentTemplateId` | - | `DeleteTargetAccountConfigurationResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes the specified target account configuration of the experiment template. |
| `GetAction` | `GET /actions/{id}` | - | `id` | - | `GetActionResponse` | `ResourceNotFoundException`, `ValidationException` | Gets information about the specified FIS action. |
| `GetExperiment` | `GET /experiments/{id}` | - | `id` | - | `GetExperimentResponse` | `ResourceNotFoundException`, `ValidationException` | Gets information about the specified experiment. |
| `GetExperimentTargetAccountConfiguration` | `GET /experiments/{experimentId}/targetAccountConfigurations/{accountId}` | - | `accountId`, `experimentId` | - | `GetExperimentTargetAccountConfigurationResponse` | `ResourceNotFoundException`, `ValidationException` | Gets information about the specified target account configuration of the experiment. |
| `GetExperimentTemplate` | `GET /experimentTemplates/{id}` | - | `id` | - | `GetExperimentTemplateResponse` | `ResourceNotFoundException`, `ValidationException` | Gets information about the specified experiment template. |
| `GetSafetyLever` | `GET /safetyLevers/{id}` | - | `id` | - | `GetSafetyLeverResponse` | `ResourceNotFoundException` | Gets information about the specified safety lever. |
| `GetTargetAccountConfiguration` | `GET /experimentTemplates/{experimentTemplateId}/targetAccountConfigurations/{accountId}` | - | `accountId`, `experimentTemplateId` | - | `GetTargetAccountConfigurationResponse` | `ResourceNotFoundException`, `ValidationException` | Gets information about the specified target account configuration of the experiment template. |
| `GetTargetResourceType` | `GET /targetResourceTypes/{resourceType}` | - | `resourceType` | - | `GetTargetResourceTypeResponse` | `ResourceNotFoundException`, `ValidationException` | Gets information about the specified resource type. |
| `ListActions` | `GET /actions` | `paginated` | - | - | `ListActionsResponse` | `ValidationException` | Lists the available FIS actions. |
| `ListExperimentResolvedTargets` | `GET /experiments/{experimentId}/resolvedTargets` | `paginated` | `experimentId` | - | `ListExperimentResolvedTargetsResponse` | `ResourceNotFoundException`, `ValidationException` | Lists the resolved targets information of the specified experiment. |
| `ListExperimentTargetAccountConfigurations` | `GET /experiments/{experimentId}/targetAccountConfigurations` | - | `experimentId` | - | `ListExperimentTargetAccountConfigurationsResponse` | `ResourceNotFoundException`, `ValidationException` | Lists the target account configurations of the specified experiment. |
| `ListExperimentTemplates` | `GET /experimentTemplates` | `paginated` | - | - | `ListExperimentTemplatesResponse` | `ValidationException` | Lists your experiment templates. |
| `ListExperiments` | `GET /experiments` | `paginated` | - | - | `ListExperimentsResponse` | `ValidationException` | Lists your experiments. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | - | Lists the tags for the specified resource. |
| `ListTargetAccountConfigurations` | `GET /experimentTemplates/{experimentTemplateId}/targetAccountConfigurations` | `paginated` | `experimentTemplateId` | - | `ListTargetAccountConfigurationsResponse` | `ResourceNotFoundException`, `ValidationException` | Lists the target account configurations of the specified experiment template. |
| `ListTargetResourceTypes` | `GET /targetResourceTypes` | `paginated` | - | - | `ListTargetResourceTypesResponse` | `ValidationException` | Lists the target resource types. |
| `StartExperiment` | `POST /experiments` | `idempotency-token` | `clientToken`, `experimentTemplateId` | `clientToken` | `StartExperimentResponse` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Starts running an experiment from the specified experiment template. |
| `StopExperiment` | `DELETE /experiments/{id}` | - | `id` | - | `StopExperimentResponse` | `ResourceNotFoundException`, `ValidationException` | Stops the specified experiment. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | - | Applies the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn` | - | `UntagResourceResponse` | - | Removes the specified tags from the specified resource. |
| `UpdateExperimentTemplate` | `PATCH /experimentTemplates/{id}` | - | `id` | - | `UpdateExperimentTemplateResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates the specified experiment template. |
| `UpdateSafetyLeverState` | `PATCH /safetyLevers/{id}/state` | - | `id`, `state` | - | `UpdateSafetyLeverStateResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Updates the specified safety lever state. |
| `UpdateTargetAccountConfiguration` | `PATCH /experimentTemplates/{experimentTemplateId}/targetAccountConfigurations/{accountId}` | - | `accountId`, `experimentTemplateId` | - | `UpdateTargetAccountConfigurationResponse` | `ResourceNotFoundException`, `ValidationException` | Updates the target account configuration for the specified experiment template. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `message` | The specified input is not valid, or fails to satisfy the constraints for the request. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource cannot be found. |
| `ConflictException` | `structure` | `message` | The request could not be processed because of a conflict. |
| `ServiceQuotaExceededException` | `structure` | `message` | You have exceeded your service quota. |
| `CreateExperimentTemplateRequest` | `structure` | `actions`, `clientToken`, `description`, `experimentOptions`, `experimentReportConfiguration`, `logConfiguration`, `roleArn`, `stopConditions`, `tags`, `targets` | - |
| `CreateExperimentTemplateResponse` | `structure` | `experimentTemplate` | - |
| `CreateTargetAccountConfigurationRequest` | `structure` | `accountId`, `clientToken`, `description`, `experimentTemplateId`, `roleArn` | - |
| `CreateTargetAccountConfigurationResponse` | `structure` | `targetAccountConfiguration` | - |
| `DeleteExperimentTemplateRequest` | `structure` | `id` | - |
| `DeleteExperimentTemplateResponse` | `structure` | `experimentTemplate` | - |
| `DeleteTargetAccountConfigurationRequest` | `structure` | `accountId`, `experimentTemplateId` | - |
| `DeleteTargetAccountConfigurationResponse` | `structure` | `targetAccountConfiguration` | - |
| `GetActionRequest` | `structure` | `id` | - |
| `GetActionResponse` | `structure` | `action` | - |
| `GetExperimentRequest` | `structure` | `id` | - |
| `GetExperimentResponse` | `structure` | `experiment` | - |
| `GetExperimentTargetAccountConfigurationRequest` | `structure` | `accountId`, `experimentId` | - |
| `GetExperimentTargetAccountConfigurationResponse` | `structure` | `targetAccountConfiguration` | - |
| `GetExperimentTemplateRequest` | `structure` | `id` | - |
| `GetExperimentTemplateResponse` | `structure` | `experimentTemplate` | - |
| `GetSafetyLeverRequest` | `structure` | `id` | - |
| `GetSafetyLeverResponse` | `structure` | `safetyLever` | - |
| `GetTargetAccountConfigurationRequest` | `structure` | `accountId`, `experimentTemplateId` | - |
| `GetTargetAccountConfigurationResponse` | `structure` | `targetAccountConfiguration` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

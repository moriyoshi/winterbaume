# AWS CodePipeline

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

CodePipeline Overview This is the CodePipeline API Reference. This guide provides descriptions of the actions and data types for CodePipeline. Some functionality for your pipeline can only be configured through the API. For more information, see the CodePipeline User Guide. You can use the CodePipeline API to work with pipelines, stages, actions, and transitions.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS CodePipeline by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS CodePipeline resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS CodePipeline workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Put`, `Get`, `Delete`, `Acknowledge` operation families, including `ListActionExecutions`, `ListActionTypes`, `ListDeployActionExecutionTargets`, `ListPipelineExecutions`, `PutActionRevision`, `PutApprovalResult`.

## Service Identity and Protocol

- AWS model slug: `codepipeline`
- AWS SDK for Rust slug: `codepipeline`
- Model version: `2015-07-09`
- Model file: `vendor/api-models-aws/models/codepipeline/service/2015-07-09/codepipeline-2015-07-09.json`
- SDK ID: `CodePipeline`
- Endpoint prefix: `codepipeline`
- ARN namespace: `codepipeline`
- CloudFormation name: `CodePipeline`
- CloudTrail event source: `codepipeline.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (9), `Put` (7), `Get` (6), `Delete` (3), `Acknowledge` (2), `Create` (2), `Poll` (2), `Update` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCustomActionType`, `CreatePipeline`, `DeleteCustomActionType`, `DeletePipeline`, `DeleteWebhook`, `DeregisterWebhookWithThirdParty`, `DisableStageTransition`, `EnableStageTransition`, `PutActionRevision`, `PutApprovalResult`, `PutJobFailureResult`, `PutJobSuccessResult`, `PutThirdPartyJobFailureResult`, `PutThirdPartyJobSuccessResult`, `PutWebhook`, `RegisterWebhookWithThirdParty`, `StartPipelineExecution`, `StopPipelineExecution`, `TagResource`, `UntagResource`, ... (+2).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetActionType`, `GetJobDetails`, `GetPipeline`, `GetPipelineExecution`, `GetPipelineState`, `GetThirdPartyJobDetails`, `ListActionExecutions`, `ListActionTypes`, `ListDeployActionExecutionTargets`, `ListPipelineExecutions`, `ListPipelines`, `ListRuleExecutions`, `ListRuleTypes`, `ListTagsForResource`, `ListWebhooks`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `AcknowledgeJob`, `AcknowledgeThirdPartyJob`, `GetJobDetails`, `GetPipelineExecution`, `GetThirdPartyJobDetails`, `ListActionExecutions`, `ListDeployActionExecutionTargets`, `ListPipelineExecutions`, `ListRuleExecutions`, `PollForJobs`, `PollForThirdPartyJobs`, `PutJobFailureResult`, `PutJobSuccessResult`, `PutThirdPartyJobFailureResult`, `PutThirdPartyJobSuccessResult`, `RetryStageExecution`, `StartPipelineExecution`, `StopPipelineExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 44 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `Lambda`, `ECR`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codepipeline/latest/userguide/pipelines-stages.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/stage-retry.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/stage-rollback-auto.html
- https://docs.aws.amazon.com/codepipeline/latest/userguide/executions-view.html

Research outcomes:
- CodePipeline pipelines consist of ordered stages and actions, and pipeline executions carry action state, source revisions, artifact metadata, and stage execution state.
- Failed stages can be retried manually as all actions or failed actions only.
- Stage retry accepts retry modes FAILED_ACTIONS and ALL_ACTIONS for a specific pipeline execution ID.
- Automatic retry on stage failure can be configured for one retry attempt and can apply to all action categories including Source actions.
- Stages can be configured for automatic rollback on failure. Rollback uses the most recent successful execution in the same pipeline structure version.
- Automatic rollback only works after there has been a successful execution in that stage after the rollback configuration is part of the pipeline definition.
- Pipeline execution history and stage state expose latest execution status, action execution details, source revisions, and artifact store information.

Parity implications:
- Model pipelines, versions, stages, actions, executions, action executions, artifacts, source revisions, transitions, retry state, and rollback state.
- Stage retry should operate on an existing failed execution and distinguish all-action retry from failed-action retry.
- Rollback must track successful executions per pipeline structure version and stage, rather than simply rerunning the current revision.

## Operation Groups

### List

- Operations: `ListActionExecutions`, `ListActionTypes`, `ListDeployActionExecutionTargets`, `ListPipelineExecutions`, `ListPipelines`, `ListRuleExecutions`, `ListRuleTypes`, `ListTagsForResource`, `ListWebhooks`
- Traits: `paginated` (8)
- Common required input members in this group: `actionExecutionId`, `pipelineName`, `resourceArn`

### Put

- Operations: `PutActionRevision`, `PutApprovalResult`, `PutJobFailureResult`, `PutJobSuccessResult`, `PutThirdPartyJobFailureResult`, `PutThirdPartyJobSuccessResult`, `PutWebhook`
- Common required input members in this group: `actionName`, `actionRevision`, `clientToken`, `failureDetails`, `jobId`, `pipelineName`, `result`, `stageName`, `token`, `webhook`

### Get

- Operations: `GetActionType`, `GetJobDetails`, `GetPipeline`, `GetPipelineExecution`, `GetPipelineState`, `GetThirdPartyJobDetails`
- Common required input members in this group: `category`, `clientToken`, `jobId`, `name`, `owner`, `pipelineExecutionId`, `pipelineName`, `provider`, `version`

### Delete

- Operations: `DeleteCustomActionType`, `DeletePipeline`, `DeleteWebhook`
- Common required input members in this group: `category`, `name`, `provider`, `version`

### Acknowledge

- Operations: `AcknowledgeJob`, `AcknowledgeThirdPartyJob`
- Common required input members in this group: `clientToken`, `jobId`, `nonce`

### Create

- Operations: `CreateCustomActionType`, `CreatePipeline`
- Common required input members in this group: `category`, `inputArtifactDetails`, `outputArtifactDetails`, `pipeline`, `provider`, `version`

### Poll

- Operations: `PollForJobs`, `PollForThirdPartyJobs`
- Common required input members in this group: `actionTypeId`

### Update

- Operations: `UpdateActionType`, `UpdatePipeline`
- Common required input members in this group: `actionType`, `pipeline`

### Deregister

- Operations: `DeregisterWebhookWithThirdParty`

### Disable

- Operations: `DisableStageTransition`
- Common required input members in this group: `pipelineName`, `reason`, `stageName`, `transitionType`

### Enable

- Operations: `EnableStageTransition`
- Common required input members in this group: `pipelineName`, `stageName`, `transitionType`

### Override

- Operations: `OverrideStageCondition`
- Common required input members in this group: `conditionType`, `pipelineExecutionId`, `pipelineName`, `stageName`

### Register

- Operations: `RegisterWebhookWithThirdParty`

### Retry

- Operations: `RetryStageExecution`
- Common required input members in this group: `pipelineExecutionId`, `pipelineName`, `retryMode`, `stageName`

### Rollback

- Operations: `RollbackStage`
- Common required input members in this group: `pipelineName`, `stageName`, `targetPipelineExecutionId`

### Start

- Operations: `StartPipelineExecution`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `name`

### Stop

- Operations: `StopPipelineExecution`
- Common required input members in this group: `pipelineExecutionId`, `pipelineName`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcknowledgeJob` | - | - | `jobId`, `nonce` | - | `AcknowledgeJobOutput` | `InvalidNonceException`, `JobNotFoundException`, `ValidationException` | Returns information about a specified job and whether that job has been received by the job worker. Used for custom actions only. |
| `AcknowledgeThirdPartyJob` | - | - | `clientToken`, `jobId`, `nonce` | - | `AcknowledgeThirdPartyJobOutput` | `InvalidClientTokenException`, `InvalidNonceException`, `JobNotFoundException`, `ValidationException` | Confirms a job worker has received the specified job. Used for partner actions only. |
| `CreateCustomActionType` | - | - | `category`, `inputArtifactDetails`, `outputArtifactDetails`, `provider`, `version` | - | `CreateCustomActionTypeOutput` | `ConcurrentModificationException`, `InvalidTagsException`, `LimitExceededException`, `TooManyTagsException`, `ValidationException` | Creates a new custom action that can be used in all pipelines associated with the Amazon Web Services account. Only used for custom actions. |
| `CreatePipeline` | - | - | `pipeline` | - | `CreatePipelineOutput` | `ConcurrentModificationException`, `InvalidActionDeclarationException`, `InvalidBlockerDeclarationException`, `InvalidStageDeclarationException`, `InvalidStructureException`, `InvalidTagsException`, `LimitExceededException`, `PipelineNameInUseException`, ... (+2) | Creates a pipeline. In the pipeline structure, you must include either `artifactStore` or `artifactStores` in your pipeline, but you cannot use both. |
| `DeleteCustomActionType` | - | - | `category`, `provider`, `version` | - | `Unit` | `ConcurrentModificationException`, `ValidationException` | Marks a custom action as deleted. `PollForJobs` for the custom action fails after the action is marked for deletion. |
| `DeletePipeline` | - | - | `name` | - | `Unit` | `ConcurrentModificationException`, `ValidationException` | Deletes the specified pipeline. |
| `DeleteWebhook` | - | - | `name` | - | `DeleteWebhookOutput` | `ConcurrentModificationException`, `ValidationException` | Deletes a previously created webhook by name. Deleting the webhook stops CodePipeline from starting a pipeline every time an external event occurs. |
| `DeregisterWebhookWithThirdParty` | - | - | - | - | `DeregisterWebhookWithThirdPartyOutput` | `ValidationException`, `WebhookNotFoundException` | Removes the connection between the webhook that was created by CodePipeline and the external tool with events to be detected. Currently supported only for webhooks that target an action type of GitHub. |
| `DisableStageTransition` | - | - | `pipelineName`, `reason`, `stageName`, `transitionType` | - | `Unit` | `PipelineNotFoundException`, `StageNotFoundException`, `ValidationException` | Prevents artifacts in a pipeline from transitioning to the next stage in the pipeline. |
| `EnableStageTransition` | - | - | `pipelineName`, `stageName`, `transitionType` | - | `Unit` | `PipelineNotFoundException`, `StageNotFoundException`, `ValidationException` | Enables artifacts in a pipeline to transition to a stage in a pipeline. |
| `GetActionType` | - | - | `category`, `owner`, `provider`, `version` | - | `GetActionTypeOutput` | `ActionTypeNotFoundException`, `ValidationException` | Returns information about an action type created for an external provider, where the action is to be used by customers of the external provider. The action can be created with any supported integration model. |
| `GetJobDetails` | - | - | `jobId` | - | `GetJobDetailsOutput` | `JobNotFoundException`, `ValidationException` | Returns information about a job. Used for custom actions only. |
| `GetPipeline` | - | - | `name` | - | `GetPipelineOutput` | `PipelineNotFoundException`, `PipelineVersionNotFoundException`, `ValidationException` | Returns the metadata, structure, stages, and actions of a pipeline. Can be used to return the entire structure of a pipeline in JSON format, which can then be modified and used to update the pipeline structure with UpdatePipeline. |
| `GetPipelineExecution` | - | - | `pipelineExecutionId`, `pipelineName` | - | `GetPipelineExecutionOutput` | `PipelineExecutionNotFoundException`, `PipelineNotFoundException`, `ValidationException` | Returns information about an execution of a pipeline, including details about artifacts, the pipeline execution ID, and the name, version, and status of the pipeline. |
| `GetPipelineState` | - | - | `name` | - | `GetPipelineStateOutput` | `PipelineNotFoundException`, `ValidationException` | Returns information about the state of a pipeline, including the stages and actions. Values returned in the `revisionId` and `revisionUrl` fields indicate the source revision information, such as the commit ID, for the current state. |
| `GetThirdPartyJobDetails` | - | - | `clientToken`, `jobId` | - | `GetThirdPartyJobDetailsOutput` | `InvalidClientTokenException`, `InvalidJobException`, `JobNotFoundException`, `ValidationException` | Requests the details of a job for a third party action. Used for partner actions only. |
| `ListActionExecutions` | - | `paginated` | `pipelineName` | - | `ListActionExecutionsOutput` | `InvalidNextTokenException`, `PipelineExecutionNotFoundException`, `PipelineNotFoundException`, `ValidationException` | Lists the action executions that have occurred in a pipeline. |
| `ListActionTypes` | - | `paginated` | - | - | `ListActionTypesOutput` | `InvalidNextTokenException`, `ValidationException` | Gets a summary of all CodePipeline action types associated with your account. |
| `ListDeployActionExecutionTargets` | - | `paginated` | `actionExecutionId` | - | `ListDeployActionExecutionTargetsOutput` | `ActionExecutionNotFoundException`, `InvalidNextTokenException`, `PipelineNotFoundException`, `ValidationException` | Lists the targets for the deploy action. |
| `ListPipelineExecutions` | - | `paginated` | `pipelineName` | - | `ListPipelineExecutionsOutput` | `InvalidNextTokenException`, `PipelineNotFoundException`, `ValidationException` | Gets a summary of the most recent executions for a pipeline. When applying the filter for pipeline executions that have succeeded in the stage, the operation returns all executions in the current pipeline version beginning on February 1, 2024. |
| `ListPipelines` | - | `paginated` | - | - | `ListPipelinesOutput` | `InvalidNextTokenException`, `ValidationException` | Gets a summary of all of the pipelines associated with your account. |
| `ListRuleExecutions` | - | `paginated` | `pipelineName` | - | `ListRuleExecutionsOutput` | `InvalidNextTokenException`, `PipelineExecutionNotFoundException`, `PipelineNotFoundException`, `ValidationException` | Lists the rule executions that have occurred in a pipeline configured for conditions with rules. |
| `ListRuleTypes` | - | - | - | - | `ListRuleTypesOutput` | `InvalidNextTokenException`, `ValidationException` | Lists the rules for the condition. For more information about conditions, see Stage conditions and How do stage conditions work?.For more information about rules, see the CodePipeline rule reference. |
| `ListTagsForResource` | - | `paginated` | `resourceArn` | - | `ListTagsForResourceOutput` | `InvalidArnException`, `InvalidNextTokenException`, `ResourceNotFoundException`, `ValidationException` | Gets the set of key-value pairs (metadata) that are used to manage the resource. |
| `ListWebhooks` | - | `paginated` | - | - | `ListWebhooksOutput` | `InvalidNextTokenException`, `ValidationException` | Gets a listing of all the webhooks in this Amazon Web Services Region for this account. The output lists all webhooks and includes the webhook URL and ARN and the configuration for each webhook. |
| `OverrideStageCondition` | - | - | `conditionType`, `pipelineExecutionId`, `pipelineName`, `stageName` | - | `Unit` | `ConcurrentPipelineExecutionsLimitExceededException`, `ConditionNotOverridableException`, `ConflictException`, `NotLatestPipelineExecutionException`, `PipelineNotFoundException`, `StageNotFoundException`, `ValidationException` | Used to override a stage condition. For more information about conditions, see Stage conditions and How do stage conditions work?. |
| `PollForJobs` | - | - | `actionTypeId` | - | `PollForJobsOutput` | `ActionTypeNotFoundException`, `ValidationException` | Returns information about any jobs for CodePipeline to act on. `PollForJobs` is valid only for action types with "Custom" in the owner field. |
| `PollForThirdPartyJobs` | - | - | `actionTypeId` | - | `PollForThirdPartyJobsOutput` | `ActionTypeNotFoundException`, `ValidationException` | Determines whether there are any third party jobs for a job worker to act on. Used for partner actions only. |
| `PutActionRevision` | - | - | `actionName`, `actionRevision`, `pipelineName`, `stageName` | - | `PutActionRevisionOutput` | `ActionNotFoundException`, `ConcurrentPipelineExecutionsLimitExceededException`, `PipelineNotFoundException`, `StageNotFoundException`, `ValidationException` | Provides information to CodePipeline about new revisions to a source. |
| `PutApprovalResult` | - | - | `actionName`, `pipelineName`, `result`, `stageName`, `token` | - | `PutApprovalResultOutput` | `ActionNotFoundException`, `ApprovalAlreadyCompletedException`, `InvalidApprovalTokenException`, `PipelineNotFoundException`, `StageNotFoundException`, `ValidationException` | Provides the response to a manual approval request to CodePipeline. Valid responses include Approved and Rejected. |
| `PutJobFailureResult` | - | - | `failureDetails`, `jobId` | - | `Unit` | `InvalidJobStateException`, `JobNotFoundException`, `ValidationException` | Represents the failure of a job as returned to the pipeline by a job worker. Used for custom actions only. |
| `PutJobSuccessResult` | - | - | `jobId` | - | `Unit` | `InvalidJobStateException`, `JobNotFoundException`, `OutputVariablesSizeExceededException`, `ValidationException` | Represents the success of a job as returned to the pipeline by a job worker. Used for custom actions only. |
| `PutThirdPartyJobFailureResult` | - | - | `clientToken`, `failureDetails`, `jobId` | - | `Unit` | `InvalidClientTokenException`, `InvalidJobStateException`, `JobNotFoundException`, `ValidationException` | Represents the failure of a third party job as returned to the pipeline by a job worker. Used for partner actions only. |
| `PutThirdPartyJobSuccessResult` | - | - | `clientToken`, `jobId` | - | `Unit` | `InvalidClientTokenException`, `InvalidJobStateException`, `JobNotFoundException`, `ValidationException` | Represents the success of a third party job as returned to the pipeline by a job worker. Used for partner actions only. |
| `PutWebhook` | - | - | `webhook` | - | `PutWebhookOutput` | `ConcurrentModificationException`, `InvalidTagsException`, `InvalidWebhookAuthenticationParametersException`, `InvalidWebhookFilterPatternException`, `LimitExceededException`, `PipelineNotFoundException`, `TooManyTagsException`, `ValidationException` | Defines a webhook and returns a unique webhook URL generated by CodePipeline. This URL can be supplied to third party source hosting providers to call every time there's a code change. |
| `RegisterWebhookWithThirdParty` | - | - | - | - | `RegisterWebhookWithThirdPartyOutput` | `ValidationException`, `WebhookNotFoundException` | Configures a connection between the webhook that was created and the external tool with events to be detected. |
| `RetryStageExecution` | - | - | `pipelineExecutionId`, `pipelineName`, `retryMode`, `stageName` | - | `RetryStageExecutionOutput` | `ConcurrentPipelineExecutionsLimitExceededException`, `ConflictException`, `NotLatestPipelineExecutionException`, `PipelineNotFoundException`, `StageNotFoundException`, `StageNotRetryableException`, `ValidationException` | You can retry a stage that has failed without having to run a pipeline again from the beginning. You do this by either retrying the failed actions in a stage or by retrying all actions in the stage starting from the first action in the stage. |
| `RollbackStage` | - | - | `pipelineName`, `stageName`, `targetPipelineExecutionId` | - | `RollbackStageOutput` | `ConflictException`, `PipelineExecutionNotFoundException`, `PipelineExecutionOutdatedException`, `PipelineNotFoundException`, `StageNotFoundException`, `UnableToRollbackStageException`, `ValidationException` | Rolls back a stage execution. |
| `StartPipelineExecution` | - | `idempotency-token` | `name` | `clientRequestToken` | `StartPipelineExecutionOutput` | `ConcurrentPipelineExecutionsLimitExceededException`, `ConflictException`, `PipelineNotFoundException`, `ValidationException` | Starts the specified pipeline. Specifically, it begins processing the latest commit to the source location specified as part of the pipeline. |
| `StopPipelineExecution` | - | - | `pipelineExecutionId`, `pipelineName` | - | `StopPipelineExecutionOutput` | `ConflictException`, `DuplicatedStopRequestException`, `PipelineExecutionNotStoppableException`, `PipelineNotFoundException`, `ValidationException` | Stops the specified pipeline execution. You choose to either stop the pipeline execution by completing in-progress actions without starting subsequent actions, or by abandoning in-progress actions. |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidTagsException`, `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Adds to or modifies the tags of the given resource. Tags are metadata that can be used to manage a resource. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `ConcurrentModificationException`, `InvalidArnException`, `InvalidTagsException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from an Amazon Web Services resource. |
| `UpdateActionType` | - | - | `actionType` | - | `Unit` | `ActionTypeNotFoundException`, `RequestFailedException`, `ValidationException` | Updates an action type that was created with any supported integration model, where the action type is to be used by customers of the action type provider. Use a JSON file with the action definition and `UpdateActionType` to provide the full structure. |
| `UpdatePipeline` | - | - | `pipeline` | - | `UpdatePipelineOutput` | `InvalidActionDeclarationException`, `InvalidBlockerDeclarationException`, `InvalidStageDeclarationException`, `InvalidStructureException`, `LimitExceededException`, `ValidationException` | Updates a specified pipeline with edits or changes to its structure. Use a JSON file with the pipeline structure and `UpdatePipeline` to provide the full structure of the pipeline. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `message` | The validation was specified in an invalid format. |
| `PipelineNotFoundException` | `structure` | `message` | The pipeline was specified in an invalid format or cannot be found. |
| `InvalidNextTokenException` | `structure` | `message` | The next token was specified in an invalid format. |
| `JobNotFoundException` | `structure` | `message` | The job was specified in an invalid format or cannot be found. |
| `ConcurrentModificationException` | `structure` | `message` | Unable to modify the tag due to a simultaneous update request. |
| `StageNotFoundException` | `structure` | `message` | The stage was specified in an invalid format or cannot be found. |
| `InvalidTagsException` | `structure` | `message` | The specified resource tags are invalid. |
| `ConflictException` | `structure` | `message` | Your request cannot be handled because the pipeline is busy handling ongoing activities. |
| `InvalidClientTokenException` | `structure` | `message` | The client token was specified in an invalid format |
| `LimitExceededException` | `structure` | `message` | The number of pipelines associated with the Amazon Web Services account has exceeded the limit allowed for the account. |
| `TooManyTagsException` | `structure` | `message` | The tags limit for a resource has been exceeded. |
| `ActionTypeNotFoundException` | `structure` | `message` | The specified action type cannot be found. |
| `PipelineExecutionNotFoundException` | `structure` | `message` | The pipeline execution was specified in an invalid format or cannot be found, or an execution ID does not belong to the specified pipeline. |
| `ConcurrentPipelineExecutionsLimitExceededException` | `structure` | `message` | The pipeline has reached the limit for concurrent pipeline executions. |
| `InvalidJobStateException` | `structure` | `message` | The job state was specified in an invalid format. |
| `InvalidArnException` | `structure` | `message` | The specified resource ARN is invalid. |
| `ResourceNotFoundException` | `structure` | `message` | The resource was specified in an invalid format. |
| `InvalidNonceException` | `structure` | `message` | The nonce was specified in an invalid format. |
| `InvalidActionDeclarationException` | `structure` | `message` | The action declaration was specified in an invalid format. |
| `InvalidBlockerDeclarationException` | `structure` | `message` | Reserved for future use. |
| `InvalidStageDeclarationException` | `structure` | `message` | The stage declaration was specified in an invalid format. |
| `InvalidStructureException` | `structure` | `message` | The structure was specified in an invalid format. |
| `WebhookNotFoundException` | `structure` | - | The specified webhook was entered in an invalid format or cannot be found. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

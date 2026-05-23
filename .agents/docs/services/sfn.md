# AWS Step Functions

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Step Functions With Step Functions, you can create workflows, also called state machines , to build distributed applications, automate processes, orchestrate microservices, and create data and machine learning pipelines. Through the Step Functions API, you can create, list, update, and delete state machines, activities, and other data types. You can start, stop, and redrive your state machines. Your activity workers can send task success, heartbeat, and failure responses. With API calls, you can also manage other aspects of your workflow, such as tags, versions, and aliases.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-sfn/tests/scenario_test.rs`: create a state machine, start an execution, describe/stop it, and delete the state machine.
- Backported from `scenario_test.rs`: publish versions, create an alias, and route executions through the alias.
- Backported from `scenario_test.rs`: redrive a failed execution back to running, and manage activity tagging lifecycle.
- Scenario insight from EC2: add full state-machine walks for AWS Step Functions resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support workflow orchestration with state machines, executions, activities, versions, aliases, maps, redrive, logging/tracing configuration, tags, and execution history.

## Service Identity and Protocol

- AWS model slug: `sfn`
- AWS SDK for Rust slug: `sfn`
- Model version: `2016-11-23`
- Model file: `vendor/api-models-aws/models/sfn/service/2016-11-23/sfn-2016-11-23.json`
- SDK ID: `SFN`
- Endpoint prefix: `states`
- ARN namespace: `states`
- CloudFormation name: `StepFunctions`
- CloudTrail event source: `states.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Describe` (6), `Delete` (4), `Create` (3), `Send` (3), `Update` (3), `Get` (2), `Start` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateActivity`, `CreateStateMachine`, `CreateStateMachineAlias`, `DeleteActivity`, `DeleteStateMachine`, `DeleteStateMachineAlias`, `DeleteStateMachineVersion`, `StartExecution`, `StartSyncExecution`, `StopExecution`, `TagResource`, `UntagResource`, `UpdateMapRun`, `UpdateStateMachine`, `UpdateStateMachineAlias`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeActivity`, `DescribeExecution`, `DescribeMapRun`, `DescribeStateMachine`, `DescribeStateMachineAlias`, `DescribeStateMachineForExecution`, `GetActivityTask`, `GetExecutionHistory`, `ListActivities`, `ListExecutions`, `ListMapRuns`, `ListStateMachineAliases`, `ListStateMachineVersions`, `ListStateMachines`, `ListTagsForResource`, `ValidateStateMachineDefinition`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeExecution`, `DescribeStateMachineForExecution`, `GetActivityTask`, `GetExecutionHistory`, `ListExecutions`, `RedriveExecution`, `SendTaskFailure`, `SendTaskHeartbeat`, `SendTaskSuccess`, `StartExecution`, `StartSyncExecution`, `StopExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 37 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `Lambda`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html
- https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html
- https://docs.aws.amazon.com/step-functions/latest/dg/choosing-workflow-type.html

Research outcomes:
- A normal service integration request/response task waits only for the integrated API call's HTTP response, then advances to the next state.
- `.sync` service integrations wait for the downstream job to complete before advancing. Step Functions may use EventBridge events and polling in the same account, and polling only for cross-account jobs.
- If a `.sync` task is aborted, Step Functions attempts best-effort cancellation of the downstream job, but cancellation can fail because of IAM permission gaps or service issues.
- `.waitForTaskToken` integrations pause execution until SendTaskSuccess or SendTaskFailure is called with the token.
- Task tokens must be returned by principals in the same AWS account. Timeout creates a new random token for retried/waiting tasks.
- Callback tasks can wait up to the service quota unless heartbeat timeouts are configured; missing heartbeats fail with States.Timeout.

Parity implications:
- ASL execution must distinguish immediate request/response, `.sync`, and callback-token task patterns.
- Execution history should record task scheduling, downstream call, wait, callback, timeout, cancellation, and failure events.
- Service integrations require cross-service API invocation, IAM-like permission validation, and task-token lifecycle state.

## Cross-Service Integration Gaps

- **`sfn-execution`** ( primary ): `start_execution()` creates and stores an execution record but does not interpret or execute the state machine definition. Optimised service integrations ( Lambda, DynamoDB `GetItem`/`PutItem`/`UpdateItem`/`DeleteItem`, SQS, SNS, EventBridge, ECS, Batch, API Gateway ) are not invoked. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `sfn-execution` ).
- **Secondary target of** `eventbridge-targets`, `eventbridge-pipes` — those gaps cite Step Functions as a target service. See the same TODO section.

## Operation Groups

### List

- Operations: `ListActivities`, `ListExecutions`, `ListMapRuns`, `ListStateMachineAliases`, `ListStateMachines`, `ListStateMachineVersions`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `stateMachineArn`

### Describe

- Operations: `DescribeActivity`, `DescribeExecution`, `DescribeMapRun`, `DescribeStateMachine`, `DescribeStateMachineAlias`, `DescribeStateMachineForExecution`
- Common required input members in this group: `executionArn`

### Delete

- Operations: `DeleteActivity`, `DeleteStateMachine`, `DeleteStateMachineAlias`, `DeleteStateMachineVersion`
- Common required input members in this group: -

### Create

- Operations: `CreateActivity`, `CreateStateMachine`, `CreateStateMachineAlias`
- Traits: `idempotent` (2)
- Common required input members in this group: `name`

### Send

- Operations: `SendTaskFailure`, `SendTaskHeartbeat`, `SendTaskSuccess`
- Common required input members in this group: `taskToken`

### Update

- Operations: `UpdateMapRun`, `UpdateStateMachine`, `UpdateStateMachineAlias`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetActivityTask`, `GetExecutionHistory`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartExecution`, `StartSyncExecution`
- Traits: `idempotent` (1)
- Common required input members in this group: `stateMachineArn`

### Publish

- Operations: `PublishStateMachineVersion`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Redrive

- Operations: `RedriveExecution`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Stop

- Operations: `StopExecution`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Test

- Operations: `TestState`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Validate

- Operations: `ValidateStateMachineDefinition`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateActivity` | `-` | `idempotent` | `name` | - | `CreateActivityOutput` | `ActivityAlreadyExists`, `ActivityLimitExceeded`, `InvalidEncryptionConfiguration`, `InvalidName`, `KmsAccessDeniedException`, `KmsThrottlingException`, `TooManyTags` | Creates an activity. An activity is a task that you write in any programming language and host on any machine that has access to Step Functions. Activities must poll Step Functions using the GetActivityTask API actio ... |
| `CreateStateMachine` | `-` | `idempotent` | `name`, `definition`, `roleArn` | - | `CreateStateMachineOutput` | `ConflictException`, `InvalidArn`, `InvalidDefinition`, `InvalidEncryptionConfiguration`, `InvalidLoggingConfiguration`, `InvalidName`, `InvalidTracingConfiguration`, `KmsAccessDeniedException`, `KmsThrottlingException`, `StateMachineAlreadyExists`, `StateMachineDeleting`, `StateMachineLimitExceeded`, `StateMachineTypeNotSupported`, `TooManyTags`, `ValidationException` | Creates a state machine. A state machine consists of a collection of states that can do work ( Task states), determine to which states to transition next ( Choice states), stop an execution with an error ( Fail state ... |
| `CreateStateMachineAlias` | `-` | - | `name`, `routingConfiguration` | - | `CreateStateMachineAliasOutput` | `ConflictException`, `InvalidArn`, `InvalidName`, `ResourceNotFound`, `ServiceQuotaExceededException`, `StateMachineDeleting`, `ValidationException` | Creates an alias for a state machine that points to one or two versions of the same state machine. You can set your application to call StartExecution with an alias and update the version the alias uses without chang ... |
| `DeleteActivity` | `-` | - | `activityArn` | - | `DeleteActivityOutput` | `InvalidArn` | Deletes an activity. |
| `DeleteStateMachine` | `-` | - | `stateMachineArn` | - | `DeleteStateMachineOutput` | `InvalidArn`, `ValidationException` | Deletes a state machine. This is an asynchronous operation. It sets the state machine's status to DELETING and begins the deletion process. A state machine is deleted only when all its executions are completed. On th ... |
| `DeleteStateMachineAlias` | `-` | - | `stateMachineAliasArn` | - | `DeleteStateMachineAliasOutput` | `ConflictException`, `InvalidArn`, `ResourceNotFound`, `ValidationException` | Deletes a state machine alias . After you delete a state machine alias, you can't use it to start executions. When you delete a state machine alias, Step Functions doesn't delete the state machine versions that alias ... |
| `DeleteStateMachineVersion` | `-` | - | `stateMachineVersionArn` | - | `DeleteStateMachineVersionOutput` | `ConflictException`, `InvalidArn`, `ValidationException` | Deletes a state machine version . After you delete a version, you can't call StartExecution using that version's ARN or use the version with a state machine alias . Deleting a state machine version won't terminate it ... |
| `DescribeActivity` | `-` | - | `activityArn` | - | `DescribeActivityOutput` | `ActivityDoesNotExist`, `InvalidArn` | Describes an activity. This operation is eventually consistent. The results are best effort and may not reflect very recent updates and changes. |
| `DescribeExecution` | `-` | - | `executionArn` | - | `DescribeExecutionOutput` | `ExecutionDoesNotExist`, `InvalidArn`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException` | Provides information about a state machine execution, such as the state machine associated with the execution, the execution input and output, and relevant execution metadata. If you've redriven an execution, you can ... |
| `DescribeMapRun` | `-` | - | `mapRunArn` | - | `DescribeMapRunOutput` | `InvalidArn`, `ResourceNotFound` | Provides information about a Map Run's configuration, progress, and results. If you've redriven a Map Run, this API action also returns information about the redrives of that Map Run. For more information, see Examin ... |
| `DescribeStateMachine` | `-` | - | `stateMachineArn` | - | `DescribeStateMachineOutput` | `InvalidArn`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException`, `StateMachineDoesNotExist` | Provides information about a state machine's definition, its IAM role Amazon Resource Name (ARN), and configuration. A qualified state machine ARN can either refer to a Distributed Map state defined within a state ma ... |
| `DescribeStateMachineAlias` | `-` | - | `stateMachineAliasArn` | - | `DescribeStateMachineAliasOutput` | `InvalidArn`, `ResourceNotFound`, `ValidationException` | Returns details about a state machine alias . Related operations: CreateStateMachineAlias ListStateMachineAliases UpdateStateMachineAlias DeleteStateMachineAlias |
| `DescribeStateMachineForExecution` | `-` | - | `executionArn` | - | `DescribeStateMachineForExecutionOutput` | `ExecutionDoesNotExist`, `InvalidArn`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException` | Provides information about a state machine's definition, its execution role ARN, and configuration. If a Map Run dispatched the execution, this action returns the Map Run Amazon Resource Name (ARN) in the response. T ... |
| `GetActivityTask` | `-` | - | `activityArn` | - | `GetActivityTaskOutput` | `ActivityDoesNotExist`, `ActivityWorkerLimitExceeded`, `InvalidArn`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException` | Used by workers to retrieve a task (with the specified activity ARN) which has been scheduled for execution by a running state machine. This initiates a long poll, where the service holds the HTTP connection open and ... |
| `GetExecutionHistory` | `-` | `paginated` | `executionArn` | - | `GetExecutionHistoryOutput` | `ExecutionDoesNotExist`, `InvalidArn`, `InvalidToken`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException` | Returns the history of the specified execution as a list of events. By default, the results are returned in ascending order of the timeStamp of the events. Use the reverseOrder parameter to get the latest events firs ... |
| `ListActivities` | `-` | `paginated` | - | - | `ListActivitiesOutput` | `InvalidToken` | Lists the existing activities. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve t ... |
| `ListExecutions` | `-` | `paginated` | - | - | `ListExecutionsOutput` | `InvalidArn`, `InvalidToken`, `ResourceNotFound`, `StateMachineDoesNotExist`, `StateMachineTypeNotSupported`, `ValidationException` | Lists all executions of a state machine or a Map Run. You can list all executions related to a state machine by specifying a state machine Amazon Resource Name (ARN), or those related to a Map Run by specifying a Map ... |
| `ListMapRuns` | `-` | `paginated` | `executionArn` | - | `ListMapRunsOutput` | `ExecutionDoesNotExist`, `InvalidArn`, `InvalidToken` | Lists all Map Runs that were started by a given state machine execution. Use this API action to obtain Map Run ARNs, and then call DescribeMapRun to obtain more information, if needed. |
| `ListStateMachineAliases` | `-` | - | `stateMachineArn` | - | `ListStateMachineAliasesOutput` | `InvalidArn`, `InvalidToken`, `ResourceNotFound`, `StateMachineDeleting`, `StateMachineDoesNotExist` | Lists aliases for a specified state machine ARN. Results are sorted by time, with the most recently created aliases listed first. To list aliases that reference a state machine version , you can specify the version A ... |
| `ListStateMachines` | `-` | `paginated` | - | - | `ListStateMachinesOutput` | `InvalidToken` | Lists the existing state machines. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrie ... |
| `ListStateMachineVersions` | `-` | - | `stateMachineArn` | - | `ListStateMachineVersionsOutput` | `InvalidArn`, `InvalidToken`, `ValidationException` | Lists versions for the specified state machine Amazon Resource Name (ARN). The results are sorted in descending order of the version creation time. If nextToken is returned, there are more results available. The valu ... |
| `ListTagsForResource` | `-` | - | `resourceArn` | - | `ListTagsForResourceOutput` | `InvalidArn`, `ResourceNotFound` | List tags for a given resource. Tags may only contain Unicode letters, digits, white space, or these symbols: _ . : / = + - @ . |
| `PublishStateMachineVersion` | `-` | `idempotent` | `stateMachineArn` | - | `PublishStateMachineVersionOutput` | `ConflictException`, `InvalidArn`, `ServiceQuotaExceededException`, `StateMachineDeleting`, `StateMachineDoesNotExist`, `ValidationException` | Creates a version from the current revision of a state machine. Use versions to create immutable snapshots of your state machine. You can start executions from versions either directly or with an alias. To create an ... |
| `RedriveExecution` | `-` | `idempotent`, `idempotency-token` | `executionArn` | `clientToken` | `RedriveExecutionOutput` | `ExecutionDoesNotExist`, `ExecutionLimitExceeded`, `ExecutionNotRedrivable`, `InvalidArn`, `ValidationException` | Restarts unsuccessful executions of Standard workflows that didn't complete successfully in the last 14 days. These include failed, aborted, or timed out executions. When you redrive an execution, it continues the fa ... |
| `SendTaskFailure` | `-` | - | `taskToken` | - | `SendTaskFailureOutput` | `InvalidToken`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException`, `TaskDoesNotExist`, `TaskTimedOut` | Used by activity workers, Task states using the callback pattern, and optionally Task states using the job run pattern to report that the task identified by the taskToken failed. For an execution with encryption enab ... |
| `SendTaskHeartbeat` | `-` | - | `taskToken` | - | `SendTaskHeartbeatOutput` | `InvalidToken`, `TaskDoesNotExist`, `TaskTimedOut` | Used by activity workers and Task states using the callback pattern, and optionally Task states using the job run pattern to report to Step Functions that the task represented by the specified taskToken is still maki ... |
| `SendTaskSuccess` | `-` | - | `taskToken`, `output` | - | `SendTaskSuccessOutput` | `InvalidOutput`, `InvalidToken`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException`, `TaskDoesNotExist`, `TaskTimedOut` | Used by activity workers, Task states using the callback pattern, and optionally Task states using the job run pattern to report that the task identified by the taskToken completed successfully. |
| `StartExecution` | `-` | `idempotent` | `stateMachineArn` | - | `StartExecutionOutput` | `ExecutionAlreadyExists`, `ExecutionLimitExceeded`, `InvalidArn`, `InvalidExecutionInput`, `InvalidName`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException`, `StateMachineDeleting`, `StateMachineDoesNotExist`, `ValidationException` | Starts a state machine execution. A qualified state machine ARN can either refer to a Distributed Map state defined within a state machine, a version ARN, or an alias ARN. The following are some examples of qualified ... |
| `StartSyncExecution` | `-` | - | `stateMachineArn` | - | `StartSyncExecutionOutput` | `InvalidArn`, `InvalidExecutionInput`, `InvalidName`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException`, `StateMachineDeleting`, `StateMachineDoesNotExist`, `StateMachineTypeNotSupported` | Starts a Synchronous Express state machine execution. StartSyncExecution is not available for STANDARD workflows. StartSyncExecution will return a 200 OK response, even if your execution fails, because the status cod ... |
| `StopExecution` | `-` | - | `executionArn` | - | `StopExecutionOutput` | `ExecutionDoesNotExist`, `InvalidArn`, `KmsAccessDeniedException`, `KmsInvalidStateException`, `KmsThrottlingException`, `ValidationException` | Stops an execution. This API action is not supported by EXPRESS state machines. For an execution with encryption enabled, Step Functions will encrypt the error and cause fields using the KMS key for the execution rol ... |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InvalidArn`, `ResourceNotFound`, `TooManyTags` | Add a tag to a Step Functions resource. An array of key-value pairs. For more information, see Using Cost Allocation Tags in the Amazon Web Services Billing and Cost Management User Guide , and Controlling Access Usi ... |
| `TestState` | `-` | - | `definition` | - | `TestStateOutput` | `InvalidArn`, `InvalidDefinition`, `InvalidExecutionInput`, `ValidationException` | Accepts the definition of a single state and executes it. You can test a state without creating a state machine or updating an existing state machine. Using this API, you can test the following: A state's input and o ... |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InvalidArn`, `ResourceNotFound` | Remove a tag from a Step Functions resource |
| `UpdateMapRun` | `-` | - | `mapRunArn` | - | `UpdateMapRunOutput` | `InvalidArn`, `ResourceNotFound`, `ValidationException` | Updates an in-progress Map Run's configuration to include changes to the settings that control maximum concurrency and Map Run failure. |
| `UpdateStateMachine` | `-` | `idempotent` | `stateMachineArn` | - | `UpdateStateMachineOutput` | `ConflictException`, `InvalidArn`, `InvalidDefinition`, `InvalidEncryptionConfiguration`, `InvalidLoggingConfiguration`, `InvalidTracingConfiguration`, `KmsAccessDeniedException`, `KmsThrottlingException`, `MissingRequiredParameter`, `ServiceQuotaExceededException`, `StateMachineDeleting`, `StateMachineDoesNotExist`, `ValidationException` | Updates an existing state machine by modifying its definition , roleArn , loggingConfiguration , or EncryptionConfiguration . Running executions will continue to use the previous definition and roleArn . You must inc ... |
| `UpdateStateMachineAlias` | `-` | - | `stateMachineAliasArn` | - | `UpdateStateMachineAliasOutput` | `ConflictException`, `InvalidArn`, `ResourceNotFound`, `StateMachineDeleting`, `ValidationException` | Updates the configuration of an existing state machine alias by modifying its description or routingConfiguration . You must specify at least one of the description or routingConfiguration parameters to update a stat ... |
| `ValidateStateMachineDefinition` | `-` | - | `definition` | - | `ValidateStateMachineDefinitionOutput` | `ValidationException` | Validates the syntax of a state machine definition specified in Amazon States Language (ASL), a JSON-based, structured language. You can validate that a state machine definition is correct without creating a state ma ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ActivityAlreadyExists` | `structure` | message | Activity already exists. EncryptionConfiguration may not be updated. |
| `ActivityDoesNotExist` | `structure` | message | The specified activity does not exist. |
| `ActivityLimitExceeded` | `structure` | message | The maximum number of activities has been reached. Existing activities must be deleted before a new activity can be created. |
| `ActivityWorkerLimitExceeded` | `structure` | message | The maximum number of workers concurrently polling for activity tasks has been reached. |
| `ConflictException` | `structure` | message | Updating or deleting a resource can cause an inconsistent state. This error occurs when there're concurrent requests for DeleteStateMachineVersion , Publish ... |
| `ExecutionAlreadyExists` | `structure` | message | The execution has the same name as another execution (but a different input ). Executions with the same name and input are considered idempotent. |
| `ExecutionDoesNotExist` | `structure` | message | The specified execution does not exist. |
| `ExecutionLimitExceeded` | `structure` | message | The maximum number of running executions has been reached. Running executions must end or be stopped before a new execution can be started. |
| `ExecutionNotRedrivable` | `structure` | message | The execution Amazon Resource Name (ARN) that you specified for executionArn cannot be redriven. |
| `InvalidArn` | `structure` | message | The provided Amazon Resource Name (ARN) is not valid. |
| `InvalidDefinition` | `structure` | message | The provided Amazon States Language definition is not valid. |
| `InvalidEncryptionConfiguration` | `structure` | message | Received when encryptionConfiguration is specified but various conditions exist which make the configuration invalid. For example, if type is set to CUSTOME ... |
| `InvalidExecutionInput` | `structure` | message | The provided JSON input data is not valid. |
| `InvalidLoggingConfiguration` | `structure` | message | Configuration is not valid. |
| `InvalidName` | `structure` | message | The provided name is not valid. |
| `InvalidOutput` | `structure` | message | The provided JSON output data is not valid. |
| `InvalidToken` | `structure` | message | The provided token is not valid. |
| `InvalidTracingConfiguration` | `structure` | message | Your tracingConfiguration key does not match, or enabled has not been set to true or false . |
| `KmsAccessDeniedException` | `structure` | message | Either your KMS key policy or API caller does not have the required permissions. |
| `KmsInvalidStateException` | `structure` | kmsKeyState, message | The KMS key is not in valid state, for example: Disabled or Deleted. |
| `KmsThrottlingException` | `structure` | message | Received when KMS returns ThrottlingException for a KMS call that Step Functions makes on behalf of the caller. |
| `MissingRequiredParameter` | `structure` | message | Request is missing a required parameter. This error occurs if both definition and roleArn are not specified. |
| `ResourceNotFound` | `structure` | message, resourceName | Could not find the referenced resource. |
| `ServiceQuotaExceededException` | `structure` | message | The request would cause a service quota to be exceeded. HTTP Status Code: 402 |
| `StateMachineAlreadyExists` | `structure` | message | A state machine with the same name but a different definition or role ARN already exists. |
| `StateMachineDeleting` | `structure` | message | The specified state machine is being deleted. |
| `StateMachineDoesNotExist` | `structure` | message | The specified state machine does not exist. |
| `StateMachineLimitExceeded` | `structure` | message | The maximum number of state machines has been reached. Existing state machines must be deleted before a new state machine can be created. |
| `StateMachineTypeNotSupported` | `structure` | message | State machine type is not supported. |
| `TaskDoesNotExist` | `structure` | message | The activity does not exist. |
| `TaskTimedOut` | `structure` | message | The task token has either expired or the task associated with the token has already been closed. |
| `TooManyTags` | `structure` | message, resourceName | You've exceeded the number of tags allowed for a resource. See the Limits Topic in the Step Functions Developer Guide. |
| `ValidationException` | `structure` | message, reason | The input does not satisfy the constraints specified by an Amazon Web Services service. |
| `CreateActivityInput` | `structure` | name, tags, encryptionConfiguration | - |
| `CreateActivityOutput` | `structure` | activityArn, creationDate | - |
| `CreateStateMachineInput` | `structure` | name, definition, roleArn, type, loggingConfiguration, tags, tracingConfiguration, publish, versionDescription, encryptionConfiguration | - |
| `CreateStateMachineOutput` | `structure` | stateMachineArn, creationDate, stateMachineVersionArn | - |
| `CreateStateMachineAliasInput` | `structure` | description, name, routingConfiguration | - |
| `CreateStateMachineAliasOutput` | `structure` | stateMachineAliasArn, creationDate | - |
| `DeleteActivityInput` | `structure` | activityArn | - |
| `EncryptionType` | `enum` | AWS_OWNED_KEY, CUSTOMER_MANAGED_KMS_KEY | - |
| `ExecutionRedriveFilter` | `enum` | REDRIVEN, NOT_REDRIVEN | - |
| `ExecutionRedriveStatus` | `enum` | REDRIVABLE, NOT_REDRIVABLE, REDRIVABLE_BY_MAP_RUN | - |
| `ExecutionStatus` | `enum` | RUNNING, SUCCEEDED, FAILED, TIMED_OUT, ABORTED, PENDING_REDRIVE | - |
| `HistoryEventType` | `enum` | ActivityFailed, ActivityScheduled, ActivityScheduleFailed, ActivityStarted, ActivitySucceeded, ActivityTimedOut, ChoiceStateEntered, ChoiceStateExited, ExecutionAborted, ExecutionFailed, ExecutionStarted, ExecutionSucceeded, ... (+50) | - |
| `IncludedData` | `enum` | ALL_DATA, METADATA_ONLY | - |
| `InspectionLevel` | `enum` | INFO, DEBUG, TRACE | - |
| `KmsKeyState` | `enum` | DISABLED, PENDING_DELETION, PENDING_IMPORT, UNAVAILABLE, CREATING | - |
| `LogLevel` | `enum` | ALL, ERROR, FATAL, OFF | - |
| `MapRunStatus` | `enum` | RUNNING, SUCCEEDED, FAILED, ABORTED | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/rule-evaluator-and-validator-crates.md, .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: full distillation.

### ASL Validation

- `winterbaume-sfn-asl-eval` owns ASL parsing and validation. Handler work should call the validator and map diagnostics into `ValidateStateMachineDefinition` output shapes rather than adding semantic checks inline.
- Review-backed ASL gaps to preserve as validator work include modern `Map` states using `ItemProcessor`, explicit diagnostics for missing `Type`, and invalid states that set both `End` and `Next`.
- `ValidateStateMachineDefinition` parity depends on diagnostic shape, severity, and location quality as well as pass/fail behaviour. Keep parser and validator changes testable without the HTTP service.

### Service Integrations

- Direct service integrations should stay scoped to AWS-documented action sets. The optimised DynamoDB integration is only `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem`, not general DynamoDB API execution.
- High-value orchestration paths include Lambda, SQS, SNS, EventBridge, API Gateway, ECS, and Batch. Tests should model the Step Functions contract for each path rather than calling the target service as a generic SDK proxy.
- For API Gateway, keep both directions distinct: Step Functions can invoke API Gateway, and API Gateway can start executions. They have different request/response and error-shaping rules.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

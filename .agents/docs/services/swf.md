# Amazon Simple Workflow Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Simple Workflow Service The Amazon Simple Workflow Service (Amazon SWF) makes it easy to build applications that use Amazon's cloud to coordinate work across distributed components. In Amazon SWF, a task represents a logical unit of work that is performed by a component of your workflow. Coordinating tasks in a workflow involves managing intertask dependencies, scheduling, and concurrency in accordance with the logical flow of the application. Amazon SWF gives you full control over implementing tasks and coordinating them without worrying about underlying complexities such as tracking their progress and maintaining their state. This documentation serves as reference only.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Simple Workflow Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Simple Workflow Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `List`, `Count`, `Describe`, `Respond`, `Deprecate` operation families, including `ListActivityTypes`, `ListClosedWorkflowExecutions`, `ListDomains`, `ListOpenWorkflowExecutions`, `CountClosedWorkflowExecutions`, `CountOpenWorkflowExecutions`.

## Service Identity and Protocol

- AWS model slug: `swf`
- AWS SDK for Rust slug: `swf`
- Model version: `2012-01-25`
- Model file: `vendor/api-models-aws/models/swf/service/2012-01-25/swf-2012-01-25.json`
- SDK ID: `SWF`
- Endpoint prefix: `swf`
- ARN namespace: `swf`
- CloudFormation name: `SWF`
- CloudTrail event source: `swf.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Count` (4), `Describe` (4), `Respond` (4), `Deprecate` (3), `Register` (3), `Undeprecate` (3), `Delete` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteActivityType`, `DeleteWorkflowType`, `RegisterActivityType`, `RegisterDomain`, `RegisterWorkflowType`, `StartWorkflowExecution`, `TagResource`, `TerminateWorkflowExecution`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeActivityType`, `DescribeDomain`, `DescribeWorkflowExecution`, `DescribeWorkflowType`, `GetWorkflowExecutionHistory`, `ListActivityTypes`, `ListClosedWorkflowExecutions`, `ListDomains`, `ListOpenWorkflowExecutions`, `ListTagsForResource`, `ListWorkflowTypes`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CountClosedWorkflowExecutions`, `CountOpenWorkflowExecutions`, `CountPendingActivityTasks`, `CountPendingDecisionTasks`, `DescribeWorkflowExecution`, `GetWorkflowExecutionHistory`, `ListClosedWorkflowExecutions`, `ListOpenWorkflowExecutions`, `PollForActivityTask`, `PollForDecisionTask`, `RecordActivityTaskHeartbeat`, `RequestCancelWorkflowExecution`, `RespondActivityTaskCanceled`, `RespondActivityTaskCompleted`, `RespondActivityTaskFailed`, `RespondDecisionTaskCompleted`, `SignalWorkflowExecution`, `StartWorkflowExecution`, `TerminateWorkflowExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `ECS`.

## Operation Groups

### List

- Operations: `ListActivityTypes`, `ListClosedWorkflowExecutions`, `ListDomains`, `ListOpenWorkflowExecutions`, `ListTagsForResource`, `ListWorkflowTypes`
- Traits: `paginated` (5)
- Common required input members in this group: `domain`, `registrationStatus`

### Count

- Operations: `CountClosedWorkflowExecutions`, `CountOpenWorkflowExecutions`, `CountPendingActivityTasks`, `CountPendingDecisionTasks`
- Common required input members in this group: `domain`, `taskList`

### Describe

- Operations: `DescribeActivityType`, `DescribeDomain`, `DescribeWorkflowExecution`, `DescribeWorkflowType`
- Common required input members in this group: `domain`

### Respond

- Operations: `RespondActivityTaskCanceled`, `RespondActivityTaskCompleted`, `RespondActivityTaskFailed`, `RespondDecisionTaskCompleted`
- Common required input members in this group: `taskToken`

### Deprecate

- Operations: `DeprecateActivityType`, `DeprecateDomain`, `DeprecateWorkflowType`
- Common required input members in this group: `domain`

### Register

- Operations: `RegisterActivityType`, `RegisterDomain`, `RegisterWorkflowType`
- Common required input members in this group: `domain`, `name`, `version`

### Undeprecate

- Operations: `UndeprecateActivityType`, `UndeprecateDomain`, `UndeprecateWorkflowType`
- Common required input members in this group: `domain`

### Delete

- Operations: `DeleteActivityType`, `DeleteWorkflowType`
- Common required input members in this group: `domain`

### Poll

- Operations: `PollForActivityTask`, `PollForDecisionTask`
- Traits: `paginated` (1)
- Common required input members in this group: `domain`, `taskList`

### Get

- Operations: `GetWorkflowExecutionHistory`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Record

- Operations: `RecordActivityTaskHeartbeat`
- Common required input members in this group: -

### Request

- Operations: `RequestCancelWorkflowExecution`
- Common required input members in this group: -

### Signal

- Operations: `SignalWorkflowExecution`
- Common required input members in this group: -

### Start

- Operations: `StartWorkflowExecution`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Terminate

- Operations: `TerminateWorkflowExecution`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CountClosedWorkflowExecutions` | `-` | - | `domain` | - | `WorkflowExecutionCount` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns the number of closed workflow executions within the given domain that meet the specified filtering criteria. This operation is eventually consistent. The results are best effort and may not exactly reflect re ... |
| `CountOpenWorkflowExecutions` | `-` | - | `domain`, `startTimeFilter` | - | `WorkflowExecutionCount` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns the number of open workflow executions within the given domain that meet the specified filtering criteria. This operation is eventually consistent. The results are best effort and may not exactly reflect rece ... |
| `CountPendingActivityTasks` | `-` | - | `domain`, `taskList` | - | `PendingTaskCount` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns the estimated number of activity tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no activity task was ever scheduled ... |
| `CountPendingDecisionTasks` | `-` | - | `domain`, `taskList` | - | `PendingTaskCount` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns the estimated number of decision tasks in the specified task list. The count returned is an approximation and isn't guaranteed to be exact. If you specify a task list that no decision task was ever scheduled ... |
| `DeleteActivityType` | `-` | - | `domain`, `activityType` | - | `Unit` | `OperationNotPermittedFault`, `TypeNotDeprecatedFault`, `UnknownResourceFault` | Deletes the specified activity type . Note: Prior to deletion, activity types must first be deprecated . After an activity type has been deleted, you cannot schedule new activities of that type. Activities that start ... |
| `DeleteWorkflowType` | `-` | - | `domain`, `workflowType` | - | `Unit` | `OperationNotPermittedFault`, `TypeNotDeprecatedFault`, `UnknownResourceFault` | Deletes the specified workflow type . Note: Prior to deletion, workflow types must first be deprecated . After a workflow type has been deleted, you cannot create new executions of that type. Executions that started ... |
| `DeprecateActivityType` | `-` | - | `domain`, `activityType` | - | `Unit` | `OperationNotPermittedFault`, `TypeDeprecatedFault`, `UnknownResourceFault` | Deprecates the specified activity type . After an activity type has been deprecated, you cannot create new tasks of that activity type. Tasks of this type that were scheduled before the type was deprecated continue t ... |
| `DeprecateDomain` | `-` | - | `name` | - | `Unit` | `DomainDeprecatedFault`, `OperationNotPermittedFault`, `UnknownResourceFault` | Deprecates the specified domain. After a domain has been deprecated it cannot be used to create new workflow executions or register new types. However, you can still use visibility actions on this domain. Deprecating ... |
| `DeprecateWorkflowType` | `-` | - | `domain`, `workflowType` | - | `Unit` | `OperationNotPermittedFault`, `TypeDeprecatedFault`, `UnknownResourceFault` | Deprecates the specified workflow type . After a workflow type has been deprecated, you cannot create new executions of that type. Executions that were started before the type was deprecated continues to run. A depre ... |
| `DescribeActivityType` | `-` | - | `domain`, `activityType` | - | `ActivityTypeDetail` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns information about the specified activity type. This includes configuration settings provided when the type was registered and other general information about the type. Access Control You can use IAM policies ... |
| `DescribeDomain` | `-` | - | `name` | - | `DomainDetail` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns information about the specified domain, including description and status. Access Control You can use IAM policies to control this action's access to Amazon SWF resources as follows: Use a Resource element wit ... |
| `DescribeWorkflowExecution` | `-` | - | `domain`, `execution` | - | `WorkflowExecutionDetail` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns information about the specified workflow execution including its type and some statistics. This operation is eventually consistent. The results are best effort and may not exactly reflect recent updates and c ... |
| `DescribeWorkflowType` | `-` | - | `domain`, `workflowType` | - | `WorkflowTypeDetail` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns information about the specified workflow type . This includes configuration settings specified when the type was registered and other information such as creation date, current status, etc. Access Control You ... |
| `GetWorkflowExecutionHistory` | `-` | `paginated` | `domain`, `execution` | - | `History` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns the history of the specified workflow execution. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call. This oper ... |
| `ListActivityTypes` | `-` | `paginated` | `domain`, `registrationStatus` | - | `ActivityTypeInfos` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns information about all activities registered in the specified domain that match the specified name and registration status. The result includes information like creation date, current status of the activity, e ... |
| `ListClosedWorkflowExecutions` | `-` | `paginated` | `domain` | - | `WorkflowExecutionInfos` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns a list of closed workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPag ... |
| `ListDomains` | `-` | `paginated` | `registrationStatus` | - | `DomainInfos` | `OperationNotPermittedFault` | Returns the list of domains registered in the account. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageToken returned by the initial call. This operat ... |
| `ListOpenWorkflowExecutions` | `-` | `paginated` | `domain`, `startTimeFilter` | - | `WorkflowExecutionInfos` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns a list of open workflow executions in the specified domain that meet the filtering criteria. The results may be split into multiple pages. To retrieve subsequent pages, make the call again using the nextPageT ... |
| `ListTagsForResource` | `-` | - | `resourceArn` | - | `ListTagsForResourceOutput` | `LimitExceededFault`, `OperationNotPermittedFault`, `UnknownResourceFault` | List tags for a given domain. |
| `ListWorkflowTypes` | `-` | `paginated` | `domain`, `registrationStatus` | - | `WorkflowTypeInfos` | `OperationNotPermittedFault`, `UnknownResourceFault` | Returns information about workflow types in the specified domain. The results may be split into multiple pages that can be retrieved by making the call repeatedly. Access Control You can use IAM policies to control t ... |
| `PollForActivityTask` | `-` | - | `domain`, `taskList` | - | `ActivityTask` | `LimitExceededFault`, `OperationNotPermittedFault`, `UnknownResourceFault` | Used by workers to get an ActivityTask from the specified activity taskList . This initiates a long poll, where the service holds the HTTP connection open and responds as soon as a task becomes available. The maximum ... |
| `PollForDecisionTask` | `-` | `paginated` | `domain`, `taskList` | - | `DecisionTask` | `LimitExceededFault`, `OperationNotPermittedFault`, `UnknownResourceFault` | Used by deciders to get a DecisionTask from the specified decision taskList . A decision task may be returned for any open workflow execution that is using the specified task list. The task includes a paginated view ... |
| `RecordActivityTaskHeartbeat` | `-` | - | `taskToken` | - | `ActivityTaskStatus` | `OperationNotPermittedFault`, `UnknownResourceFault` | Used by activity workers to report to the service that the ActivityTask represented by the specified taskToken is still making progress. The worker can also specify details of the progress, for example percent comple ... |
| `RegisterActivityType` | `-` | - | `domain`, `name`, `version` | - | `Unit` | `LimitExceededFault`, `OperationNotPermittedFault`, `TypeAlreadyExistsFault`, `UnknownResourceFault` | Registers a new activity type along with its configuration settings in the specified domain. A TypeAlreadyExists fault is returned if the type already exists in the domain. You cannot change any configuration setting ... |
| `RegisterDomain` | `-` | - | `name`, `workflowExecutionRetentionPeriodInDays` | - | `Unit` | `DomainAlreadyExistsFault`, `LimitExceededFault`, `OperationNotPermittedFault`, `TooManyTagsFault` | Registers a new domain. Access Control You can use IAM policies to control this action's access to Amazon SWF resources as follows: You cannot use an IAM policy to control domain access for this action. The name of t ... |
| `RegisterWorkflowType` | `-` | - | `domain`, `name`, `version` | - | `Unit` | `LimitExceededFault`, `OperationNotPermittedFault`, `TypeAlreadyExistsFault`, `UnknownResourceFault` | Registers a new workflow type and its configuration settings in the specified domain. The retention period for the workflow history is set by the RegisterDomain action. If the type already exists, then a TypeAlreadyE ... |
| `RequestCancelWorkflowExecution` | `-` | - | `domain`, `workflowId` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Records a WorkflowExecutionCancelRequested event in the currently running workflow execution identified by the given domain, workflowId, and runId. This logically requests the cancellation of the workflow execution a ... |
| `RespondActivityTaskCanceled` | `-` | - | `taskToken` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Used by workers to tell the service that the ActivityTask identified by the taskToken was successfully canceled. Additional details can be provided using the details argument. These details (if provided) appear in th ... |
| `RespondActivityTaskCompleted` | `-` | - | `taskToken` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Used by workers to tell the service that the ActivityTask identified by the taskToken completed successfully with a result (if provided). The result appears in the ActivityTaskCompleted event in the workflow history. ... |
| `RespondActivityTaskFailed` | `-` | - | `taskToken` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Used by workers to tell the service that the ActivityTask identified by the taskToken has failed with reason (if specified). The reason and details appear in the ActivityTaskFailed event added to the workflow history ... |
| `RespondDecisionTaskCompleted` | `-` | - | `taskToken` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Used by deciders to tell the service that the DecisionTask identified by the taskToken has successfully completed. The decisions argument specifies the list of decisions made while processing the task. A DecisionTask ... |
| `SignalWorkflowExecution` | `-` | - | `domain`, `workflowId`, `signalName` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Records a WorkflowExecutionSignaled event in the workflow execution history and creates a decision task for the workflow execution identified by the given domain, workflowId and runId. The event is recorded with the ... |
| `StartWorkflowExecution` | `-` | - | `domain`, `workflowId`, `workflowType` | - | `Run` | `DefaultUndefinedFault`, `LimitExceededFault`, `OperationNotPermittedFault`, `TypeDeprecatedFault`, `UnknownResourceFault`, `WorkflowExecutionAlreadyStartedFault` | Starts an execution of the workflow type in the specified domain using the provided workflowId and input data. This action returns the newly started workflow execution. Access Control You can use IAM policies to cont ... |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `Unit` | `LimitExceededFault`, `OperationNotPermittedFault`, `TooManyTagsFault`, `UnknownResourceFault` | Add a tag to a Amazon SWF domain. Amazon SWF supports a maximum of 50 tags per resource. |
| `TerminateWorkflowExecution` | `-` | - | `domain`, `workflowId` | - | `Unit` | `OperationNotPermittedFault`, `UnknownResourceFault` | Records a WorkflowExecutionTerminated event and forces closure of the workflow execution identified by the given domain, runId, and workflowId. The child policy, registered with the workflow type or specified when st ... |
| `UndeprecateActivityType` | `-` | - | `domain`, `activityType` | - | `Unit` | `OperationNotPermittedFault`, `TypeAlreadyExistsFault`, `UnknownResourceFault` | Undeprecates a previously deprecated activity type . After an activity type has been undeprecated, you can create new tasks of that activity type. This operation is eventually consistent. The results are best effort ... |
| `UndeprecateDomain` | `-` | - | `name` | - | `Unit` | `DomainAlreadyExistsFault`, `OperationNotPermittedFault`, `UnknownResourceFault` | Undeprecates a previously deprecated domain. After a domain has been undeprecated it can be used to create new workflow executions or register new types. This operation is eventually consistent. The results are best ... |
| `UndeprecateWorkflowType` | `-` | - | `domain`, `workflowType` | - | `Unit` | `OperationNotPermittedFault`, `TypeAlreadyExistsFault`, `UnknownResourceFault` | Undeprecates a previously deprecated workflow type . After a workflow type has been undeprecated, you can create new executions of that type. This operation is eventually consistent. The results are best effort and m ... |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `Unit` | `LimitExceededFault`, `OperationNotPermittedFault`, `UnknownResourceFault` | Remove a tag from a Amazon SWF domain. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DefaultUndefinedFault` | `structure` | message | The StartWorkflowExecution API action was called without the required parameters set. Some workflow execution parameters, such as the decision taskList , mu ... |
| `DomainAlreadyExistsFault` | `structure` | message | Returned if the domain already exists. You may get this fault if you are registering a domain that is either already registered or deprecated, or if you und ... |
| `DomainDeprecatedFault` | `structure` | message | Returned when the specified domain has been deprecated. |
| `LimitExceededFault` | `structure` | message | Returned by any operation if a system imposed limitation has been reached. To address this fault you should either clean up unused resources or increase the ... |
| `OperationNotPermittedFault` | `structure` | message | Returned when the caller doesn't have sufficient permissions to invoke the action. |
| `TooManyTagsFault` | `structure` | message | You've exceeded the number of tags allowed for a domain. |
| `TypeAlreadyExistsFault` | `structure` | message | Returned if the type already exists in the specified domain. You may get this fault if you are registering a type that is either already registered or depre ... |
| `TypeDeprecatedFault` | `structure` | message | Returned when the specified activity or workflow type was already deprecated. |
| `TypeNotDeprecatedFault` | `structure` | message | Returned when the resource type has not been deprecated. |
| `UnknownResourceFault` | `structure` | message | Returned when the named resource cannot be found with in the scope of this operation (region or domain). This could happen if the named resource was never c ... |
| `WorkflowExecutionAlreadyStartedFault` | `structure` | message | Returned by StartWorkflowExecution when an open execution with the same workflowId is already running in the specified domain. |
| `CountClosedWorkflowExecutionsInput` | `structure` | domain, startTimeFilter, closeTimeFilter, executionFilter, typeFilter, tagFilter, closeStatusFilter | - |
| `WorkflowExecutionCount` | `structure` | count, truncated | Contains the count of workflow executions returned from CountOpenWorkflowExecutions or CountClosedWorkflowExecutions |
| `CountOpenWorkflowExecutionsInput` | `structure` | domain, startTimeFilter, typeFilter, tagFilter, executionFilter | - |
| `CountPendingActivityTasksInput` | `structure` | domain, taskList | - |
| `PendingTaskCount` | `structure` | count, truncated | Contains the count of tasks in a task list. |
| `CountPendingDecisionTasksInput` | `structure` | domain, taskList | - |
| `DeleteActivityTypeInput` | `structure` | domain, activityType | - |
| `DeleteWorkflowTypeInput` | `structure` | domain, workflowType | - |
| `DeprecateActivityTypeInput` | `structure` | domain, activityType | - |
| `DeprecateDomainInput` | `structure` | name | - |
| `DeprecateWorkflowTypeInput` | `structure` | domain, workflowType | - |
| `DescribeActivityTypeInput` | `structure` | domain, activityType | - |
| `ActivityTypeDetail` | `structure` | typeInfo, configuration | Detailed information about an activity type. |
| `DescribeDomainInput` | `structure` | name | - |
| `DomainDetail` | `structure` | domainInfo, configuration | Contains details of a domain. |
| `DescribeWorkflowExecutionInput` | `structure` | domain, execution | - |
| `WorkflowExecutionDetail` | `structure` | executionInfo, executionConfiguration, openCounts, latestActivityTaskTimestamp, latestExecutionContext | Contains details about a workflow execution. |
| `DescribeWorkflowTypeInput` | `structure` | domain, workflowType | - |
| `WorkflowTypeDetail` | `structure` | typeInfo, configuration | Contains details about a workflow type. |
| `GetWorkflowExecutionHistoryInput` | `structure` | domain, execution, nextPageToken, maximumPageSize, reverseOrder | - |
| `History` | `structure` | events, nextPageToken | Paginated representation of a workflow history for a workflow execution. This is the up to date, complete and authoritative record of the events related to ... |
| `ListActivityTypesInput` | `structure` | domain, name, registrationStatus, nextPageToken, maximumPageSize, reverseOrder | - |
| `ActivityTypeInfos` | `structure` | typeInfos, nextPageToken | Contains a paginated list of activity type information structures. |
| `ListClosedWorkflowExecutionsInput` | `structure` | domain, startTimeFilter, closeTimeFilter, executionFilter, closeStatusFilter, typeFilter, tagFilter, nextPageToken, maximumPageSize, reverseOrder | - |
| `WorkflowExecutionInfos` | `structure` | executionInfos, nextPageToken | Contains a paginated list of information about workflow executions. |
| `ListDomainsInput` | `structure` | nextPageToken, registrationStatus, maximumPageSize, reverseOrder | - |
| `DomainInfos` | `structure` | domainInfos, nextPageToken | Contains a paginated collection of DomainInfo structures. |
| `ListOpenWorkflowExecutionsInput` | `structure` | domain, startTimeFilter, typeFilter, tagFilter, nextPageToken, maximumPageSize, reverseOrder, executionFilter | - |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ActivityTaskTimeoutType` | `enum` | START_TO_CLOSE, SCHEDULE_TO_START, SCHEDULE_TO_CLOSE, HEARTBEAT | - |
| `CancelTimerFailedCause` | `enum` | TIMER_ID_UNKNOWN, OPERATION_NOT_PERMITTED | - |
| `CancelWorkflowExecutionFailedCause` | `enum` | UNHANDLED_DECISION, OPERATION_NOT_PERMITTED | - |
| `ChildPolicy` | `enum` | TERMINATE, REQUEST_CANCEL, ABANDON | - |
| `CloseStatus` | `enum` | COMPLETED, FAILED, CANCELED, TERMINATED, CONTINUED_AS_NEW, TIMED_OUT | - |
| `CompleteWorkflowExecutionFailedCause` | `enum` | UNHANDLED_DECISION, OPERATION_NOT_PERMITTED | - |
| `ContinueAsNewWorkflowExecutionFailedCause` | `enum` | UNHANDLED_DECISION, WORKFLOW_TYPE_DEPRECATED, WORKFLOW_TYPE_DOES_NOT_EXIST, DEFAULT_EXECUTION_START_TO_CLOSE_TIMEOUT_UNDEFINED, DEFAULT_TASK_START_TO_CLOSE_TIMEOUT_UNDEFINED, DEFAULT_TASK_LIST_UNDEFINED, DEFAULT_CHILD_POLICY_UNDEFINED, CONTINUE_AS_NEW_WORKFLOW_EXECUTION_RATE_EXCEEDED, OPERATION_NOT_PERMITTED | - |
| `DecisionTaskTimeoutType` | `enum` | START_TO_CLOSE, SCHEDULE_TO_START | - |
| `DecisionType` | `enum` | ScheduleActivityTask, RequestCancelActivityTask, CompleteWorkflowExecution, FailWorkflowExecution, CancelWorkflowExecution, ContinueAsNewWorkflowExecution, RecordMarker, StartTimer, CancelTimer, SignalExternalWorkflowExecution, RequestCancelExternalWorkflowExecution, StartChildWorkflowExecution, ... (+1) | - |
| `EventType` | `enum` | WorkflowExecutionStarted, WorkflowExecutionCancelRequested, WorkflowExecutionCompleted, CompleteWorkflowExecutionFailed, WorkflowExecutionFailed, FailWorkflowExecutionFailed, WorkflowExecutionTimedOut, WorkflowExecutionCanceled, CancelWorkflowExecutionFailed, WorkflowExecutionContinuedAsNew, ContinueAsNewWorkflowExecutionFailed, WorkflowExecutionTerminated, ... (+42) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.

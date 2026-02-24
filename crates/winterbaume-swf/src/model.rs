//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-swf

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTask {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "activityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_token: Option<String>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityType {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecution {
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskStatus {
    #[serde(rename = "cancelRequested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_requested: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTypeDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ActivityTypeConfiguration>,
    #[serde(rename = "typeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_info: Option<ActivityTypeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTypeConfiguration {
    #[serde(rename = "defaultTaskHeartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_heartbeat_timeout: Option<String>,
    #[serde(rename = "defaultTaskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    #[serde(rename = "defaultTaskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    #[serde(rename = "defaultTaskScheduleToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_close_timeout: Option<String>,
    #[serde(rename = "defaultTaskScheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_start_timeout: Option<String>,
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskList {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTypeInfo {
    #[serde(rename = "activityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "deprecationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTypeInfos {
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "typeInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_infos: Option<Vec<ActivityTypeInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountClosedWorkflowExecutionsInput {
    #[serde(rename = "closeStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_status_filter: Option<CloseStatusFilter>,
    #[serde(rename = "closeTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time_filter: Option<ExecutionTimeFilter>,
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "executionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    #[serde(rename = "startTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_filter: Option<ExecutionTimeFilter>,
    #[serde(rename = "tagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    #[serde(rename = "typeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloseStatusFilter {
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionTimeFilter {
    #[serde(rename = "latestDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_date: Option<f64>,
    #[serde(rename = "oldestDate")]
    #[serde(default)]
    pub oldest_date: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionFilter {
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
    #[serde(default)]
    pub tag: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowTypeFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountOpenWorkflowExecutionsInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "executionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    #[serde(rename = "startTimeFilter")]
    #[serde(default)]
    pub start_time_filter: ExecutionTimeFilter,
    #[serde(rename = "tagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    #[serde(rename = "typeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountPendingActivityTasksInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "taskList")]
    #[serde(default)]
    pub task_list: TaskList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountPendingDecisionTasksInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "taskList")]
    #[serde(default)]
    pub task_list: TaskList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecisionTask {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<HistoryEvent>>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "previousStartedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_started_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_token: Option<String>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoryEvent {
    #[serde(rename = "activityTaskCancelRequestedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_cancel_requested_event_attributes:
        Option<ActivityTaskCancelRequestedEventAttributes>,
    #[serde(rename = "activityTaskCanceledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_canceled_event_attributes: Option<ActivityTaskCanceledEventAttributes>,
    #[serde(rename = "activityTaskCompletedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_completed_event_attributes: Option<ActivityTaskCompletedEventAttributes>,
    #[serde(rename = "activityTaskFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_failed_event_attributes: Option<ActivityTaskFailedEventAttributes>,
    #[serde(rename = "activityTaskScheduledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_scheduled_event_attributes: Option<ActivityTaskScheduledEventAttributes>,
    #[serde(rename = "activityTaskStartedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_started_event_attributes: Option<ActivityTaskStartedEventAttributes>,
    #[serde(rename = "activityTaskTimedOutEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_task_timed_out_event_attributes: Option<ActivityTaskTimedOutEventAttributes>,
    #[serde(rename = "cancelTimerFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_timer_failed_event_attributes: Option<CancelTimerFailedEventAttributes>,
    #[serde(rename = "cancelWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_workflow_execution_failed_event_attributes:
        Option<CancelWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "childWorkflowExecutionCanceledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_canceled_event_attributes:
        Option<ChildWorkflowExecutionCanceledEventAttributes>,
    #[serde(rename = "childWorkflowExecutionCompletedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_completed_event_attributes:
        Option<ChildWorkflowExecutionCompletedEventAttributes>,
    #[serde(rename = "childWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_failed_event_attributes:
        Option<ChildWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "childWorkflowExecutionStartedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_started_event_attributes:
        Option<ChildWorkflowExecutionStartedEventAttributes>,
    #[serde(rename = "childWorkflowExecutionTerminatedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_terminated_event_attributes:
        Option<ChildWorkflowExecutionTerminatedEventAttributes>,
    #[serde(rename = "childWorkflowExecutionTimedOutEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_workflow_execution_timed_out_event_attributes:
        Option<ChildWorkflowExecutionTimedOutEventAttributes>,
    #[serde(rename = "completeWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_workflow_execution_failed_event_attributes:
        Option<CompleteWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "continueAsNewWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_as_new_workflow_execution_failed_event_attributes:
        Option<ContinueAsNewWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "decisionTaskCompletedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_attributes: Option<DecisionTaskCompletedEventAttributes>,
    #[serde(rename = "decisionTaskScheduledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_scheduled_event_attributes: Option<DecisionTaskScheduledEventAttributes>,
    #[serde(rename = "decisionTaskStartedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_started_event_attributes: Option<DecisionTaskStartedEventAttributes>,
    #[serde(rename = "decisionTaskTimedOutEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_timed_out_event_attributes: Option<DecisionTaskTimedOutEventAttributes>,
    #[serde(rename = "eventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    #[serde(rename = "eventTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<f64>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "externalWorkflowExecutionCancelRequestedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution_cancel_requested_event_attributes:
        Option<ExternalWorkflowExecutionCancelRequestedEventAttributes>,
    #[serde(rename = "externalWorkflowExecutionSignaledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution_signaled_event_attributes:
        Option<ExternalWorkflowExecutionSignaledEventAttributes>,
    #[serde(rename = "failWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_workflow_execution_failed_event_attributes:
        Option<FailWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "lambdaFunctionCompletedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_completed_event_attributes: Option<LambdaFunctionCompletedEventAttributes>,
    #[serde(rename = "lambdaFunctionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_failed_event_attributes: Option<LambdaFunctionFailedEventAttributes>,
    #[serde(rename = "lambdaFunctionScheduledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_scheduled_event_attributes: Option<LambdaFunctionScheduledEventAttributes>,
    #[serde(rename = "lambdaFunctionStartedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_started_event_attributes: Option<LambdaFunctionStartedEventAttributes>,
    #[serde(rename = "lambdaFunctionTimedOutEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_timed_out_event_attributes: Option<LambdaFunctionTimedOutEventAttributes>,
    #[serde(rename = "markerRecordedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_recorded_event_attributes: Option<MarkerRecordedEventAttributes>,
    #[serde(rename = "recordMarkerFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_marker_failed_event_attributes: Option<RecordMarkerFailedEventAttributes>,
    #[serde(rename = "requestCancelActivityTaskFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_activity_task_failed_event_attributes:
        Option<RequestCancelActivityTaskFailedEventAttributes>,
    #[serde(rename = "requestCancelExternalWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_external_workflow_execution_failed_event_attributes:
        Option<RequestCancelExternalWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "requestCancelExternalWorkflowExecutionInitiatedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_external_workflow_execution_initiated_event_attributes:
        Option<RequestCancelExternalWorkflowExecutionInitiatedEventAttributes>,
    #[serde(rename = "scheduleActivityTaskFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_activity_task_failed_event_attributes:
        Option<ScheduleActivityTaskFailedEventAttributes>,
    #[serde(rename = "scheduleLambdaFunctionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_lambda_function_failed_event_attributes:
        Option<ScheduleLambdaFunctionFailedEventAttributes>,
    #[serde(rename = "signalExternalWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_external_workflow_execution_failed_event_attributes:
        Option<SignalExternalWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "signalExternalWorkflowExecutionInitiatedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_external_workflow_execution_initiated_event_attributes:
        Option<SignalExternalWorkflowExecutionInitiatedEventAttributes>,
    #[serde(rename = "startChildWorkflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_child_workflow_execution_failed_event_attributes:
        Option<StartChildWorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "startChildWorkflowExecutionInitiatedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_child_workflow_execution_initiated_event_attributes:
        Option<StartChildWorkflowExecutionInitiatedEventAttributes>,
    #[serde(rename = "startLambdaFunctionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_lambda_function_failed_event_attributes:
        Option<StartLambdaFunctionFailedEventAttributes>,
    #[serde(rename = "startTimerFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timer_failed_event_attributes: Option<StartTimerFailedEventAttributes>,
    #[serde(rename = "timerCanceledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_canceled_event_attributes: Option<TimerCanceledEventAttributes>,
    #[serde(rename = "timerFiredEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_fired_event_attributes: Option<TimerFiredEventAttributes>,
    #[serde(rename = "timerStartedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_started_event_attributes: Option<TimerStartedEventAttributes>,
    #[serde(rename = "workflowExecutionCancelRequestedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_cancel_requested_event_attributes:
        Option<WorkflowExecutionCancelRequestedEventAttributes>,
    #[serde(rename = "workflowExecutionCanceledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_canceled_event_attributes:
        Option<WorkflowExecutionCanceledEventAttributes>,
    #[serde(rename = "workflowExecutionCompletedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_completed_event_attributes:
        Option<WorkflowExecutionCompletedEventAttributes>,
    #[serde(rename = "workflowExecutionContinuedAsNewEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_continued_as_new_event_attributes:
        Option<WorkflowExecutionContinuedAsNewEventAttributes>,
    #[serde(rename = "workflowExecutionFailedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_failed_event_attributes: Option<WorkflowExecutionFailedEventAttributes>,
    #[serde(rename = "workflowExecutionSignaledEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_signaled_event_attributes:
        Option<WorkflowExecutionSignaledEventAttributes>,
    #[serde(rename = "workflowExecutionStartedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_started_event_attributes:
        Option<WorkflowExecutionStartedEventAttributes>,
    #[serde(rename = "workflowExecutionTerminatedEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_terminated_event_attributes:
        Option<WorkflowExecutionTerminatedEventAttributes>,
    #[serde(rename = "workflowExecutionTimedOutEventAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_timed_out_event_attributes:
        Option<WorkflowExecutionTimedOutEventAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskCancelRequestedEventAttributes {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskCanceledEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "latestCancelRequestedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_cancel_requested_event_id: Option<i64>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskCompletedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskScheduledEventAttributes {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "activityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "heartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "scheduleToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_close_timeout: Option<String>,
    #[serde(rename = "scheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_start_timeout: Option<String>,
    #[serde(rename = "startToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskStartedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityTaskTimedOutEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "timeoutType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTimerFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "timerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildWorkflowExecutionCanceledEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowType {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildWorkflowExecutionCompletedEventAttributes {
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildWorkflowExecutionStartedEventAttributes {
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildWorkflowExecutionTerminatedEventAttributes {
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildWorkflowExecutionTimedOutEventAttributes {
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "timeoutType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinueAsNewWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecisionTaskCompletedEventAttributes {
    #[serde(rename = "executionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskListScheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list_schedule_to_start_timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecisionTaskScheduledEventAttributes {
    #[serde(rename = "scheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_start_timeout: Option<String>,
    #[serde(rename = "startToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecisionTaskStartedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecisionTaskTimedOutEventAttributes {
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "timeoutType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalWorkflowExecutionCancelRequestedEventAttributes {
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalWorkflowExecutionSignaledEventAttributes {
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "workflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution: Option<WorkflowExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionCompletedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionScheduledEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionStartedEventAttributes {
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionTimedOutEventAttributes {
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "timeoutType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarkerRecordedEventAttributes {
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "markerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordMarkerFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "markerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCancelActivityTaskFailedEventAttributes {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCancelExternalWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCancelExternalWorkflowExecutionInitiatedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleActivityTaskFailedEventAttributes {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "activityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleLambdaFunctionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalExternalWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalExternalWorkflowExecutionInitiatedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "signalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_name: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChildWorkflowExecutionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "initiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_event_id: Option<i64>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChildWorkflowExecutionInitiatedEventAttributes {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLambdaFunctionFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "scheduledEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTimerFailedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "timerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimerCanceledEventAttributes {
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "timerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimerFiredEventAttributes {
    #[serde(rename = "startedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_event_id: Option<i64>,
    #[serde(rename = "timerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimerStartedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "startToFireTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_fire_timeout: Option<String>,
    #[serde(rename = "timerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionCancelRequestedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "externalInitiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_initiated_event_id: Option<i64>,
    #[serde(rename = "externalWorkflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution: Option<WorkflowExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionCanceledEventAttributes {
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionCompletedEventAttributes {
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionContinuedAsNewEventAttributes {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "newExecutionRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_execution_run_id: Option<String>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionFailedEventAttributes {
    #[serde(rename = "decisionTaskCompletedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_completed_event_id: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionSignaledEventAttributes {
    #[serde(rename = "externalInitiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_initiated_event_id: Option<i64>,
    #[serde(rename = "externalWorkflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_workflow_execution: Option<WorkflowExecution>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "signalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionStartedEventAttributes {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(rename = "continuedExecutionRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continued_execution_run_id: Option<String>,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "parentInitiatedEventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_initiated_event_id: Option<i64>,
    #[serde(rename = "parentWorkflowExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_workflow_execution: Option<WorkflowExecution>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionTerminatedEventAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionTimedOutEventAttributes {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(rename = "timeoutType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActivityTypeInput {
    #[serde(rename = "activityType")]
    #[serde(default)]
    pub activity_type: ActivityType,
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkflowTypeInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecateActivityTypeInput {
    #[serde(rename = "activityType")]
    #[serde(default)]
    pub activity_type: ActivityType,
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecateDomainInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecateWorkflowTypeInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActivityTypeInput {
    #[serde(rename = "activityType")]
    #[serde(default)]
    pub activity_type: ActivityType,
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkflowExecutionInput {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub execution: WorkflowExecution,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkflowTypeInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DomainConfiguration>,
    #[serde(rename = "domainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_info: Option<DomainInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainConfiguration {
    #[serde(rename = "workflowExecutionRetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_execution_retention_period_in_days: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainInfos {
    #[serde(rename = "domainInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_infos: Option<Vec<DomainInfo>>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkflowExecutionHistoryInput {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub execution: WorkflowExecution,
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct History {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<HistoryEvent>>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActivityTypesInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "registrationStatus")]
    #[serde(default)]
    pub registration_status: String,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClosedWorkflowExecutionsInput {
    #[serde(rename = "closeStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_status_filter: Option<CloseStatusFilter>,
    #[serde(rename = "closeTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_time_filter: Option<ExecutionTimeFilter>,
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "executionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    #[serde(rename = "startTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_filter: Option<ExecutionTimeFilter>,
    #[serde(rename = "tagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    #[serde(rename = "typeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsInput {
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "registrationStatus")]
    #[serde(default)]
    pub registration_status: String,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpenWorkflowExecutionsInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "executionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_filter: Option<WorkflowExecutionFilter>,
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    #[serde(rename = "startTimeFilter")]
    #[serde(default)]
    pub start_time_filter: ExecutionTimeFilter,
    #[serde(rename = "tagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<TagFilter>,
    #[serde(rename = "typeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<WorkflowTypeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkflowTypesInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "registrationStatus")]
    #[serde(default)]
    pub registration_status: String,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingTaskCount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForActivityTaskInput {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    pub task_list: TaskList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForDecisionTaskInput {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "maximumPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_page_size: Option<i32>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    #[serde(rename = "startAtPreviousStartedEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at_previous_started_event: Option<bool>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    pub task_list: TaskList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordActivityTaskHeartbeatInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterActivityTypeInput {
    #[serde(rename = "defaultTaskHeartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_heartbeat_timeout: Option<String>,
    #[serde(rename = "defaultTaskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    #[serde(rename = "defaultTaskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    #[serde(rename = "defaultTaskScheduleToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_close_timeout: Option<String>,
    #[serde(rename = "defaultTaskScheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_schedule_to_start_timeout: Option<String>,
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDomainInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ResourceTag>>,
    #[serde(rename = "workflowExecutionRetentionPeriodInDays")]
    #[serde(default)]
    pub workflow_execution_retention_period_in_days: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterWorkflowTypeInput {
    #[serde(rename = "defaultChildPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child_policy: Option<String>,
    #[serde(rename = "defaultExecutionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_execution_start_to_close_timeout: Option<String>,
    #[serde(rename = "defaultLambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lambda_role: Option<String>,
    #[serde(rename = "defaultTaskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    #[serde(rename = "defaultTaskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCancelWorkflowExecutionInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RespondActivityTaskCanceledInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RespondActivityTaskCompletedInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RespondActivityTaskFailedInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RespondDecisionTaskCompletedInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decisions: Option<Vec<Decision>>,
    #[serde(rename = "executionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context: Option<String>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskListScheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list_schedule_to_start_timeout: Option<String>,
    #[serde(rename = "taskToken")]
    #[serde(default)]
    pub task_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Decision {
    #[serde(rename = "cancelTimerDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_timer_decision_attributes: Option<CancelTimerDecisionAttributes>,
    #[serde(rename = "cancelWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_workflow_execution_decision_attributes:
        Option<CancelWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "completeWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_workflow_execution_decision_attributes:
        Option<CompleteWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "continueAsNewWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_as_new_workflow_execution_decision_attributes:
        Option<ContinueAsNewWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "decisionType")]
    #[serde(default)]
    pub decision_type: String,
    #[serde(rename = "failWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_workflow_execution_decision_attributes:
        Option<FailWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "recordMarkerDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_marker_decision_attributes: Option<RecordMarkerDecisionAttributes>,
    #[serde(rename = "requestCancelActivityTaskDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_activity_task_decision_attributes:
        Option<RequestCancelActivityTaskDecisionAttributes>,
    #[serde(rename = "requestCancelExternalWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_cancel_external_workflow_execution_decision_attributes:
        Option<RequestCancelExternalWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "scheduleActivityTaskDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_activity_task_decision_attributes: Option<ScheduleActivityTaskDecisionAttributes>,
    #[serde(rename = "scheduleLambdaFunctionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_lambda_function_decision_attributes:
        Option<ScheduleLambdaFunctionDecisionAttributes>,
    #[serde(rename = "signalExternalWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_external_workflow_execution_decision_attributes:
        Option<SignalExternalWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "startChildWorkflowExecutionDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_child_workflow_execution_decision_attributes:
        Option<StartChildWorkflowExecutionDecisionAttributes>,
    #[serde(rename = "startTimerDecisionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timer_decision_attributes: Option<StartTimerDecisionAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTimerDecisionAttributes {
    #[serde(rename = "timerId")]
    #[serde(default)]
    pub timer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelWorkflowExecutionDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteWorkflowExecutionDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinueAsNewWorkflowExecutionDecisionAttributes {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    #[serde(rename = "workflowTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailWorkflowExecutionDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordMarkerDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "markerName")]
    #[serde(default)]
    pub marker_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCancelActivityTaskDecisionAttributes {
    #[serde(rename = "activityId")]
    #[serde(default)]
    pub activity_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCancelExternalWorkflowExecutionDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleActivityTaskDecisionAttributes {
    #[serde(rename = "activityId")]
    #[serde(default)]
    pub activity_id: String,
    #[serde(rename = "activityType")]
    #[serde(default)]
    pub activity_type: ActivityType,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "heartbeatTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "scheduleToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_close_timeout: Option<String>,
    #[serde(rename = "scheduleToStartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_to_start_timeout: Option<String>,
    #[serde(rename = "startToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleLambdaFunctionDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "startToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_to_close_timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalExternalWorkflowExecutionDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "signalName")]
    #[serde(default)]
    pub signal_name: String,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChildWorkflowExecutionDecisionAttributes {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTimerDecisionAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    #[serde(rename = "startToFireTimeout")]
    #[serde(default)]
    pub start_to_fire_timeout: String,
    #[serde(rename = "timerId")]
    #[serde(default)]
    pub timer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Run {
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalWorkflowExecutionInput {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "signalName")]
    #[serde(default)]
    pub signal_name: String,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWorkflowExecutionInput {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<ResourceTag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateWorkflowExecutionInput {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(default)]
    pub workflow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UndeprecateActivityTypeInput {
    #[serde(rename = "activityType")]
    #[serde(default)]
    pub activity_type: ActivityType,
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UndeprecateDomainInput {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UndeprecateWorkflowTypeInput {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    pub workflow_type: WorkflowType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionCount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionDetail {
    #[serde(rename = "executionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_configuration: Option<WorkflowExecutionConfiguration>,
    #[serde(rename = "executionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<WorkflowExecutionInfo>,
    #[serde(rename = "latestActivityTaskTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_activity_task_timestamp: Option<f64>,
    #[serde(rename = "latestExecutionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_execution_context: Option<String>,
    #[serde(rename = "openCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_counts: Option<WorkflowExecutionOpenCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionConfiguration {
    #[serde(rename = "childPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_policy: Option<String>,
    #[serde(rename = "executionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_to_close_timeout: Option<String>,
    #[serde(rename = "lambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_role: Option<String>,
    #[serde(rename = "taskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<TaskList>,
    #[serde(rename = "taskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_priority: Option<String>,
    #[serde(rename = "taskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_to_close_timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionInfo {
    #[serde(rename = "cancelRequested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_requested: Option<bool>,
    #[serde(rename = "closeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_status: Option<String>,
    #[serde(rename = "closeTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_timestamp: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<WorkflowExecution>,
    #[serde(rename = "executionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<WorkflowExecution>,
    #[serde(rename = "startTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionOpenCounts {
    #[serde(rename = "openActivityTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_activity_tasks: Option<i32>,
    #[serde(rename = "openChildWorkflowExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_child_workflow_executions: Option<i32>,
    #[serde(rename = "openDecisionTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_decision_tasks: Option<i32>,
    #[serde(rename = "openLambdaFunctions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_lambda_functions: Option<i32>,
    #[serde(rename = "openTimers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_timers: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowExecutionInfos {
    #[serde(rename = "executionInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_infos: Option<Vec<WorkflowExecutionInfo>>,
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowTypeDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<WorkflowTypeConfiguration>,
    #[serde(rename = "typeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_info: Option<WorkflowTypeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowTypeConfiguration {
    #[serde(rename = "defaultChildPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_child_policy: Option<String>,
    #[serde(rename = "defaultExecutionStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_execution_start_to_close_timeout: Option<String>,
    #[serde(rename = "defaultLambdaRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_lambda_role: Option<String>,
    #[serde(rename = "defaultTaskList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_list: Option<TaskList>,
    #[serde(rename = "defaultTaskPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_priority: Option<String>,
    #[serde(rename = "defaultTaskStartToCloseTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_task_start_to_close_timeout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowTypeInfo {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "deprecationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "workflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_type: Option<WorkflowType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowTypeInfos {
    #[serde(rename = "nextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "typeInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_infos: Option<Vec<WorkflowTypeInfo>>,
}

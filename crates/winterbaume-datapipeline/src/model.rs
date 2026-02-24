//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-data-pipeline

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivatePipelineInput {
    #[serde(rename = "parameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(rename = "startTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterValue {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    pub string_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivatePipelineOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsInput {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "uniqueId")]
    #[serde(default)]
    pub unique_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineOutput {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivatePipelineInput {
    #[serde(rename = "cancelActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_active: Option<bool>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivatePipelineOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipelineInput {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeObjectsInput {
    #[serde(rename = "evaluateExpressions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_expressions: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "objectIds")]
    #[serde(default)]
    pub object_ids: Vec<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeObjectsOutput {
    #[serde(rename = "hasMoreResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pipelineObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_objects: Option<Vec<PipelineObject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineObject {
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "refValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_value: Option<String>,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePipelinesInput {
    #[serde(rename = "pipelineIds")]
    #[serde(default)]
    pub pipeline_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePipelinesOutput {
    #[serde(rename = "pipelineDescriptionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_description_list: Option<Vec<PipelineDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateExpressionInput {
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "objectId")]
    #[serde(default)]
    pub object_id: String,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateExpressionOutput {
    #[serde(rename = "evaluatedExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineDefinitionInput {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineDefinitionOutput {
    #[serde(rename = "parameterObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_objects: Option<Vec<ParameterObject>>,
    #[serde(rename = "parameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    #[serde(rename = "pipelineObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_objects: Option<Vec<PipelineObject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterObject {
    #[serde(default)]
    pub attributes: Vec<ParameterAttribute>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterAttribute {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    pub string_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelinesInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelinesOutput {
    #[serde(rename = "hasMoreResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pipelineIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id_list: Option<Vec<PipelineIdName>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineIdName {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForTaskInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "instanceIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity: Option<InstanceIdentity>,
    #[serde(rename = "workerGroup")]
    #[serde(default)]
    pub worker_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceIdentity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PollForTaskOutput {
    #[serde(rename = "taskObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_object: Option<TaskObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskObject {
    #[serde(rename = "attemptId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<std::collections::HashMap<String, PipelineObject>>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPipelineDefinitionInput {
    #[serde(rename = "parameterObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_objects: Option<Vec<ParameterObject>>,
    #[serde(rename = "parameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(rename = "pipelineObjects")]
    #[serde(default)]
    pub pipeline_objects: Vec<PipelineObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPipelineDefinitionOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<bool>,
    #[serde(rename = "validationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(rename = "validationWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_warnings: Option<Vec<ValidationWarning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationWarning {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryObjectsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Query>,
    #[serde(default)]
    pub sphere: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Query {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<Selector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Selector {
    #[serde(rename = "fieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Operator {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryObjectsOutput {
    #[serde(rename = "hasMoreResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsInput {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportTaskProgressInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportTaskProgressOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportTaskRunnerHeartbeatInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "taskrunnerId")]
    #[serde(default)]
    pub taskrunner_id: String,
    #[serde(rename = "workerGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportTaskRunnerHeartbeatOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetStatusInput {
    #[serde(rename = "objectIds")]
    #[serde(default)]
    pub object_ids: Vec<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetTaskStatusInput {
    #[serde(rename = "errorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "errorStackTrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_stack_trace: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    pub task_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetTaskStatusOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePipelineDefinitionInput {
    #[serde(rename = "parameterObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_objects: Option<Vec<ParameterObject>>,
    #[serde(rename = "parameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(rename = "pipelineObjects")]
    #[serde(default)]
    pub pipeline_objects: Vec<PipelineObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePipelineDefinitionOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<bool>,
    #[serde(rename = "validationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(rename = "validationWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_warnings: Option<Vec<ValidationWarning>>,
}

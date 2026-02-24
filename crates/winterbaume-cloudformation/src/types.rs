use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub stack_id: String,
    pub stack_name: String,
    pub stack_status: String,
    pub creation_time: String,
    pub last_updated_time: Option<String>,
    pub deletion_time: Option<String>,
    pub description: Option<String>,
    pub template_body: Option<String>,
    pub stack_policy_body: Option<String>,
    pub parameters: Vec<StackParameter>,
    pub outputs: Vec<StackOutput>,
    pub tags: Vec<StackTag>,
    pub capabilities: Vec<String>,
    pub resources: Vec<StackResource>,
    pub events: Vec<StackEvent>,
    pub change_sets: Vec<ChangeSet>,
    pub exports: Vec<StackExport>,
    pub role_arn: Option<String>,
    pub timeout_in_minutes: Option<i32>,
    pub disable_rollback: bool,
    pub enable_termination_protection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackParameter {
    pub parameter_key: String,
    pub parameter_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackOutput {
    pub output_key: String,
    pub output_value: String,
    pub description: Option<String>,
    pub export_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackResource {
    pub logical_resource_id: String,
    pub physical_resource_id: Option<String>,
    pub resource_type: String,
    pub resource_status: String,
    pub timestamp: String,
    pub stack_id: String,
    pub stack_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackEvent {
    pub event_id: String,
    pub stack_id: String,
    pub stack_name: String,
    pub logical_resource_id: String,
    pub physical_resource_id: Option<String>,
    pub resource_type: String,
    pub timestamp: String,
    pub resource_status: String,
    pub resource_status_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackExport {
    pub name: String,
    pub value: String,
    pub exporting_stack_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    pub change_set_id: String,
    pub change_set_name: String,
    pub stack_id: String,
    pub stack_name: String,
    pub status: String,
    pub status_reason: Option<String>,
    pub execution_status: String,
    pub description: Option<String>,
    pub creation_time: String,
    pub parameters: Vec<StackParameter>,
    pub tags: Vec<StackTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackSet {
    pub stack_set_id: String,
    pub stack_set_name: String,
    pub stack_set_arn: String,
    pub status: String,
    pub description: Option<String>,
    pub template_body: Option<String>,
    pub parameters: Vec<StackParameter>,
    pub tags: Vec<StackTag>,
    pub capabilities: Vec<String>,
    pub operations: Vec<StackSetOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackSetOperation {
    pub operation_id: String,
    pub action: String,
    pub status: String,
    pub creation_timestamp: String,
    pub end_timestamp: Option<String>,
    pub stack_set_id: String,
    pub results: Vec<StackSetOperationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackSetOperationResult {
    pub account: String,
    pub region: String,
    pub status: String,
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackInstance {
    pub stack_set_name: String,
    pub account: String,
    pub region: String,
    pub stack_id: Option<String>,
    pub status: String,
    pub status_reason: Option<String>,
    pub stack_set_id: String,
    pub parameter_overrides: Vec<StackParameter>,
}

/// Key used to look up a stack by name or ARN.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackKey(pub String);

/// Key used to look up a StackSet instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackInstanceKey {
    pub stack_set_name: String,
    pub account: String,
    pub region: String,
}

/// Represents a registered CloudFormation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredType {
    pub type_name: String,
    pub type_kind: String,
    pub type_arn: Option<String>,
    pub default_version_id: Option<String>,
    pub last_updated: Option<String>,
    pub description: Option<String>,
}

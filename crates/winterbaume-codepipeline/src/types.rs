use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub name: String,
    pub arn: String,
    pub role_arn: String,
    pub stages: Value,
    pub version: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    /// Stage transitions: key is (stage_name, transition_type), value is disabled info.
    pub disabled_transitions: HashMap<(String, String), DisabledTransition>,
    /// Raw JSON for `artifact_store` blocks.
    pub artifact_store: Option<Value>,
    /// Raw JSON for `trigger` blocks.
    pub trigger: Option<Value>,
    /// Raw JSON for `variable` blocks.
    pub variable: Option<Value>,
    /// Pipeline execution mode.
    pub execution_mode: Option<String>,
    /// Pipeline type.
    pub pipeline_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DisabledTransition {
    pub reason: String,
    pub last_changed_at: DateTime<Utc>,
}

/// A custom action type registered by the user.
#[derive(Debug, Clone)]
pub struct CustomActionType {
    pub category: String,
    pub provider: String,
    pub version: String,
    pub settings: Option<Value>,
    pub configuration_properties: Option<Value>,
    pub input_artifact_details: ArtifactDetailsData,
    pub output_artifact_details: ArtifactDetailsData,
    pub tags: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ArtifactDetailsData {
    pub minimum_count: i32,
    pub maximum_count: i32,
}

/// Composite key for a custom action type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionTypeKey {
    pub category: String,
    pub provider: String,
    pub version: String,
}

/// A webhook registered in the account.
#[derive(Debug, Clone)]
pub struct Webhook {
    pub name: String,
    pub arn: String,
    pub url: String,
    pub definition: Value,
    pub tags: HashMap<String, String>,
    pub registered_with_third_party: bool,
    pub created: DateTime<Utc>,
}

/// A pipeline execution record.
#[derive(Debug, Clone)]
pub struct PipelineExecution {
    pub pipeline_execution_id: String,
    pub pipeline_name: String,
    pub pipeline_version: i32,
    pub status: String,
    pub status_summary: Option<String>,
    pub start_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
    pub trigger: Option<Value>,
    pub source_revisions: Vec<Value>,
    pub variables: Vec<Value>,
    pub execution_mode: Option<String>,
    pub execution_type: Option<String>,
}

/// A job for a custom action.
#[derive(Debug, Clone)]
pub struct ActionJob {
    pub id: String,
    pub nonce: String,
    pub status: JobStatus,
    pub account_id: String,
    pub action_type_id: Value,
    pub pipeline_context: Option<Value>,
    pub data: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Created,
    InProgress,
    Succeeded,
    Failed,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Created => write!(f, "Created"),
            JobStatus::InProgress => write!(f, "InProgress"),
            JobStatus::Succeeded => write!(f, "Succeeded"),
            JobStatus::Failed => write!(f, "Failed"),
        }
    }
}

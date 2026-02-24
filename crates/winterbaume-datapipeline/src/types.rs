use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub pipeline_id: String,
    pub name: String,
    pub description: String,
    pub unique_id: String,
    pub tags: Vec<PipelineTag>,
    pub created_at: DateTime<Utc>,
    pub status: PipelineStatus,
    pub pipeline_objects: Vec<PipelineObject>,
    pub parameter_objects: Vec<Value>,
    pub parameter_values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStatus {
    Pending,
    Activating,
    Active,
    Deactivating,
    Inactive,
    Deleting,
}

impl PipelineStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PipelineStatus::Pending => "PENDING",
            PipelineStatus::Activating => "ACTIVATING",
            PipelineStatus::Active => "ACTIVE",
            PipelineStatus::Deactivating => "DEACTIVATING",
            PipelineStatus::Inactive => "INACTIVE",
            PipelineStatus::Deleting => "DELETING",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PipelineTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct PipelineObject {
    pub id: String,
    pub name: String,
    pub fields: Vec<PipelineField>,
}

#[derive(Debug, Clone)]
pub struct PipelineField {
    pub key: String,
    pub string_value: Option<String>,
    pub ref_value: Option<String>,
}

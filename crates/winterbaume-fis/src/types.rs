use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An FIS experiment template.
#[derive(Debug, Clone)]
pub struct ExperimentTemplate {
    pub id: String,
    pub arn: String,
    pub description: String,
    pub role_arn: String,
    pub targets: HashMap<String, ExperimentTemplateTarget>,
    pub actions: HashMap<String, ExperimentTemplateAction>,
    pub stop_conditions: Vec<ExperimentTemplateStopCondition>,
    pub tags: HashMap<String, String>,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
}

/// A target for an experiment template.
#[derive(Debug, Clone)]
pub struct ExperimentTemplateTarget {
    pub resource_type: String,
    pub resource_arns: Vec<String>,
    pub resource_tags: HashMap<String, String>,
    pub filters: Vec<ExperimentTemplateTargetFilter>,
    pub selection_mode: String,
    pub parameters: HashMap<String, String>,
}

/// A filter for an experiment template target.
#[derive(Debug, Clone)]
pub struct ExperimentTemplateTargetFilter {
    pub path: String,
    pub values: Vec<String>,
}

/// An action for an experiment template.
#[derive(Debug, Clone)]
pub struct ExperimentTemplateAction {
    pub action_id: String,
    pub description: Option<String>,
    pub parameters: HashMap<String, String>,
    pub targets: HashMap<String, String>,
    pub start_after: Vec<String>,
}

/// A stop condition for an experiment template.
#[derive(Debug, Clone)]
pub struct ExperimentTemplateStopCondition {
    pub source: String,
    pub value: Option<String>,
}

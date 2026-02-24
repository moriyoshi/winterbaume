use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub workspace_id: String,
    pub arn: String,
    pub alias: Option<String>,
    pub status: WorkspaceStatus,
    pub prometheus_endpoint: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceSummary {
    pub workspace_id: String,
    pub arn: String,
    pub alias: Option<String>,
    pub status: WorkspaceStatus,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceStatus {
    pub status_code: String,
}

#[derive(Debug, Clone)]
pub struct LoggingConfiguration {
    pub workspace_id: String,
    pub log_group_arn: String,
    pub status: LoggingConfigurationStatus,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LoggingConfigurationStatus {
    pub status_code: String,
}

#[derive(Debug, Clone)]
pub struct RuleGroupsNamespace {
    pub name: String,
    pub arn: String,
    pub workspace_id: String,
    pub data: String,
    pub status: RuleGroupsNamespaceStatus,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct RuleGroupsNamespaceStatus {
    pub status_code: String,
}

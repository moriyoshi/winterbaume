use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Application {
    pub application_id: String,
    pub application_name: String,
    pub compute_platform: String,
    pub create_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DeploymentGroup {
    pub deployment_group_id: String,
    pub deployment_group_name: String,
    pub application_name: String,
    pub service_role_arn: String,
    pub deployment_config_name: String,
    pub compute_platform: String,
    pub create_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Deployment {
    pub deployment_id: String,
    pub application_name: String,
    pub deployment_group_name: String,
    pub deployment_config_name: String,
    pub description: String,
    pub revision_type: Option<String>,
    pub revision_s3_bucket: Option<String>,
    pub revision_s3_key: Option<String>,
    pub revision_s3_bundle_type: Option<String>,
    pub revision_github_repository: Option<String>,
    pub revision_github_commit_id: Option<String>,
    pub status: String,
    pub create_time: DateTime<Utc>,
    pub file_exists_behavior: Option<String>,
    pub ignore_application_stop_failures: bool,
}

/// Tags stored per ARN.
pub type TagMap = HashMap<String, HashMap<String, String>>;

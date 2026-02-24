use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Resilience Hub application.
#[derive(Debug, Clone)]
pub struct App {
    pub app_arn: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub compliance_status: String,
    pub policy_arn: String,
    pub assessment_schedule: String,
    pub tags: HashMap<String, String>,
    pub versions: Vec<AppVersion>,
    pub next_version_id: i64,
    pub resources: Vec<AppVersionResource>,
    pub app_template_body: String,
    /// App components (draft version)
    pub app_components: Vec<AppComponentData>,
    /// Assessments run against this app
    pub assessments: Vec<AssessmentData>,
}

/// An app component stored in state.
#[derive(Debug, Clone)]
pub struct AppComponentData {
    pub id: String,
    pub name: String,
    pub component_type: String,
}

/// A lightweight assessment record stored in state.
#[derive(Debug, Clone)]
pub struct AssessmentData {
    pub assessment_arn: String,
    pub assessment_name: String,
    pub assessment_status: String,
    pub app_arn: String,
    pub app_version: String,
}

/// Summary of a Resilience Hub application (used in list responses).
#[derive(Debug, Clone)]
pub struct AppSummary {
    pub app_arn: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub compliance_status: String,
    pub assessment_schedule: String,
}

/// A resiliency policy.
#[derive(Debug, Clone)]
pub struct ResiliencyPolicyData {
    pub policy_arn: String,
    pub policy_name: String,
    pub policy_description: String,
    pub data_location_constraint: String,
    pub tier: String,
    pub policy: HashMap<String, FailurePolicyData>,
    pub creation_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// A failure policy (RTO/RPO).
#[derive(Debug, Clone)]
pub struct FailurePolicyData {
    pub rpo_in_secs: i32,
    pub rto_in_secs: i32,
}

/// An application version.
#[derive(Debug, Clone)]
pub struct AppVersion {
    pub app_version: String,
    pub identifier: i64,
    pub creation_time: DateTime<Utc>,
}

/// A physical resource attached to an app version.
#[derive(Debug, Clone)]
pub struct AppVersionResource {
    pub app_arn: String,
    pub resource_name: String,
    pub logical_resource_id: String,
    pub physical_resource_id: String,
    pub resource_type: String,
    pub app_components: Vec<String>,
    pub aws_region: String,
    pub aws_account_id: String,
}

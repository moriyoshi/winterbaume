use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ReportGroup {
    pub arn: String,
    pub name: String,
    pub r#type: String,
    pub export_config_type: Option<String>,
    pub tags: Vec<Tag>,
    pub created: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Webhook {
    pub project_name: String,
    pub url: String,
    pub branch_filter: Option<String>,
    pub build_type: Option<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceCredential {
    pub arn: String,
    pub server_type: String,
    pub auth_type: String,
    pub resource: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub source_type: String,
    pub source_location: String,
    pub artifact_type: String,
    pub artifact_location: Option<String>,
    pub environment_type: String,
    pub environment_image: String,
    pub environment_compute_type: String,
    pub service_role: String,
    pub tags: Vec<Tag>,
    pub created: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BuildPhase {
    pub phase_type: String,
    pub phase_status: Option<String>,
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub duration_in_seconds: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Build {
    pub id: String,
    pub arn: String,
    pub project_name: String,
    pub build_status: String,
    pub current_phase: String,
    pub source_type: String,
    pub source_location: String,
    pub source_version: String,
    pub artifact_type: String,
    pub artifact_location: Option<String>,
    pub environment_type: String,
    pub environment_image: String,
    pub environment_compute_type: String,
    pub service_role: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub build_number: i64,
    pub phases: Vec<BuildPhase>,
}

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Synthetics canary.
#[derive(Debug, Clone)]
pub struct Canary {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub artifact_s3_location: String,
    pub runtime_version: String,
    pub handler: String,
    pub schedule_expression: String,
    pub schedule_duration_in_seconds: Option<i64>,
    pub success_retention_period_in_days: i32,
    pub failure_retention_period_in_days: i32,
    pub status: CanaryStatus,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub execution_role_arn: String,
    pub s3_encryption_mode: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
}

/// Canary status information.
#[derive(Debug, Clone)]
pub struct CanaryStatus {
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
}

/// Input for creating a canary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCanaryInput {
    pub name: Option<String>,
    pub code: Option<CanaryCodeInput>,
    pub artifact_s3_location: Option<String>,
    pub execution_role_arn: Option<String>,
    pub schedule: Option<CanaryScheduleInput>,
    pub runtime_version: Option<String>,
    pub success_retention_period_in_days: Option<i32>,
    pub failure_retention_period_in_days: Option<i32>,
    pub tags: Option<std::collections::HashMap<String, String>>,
}

/// Code input for canary creation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanaryCodeInput {
    pub s3_bucket: Option<String>,
    pub s3_key: Option<String>,
    pub s3_version: Option<String>,
    pub zip_file: Option<String>,
    pub handler: Option<String>,
}

/// Schedule input for canary creation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanaryScheduleInput {
    pub expression: Option<String>,
    pub duration_in_seconds: Option<i64>,
}

/// Serializable canary representation for API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanaryOutput {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub artifact_s3_location: String,
    pub status: CanaryStatusOutput,
    pub schedule: CanaryScheduleOutput,
    pub code: CanaryCodeOutput,
    pub execution_role_arn: String,
    pub runtime_version: String,
    pub success_retention_period_in_days: i32,
    pub failure_retention_period_in_days: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

/// Serializable canary status for API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanaryStatusOutput {
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_code: Option<String>,
}

/// Serializable canary schedule for API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanaryScheduleOutput {
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

/// Serializable canary code output for API responses.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CanaryCodeOutput {
    pub handler: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location_arn: Option<String>,
}

impl From<&Canary> for CanaryOutput {
    fn from(c: &Canary) -> Self {
        let tags = if c.tags.is_empty() {
            None
        } else {
            Some(c.tags.clone())
        };
        CanaryOutput {
            name: c.name.clone(),
            id: c.id.clone(),
            arn: c.arn.clone(),
            artifact_s3_location: c.artifact_s3_location.clone(),
            status: CanaryStatusOutput {
                state: c.status.state.clone(),
                state_reason: c.status.state_reason.clone(),
                state_reason_code: c.status.state_reason_code.clone(),
            },
            schedule: CanaryScheduleOutput {
                expression: c.schedule_expression.clone(),
                duration_in_seconds: c.schedule_duration_in_seconds,
            },
            code: CanaryCodeOutput {
                handler: c.handler.clone(),
                source_location_arn: None,
            },
            execution_role_arn: c.execution_role_arn.clone(),
            runtime_version: c.runtime_version.clone(),
            success_retention_period_in_days: c.success_retention_period_in_days,
            failure_retention_period_in_days: c.failure_retention_period_in_days,
            tags,
        }
    }
}

/// A single canary run record.
#[derive(Debug, Clone)]
pub struct CanaryRunRecord {
    pub id: String,
    pub name: String,
    pub artifact_s3_location: String,
    pub status: CanaryRunRecordStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status of a canary run.
#[derive(Debug, Clone)]
pub struct CanaryRunRecordStatus {
    pub state: String,
    pub state_reason: Option<String>,
    pub state_reason_code: Option<String>,
    pub test_result: Option<String>,
}

/// A Synthetics group.
#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub id: String,
    pub arn: String,
    /// Set of canary ARNs associated with this group.
    pub resource_arns: HashSet<String>,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: std::collections::HashMap<String, String>,
}

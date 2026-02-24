use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Secret {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
    pub last_changed_date: DateTime<Utc>,
    pub versions: HashMap<String, SecretVersion>,
    pub current_version_id: Option<String>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
    pub resource_policy: Option<String>,
    pub rotation_enabled: Option<bool>,
    pub rotation_lambda_arn: Option<String>,
    pub rotation_rules: Option<RotationRules>,
    pub last_rotated_date: Option<DateTime<Utc>>,
    pub replication_status: Vec<ReplicationStatus>,
    pub primary_region: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SecretVersion {
    pub version_id: String,
    pub secret_string: Option<String>,
    pub secret_binary: Option<Vec<u8>>,
    pub created_date: DateTime<Utc>,
    pub version_stages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RotationRules {
    pub automatically_after_days: Option<i64>,
    pub duration: Option<String>,
    pub schedule_expression: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReplicationStatus {
    pub region: String,
    pub status: String,
    pub status_message: Option<String>,
    pub kms_key_id: Option<String>,
    pub last_accessed_date: Option<DateTime<Utc>>,
}

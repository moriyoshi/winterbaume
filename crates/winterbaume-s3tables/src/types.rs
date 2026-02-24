use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct EncryptionConfig {
    pub sse_algorithm: String,
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StorageClassConfig {
    pub storage_class: String,
}

/// A single maintenance configuration entry (stored as raw JSON value string for simplicity).
pub type MaintenanceValueJson = String;

#[derive(Debug, Clone)]
pub struct TableBucket {
    pub name: String,
    pub arn: String,
    pub owner_account_id: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub encryption: Option<EncryptionConfig>,
    pub maintenance_config: HashMap<String, MaintenanceValueJson>,
    pub metrics_config: Option<String>,
    pub policy: Option<String>,
    pub storage_class: Option<StorageClassConfig>,
    pub replication_config: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub table_bucket_arn: String,
    /// The namespace components, e.g. ["mydb"]
    pub namespace: Vec<String>,
    /// Primary namespace name (first element), used as the path key
    pub name: String,
    pub owner_account_id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub table_bucket_arn: String,
    pub namespace: String,
    pub name: String,
    pub arn: String,
    pub version_token: String,
    pub format: String,
    pub warehouse_location: String,
    pub owner_account_id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub modified_at: DateTime<Utc>,
    pub modified_by: String,
    pub tags: HashMap<String, String>,
    pub policy: Option<String>,
    pub metadata_location: Option<String>,
    pub maintenance_config: HashMap<String, MaintenanceValueJson>,
    pub storage_class: Option<StorageClassConfig>,
    pub replication_config: Option<String>,
    pub record_expiration_config: Option<String>,
}

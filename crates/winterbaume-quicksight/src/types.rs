use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DataSet {
    pub data_set_id: String,
    pub name: String,
    pub arn: String,
    pub import_mode: String,
    pub physical_table_map: HashMap<String, serde_json::Value>,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct QuickSightDataSource {
    pub data_source_id: String,
    pub name: String,
    pub arn: String,
    pub r#type: String,
    pub status: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub permissions: Vec<DataSourceResourcePermission>,
}

#[derive(Debug, Clone, Default)]
pub struct DataSourceResourcePermission {
    pub principal: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Dashboard {
    pub dashboard_id: String,
    pub name: String,
    pub arn: String,
    pub version_arn: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct QuickSightGroup {
    pub group_name: String,
    pub arn: String,
    pub description: String,
    pub principal_id: String,
}

#[derive(Debug, Clone)]
pub struct QuickSightUser {
    pub user_name: String,
    pub arn: String,
    pub email: String,
    pub role: String,
    pub identity_type: String,
    pub active: bool,
    pub principal_id: String,
}

#[derive(Debug, Clone)]
pub struct GroupMembership {
    pub member_name: String,
    pub group_name: String,
}

#[derive(Debug, Clone)]
pub struct Ingestion {
    pub ingestion_id: String,
    pub arn: String,
    pub ingestion_status: String,
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct AccountSettings {
    pub account_name: String,
    pub edition: String,
    pub default_namespace: String,
    pub notification_email: String,
    pub public_sharing_enabled: bool,
    pub termination_protection_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct QuickSightAnalysis {
    pub analysis_id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct QuickSightFolder {
    pub folder_id: String,
    pub name: String,
    pub arn: String,
    pub folder_type: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub member_ids: Vec<String>, // member_id values
}

#[derive(Debug, Clone)]
pub struct FolderMemberEntry {
    pub member_id: String,
    pub member_type: String,
}

#[derive(Debug, Clone)]
pub struct QuickSightNamespace {
    pub name: String,
    pub arn: String,
    pub capacity_region: String,
    pub creation_status: String,
    pub identity_store: String,
}

#[derive(Debug, Clone)]
pub struct QuickSightTemplate {
    pub template_id: String,
    pub name: String,
    pub arn: String,
    pub version_arn: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub version_number: i64,
}

#[derive(Debug, Clone)]
pub struct QuickSightTheme {
    pub theme_id: String,
    pub name: String,
    pub arn: String,
    pub version_arn: String,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub version_number: i64,
}

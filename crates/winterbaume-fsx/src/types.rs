use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub file_system_id: String,
    pub file_system_type: String,
    pub storage_capacity: i64,
    pub storage_type: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub dns_name: String,
    pub kms_key_id: Option<String>,
    pub resource_arn: String,
    pub tags: Vec<Tag>,
    pub windows_configuration: Option<Value>,
    pub lustre_configuration: Option<Value>,
    pub ontap_configuration: Option<Value>,
    pub open_zfs_configuration: Option<Value>,
    pub creation_time: Option<String>,
    pub lifecycle: String,
    pub owner_id: Option<String>,
    pub vpc_id: Option<String>,
    pub deployment_type: Option<String>,
    pub copy_tags_to_backups: bool,
    pub automatic_backup_retention_days: i32,
    pub daily_automatic_backup_start_time: Option<String>,
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backup {
    pub backup_id: String,
    pub file_system_id: String,
    pub lifecycle: String,
    pub creation_time: String,
    pub resource_arn: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub file_system_id: String,
    pub file_system_arn: String,
    pub bucket: String,
    pub prefix: Option<String>,
    pub kms_key_id: Option<String>,
    pub role_arn: String,
    pub client_token: Option<String>,
    pub name: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub owner_id: String,
    pub tags: HashMap<String, String>,
    pub policy: Option<String>,
    pub synchronization_configuration: SynchronizationConfiguration,
}

#[derive(Debug, Clone, Default)]
pub struct SynchronizationConfiguration {
    pub latest_version_number: i32,
    pub import_data_rules: Vec<ImportDataRule>,
    pub expiration_data_rules: Vec<ExpirationDataRule>,
}

#[derive(Debug, Clone)]
pub struct ImportDataRule {
    pub prefix: String,
    pub size_less_than: i64,
    pub trigger: String,
}

#[derive(Debug, Clone)]
pub struct ExpirationDataRule {
    pub days_after_last_access: i32,
}

#[derive(Debug, Clone)]
pub struct MountTarget {
    pub mount_target_id: String,
    pub file_system_id: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub availability_zone_id: String,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub ip_address_type: String,
    pub network_interface_id: String,
    pub security_groups: Vec<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub access_point_id: String,
    pub access_point_arn: String,
    pub file_system_id: String,
    pub name: Option<String>,
    pub posix_user: Option<PosixUser>,
    pub root_directory: Option<RootDirectory>,
    pub status: String,
    pub owner_id: String,
    pub client_token: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PosixUser {
    pub uid: i64,
    pub gid: i64,
    pub secondary_gids: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct RootDirectory {
    pub path: Option<String>,
    pub creation_permissions: Option<CreationPermissions>,
}

#[derive(Debug, Clone)]
pub struct CreationPermissions {
    pub owner_uid: i64,
    pub owner_gid: i64,
    pub permissions: String,
}
